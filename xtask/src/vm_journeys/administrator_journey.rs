use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    time::Duration,
};

use gtk_scene_capture::accessible_widget_lookup::{NameMatch, WidgetRole, WidgetTarget};

use crate::vm_journeys::{
    desktop_environment::{
        ApplicationPage, ChordKey, KeyInput, answer_password_prompt, dismiss_password_prompt,
        notifications_mentioning, scan_with_lazaret, send_keys, show_page, take_screenshot,
        wait_for_lazaret_service, wait_for_notification,
    },
    fixture_corpus::FixtureSample,
    journey_failure::{JourneyFailure, ensure},
    journey_run::{JourneyRun, SAMPLE_HOME, SAMPLE_PASSWORD},
    session_widgets::{button, in_alert, in_expanded_row, in_row, label_containing, row_titled},
    system_commands::{
        CommandOutcome, as_root, as_root_outcome, journal_text, root_file_text, root_path_exists,
        run_command, settle, succeed, wait_until, write_root_file,
    },
    ui_messages::{findings, scans, text_before_placeholder, widgets},
};

const SCAN_UNIT: &str = "clamdscan.service";
const SCANNED_FILE: &str = "/var/lib/sample-scan/dropper.bin";
const TRUSTED_FILES_LIST: &str = "/var/lib/clamav/lazaret-local.sfp";
const IGNORED_SIGNATURES_LIST: &str = "/var/lib/clamav/lazaret-local.ign2";
const FOREIGN_ALLOWLIST: &str = "local.fp";
const IGNORED_SIGNATURE: &str = "Doc.Fixture.FalsePositive-1";
const ROOT_EICAR_COPY: &str = "/root/invoice-copy.exe";
const ROOT_DOCUMENT_COPY: &str = "/root/payroll-copy.docm";
const MAX_THREADS: &str = "MaxThreads";
const MERGED_CONFIG: &str = "/run/lazaret/clamd.conf";
const OVERRIDES_FILE: &str = "/var/lib/lazaret/live-overrides.toml";
const ADMIN_BUS_NAME: &str = "page.codeberg.wvhulle.Lazaret.ClamavAdmin";
const ADMIN_OBJECT_PATH: &str = "/page/codeberg/wvhulle/Lazaret/ClamavAdmin";
const SCHEDULE_TIMER: &str = ".config/systemd/user/lazaret-scheduled-scan.timer";
const PRIVILEGED_UNITS: [&str; 3] = [
    "lazaret-clamav-admin.service",
    "lazaret-settings-merge.service",
    "lazaret-reduction-expiry.service",
];
const MAXIMUM_EXPOSURE: f64 = 3.0;
const SHORT_WAIT: Duration = Duration::from_secs(60);
const LONG_WAIT: Duration = Duration::from_secs(120);

enum SettingsChange<'setting> {
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

fn permanent_override(setting_name: &str, toml_value: &str) -> String {
    [
        format!("[{setting_name}]"),
        "state = \"permanent\"".to_owned(),
        format!("value = {toml_value}"),
        String::new(),
    ]
    .join("\n")
}

fn reset_start_limits() -> Result<(), JourneyFailure> {
    as_root(&[
        "systemctl",
        "reset-failed",
        "clamav-daemon.service",
        "lazaret-settings-merge.service",
    ])
    .map(|_reset| ())
}

fn apply_as_root(settings_change: &SettingsChange<'_>) -> Result<CommandOutcome, JourneyFailure> {
    reset_start_limits()?;
    let call_arguments = settings_change.call_arguments();
    let root_arguments = std::iter::once("busctl")
        .chain(call_arguments.iter().map(String::as_str))
        .collect::<Vec<_>>();
    as_root_outcome(&root_arguments)
}

struct SessionApplyCall {
    answer_path: PathBuf,
}

const SESSION_CALL_FINISHED: &str = "session call finished";

fn start_apply_in_session(
    settings_change: &SettingsChange<'_>,
) -> Result<SessionApplyCall, JourneyFailure> {
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
    succeed("swaymsg", &["exec", &script_path.to_string_lossy()])?;
    Ok(SessionApplyCall { answer_path })
}

