use std::time::Duration;

use super::{
    LONG_WAIT,
    clamd_settings::{
        MAX_THREADS, MERGED_CONFIG, OVERRIDES_FILE, SettingsChange, answer_of, apply_as_root,
        config_seen_by_clamd, merge_again, merged_config, permanent_override,
        start_apply_in_session, stored_overrides, wait_for_clamd,
    },
};
use crate::vm_journeys::harness::{
    journey_progress::{SAMPLE_PASSWORD, announce_step},
    sway_windows::{answer_password_prompt, dismiss_password_prompt},
    system_commands::{
        CommandOutcome, as_root, journal_text, root_file_text, root_path_exists, settle,
        wait_until, write_root_file,
    },
};

const PRIVILEGED_UNITS: [&str; 3] = [
    "lazaret-clamav-admin.service",
    "lazaret-settings-merge.service",
    "lazaret-reduction-expiry.service",
];
const MAXIMUM_EXPOSURE: f64 = 3.0;
const CLAMD_SOCKET: &str = "/run/clamav/clamd.ctl";
const REDUCTION_LIFETIME: Duration = Duration::from_secs(35);

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

fn assert_applied(call_outcome: &CommandOutcome, expected_answer: &str) {
    assert!(
        call_outcome.exit_status.success()
            && call_outcome.combined_output.contains(expected_answer),
        "expected the admin service to answer {expected_answer:?}:\n{}",
        call_outcome.combined_output
    );
}

pub fn privileged_units_run_sandboxed() {
    announce_step("the privileged units run sandboxed");
    for privileged_unit in PRIVILEGED_UNITS {
        let exposure_report =
            as_root(&["systemd-analyze", "security", "--no-pager", privileged_unit]);
        let Some(unit_exposure) = exposure_score(&exposure_report) else {
            panic!("no exposure score for {privileged_unit}:\n{exposure_report}");
        };
        assert!(
            unit_exposure <= MAXIMUM_EXPOSURE,
            "{privileged_unit} exposure {unit_exposure}:\n{exposure_report}"
        );
    }
}

pub fn socket_survives_daemon_restart() {
    announce_step("the socket survives a daemon restart");
    assert!(root_path_exists(CLAMD_SOCKET), "no clamd socket");
    as_root(&["systemctl", "restart", "clamav-daemon.service"]);
    assert!(
        root_path_exists(CLAMD_SOCKET),
        "the socket disappeared with the restart"
    );
    wait_for_clamd();
}

pub fn daemon_sees_merged_configuration() {
    announce_step("live tuning is on by default and the daemon sees the merged configuration");
    assert!(
        root_file_text("/etc/lazaret/tunable-settings")
            .lines()
            .any(|setting_line| setting_line == "DetectPUA"),
        "DetectPUA is not tunable"
    );
    let daemon_unit = as_root(&["systemctl", "cat", "clamav-daemon.service"]);
    assert!(
        daemon_unit.contains(&format!(
            "BindReadOnlyPaths=-{MERGED_CONFIG}:/etc/clamav/clamd.conf"
        )),
        "clamd does not see the merged configuration"
    );
    assert!(
        !daemon_unit.contains("--config-file"),
        "clamd is started with --config-file"
    );
    let merged = merged_config();
    assert!(
        merged.contains("DatabaseDirectory"),
        "the merged configuration is incomplete"
    );
    assert_eq!(
        config_seen_by_clamd(),
        merged,
        "clamd sees another configuration"
    );
}

pub fn user_change_asks_for_password_and_applies() {
    announce_step("a user change asks for a password, shows the change, and applies");
    let apply_call = start_apply_in_session(&SettingsChange::SetValue {
        setting_name: MAX_THREADS,
        toml_value: "7",
    });
    answer_password_prompt(SAMPLE_PASSWORD);
    let call_answer = answer_of(apply_call);
    assert!(call_answer.contains("applied"), "{call_answer}");
    assert!(
        merged_config().contains("MaxThreads 7"),
        "the change did not reach the merged configuration"
    );
    wait_for_clamd();
}

pub fn second_change_asks_for_password_again() {
    announce_step("authorization is not retained, so a second change asks again");
    let apply_call = start_apply_in_session(&SettingsChange::SetValue {
        setting_name: MAX_THREADS,
        toml_value: "9",
    });
    dismiss_password_prompt();
    let call_answer = answer_of(apply_call);
    assert!(
        !call_answer.contains("applied"),
        "applied without a new password: {call_answer}"
    );
    assert!(
        !merged_config().contains("MaxThreads 9"),
        "a refused change reached clamd"
    );
}

