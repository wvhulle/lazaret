use std::{fs, os::unix::ffi::OsStrExt, path::Path, time::Duration};

use gtk_scene_capture::accessible_widget_lookup::{NameMatch, WidgetRole, WidgetTarget};
use serde_json::{Value, json};

use crate::vm_journeys::{
    desktop_environment::{
        ApplicationAction, ApplicationPage, KeyInput, NotificationUrgency, TrayStatus,
        activate_application_action, answer_every_password_prompt, dismiss_notifications,
        file_chooser_closed, invoke_latest_notification, lazaret_window_is_visible,
        notifications_mentioning, scan_with_lazaret, send_keys, show_page, take_screenshot,
        type_path_into_file_chooser, wait_for_lazaret_service, wait_for_lazaret_window,
        wait_for_notification, wait_for_notification_withdrawn, wait_for_tray_status,
        wait_for_window_closed,
    },
    fake_internet::{
        BAZAAR_FIXTURE_FAMILY, FakeInternetScenario, FakeRoute, RouteOverride, SignatureRelease,
        request_count_stays, requests_to, wait_for_first_request, wait_for_request_after,
    },
    fixture_corpus::FixtureSample,
    journey_failure::{JourneyFailure, ensure},
    journey_run::{JourneyRun, SAMPLE_HOME, SAMPLE_PASSWORD, SAMPLE_USER},
    lazaret_database::{SubmissionKey, SubmissionKind, SubmissionService, SubmissionStatus},
    session_widgets::{button, in_alert, in_expanded_row, in_row, label_containing, row_titled},
    system_commands::{
        as_root, journal_text, settle, succeed, unit_journal_lines_mentioning, wait_until,
    },
    ui_messages::{findings, host, text_before_placeholder, widgets},
};

const ON_ACCESS_UNIT: &str = "clamav-clamonacc.service";
const ABUSE_CH_KEY: &str = "FIXTUREKEYMUSTNEVERAPPEAR";
const VIRUSTOTAL_KEY: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const REPORTER_NAME: &str = "Sample User";
const REPORTER_EMAIL: &str = "sample-user@example.org";
const NON_UTF8_FILE_NAME: &[u8] = b"r\xe9sum\xe9-copy.exe";
const UNREMOVABLE_FILE: &str = "/srv/shared/shared-report.exe";
const SIGNATURE_DATABASE_FILE: &str = "/var/lib/clamav/lazaret-fixture.hdb";
const GLOBAL_IGNORE_LIST: &str = "/var/lib/clamav/lazaret-local.ign2";
const FIXTURE_SIGNATURE_ADDRESS: &str = "https://signatures.fixture.test/lazaret-fixture.hdb";
const FAKE_INTERNET_REQUEST_LOG: &str = "/var/lib/fake-internet/requests.jsonl";
const IGNORE_DETECTION_MENU_POSITION: usize = 1;
const WRONG_URLHAUS_DIGEST: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";
const GTK_OPEN_LABEL: &str = "Open";
const SHORT_WAIT: Duration = Duration::from_secs(30);
const LONG_WAIT: Duration = Duration::from_secs(120);

struct JourneyPaths {
    downloads: String,
    documents: String,
    quarantine_directory: String,
}

impl JourneyPaths {
    fn for_sample_user() -> Self {
        Self {
            downloads: format!("{SAMPLE_HOME}/Downloads"),
            documents: format!("{SAMPLE_HOME}/Documents"),
            quarantine_directory: format!("{SAMPLE_HOME}/.local/share/lazaret/quarantine"),
        }
    }

    fn download(&self, file_name: &str) -> String {
        format!("{}/{file_name}", self.downloads)
    }

    fn document(&self, file_name: &str) -> String {
        format!("{}/{file_name}", self.documents)
    }
}

struct SampleHashes {
    eicar_invoice: String,
    holiday_archive: String,
    payroll_document: String,
    toolbar_installer: String,
    missed_dropper: String,
}

impl SampleHashes {
    fn read() -> Result<Self, JourneyFailure> {
        Ok(Self {
            eicar_invoice: FixtureSample::EicarInvoice.content_sha256()?,
            holiday_archive: FixtureSample::HolidayArchive.content_sha256()?,
            payroll_document: FixtureSample::PayrollDocument.content_sha256()?,
            toolbar_installer: FixtureSample::ToolbarInstaller.content_sha256()?,
            missed_dropper: FixtureSample::MissedDropper.content_sha256()?,
        })
    }
}

fn wait_until_blocked(file_path: &str) -> Result<(), JourneyFailure> {
    wait_until(
        &format!("on-access scanning blocking {file_path}"),
        Duration::from_secs(60),
        || {
            Ok(matches!(
                fs::read(file_path),
                Err(read_failure) if read_failure.kind() == std::io::ErrorKind::PermissionDenied
            ))
        },
    )
}

fn wait_until_missing(file_path: &str) -> Result<(), JourneyFailure> {
    wait_until(&format!("{file_path} disappearing"), SHORT_WAIT, || {
        Ok(!Path::new(file_path).exists())
    })
}

fn quarantine_payload_count(journey_paths: &JourneyPaths) -> Result<usize, JourneyFailure> {
    Ok(fs::read_dir(&journey_paths.quarantine_directory)?.count())
}

fn assert_lines_stay_at(file_path: &str, settle_duration: Duration) -> Result<(), JourneyFailure> {
    let settled_count = unit_journal_lines_mentioning(ON_ACCESS_UNIT, file_path)?
        .iter()
        .filter(|journal_line| journal_line.ends_with("FOUND"))
        .count();
    settle(settle_duration);
    let final_count = unit_journal_lines_mentioning(ON_ACCESS_UNIT, file_path)?
        .iter()
        .filter(|journal_line| journal_line.ends_with("FOUND"))
        .count();
    ensure(final_count == settled_count, || {
        format!("{file_path} kept producing detections: {settled_count} -> {final_count}")
    })
}

fn quarantine_search_field() -> WidgetTarget {
    WidgetTarget::new(WidgetRole::Entry, "").matching_names(NameMatch::ContainedName)
}

