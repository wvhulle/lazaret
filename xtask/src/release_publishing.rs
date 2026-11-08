use std::{
    env,
    ffi::OsStr,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use clap::ValueEnum;

use crate::{
    codeberg_bot_account::{
        BOT_ACCOUNT_EMAIL, BOT_ACCOUNT_NAME, MAIN_BRANCH, TEA_LOGIN_NAME, log_tea_into_bot_account,
        point_origin_at_bot_account,
    },
    task_failure::TaskFailure,
    tool_commands::{required_variable, run_tool, tool_stdout},
};

const CHECKSUM_EXTENSION: &str = "sha256";
const SIGNATURE_EXTENSION: &str = "minisig";
const SIGNING_KEY_VARIABLE: &str = "MINISIGN_SECRET_KEY";
const CHANGELOG_PATH: &str = "CHANGELOG.md";
const RELEASED_MANIFEST_PATH: &str = "Cargo.toml";

#[derive(Clone, Copy, ValueEnum)]
pub enum ReleasePlzCommand {
    Release,
    ReleasePr,
}

impl ReleasePlzCommand {
    const fn subcommand_name(self) -> &'static str {
        match self {
            Self::Release => "release",
            Self::ReleasePr => "release-pr",
        }
    }
}

fn manifest_package_version(manifest_text: &str) -> Option<&str> {
    manifest_text
        .split("\n[")
        .find(|manifest_section| manifest_section.starts_with("package]"))?
        .lines()
        .find_map(|manifest_line| manifest_line.strip_prefix("version = \""))
        .and_then(|quoted_tail| quoted_tail.strip_suffix('"'))
}

fn changelog_has_version_section(changelog_text: &str, version: &str) -> bool {
    let section_prefix = format!("## [{version}]");
    changelog_text
        .lines()
        .any(|changelog_line| changelog_line.starts_with(&section_prefix))
}

pub fn released_package_version() -> Result<String, TaskFailure> {
    let manifest_text = fs::read_to_string(RELEASED_MANIFEST_PATH)?;
    manifest_package_version(&manifest_text)
        .map(str::to_owned)
        .ok_or(TaskFailure::PackageVersionMissing {
            manifest_path: RELEASED_MANIFEST_PATH,
        })
}

fn released_version_has_changelog() -> Result<bool, TaskFailure> {
    let version = released_package_version()?;
    let changelog_text = fs::read_to_string(CHANGELOG_PATH)?;
    Ok(changelog_has_version_section(&changelog_text, &version))
}

fn prepare_ci_checkout() -> Result<(), TaskFailure> {
    let commit_hash = required_variable("CI_COMMIT_SHA")?;
    point_origin_at_bot_account()?;
    run_tool(
        "git",
        Command::new("git").args(["checkout", "-B", MAIN_BRANCH, &commit_hash]),
    )?;
    let tracking_reference = format!("origin/{MAIN_BRANCH}");
    run_tool(
        "git",
        Command::new("git").args([
            "fetch",
            "--quiet",
            "origin",
            &format!("+{MAIN_BRANCH}:refs/remotes/{tracking_reference}"),
        ]),
    )?;
    run_tool(
        "git",
        Command::new("git").args([
            "branch",
            &format!("--set-upstream-to={tracking_reference}"),
            MAIN_BRANCH,
        ]),
    )
}

pub fn run_release_plz(release_plz_command: ReleasePlzCommand) -> Result<(), TaskFailure> {
    if env::var_os("CI").is_some() {
        prepare_ci_checkout()?;
    }
    if matches!(release_plz_command, ReleasePlzCommand::Release)
        && !released_version_has_changelog()?
    {
        eprintln!("xtask: {CHANGELOG_PATH} has no section for this version, skipping the release");
        return Ok(());
    }
    let bot_token = required_variable("CODEBERG_TOKEN")?;
    run_tool(
        "release-plz",
        Command::new("release-plz")
            .args([
                release_plz_command.subcommand_name(),
                "--forge",
                "gitea",
                "--git-token",
                &bot_token,
            ])
            .env("GIT_AUTHOR_NAME", BOT_ACCOUNT_NAME)
            .env("GIT_AUTHOR_EMAIL", BOT_ACCOUNT_EMAIL)
            .env("GIT_COMMITTER_NAME", BOT_ACCOUNT_NAME)
            .env("GIT_COMMITTER_EMAIL", BOT_ACCOUNT_EMAIL),
    )
}

fn is_release_metadata_file(file_path: &Path) -> bool {
    [CHECKSUM_EXTENSION, SIGNATURE_EXTENSION]
        .iter()
        .any(|metadata_extension| file_path.extension() == Some(OsStr::new(metadata_extension)))
}