pub fn protected_and_unlisted_settings_are_refused() {
    announce_step("protected and unlisted settings are refused by the admin service");
    for (setting_name, toml_value) in [
        ("VirusEvent", "\"/bin/sh\""),
        ("User", "\"root\""),
        ("MaxFileSize", "\"1M\""),
        (MAX_THREADS, "\"many\""),
    ] {
        let refused_outcome = apply_as_root(&SettingsChange::SetValue {
            setting_name,
            toml_value,
        });
        assert!(
            !refused_outcome.exit_status.success(),
            "{setting_name} was accepted: {}",
            refused_outcome.combined_output
        );
    }
    let merged = merged_config();
    assert!(
        !merged.contains("VirusEvent")
            && !merged.contains("User root")
            && !merged.contains("MaxFileSize"),
        "{merged}"
    );
}

pub fn planted_protected_key_falls_back_to_base() {
    announce_step(
        "a protected key planted in the stored changes makes the merge fall back to the base",
    );
    write_root_file(
        OVERRIDES_FILE,
        &[
            permanent_override("VirusEvent", "\"/bin/sh\""),
            permanent_override(MAX_THREADS, "5"),
        ]
        .join("\n"),
    );
    merge_again();
    let merged = merged_config();
    assert!(
        !merged.contains("VirusEvent") && !merged.contains("MaxThreads 5"),
        "{merged}"
    );
    assert!(merged.contains("MaxThreads 4"), "{merged}");
}

pub fn clamd_starts_with_unreadable_changes() {
    announce_step("clamd still starts when the stored changes cannot be read");
    write_root_file(OVERRIDES_FILE, "not toml at all\n\0\0");
    merge_again();
}

pub fn failed_restart_restores_previous_settings() {
    announce_step("a restart that fails restores the previous settings");
    write_root_file(OVERRIDES_FILE, &permanent_override(MAX_THREADS, "6"));
    merge_again();
    let rollback_outcome = apply_as_root(&SettingsChange::SetValue {
        setting_name: MAX_THREADS,
        toml_value: "3",
    });
    assert_applied(&rollback_outcome, "rolled-back");
    assert!(
        stored_overrides().contains("value = 6"),
        "the stored changes were not restored"
    );
    assert!(
        !merged_config().contains("MaxThreads 3"),
        "the failed value stayed in the merged configuration"
    );
    wait_for_clamd();
}

pub fn protection_reduction_ends_on_its_own() {
    announce_step("a change that reduces protection ends on its own");
    let reduction_outcome = apply_as_root(&SettingsChange::SetValue {
        setting_name: "DetectPUA",
        toml_value: "false",
    });
    assert_applied(&reduction_outcome, "applied");
    let overrides = stored_overrides();
    assert!(
        overrides.contains("temporary-reduction"),
        "stored:\n{overrides}"
    );
    assert!(
        config_seen_by_clamd().contains("DetectPUA no"),
        "clamd did not get the reduction"
    );
    settle(REDUCTION_LIFETIME);
    as_root(&["systemctl", "start", "lazaret-reduction-expiry.service"]);
    wait_for_clamd();
    assert!(
        !config_seen_by_clamd().contains("DetectPUA no"),
        "the reduction did not end"
    );
    let expiry_journal = journal_text(&["LAZARET_ACTION=expire-reductions", "-o", "verbose"]);
    assert!(
        expiry_journal.contains("DetectPUA"),
        "the expiry did not record DetectPUA:\n{expiry_journal}"
    );
}

pub fn back_to_system_removes_change() {
    announce_step("going back to the system configuration removes the change");
    let reset_outcome = apply_as_root(&SettingsChange::BackToSystem {
        setting_name: MAX_THREADS,
    });
    assert_applied(&reset_outcome, "applied");
    wait_for_clamd();
    assert!(
        config_seen_by_clamd().contains("MaxThreads 4"),
        "clamd kept the change"
    );
}

pub fn rebuild_reaches_clamd_without_reboot() {
    announce_step("a rebuild that changes the base reaches clamd without a reboot");
    as_root(&[
        "/run/current-system/specialisation/newBase/bin/switch-to-configuration",
        "test",
    ]);
    wait_until("MaxThreads 11 from the new base", LONG_WAIT, || {
        merged_config().contains("MaxThreads 11")
    });
    wait_for_clamd();
    assert!(
        config_seen_by_clamd().contains("MaxThreads 11"),
        "clamd did not get the new base"
    );
}