fn show_quarantine_entry(
    journey_run: &mut JourneyRun,
    path_fragment: &str,
    search_text: &str,
) -> Result<String, JourneyFailure> {
    show_page(ApplicationPage::Quarantine)?;
    journey_run.press(&button(findings::refresh_tooltip()))?;
    if !journey_run.widgets.is_shown(&quarantine_search_field()) {
        journey_run.press(&WidgetTarget::toggle_button(findings::search_tooltip()))?;
    }
    journey_run
        .widgets
        .type_into(&quarantine_search_field(), search_text)?;
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

fn choose_menu_item_by_keyboard(
    journey_run: &JourneyRun,
    item_position: usize,
    opened_dialog: &WidgetTarget,
) -> Result<(), JourneyFailure> {
    for extra_down_presses in 0..2 {
        if extra_down_presses > 0 {
            send_keys(&[KeyInput::Escape])?;
            journey_run.press(&button(widgets::more_actions_tooltip()))?;
        }
        settle(Duration::from_secs(1));
        let down_presses = vec![KeyInput::Down; item_position + extra_down_presses];
        send_keys(&down_presses)?;
        send_keys(&[KeyInput::Enter])?;
        if journey_run
            .widgets
            .await_shown_within(opened_dialog, Duration::from_secs(5))
            .is_ok()
        {
            return Ok(());
        }
    }
    Err(JourneyFailure::ExpectationFailed(format!(
        "the keyboard never reached menu item {item_position}"
    )))
}

fn entry_button(button_label: String, entry_title: &str) -> WidgetTarget {
    in_expanded_row(button(button_label), entry_title)
}

fn submit_quarantined_entry(
    journey_run: &JourneyRun,
    entry_title: &str,
    target_label: String,
    confirm_heading: String,
    confirm_label: String,
) -> Result<(), JourneyFailure> {
    journey_run.press(&entry_button(
        findings::submit_quarantine_label(),
        entry_title,
    ))?;
    journey_run.press(&in_alert(
        button(target_label),
        findings::choose_service_heading(),
    ))?;
    journey_run.press(&in_alert(button(confirm_label), confirm_heading))
}

fn type_reporter_contact(
    journey_run: &JourneyRun,
    dialog_heading: String,
) -> Result<(), JourneyFailure> {
    journey_run.widgets.type_into(
        &in_alert(
            WidgetTarget::new(WidgetRole::Text, widgets::name_title()),
            dialog_heading.clone(),
        ),
        REPORTER_NAME,
    )?;
    journey_run.widgets.type_into(
        &in_alert(
            WidgetTarget::new(WidgetRole::Text, widgets::email_address_title()),
            dialog_heading,
        ),
        REPORTER_EMAIL,
    )
}

fn quarantine_scan_results(
    journey_run: &JourneyRun,
    threat_count: u32,
) -> Result<(), JourneyFailure> {
    journey_run
        .widgets
        .press_within(&button(widgets::quarantine_all(threat_count)), LONG_WAIT)?;
    journey_run.press(&in_alert(
        button(widgets::quarantine_label()),
        widgets::move_quarantine(),
    ))
}

fn submission(
    content_sha256: &str,
    submission_service: SubmissionService,
    submission_kind: SubmissionKind,
) -> SubmissionKey<'_> {
    SubmissionKey {
        content_sha256,
        submission_service,
        submission_kind,
    }
}

fn reset_scenario() -> Result<(), JourneyFailure> {
    FakeInternetScenario::default().apply()
}