fn answer_of(session_call: SessionApplyCall) -> Result<String, JourneyFailure> {
    wait_until(
        "the session call answering",
        Duration::from_secs(240),
        || {
            Ok(fs::read_to_string(&session_call.answer_path)
                .is_ok_and(|call_answer| call_answer.contains(SESSION_CALL_FINISHED)))
        },
    )?;
    Ok(fs::read_to_string(&session_call.answer_path)?)
}

fn merged_config() -> Result<String, JourneyFailure> {
    root_file_text(MERGED_CONFIG)
}

fn stored_overrides() -> Result<String, JourneyFailure> {
    Ok(root_file_text(OVERRIDES_FILE).unwrap_or_default())
}

fn config_seen_by_clamd() -> Result<String, JourneyFailure> {
    let clamd_process = as_root(&[
        "systemctl",
        "show",
        "--property=MainPID",
        "--value",
        "clamav-daemon.service",
    ])?;
    as_root(&[
        "nsenter",
        "--target",
        clamd_process.trim(),
        "--mount",
        "cat",
        "/etc/clamav/clamd.conf",
    ])
}

fn wait_for_clamd() -> Result<(), JourneyFailure> {
    wait_until(
        "clamd answering on the merged configuration",
        LONG_WAIT,
        || {
            Ok(run_command(
                "clamdscan",
                &[&format!("--config-file={MERGED_CONFIG}"), "--ping", "1"],
            )?
            .exit_status
            .success())
        },
    )
}

fn merge_again() -> Result<(), JourneyFailure> {
    reset_start_limits()?;
    as_root(&["systemctl", "restart", "lazaret-settings-merge.service"])?;
    reset_start_limits()?;
    as_root(&["systemctl", "start", "clamav-daemon.service"])?;
    wait_for_clamd()
}

fn root_scan_finds(file_path: &str) -> Result<bool, JourneyFailure> {
    Ok(
        as_root_outcome(&["clamdscan", "--no-summary", "--fdpass", file_path])?
            .combined_output
            .contains("FOUND"),
    )
}

fn wait_for_root_scan_verdict(file_path: &str, expect_found: bool) -> Result<(), JourneyFailure> {
    wait_until(
        &format!(
            "clamd reporting {file_path} as {}",
            if expect_found { "found" } else { "clean" }
        ),
        SHORT_WAIT,
        || Ok(root_scan_finds(file_path)? == expect_found),
    )
}

fn quarantine_with_lazaret(
    journey_run: &JourneyRun,
    file_path: &str,
) -> Result<(), JourneyFailure> {
    scan_with_lazaret(&[file_path])?;
    journey_run
        .widgets
        .press_within(&button(widgets::quarantine_all(1)), LONG_WAIT)?;
    journey_run.press(&in_alert(
        button(widgets::quarantine_label()),
        widgets::move_quarantine(),
    ))?;
    wait_until(
        &format!("{file_path} moving to quarantine"),
        Duration::from_secs(30),
        || Ok(!Path::new(file_path).exists()),
    )
}

fn open_quarantined_entry(
    journey_run: &mut JourneyRun,
    path_fragment: &str,
) -> Result<String, JourneyFailure> {
    show_page(ApplicationPage::Detections)?;
    show_page(ApplicationPage::Quarantine)?;
    let entry_title = journey_run
        .database
        .wait_for_quarantined_signature(path_fragment)?;
    let trust_button = in_expanded_row(button(findings::trust_label()), entry_title.clone());
    if !journey_run.widgets.is_shown(&trust_button) {
        journey_run.press(&row_titled(entry_title.clone()))?;
    }
    journey_run.await_shown(&trust_button)?;
    Ok(entry_title)
}

fn ask_to_trust_for_all_users(
    journey_run: &mut JourneyRun,
    path_fragment: &str,
) -> Result<(), JourneyFailure> {
    let entry_title = open_quarantined_entry(journey_run, path_fragment)?;
    journey_run.press(&in_expanded_row(
        button(findings::trust_label()),
        entry_title,
    ))?;
    journey_run.press(&in_alert(
        button(widgets::trust_for_all_users_button()),
        widgets::trust_file_heading(),
    ))
}

