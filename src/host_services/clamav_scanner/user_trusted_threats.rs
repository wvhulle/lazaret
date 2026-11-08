use std::{io, path::Path, sync::Arc};

use futures::{Stream, StreamExt, future::Either, stream};
use sha2::{Digest, Sha256};
use tokio::io::AsyncReadExt;

use crate::host_services::{
    activity_history::{
        activity_history_database::ActivityHistoryFailure, user_trust_store::UserTrustList,
    },
    clamav_scanner::{
        scan_progress_events::ScanProgressEvent,
        scanner_output_parser::{DetectedThreat, ThreatIdentity},
    },
};

const HASH_CHUNK_SIZE: usize = 64 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreatTrust {
    NotTrustedByUser,
    SignatureIgnoredByUser,
    ContentTrustedByUser,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserTrustedThreat {
    pub detected_threat: DetectedThreat,
    pub threat_trust: ThreatTrust,
}

async fn content_sha256_without_following_links(file_path: &Path) -> io::Result<String> {
    let mut opened_file = tokio::fs::OpenOptions::new()
        .read(true)
        .custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32)
        .open(file_path)
        .await?;
    let mut content_hash = Sha256::new();
    let mut read_buffer = vec![0_u8; HASH_CHUNK_SIZE];
    loop {
        match opened_file.read(&mut read_buffer).await? {
            0 => return Ok(hex::encode(content_hash.finalize())),
            read_length => content_hash.update(&read_buffer[..read_length]),
        }
    }
}

async fn user_trust_of_content(user_trust: &UserTrustList, file_path: &Path) -> ThreatTrust {
    let file_size = match tokio::fs::symlink_metadata(file_path).await {
        Ok(file_metadata) => file_metadata.len(),
        Err(_unreadable_metadata) => return ThreatTrust::NotTrustedByUser,
    };
    if !user_trust.trusts_content_of_size(file_size) {
        return ThreatTrust::NotTrustedByUser;
    }
    match content_sha256_without_following_links(file_path).await {
        Ok(content_sha256) if user_trust.trusts_content(&content_sha256) => {
            ThreatTrust::ContentTrustedByUser
        }
        Ok(_) | Err(_) => ThreatTrust::NotTrustedByUser,
    }
}

async fn user_trust_of_signature_match(
    user_trust: &UserTrustList,
    file_path: &Path,
    signature_name: &str,
) -> ThreatTrust {
    if user_trust.ignores_signature(signature_name) {
        ThreatTrust::SignatureIgnoredByUser
    } else {
        user_trust_of_content(user_trust, file_path).await
    }
}

pub async fn user_trust_of_threat(
    user_trust: &UserTrustList,
    detected_threat: &DetectedThreat,
) -> ThreatTrust {
    match &detected_threat.threat_identity {
        ThreatIdentity::Signature { signature_name } => {
            user_trust_of_signature_match(user_trust, &detected_threat.file_path, signature_name)
                .await
        }
        ThreatIdentity::BlockedByOnAccess => {
            user_trust_of_content(user_trust, &detected_threat.file_path).await
        }
    }
}

async fn event_after_user_trust(
    user_trust: Arc<UserTrustList>,
    progress_event: ScanProgressEvent,
) -> ScanProgressEvent {
    match progress_event {
        ScanProgressEvent::ThreatDetected(detected_threat) => {
            match user_trust_of_threat(&user_trust, &detected_threat).await {
                ThreatTrust::NotTrustedByUser => ScanProgressEvent::ThreatDetected(detected_threat),
                threat_trust @ (ThreatTrust::SignatureIgnoredByUser
                | ThreatTrust::ContentTrustedByUser) => {
                    ScanProgressEvent::UserTrustedThreatSkipped(UserTrustedThreat {
                        detected_threat,
                        threat_trust,
                    })
                }
            }
        }
        other_event => other_event,
    }
}

pub fn skipping_user_trusted_threats(
    scan_events: impl Stream<Item = ScanProgressEvent>,
    user_trust_loading: Result<UserTrustList, ActivityHistoryFailure>,
) -> impl Stream<Item = ScanProgressEvent> {
    match user_trust_loading {
        Ok(user_trust) if user_trust.is_empty() => Either::Left(Either::Left(scan_events)),
        Ok(user_trust) => {
            let shared_trust = Arc::new(user_trust);
            Either::Right(scan_events.then(move |progress_event| {
                event_after_user_trust(Arc::clone(&shared_trust), progress_event)
            }))
        }
        Err(loading_failure) => Either::Left(Either::Right(
            stream::once(futures::future::ready(
                ScanProgressEvent::UserTrustUnreadable {
                    failure_text: loading_failure.to_string(),
                },
            ))
            .chain(scan_events),
        )),
    }
}

