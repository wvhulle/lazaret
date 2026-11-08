use std::{
    collections::{BTreeMap, BTreeSet},
    io::{self, Write as _},
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{Arc, Mutex, MutexGuard, PoisonError},
};

use axum::{
    Json, Router,
    body::Bytes,
    extract::{ConnectInfo, State},
    http::{HeaderMap, Method, StatusCode, Uri, header},
    response::{IntoResponse, Response},
    routing::get,
};
use axum_server::tls_rustls::RustlsConfig;
use serde_json::{Value, json};

use crate::vm_journeys::remote_nodes::{
    fake_internet_protocol::{
        BAZAAR_FIXTURE_FAMILY, FakeInternetScenario, FakeRoute, FormField, LoggedRequest,
        NetworkCondition,
    },
    submitted_form_fields::summarised_form_fields,
};

const OUTAGE_TABLE: &str = "inet fake_internet_outage";

#[derive(Debug, thiserror::Error)]
pub enum FakeInternetFailure {
    #[error("the fake internet could not serve: {0}")]
    ServingFailed(#[from] io::Error),
    #[error("nft refused the outage rules: {nft_output}")]
    OutageRulesRefused { nft_output: String },
}

#[derive(clap::Args, Debug)]
pub struct FakeInternetServerOptions {
    #[arg(long)]
    certificate: PathBuf,
    #[arg(long)]
    private_key: PathBuf,
    #[arg(long)]
    signature_directory: PathBuf,
    #[arg(long)]
    listen_address: IpAddr,
    #[arg(long, default_value_t = 443)]
    tls_port: u16,
    #[arg(long)]
    control_port: u16,
}

#[derive(Default)]
struct FakeInternetMemory {
    scenario: FakeInternetScenario,
    logged_requests: Vec<LoggedRequest>,
    bazaar_uploaded_hashes: BTreeSet<String>,
}

struct FakeInternet {
    signature_directory: PathBuf,
    memory: Mutex<FakeInternetMemory>,
}

struct RoutedAnswer {
    fake_route: FakeRoute,
    status: u16,
    content_type: String,
    body: Vec<u8>,
}

impl RoutedAnswer {
    fn json(fake_route: FakeRoute, status: u16, answer_json: &Value) -> Self {
        Self {
            fake_route,
            status,
            content_type: "application/json".to_owned(),
            body: answer_json.to_string().into_bytes(),
        }
    }

    fn file_or_not_found(fake_route: FakeRoute, file_path: &Path) -> Self {
        match std::fs::read(file_path) {
            Ok(file_bytes) => Self {
                fake_route,
                status: 200,
                content_type: "application/octet-stream".to_owned(),
                body: file_bytes,
            },
            Err(_missing_file) => Self {
                fake_route,
                status: 404,
                content_type: "text/plain".to_owned(),
                body: b"not found".to_vec(),
            },
        }
    }
}

fn last_path_segment(requested_path: &str) -> &str {
    requested_path.rsplit('/').next().unwrap_or_default()
}

fn text_field<'fields>(
    form_fields: &'fields BTreeMap<String, FormField>,
    field_name: &str,
) -> Option<&'fields str> {
    match form_fields.get(field_name)? {
        FormField::TextField(field_text) => Some(field_text),
        FormField::UploadedFileField(_uploaded_file) => None,
    }
}

impl FakeInternet {
    fn memory(&self) -> MutexGuard<'_, FakeInternetMemory> {
        self.memory.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn bazaar_answer(&self, form_fields: &BTreeMap<String, FormField>) -> RoutedAnswer {
        if let Some(FormField::UploadedFileField(uploaded_file)) = form_fields.get("file") {
            self.memory()
                .bazaar_uploaded_hashes
                .insert(uploaded_file.content_sha256.clone());
            return RoutedAnswer::json(
                FakeRoute::BazaarUpload,
                200,
                &json!({ "query_status": "inserted" }),
            );
        }
        match text_field(form_fields, "query") {
            Some("get_clamavinfo") => RoutedAnswer::json(
                FakeRoute::BazaarGetClamavinfo,
                200,
                &json!({
                    "query_status": "ok",
                    "data": [{
                        "sha256_hash": "0".repeat(64),
                        "signature": BAZAAR_FIXTURE_FAMILY,
                        "first_seen": "2026-01-02 03:04:05",
                        "tags": ["fixture", "loader"],
                    }],
                }),
            ),
            Some("get_info") => {
                let requested_hash = text_field(form_fields, "hash").unwrap_or_default();
                let bazaar_answer = if self
                    .memory()
                    .bazaar_uploaded_hashes
                    .contains(requested_hash)
                {
                    json!({
                        "query_status": "ok",
                        "data": [{
                            "sha256_hash": requested_hash,
                            "signature": BAZAAR_FIXTURE_FAMILY,
                            "tags": [],
                        }],
                    })
                } else {
                    json!({ "query_status": "hash_not_found" })
                };
                RoutedAnswer::json(FakeRoute::BazaarGetInfo, 200, &bazaar_answer)
            }
            _unknown_query => RoutedAnswer::json(
                FakeRoute::BazaarUnknownQuery,
                200,
                &json!({ "query_status": "unknown_query" }),
            ),
        }
    }