fn show_scan_engine_page(journey_run: &JourneyRun) -> Result<(), JourneyFailure> {
    show_page(ApplicationPage::ScanEngine)?;
    journey_run.await_shown(&row_titled(MAX_THREADS))
}

fn edit_max_threads(journey_run: &JourneyRun, new_value: u32) -> Result<(), JourneyFailure> {
    journey_run.press(&row_titled(MAX_THREADS))?;
    send_keys(&[
        KeyInput::ControlChord(ChordKey::SelectAll),
        KeyInput::TypedText(&new_value.to_string()),
        KeyInput::Enter,
    ])?;
    journey_run.await_shown(&WidgetTarget::alert(widgets::change_setting()))
}

fn exposure_score(exposure_report: &str) -> Option<f64> {
    exposure_report
        .lines()
        .last()?
        .split("level for")
        .last()?
        .split(':')
        .nth(1)?
        .split_whitespace()
        .next()?
        .parse()
        .ok()
}

pub fn run_administrator_journey() -> Result<(), JourneyFailure> {
    let eicar_sha256 = FixtureSample::EicarInvoice.content_sha256()?;
    let document_sha256 = FixtureSample::PayrollDocument.content_sha256()?;
    let trust_candidate = format!("{SAMPLE_HOME}/Documents/invoice.pdf.exe");
    let promoted_candidate = format!("{SAMPLE_HOME}/Documents/payroll-macro.docm");

    wait_for_lazaret_service()?;
    as_root(&["systemctl", "start", "clamav-daemon.service"])?;
    wait_for_clamd()?;
    as_root(&[
        "cp",
        &FixtureSample::EicarInvoice.source_path(),
        ROOT_EICAR_COPY,
    ])?;
    as_root(&[
        "cp",
        &FixtureSample::PayrollDocument.source_path(),
        ROOT_DOCUMENT_COPY,
    ])?;
    let mut journey_run = JourneyRun::start()?;

    journey_run.step(
        "an allowlist entry for all users is listed with its warning, next to a foreign allowlist",
        |journey_run| {
            ensure(!root_scan_finds(ROOT_DOCUMENT_COPY)?, || {
                "the ignored detection was reported".to_owned()
            })?;
            show_page(ApplicationPage::TrustedDetections)?;
            journey_run.await_shown(&row_titled(IGNORED_SIGNATURE))?;
            journey_run.await_shown(&row_titled(FOREIGN_ALLOWLIST))?;
            take_screenshot("allowlists_for_all_users");
            Ok(())
        },
    )?;

    journey_run.step(
        "reporting an ignored detection again needs the password and shrinks the list",
        |journey_run| {
            journey_run.press(&in_row(
                button(findings::remove_tooltip()),
                IGNORED_SIGNATURE,
            ))?;
            journey_run.press(&in_alert(
                button(widgets::report_again_button()),
                widgets::report_detection_again_heading(),
            ))?;
            answer_password_prompt(SAMPLE_PASSWORD)?;
            wait_until("the ignored detection leaving the list", SHORT_WAIT, || {
                Ok(!root_file_text(IGNORED_SIGNATURES_LIST)
                    .unwrap_or_default()
                    .contains(IGNORED_SIGNATURE))
            })?;
            wait_for_root_scan_verdict(ROOT_DOCUMENT_COPY, true)?;
            ensure(
                root_path_exists(&format!("/var/lib/clamav/{FOREIGN_ALLOWLIST}"))?,
                || "the foreign allowlist was removed".to_owned(),
            )
        },
    )?;

    journey_run.step(
        "cancelling or failing the password prompt trusts nothing",
        |journey_run| {
            FixtureSample::EicarInvoice.place_at(&trust_candidate)?;
            quarantine_with_lazaret(journey_run, &trust_candidate)?;
            ask_to_trust_for_all_users(journey_run, "invoice.pdf.exe")?;
            take_screenshot("trust_password_prompt");
            dismiss_password_prompt()?;
            ensure(!root_path_exists(TRUSTED_FILES_LIST)?, || {
                "a cancelled prompt trusted the file".to_owned()
            })?;
            ask_to_trust_for_all_users(journey_run, "invoice.pdf.exe")?;
            answer_password_prompt("not-the-password")?;
            dismiss_password_prompt()?;
            ensure(!root_path_exists(TRUSTED_FILES_LIST)?, || {
                "a wrong password trusted the file".to_owned()
            })?;
            ensure(root_scan_finds(ROOT_EICAR_COPY)?, || {
                "a refused trust change still hid the file".to_owned()
            })
        },
    )?;

    journey_run.step(
        "trusting a file for all users tells clamd and survives as a hash line",
        |journey_run| {
            ask_to_trust_for_all_users(journey_run, "invoice.pdf.exe")?;
            answer_password_prompt(SAMPLE_PASSWORD)?;
            let expected_line = format!("{eicar_sha256}:68:Lazaret.TrustedFile");
            wait_until(
                "the hash line in the trusted files list",
                SHORT_WAIT,
                || {
                    Ok(root_file_text(TRUSTED_FILES_LIST)
                        .unwrap_or_default()
                        .lines()
                        .any(|listed_line| listed_line == expected_line))
                },
            )?;
            wait_for_root_scan_verdict(ROOT_EICAR_COPY, false)?;
            ensure(
                as_root(&["stat", "-c", "%a %U", TRUSTED_FILES_LIST])?.trim() == "644 root",
                || "the trusted files list has the wrong mode or owner".to_owned(),
            )?;
            ensure(
                journal_text(&["-o", "verbose", "-u", "lazaret-clamav-admin.service"])?
                    .contains(&eicar_sha256),
                || "the admin service did not record the trust change".to_owned(),
            )?;
            show_page(ApplicationPage::TrustedDetections)?;
            journey_run.await_shown(&row_titled(eicar_sha256.clone()))?;
            take_screenshot("trusted_for_all_users");
            Ok(())
        },
    )?;

    journey_run.step(
        "removing the trust for all users reports the file again and drops the empty list",
        |journey_run| {
            journey_run.press(&in_row(
                button(findings::remove_tooltip()),
                eicar_sha256.clone(),
            ))?;
            journey_run.press(&in_alert(
                button(widgets::stop_trusting_button()),
                widgets::stop_trusting_file_heading(),
            ))?;
            answer_password_prompt(SAMPLE_PASSWORD)?;
            wait_until(
                "the empty trusted files list disappearing",
                SHORT_WAIT,
                || Ok(!root_path_exists(TRUSTED_FILES_LIST)?),
            )?;
            wait_for_root_scan_verdict(ROOT_EICAR_COPY, true)
        },
    )?;

    journey_run.step(
        "a file trusted for me can be promoted to all users from Trusted Detections",
        |journey_run| {
            FixtureSample::PayrollDocument.place_at(&promoted_candidate)?;
            quarantine_with_lazaret(journey_run, &promoted_candidate)?;
            let entry_title = open_quarantined_entry(journey_run, "payroll")?;
            journey_run.press(&in_expanded_row(
                button(findings::trust_label()),
                entry_title,
            ))?;
            journey_run.press(&in_alert(
                button(widgets::trust_for_me_button()),
                widgets::trust_file_heading(),
            ))?;
            wait_until("the document trusted for me", SHORT_WAIT, || {
                journey_run.database.is_trusted_content(&document_sha256)
            })?;
            show_page(ApplicationPage::TrustedDetections)?;
            journey_run.press(&in_row(
                button(findings::trust_for_all_users_tooltip()),
                document_sha256.clone(),
            ))?;
            journey_run.press(&in_alert(
                button(widgets::trust_for_all_users_button()),
                widgets::trust_file_heading(),
            ))?;
            answer_password_prompt(SAMPLE_PASSWORD)?;
            wait_until("the promoted hash line", SHORT_WAIT, || {
                Ok(root_file_text(TRUSTED_FILES_LIST)
                    .unwrap_or_default()
                    .starts_with(&format!("{document_sha256}:")))
            })?;
            wait_for_root_scan_verdict(ROOT_DOCUMENT_COPY, false)
        },
    )?;

    journey_run.step(
        "Run Now asks for a password and runs the system scan",
        |journey_run| {
            as_root(&[
                "cp",
                &FixtureSample::EicarInvoice.source_path(),
                SCANNED_FILE,
            ])?;
            show_page(ApplicationPage::ScheduledScans)?;
            journey_run.press(&button(scans::run_now_label()))?;
            answer_password_prompt(SAMPLE_PASSWORD)?;
            wait_until("the system scan finding the file", LONG_WAIT, || {
                Ok(journal_text(&["-u", SCAN_UNIT])?
                    .lines()
                    .any(|journal_line| {
                        journal_line.contains(SCANNED_FILE) && journal_line.ends_with("FOUND")
                    }))
            })?;
            wait_until("the system scan ending", SHORT_WAIT, || {
                Ok(
                    as_root(&["systemctl", "show", "-p", "Result", "--value", SCAN_UNIT])?.trim()
                        == "exit-code",
                )
            })?;
            let exit_status = as_root(&[
                "systemctl",
                "show",
                "-p",
                "ExecMainStatus",
                "--value",
                SCAN_UNIT,
            ])?;
            ensure(exit_status.trim() == "1", || {
                format!("a detection should exit 1: {exit_status}")
            })
        },
    )?;

    journey_run.step(
        "finding malware is not shown as a scanner failure",
        |journey_run| {
            take_screenshot("scan_finished_toast");
            journey_run.await_shown(&label_containing(scans::found_malware()))?;
            ensure(
                !journey_run
                    .widgets
                    .is_shown(&label_containing(scans::not_finish_cleanly())),
                || "a found threat was shown as an unclean scan".to_owned(),
            )
        },
    )?;

    journey_run.step(
        "the scan's journal line becomes a detection that is not called blocked",
        |journey_run| {
            let detection = journey_run
                .database
                .wait_for_stored_detection(SCANNED_FILE)?;
            ensure(
                detection.signature_name.starts_with("Win.Fixture.Agent-1"),
                || format!("{detection:?}"),
            )?;
            ensure(detection.detection_sources == "scheduled_scan", || {
                format!("wrong source: {detection:?}")
            })?;
            wait_for_notification(&text_before_placeholder(findings::malware_found_signature))?;
            ensure(
                notifications_mentioning(&text_before_placeholder(
                    findings::malware_blocked_signature,
                ))?
                .is_empty(),
                || "a scanned file was called blocked".to_owned(),
            )?;
            ensure(
                !journey_run
                    .widgets
                    .is_shown(&label_containing(scans::last_scan_ended_error())),
                || "a found threat was shown as a failed scan".to_owned(),
            )
        },
    )?;

    journey_run.step(
        "the Lazaret schedule is locked because the system has a timer",
        |journey_run| {
            show_page(ApplicationPage::ScheduledScans)?;
            journey_run.await_shown(&label_containing(text_before_placeholder(
                scans::system_already_runs_scheduled,
            )))?;
            let _locked_toggle = journey_run.widgets.press_within(
                &WidgetTarget::label(scans::enable_scheduled_scans_title()),
                Duration::from_secs(5),
            );
            let _locked_apply = journey_run
                .widgets
                .press_within(&button(scans::apply_label()), Duration::from_secs(5));
            ensure(
                !Path::new(&format!("{SAMPLE_HOME}/{SCHEDULE_TIMER}")).exists(),
                || "Lazaret wrote a second schedule".to_owned(),
            )
        },
    )?;

    journey_run.step(
        "settings the system owns are locked and copy a snippet",
        |journey_run| {
            show_page(ApplicationPage::ScanEngine)?;
            journey_run.await_shown(
                &WidgetTarget::new(WidgetRole::Image, widgets::administrator_not_copy())
                    .matching_names(NameMatch::ContainedName),
            )?;
            take_screenshot("scanner_engine_settings");
            Ok(())
        },
    )?;

    journey_run.step("signature sources have their own page", |journey_run| {
        show_page(ApplicationPage::SignatureSources)?;
        journey_run.widgets.await_shown_within(
            &label_containing(crate::vm_journeys::ui_messages::window::signature_sources()),
            SHORT_WAIT,
        )
    })?;

    journey_run.step("the privileged units run sandboxed", |_journey_run| {
        for privileged_unit in PRIVILEGED_UNITS {
            let exposure_report =
                as_root(&["systemd-analyze", "security", "--no-pager", privileged_unit])?;
            let unit_exposure = exposure_score(&exposure_report).ok_or_else(|| {
                JourneyFailure::ExpectationFailed(format!(
                    "no exposure score for {privileged_unit}"
                ))
            })?;
            ensure(unit_exposure <= MAXIMUM_EXPOSURE, || {
                format!("{privileged_unit} exposure {unit_exposure}:\n{exposure_report}")
            })?;
        }
        Ok(())
    })?;

    journey_run.step("the socket survives a daemon restart", |_journey_run| {
        ensure(root_path_exists("/run/clamav/clamd.ctl")?, || {
            "no clamd socket".to_owned()
        })?;
        as_root(&["systemctl", "restart", "clamav-daemon.service"])?;
        ensure(root_path_exists("/run/clamav/clamd.ctl")?, || {
            "the socket disappeared with the restart".to_owned()
        })?;
        wait_for_clamd()
    })?;

    journey_run.step(
        "live tuning is on by default and the daemon sees the merged configuration",
        |_journey_run| {
            ensure(
                root_file_text("/etc/lazaret/tunable-settings")?
                    .lines()
                    .any(|setting_line| setting_line == "DetectPUA"),
                || "DetectPUA is not tunable".to_owned(),
            )?;
            let daemon_unit = as_root(&["systemctl", "cat", "clamav-daemon.service"])?;
            ensure(
                daemon_unit.contains(&format!(
                    "BindReadOnlyPaths=-{MERGED_CONFIG}:/etc/clamav/clamd.conf"
                )),
                || "clamd does not see the merged configuration".to_owned(),
            )?;
            ensure(!daemon_unit.contains("--config-file"), || {
                "clamd is started with --config-file".to_owned()
            })?;
            let merged = merged_config()?;
            ensure(merged.contains("DatabaseDirectory"), || {
                "the merged configuration is incomplete".to_owned()
            })?;
            ensure(config_seen_by_clamd()? == merged, || {
                "clamd sees another configuration".to_owned()
            })
        },
    )?;

    journey_run.step(
        "a user change asks for a password, shows the change, and applies",
        |_journey_run| {
            let apply_process = start_apply_in_session(&SettingsChange::SetValue {
                setting_name: MAX_THREADS,
                toml_value: "7",
            })?;
            answer_password_prompt(SAMPLE_PASSWORD)?;
            let call_answer = answer_of(apply_process)?;
            ensure(call_answer.contains("applied"), || call_answer.clone())?;
            ensure(merged_config()?.contains("MaxThreads 7"), || {
                "the change did not reach the merged configuration".to_owned()
            })?;
            wait_for_clamd()
        },
    )?;

    journey_run.step(
        "authorization is not retained, so a second change asks again",
        |_journey_run| {
            let apply_process = start_apply_in_session(&SettingsChange::SetValue {
                setting_name: MAX_THREADS,
                toml_value: "9",
            })?;
            dismiss_password_prompt()?;
            let call_answer = answer_of(apply_process)?;
            ensure(!call_answer.contains("applied"), || {
                format!("applied without a new password: {call_answer}")
            })?;
            ensure(!merged_config()?.contains("MaxThreads 9"), || {
                "a refused change reached clamd".to_owned()
            })
        },
    )?;

    journey_run.step(
        "the Scanner Engine settings offer to unlock allowed settings",
        |journey_run| {
            wait_for_lazaret_service()?;
            show_scan_engine_page(journey_run)?;
            journey_run.await_shown(&label_containing(widgets::live_tunable_config(
                MERGED_CONFIG,
            )))?;
            take_screenshot("scanner_engine_locked");
            journey_run.await_shown(&WidgetTarget::toggle_button(widgets::unlock_label()))
        },
    )?;

    journey_run.step(
        "while locked, Copy configuration line only copies the line",
        |journey_run| {
            let merged_before_copy = merged_config()?;
            let overrides_before_copy = stored_overrides()?;
            journey_run.press(&in_row(button(MAX_THREADS), MAX_THREADS))?;
            journey_run.await_shown(&label_containing(
                widgets::nothing_applied_choose_description(),
            ))?;
            journey_run.press(&button(widgets::copy_line_label()))?;
            journey_run.widgets.await_shown_within(
                &label_containing(widgets::change_snippet_copied()),
                SHORT_WAIT,
            )?;
            take_screenshot("locked_copy");
            ensure(merged_config()? == merged_before_copy, || {
                "a locked copy changed clamd".to_owned()
            })?;
            ensure(stored_overrides()? == overrides_before_copy, || {
                "a locked copy changed the stored changes".to_owned()
            })
        },
    )?;

    journey_run.step(
        "unlocking exposes the allowed settings to assistive technology",
        |journey_run| {
            show_scan_engine_page(journey_run)?;
            journey_run.press(&WidgetTarget::toggle_button(widgets::unlock_label()))?;
            journey_run.await_shown(&WidgetTarget::toggle_button(widgets::lock_label()))?;
            take_screenshot("preferences_unlocked");
            journey_run
                .widgets
                .describe_ancestry_of_named(MAX_THREADS)
                .iter()
                .for_each(|ancestry_line| eprintln!("ancestry: {ancestry_line}"));
            journey_run.expect_behaviour(
                "an unlocked setting can be reached by its name",
                |journey_run| {
                    journey_run
                        .widgets
                        .await_shown_within(&row_titled(MAX_THREADS), Duration::from_secs(15))
                },
            );
            Ok(())
        },
    )?;

    if journey_run.widgets.is_shown(&row_titled(MAX_THREADS)) {
        journey_run.step(
            "editing and confirming applies through a real password prompt",
            |journey_run| {
                edit_max_threads(journey_run, 8)?;
                take_screenshot("confirm_change");
                journey_run.press(&in_alert(
                    button(widgets::apply_label()),
                    widgets::change_setting(),
                ))?;
                answer_password_prompt(SAMPLE_PASSWORD)?;
                wait_until(
                    "MaxThreads 8 in the merged configuration",
                    LONG_WAIT,
                    || Ok(merged_config()?.contains("MaxThreads 8")),
                )?;
                wait_for_clamd()
            },
        )?;

        journey_run.step(
            "a second change starts from the value just applied",
            |journey_run| {
                show_scan_engine_page(journey_run)?;
                edit_max_threads(journey_run, 10)?;
                journey_run.await_shown(&label_containing(
                    crate::vm_journeys::ui_messages::host::key_now_current_new(MAX_THREADS, 8, 10),
                ))?;
                take_screenshot("confirm_second_change");
                journey_run.press(&in_alert(
                    button(widgets::cancel_engine()),
                    widgets::change_setting(),
                ))
            },
        )?;
    }

    journey_run.step(
        "protected and unlisted settings are refused by the admin service",
        |_journey_run| {
            for (setting_name, toml_value) in [
                ("VirusEvent", "\"/bin/sh\""),
                ("User", "\"root\""),
                ("MaxFileSize", "\"1M\""),
                (MAX_THREADS, "\"many\""),
            ] {
                let refused_outcome = apply_as_root(&SettingsChange::SetValue {
                    setting_name,
                    toml_value,
                })?;
                ensure(!refused_outcome.exit_status.success(), || {
                    format!(
                        "{setting_name} was accepted: {}",
                        refused_outcome.combined_output
                    )
                })?;
            }
            let merged = merged_config()?;
            ensure(
                !merged.contains("VirusEvent")
                    && !merged.contains("User root")
                    && !merged.contains("MaxFileSize"),
                || merged.clone(),
            )
        },
    )?;

    journey_run.step(
        "a protected key planted in the stored changes makes the merge fall back to the base",
        |_journey_run| {
            write_root_file(
                OVERRIDES_FILE,
                &[
                    permanent_override("VirusEvent", "\"/bin/sh\""),
                    permanent_override(MAX_THREADS, "5"),
                ]
                .join("\n"),
            )?;
            merge_again()?;
            let merged = merged_config()?;
            ensure(
                !merged.contains("VirusEvent") && !merged.contains("MaxThreads 5"),
                || merged.clone(),
            )?;
            ensure(merged.contains("MaxThreads 4"), || merged.clone())
        },
    )?;

    journey_run.step(
        "clamd still starts when the stored changes cannot be read",
        |_journey_run| {
            write_root_file(OVERRIDES_FILE, "not toml at all\n\0\0")?;
            merge_again()
        },
    )?;

    journey_run.step(
        "a restart that fails restores the previous settings",
        |_journey_run| {
            write_root_file(OVERRIDES_FILE, &permanent_override(MAX_THREADS, "6"))?;
            merge_again()?;
            let rollback_outcome = apply_as_root(&SettingsChange::SetValue {
                setting_name: MAX_THREADS,
                toml_value: "3",
            })?;
            ensure(
                rollback_outcome.exit_status.success()
                    && rollback_outcome.combined_output.contains("rolled-back"),
                || rollback_outcome.combined_output.clone(),
            )?;
            ensure(stored_overrides()?.contains("value = 6"), || {
                "the stored changes were not restored".to_owned()
            })?;
            ensure(!merged_config()?.contains("MaxThreads 3"), || {
                "the failed value stayed in the merged configuration".to_owned()
            })?;
            wait_for_clamd()
        },
    )?;

    journey_run.step(
        "a change that reduces protection ends on its own",
        |_journey_run| {
            let reduction_outcome = apply_as_root(&SettingsChange::SetValue {
                setting_name: "DetectPUA",
                toml_value: "false",
            })?;
            ensure(
                reduction_outcome.exit_status.success()
                    && reduction_outcome.combined_output.contains("applied"),
                || reduction_outcome.combined_output.clone(),
            )?;
            let overrides = stored_overrides()?;
            ensure(overrides.contains("temporary-reduction"), || {
                format!("stored:\n{overrides}")
            })?;
            ensure(config_seen_by_clamd()?.contains("DetectPUA no"), || {
                "clamd did not get the reduction".to_owned()
            })?;
            settle(Duration::from_secs(35));
            as_root(&["systemctl", "start", "lazaret-reduction-expiry.service"])?;
            wait_for_clamd()?;
            ensure(!config_seen_by_clamd()?.contains("DetectPUA no"), || {
                "the reduction did not end".to_owned()
            })?;
            let expiry_journal =
                journal_text(&["LAZARET_ACTION=expire-reductions", "-o", "verbose"])?;
            ensure(expiry_journal.contains("DetectPUA"), || {
                format!("the expiry did not record DetectPUA:\n{expiry_journal}")
            })
        },
    )?;

    journey_run.step(
        "going back to the system configuration removes the change",
        |_journey_run| {
            let reset_outcome = apply_as_root(&SettingsChange::BackToSystem {
                setting_name: MAX_THREADS,
            })?;
            ensure(
                reset_outcome.exit_status.success()
                    && reset_outcome.combined_output.contains("applied"),
                || reset_outcome.combined_output.clone(),
            )?;
            wait_for_clamd()?;
            ensure(config_seen_by_clamd()?.contains("MaxThreads 4"), || {
                "clamd kept the change".to_owned()
            })
        },
    )?;

    journey_run.step(
        "a rebuild that changes the base reaches clamd without a reboot",
        |_journey_run| {
            as_root(&[
                "/run/current-system/specialisation/newBase/bin/switch-to-configuration",
                "test",
            ])?;
            wait_until("MaxThreads 11 from the new base", LONG_WAIT, || {
                Ok(merged_config()?.contains("MaxThreads 11"))
            })?;
            wait_for_clamd()?;
            ensure(config_seen_by_clamd()?.contains("MaxThreads 11"), || {
                "clamd did not get the new base".to_owned()
            })
        },
    )?;

    journey_run.finish()
}