#[cfg(test)]
mod user_trusted_threats_tests {
    use std::collections::{BTreeMap, BTreeSet};

    use super::*;

    const ABC_SHA256: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

    fn trust_of_abc() -> UserTrustList {
        UserTrustList {
            trusted_content_sizes: BTreeMap::from([(ABC_SHA256.to_owned(), 3)]),
            ignored_signatures: BTreeSet::from(["Sanesecurity.Foo".to_owned()]),
        }
    }

    fn signature_threat(file_path: &Path, signature_name: &str) -> DetectedThreat {
        DetectedThreat {
            file_path: file_path.to_path_buf(),
            threat_identity: ThreatIdentity::Signature {
                signature_name: signature_name.to_owned(),
            },
        }
    }

    #[tokio::test]
    async fn an_ignored_signature_is_trusted_without_reading_the_file() {
        assert_eq!(
            user_trust_of_threat(
                &trust_of_abc(),
                &signature_threat(Path::new("/nonexistent"), "Sanesecurity.Foo.UNOFFICIAL"),
            )
            .await,
            ThreatTrust::SignatureIgnoredByUser
        );
    }

    #[tokio::test]
    async fn only_the_trusted_content_is_trusted() {
        let scanned_directory = tempfile::tempdir().unwrap();
        let trusted_file = scanned_directory.path().join("trusted");
        let changed_file = scanned_directory.path().join("changed");
        std::fs::write(&trusted_file, b"abc").unwrap();
        std::fs::write(&changed_file, b"xyz").unwrap();
        assert_eq!(
            user_trust_of_threat(
                &trust_of_abc(),
                &signature_threat(&trusted_file, "Win.Test.EICAR_HDB-1")
            )
            .await,
            ThreatTrust::ContentTrustedByUser
        );
        assert_eq!(
            user_trust_of_threat(
                &trust_of_abc(),
                &signature_threat(&changed_file, "Win.Test.EICAR_HDB-1")
            )
            .await,
            ThreatTrust::NotTrustedByUser
        );
    }

    #[tokio::test]
    async fn a_symbolic_link_or_unreadable_file_is_never_trusted() {
        let scanned_directory = tempfile::tempdir().unwrap();
        let trusted_file = scanned_directory.path().join("trusted");
        std::fs::write(&trusted_file, b"abc").unwrap();
        let link_path = scanned_directory.path().join("link");
        std::os::unix::fs::symlink(&trusted_file, &link_path).unwrap();
        for untrusted_path in [link_path, scanned_directory.path().join("missing")] {
            assert_eq!(
                user_trust_of_threat(
                    &trust_of_abc(),
                    &signature_threat(&untrusted_path, "Win.Test.EICAR_HDB-1")
                )
                .await,
                ThreatTrust::NotTrustedByUser
            );
        }
    }

    #[tokio::test]
    async fn an_unreadable_trust_list_is_reported_before_every_detection() {
        let filtered_events = skipping_user_trusted_threats(
            stream::iter([ScanProgressEvent::ThreatDetected(signature_threat(
                Path::new("/scan/file"),
                "Win.Test.EICAR_HDB-1",
            ))]),
            Err(ActivityHistoryFailure::DatabasePathNotUtf8),
        )
        .collect::<Vec<_>>()
        .await;
        assert!(matches!(
            &filtered_events[..],
            [
                ScanProgressEvent::UserTrustUnreadable { .. },
                ScanProgressEvent::ThreatDetected(_),
            ]
        ));
    }

    #[tokio::test]
    async fn trusted_threats_become_skip_events_and_others_pass_through() {
        let scanned_directory = tempfile::tempdir().unwrap();
        let trusted_file = scanned_directory.path().join("trusted");
        let malware_file = scanned_directory.path().join("malware");
        std::fs::write(&trusted_file, b"abc").unwrap();
        std::fs::write(&malware_file, b"evil").unwrap();
        let filtered_events = skipping_user_trusted_threats(
            stream::iter([
                ScanProgressEvent::ThreatDetected(signature_threat(
                    &trusted_file,
                    "Win.Test.EICAR_HDB-1",
                )),
                ScanProgressEvent::ThreatDetected(signature_threat(
                    &malware_file,
                    "Win.Test.EICAR_HDB-1",
                )),
            ]),
            Ok(trust_of_abc()),
        )
        .collect::<Vec<_>>()
        .await;
        assert!(matches!(
            &filtered_events[..],
            [
                ScanProgressEvent::UserTrustedThreatSkipped(UserTrustedThreat {
                    threat_trust: ThreatTrust::ContentTrustedByUser,
                    ..
                }),
                ScanProgressEvent::ThreatDetected(DetectedThreat { file_path, .. }),
            ] if *file_path == malware_file
        ));
    }
}
