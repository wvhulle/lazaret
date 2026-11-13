use std::{path::Path, time::Duration};

use gtk_scene_capture::accessible_widget_lookup::{NameMatch, WidgetRole, WidgetTarget};
use serde_json::Value;

use super::{JourneyPaths, LONG_WAIT, NON_UTF8_FILE_NAME};
use crate::vm_journeys::{
    harness::{
        fixture_corpus::FixtureSample,
        journey_progress::announce_step,
        lazaret_application::{ApplicationPage, scan_with_lazaret, show_page},
        lazaret_database::{
            LazaretDatabase, SubmissionKey, SubmissionKind, SubmissionService, SubmissionStatus,
        },
        session_widgets::{SessionWidgets, button, in_alert, row_titled},
        sway_windows::{KeyInput, send_keys, take_screenshot},
        system_commands::{settle, wait_until},
        ui_messages::{MessageCatalog, findings, widgets},
    },
    remote_nodes::{
        fake_internet::{request_count_stays, wait_for_first_request},
        fake_internet_protocol::FakeRoute,
    },
};

const IGNORE_DETECTION_MENU_POSITION: usize = 1;
const MENU_ITEM_WAIT: Duration = Duration::from_secs(5);

fn quarantine_search_field() -> WidgetTarget {
    WidgetTarget::new(WidgetRole::Entry, "").matching_names(NameMatch::ContainedName)
}

fn show_quarantine_list(session_widgets: &SessionWidgets) {
    let back_button = button(findings::back_quarantine_tooltip(
        &MessageCatalog::source_language(),
    ));
    if session_widgets.is_shown(&back_button) {
        session_widgets.press(&back_button);
    }
}

fn show_quarantine_entry(
    session_widgets: &SessionWidgets,
    database: &LazaretDatabase,
    path_fragment: &str,
    search_text: &str,
) {
    show_page(ApplicationPage::Quarantine);
    show_quarantine_list(session_widgets);
    if !session_widgets.is_shown(&quarantine_search_field()) {
        session_widgets.press(&WidgetTarget::toggle_button(findings::search_tooltip(
            &MessageCatalog::source_language(),
        )));
    }
    session_widgets.type_into(&quarantine_search_field(), search_text);
    let entry_title = database.wait_for_quarantined_signature(path_fragment);
    session_widgets.press(&row_titled(entry_title));
    session_widgets.await_shown(&button(findings::trust_label(
        &MessageCatalog::source_language(),
    )));
}

fn choose_menu_item_by_keyboard(
    session_widgets: &SessionWidgets,
    item_position: usize,
    opened_dialog: &WidgetTarget,
) {
    for extra_down_presses in 0..2 {
        if extra_down_presses > 0 {
            send_keys(&[KeyInput::Escape]);
            session_widgets.press(&button(widgets::more_actions_tooltip(
                &MessageCatalog::source_language(),
            )));
        }
        settle(Duration::from_secs(1));
        send_keys(&vec![KeyInput::Down; item_position + extra_down_presses]);
        send_keys(&[KeyInput::Enter]);
        if session_widgets.is_shown_within(opened_dialog, MENU_ITEM_WAIT) {
            return;
        }
    }
    panic!("the keyboard never reached menu item {item_position}");
}

fn upload_quarantined_entry_to_malwarebazaar(session_widgets: &SessionWidgets) {
    session_widgets.press(&button(findings::submit_quarantine_label(
        &MessageCatalog::source_language(),
    )));
    session_widgets.press(&in_alert(
        button(findings::malwarebazaar_sample_choice(
            &MessageCatalog::source_language(),
        )),
        findings::choose_service_heading(&MessageCatalog::source_language()),
    ));
    session_widgets.press(&in_alert(
        button(widgets::upload_publicly_button(
            &MessageCatalog::source_language(),
        )),
        widgets::upload_malwarebazaar_heading(&MessageCatalog::source_language()),
    ));
}

fn quarantine_scan_results(session_widgets: &SessionWidgets, threat_count: u32) {
    session_widgets.press_within(
        &button(widgets::quarantine_all(
            &MessageCatalog::source_language(),
            threat_count,
        )),
        LONG_WAIT,
    );
    session_widgets.press(&in_alert(
        button(widgets::quarantine_label(&MessageCatalog::source_language())),
        widgets::move_quarantine(&MessageCatalog::source_language()),
    ));
}