pub fn regular_files_sorted(directory_path: &Path) -> Result<Vec<PathBuf>, TaskFailure> {
    let mut file_paths = fs::read_dir(directory_path)?
        .map(|directory_entry| directory_entry.map(|directory_entry| directory_entry.path()))
        .collect::<Result<Vec<_>, _>>()?;
    file_paths.retain(|file_path| file_path.is_file());
    file_paths.sort();
    Ok(file_paths)
}

fn write_asset_checksum(assets_directory: &Path, asset_path: &Path) -> Result<(), TaskFailure> {
    let asset_checksum = tool_stdout(
        "sha256sum",
        Command::new("sha256sum")
            .args(asset_path.file_name())
            .current_dir(assets_directory),
    )?;
    fs::write(
        asset_path.with_added_extension(CHECKSUM_EXTENSION),
        asset_checksum,
    )?;
    Ok(())
}

fn write_asset_signature(signing_key_file: &Path, asset_path: &Path) -> Result<(), TaskFailure> {
    run_tool(
        "minisign",
        Command::new("minisign")
            .arg("-S")
            .arg("-s")
            .arg(signing_key_file)
            .arg("-m")
            .arg(asset_path)
            .arg("-x")
            .arg(asset_path.with_added_extension(SIGNATURE_EXTENSION)),
    )
}

fn signing_key_file() -> Result<tempfile::NamedTempFile, TaskFailure> {
    let mut key_file = tempfile::NamedTempFile::new()?;
    key_file.write_all(required_variable(SIGNING_KEY_VARIABLE)?.as_bytes())?;
    key_file.as_file().sync_all()?;
    Ok(key_file)
}

pub fn upload_release_assets(assets_directory: &Path) -> Result<(), TaskFailure> {
    let signing_key = signing_key_file()?;
    let released_files = regular_files_sorted(assets_directory)?
        .into_iter()
        .filter(|file_path| !is_release_metadata_file(file_path))
        .collect::<Vec<_>>();
    released_files.iter().try_for_each(|asset_path| {
        write_asset_checksum(assets_directory, asset_path)?;
        write_asset_signature(signing_key.path(), asset_path)
    })?;
    let repository_name = required_variable("CI_REPO")?;
    let release_tag = required_variable("CI_COMMIT_TAG")?;
    log_tea_into_bot_account()?;
    let release_assets = regular_files_sorted(assets_directory)?;
    run_tool(
        "tea",
        Command::new("tea")
            .args([
                "releases",
                "assets",
                "create",
                "--login",
                TEA_LOGIN_NAME,
                "--repo",
                &repository_name,
                &release_tag,
            ])
            .args(
                release_assets
                    .iter()
                    .filter_map(|asset_path| asset_path.file_name()),
            )
            .current_dir(assets_directory),
    )
}

#[cfg(test)]
mod release_publishing_tests {
    use super::*;

    #[test]
    fn recognises_checksum_and_signature_files_by_extension() {
        assert!(is_release_metadata_file(Path::new(
            "dist/lazaret_0.2.0_amd64.deb.sha256"
        )));
        assert!(is_release_metadata_file(Path::new(
            "dist/lazaret_0.2.0_amd64.deb.minisig"
        )));
        assert!(!is_release_metadata_file(Path::new(
            "dist/lazaret_0.2.0_amd64.deb"
        )));
        assert!(!is_release_metadata_file(Path::new(
            "dist/lazaret-x86_64.flatpak"
        )));
    }

    #[test]
    fn reads_the_version_of_the_package_section_after_the_workspace() {
        let manifest_text = "[workspace]\nmembers = [\"xtask\"]\n\n[workspace.package]\nversion = \
                             \"9.9.9\"\n\n[package]\nname = \"lazaret\"\nversion = \"0.1.0\"\n";
        assert_eq!(manifest_package_version(manifest_text), Some("0.1.0"));
    }

    #[test]
    fn ignores_dependency_versions_written_inline() {
        let manifest_text = "[dependencies]\nclap = { version = \"4\" }\n";
        assert_eq!(manifest_package_version(manifest_text), None);
    }

    #[test]
    fn finds_generated_version_section() {
        let changelog_text = "## [Unreleased]\n\n## [0.1.0](https://example.org) - 2026-11-02\n";
        assert!(changelog_has_version_section(changelog_text, "0.1.0"));
    }

    #[test]
    fn rejects_changelog_without_version_section() {
        assert!(!changelog_has_version_section(
            "# Changelog\n\n## [Unreleased]\n",
            "0.1.0"
        ));
    }

    #[test]
    fn does_not_match_longer_version_with_same_prefix() {
        assert!(!changelog_has_version_section(
            "## [0.1.01] - 2026-11-02\n",
            "0.1.0"
        ));
    }
}
