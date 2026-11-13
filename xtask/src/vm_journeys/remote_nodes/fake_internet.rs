use std::time::Duration;

use crate::vm_journeys::{
    harness::system_commands::{poll_until, settle, succeed},
    remote_nodes::fake_internet_protocol::{FakeInternetScenario, FakeRoute, LoggedRequest},
};

const REQUEST_LOG_ADDRESS: &str = "http://internet:8080/requests";
const SCENARIO_ADDRESS: &str = "http://internet:8080/scenario";
const CONTROL_CURL_ARGUMENTS: [&str; 3] = ["--fail", "--silent", "--show-error"];

impl FakeInternetScenario {
    pub fn apply(&self) {
        let scenario_json = serde_json::to_string(self)
            .unwrap_or_else(|encoding_failure| panic!("{self:?} is not JSON: {encoding_failure}"));
        control_request(&[
            "--request",
            "PUT",
            "--header",
            "Content-Type: application/json",
            "--data-binary",
            &scenario_json,
            SCENARIO_ADDRESS,
        ]);
    }
}

fn control_request(request_arguments: &[&str]) -> String {
    let curl_arguments = CONTROL_CURL_ARGUMENTS
        .iter()
        .chain(request_arguments)
        .copied()
        .collect::<Vec<_>>();
    succeed("curl", &curl_arguments)
}

pub fn clear_request_log() {
    control_request(&["--request", "DELETE", REQUEST_LOG_ADDRESS]);
}

pub fn logged_requests() -> Vec<LoggedRequest> {
    let request_log = control_request(&[REQUEST_LOG_ADDRESS]);
    serde_json::from_str(&request_log).unwrap_or_else(|parse_failure| {
        panic!("the fake internet's request log is unreadable: {parse_failure}\n{request_log}")
    })
}

pub fn requests_to(fake_route: FakeRoute) -> Vec<LoggedRequest> {
    logged_requests()
        .into_iter()
        .filter(|logged_request| logged_request.route_name == fake_route)
        .collect()
}

pub fn wait_for_request_after(
    fake_route: FakeRoute,
    earlier_count: usize,
    timeout: Duration,
) -> LoggedRequest {
    poll_until(&format!("a {fake_route:?} request"), timeout, || {
        requests_to(fake_route).into_iter().nth(earlier_count)
    })
}

pub fn wait_for_first_request(fake_route: FakeRoute) -> LoggedRequest {
    wait_for_request_after(fake_route, 0, Duration::from_secs(60))
}

pub fn request_count_stays(fake_route: FakeRoute, expected_count: usize) {
    settle(Duration::from_secs(5));
    let routed_count = requests_to(fake_route).len();
    assert_eq!(
        routed_count, expected_count,
        "{fake_route:?} was requested {routed_count} times, expected {expected_count}"
    );
}