    fn routed_answer(
        &self,
        requested_host: &str,
        requested_path: &str,
        form_fields: &BTreeMap<String, FormField>,
    ) -> RoutedAnswer {
        match requested_host {
            "mb-api.abuse.ch" => self.bazaar_answer(form_fields),
            "hashlookup.circl.lu" => RoutedAnswer::json(
                FakeRoute::CirclLookup,
                404,
                &json!({ "message": "Non existing SHA-256" }),
            ),
            "www.virustotal.com" if requested_path.starts_with("/api/v3/files/") => {
                RoutedAnswer::json(
                    FakeRoute::VirustotalFileReport,
                    404,
                    &json!({ "error": { "code": "NotFoundError", "message": "File not found" } }),
                )
            }
            "www.virustotal.com" => RoutedAnswer::json(
                FakeRoute::VirustotalUnknownPath,
                404,
                &json!({ "error": { "code": "NotFoundError" } }),
            ),
            "signatures.fixture.test" => {
                let release_directory = self.memory().scenario.signature_release.directory_name();
                RoutedAnswer::file_or_not_found(
                    FakeRoute::SignatureDownload,
                    &self
                        .signature_directory
                        .join(release_directory)
                        .join(last_path_segment(requested_path)),
                )
            }
            "urlhaus-api.abuse.ch" => RoutedAnswer::file_or_not_found(
                FakeRoute::UrlhausDownload,
                &self
                    .signature_directory
                    .join("urlhaus")
                    .join(last_path_segment(requested_path)),
            ),
            _unknown_host => RoutedAnswer {
                fake_route: FakeRoute::UnknownHost,
                status: 404,
                content_type: "text/plain".to_owned(),
                body: b"unknown host".to_vec(),
            },
        }
    }

    fn overridden_answer(
        &self,
        routed_answer: RoutedAnswer,
        requested_path: &str,
        form_fields: &BTreeMap<String, FormField>,
    ) -> RoutedAnswer {
        match self
            .memory()
            .scenario
            .route_overrides
            .get(&routed_answer.fake_route)
        {
            Some(route_override) if route_override.applies_to(requested_path, form_fields) => {
                RoutedAnswer {
                    fake_route: routed_answer.fake_route,
                    status: route_override.status,
                    content_type: route_override.content_type.clone(),
                    body: route_override.body_bytes(),
                }
            }
            _no_applicable_override => routed_answer,
        }
    }
}

