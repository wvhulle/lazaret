use crate::{
    application_window::protection_evidence_refresh::GatheredEvidence,
    host_services::threat_intel::service_access::ServiceAccess,
    signatures::signature_sources_page::source_check_results::{
        AuditCheck, AuditStatus, overall_check_status,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceChecks {
    ChecksRunning,
    ChecksFinished(Vec<AuditCheck>),
}

impl SourceChecks {
    pub fn overall_status(&self) -> AuditStatus {
        match self {
            Self::ChecksRunning => AuditStatus::CheckInProgress,
            Self::ChecksFinished(source_checks) => overall_check_status(source_checks),
        }
    }

    pub const fn checks_running(&self) -> bool {
        match self {
            Self::ChecksRunning => true,
            Self::ChecksFinished(_) => false,
        }
    }
}

fn gathered_abuse_ch_access(gathered_evidence: &GatheredEvidence) -> Option<&ServiceAccess> {
    match gathered_evidence {
        GatheredEvidence::NotGatheredYet => None,
        GatheredEvidence::Gathered {
            protection_evidence,
            ..
        } => Some(
            &protection_evidence
                .online_service_access
                .malware_bazaar_access,
        ),
    }
}

pub fn abuse_ch_key_subtitle(
    message_catalog: &MessageCatalog,
    gathered_evidence: &GatheredEvidence,
) -> String {
    match gathered_abuse_ch_access(gathered_evidence) {
        None => translated_messages::signatures::checking_api_key(message_catalog),
        Some(ServiceAccess::KeyReady(api_key)) => api_key.masked_text(),
        Some(ServiceAccess::KeyMissing) => {
            translated_messages::signatures::abuse_ch_no_key(message_catalog)
        }
        Some(ServiceAccess::KeyBroken(_key_problem)) => {
            translated_messages::signatures::abuse_ch_key_unreadable(message_catalog)
        }
    }
}

pub fn abuse_ch_key_needs_attention(gathered_evidence: &GatheredEvidence) -> bool {
    gathered_abuse_ch_access(gathered_evidence)
        .is_some_and(|abuse_ch_access| !abuse_ch_access.is_usable())
}

#[cfg(test)]
mod signature_sources_presentation_tests {
    use super::*;
    use crate::host_services::{
        protection_evidence::protection_evidence_fixtures::{REFERENCE_MOMENT, healthy_evidence},
        threat_intel::service_access::{ApiKey, KeyProblem},
    };

    fn gathered_with_abuse_ch_access(abuse_ch_access: ServiceAccess) -> GatheredEvidence {
        GatheredEvidence::Gathered {
            protection_evidence: Box::new({
                let mut protection_evidence = healthy_evidence();
                protection_evidence
                    .online_service_access
                    .malware_bazaar_access = abuse_ch_access;
                protection_evidence
            }),
            gathered_at: REFERENCE_MOMENT,
        }
    }

    #[test]
    fn the_abuse_ch_key_row_shows_a_short_status_and_flags_unusable_keys() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let ready = gathered_with_abuse_ch_access(ServiceAccess::KeyReady(ApiKey::new(format!(
            "{}cafe",
            "a".repeat(16)
        ))));
        assert_eq!(abuse_ch_key_subtitle(message_catalog, &ready), "••••cafe");
        assert!(!abuse_ch_key_needs_attention(&ready));
        let missing = gathered_with_abuse_ch_access(ServiceAccess::KeyMissing);
        assert_eq!(abuse_ch_key_subtitle(message_catalog, &missing), "No key");
        assert!(abuse_ch_key_needs_attention(&missing));
        let broken = gathered_with_abuse_ch_access(ServiceAccess::KeyBroken(
            KeyProblem::KeyFileNotPermitted,
        ));
        assert_eq!(
            abuse_ch_key_subtitle(message_catalog, &broken),
            "Key unreadable"
        );
        assert!(abuse_ch_key_needs_attention(&broken));
        assert!(!abuse_ch_key_needs_attention(
            &GatheredEvidence::NotGatheredYet
        ));
    }

    #[test]
    fn running_checks_show_progress_until_their_results_arrive() {
        assert_eq!(
            SourceChecks::ChecksRunning.overall_status(),
            AuditStatus::CheckInProgress
        );
        assert!(SourceChecks::ChecksRunning.checks_running());
        let finished = SourceChecks::ChecksFinished(vec![AuditCheck::new(
            "Sanesecurity".to_owned(),
            AuditStatus::CheckPassed,
            "In use".to_owned(),
        )]);
        assert_eq!(finished.overall_status(), AuditStatus::CheckPassed);
        assert!(!finished.checks_running());
    }
}
