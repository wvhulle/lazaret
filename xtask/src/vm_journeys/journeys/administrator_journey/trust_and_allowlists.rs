use std::{path::Path, time::Duration};

use super::{
    LONG_WAIT, ROOT_DOCUMENT_COPY, ROOT_EICAR_COPY, SHORT_WAIT,
    clamd_settings::{ScanVerdict, root_scan_verdict, wait_for_root_scan_verdict},
};
use crate::vm_journeys::harness::{
    fixture_corpus::FixtureSample,
    journey_progress::{SAMPLE_HOME, SAMPLE_PASSWORD, announce_step},
    lazaret_application::{ApplicationPage, scan_with_lazaret, show_page},
    lazaret_database::LazaretDatabase,
    session_widgets::{SessionWidgets, button, in_alert, in_row, row_titled},
    sway_windows::{answer_password_prompt, dismiss_password_prompt, take_screenshot},
    system_commands::{
        as_root, journal_text, root_file_text_or_empty, root_path_exists, wait_until,
    },
    ui_messages::{MessageCatalog, findings, widgets},
};

const TRUSTED_FILES_LIST: &str = "/var/lib/clamav/lazaret-local.sfp";
const IGNORED_SIGNATURES_LIST: &str = "/var/lib/clamav/lazaret-local.ign2";
const FOREIGN_ALLOWLIST: &str = "local.fp";
const IGNORED_SIGNATURE: &str = "Doc.Fixture.FalsePositive-1";

fn quarantine_with_lazaret(session_widgets: &SessionWidgets, file_path: &str) {
    scan_with_lazaret(&[file_path]);
    session_widgets.press_within(
        &button(widgets::quarantine_all(
            &MessageCatalog::source_language(),
            1,
        )),
        LONG_WAIT,
    );
    session_widgets.press(&in_alert(
        button(widgets::quarantine_label(&MessageCatalog::source_language())),
        widgets::move_quarantine(&MessageCatalog::source_language()),
    ));
    wait_until(
        &format!("{file_path} moving to quarantine"),
        Duration::from_secs(30),
        || !Path::new(file_path).exists(),
    );
}

fn open_quarantined_entry(
    session_widgets: &SessionWidgets,
    database: &LazaretDatabase,
    path_fragment: &str,
) {
    show_page(ApplicationPage::Detections);
    show_page(ApplicationPage::Quarantine);
    let back_button = button(findings::back_quarantine_tooltip(
        &MessageCatalog::source_language(),
    ));
    if session_widgets.is_shown(&back_button) {
        session_widgets.press(&back_button);
    }
    let entry_title = database.wait_for_quarantined_signature(path_fragment);
    session_widgets.press(&row_titled(entry_title));
    session_widgets.await_shown(&button(findings::trust_label(
        &MessageCatalog::source_language(),
    )));
}

fn ask_to_trust_for_all_users(
    session_widgets: &SessionWidgets,
    database: &LazaretDatabase,
    path_fragment: &str,
) {
    open_quarantined_entry(session_widgets, database, path_fragment);
    session_widgets.press(&button(findings::trust_label(
        &MessageCatalog::source_language(),
    )));
    session_widgets.press(&in_alert(
        button(widgets::trust_for_all_users_button(
            &MessageCatalog::source_language(),
        )),
        widgets::trust_file_heading(&MessageCatalog::source_language()),
    ));
}

pub fn allowlist_for_all_users_is_listed(session_widgets: &SessionWidgets) {
    announce_step(
        "an allowlist entry for all users is listed with its warning, next to a foreign allowlist",
    );
    assert_eq!(
        root_scan_verdict(ROOT_DOCUMENT_COPY),
        ScanVerdict::Clean,
        "the ignored detection was reported"
    );
    show_page(ApplicationPage::TrustedDetections);
    session_widgets.await_shown(&row_titled(IGNORED_SIGNATURE));
    session_widgets.await_shown(&row_titled(FOREIGN_ALLOWLIST));
    take_screenshot("allowlists_for_all_users");
}

pub fn reporting_ignored_detection_again_needs_password(session_widgets: &SessionWidgets) {
    announce_step("reporting an ignored detection again needs the password and shrinks the list");
    session_widgets.press(&in_row(
        button(findings::remove_tooltip(&MessageCatalog::source_language())),
        IGNORED_SIGNATURE,
    ));
    session_widgets.press(&in_alert(
        button(widgets::report_again_button(
            &MessageCatalog::source_language(),
        )),
        widgets::report_detection_again_heading(&MessageCatalog::source_language()),
    ));
    answer_password_prompt(SAMPLE_PASSWORD);
    wait_until("the ignored detection leaving the list", SHORT_WAIT, || {
        !root_file_text_or_empty(IGNORED_SIGNATURES_LIST).contains(IGNORED_SIGNATURE)
    });
    wait_for_root_scan_verdict(ROOT_DOCUMENT_COPY, ScanVerdict::Found);
    assert!(
        root_path_exists(&format!("/var/lib/clamav/{FOREIGN_ALLOWLIST}")),
        "the foreign allowlist was removed"
    );
}

