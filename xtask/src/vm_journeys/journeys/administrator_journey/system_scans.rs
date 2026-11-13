use std::{path::Path, time::Duration};

use gtk_scene_capture::accessible_widget_lookup::WidgetTarget;

use super::{LONG_WAIT, SHORT_WAIT};
use crate::vm_journeys::harness::{
    desktop_notifications::{notifications_mentioning, wait_for_notification},
    fixture_corpus::FixtureSample,
    journey_progress::{SAMPLE_HOME, SAMPLE_PASSWORD, announce_step},
    lazaret_application::{ApplicationPage, show_page},
    lazaret_database::LazaretDatabase,
    session_widgets::{SessionWidgets, button, label_containing},
    sway_windows::{answer_password_prompt, take_screenshot},
    system_commands::{as_root, journal_text, wait_until},
    ui_messages::{MessageCatalog, findings, scans, text_before_placeholder, widgets, window},
};

const SCAN_UNIT: &str = "clamdscan.service";
const SCANNED_FILE: &str = "/var/lib/sample-scan/dropper.bin";
const SCHEDULE_TIMER: &str = ".config/systemd/user/lazaret-scheduled-scan.timer";
const LOCKED_CONTROL_WAIT: Duration = Duration::from_secs(5);

fn scan_unit_property(property_name: &str) -> String {
    as_root(&[
        "systemctl",
        "show",
        "-p",
        property_name,
        "--value",
        SCAN_UNIT,
    ])
    .trim()
    .to_owned()
}

pub fn run_now_asks_for_password_and_scans(session_widgets: &SessionWidgets) {
    announce_step("Run Now asks for a password and runs the system scan");
    as_root(&[
        "cp",
        &FixtureSample::EicarInvoice.source_path(),
        SCANNED_FILE,
    ]);
    show_page(ApplicationPage::ScheduledScans);
    session_widgets.press(&button(scans::run_now_label(
        &MessageCatalog::source_language(),
    )));
    answer_password_prompt(SAMPLE_PASSWORD);
    wait_until("the system scan finding the file", LONG_WAIT, || {
        journal_text(&["-u", SCAN_UNIT])
            .lines()
            .any(|journal_line| {
                journal_line.contains(SCANNED_FILE) && journal_line.ends_with("FOUND")
            })
    });
    wait_until("the system scan ending", SHORT_WAIT, || {
        scan_unit_property("Result") == "exit-code"
    });
    let exit_status = scan_unit_property("ExecMainStatus");
    assert_eq!(exit_status, "1", "a detection should exit 1: {exit_status}");
}

pub fn found_malware_is_not_a_scanner_failure(session_widgets: &SessionWidgets) {
    announce_step("finding malware is not shown as a scanner failure");
    take_screenshot("scan_finished_toast");
    session_widgets.await_shown(&label_containing(scans::found_malware(
        &MessageCatalog::source_language(),
    )));
    assert!(
        !session_widgets.is_shown(&label_containing(scans::not_finish_cleanly(
            &MessageCatalog::source_language(),
        ))),
        "a found threat was shown as an unclean scan"
    );
}

pub fn scan_journal_line_becomes_detection(
    session_widgets: &SessionWidgets,
    database: &LazaretDatabase,
) {
    announce_step("the scan's journal line becomes a detection that is not called blocked");
    let detection = database.wait_for_stored_detection(SCANNED_FILE);
    assert!(
        detection.signature_name.starts_with("Win.Fixture.Agent-1"),
        "{detection:?}"
    );
    assert_eq!(
        detection.detection_sources, "scheduled_scan",
        "wrong source: {detection:?}"
    );
    wait_for_notification(&text_before_placeholder(|signature_name| {
        findings::malware_found_signature(&MessageCatalog::source_language(), signature_name)
    }));
    assert!(
        notifications_mentioning(&text_before_placeholder(|signature_name| {
            findings::malware_blocked_signature(&MessageCatalog::source_language(), signature_name)
        }))
        .is_empty(),
        "a scanned file was called blocked"
    );
    assert!(
        !session_widgets.is_shown(&label_containing(scans::last_scan_ended_error(
            &MessageCatalog::source_language(),
        ))),
        "a found threat was shown as a failed scan"
    );
}

pub fn lazaret_schedule_is_locked_by_system_timer(session_widgets: &SessionWidgets) {
    announce_step("the Lazaret schedule is locked because the system has a timer");
    show_page(ApplicationPage::ScheduledScans);
    session_widgets.await_shown(&label_containing(text_before_placeholder(
        |schedule_text| {
            scans::system_already_runs_scheduled(&MessageCatalog::source_language(), schedule_text)
        },
    )));
    let _locked_toggle_pressed = session_widgets.try_press_within(
        &WidgetTarget::label(scans::enable_scheduled_scans_title(
            &MessageCatalog::source_language(),
        )),
        LOCKED_CONTROL_WAIT,
    );
    let _locked_apply_pressed = session_widgets.try_press_within(
        &button(scans::apply_label(&MessageCatalog::source_language())),
        LOCKED_CONTROL_WAIT,
    );
    assert!(
        !Path::new(&format!("{SAMPLE_HOME}/{SCHEDULE_TIMER}")).exists(),
        "Lazaret wrote a second schedule"
    );
}

pub fn system_owned_settings_name_their_source(session_widgets: &SessionWidgets) {
    announce_step("settings the system owns name their source in their row");
    show_page(ApplicationPage::ScanEngine);
    session_widgets.await_shown(&label_containing(widgets::source_nixos_tag(
        &MessageCatalog::source_language(),
    )));
    take_screenshot("scanner_engine_settings");
}

pub fn signature_sources_have_their_own_page(session_widgets: &SessionWidgets) {
    announce_step("signature sources have their own page");
    show_page(ApplicationPage::SignatureSources);
    session_widgets.await_shown_within(
        &label_containing(window::signature_sources(&MessageCatalog::source_language())),
        SHORT_WAIT,
    );
}