pub fn manual_scan_quarantines_documents(
    session_widgets: &SessionWidgets,
    database: &LazaretDatabase,
    journey_paths: &JourneyPaths,
) {
    announce_step(
        "a manual scan of Documents finds the archive, the odd name, the false positive and the \
         toolbar",
    );
    let archive_file = journey_paths.document("holiday photos.zip");
    let false_positive_file = journey_paths.document("payroll-macro.docm");
    let unwanted_program_file = journey_paths.document("toolbar-setup.exe");
    let non_utf8_file = journey_paths.non_utf8_document();
    FixtureSample::HolidayArchive.place_at(&archive_file);
    FixtureSample::PayrollDocument.place_at(&false_positive_file);
    FixtureSample::ToolbarInstaller.place_at(&unwanted_program_file);
    FixtureSample::EicarInvoice.place_at(&non_utf8_file);
    let entries_before_scan = database.quarantine_entry_count();
    scan_with_lazaret(&[&journey_paths.documents]);
    quarantine_scan_results(session_widgets, 4);
    wait_until(
        "four new quarantine entries",
        Duration::from_secs(60),
        || database.quarantine_entry_count() == entries_before_scan + 4,
    );
    for moved_file in [&archive_file, &false_positive_file, &unwanted_program_file] {
        assert!(
            !Path::new(moved_file).exists(),
            "{moved_file} was not moved"
        );
    }
    assert!(
        !non_utf8_file.exists(),
        "the file with a non-UTF-8 name was not moved"
    );
    assert!(
        database
            .quarantined_original_paths()
            .iter()
            .any(|original_path| original_path.ends_with(NON_UTF8_FILE_NAME)),
        "the non-UTF-8 name was not stored byte for byte"
    );
    assert!(
        database
            .wait_for_quarantined_signature("payroll")
            .starts_with("Doc.Fixture.FalsePositive-1"),
        "wrong signature for the document"
    );
    assert!(
        database
            .wait_for_quarantined_signature("toolbar")
            .starts_with("PUA.Win.Fixture.Toolbar-1"),
        "wrong signature for the toolbar"
    );
    take_screenshot("documents_quarantined");
}

pub fn invoice_upload_is_named_by_hash_and_never_repeated(
    session_widgets: &SessionWidgets,
    database: &LazaretDatabase,
    invoice_sha256: &str,
) {
    announce_step(
        "uploading the invoice to MalwareBazaar names it by its hash and is never repeated",
    );
    show_quarantine_entry(session_widgets, database, "invoice.pdf.exe", "invoice");
    upload_quarantined_entry_to_malwarebazaar(session_widgets);
    let upload_request = wait_for_first_request(FakeRoute::BazaarUpload);
    let Some(uploaded_file) = upload_request.uploaded_file("file") else {
        panic!("the upload had no file: {upload_request:?}");
    };
    assert_eq!(
        uploaded_file.uploaded_name, invoice_sha256,
        "{uploaded_file:?}"
    );
    assert_eq!(
        uploaded_file.content_sha256, invoice_sha256,
        "the upload is not the quarantined file"
    );
    let options_text = upload_request.text_field("json_data").unwrap_or("{}");
    let upload_options: Value =
        serde_json::from_str(options_text).unwrap_or_else(|parse_failure| {
            panic!("the upload options are not JSON: {parse_failure}\n{options_text}")
        });
    assert_eq!(upload_options["anonymous"], 1, "{upload_options}");
    assert!(
        upload_options["tags"][0]
            .as_str()
            .is_some_and(|signature_tag| signature_tag.starts_with("Win.Fixture.Agent-1")),
        "{upload_options}"
    );
    database.wait_for_submission_status(
        &SubmissionKey {
            content_sha256: invoice_sha256,
            submission_service: SubmissionService::MalwareBazaar,
            submission_kind: SubmissionKind::MalwareSample,
        },
        SubmissionStatus::UploadAccepted,
        Duration::from_secs(90),
    );
    show_quarantine_entry(session_widgets, database, "invoice.pdf.exe", "invoice");
    upload_quarantined_entry_to_malwarebazaar(session_widgets);
    request_count_stays(FakeRoute::BazaarUpload, 1);
}

pub fn scan_result_menu_offers_working_actions(
    session_widgets: &SessionWidgets,
    journey_paths: &JourneyPaths,
) {
    announce_step("a scan result's menu offers working actions");
    let menu_sample = journey_paths.document("menu/payroll-macro.docm");
    FixtureSample::PayrollDocument.place_at(&menu_sample);
    scan_with_lazaret(&[&menu_sample]);
    session_widgets.await_shown_within(
        &button(widgets::quarantine_all(
            &MessageCatalog::source_language(),
            1,
        )),
        LONG_WAIT,
    );
    let more_actions_label = widgets::more_actions_tooltip(&MessageCatalog::source_language());
    session_widgets.press(&button(more_actions_label.clone()));
    session_widgets.await_shown_within(
        &WidgetTarget::new(
            WidgetRole::MenuItem,
            widgets::ignore_detection_for_me(&MessageCatalog::source_language()),
        )
        .inside(WidgetTarget::new(WidgetRole::Menu, more_actions_label)),
        MENU_ITEM_WAIT,
    );
    choose_menu_item_by_keyboard(
        session_widgets,
        IGNORE_DETECTION_MENU_POSITION,
        &WidgetTarget::alert(widgets::ignore_detection_heading(
            &MessageCatalog::source_language(),
        )),
    );
    send_keys(&[KeyInput::Escape]);
}
