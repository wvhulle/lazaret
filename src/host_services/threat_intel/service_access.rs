use std::{fmt, io};

use crate::host_services::threat_intel::service_key_storage::{
    KeyStorageFailure, ResolvedApiKey, ThreatIntelService, masked_api_key, read_api_key,
};

#[derive(Clone, Eq, PartialEq)]
pub struct ApiKey(String);

impl ApiKey {
    pub const fn new(key_text: String) -> Self {
        Self(key_text)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn masked_text(&self) -> String {
        masked_api_key(&self.0)
    }
}

impl fmt::Debug for ApiKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.masked_text())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyProblem {
    KeyFileNotPermitted,
    KeyFileMissing,
    KeyFileUnreadable,
    KeyNotValid,
    KeyringUnavailable,
    StoredKeyUnreadable,
}

impl From<&KeyStorageFailure> for KeyProblem {
    fn from(key_failure: &KeyStorageFailure) -> Self {
        match key_failure {
            KeyStorageFailure::KeyFilePermissionDenied { .. } => Self::KeyFileNotPermitted,
            KeyStorageFailure::KeyFileUnreadable { read_failure, .. }
                if read_failure.kind() == io::ErrorKind::NotFound =>
            {
                Self::KeyFileMissing
            }
            KeyStorageFailure::KeyFileUnreadable { .. } => Self::KeyFileUnreadable,
            KeyStorageFailure::KeyFileContentInvalid { .. }
            | KeyStorageFailure::KeyFormatInvalid => Self::KeyNotValid,
            KeyStorageFailure::KeyringUnavailable(_keyring_failure) => Self::KeyringUnavailable,
            KeyStorageFailure::StoredKeyUnreadable => Self::StoredKeyUnreadable,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServiceAccess {
    KeyReady(ApiKey),
    KeyMissing,
    KeyBroken(KeyProblem),
}

impl ServiceAccess {
    pub fn from_key_reading(key_reading: &Result<ResolvedApiKey, KeyStorageFailure>) -> Self {
        match key_reading {
            Ok(ResolvedApiKey::FromKeyFile(key_text) | ResolvedApiKey::FromKeyring(key_text)) => {
                Self::KeyReady(ApiKey::new(key_text.clone()))
            }
            Ok(ResolvedApiKey::NotConfigured) => Self::KeyMissing,
            Err(key_failure) => Self::KeyBroken(KeyProblem::from(key_failure)),
        }
    }

    pub const fn ready_key(&self) -> Option<&ApiKey> {
        match self {
            Self::KeyReady(api_key) => Some(api_key),
            Self::KeyMissing | Self::KeyBroken(_) => None,
        }
    }

    pub const fn is_usable(&self) -> bool {
        self.ready_key().is_some()
    }
}

pub async fn read_service_access(threat_intel_service: ThreatIntelService) -> ServiceAccess {
    ServiceAccess::from_key_reading(&read_api_key(threat_intel_service).await)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OnlineServiceAccess {
    pub malware_bazaar_access: ServiceAccess,
    pub virustotal_access: ServiceAccess,
}

pub async fn read_online_service_access() -> OnlineServiceAccess {
    let (malware_bazaar_access, virustotal_access) = futures::join!(
        read_service_access(ThreatIntelService::AbuseChService),
        read_service_access(ThreatIntelService::VirusTotalService),
    );
    OnlineServiceAccess {
        malware_bazaar_access,
        virustotal_access,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KnownServiceAccess {
    AccessNotReadYet,
    AccessRead(OnlineServiceAccess),
}

impl KnownServiceAccess {
    pub const fn malware_bazaar_blocker(&self) -> Option<&ServiceAccess> {
        match self {
            Self::AccessRead(OnlineServiceAccess {
                malware_bazaar_access,
                ..
            }) if !malware_bazaar_access.is_usable() => Some(malware_bazaar_access),
            Self::AccessNotReadYet | Self::AccessRead(_) => None,
        }
    }

    pub const fn virustotal_blocker(&self) -> Option<&ServiceAccess> {
        match self {
            Self::AccessRead(OnlineServiceAccess {
                virustotal_access, ..
            }) if !virustotal_access.is_usable() => Some(virustotal_access),
            Self::AccessNotReadYet | Self::AccessRead(_) => None,
        }
    }
}

#[cfg(test)]
mod service_access_tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn key_readings_become_ready_missing_or_broken() {
        assert_eq!(
            ServiceAccess::from_key_reading(&Ok(ResolvedApiKey::FromKeyring(
                "0123456789abcdef".to_owned()
            ))),
            ServiceAccess::KeyReady(ApiKey::new("0123456789abcdef".to_owned()))
        );
        assert_eq!(
            ServiceAccess::from_key_reading(&Ok(ResolvedApiKey::NotConfigured)),
            ServiceAccess::KeyMissing
        );
        assert_eq!(
            ServiceAccess::from_key_reading(&Err(KeyStorageFailure::KeyFilePermissionDenied {
                key_file_path: PathBuf::from("/run/agenix/abuse-ch"),
            })),
            ServiceAccess::KeyBroken(KeyProblem::KeyFileNotPermitted)
        );
    }

    #[test]
    fn a_missing_key_file_is_told_apart_from_an_unreadable_one() {
        let missing_file = KeyStorageFailure::KeyFileUnreadable {
            key_file_path: PathBuf::from("/run/agenix/abuse-ch"),
            read_failure: io::Error::from(io::ErrorKind::NotFound),
        };
        assert_eq!(KeyProblem::from(&missing_file), KeyProblem::KeyFileMissing);
        let directory_instead = KeyStorageFailure::KeyFileUnreadable {
            key_file_path: PathBuf::from("/run/agenix"),
            read_failure: io::Error::from(io::ErrorKind::InvalidInput),
        };
        assert_eq!(
            KeyProblem::from(&directory_instead),
            KeyProblem::KeyFileUnreadable
        );
    }

    #[test]
    fn an_api_key_never_shows_in_debug_output() {
        let debug_text = format!("{:?}", ApiKey::new(format!("{}cafe", "0".repeat(60))));
        assert_eq!(debug_text, "••••cafe");
    }

    #[test]
    fn only_a_read_unusable_service_blocks_its_lookups() {
        assert!(
            KnownServiceAccess::AccessNotReadYet
                .malware_bazaar_blocker()
                .is_none()
        );
        let known_access = KnownServiceAccess::AccessRead(OnlineServiceAccess {
            malware_bazaar_access: ServiceAccess::KeyBroken(KeyProblem::KeyFileNotPermitted),
            virustotal_access: ServiceAccess::KeyReady(ApiKey::new("a".repeat(64))),
        });
        assert_eq!(
            known_access.malware_bazaar_blocker(),
            Some(&ServiceAccess::KeyBroken(KeyProblem::KeyFileNotPermitted))
        );
        assert!(known_access.virustotal_blocker().is_none());
    }
}