pub fn refused_password_prompt_trusts_nothing(
    session_widgets: &SessionWidgets,
    database: &LazaretDatabase,
) {
    announce_step("cancelling or failing the password prompt trusts nothing");
    let trust_candidate = format!("{SAMPLE_HOME}/Documents/invoice.pdf.exe");
    FixtureSample::EicarInvoice.place_at(&trust_candidate);
    quarantine_with_lazaret(session_widgets, &trust_candidate);
    ask_to_trust_for_all_users(session_widgets, database, "invoice.pdf.exe");
    take_screenshot("trust_password_prompt");
    dismiss_password_prompt();
    assert!(
        !root_path_exists(TRUSTED_FILES_LIST),
        "a cancelled prompt trusted the file"
    );
    ask_to_trust_for_all_users(session_widgets, database, "invoice.pdf.exe");
    answer_password_prompt("not-the-password");
    dismiss_password_prompt();
    assert!(
        !root_path_exists(TRUSTED_FILES_LIST),
        "a wrong password trusted the file"
    );
    assert_eq!(
        root_scan_verdict(ROOT_EICAR_COPY),
        ScanVerdict::Found,
        "a refused trust change still hid the file"
    );
}

pub fn trusting_for_all_users_survives_as_hash_line(
    session_widgets: &SessionWidgets,
    database: &LazaretDatabase,
    eicar_sha256: &str,
) {
    announce_step("trusting a file for all users tells clamd and survives as a hash line");
    ask_to_trust_for_all_users(session_widgets, database, "invoice.pdf.exe");
    answer_password_prompt(SAMPLE_PASSWORD);
    let expected_line = format!("{eicar_sha256}:68:Lazaret.TrustedFile");
    wait_until(
        "the hash line in the trusted files list",
        SHORT_WAIT,
        || {
            root_file_text_or_empty(TRUSTED_FILES_LIST)
                .lines()
                .any(|listed_line| listed_line == expected_line)
        },
    );
    wait_for_root_scan_verdict(ROOT_EICAR_COPY, ScanVerdict::Clean);
    assert_eq!(
        as_root(&["stat", "-c", "%a %U", TRUSTED_FILES_LIST]).trim(),
        "644 root",
        "the trusted files list has the wrong mode or owner"
    );
    assert!(
        journal_text(&["-o", "verbose", "-u", "lazaret-clamav-admin.service"])
            .contains(eicar_sha256),
        "the admin service did not record the trust change"
    );
    show_page(ApplicationPage::TrustedDetections);
    session_widgets.await_shown(&row_titled(eicar_sha256));
    take_screenshot("trusted_for_all_users");
}

pub fn removing_trust_for_all_users_reports_file_again(
    session_widgets: &SessionWidgets,
    eicar_sha256: &str,
) {
    announce_step(
        "removing the trust for all users reports the file again and drops the empty list",
    );
    session_widgets.press(&in_row(
        button(findings::remove_tooltip(&MessageCatalog::source_language())),
        eicar_sha256,
    ));
    session_widgets.press(&in_alert(
        button(widgets::stop_trusting_button(
            &MessageCatalog::source_language(),
        )),
        widgets::stop_trusting_file_heading(&MessageCatalog::source_language()),
    ));
    answer_password_prompt(SAMPLE_PASSWORD);
    wait_until(
        "the empty trusted files list disappearing",
        SHORT_WAIT,
        || !root_path_exists(TRUSTED_FILES_LIST),
    );
    wait_for_root_scan_verdict(ROOT_EICAR_COPY, ScanVerdict::Found);
}

pub fn file_trusted_for_me_can_be_promoted(
    session_widgets: &SessionWidgets,
    database: &LazaretDatabase,
    document_sha256: &str,
) {
    announce_step("a file trusted for me can be promoted to all users from Trusted Detections");
    let promoted_candidate = format!("{SAMPLE_HOME}/Documents/payroll-macro.docm");
    FixtureSample::PayrollDocument.place_at(&promoted_candidate);
    quarantine_with_lazaret(session_widgets, &promoted_candidate);
    open_quarantined_entry(session_widgets, database, "payroll");
    session_widgets.press(&button(findings::trust_label(
        &MessageCatalog::source_language(),
    )));
    session_widgets.press(&in_alert(
        button(widgets::trust_for_me_button(
            &MessageCatalog::source_language(),
        )),
        widgets::trust_file_heading(&MessageCatalog::source_language()),
    ));
    wait_until("the document trusted for me", SHORT_WAIT, || {
        database.is_trusted_content(document_sha256)
    });
    show_page(ApplicationPage::TrustedDetections);
    session_widgets.press(&in_row(
        button(findings::trust_for_all_users_tooltip(
            &MessageCatalog::source_language(),
        )),
        document_sha256,
    ));
    session_widgets.press(&in_alert(
        button(widgets::trust_for_all_users_button(
            &MessageCatalog::source_language(),
        )),
        widgets::trust_file_heading(&MessageCatalog::source_language()),
    ));
    answer_password_prompt(SAMPLE_PASSWORD);
    wait_until("the promoted hash line", SHORT_WAIT, || {
        root_file_text_or_empty(TRUSTED_FILES_LIST).starts_with(&format!("{document_sha256}:"))
    });
    wait_for_root_scan_verdict(ROOT_DOCUMENT_COPY, ScanVerdict::Clean);
}
