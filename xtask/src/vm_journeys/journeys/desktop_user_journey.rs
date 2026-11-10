use std::{fs, os::unix::ffi::OsStrExt, path::Path, time::Duration};

use gtk_scene_capture::accessible_widget_lookup::{NameMatch, WidgetRole, WidgetTarget};
use serde_json::Value;

use crate::vm_journeys::{
    harness::{
        desktop_environment::{
            ApplicationAction, ApplicationPage, KeyInput, NotificationUrgency, TrayStatus,
            activate_application_action, answer_every_password_prompt,
            answer_password_prompts_if_asked, dismiss_notifications, invoke_latest_notification,
            lazaret_window_is_visible, notifications_mentioning, scan_with_lazaret, send_keys,
            show_page, take_screenshot, wait_for_lazaret_service, wait_for_lazaret_window,
            wait_for_notification, wait_for_notification_withdrawn, wait_for_tray_status,
        },
        fixture_corpus::FixtureSample,
        journey_failure::{JourneyFailure, ensure},
        journey_run::{JourneyRun, SAMPLE_HOME, SAMPLE_PASSWORD},
        lazaret_database::{SubmissionKey, SubmissionKind, SubmissionService, SubmissionStatus},
        session_widgets::{button, in_alert, label_containing, row_titled},
        system_commands::{
            as_root, journal_text, settle, succeed, unit_journal_lines_mentioning, wait_until,
        },
        ui_messages::{MessageCatalog, findings, host, text_before_placeholder, widgets},
    },
    remote_nodes::{
        fake_internet::{
            clear_request_log, request_count_stays, requests_to, wait_for_first_request,
        },
        fake_internet_protocol::{
            BAZAAR_FIXTURE_FAMILY, FakeInternetScenario, FakeRoute, NetworkCondition,
            RouteOverride, SignatureRelease,
        },
        team_share::colleague_places_on_team_share,
    },
};

const ON_ACCESS_UNIT: &str = "clamav-clamonacc.service";
const ABUSE_CH_KEY: &str = "FIXTUREKEYMUSTNEVERAPPEAR";
const VIRUSTOTAL_KEY: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const NON_UTF8_FILE_NAME: &[u8] = b"r\xe9sum\xe9-copy.exe";
const SIGNATURE_DATABASE_FILE: &str = "/var/lib/clamav/lazaret-fixture.hdb";
const FIXTURE_SIGNATURE_ADDRESS: &str = "https://signatures.fixture.test/lazaret-fixture.hdb";
const IGNORE_DETECTION_MENU_POSITION: usize = 1;
const WRONG_URLHAUS_DIGEST: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";
const SHORT_WAIT: Duration = Duration::from_secs(30);
const LONG_WAIT: Duration = Duration::from_secs(120);
const UNREACHABLE_UPDATE_WAIT: Duration = Duration::from_secs(420);

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
) -> Result<(), JourneyFailure> {
    show_page(ApplicationPage::Quarantine)?;
    show_quarantine_list(journey_run)?;
    if !journey_run.widgets.is_shown(&quarantine_search_field()) {
        journey_run.press(&WidgetTarget::toggle_button(findings::search_tooltip(
            &MessageCatalog::source_language(),
        )))?;
    }
    journey_run
        .widgets
        .type_into(&quarantine_search_field(), search_text)?;
    let entry_title = journey_run
        .database
        .wait_for_quarantined_signature(path_fragment)?;
    journey_run.press(&row_titled(entry_title))?;
    journey_run.await_shown(&button(findings::trust_label(
        &MessageCatalog::source_language(),
    )))
}

fn show_quarantine_list(journey_run: &JourneyRun) -> Result<(), JourneyFailure> {
    let back_button = button(findings::back_quarantine_tooltip(
        &MessageCatalog::source_language(),
    ));
    if journey_run.widgets.is_shown(&back_button) {
        journey_run.press(&back_button)?;
    }
    Ok(())
}

