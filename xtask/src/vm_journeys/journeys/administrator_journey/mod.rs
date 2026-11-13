mod clamd_settings;
mod live_tuning_interface;
mod live_tuning_service;
mod system_scans;
mod trust_and_allowlists;

use std::time::Duration;

use crate::vm_journeys::harness::{
    fixture_corpus::FixtureSample, journey_progress::SAMPLE_HOME,
    lazaret_application::wait_for_lazaret_service, lazaret_database::LazaretDatabase,
    session_widgets::SessionWidgets, system_commands::as_root,
};

const ROOT_EICAR_COPY: &str = "/root/invoice-copy.exe";
const ROOT_DOCUMENT_COPY: &str = "/root/payroll-copy.docm";
const SHORT_WAIT: Duration = Duration::from_secs(60);
const LONG_WAIT: Duration = Duration::from_secs(120);

fn copy_samples_for_root() {
    as_root(&[
        "cp",
        &FixtureSample::EicarInvoice.source_path(),
        ROOT_EICAR_COPY,
    ]);
    as_root(&[
        "cp",
        &FixtureSample::PayrollDocument.source_path(),
        ROOT_DOCUMENT_COPY,
    ]);
}

pub fn run_administrator_journey() {
    let eicar_sha256 = FixtureSample::EicarInvoice.content_sha256();
    let document_sha256 = FixtureSample::PayrollDocument.content_sha256();

    wait_for_lazaret_service();
    as_root(&["systemctl", "start", "clamav-daemon.service"]);
    clamd_settings::wait_for_clamd();
    copy_samples_for_root();
    let session_widgets = SessionWidgets::connect();
    let database = LazaretDatabase::for_home(SAMPLE_HOME);

    trust_and_allowlists::allowlist_for_all_users_is_listed(&session_widgets);
    trust_and_allowlists::reporting_ignored_detection_again_needs_password(&session_widgets);
    trust_and_allowlists::refused_password_prompt_trusts_nothing(&session_widgets, &database);
    trust_and_allowlists::trusting_for_all_users_survives_as_hash_line(
        &session_widgets,
        &database,
        &eicar_sha256,
    );
    trust_and_allowlists::removing_trust_for_all_users_reports_file_again(
        &session_widgets,
        &eicar_sha256,
    );
    trust_and_allowlists::file_trusted_for_me_can_be_promoted(
        &session_widgets,
        &database,
        &document_sha256,
    );
    system_scans::run_now_asks_for_password_and_scans(&session_widgets);
    system_scans::found_malware_is_not_a_scanner_failure(&session_widgets);
    system_scans::scan_journal_line_becomes_detection(&session_widgets, &database);
    system_scans::lazaret_schedule_is_locked_by_system_timer(&session_widgets);
    system_scans::system_owned_settings_name_their_source(&session_widgets);
    system_scans::signature_sources_have_their_own_page(&session_widgets);
    live_tuning_service::privileged_units_run_sandboxed();
    live_tuning_service::socket_survives_daemon_restart();
    live_tuning_service::daemon_sees_merged_configuration();
    live_tuning_service::user_change_asks_for_password_and_applies();
    live_tuning_service::second_change_asks_for_password_again();
    live_tuning_interface::engine_settings_show_their_source(&session_widgets);
    live_tuning_interface::disallowed_setting_offers_only_configuration_line(&session_widgets);
    live_tuning_interface::allowed_settings_are_editable(&session_widgets);
    live_tuning_interface::confirmed_edit_applies_through_password_prompt(&session_widgets);
    live_tuning_interface::second_edit_starts_from_applied_value(&session_widgets);
    live_tuning_interface::changed_row_reverts_in_place(&session_widgets);
    live_tuning_service::protected_and_unlisted_settings_are_refused();
    live_tuning_service::planted_protected_key_falls_back_to_base();
    live_tuning_service::clamd_starts_with_unreadable_changes();
    live_tuning_service::failed_restart_restores_previous_settings();
    live_tuning_service::protection_reduction_ends_on_its_own();
    live_tuning_service::back_to_system_removes_change();
    live_tuning_service::rebuild_reaches_clamd_without_reboot();
}
