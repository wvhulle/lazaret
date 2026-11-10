use std::{fs::File, io, os::unix::fs::OpenOptionsExt, path::Path};

use bytesize::ByteSize;
use sha2::{Digest, Sha256};
use zbus::{Connection, proxy};

use crate::{
    host_services::{
        activity_history::activity_history_database::{ActivityHistory, ActivityHistoryFailure},
        clamav_scanner::scanner_output_parser::sanitize_scanner_line,
        desktop::{
            application_directories::application_data_directory, flatpak_sandbox::SandboxKind,
        },
        lazaret_database::locked_database_connection::lazaret_database_path,
    },
    translated_messages,
};

const ENGINE_HEURISTIC_PREFIXES: [&str; 2] = ["Heuristics.", "Heuristic."];
const UNOFFICIAL_SUFFIX: &str = ".UNOFFICIAL";
const APPLIED_ANSWER: &str = "applied";
const RELOAD_PENDING_ANSWER: &str = "applied-reload-pending";
const SERVICE_UNKNOWN_ERROR: &str = "org.freedesktop.DBus.Error.ServiceUnknown";
const ACCESS_DENIED_ERROR: &str = "org.freedesktop.DBus.Error.AccessDenied";

#[proxy(
    interface = "page.codeberg.wvhulle.Lazaret.ClamavAdmin",
    default_service = "page.codeberg.wvhulle.Lazaret.ClamavAdmin",
    default_path = "/page/codeberg/wvhulle/Lazaret/ClamavAdmin"
)]
trait DetectionTrustAdmin {
    fn trust_file(&self, sha256_hex: &str, file_size: u64) -> zbus::Result<String>;

    fn forget_trusted_file(&self, sha256_hex: &str) -> zbus::Result<String>;

