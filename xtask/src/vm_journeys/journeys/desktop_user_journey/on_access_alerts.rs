use std::{fs, path::Path, time::Duration};

use super::{ABUSE_CH_KEY, JourneyPaths, LONG_WAIT, SHORT_WAIT, contains_bytes};
use crate::vm_journeys::{
    harness::{
        desktop_notifications::{
            NotificationUrgency, dismiss_notifications, invoke_latest_notification,
            notifications_mentioning, wait_for_notification, wait_for_notification_withdrawn,
        },
        fixture_corpus::FixtureSample,
        journey_progress::announce_step,
        lazaret_application::{
            ApplicationAction, TrayStatus, activate_application_action, lazaret_window_is_visible,
            wait_for_lazaret_window, wait_for_tray_status,
        },
        lazaret_database::LazaretDatabase,
        session_widgets::{SessionWidgets, button, label_containing, row_titled},
        sway_windows::take_screenshot,
        system_commands::{
            as_root, journal_text, settle, succeed, unit_journal_lines_mentioning, wait_until,
        },
        ui_messages::{MessageCatalog, findings, host, text_before_placeholder},
    },
    remote_nodes::{
        fake_internet::wait_for_first_request,
        fake_internet_protocol::{
            BAZAAR_FIXTURE_FAMILY, FakeInternetScenario, FakeRoute, NetworkCondition,
        },
        team_share::colleague_places_on_team_share,
    },
};

const ON_ACCESS_UNIT: &str = "clamav-clamonacc.service";

fn wait_until_blocked(file_path: &str) {
    wait_until(
        &format!("on-access scanning blocking {file_path}"),
        Duration::from_secs(60),
        || {
            matches!(
                fs::read(file_path),
                Err(read_failure) if read_failure.kind() == std::io::ErrorKind::PermissionDenied
            )
        },
    );
}

fn wait_until_missing(file_path: &str) {
    wait_until(&format!("{file_path} disappearing"), SHORT_WAIT, || {
        !Path::new(file_path).exists()
    });
}

fn found_line_count(file_path: &str) -> usize {
    unit_journal_lines_mentioning(ON_ACCESS_UNIT, file_path)
        .iter()
        .filter(|journal_line| journal_line.ends_with("FOUND"))
        .count()
}

fn assert_lines_stay_at(file_path: &str, settle_duration: Duration) {
    let settled_count = found_line_count(file_path);
    settle(settle_duration);
    let final_count = found_line_count(file_path);
    assert_eq!(
        final_count, settled_count,
        "{file_path} kept producing detections: {settled_count} -> {final_count}"
    );
}

pub fn lazaret_starts_without_window() {
    announce_step("logging in starts Lazaret through the module, without a window");
    assert!(
        !lazaret_window_is_visible(),
        "Lazaret started with a window"
    );
}

pub fn on_access_scanning_is_ready() {
    announce_step("on-access scanning is ready");
    as_root(&["systemctl", "restart", ON_ACCESS_UNIT]);
    wait_until("clamonacc watching Downloads", LONG_WAIT, || {
        journal_text(&["-u", ON_ACCESS_UNIT]).contains("ClamInotif: watching")
    });
}

pub fn downloaded_invoice_raises_one_urgent_alert(database: &LazaretDatabase, blocked_file: &str) {
    announce_step("a downloaded invoice is blocked and raises one urgent alert");
    FixtureSample::EicarInvoice.place_at(blocked_file);
    wait_until_blocked(blocked_file);
    let blocked_heading = text_before_placeholder(|signature_name| {
        findings::malware_blocked_signature(&MessageCatalog::source_language(), signature_name)
    });
    let matching_alerts = wait_for_notification(blocked_file);
    assert_eq!(
        matching_alerts.len(),
        1,
        "expected one notification, got {}",
        matching_alerts.len()
    );
    assert!(
        matching_alerts[0].shown_text.contains(&blocked_heading),
        "the alert does not say {blocked_heading:?}: {}",
        matching_alerts[0].shown_text
    );
    assert_eq!(
        matching_alerts[0].urgency,
        NotificationUrgency::Critical,
        "the alert is not urgent"
    );
    let detection = database.wait_for_stored_detection(blocked_file);
    assert!(
        detection.signature_name.starts_with("Win.Fixture.Agent-1"),
        "wrong signature: {detection:?}"
    );
    assert!(
        detection.detection_sources.starts_with("on_access"),
        "not recorded as on-access: {detection:?}"
    );
    assert!(
        detection.resolution_kind.is_empty(),
        "should be unresolved: {detection:?}"
    );
    assert_lines_stay_at(blocked_file, Duration::from_secs(8));
    wait_for_tray_status(TrayStatus::NeedsAttention);
}

