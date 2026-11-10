use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
    process::Command,
};

use relm4::gtk::glib;
use zbus::zvariant::Value;

const FLATPAK_INFO_PATH: &str = "/.flatpak-info";
const HOST_DATA_HOME_VARIABLE: &str = "HOST_XDG_DATA_HOME";
const DEFAULT_DATA_HOME_IN_HOME: &str = ".local/share";
const HOST_ROOT_IN_SANDBOX: &str = "/run/host";
const FLATPAK_SPAWN_PROGRAM: &str = "flatpak-spawn";
const PORTAL_BUS_NAME: &str = "org.freedesktop.portal.Desktop";
const PORTAL_OBJECT_PATH: &str = "/org/freedesktop/portal/desktop";
const BACKGROUND_PORTAL_INTERFACE: &str = "org.freedesktop.portal.Background";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SandboxKind {
    FlatpakSandbox,
    HostSystem,
}

impl SandboxKind {
    pub fn detect() -> Self {
        if Path::new(FLATPAK_INFO_PATH).exists() {
            Self::FlatpakSandbox
        } else {
            Self::HostSystem
        }
    }
}

pub fn host_path_for_sandbox_path(sandbox_path: &Path) -> PathBuf {
    match sandbox_path.strip_prefix(HOST_ROOT_IN_SANDBOX) {
        Ok(host_relative_path) => Path::new("/").join(host_relative_path),
        Err(_outside_host_root) => sandbox_path.to_path_buf(),
    }
}

const HOST_DIRECTORIES_UNDER_RUN_HOST: [&str; 2] = ["/etc", "/usr"];
pub const HOST_PROGRAM_DIRECTORIES: [&str; 6] = [
    "/usr/local/bin",
    "/usr/bin",
    "/bin",
    "/usr/local/sbin",
    "/usr/sbin",
    "/sbin",
];

fn run_host_path_for(host_path: &Path) -> PathBuf {
    if HOST_DIRECTORIES_UNDER_RUN_HOST
        .iter()
        .any(|host_directory| host_path.starts_with(host_directory))
    {
        Path::new(HOST_ROOT_IN_SANDBOX).join(host_path.strip_prefix("/").unwrap_or(host_path))
    } else {
        host_path.to_path_buf()
    }
}

pub fn readable_host_path(sandbox_kind: SandboxKind, host_path: impl AsRef<Path>) -> PathBuf {
    match sandbox_kind {
        SandboxKind::FlatpakSandbox => run_host_path_for(host_path.as_ref()),
        SandboxKind::HostSystem => host_path.as_ref().to_path_buf(),
    }
}

