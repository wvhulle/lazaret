use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const BAZAAR_FIXTURE_FAMILY: &str = "FixtureLoader";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FakeRoute {
    BazaarGetClamavinfo,
    BazaarGetInfo,
    BazaarUpload,
    BazaarUnknownQuery,
    CirclLookup,
    VirustotalFileReport,
    VirustotalUnknownPath,
    SignatureDownload,
    UrlhausDownload,
    UnknownHost,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UploadedFile {
    pub uploaded_name: String,
    pub content_sha256: String,
    pub byte_size: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FormField {
    UploadedFileField(UploadedFile),
    TextField(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoggedRequest {
    pub client_address: String,
    pub host: String,
    pub method: String,
    pub path: String,
    pub query: String,
    pub headers: BTreeMap<String, String>,
    pub form_fields: BTreeMap<String, FormField>,
    pub body_byte_size: u64,
    pub route_name: FakeRoute,
    pub answer_status: u16,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RouteOverride {
    pub status: u16,
    pub body: Value,
    pub content_type: String,
    pub request_filter: RequestFilter,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestFilter {
    EveryRequest,
    MatchingFragment(String),
}

impl RouteOverride {
    pub fn html_page(status: u16, page_text: &str) -> Self {
        Self {
            status,
            body: Value::String(page_text.to_owned()),
            content_type: "text/html".to_owned(),
            request_filter: RequestFilter::EveryRequest,
        }
    }

    pub fn only_for(self, hash_or_path_fragment: &str) -> Self {
        Self {
            request_filter: RequestFilter::MatchingFragment(hash_or_path_fragment.to_owned()),
            ..self
        }
    }

    pub fn applies_to(
        &self,
        requested_path: &str,
        form_fields: &BTreeMap<String, FormField>,
    ) -> bool {
        match &self.request_filter {
            RequestFilter::EveryRequest => true,
            RequestFilter::MatchingFragment(path_or_hash_fragment) => {
                requested_path.contains(path_or_hash_fragment.as_str())
                    || matches!(
                        form_fields.get("hash"),
                        Some(FormField::TextField(requested_hash))
                            if requested_hash == path_or_hash_fragment
                    )
            }
        }
    }

    pub fn body_bytes(&self) -> Vec<u8> {
        match &self.body {
            Value::String(body_text) => body_text.as_bytes().to_vec(),
            structured_body => structured_body.to_string().into_bytes(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SignatureRelease {
    #[default]
    V1,
    V2,
}

impl SignatureRelease {
    pub const fn directory_name(self) -> &'static str {
        match self {
            Self::V1 => "v1",
            Self::V2 => "v2",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkCondition {
    #[default]
    Reachable,
    RefusedConnections,
    SilentDrop,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FakeInternetScenario {
    pub route_overrides: BTreeMap<FakeRoute, RouteOverride>,
    pub signature_release: SignatureRelease,
    pub network_condition: NetworkCondition,
}

impl FakeInternetScenario {
    pub fn answering(mut self, fake_route: FakeRoute, route_override: RouteOverride) -> Self {
        self.route_overrides.insert(fake_route, route_override);
        self
    }

    pub const fn serving_signatures(mut self, signature_release: SignatureRelease) -> Self {
        self.signature_release = signature_release;
        self
    }

    pub const fn with_network(mut self, network_condition: NetworkCondition) -> Self {
        self.network_condition = network_condition;
        self
    }
}
