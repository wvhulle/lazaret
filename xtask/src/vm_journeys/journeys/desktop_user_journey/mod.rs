mod documents_and_submissions;
mod on_access_alerts;
mod signature_updates;

use std::{ffi::OsStr, os::unix::ffi::OsStrExt, path::PathBuf, time::Duration};

use crate::vm_journeys::{
    harness::{
        fixture_corpus::FixtureSample,
        journey_progress::SAMPLE_HOME,
        lazaret_application::wait_for_lazaret_service,
        lazaret_database::LazaretDatabase,
        session_widgets::SessionWidgets,
        system_commands::{run_command, wait_until},
    },
    remote_nodes::fake_internet::clear_request_log,
};

const ABUSE_CH_KEY: &str = "FIXTUREKEYMUSTNEVERAPPEAR";
const VIRUSTOTAL_KEY: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const NON_UTF8_FILE_NAME: &[u8] = b"r\xe9sum\xe9-copy.exe";
const FIXTURE_SIGNATURE_ADDRESS: &str = "https://signatures.fixture.test/lazaret-fixture.hdb";
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

    fn non_utf8_document(&self) -> PathBuf {
        PathBuf::from(&self.documents).join(OsStr::from_bytes(NON_UTF8_FILE_NAME))
    }
}

fn contains_bytes(searched_bytes: &[u8], wanted_bytes: &[u8]) -> bool {
    searched_bytes
        .windows(wanted_bytes.len())
        .any(|byte_window| byte_window == wanted_bytes)
}

fn wait_for_fake_internet() {
    wait_until(
        "the fake internet answering over TLS",
        Duration::from_secs(60),
        || {
            let signature_download =
                run_command("curl", &["--fail", "--silent", FIXTURE_SIGNATURE_ADDRESS]);
            signature_download.exit_status.success()
                && signature_download
                    .combined_output
                    .contains("Win.Fixture.Agent-1")
        },
    );
}

pub fn run_desktop_user_journey() {
    let journey_paths = JourneyPaths::for_sample_user();
    let invoice_sha256 = FixtureSample::EicarInvoice.content_sha256();
    let blocked_file = journey_paths.download("invoice.pdf.exe");

    wait_for_fake_internet();
    clear_request_log();
    wait_for_lazaret_service();
    let session_widgets = SessionWidgets::connect();
    let database = LazaretDatabase::for_home(SAMPLE_HOME);

    on_access_alerts::lazaret_starts_without_window();
    on_access_alerts::on_access_scanning_is_ready();
    on_access_alerts::downloaded_invoice_raises_one_urgent_alert(&database, &blocked_file);
    on_access_alerts::alert_opens_detection_and_look_up_sends_only_signature(&session_widgets);
    on_access_alerts::quarantining_from_timeline_neutralises_file(
        &session_widgets,
        &database,
        &blocked_file,
        &journey_paths.quarantine_directory,
    );
    on_access_alerts::tray_action_opens_detections(&session_widgets);
    on_access_alerts::deleting_takes_two_choices_and_withdraws_alert(
        &session_widgets,
        &journey_paths,
    );
    on_access_alerts::colleague_file_on_team_share_is_blocked();
    documents_and_submissions::manual_scan_quarantines_documents(
        &session_widgets,
        &database,
        &journey_paths,
    );
    documents_and_submissions::invoice_upload_is_named_by_hash_and_never_repeated(
        &session_widgets,
        &database,
        &invoice_sha256,
    );
    documents_and_submissions::scan_result_menu_offers_working_actions(
        &session_widgets,
        &journey_paths,
    );
    signature_updates::update_fails_while_internet_drops_packets(&database);
    signature_updates::update_fetches_fixed_signatures_and_hides_key(&session_widgets, &database);
    on_access_alerts::missed_detections_summarised_at_next_start(&journey_paths);
    signature_updates::api_keys_never_reach_database(&database);
}
