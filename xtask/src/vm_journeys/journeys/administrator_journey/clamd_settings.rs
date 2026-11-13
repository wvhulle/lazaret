use std::{fs, io, os::unix::fs::PermissionsExt, path::PathBuf, time::Duration};

use super::{LONG_WAIT, SHORT_WAIT};
use crate::vm_journeys::harness::system_commands::{
    CommandOutcome, as_root, as_root_outcome, command_succeeds, root_file_text,
    root_file_text_or_empty, succeed, wait_until,
};

pub const MAX_THREADS: &str = "MaxThreads";
pub const MERGED_CONFIG: &str = "/run/lazaret/clamd.conf";
pub const OVERRIDES_FILE: &str = "/var/lib/lazaret/live-overrides.toml";
const ADMIN_BUS_NAME: &str = "page.codeberg.wvhulle.Lazaret.ClamavAdmin";
const ADMIN_OBJECT_PATH: &str = "/page/codeberg/wvhulle/Lazaret/ClamavAdmin";
const SESSION_CALL_FINISHED: &str = "session call finished";
const SESSION_CALL_WAIT: Duration = Duration::from_secs(240);

pub enum SettingsChange<'setting> {
    SetValue {
        setting_name: &'setting str,
        toml_value: &'setting str,
    },
    BackToSystem {
        setting_name: &'setting str,
    },
}

impl SettingsChange<'_> {
    fn change_document(&self) -> String {
        match self {
            Self::SetValue {
                setting_name,
                toml_value,
            } => format!(
                "change = \"set-values\"\n\n[setting_values]\n{setting_name} = {toml_value}\n"
            ),
            Self::BackToSystem { setting_name } => {
                format!("change = \"back-to-system\"\nsetting_names = [\"{setting_name}\"]\n")
            }
        }
    }

    fn call_arguments(&self) -> Vec<String> {
        [
            "--timeout=400",
            "call",
            ADMIN_BUS_NAME,
            ADMIN_OBJECT_PATH,
            ADMIN_BUS_NAME,
            "ApplyChanges",
            "s",
        ]
        .into_iter()
        .map(str::to_owned)
        .chain(std::iter::once(self.change_document()))
        .collect()
    }
}

pub fn permanent_override(setting_name: &str, toml_value: &str) -> String {
    [
        format!("[{setting_name}]"),
        "state = \"permanent\"".to_owned(),
        format!("value = {toml_value}"),
        String::new(),
    ]
    .join("\n")
}

fn reset_start_limits() {
    as_root(&[
        "systemctl",
        "reset-failed",
        "clamav-daemon.service",
        "lazaret-settings-merge.service",
    ]);
}

pub fn apply_as_root(settings_change: &SettingsChange<'_>) -> CommandOutcome {
    reset_start_limits();
    let call_arguments = settings_change.call_arguments();
    let root_arguments = std::iter::once("busctl")
        .chain(call_arguments.iter().map(String::as_str))
        .collect::<Vec<_>>();
    as_root_outcome(&root_arguments)
}

pub struct SessionApplyCall {
    answer_path: PathBuf,
}

fn write_session_call_script(settings_change: &SettingsChange<'_>) -> io::Result<SessionApplyCall> {
    let call_directory = std::env::temp_dir().join("vm-journey-session-calls");
    fs::create_dir_all(&call_directory)?;
    let answer_path = call_directory.join("answer");
    let script_path = call_directory.join("apply.sh");
    let document_path = call_directory.join("change.toml");
    let _stale_answer = fs::remove_file(&answer_path);
    fs::write(&document_path, settings_change.change_document())?;
    let busctl_arguments = settings_change.call_arguments();
    let quoted_prefix = busctl_arguments[..busctl_arguments.len() - 1].join(" ");
    fs::write(
        &script_path,
        format!(
            "#!/bin/sh\n(busctl {quoted_prefix} \"$(cat {document})\" 2>&1; echo \
             '{SESSION_CALL_FINISHED}') > {answer}\n",
            document = document_path.display(),
            answer = answer_path.display()
        ),
    )?;
    fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755))?;
    succeed("swaymsg", &["exec", &script_path.to_string_lossy()]);
    Ok(SessionApplyCall { answer_path })
}

pub fn start_apply_in_session(settings_change: &SettingsChange<'_>) -> SessionApplyCall {
    write_session_call_script(settings_change).unwrap_or_else(|script_failure| {
        panic!("the session call script could not be written: {script_failure}")
    })
}

pub fn answer_of(session_call: SessionApplyCall) -> String {
    wait_until("the session call answering", SESSION_CALL_WAIT, || {
        fs::read_to_string(&session_call.answer_path)
            .is_ok_and(|call_answer| call_answer.contains(SESSION_CALL_FINISHED))
    });
    fs::read_to_string(&session_call.answer_path).unwrap_or_else(|read_failure| {
        panic!(
            "{} is unreadable: {read_failure}",
            session_call.answer_path.display()
        )
    })
}

pub fn merged_config() -> String {
    root_file_text(MERGED_CONFIG)
}

pub fn stored_overrides() -> String {
    root_file_text_or_empty(OVERRIDES_FILE)
}

pub fn config_seen_by_clamd() -> String {
    let clamd_process = as_root(&[
        "systemctl",
        "show",
        "--property=MainPID",
        "--value",
        "clamav-daemon.service",
    ]);
    as_root(&[
        "nsenter",
        "--target",
        clamd_process.trim(),
        "--mount",
        "cat",
        "/etc/clamav/clamd.conf",
    ])
}

pub fn wait_for_clamd() {
    wait_until(
        "clamd answering on the merged configuration",
        LONG_WAIT,
        || {
            command_succeeds(
                "clamdscan",
                &[&format!("--config-file={MERGED_CONFIG}"), "--ping", "1"],
            )
        },
    );
}

pub fn merge_again() {
    reset_start_limits();
    as_root(&["systemctl", "restart", "lazaret-settings-merge.service"]);
    reset_start_limits();
    as_root(&["systemctl", "start", "clamav-daemon.service"]);
    wait_for_clamd();
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScanVerdict {
    Found,
    Clean,
}

pub fn root_scan_verdict(file_path: &str) -> ScanVerdict {
    if as_root_outcome(&["clamdscan", "--no-summary", "--fdpass", file_path])
        .combined_output
        .contains("FOUND")
    {
        ScanVerdict::Found
    } else {
        ScanVerdict::Clean
    }
}

pub fn wait_for_root_scan_verdict(file_path: &str, expected_verdict: ScanVerdict) {
    wait_until(
        &format!("clamd reporting {file_path} as {expected_verdict:?}"),
        SHORT_WAIT,
        || root_scan_verdict(file_path) == expected_verdict,
    );
}
