use std::{collections::BTreeMap, fs, time::Duration};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::vm_journeys::{
    journey_failure::JourneyFailure,
    system_commands::{settle, wait_until, write_root_file},
};

const REQUEST_LOG_PATH: &str = "/var/lib/fake-internet/requests.jsonl";
const SCENARIO_PATH: &str = "/var/lib/fake-internet/scenario.json";
pub const BAZAAR_FIXTURE_FAMILY: &str = "FixtureLoader";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FakeRoute {
    BazaarGetClamavinfo,
    BazaarGetInfo,
    BazaarUpload,
    BazaarUnknownQuery,
    CirclLookup,
    VirustotalFileReport,
    VirustotalUpload,
    VirustotalAnalysis,
    VirustotalUnknownPath,
    ClamavReportForm,
    ClamavPresigned,
    ClamavSubmit,
    ClamavSampleUpload,
    ClamavUnknownPath,
    SignatureDownload,
    UrlhausDownload,
    UnknownHost,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UploadedFile {
    pub uploaded_name: String,
    pub content_sha256: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum FormField {
    UploadedFileField(UploadedFile),
    TextField(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoggedRequest {
    pub path: String,
    pub headers: BTreeMap<String, String>,
    pub form_fields: BTreeMap<String, FormField>,
    pub body_byte_size: u64,
    pub route_name: FakeRoute,
}

impl LoggedRequest {
    pub fn text_field(&self, field_name: &str) -> Option<&str> {
        match self.form_fields.get(field_name)? {
            FormField::TextField(field_text) => Some(field_text),
            FormField::UploadedFileField(_uploaded_file) => None,
        }
    }

    pub fn uploaded_file(&self, field_name: &str) -> Option<&UploadedFile> {
        match self.form_fields.get(field_name)? {
            FormField::UploadedFileField(uploaded_file) => Some(uploaded_file),
            FormField::TextField(_field_text) => None,
        }
    }

    pub fn header(&self, header_name: &str) -> Option<&str> {
        self.headers.get(header_name).map(String::as_str)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct RouteOverride {
    pub status: u16,
    pub body: Value,
    pub content_type: &'static str,
    #[serde(
        rename = "only_hash",
        skip_serializing_if = "RequestFilter::applies_to_every_request"
    )]
    pub request_filter: RequestFilter,
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum RequestFilter {
    EveryRequest,
    MatchingFragment(String),
}

impl RequestFilter {
    const fn applies_to_every_request(&self) -> bool {
        matches!(self, Self::EveryRequest)
    }
}

impl RouteOverride {
    pub fn html_page(status: u16, page_text: &str) -> Self {
        Self {
            status,
            body: Value::String(page_text.to_owned()),
            content_type: "text/html",
            request_filter: RequestFilter::EveryRequest,
        }
    }

    pub fn only_for(self, hash_or_path_fragment: &str) -> Self {
        Self {
            request_filter: RequestFilter::MatchingFragment(hash_or_path_fragment.to_owned()),
            ..self
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SignatureRelease {
    V2,
}

#[derive(Debug, Default, Serialize)]
pub struct FakeInternetScenario {
    #[serde(skip_serializing_if = "BTreeMap::is_empty", flatten)]
    route_overrides: BTreeMap<String, RouteOverride>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature_release: Option<SignatureRelease>,
}

impl FakeInternetScenario {
    pub fn answering(mut self, fake_route: FakeRoute, route_override: RouteOverride) -> Self {
        let route_key = serde_json::to_value(fake_route)
            .ok()
            .and_then(|route_value| route_value.as_str().map(str::to_owned))
            .unwrap_or_default();
        self.route_overrides.insert(route_key, route_override);
        self
    }

    pub const fn serving_signatures(mut self, signature_release: SignatureRelease) -> Self {
        self.signature_release = Some(signature_release);
        self
    }

    pub fn apply(&self) -> Result<(), JourneyFailure> {
        write_root_file(SCENARIO_PATH, &serde_json::to_string(self)?)
    }
}

pub fn logged_requests() -> Result<Vec<LoggedRequest>, JourneyFailure> {
    let request_log = fs::read_to_string(REQUEST_LOG_PATH).unwrap_or_default();
    request_log
        .lines()
        .filter(|log_line| !log_line.is_empty())
        .map(|log_line| serde_json::from_str(log_line).map_err(JourneyFailure::from))
        .collect()
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
    crate::vm_journeys::journey_failure::ensure(routed_count == expected_count, || {
        format!("{fake_route:?} was requested {routed_count} times, expected {expected_count}")
    })
}