fn choose_menu_item_by_keyboard(
    journey_run: &JourneyRun,
    item_position: usize,
    opened_dialog: &WidgetTarget,
) -> Result<(), JourneyFailure> {
    for extra_down_presses in 0..2 {
        if extra_down_presses > 0 {
            send_keys(&[KeyInput::Escape])?;
            journey_run.press(&button(widgets::more_actions_tooltip(
                &MessageCatalog::source_language(),
            )))?;
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

fn submit_quarantined_entry(
    journey_run: &JourneyRun,
    target_label: String,
    confirm_heading: String,
    confirm_label: String,
) -> Result<(), JourneyFailure> {
    journey_run.press(&button(findings::submit_quarantine_label(
        &MessageCatalog::source_language(),
    )))?;
    journey_run.press(&in_alert(
        button(target_label),
        findings::choose_service_heading(&MessageCatalog::source_language()),
    ))?;
    journey_run.press(&in_alert(button(confirm_label), confirm_heading))
}

fn quarantine_scan_results(
    journey_run: &JourneyRun,
    threat_count: u32,
) -> Result<(), JourneyFailure> {
    journey_run.widgets.press_within(
        &button(widgets::quarantine_all(
            &MessageCatalog::source_language(),
            threat_count,
        )),
        LONG_WAIT,
    )?;
    journey_run.press(&in_alert(
        button(widgets::quarantine_label(&MessageCatalog::source_language())),
        widgets::move_quarantine(&MessageCatalog::source_language()),
    ))
}

fn finished_update_statuses(journey_run: &JourneyRun) -> Result<Vec<String>, JourneyFailure> {
    Ok(journey_run
        .database
        .update_activity_statuses()?
        .into_iter()
        .filter(|activity_status| activity_status != "update_requested")
        .collect())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UpdateAuthorization {
    AsksForPassword,
    MayReuseEarlierAuthorization,
}

fn run_update_now(
    journey_run: &JourneyRun,
    update_authorization: UpdateAuthorization,
    update_timeout: Duration,
) -> Result<String, JourneyFailure> {
    let earlier_finished_count = finished_update_statuses(journey_run)?.len();
    activate_application_action(ApplicationAction::UpdateDefinitions)?;
    match update_authorization {
        UpdateAuthorization::AsksForPassword => answer_every_password_prompt(SAMPLE_PASSWORD)?,
        UpdateAuthorization::MayReuseEarlierAuthorization => {
            answer_password_prompts_if_asked(SAMPLE_PASSWORD)?;
        }
    }
    wait_until("a finished update", update_timeout, || {
        Ok(finished_update_statuses(journey_run)?.len() > earlier_finished_count)
    })?;
    finished_update_statuses(journey_run)?
        .into_iter()
        .nth(earlier_finished_count)
        .ok_or_else(|| JourneyFailure::ExpectationFailed("no finished update".to_owned()))
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

pub fn run_desktop_user_journey() -> Result<(), JourneyFailure> {
    let journey_paths = JourneyPaths::for_sample_user();
    let invoice_sha256 = FixtureSample::EicarInvoice.content_sha256()?;
    let blocked_file = journey_paths.download("invoice.pdf.exe");
    let archive_file = journey_paths.document("holiday photos.zip");
    let false_positive_file = journey_paths.document("payroll-macro.docm");
    let unwanted_program_file = journey_paths.document("toolbar-setup.exe");
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
    clear_request_log()?;
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
            let blocked_heading = text_before_placeholder(|signature_name| {
                findings::malware_blocked_signature(
                    &MessageCatalog::source_language(),
                    signature_name,
                )
            });
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
            FakeInternetScenario::default()
                .with_network(NetworkCondition::RefusedConnections)
                .apply()?;
            journey_run.press(&button(findings::look_up_incident_signature(
                &MessageCatalog::source_language(),
            )))?;
            journey_run.await_shown(&label_containing(text_before_placeholder(
                |request_text| {
                    host::request_failed_bazaar(&MessageCatalog::source_language(), request_text)
                },
            )))?;
            FakeInternetScenario::default().apply()?;
            journey_run.press(&button(findings::look_up_incident_signature(
                &MessageCatalog::source_language(),
            )))?;
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
            journey_run.press(&button(findings::quarantine_incident(
                &MessageCatalog::source_language(),
            )))?;
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
            findings::access_scanning_scheduled_scans(&MessageCatalog::source_language()),
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
            journey_run.press(&button(findings::ask_incident_deletion(
                &MessageCatalog::source_language(),
            )))?;
            ensure(Path::new(&deleted_file).exists(), || {
                "the first choice already deleted the file".to_owned()
            })?;
            journey_run.press(&button(findings::delete_incident_file(
                &MessageCatalog::source_language(),
            )))?;
            wait_until_missing(&deleted_file)?;
            wait_for_notification_withdrawn(&deleted_file)?;
            wait_for_tray_status(TrayStatus::Active)
        },
    )?;

    journey_run.step(
        "a file a colleague put on the team share is blocked when it is opened",
        |journey_run| {
            dismiss_notifications()?;
            let shared_file = colleague_places_on_team_share(FixtureSample::EicarInvoice)?;
            journey_run.expect_behaviour(
                "on-access scanning blocks a file another machine wrote to an NFS share",
                |_journey_run| {
                    wait_until_blocked(&shared_file)?;
                    wait_for_notification(&shared_file).map(|_shared_alerts| ())
                },
            );
            Ok(())
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

    journey_run.step(
        "uploading the invoice to MalwareBazaar names it by its hash and is never repeated",
        |journey_run| {
            show_quarantine_entry(journey_run, "invoice.pdf.exe", "invoice")?;
            submit_quarantined_entry(
                journey_run,
                findings::malwarebazaar_sample_choice(&MessageCatalog::source_language()),
                widgets::upload_malwarebazaar_heading(&MessageCatalog::source_language()),
                widgets::upload_publicly_button(&MessageCatalog::source_language()),
            )?;
            let upload_request = wait_for_first_request(FakeRoute::BazaarUpload)?;
            let uploaded_file = upload_request.uploaded_file("file").ok_or_else(|| {
                JourneyFailure::ExpectationFailed("the upload had no file".to_owned())
            })?;
            ensure(uploaded_file.uploaded_name == invoice_sha256, || {
                format!("{uploaded_file:?}")
            })?;
            ensure(uploaded_file.content_sha256 == invoice_sha256, || {
                "the upload is not the quarantined file".to_owned()
            })?;
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
                    &invoice_sha256,
                    SubmissionService::MalwareBazaar,
                    SubmissionKind::MalwareSample,
                ),
                SubmissionStatus::UploadAccepted,
                Duration::from_secs(90),
            )?;
            show_quarantine_entry(journey_run, "invoice.pdf.exe", "invoice")?;
            submit_quarantined_entry(
                journey_run,
                findings::malwarebazaar_sample_choice(&MessageCatalog::source_language()),
                widgets::upload_malwarebazaar_heading(&MessageCatalog::source_language()),
                widgets::upload_publicly_button(&MessageCatalog::source_language()),
            )?;
            request_count_stays(FakeRoute::BazaarUpload, 1)
        },
    )?;

    journey_run.step(
        "a scan result's menu offers working actions",
        |journey_run| {
            let menu_sample = journey_paths.document("menu/payroll-macro.docm");
            FixtureSample::PayrollDocument.place_at(&menu_sample)?;
            scan_with_lazaret(&[&menu_sample])?;
            journey_run.widgets.await_shown_within(
                &button(widgets::quarantine_all(
                    &MessageCatalog::source_language(),
                    1,
                )),
                LONG_WAIT,
            )?;
            journey_run.press(&button(widgets::more_actions_tooltip(
                &MessageCatalog::source_language(),
            )))?;
            let threat_actions_menu = WidgetTarget::new(
                WidgetRole::Menu,
                widgets::more_actions_tooltip(&MessageCatalog::source_language()),
            );
            journey_run.expect_behaviour(
                "the threat menu items have accessible names",
                |journey_run| {
                    journey_run.widgets.await_shown_within(
                        &WidgetTarget::new(
                            WidgetRole::MenuItem,
                            widgets::ignore_detection_for_me(&MessageCatalog::source_language()),
                        )
                        .inside(threat_actions_menu.clone()),
                        Duration::from_secs(5),
                    )
                },
            );
            journey_run.expect_behaviour(
                "the actions in a scan result's menu are enabled",
                |journey_run| {
                    choose_menu_item_by_keyboard(
                        journey_run,
                        IGNORE_DETECTION_MENU_POSITION,
                        &WidgetTarget::alert(widgets::ignore_detection_heading(
                            &MessageCatalog::source_language(),
                        )),
                    )
                },
            );
            send_keys(&[KeyInput::Escape])
        },
    )?;

    journey_run.step(
        "Update Now reports a failure while the internet silently drops every packet",
        |journey_run| {
            FakeInternetScenario::default()
                .serving_signatures(SignatureRelease::V2)
                .with_network(NetworkCondition::SilentDrop)
                .apply()?;
            let update_status = run_update_now(
                journey_run,
                UpdateAuthorization::AsksForPassword,
                UNREACHABLE_UPDATE_WAIT,
            )?;
            take_screenshot("update_unreachable");
            ensure(update_status == "update_failed", || {
                format!("an unreachable mirror ended as {update_status}")
            })?;
            ensure(
                !fs::read_to_string(SIGNATURE_DATABASE_FILE)?.contains("Win.Fixture.Missed-1"),
                || "signatures changed although the mirror was unreachable".to_owned(),
            )
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
            let update_status = run_update_now(
                journey_run,
                UpdateAuthorization::MayReuseEarlierAuthorization,
                Duration::from_secs(240),
            )?;
            take_screenshot("update_finished");
            ensure(update_status == "update_applied", || {
                format!("the update did not apply: {update_status}")
            })?;
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
        "detections missed while closed are summarised once at the next start",
        |_journey_run| {
            let missed_while_closed = journey_paths.download("while-closed.exe");
            dismiss_notifications()?;
            succeed("systemctl", &["--user", "stop", "lazaret.service"])?;
            FixtureSample::EicarInvoice.place_at(&missed_while_closed)?;
            wait_until_blocked(&missed_while_closed)?;
            settle(Duration::from_secs(2));
            succeed("systemctl", &["--user", "start", "lazaret.service"])?;
            wait_for_notification(&findings::count_detection_not_running(
                &MessageCatalog::source_language(),
                1,
            ))?;
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