pub fn scanner_visible_path(sandbox_kind: SandboxKind, sandbox_path: &Path) -> PathBuf {
    match sandbox_kind {
        SandboxKind::FlatpakSandbox => host_path_for_sandbox_path(sandbox_path),
        SandboxKind::HostSystem => sandbox_path.to_path_buf(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgramEnvironmentVariable {
    pub variable_name: &'static str,
    pub variable_value: &'static str,
}

pub const C_LOCALE_ENVIRONMENT: ProgramEnvironmentVariable = ProgramEnvironmentVariable {
    variable_name: "LC_ALL",
    variable_value: "C",
};

fn flatpak_spawn_arguments(
    program_path: &Path,
    environment_variables: &[ProgramEnvironmentVariable],
) -> Vec<OsString> {
    let mut spawn_arguments = vec![OsString::from("--host"), OsString::from("--watch-bus")];
    spawn_arguments.extend(environment_variables.iter().map(
        |ProgramEnvironmentVariable {
             variable_name,
             variable_value,
         }| OsString::from(format!("--env={variable_name}={variable_value}")),
    ));
    spawn_arguments.push(OsString::from(program_path));
    spawn_arguments
}

fn command_for_sandbox(
    sandbox_kind: SandboxKind,
    program_path: &Path,
    environment_variables: &[ProgramEnvironmentVariable],
) -> Command {
    match sandbox_kind {
        SandboxKind::FlatpakSandbox => {
            let mut spawn_command = Command::new(FLATPAK_SPAWN_PROGRAM);
            spawn_command.args(flatpak_spawn_arguments(program_path, environment_variables));
            spawn_command
        }
        SandboxKind::HostSystem => {
            let mut direct_command = Command::new(program_path);
            direct_command.envs(environment_variables.iter().map(
                |ProgramEnvironmentVariable {
                     variable_name,
                     variable_value,
                 }| (variable_name, variable_value),
            ));
            direct_command
        }
    }
}

pub fn host_program_command(
    sandbox_kind: SandboxKind,
    program_path: impl AsRef<OsStr>,
    environment_variables: &[ProgramEnvironmentVariable],
) -> Command {
    command_for_sandbox(
        sandbox_kind,
        Path::new(program_path.as_ref()),
        environment_variables,
    )
}

pub fn host_program_async_command(
    sandbox_kind: SandboxKind,
    program_path: impl AsRef<OsStr>,
    environment_variables: &[ProgramEnvironmentVariable],
) -> tokio::process::Command {
    tokio::process::Command::from(host_program_command(
        sandbox_kind,
        program_path,
        environment_variables,
    ))
}

pub fn host_user_data_directory(sandbox_kind: SandboxKind) -> PathBuf {
    match sandbox_kind {
        SandboxKind::HostSystem => glib::user_data_dir(),
        SandboxKind::FlatpakSandbox => std::env::var_os(HOST_DATA_HOME_VARIABLE)
            .filter(|host_data_home| !host_data_home.is_empty())
            .map_or_else(
                || glib::home_dir().join(DEFAULT_DATA_HOME_IN_HOME),
                PathBuf::from,
            ),
    }
}

pub fn sandboxed_launcher_words(application_id: &str) -> Vec<String> {
    vec![
        "flatpak".to_owned(),
        "run".to_owned(),
        "--command=lazaret".to_owned(),
        application_id.to_owned(),
    ]
}

pub async fn request_background_autostart(autostart_enabled: bool) -> zbus::Result<()> {
    let session_connection = zbus::Connection::session().await?;
    let mut request_options: HashMap<&str, Value<'_>> = HashMap::new();
    request_options.insert(
        "reason",
        Value::from("Raise alerts when ClamAV detects malware"),
    );
    request_options.insert("autostart", Value::from(autostart_enabled));
    request_options.insert(
        "commandline",
        Value::from(vec!["lazaret".to_owned(), "--background".to_owned()]),
    );
    session_connection
        .call_method(
            Some(PORTAL_BUS_NAME),
            PORTAL_OBJECT_PATH,
            Some(BACKGROUND_PORTAL_INTERFACE),
            "RequestBackground",
            &("", request_options),
        )
        .await?;
    Ok(())
}

#[cfg(test)]
mod flatpak_sandbox_tests {
    use super::*;

    #[test]
    fn host_directories_seen_through_run_host_map_back_to_host_paths() {
        assert_eq!(
            host_path_for_sandbox_path(Path::new("/run/host/usr/share/doc")),
            PathBuf::from("/usr/share/doc")
        );
        assert_eq!(
            host_path_for_sandbox_path(Path::new("/home/user/Downloads")),
            PathBuf::from("/home/user/Downloads")
        );
        assert_eq!(
            host_path_for_sandbox_path(Path::new("/run/hostile/file")),
            PathBuf::from("/run/hostile/file")
        );
    }

    #[test]
    fn host_configuration_is_read_through_run_host() {
        assert_eq!(
            run_host_path_for(Path::new("/etc/clamav/clamd.conf")),
            PathBuf::from("/run/host/etc/clamav/clamd.conf")
        );
        assert_eq!(
            run_host_path_for(Path::new("/usr/local/share/clamav")),
            PathBuf::from("/run/host/usr/local/share/clamav")
        );
        assert_eq!(
            run_host_path_for(Path::new("/var/lib/clamav")),
            PathBuf::from("/var/lib/clamav")
        );
        assert_eq!(
            run_host_path_for(Path::new("/etcetera/file")),
            PathBuf::from("/etcetera/file")
        );
    }

    #[test]
    fn sandboxed_programs_run_on_the_host_with_their_environment() {
        let spawn_command = command_for_sandbox(
            SandboxKind::FlatpakSandbox,
            Path::new("clamscan"),
            &[C_LOCALE_ENVIRONMENT],
        );
        assert_eq!(spawn_command.get_program(), FLATPAK_SPAWN_PROGRAM);
        assert_eq!(
            spawn_command.get_args().collect::<Vec<_>>(),
            ["--host", "--watch-bus", "--env=LC_ALL=C", "clamscan"]
        );
    }

    #[test]
    fn programs_outside_the_sandbox_run_directly() {
        let direct_command = command_for_sandbox(
            SandboxKind::HostSystem,
            Path::new("clamscan"),
            &[C_LOCALE_ENVIRONMENT],
        );
        assert_eq!(direct_command.get_program(), "clamscan");
        assert_eq!(direct_command.get_args().count(), 0);
        assert_eq!(
            direct_command.get_envs().collect::<Vec<_>>(),
            [(OsStr::new("LC_ALL"), Some(OsStr::new("C")))]
        );
    }

    #[test]
    fn scheduled_scans_in_the_sandbox_start_through_flatpak_run() {
        assert_eq!(
            sandboxed_launcher_words("page.codeberg.wvhulle.Lazaret"),
            [
                "flatpak",
                "run",
                "--command=lazaret",
                "page.codeberg.wvhulle.Lazaret"
            ]
        );
    }
}