fn apply_network_condition(network_condition: NetworkCondition) -> Result<(), FakeInternetFailure> {
    let outage_verdict = match network_condition {
        NetworkCondition::Reachable => None,
        NetworkCondition::RefusedConnections => Some("reject with tcp reset"),
        NetworkCondition::SilentDrop => Some("drop"),
    };
    let outage_rules = outage_verdict.map_or_else(String::new, |verdict| {
        format!(
            "table {OUTAGE_TABLE} {{\n  chain input {{\n    type filter hook input priority -10; \
             policy accept;\n    tcp dport 443 {verdict}\n  }}\n}}\n"
        )
    });
    let ruleset = format!("table {OUTAGE_TABLE}\ndelete table {OUTAGE_TABLE}\n{outage_rules}");
    let mut nft_process = Command::new("nft")
        .args(["-f", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    if let Some(mut nft_input) = nft_process.stdin.take() {
        nft_input.write_all(ruleset.as_bytes())?;
    }
    let nft_outcome = nft_process.wait_with_output()?;
    if nft_outcome.status.success() {
        Ok(())
    } else {
        Err(FakeInternetFailure::OutageRulesRefused {
            nft_output: String::from_utf8_lossy(&nft_outcome.stderr).into_owned(),
        })
    }
}

async fn answer_internet_request(
    State(fake_internet): State<Arc<FakeInternet>>,
    ConnectInfo(client_address): ConnectInfo<SocketAddr>,
    request_method: Method,
    request_uri: Uri,
    request_headers: HeaderMap,
    request_body: Bytes,
) -> Response {
    let requested_host = request_uri
        .host()
        .or_else(|| {
            request_headers
                .get(header::HOST)
                .and_then(|host_header| host_header.to_str().ok())
                .and_then(|host_text| host_text.split(':').next())
        })
        .unwrap_or_default()
        .to_owned();
    let content_type = request_headers
        .get(header::CONTENT_TYPE)
        .and_then(|content_type| content_type.to_str().ok())
        .unwrap_or_default()
        .to_owned();
    let body_byte_size = request_body.len() as u64;
    let form_fields = summarised_form_fields(&content_type, request_body).await;
    let requested_path = request_uri.path();
    let routed_answer = fake_internet.overridden_answer(
        fake_internet.routed_answer(&requested_host, requested_path, &form_fields),
        requested_path,
        &form_fields,
    );
    println!(
        "{client_address} {requested_host} {request_method} {requested_path} -> {}",
        routed_answer.status
    );
    fake_internet.memory().logged_requests.push(LoggedRequest {
        client_address: client_address.ip().to_string(),
        host: requested_host,
        method: request_method.to_string(),
        path: requested_path.to_owned(),
        query: request_uri.query().unwrap_or_default().to_owned(),
        headers: request_headers
            .iter()
            .map(|(header_name, header_value)| {
                (
                    header_name.as_str().to_owned(),
                    String::from_utf8_lossy(header_value.as_bytes()).into_owned(),
                )
            })
            .collect(),
        form_fields,
        body_byte_size,
        route_name: routed_answer.fake_route,
        answer_status: routed_answer.status,
    });
    (
        StatusCode::from_u16(routed_answer.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        [(header::CONTENT_TYPE, routed_answer.content_type)],
        routed_answer.body,
    )
        .into_response()
}

async fn answer_logged_requests(
    State(fake_internet): State<Arc<FakeInternet>>,
) -> Json<Vec<LoggedRequest>> {
    Json(fake_internet.memory().logged_requests.clone())
}

async fn clear_logged_requests(State(fake_internet): State<Arc<FakeInternet>>) -> StatusCode {
    fake_internet.memory().logged_requests.clear();
    StatusCode::NO_CONTENT
}

async fn replace_scenario(
    State(fake_internet): State<Arc<FakeInternet>>,
    Json(new_scenario): Json<FakeInternetScenario>,
) -> Response {
    match apply_network_condition(new_scenario.network_condition) {
        Ok(()) => {
            fake_internet.memory().scenario = new_scenario;
            StatusCode::NO_CONTENT.into_response()
        }
        Err(outage_failure) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            outage_failure.to_string(),
        )
            .into_response(),
    }
}

pub fn serve_fake_internet(
    server_options: FakeInternetServerOptions,
) -> Result<(), FakeInternetFailure> {
    apply_network_condition(NetworkCondition::Reachable)?;
    let fake_internet = Arc::new(FakeInternet {
        signature_directory: server_options.signature_directory,
        memory: Mutex::new(FakeInternetMemory::default()),
    });
    let control_router = Router::new()
        .route(
            "/requests",
            get(answer_logged_requests).delete(clear_logged_requests),
        )
        .route("/scenario", axum::routing::put(replace_scenario))
        .with_state(Arc::clone(&fake_internet));
    let internet_router = Router::new()
        .fallback(answer_internet_request)
        .with_state(fake_internet);
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let tls_config =
                RustlsConfig::from_pem_file(server_options.certificate, server_options.private_key)
                    .await?;
            let control_listener = tokio::net::TcpListener::bind(SocketAddr::new(
                server_options.listen_address,
                server_options.control_port,
            ))
            .await?;
            let control_serving = axum::serve(control_listener, control_router).into_future();
            let internet_serving = axum_server::bind_rustls(
                SocketAddr::new(server_options.listen_address, server_options.tls_port),
                tls_config,
            )
            .serve(internet_router.into_make_service_with_connect_info::<SocketAddr>());
            futures_util::future::try_join(control_serving, internet_serving).await?;
            Ok(())
        })
}
