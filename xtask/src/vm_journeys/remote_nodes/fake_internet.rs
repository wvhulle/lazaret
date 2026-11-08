use std::time::Duration;

use crate::vm_journeys::{
    harness::{
        journey_failure::{JourneyFailure, ensure},
        system_commands::{settle, succeed, wait_until},
    },
    remote_nodes::fake_internet_protocol::{FakeInternetScenario, FakeRoute, LoggedRequest},
};

const REQUEST_LOG_ADDRESS: &str = "http://internet:8080/requests";
const SCENARIO_ADDRESS: &str = "http://internet:8080/scenario";
const CONTROL_CURL_ARGUMENTS: [&str; 3] = ["--fail", "--silent", "--show-error"];

impl FakeInternetScenario {
    pub fn apply(&self) -> Result<(), JourneyFailure> {
        let scenario_json = serde_json::to_string(self)?;
        control_request(&[
            "--request",
            "PUT",
            "--header",
            "Content-Type: application/json",
            "--data-binary",
            &scenario_json,
            SCENARIO_ADDRESS,
        ])?;
        Ok(())
    }
}

fn control_request(request_arguments: &[&str]) -> Result<String, JourneyFailure> {
    let curl_arguments = CONTROL_CURL_ARGUMENTS
        .iter()
        .chain(request_arguments)
        .copied()
        .collect::<Vec<_>>();
    succeed("curl", &curl_arguments)
}

pub fn clear_request_log() -> Result<(), JourneyFailure> {
    control_request(&["--request", "DELETE", REQUEST_LOG_ADDRESS])?;
    Ok(())
}

pub fn logged_requests() -> Result<Vec<LoggedRequest>, JourneyFailure> {
    Ok(serde_json::from_str(&control_request(&[
        REQUEST_LOG_ADDRESS,
    ])?)?)
}

pub fn requests_to(fake_route: FakeRoute) -> Result<Vec<LoggedRequest>, JourneyFailure> {
    Ok(logged_requests()?
        .into_iter()
        .filter(|logged_request| logged_request.route_name == fake_route)
        .collect())
}

pub fn wait_for_request_after(
    fake_route: FakeRoute,
    earlier_count: usize,
    timeout: Duration,
) -> Result<LoggedRequest, JourneyFailure> {
    wait_until(&format!("a {fake_route:?} request"), timeout, || {
        Ok(requests_to(fake_route)?.len() > earlier_count)
    })?;
    requests_to(fake_route)?
        .into_iter()
        .nth(earlier_count)
        .ok_or_else(|| JourneyFailure::ExpectationFailed(format!("no {fake_route:?} request")))
}

pub fn wait_for_first_request(fake_route: FakeRoute) -> Result<LoggedRequest, JourneyFailure> {
    wait_for_request_after(fake_route, 0, Duration::from_secs(60))
}

pub fn request_count_stays(
    fake_route: FakeRoute,
    expected_count: usize,
) -> Result<(), JourneyFailure> {
    settle(Duration::from_secs(5));
    let routed_count = requests_to(fake_route)?.len();
    ensure(routed_count == expected_count, || {
        format!("{fake_route:?} was requested {routed_count} times, expected {expected_count}")
    })
}