pub fn alert_opens_detection_and_look_up_sends_only_signature(session_widgets: &SessionWidgets) {
    announce_step(
        "the alert opens the detection, and Look Up sends MalwareBazaar only the signature name",
    );
    let look_up_button = button(findings::look_up_incident_signature(
        &MessageCatalog::source_language(),
    ));
    invoke_latest_notification();
    wait_for_lazaret_window();
    FakeInternetScenario::default()
        .with_network(NetworkCondition::RefusedConnections)
        .apply();
    session_widgets.press(&look_up_button);
    session_widgets.await_shown(&label_containing(text_before_placeholder(|request_text| {
        host::request_failed_bazaar(&MessageCatalog::source_language(), request_text)
    })));
    FakeInternetScenario::default().apply();
    session_widgets.press(&look_up_button);
    let lookup_request = wait_for_first_request(FakeRoute::BazaarGetClamavinfo);
    assert!(
        lookup_request
            .text_field("clamav")
            .is_some_and(|signature_name| signature_name.starts_with("Win.Fixture.Agent-1")),
        "the lookup did not name the signature: {lookup_request:?}"
    );
    assert_eq!(
        lookup_request.header("auth-key"),
        Some(ABUSE_CH_KEY),
        "the lookup lacked the abuse.ch key"
    );
    assert!(
        lookup_request.body_byte_size < 200,
        "the lookup sent more than a name"
    );
    session_widgets.await_shown(&label_containing(BAZAAR_FIXTURE_FAMILY));
    take_screenshot("detection_looked_up");
}

pub fn quarantining_from_timeline_neutralises_file(
    session_widgets: &SessionWidgets,
    database: &LazaretDatabase,
    blocked_file: &str,
    quarantine_directory: &str,
) {
    announce_step("quarantining from the timeline neutralises the file");
    session_widgets.press(&button(findings::quarantine_incident(
        &MessageCatalog::source_language(),
    )));
    wait_until_missing(blocked_file);
    wait_until("the detection resolved as quarantined", SHORT_WAIT, || {
        database
            .stored_detection(blocked_file)
            .is_some_and(|detection| detection.resolution_kind == "quarantined")
    });
    wait_for_tray_status(TrayStatus::Active);
    assert!(
        database
            .wait_for_quarantined_signature("invoice.pdf.exe")
            .starts_with("Win.Fixture.Agent-1"),
        "the quarantine lost the signature"
    );
    let payload_entries = fs::read_dir(quarantine_directory).unwrap_or_else(|listing_failure| {
        panic!("{quarantine_directory} is unreadable: {listing_failure}")
    });
    for payload_entry in payload_entries.flatten() {
        let payload_path = payload_entry.path();
        let payload_bytes = fs::read(&payload_path).unwrap_or_else(|read_failure| {
            panic!("{} is unreadable: {read_failure}", payload_path.display())
        });
        assert!(
            !contains_bytes(&payload_bytes, b"EICAR-STANDARD"),
            "the quarantined payload {} still holds the EICAR text",
            payload_path.display()
        );
    }
}

pub fn tray_action_opens_detections(session_widgets: &SessionWidgets) {
    announce_step("the tray action opens the Detections page");
    activate_application_action(ApplicationAction::OpenDetections);
    session_widgets.await_shown(&label_containing(
        findings::access_scanning_scheduled_scans(&MessageCatalog::source_language()),
    ));
}

pub fn deleting_takes_two_choices_and_withdraws_alert(
    session_widgets: &SessionWidgets,
    journey_paths: &JourneyPaths,
) {
    announce_step("deleting takes two choices and withdraws the alert that is still showing");
    dismiss_notifications();
    let deleted_file = journey_paths.download("throwaway.exe");
    FixtureSample::EicarInvoice.place_at(&deleted_file);
    wait_until_blocked(&deleted_file);
    wait_for_notification(&deleted_file);
    let ask_deletion_button = button(findings::ask_incident_deletion(
        &MessageCatalog::source_language(),
    ));
    session_widgets.press_until_shown(&row_titled("throwaway.exe"), &ask_deletion_button);
    session_widgets.press(&ask_deletion_button);
    assert!(
        Path::new(&deleted_file).exists(),
        "the first choice already deleted the file"
    );
    session_widgets.press(&button(findings::delete_incident_file(
        &MessageCatalog::source_language(),
    )));
    wait_until_missing(&deleted_file);
    wait_for_notification_withdrawn(&deleted_file);
    wait_for_tray_status(TrayStatus::Active);
}

pub fn colleague_file_on_team_share_is_blocked() {
    announce_step("a file a colleague put on the team share is blocked when it is opened");
    dismiss_notifications();
    let shared_file = colleague_places_on_team_share(FixtureSample::EicarInvoice);
    wait_until_blocked(&shared_file);
    wait_for_notification(&shared_file);
}

pub fn missed_detections_summarised_at_next_start(journey_paths: &JourneyPaths) {
    announce_step("detections missed while closed are summarised once at the next start");
    let missed_while_closed = journey_paths.download("while-closed.exe");
    dismiss_notifications();
    succeed("systemctl", &["--user", "stop", "lazaret.service"]);
    FixtureSample::EicarInvoice.place_at(&missed_while_closed);
    wait_until_blocked(&missed_while_closed);
    settle(Duration::from_secs(2));
    succeed("systemctl", &["--user", "start", "lazaret.service"]);
    wait_for_notification(&findings::count_detection_not_running(
        &MessageCatalog::source_language(),
        1,
    ));
    assert!(
        notifications_mentioning(&missed_while_closed).is_empty(),
        "alerted individually about a missed file"
    );
    assert_lines_stay_at(&missed_while_closed, Duration::from_secs(10));
}