    fn stop_ignoring_signature(&self, signature_name: &str) -> zbus::Result<String>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileFingerprint {
    pub sha256_hex: String,
    pub file_size: ByteSize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrustScope {
    OnlyThisUser,
    EveryUser,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TrustChoice {
    TrustFile {
        file_fingerprint: FileFingerprint,
        shown_path: String,
        trust_scope: TrustScope,
    },
    IgnoreSignatureForThisUser {
        signature_name: String,
    },
    ForgetTrustedFile {
        sha256_hex: String,
        trust_scope: TrustScope,
    },
    StopIgnoringSignature {
        signature_name: String,
        trust_scope: TrustScope,
    },
}

impl TrustChoice {
    pub fn for_every_user(self) -> Self {
        match self {
            Self::TrustFile {
                file_fingerprint,
                shown_path,
                ..
            } => Self::TrustFile {
                file_fingerprint,
                shown_path,
                trust_scope: TrustScope::EveryUser,
            },
            other_choice => other_choice,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TrustOutcome {
    RecordedForThisUser,
    TrustApplied,
    AppliedReloadPending,
    AuthorizationRefused,
    AdminServiceMissing,
    TrustFailed(TrustChangeFailure),
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum TrustChangeFailure {
    #[error("the ClamAV admin service answered with an unknown outcome: {admin_service_answer}")]
    UnknownAdminServiceAnswer { admin_service_answer: String },
    #[error("the ClamAV admin service reported: {admin_service_message}")]
    ReportedByAdminService { admin_service_message: String },
    #[error(transparent)]
    AdminServiceCallFailed(#[from] zbus::Error),
    #[error("the trust list in your history could not be changed: {store_failure_text}")]
    UserTrustNotRecorded { store_failure_text: String },
}

pub fn fingerprint_file(file_path: &Path) -> io::Result<FileFingerprint> {
    let mut opened_file = File::options()
        .read(true)
        .custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32)
        .open(file_path)?;
    let mut content_hash = Sha256::new();
    let file_size = ByteSize::b(io::copy(&mut opened_file, &mut content_hash)?);
    Ok(FileFingerprint {
        sha256_hex: hex::encode(content_hash.finalize()),
        file_size,
    })
}

pub fn signature_of_detection(detected_signature_name: &str) -> String {
    detected_signature_name
        .trim()
        .strip_suffix(UNOFFICIAL_SUFFIX)
        .unwrap_or(detected_signature_name.trim())
        .to_owned()
}

pub fn signature_can_be_ignored(detected_signature_name: &str) -> bool {
    let signature_name = signature_of_detection(detected_signature_name);
    !signature_name.is_empty()
        && !ENGINE_HEURISTIC_PREFIXES
            .iter()
            .any(|heuristic_prefix| signature_name.starts_with(heuristic_prefix))
}

fn outcome_of_answer(admin_service_answer: zbus::Result<String>) -> TrustOutcome {
    match admin_service_answer {
        Ok(answer_text) if answer_text == APPLIED_ANSWER => TrustOutcome::TrustApplied,
        Ok(answer_text) if answer_text == RELOAD_PENDING_ANSWER => {
            TrustOutcome::AppliedReloadPending
        }
        Ok(other_answer) => {
            TrustOutcome::TrustFailed(TrustChangeFailure::UnknownAdminServiceAnswer {
                admin_service_answer: other_answer,
            })
        }
        Err(zbus::Error::MethodError(error_name, _error_text, _reply))
            if error_name.as_str() == SERVICE_UNKNOWN_ERROR =>
        {
            TrustOutcome::AdminServiceMissing
        }
        Err(zbus::Error::MethodError(error_name, _error_text, _reply))
            if error_name.as_str() == ACCESS_DENIED_ERROR =>
        {
            TrustOutcome::AuthorizationRefused
        }
        Err(zbus::Error::MethodError(_error_name, Some(admin_service_message), _reply)) => {
            TrustOutcome::TrustFailed(TrustChangeFailure::ReportedByAdminService {
                admin_service_message,
            })
        }
        Err(call_failure) => TrustOutcome::TrustFailed(call_failure.into()),
    }
}

async fn record_trust_for_this_user(
    activity_history: &ActivityHistory,
    trust_choice: &TrustChoice,
) -> Result<(), ActivityHistoryFailure> {
    match trust_choice {
        TrustChoice::TrustFile {
            file_fingerprint, ..
        } => {
            activity_history
                .trust_content_for_user(
                    &file_fingerprint.sha256_hex,
                    file_fingerprint.file_size.as_u64(),
                )
                .await
        }
        TrustChoice::IgnoreSignatureForThisUser { signature_name } => {
            activity_history
                .ignore_signature_for_user(signature_name)
                .await
        }
        TrustChoice::ForgetTrustedFile { sha256_hex, .. } => {
            activity_history.forget_content_for_user(sha256_hex).await
        }
        TrustChoice::StopIgnoringSignature { signature_name, .. } => {
            activity_history
                .stop_ignoring_signature_for_user(signature_name)
                .await
        }
    }
}

enum AdminTrustCall<'choice> {
    TrustFile {
        sha256_hex: &'choice str,
        file_size: u64,
    },
    ForgetTrustedFile {
        sha256_hex: &'choice str,
    },
    StopIgnoringSignature {
        signature_name: &'choice str,
    },
}

enum TrustRoute<'choice> {
    ThisUserHistory,
    AdminService(AdminTrustCall<'choice>),
}

fn trust_route(trust_choice: &TrustChoice) -> TrustRoute<'_> {
    match trust_choice {
        TrustChoice::TrustFile {
            file_fingerprint,
            trust_scope: TrustScope::EveryUser,
            ..
        } => TrustRoute::AdminService(AdminTrustCall::TrustFile {
            sha256_hex: &file_fingerprint.sha256_hex,
            file_size: file_fingerprint.file_size.as_u64(),
        }),
        TrustChoice::ForgetTrustedFile {
            sha256_hex,
            trust_scope: TrustScope::EveryUser,
        } => TrustRoute::AdminService(AdminTrustCall::ForgetTrustedFile { sha256_hex }),
        TrustChoice::StopIgnoringSignature {
            signature_name,
            trust_scope: TrustScope::EveryUser,
        } => TrustRoute::AdminService(AdminTrustCall::StopIgnoringSignature { signature_name }),
        TrustChoice::TrustFile {
            trust_scope: TrustScope::OnlyThisUser,
            ..
        }
        | TrustChoice::ForgetTrustedFile {
            trust_scope: TrustScope::OnlyThisUser,
            ..
        }
        | TrustChoice::StopIgnoringSignature {
            trust_scope: TrustScope::OnlyThisUser,
            ..
        }
        | TrustChoice::IgnoreSignatureForThisUser { .. } => TrustRoute::ThisUserHistory,
    }
}

async fn call_admin_service(admin_trust_call: AdminTrustCall<'_>) -> TrustOutcome {
    let admin_service_answer = async {
        let trust_admin_service =
            DetectionTrustAdminProxy::new(&Connection::system().await?).await?;
        match admin_trust_call {
            AdminTrustCall::TrustFile {
                sha256_hex,
                file_size,
            } => trust_admin_service.trust_file(sha256_hex, file_size).await,
            AdminTrustCall::ForgetTrustedFile { sha256_hex } => {
                trust_admin_service.forget_trusted_file(sha256_hex).await
            }
            AdminTrustCall::StopIgnoringSignature { signature_name } => {
                trust_admin_service
                    .stop_ignoring_signature(signature_name)
                    .await
            }
        }
    };
    outcome_of_answer(admin_service_answer.await)
}

pub async fn request_trust_change(trust_choice: TrustChoice) -> TrustOutcome {
    match trust_route(&trust_choice) {
        TrustRoute::AdminService(admin_trust_call) => call_admin_service(admin_trust_call).await,
        TrustRoute::ThisUserHistory => {
            let activity_history = ActivityHistory::at_database_path(lazaret_database_path(
                &application_data_directory(),
            ));
            match record_trust_for_this_user(&activity_history, &trust_choice).await {
                Ok(()) => TrustOutcome::RecordedForThisUser,
                Err(store_failure) => {
                    TrustOutcome::TrustFailed(TrustChangeFailure::UserTrustNotRecorded {
                        store_failure_text: store_failure.to_string(),
                    })
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SystemWideTrust {
    OfferedOutsideSandbox,
    UnavailableInFlatpak,
}

pub fn system_wide_trust(sandbox_kind: SandboxKind) -> SystemWideTrust {
    match sandbox_kind {
        SandboxKind::FlatpakSandbox => SystemWideTrust::UnavailableInFlatpak,
        SandboxKind::HostSystem => SystemWideTrust::OfferedOutsideSandbox,
    }
}

pub fn trust_outcome_text(trust_outcome: &TrustOutcome) -> String {
    match trust_outcome {
        TrustOutcome::RecordedForThisUser => translated_messages::host::trust_recorded_for_you(),
        TrustOutcome::TrustApplied => translated_messages::host::trust_applied(),
        TrustOutcome::AppliedReloadPending => translated_messages::host::applied_reload_pending(),
        TrustOutcome::AuthorizationRefused => {
            translated_messages::host::authorization_refused_outcome()
        }
        TrustOutcome::AdminServiceMissing => translated_messages::host::admin_service_missing(),
        TrustOutcome::TrustFailed(change_failure) => {
            translated_messages::host::trust_failed(trust_change_failure_text(change_failure))
        }
    }
}

fn trust_change_failure_text(change_failure: &TrustChangeFailure) -> String {
    match change_failure {
        TrustChangeFailure::UnknownAdminServiceAnswer {
            admin_service_answer,
        } => sanitize_scanner_line(admin_service_answer),
        TrustChangeFailure::ReportedByAdminService {
            admin_service_message,
        } => sanitize_scanner_line(admin_service_message),
        TrustChangeFailure::AdminServiceCallFailed(call_failure) => {
            sanitize_scanner_line(&call_failure.to_string())
        }
        TrustChangeFailure::UserTrustNotRecorded { store_failure_text } => {
            sanitize_scanner_line(store_failure_text)
        }
    }
}

#[cfg(test)]
mod trust_requests_tests {
    use super::*;

    #[test]
    fn fingerprint_matches_the_known_hash_of_the_content() {
        let fingerprinted_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(fingerprinted_file.path(), b"abc").unwrap();
        assert_eq!(
            fingerprint_file(fingerprinted_file.path()).unwrap(),
            FileFingerprint {
                sha256_hex: "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
                    .to_owned(),
                file_size: ByteSize::b(3),
            }
        );
    }

    #[test]
    fn fingerprinting_never_follows_a_symbolic_link() {
        let link_directory = tempfile::tempdir().unwrap();
        let target_file = link_directory.path().join("target");
        std::fs::write(&target_file, b"abc").unwrap();
        let link_path = link_directory.path().join("link");
        std::os::unix::fs::symlink(&target_file, &link_path).unwrap();
        assert!(fingerprint_file(&link_path).is_err());
    }

    #[test]
    fn heuristics_can_only_be_trusted_by_hash() {
        assert!(!signature_can_be_ignored("Heuristics.Encrypted.Zip"));
        assert!(signature_can_be_ignored("Win.Test.EICAR_HDB-1"));
        assert_eq!(
            signature_of_detection("Sanesecurity.Foo.UNOFFICIAL"),
            "Sanesecurity.Foo"
        );
    }
}