pub fn run_desktop_user_journey() -> Result<(), JourneyFailure> {
    let journey_paths = JourneyPaths::for_sample_user();
    let sample_hashes = SampleHashes::read()?;
    let blocked_file = journey_paths.download("invoice.pdf.exe");
    let archive_file = journey_paths.document("holiday photos.zip");
    let false_positive_file = journey_paths.document("payroll-macro.docm");
    let unwanted_program_file = journey_paths.document("toolbar-setup.exe");
    let missed_malware_file = journey_paths.document("dropper-2026.bin");
    let non_utf8_file =
        Path::new(&journey_paths.documents).join(std::ffi::OsStr::from_bytes(NON_UTF8_FILE_NAME));

    wait_until(
        "the fake internet answering over TLS",
        Duration::from_secs(60),
        || {
            Ok(
                succeed("curl", &["--fail", "--silent", FIXTURE_SIGNATURE_ADDRESS]).is_ok_and(
                    |served_signatures| served_signatures.contains("Win.Fixture.Agent-1"),
                ),
            )
        },
    )?;
    as_root(&["rm", "-f", FAKE_INTERNET_REQUEST_LOG])?;
    wait_for_lazaret_service()?;
    let mut journey_run = JourneyRun::start()?;

    journey_run.step(
        "logging in starts Lazaret through the module, without a window",
        |_journey_run| {
            ensure(!lazaret_window_is_visible()?, || {
                "Lazaret started with a window".to_owned()
            })
        },
    )?;

    journey_run.step("on-access scanning is ready", |_journey_run| {
        as_root(&["systemctl", "restart", ON_ACCESS_UNIT])?;
        wait_until("clamonacc watching Downloads", LONG_WAIT, || {
            Ok(journal_text(&["-u", ON_ACCESS_UNIT])?.contains("ClamInotif: watching"))
        })
    })?;

    journey_run.step(
        "a downloaded invoice is blocked and raises one urgent alert",
        |journey_run| {
            FixtureSample::EicarInvoice.place_at(&blocked_file)?;
            wait_until_blocked(&blocked_file)?;
            let blocked_heading = text_before_placeholder(findings::malware_blocked_signature);
            let matching = wait_for_notification(&blocked_file)?;
            ensure(matching.len() == 1, || {
                format!("expected one notification, got {}", matching.len())
            })?;
            ensure(matching[0].shown_text.contains(&blocked_heading), || {
                format!(
                    "the alert does not say {blocked_heading:?}: {}",
                    matching[0].shown_text
                )
            })?;
            ensure(matching[0].urgency == NotificationUrgency::Critical, || {
                "the alert is not urgent".to_owned()
            })?;
            let detection = journey_run
                .database
                .wait_for_stored_detection(&blocked_file)?;
            ensure(
                detection.signature_name.starts_with("Win.Fixture.Agent-1"),
                || format!("wrong signature: {detection:?}"),
            )?;
            ensure(detection.detection_sources.starts_with("on_access"), || {
                format!("not recorded as on-access: {detection:?}")
            })?;
            ensure(detection.resolution_kind.is_empty(), || {
                format!("should be unresolved: {detection:?}")
            })?;
            assert_lines_stay_at(&blocked_file, Duration::from_secs(8))?;
            wait_for_tray_status(TrayStatus::NeedsAttention)
        },
    )?;

    journey_run.step(
        "the alert opens the detection, and Look Up sends MalwareBazaar only the signature name",
        |journey_run| {
            invoke_latest_notification()?;
            wait_for_lazaret_window()?;
            journey_run.press(&button(findings::look_up_incident_signature()))?;
            let lookup_request = wait_for_first_request(FakeRoute::BazaarGetClamavinfo)?;
            ensure(
                lookup_request
                    .text_field("clamav")
                    .is_some_and(|signature_name| {
                        signature_name.starts_with("Win.Fixture.Agent-1")
                    }),
                || format!("the lookup did not name the signature: {lookup_request:?}"),
            )?;
            ensure(
                lookup_request.header("auth-key") == Some(ABUSE_CH_KEY),
                || "the lookup lacked the abuse.ch key".to_owned(),
            )?;
            ensure(lookup_request.body_byte_size < 200, || {
                "the lookup sent more than a name".to_owned()
            })?;
            journey_run
                .widgets
                .await_shown(&label_containing(BAZAAR_FIXTURE_FAMILY))?;
            take_screenshot("detection_looked_up");
            Ok(())
        },
    )?;

    journey_run.step(
        "quarantining from the timeline neutralises the file",
        |journey_run| {
            journey_run.press(&button(findings::quarantine_incident()))?;
            wait_until_missing(&blocked_file)?;
            wait_until("the detection resolved as quarantined", SHORT_WAIT, || {
                Ok(journey_run
                    .database
                    .stored_detection(&blocked_file)?
                    .is_some_and(|detection| detection.resolution_kind == "quarantined"))
            })?;
            wait_for_tray_status(TrayStatus::Active)?;
            ensure(
                journey_run
                    .database
                    .wait_for_quarantined_signature("invoice.pdf.exe")?
                    .starts_with("Win.Fixture.Agent-1"),
                || "the quarantine lost the signature".to_owned(),
            )?;
            for payload_entry in fs::read_dir(&journey_paths.quarantine_directory)? {
                let payload_bytes = fs::read(payload_entry?.path())?;
                ensure(
                    !payload_bytes
                        .windows(14)
                        .any(|byte_window| byte_window == b"EICAR-STANDARD"),
                    || "a quarantined payload still holds the EICAR text".to_owned(),
                )?;
            }
            Ok(())
        },
    )?;

    journey_run.step("the tray action opens the Detections page", |journey_run| {
        activate_application_action(ApplicationAction::OpenDetections)?;
        journey_run.await_shown(&label_containing(
            findings::access_scanning_scheduled_scans(),
        ))
    })?;

    journey_run.step(
        "deleting takes two choices and withdraws the alert that is still showing",
        |journey_run| {
            dismiss_notifications()?;
            let deleted_file = journey_paths.download("throwaway.exe");
            FixtureSample::EicarInvoice.place_at(&deleted_file)?;
            wait_until_blocked(&deleted_file)?;
            wait_for_notification(&deleted_file)?;
            journey_run.press(&row_titled("throwaway.exe"))?;
            journey_run.press(&button(findings::ask_incident_deletion()))?;
            ensure(Path::new(&deleted_file).exists(), || {
                "the first choice already deleted the file".to_owned()
            })?;
            journey_run.press(&button(findings::delete_incident_file()))?;
            wait_until_missing(&deleted_file)?;
            wait_for_notification_withdrawn(&deleted_file)?;
            wait_for_tray_status(TrayStatus::Active)
        },
    )?;

    journey_run.step(
        "a manual scan of Documents finds the archive, the odd name, the false positive and the \
         toolbar",
        |journey_run| {
            FixtureSample::HolidayArchive.place_at(&archive_file)?;
            FixtureSample::PayrollDocument.place_at(&false_positive_file)?;
            FixtureSample::ToolbarInstaller.place_at(&unwanted_program_file)?;
            FixtureSample::EicarInvoice.place_at(&non_utf8_file)?;
            let entries_before_scan = journey_run.database.quarantine_entry_count()?;
            scan_with_lazaret(&[&journey_paths.documents])?;
            quarantine_scan_results(journey_run, 4)?;
            wait_until(
                "four new quarantine entries",
                Duration::from_secs(60),
                || Ok(journey_run.database.quarantine_entry_count()? == entries_before_scan + 4),
            )?;
            for moved_file in [&archive_file, &false_positive_file, &unwanted_program_file] {
                ensure(!Path::new(moved_file).exists(), || {
                    format!("{moved_file} was not moved")
                })?;
            }
            ensure(!non_utf8_file.exists(), || {
                "the file with a non-UTF-8 name was not moved".to_owned()
            })?;
            ensure(
                journey_run
                    .database
                    .quarantined_original_paths()?
                    .iter()
                    .any(|original_path| original_path.ends_with(NON_UTF8_FILE_NAME)),
                || "the non-UTF-8 name was not stored byte for byte".to_owned(),
            )?;
            ensure(
                journey_run
                    .database
                    .wait_for_quarantined_signature("payroll")?
                    .starts_with("Doc.Fixture.FalsePositive-1"),
                || "wrong signature for the document".to_owned(),
            )?;
            ensure(
                journey_run
                    .database
                    .wait_for_quarantined_signature("toolbar")?
                    .starts_with("PUA.Win.Fixture.Toolbar-1"),
                || "wrong signature for the toolbar".to_owned(),
            )?;
            take_screenshot("documents_quarantined");
            Ok(())
        },
    )?;

    let mut eicar_title = String::new();
    journey_run.step(
        "the reputation check sends each service only the hash",
        |journey_run| {
            eicar_title = show_quarantine_entry(journey_run, "invoice.pdf.exe", "invoice")?;
            journey_run.press(&entry_button(findings::check_label(), &eicar_title))?;
            let circl_request = wait_for_first_request(FakeRoute::CirclLookup)?;
            let bazaar_request = wait_for_first_request(FakeRoute::BazaarGetInfo)?;
            let virustotal_request = wait_for_first_request(FakeRoute::VirustotalFileReport)?;
            ensure(
                circl_request.path.ends_with(&sample_hashes.eicar_invoice),
                || format!("{circl_request:?}"),
            )?;
            ensure(
                circl_request.header("auth-key").is_none()
                    && circl_request.header("x-apikey").is_none(),
                || "CIRCL got a key".to_owned(),
            )?;
            ensure(
                bazaar_request.text_field("query") == Some("get_info")
                    && bazaar_request.text_field("hash")
                        == Some(sample_hashes.eicar_invoice.as_str()),
                || format!("{bazaar_request:?}"),
            )?;
            ensure(
                virustotal_request.path == format!("/api/v3/files/{}", sample_hashes.eicar_invoice),
                || format!("{virustotal_request:?}"),
            )?;
            ensure(
                virustotal_request.header("x-apikey") == Some(VIRUSTOTAL_KEY),
                || "VirusTotal lacked its key".to_owned(),
            )?;
            for lookup_request in [&circl_request, &bazaar_request, &virustotal_request] {
                ensure(lookup_request.body_byte_size < 200, || {
                    format!("a lookup sent the file: {lookup_request:?}")
                })?;
            }
            journey_run.await_shown(&label_containing(host::file_unknown_virustotal()))?;
            take_screenshot("reputation_checked");
            Ok(())
        },
    )?;

    journey_run.step(
        "a VirusTotal rate limit and a rejected key are explained",
        |journey_run| {
            FakeInternetScenario::default()
                .answering(
                    FakeRoute::VirustotalFileReport,
                    RouteOverride::json_answer(
                        429,
                        json!({"error": {"code": "QuotaExceededError"}}),
                    ),
                )
                .apply()?;
            journey_run.press(&entry_button(findings::check_label(), &eicar_title))?;
            journey_run.await_shown(&label_containing(host::request_limit_reached()))?;
            FakeInternetScenario::default()
                .answering(
                    FakeRoute::VirustotalFileReport,
                    RouteOverride::json_answer(
                        401,
                        json!({"error": {"code": "WrongCredentialsError"}}),
                    ),
                )
                .apply()?;
            journey_run.press(&entry_button(findings::check_label(), &eicar_title))?;
            journey_run.await_shown(&label_containing(host::api_key_rejected()))?;
            reset_scenario()
        },
    )?;

    journey_run.step(
        "an unavailable MalwareBazaar is reported as unavailable, not as an unreadable answer",
        |journey_run| {
            FakeInternetScenario::default()
                .answering(
                    FakeRoute::BazaarGetInfo,
                    RouteOverride::html_page(503, "<html>Service Unavailable</html>"),
                )
                .apply()?;
            let lookups_before = requests_to(FakeRoute::BazaarGetInfo)?.len();
            journey_run.press(&entry_button(findings::check_label(), &eicar_title))?;
            wait_for_request_after(FakeRoute::BazaarGetInfo, lookups_before, SHORT_WAIT)?;
            journey_run.expect_behaviour(
                "an HTTP 503 from MalwareBazaar is reported with its status",
                |journey_run| {
                    journey_run.await_shown(&label_containing(host::unexpected_status_bazaar(503)))
                },
            );
            take_screenshot("bazaar_unavailable");
            reset_scenario()
        },
    )?;

    journey_run.step(
        "uploading the invoice to MalwareBazaar names it by its hash and is never repeated",
        |journey_run| {
            submit_quarantined_entry(
                journey_run,
                &eicar_title,
                findings::malwarebazaar_sample_choice(),
                widgets::upload_malwarebazaar_heading(),
                widgets::upload_publicly_button(),
            )?;
            let upload_request = wait_for_first_request(FakeRoute::BazaarUpload)?;
            let uploaded_file = upload_request.uploaded_file("file").ok_or_else(|| {
                JourneyFailure::ExpectationFailed("the upload had no file".to_owned())
            })?;
            ensure(
                uploaded_file.uploaded_name == sample_hashes.eicar_invoice,
                || format!("{uploaded_file:?}"),
            )?;
            ensure(
                uploaded_file.content_sha256 == sample_hashes.eicar_invoice,
                || "the upload is not the quarantined file".to_owned(),
            )?;
            let upload_options: Value =
                serde_json::from_str(upload_request.text_field("json_data").unwrap_or("{}"))?;
            ensure(upload_options["anonymous"] == 1, || {
                format!("{upload_options}")
            })?;
            ensure(
                upload_options["tags"][0]
                    .as_str()
                    .is_some_and(|signature_tag| signature_tag.starts_with("Win.Fixture.Agent-1")),
                || format!("{upload_options}"),
            )?;
            journey_run.database.wait_for_submission_status(
                &submission(
                    &sample_hashes.eicar_invoice,
                    SubmissionService::MalwareBazaar,
                    SubmissionKind::MalwareSample,
                ),
                SubmissionStatus::UploadAccepted,
                Duration::from_secs(90),
            )?;
            eicar_title = show_quarantine_entry(journey_run, "invoice.pdf.exe", "invoice")?;
            submit_quarantined_entry(
                journey_run,
                &eicar_title,
                findings::malwarebazaar_sample_choice(),
                widgets::upload_malwarebazaar_heading(),
                widgets::upload_publicly_button(),
            )?;
            request_count_stays(FakeRoute::BazaarUpload, 1)
        },
    )?;

    journey_run.step("the archive goes to MalwareBazaar too", |journey_run| {
        let archive_title = show_quarantine_entry(journey_run, "holiday", "holiday")?;
        submit_quarantined_entry(
            journey_run,
            &archive_title,
            findings::malwarebazaar_sample_choice(),
            widgets::upload_malwarebazaar_heading(),
            widgets::upload_publicly_button(),
        )?;
        wait_for_request_after(FakeRoute::BazaarUpload, 1, Duration::from_secs(60))?;
        journey_run.database.wait_for_submission_status(
            &submission(
                &sample_hashes.holiday_archive,
                SubmissionService::MalwareBazaar,
                SubmissionKind::MalwareSample,
            ),
            SubmissionStatus::UploadAccepted,
            Duration::from_secs(90),
        )
    })?;

    journey_run.step(
        "a potentially unwanted program is never offered to MalwareBazaar",
        |journey_run| {
            let toolbar_title = show_quarantine_entry(journey_run, "toolbar", "toolbar")?;
            journey_run.press(&entry_button(
                findings::submit_quarantine_label(),
                &toolbar_title,
            ))?;
            journey_run.await_shown(&WidgetTarget::alert(findings::choose_service_heading()))?;
            ensure(
                !journey_run.widgets.is_shown(&in_alert(
                    button(findings::malwarebazaar_sample_choice()),
                    findings::choose_service_heading(),
                )),
                || "MalwareBazaar was offered a potentially unwanted program".to_owned(),
            )?;
            journey_run.press(&in_alert(
                button(findings::cancel_submission()),
                findings::choose_service_heading(),
            ))?;
            let upload_count = requests_to(FakeRoute::BazaarUpload)?.len();
            ensure(upload_count == 2, || {
                format!("{upload_count} MalwareBazaar uploads, expected 2")
            })
        },
    )?;

    journey_run.step(
        "uploading the invoice to VirusTotal queues an analysis",
        |journey_run| {
            eicar_title = show_quarantine_entry(journey_run, "invoice.pdf.exe", "invoice")?;
            submit_quarantined_entry(
                journey_run,
                &eicar_title,
                findings::virustotal_sample_choice(),
                widgets::upload_virustotal_heading(),
                widgets::upload_button(),
            )?;
            let virustotal_upload = wait_for_first_request(FakeRoute::VirustotalUpload)?;
            ensure(
                virustotal_upload
                    .uploaded_file("file")
                    .is_some_and(|uploaded_file| {
                        uploaded_file.uploaded_name == sample_hashes.eicar_invoice
                    }),
                || format!("{virustotal_upload:?}"),
            )?;
            ensure(
                virustotal_upload.header("x-apikey") == Some(VIRUSTOTAL_KEY),
                || "the upload lacked the VirusTotal key".to_owned(),
            )?;
            journey_run.database.wait_for_submission_status(
                &submission(
                    &sample_hashes.eicar_invoice,
                    SubmissionService::VirusTotal,
                    SubmissionKind::MalwareSample,
                ),
                SubmissionStatus::AnalysisQueued,
                Duration::from_secs(90),
            )
        },
    )?;

    journey_run.step(
        "one failed MalwareBazaar lookup doesn't stop the refresh of the other uploads",
        |journey_run| {
            FakeInternetScenario::default()
                .answering(
                    FakeRoute::BazaarGetInfo,
                    RouteOverride::html_page(500, "<html>Internal error</html>")
                        .only_for(&sample_hashes.eicar_invoice),
                )
                .apply()?;
            show_page(ApplicationPage::SentSamples)?;
            journey_run.press(&button(findings::check_malwarebazaar_virustotal_tooltip()))?;
            wait_for_first_request(FakeRoute::VirustotalAnalysis)?;
            journey_run.press(&button(findings::check_malwarebazaar_virustotal_tooltip()))?;
            journey_run.database.wait_for_submission_status(
                &submission(
                    &sample_hashes.eicar_invoice,
                    SubmissionService::VirusTotal,
                    SubmissionKind::MalwareSample,
                ),
                SubmissionStatus::Analysed,
                Duration::from_secs(90),
            )?;
            reset_scenario()?;
            take_screenshot("sent_samples_after_refresh");
            journey_run.expect_behaviour(
                "one failed MalwareBazaar lookup leaves the other uploads refreshed",
                |journey_run| {
                    journey_run.database.wait_for_submission_status(
                        &submission(
                            &sample_hashes.holiday_archive,
                            SubmissionService::MalwareBazaar,
                            SubmissionKind::MalwareSample,
                        ),
                        SubmissionStatus::Listed,
                        SHORT_WAIT,
                    )
                },
            );
            Ok(())
        },
    )?;

    journey_run.step(
        "reporting the false positive runs clamsubmit against clamav.net",
        |journey_run| {
            let false_positive_title = show_quarantine_entry(journey_run, "payroll", "payroll")?;
            journey_run.press(&entry_button(
                findings::report_label(),
                &false_positive_title,
            ))?;
            type_reporter_contact(journey_run, widgets::report_false_positive_clamav())?;
            journey_run.press(&in_alert(
                button(widgets::send_report_button()),
                widgets::report_false_positive_clamav(),
            ))?;
            let submit_request = wait_for_request_after(FakeRoute::ClamavSubmit, 0, LONG_WAIT)?;
            ensure(submit_request.text_field("type") == Some("fp"), || {
                format!("{submit_request:?}")
            })?;
            ensure(
                submit_request.text_field("sendername") == Some(REPORTER_NAME)
                    && submit_request.text_field("email") == Some(REPORTER_EMAIL),
                || format!("{submit_request:?}"),
            )?;
            ensure(
                submit_request
                    .text_field("virusname")
                    .is_some_and(|signature_name| {
                        signature_name.starts_with("Doc.Fixture.FalsePositive-1")
                    }),
                || format!("{submit_request:?}"),
            )?;
            let sample_upload = requests_to(FakeRoute::ClamavSampleUpload)?
                .pop()
                .ok_or_else(|| {
                    JourneyFailure::ExpectationFailed("clamsubmit uploaded nothing".to_owned())
                })?;
            ensure(
                sample_upload
                    .uploaded_file("file")
                    .is_some_and(|uploaded_file| {
                        uploaded_file.content_sha256 == sample_hashes.payroll_document
                    }),
                || format!("{sample_upload:?}"),
            )?;
            journey_run.database.wait_for_submission_status(
                &submission(
                    &sample_hashes.payroll_document,
                    SubmissionService::Clamav,
                    SubmissionKind::FalsePositive,
                ),
                SubmissionStatus::AwaitingReview,
                Duration::from_secs(90),
            )
        },
    )?;

    journey_run.step(
        "missed malware picked from disk is reported to ClamAV",
        |journey_run| {
            FixtureSample::MissedDropper.place_at(&missed_malware_file)?;
            show_page(ApplicationPage::SentSamples)?;
            journey_run.press(&button(findings::report_missed_malware_label()))?;
            let chooser_title = findings::choose_file_clamav_misses();
            type_path_into_file_chooser(&chooser_title, &missed_malware_file)?;
            if !file_chooser_closed(&chooser_title)? {
                journey_run.press(&button(GTK_OPEN_LABEL))?;
                wait_for_window_closed(&chooser_title)?;
            }
            type_reporter_contact(journey_run, widgets::report_missed_malware_clamav())?;
            journey_run.press(&in_alert(
                button(widgets::send_report_button()),
                widgets::report_missed_malware_clamav(),
            ))?;
            let malware_report = wait_for_request_after(FakeRoute::ClamavSubmit, 1, LONG_WAIT)?;
            ensure(malware_report.text_field("type") == Some("malware"), || {
                format!("{malware_report:?}")
            })?;
            journey_run.database.wait_for_submission_status(
                &submission(
                    &sample_hashes.missed_dropper,
                    SubmissionService::Clamav,
                    SubmissionKind::MissedMalware,
                ),
                SubmissionStatus::AwaitingReview,
                Duration::from_secs(90),
            )
        },
    )?;

    let trusted_copy = journey_paths.document("trusted/toolbar-setup.exe");
    journey_run.step(
        "trusting the toolbar for me hides it from my scans, but not a changed copy",
        |journey_run| {
            let toolbar_title = show_quarantine_entry(journey_run, "toolbar", "toolbar")?;
            journey_run.press(&entry_button(findings::trust_label(), &toolbar_title))?;
            journey_run.press(&in_alert(
                button(widgets::trust_for_me_button()),
                widgets::trust_file_heading(),
            ))?;
            wait_until("the toolbar trusted for me", SHORT_WAIT, || {
                journey_run
                    .database
                    .is_trusted_content(&sample_hashes.toolbar_installer)
            })?;
            let changed_copy = journey_paths.document("changed/toolbar-setup.exe");
            FixtureSample::ToolbarInstaller.place_at(&trusted_copy)?;
            FixtureSample::ToolbarInstaller.place_at(&changed_copy)?;
            let mut changed_bytes = fs::read(&changed_copy)?;
            changed_bytes.extend_from_slice(b"tampered");
            fs::write(&changed_copy, changed_bytes)?;
            scan_with_lazaret(&[&trusted_copy, &changed_copy])?;
            journey_run.expect_behaviour(
                "a scan of a trusted copy and a changed copy lists only the changed copy, once",
                |journey_run| {
                    journey_run
                        .widgets
                        .await_shown_within(&button(widgets::quarantine_all(1)), LONG_WAIT)
                },
            );
            let listed_threats = if journey_run
                .widgets
                .is_shown(&button(widgets::quarantine_all(1)))
            {
                1
            } else {
                2
            };
            quarantine_scan_results(journey_run, listed_threats)?;
            wait_until_missing(&changed_copy)?;
            journey_run.expect_behaviour(
                "quarantining the scan results leaves the trusted copy",
                |_journey_run| {
                    ensure(Path::new(&trusted_copy).exists(), || {
                        "the trusted copy was quarantined".to_owned()
                    })
                },
            );
            if !Path::new(&trusted_copy).exists() {
                FixtureSample::ToolbarInstaller.place_at(&trusted_copy)?;
            }
            Ok(())
        },
    )?;

    journey_run.step(
        "a file trusted only for me is still blocked on access and still alerts, as the dialog \
         says",
        |_journey_run| {
            dismiss_notifications()?;
            let trusted_download = journey_paths.download("toolbar-setup.exe");
            FixtureSample::ToolbarInstaller.place_at(&trusted_download)?;
            wait_until_blocked(&trusted_download)?;
            wait_for_notification(&trusted_download).map(|_matching| ())
        },
    )?;

    journey_run.step(
        "ignoring a detection for me works from a scan result and silences background alerts",
        |journey_run| {
            dismiss_notifications()?;
            let ignored_copy = journey_paths.document("ignored/payroll-macro.docm");
            FixtureSample::PayrollDocument.place_at(&ignored_copy)?;
            scan_with_lazaret(&[&ignored_copy])?;
            journey_run
                .widgets
                .await_shown_within(&button(widgets::quarantine_all(1)), LONG_WAIT)?;
            journey_run.press(&button(widgets::more_actions_tooltip()))?;
            let threat_actions_menu =
                WidgetTarget::new(WidgetRole::Menu, widgets::more_actions_tooltip());
            journey_run.expect_behaviour(
                "the threat menu items have accessible names",
                |journey_run| {
                    journey_run.await_shown(
                        &WidgetTarget::new(
                            WidgetRole::MenuItem,
                            widgets::ignore_detection_for_me(),
                        )
                        .inside(threat_actions_menu.clone()),
                    )
                },
            );
            journey_run.expect_behaviour(
                "the actions in a scan result's menu are enabled",
                |journey_run| {
                    choose_menu_item_by_keyboard(
                        journey_run,
                        IGNORE_DETECTION_MENU_POSITION,
                        &WidgetTarget::alert(widgets::ignore_detection_heading()),
                    )
                },
            );
            if !journey_run
                .widgets
                .is_shown(&WidgetTarget::alert(widgets::ignore_detection_heading()))
            {
                send_keys(&[KeyInput::Escape])?;
                return Ok(());
            }
            journey_run.press(&in_alert(
                button(widgets::ignore_button()),
                widgets::ignore_detection_heading(),
            ))?;
            wait_until("the detection ignored for me", SHORT_WAIT, || {
                Ok(journey_run
                    .database
                    .ignored_signatures()?
                    .iter()
                    .any(|signature_name| {
                        signature_name.starts_with("Doc.Fixture.FalsePositive-1")
                    }))
            })?;
            let ignored_download = journey_paths.download("payroll-macro.docm");
            FixtureSample::PayrollDocument.place_at(&ignored_download)?;
            wait_until(
                "clamonacc scanning the ignored download",
                Duration::from_secs(60),
                || {
                    Ok(
                        !unit_journal_lines_mentioning(ON_ACCESS_UNIT, &ignored_download)?
                            .is_empty(),
                    )
                },
            )?;
            settle(Duration::from_secs(10));
            ensure(
                notifications_mentioning(&ignored_download)?.is_empty(),
                || "alerted about a detection the user ignored".to_owned(),
            )?;
            ensure(
                journey_run
                    .database
                    .stored_detection(&ignored_download)?
                    .is_none(),
                || "recorded a detection the user ignored".to_owned(),
            )?;
            ensure(!Path::new(GLOBAL_IGNORE_LIST).exists(), || {
                "ignoring for me changed the list for all users".to_owned()
            })
        },
    )?;

    journey_run.step(
        "removing my trust and my ignored detection brings both back",
        |journey_run| {
            show_page(ApplicationPage::TrustedDetections)?;
            journey_run.press(&in_row(
                button(findings::remove_tooltip()),
                sample_hashes.toolbar_installer.clone(),
            ))?;
            journey_run.press(&in_alert(
                button(widgets::stop_trusting_button()),
                widgets::stop_trusting_file_heading(),
            ))?;
            if let Some(ignored_signature) = journey_run
                .database
                .ignored_signatures()?
                .into_iter()
                .next()
            {
                journey_run.press(&in_row(
                    button(findings::remove_tooltip()),
                    ignored_signature,
                ))?;
                journey_run.press(&in_alert(
                    button(widgets::report_again_button()),
                    widgets::report_detection_again_heading(),
                ))?;
            }
            wait_until("no trust and no ignored detection left", SHORT_WAIT, || {
                Ok(journey_run.database.trusted_content_count()? == 0
                    && journey_run.database.ignored_signatures()?.is_empty())
            })?;
            scan_with_lazaret(&[&trusted_copy])?;
            quarantine_scan_results(journey_run, 1)?;
            wait_until_missing(&trusted_copy)
        },
    )?;

    journey_run.step(
        "Update Now fetches the fixed signatures and keeps the abuse.ch key out of the UI",
        |journey_run| {
            FakeInternetScenario::default()
                .serving_signatures(SignatureRelease::V2)
                .answering(
                    FakeRoute::UrlhausDownload,
                    RouteOverride::html_page(200, WRONG_URLHAUS_DIGEST).only_for(".sha256"),
                )
                .apply()?;
            activate_application_action(ApplicationAction::UpdateDefinitions)?;
            answer_every_password_prompt(SAMPLE_PASSWORD)?;
            wait_until("a finished update", Duration::from_secs(240), || {
                Ok(journey_run
                    .database
                    .update_activity_statuses()?
                    .iter()
                    .any(|activity_status| activity_status != "update_requested"))
            })?;
            take_screenshot("update_finished");
            let update_statuses = journey_run.database.update_activity_statuses()?;
            ensure(
                update_statuses
                    .iter()
                    .any(|activity_status| activity_status == "update_applied"),
                || format!("the update did not apply: {update_statuses:?}"),
            )?;
            ensure(
                fs::read_to_string(SIGNATURE_DATABASE_FILE)?.contains("Win.Fixture.Missed-1"),
                || "freshclam did not install the second release".to_owned(),
            )?;
            let urlhaus_requests = requests_to(FakeRoute::UrlhausDownload)?;
            ensure(
                urlhaus_requests
                    .iter()
                    .any(|urlhaus_request| urlhaus_request.path.contains(ABUSE_CH_KEY)),
                || "fangfrisch never asked URLhaus".to_owned(),
            )?;
            let stored_output = journey_run.database.activity_outputs()?;
            ensure(
                stored_output.contains("lazaret-fixture.hdb updated"),
                || format!("freshclam's update was not shown:\n{stored_output}"),
            )?;
            ensure(stored_output.contains("digest mismatch"), || {
                format!("fangfrisch's warning was not shown:\n{stored_output}")
            })?;
            ensure(
                !stored_output.contains(ABUSE_CH_KEY) && !stored_output.contains(VIRUSTOTAL_KEY),
                || "the update log saved an API key".to_owned(),
            )?;
            ensure(stored_output.contains("address hidden"), || {
                format!("the address was not hidden:\n{stored_output}")
            })?;
            ensure(
                !journey_run
                    .widgets
                    .is_shown(&label_containing(ABUSE_CH_KEY)),
                || "the abuse.ch key is on the screen".to_owned(),
            )
        },
    )?;

    journey_run.step(
        "after the update the false positive is no longer detected and its report is settled",
        |journey_run| {
            wait_until("clamd loading the fixed signatures", LONG_WAIT, || {
                Ok(!succeed(
                    "clamdscan",
                    &[
                        "--no-summary",
                        "--fdpass",
                        &FixtureSample::PayrollDocument.source_path(),
                    ],
                )
                .unwrap_or_else(|command_failure| command_failure.to_string())
                .contains("FOUND"))
            })?;
            show_page(ApplicationPage::Quarantine)?;
            journey_run.press(&button(findings::check_again_label()))?;
            journey_run
                .widgets
                .await_shown_within(&label_containing(findings::no_longer_detected()), LONG_WAIT)?;
            take_screenshot("false_positive_no_longer_detected");
            journey_run.expect_behaviour(
                "a signature update settles the false positive report",
                |journey_run| {
                    journey_run.database.wait_for_submission_status(
                        &submission(
                            &sample_hashes.payroll_document,
                            SubmissionService::Clamav,
                            SubmissionKind::FalsePositive,
                        ),
                        SubmissionStatus::FixedInDatabase,
                        Duration::from_secs(90),
                    )
                },
            );
            Ok(())
        },
    )?;

    journey_run.step(
        "the missed malware is detected since the update and its report is settled once \
         quarantined",
        |journey_run| {
            scan_with_lazaret(&[&missed_malware_file])?;
            quarantine_scan_results(journey_run, 1)?;
            wait_until_missing(&missed_malware_file)?;
            show_page(ApplicationPage::Quarantine)?;
            journey_run.press(&button(findings::check_again_label()))?;
            journey_run.expect_behaviour(
                "quarantining reported missed malware after the update settles its report",
                |journey_run| {
                    journey_run.database.wait_for_submission_status(
                        &submission(
                            &sample_hashes.missed_dropper,
                            SubmissionService::Clamav,
                            SubmissionKind::MissedMalware,
                        ),
                        SubmissionStatus::NowDetected,
                        LONG_WAIT,
                    )
                },
            );
            Ok(())
        },
    )?;

    journey_run.step(
        "restoring the fixed false positive gives back the same file",
        |journey_run| {
            let false_positive_title = show_quarantine_entry(journey_run, "payroll", "payroll")?;
            journey_run.press(&entry_button(
                findings::restore_label(),
                &false_positive_title,
            ))?;
            journey_run.press(&in_alert(
                button(findings::restore_button()),
                findings::restore_file_heading(),
            ))?;
            wait_until("the restored document", SHORT_WAIT, || {
                Ok(Path::new(&false_positive_file).exists())
            })?;
            let restored_hash = hex::encode(sha2::Digest::finalize(sha2::Digest::chain_update(
                <sha2::Sha256 as sha2::Digest>::new(),
                fs::read(&false_positive_file)?,
            )));
            ensure(restored_hash == sample_hashes.payroll_document, || {
                "the restored file differs from the quarantined one".to_owned()
            })?;
            let restored_metadata = fs::metadata(&false_positive_file)?;
            ensure(
                std::os::unix::fs::PermissionsExt::mode(&restored_metadata.permissions()) & 0o777
                    == 0o640,
                || "the restored file lost its permissions".to_owned(),
            )?;
            ensure(
                succeed("stat", &["-c", "%U", &false_positive_file])?.trim() == SAMPLE_USER,
                || "the restored file has another owner".to_owned(),
            )
        },
    )?;

    journey_run.step(
        "restoring over a file that came back in the meantime is refused and keeps the entry",
        |journey_run| {
            fs::write(&archive_file, b"new holiday photos")?;
            let archive_title = show_quarantine_entry(journey_run, "holiday", "holiday")?;
            let payloads_before_restore = quarantine_payload_count(&journey_paths)?;
            journey_run.press(&entry_button(findings::restore_label(), &archive_title))?;
            journey_run.press(&in_alert(
                button(findings::restore_anyway_button()),
                findings::restore_file_clamav_heading(),
            ))?;
            settle(Duration::from_secs(5));
            take_screenshot("restore_collision");
            journey_run.expect_behaviour(
                "a refused restore keeps the newer file, the entry and its payload",
                |journey_run| {
                    ensure(fs::read(&archive_file)? == b"new holiday photos", || {
                        "the restore overwrote a newer file".to_owned()
                    })?;
                    ensure(
                        journey_run.database.has_quarantine_entry_for("holiday")?,
                        || "the entry disappeared after a refused restore".to_owned(),
                    )?;
                    ensure(
                        quarantine_payload_count(&journey_paths)? == payloads_before_restore,
                        || "the payload count changed after a refused restore".to_owned(),
                    )
                },
            );
            fs::remove_file(&archive_file)?;
            Ok(())
        },
    )?;

    journey_run.step(
        "restoring a file ClamAV still detects into a watched folder opens the detection again",
        |journey_run| {
            eicar_title = show_quarantine_entry(journey_run, "invoice.pdf.exe", "invoice")?;
            journey_run.press(&entry_button(findings::restore_label(), &eicar_title))?;
            journey_run.press(&in_alert(
                button(findings::restore_anyway_button()),
                findings::restore_file_clamav_heading(),
            ))?;
            wait_until("the restored invoice", SHORT_WAIT, || {
                Ok(Path::new(&blocked_file).exists())
            })?;
            ensure(
                !journey_run
                    .database
                    .has_quarantine_entry_for("Downloads/invoice.pdf.exe")?,
                || "the restored entry is still listed".to_owned(),
            )?;
            wait_until_blocked(&blocked_file)?;
            wait_until(
                "the old resolution cleared",
                Duration::from_secs(60),
                || {
                    Ok(journey_run
                        .database
                        .stored_detection(&blocked_file)?
                        .is_some_and(|detection| detection.resolution_kind.is_empty()))
                },
            )?;
            wait_for_tray_status(TrayStatus::NeedsAttention)
        },
    )?;

    journey_run.step(
        "deleting for good and clearing old items remove the payloads",
        |journey_run| {
            let renamed_title = show_quarantine_entry(journey_run, "-copy.exe", "copy")?;
            let payload_path = journey_run
                .database
                .payload_path_for("-copy.exe")?
                .ok_or_else(|| {
                    JourneyFailure::ExpectationFailed("no payload for the copy".to_owned())
                })?;
            journey_run.press(&entry_button(
                findings::delete_entry_label(),
                &renamed_title,
            ))?;
            journey_run.press(&in_alert(
                button(findings::delete_entries()),
                findings::delete_quarantined_file(),
            ))?;
            wait_until_missing(&payload_path)?;
            journey_run.database.backdate_quarantine_entry("holiday")?;
            show_page(ApplicationPage::Quarantine)?;
            journey_run.press(&button(findings::refresh_tooltip()))?;
            journey_run.press(&button(findings::more_tooltip()))?;
            journey_run.press(&button(findings::clear_old_items_label()))?;
            journey_run.press(&in_alert(
                button(findings::delete_entries()),
                findings::clear_old_items(),
            ))?;
            wait_until("the old archive entry cleared", SHORT_WAIT, || {
                Ok(!journey_run.database.has_quarantine_entry_for("holiday")?)
            })
        },
    )?;

    journey_run.step(
        "a detected file Lazaret can read but not remove leaves no orphaned payload",
        |journey_run| {
            as_root(&[
                "cp",
                &FixtureSample::EicarInvoice.source_path(),
                UNREMOVABLE_FILE,
            ])?;
            as_root(&["chmod", "0644", UNREMOVABLE_FILE])?;
            let payloads_before_quarantine = quarantine_payload_count(&journey_paths)?;
            scan_with_lazaret(&[UNREMOVABLE_FILE])?;
            quarantine_scan_results(journey_run, 1)?;
            settle(Duration::from_secs(5));
            take_screenshot("unremovable_file");
            ensure(Path::new(UNREMOVABLE_FILE).exists(), || {
                "a file that can't be removed disappeared".to_owned()
            })?;
            journey_run.expect_behaviour(
                "a file that can't be removed leaves no pending record and no orphaned payload",
                |journey_run| {
                    ensure(journey_run.database.pending_move_count()? == 0, || {
                        "a pending quarantine record was left behind".to_owned()
                    })?;
                    ensure(
                        quarantine_payload_count(&journey_paths)? == payloads_before_quarantine,
                        || "a payload was left in the quarantine".to_owned(),
                    )
                },
            );
            Ok(())
        },
    )?;

    journey_run.step(
        "a scheduled scan only reports while Lazaret is open, and quarantines once it is closed",
        |journey_run| {
            let scheduled_target = journey_paths.document("scheduled");
            let scheduled_file = format!("{scheduled_target}/invoice-copy.exe");
            FixtureSample::EicarInvoice.place_at(&scheduled_file)?;
            let scheduled_scan_arguments = [
                "--user",
                "--wait",
                "--collect",
                "/run/current-system/sw/bin/lazaret",
                "--scheduled-scan",
                "--auto-quarantine",
                "--target",
                scheduled_target.as_str(),
            ];
            let _open_scan = crate::vm_journeys::system_commands::run_command(
                "systemd-run",
                &scheduled_scan_arguments,
            )?;
            ensure(Path::new(&scheduled_file).exists(), || {
                "the scheduled scan quarantined while Lazaret was open".to_owned()
            })?;
            succeed("systemctl", &["--user", "stop", "lazaret.service"])?;
            let _closed_scan = crate::vm_journeys::system_commands::run_command(
                "systemd-run",
                &scheduled_scan_arguments,
            )?;
            wait_until_missing(&scheduled_file)?;
            ensure(
                journey_run
                    .database
                    .has_quarantine_entry_for("scheduled/invoice-copy.exe")?,
                || "the closed-app scheduled scan did not record the quarantined file".to_owned(),
            )?;
            succeed("systemctl", &["--user", "start", "lazaret.service"])?;
            wait_for_lazaret_service()
        },
    )?;

    journey_run.step(
        "detections missed while closed are summarised once at the next start",
        |_journey_run| {
            let missed_while_closed = journey_paths.download("while-closed.exe");
            dismiss_notifications()?;
            succeed("systemctl", &["--user", "stop", "lazaret.service"])?;
            FixtureSample::EicarInvoice.place_at(&missed_while_closed)?;
            wait_until_blocked(&missed_while_closed)?;
            settle(Duration::from_secs(2));
            succeed("systemctl", &["--user", "start", "lazaret.service"])?;
            wait_for_notification(&findings::count_detection_not_running(1))?;
            ensure(
                notifications_mentioning(&missed_while_closed)?.is_empty(),
                || "alerted individually about a missed file".to_owned(),
            )?;
            assert_lines_stay_at(&missed_while_closed, Duration::from_secs(10))
        },
    )?;

    journey_run.step("neither API key ever reached the database", |journey_run| {
        let database_bytes = journey_run.database.raw_file_contents()?;
        for api_key in [ABUSE_CH_KEY, VIRUSTOTAL_KEY] {
            ensure(
                !database_bytes
                    .windows(api_key.len())
                    .any(|byte_window| byte_window == api_key.as_bytes()),
                || "an API key is in the database".to_owned(),
            )?;
        }
        Ok(())
    })?;

    journey_run.finish()
}
