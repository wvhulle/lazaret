use std::{fs, time::Duration};

use super::{ABUSE_CH_KEY, VIRUSTOTAL_KEY, contains_bytes};
use crate::vm_journeys::{
    harness::{
        journey_progress::{SAMPLE_PASSWORD, announce_step},
        lazaret_application::{ApplicationAction, activate_application_action},
        lazaret_database::LazaretDatabase,
        session_widgets::{SessionWidgets, label_containing},
        sway_windows::{
            answer_every_password_prompt, answer_password_prompts_if_asked, take_screenshot,
        },
        system_commands::poll_until,
    },
    remote_nodes::{
        fake_internet::requests_to,
        fake_internet_protocol::{
            FakeInternetScenario, FakeRoute, NetworkCondition, RouteOverride, SignatureRelease,
        },
    },
};

const SIGNATURE_DATABASE_FILE: &str = "/var/lib/clamav/lazaret-fixture.hdb";
const WRONG_URLHAUS_DIGEST: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";
const UNREACHABLE_UPDATE_WAIT: Duration = Duration::from_secs(420);
const REACHABLE_UPDATE_WAIT: Duration = Duration::from_secs(240);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UpdateAuthorization {
    AsksForPassword,
    MayReuseEarlierAuthorization,
}

fn finished_update_statuses(database: &LazaretDatabase) -> Vec<String> {
    database
        .update_activity_statuses()
        .into_iter()
        .filter(|activity_status| activity_status != "update_requested")
        .collect()
}

fn run_update_now(
    database: &LazaretDatabase,
    update_authorization: UpdateAuthorization,
    update_timeout: Duration,
) -> String {
    let earlier_finished_count = finished_update_statuses(database).len();
    activate_application_action(ApplicationAction::UpdateDefinitions);
    match update_authorization {
        UpdateAuthorization::AsksForPassword => answer_every_password_prompt(SAMPLE_PASSWORD),
        UpdateAuthorization::MayReuseEarlierAuthorization => {
            answer_password_prompts_if_asked(SAMPLE_PASSWORD);
        }
    }
    poll_until("a finished update", update_timeout, || {
        finished_update_statuses(database)
            .into_iter()
            .nth(earlier_finished_count)
    })
}

fn installed_signatures() -> String {
    fs::read_to_string(SIGNATURE_DATABASE_FILE).unwrap_or_else(|read_failure| {
        panic!("{SIGNATURE_DATABASE_FILE} is unreadable: {read_failure}")
    })
}

pub fn update_fails_while_internet_drops_packets(database: &LazaretDatabase) {
    announce_step("Update Now reports a failure while the internet silently drops every packet");
    FakeInternetScenario::default()
        .serving_signatures(SignatureRelease::V2)
        .with_network(NetworkCondition::SilentDrop)
        .apply();
    let update_status = run_update_now(
        database,
        UpdateAuthorization::AsksForPassword,
        UNREACHABLE_UPDATE_WAIT,
    );
    take_screenshot("update_unreachable");
    assert_eq!(
        update_status, "update_failed",
        "an unreachable mirror ended as {update_status}"
    );
    assert!(
        !installed_signatures().contains("Win.Fixture.Missed-1"),
        "signatures changed although the mirror was unreachable"
    );
}

pub fn update_fetches_fixed_signatures_and_hides_key(
    session_widgets: &SessionWidgets,
    database: &LazaretDatabase,
) {
    announce_step(
        "Update Now fetches the fixed signatures and keeps the abuse.ch key out of the UI",
    );
    FakeInternetScenario::default()
        .serving_signatures(SignatureRelease::V2)
        .answering(
            FakeRoute::UrlhausDownload,
            RouteOverride::html_page(200, WRONG_URLHAUS_DIGEST).only_for(".sha256"),
        )
        .apply();
    let update_status = run_update_now(
        database,
        UpdateAuthorization::MayReuseEarlierAuthorization,
        REACHABLE_UPDATE_WAIT,
    );
    take_screenshot("update_finished");
    assert_eq!(
        update_status, "update_applied",
        "the update did not apply: {update_status}"
    );
    assert!(
        installed_signatures().contains("Win.Fixture.Missed-1"),
        "freshclam did not install the second release"
    );
    assert!(
        requests_to(FakeRoute::UrlhausDownload)
            .iter()
            .any(|urlhaus_request| urlhaus_request.path.contains(ABUSE_CH_KEY)),
        "fangfrisch never asked URLhaus"
    );
    let stored_output = database.activity_outputs();
    assert!(
        stored_output.contains("lazaret-fixture.hdb updated"),
        "freshclam's update was not shown:\n{stored_output}"
    );
    assert!(
        stored_output.contains("digest mismatch"),
        "fangfrisch's warning was not shown:\n{stored_output}"
    );
    assert!(
        !stored_output.contains(ABUSE_CH_KEY) && !stored_output.contains(VIRUSTOTAL_KEY),
        "the update log saved an API key"
    );
    assert!(
        stored_output.contains("address hidden"),
        "the address was not hidden:\n{stored_output}"
    );
    assert!(
        !session_widgets.is_shown(&label_containing(ABUSE_CH_KEY)),
        "the abuse.ch key is on the screen"
    );
}

pub fn api_keys_never_reach_database(database: &LazaretDatabase) {
    announce_step("neither API key ever reached the database");
    let database_bytes = database.raw_file_contents();
    for api_key in [ABUSE_CH_KEY, VIRUSTOTAL_KEY] {
        assert!(
            !contains_bytes(&database_bytes, api_key.as_bytes()),
            "an API key is in the database"
        );
    }
}
