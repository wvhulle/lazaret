use std::sync::Arc;

use crate::{
    host_services::{
        clamav_scanner::scanner_output_parser::sanitize_scanner_line,
        threat_intel::{
            circl_hashlookup_client::{
                CirclLookupFailure, KnownFileLookup, KnownName, ListingSource, look_up_known_file,
            },
            malware_bazaar_client::{
                KnownSampleLookup, MalwareBazaarFailure, MalwareFamily, SampleFirstSeen,
                look_up_file_hash,
            },
            service_key_storage::{
                KeyStorageFailure, ResolvedApiKey, ThreatIntelService, describe_key_failure,
                read_api_key,
            },
            virustotal_api_client::{FileReportLookup, VirusTotalClient, VirusTotalFailure},
        },
    },
    translated_messages,
};

#[derive(Debug, thiserror::Error)]
pub enum ReputationLookupFailure {
    #[error(transparent)]
    KeyUnavailable(#[from] KeyStorageFailure),
    #[error(transparent)]
    CirclLookupFailed(#[from] CirclLookupFailure),
    #[error(transparent)]
    BazaarLookupFailed(#[from] MalwareBazaarFailure),
    #[error(transparent)]
    VirusTotalLookupFailed(#[from] VirusTotalFailure),
}

#[derive(Clone, Debug)]
pub enum SourceAnswer {
    NeedsKey,
    Answered {
        verdict_text: String,
    },
    AnsweredWithReport {
        verdict_text: String,
        report_url: String,
    },
    Failed(Arc<ReputationLookupFailure>),
}

impl From<ReputationLookupFailure> for SourceAnswer {
    fn from(lookup_failure: ReputationLookupFailure) -> Self {
        Self::Failed(Arc::new(lookup_failure))
    }
}

impl SourceAnswer {
    pub fn displayed_text(&self) -> String {
        match self {
            Self::NeedsKey => translated_messages::host::needs_key(),
            Self::Answered { verdict_text } | Self::AnsweredWithReport { verdict_text, .. } => {
                verdict_text.clone()
            }
            Self::Failed(lookup_failure) => describe_lookup_failure(lookup_failure),
        }
    }

    pub const fn needs_key(&self) -> bool {
        matches!(self, Self::NeedsKey)
    }
}

#[derive(Clone, Debug)]
pub struct ReputationAnswers {
    pub circl_answer: SourceAnswer,
    pub bazaar_answer: SourceAnswer,
    pub virustotal_answer: SourceAnswer,
}

pub fn describe_known_file(known_file_lookup: &KnownFileLookup) -> SourceAnswer {
    let verdict_text = match known_file_lookup {
        KnownFileLookup::HashUnknown => translated_messages::host::hash_unknown(),
        KnownFileLookup::KnownMalicious {
            file_name: KnownName::NamedFile(file_name),
            flagged_by: ListingSource::NamedSource(source_name),
        } => translated_messages::host::listed_malicious_source_name(
            sanitize_scanner_line(source_name),
            sanitize_scanner_line(file_name),
        ),
        KnownFileLookup::KnownMalicious {
            file_name: KnownName::NamedFile(file_name),
            flagged_by: ListingSource::UnnamedSource,
        } => translated_messages::host::listed_malicious_unnamed_source_name(
            sanitize_scanner_line(file_name),
        ),
        KnownFileLookup::KnownMalicious {
            file_name: KnownName::UnnamedFile,
            flagged_by: ListingSource::NamedSource(source_name),
        } => translated_messages::host::listed_malicious_source(sanitize_scanner_line(source_name)),
        KnownFileLookup::KnownMalicious {
            file_name: KnownName::UnnamedFile,
            flagged_by: ListingSource::UnnamedSource,
        } => translated_messages::host::listed_malicious_unnamed_source(),
        KnownFileLookup::KnownFile {
            file_name: KnownName::UnnamedFile,
            source_name: ListingSource::NamedSource(source_name),
        } => {
            translated_messages::host::listed_known_file_source(sanitize_scanner_line(source_name))
        }
        KnownFileLookup::KnownFile {
            file_name: KnownName::UnnamedFile,
            source_name: ListingSource::UnnamedSource,
        } => translated_messages::host::listed_known_file_unnamed_source(),
        KnownFileLookup::KnownFile {
            file_name: KnownName::NamedFile(file_name),
            source_name: ListingSource::NamedSource(source_name),
        } => translated_messages::host::known_file_name_listed(
            sanitize_scanner_line(file_name),
            sanitize_scanner_line(source_name),
        ),
        KnownFileLookup::KnownFile {
            file_name: KnownName::NamedFile(file_name),
            source_name: ListingSource::UnnamedSource,
        } => translated_messages::host::known_file_name_unnamed_source(sanitize_scanner_line(
            file_name,
        )),
    };
    SourceAnswer::Answered { verdict_text }
}

pub fn malware_family_text(malware_family: &MalwareFamily) -> String {
    match malware_family {
        MalwareFamily::NamedFamily(family_name) => sanitize_scanner_line(family_name),
        MalwareFamily::UnnamedFamily => translated_messages::host::unnamed_family(),
    }
}

pub fn describe_known_samples(sample_lookup: &KnownSampleLookup) -> SourceAnswer {
    let verdict_text = match sample_lookup {
        KnownSampleLookup::NoKnownSamples => translated_messages::host::no_known_samples(),
        KnownSampleLookup::SamplesFound { known_samples } => known_samples.first().map_or_else(
            translated_messages::host::known_malware_sample,
            |first_sample| match (&first_sample.malware_family, &first_sample.first_seen) {
                (MalwareFamily::NamedFamily(family_name), SampleFirstSeen::SeenAt(seen_at)) => {
                    translated_messages::host::samples_found(
                        sanitize_scanner_line(family_name),
                        sanitize_scanner_line(seen_at),
                    )
                }
                (MalwareFamily::NamedFamily(family_name), SampleFirstSeen::FirstSeenUnknown) => {
                    translated_messages::host::samples_found_date_unknown(sanitize_scanner_line(
                        family_name,
                    ))
                }
                (MalwareFamily::UnnamedFamily, SampleFirstSeen::SeenAt(seen_at)) => {
                    translated_messages::host::unnamed_family_samples_found(sanitize_scanner_line(
                        seen_at,
                    ))
                }
                (MalwareFamily::UnnamedFamily, SampleFirstSeen::FirstSeenUnknown) => {
                    translated_messages::host::unnamed_family_date_unknown()
                }
            },
        ),
    };
    SourceAnswer::Answered { verdict_text }
}

pub fn describe_virustotal_report(report_lookup: &FileReportLookup) -> SourceAnswer {
    match report_lookup {
        FileReportLookup::FileUnknownToVirusTotal => SourceAnswer::Answered {
            verdict_text: translated_messages::host::file_unknown_virustotal(),
        },
        FileReportLookup::ReportAvailable(file_report) => SourceAnswer::AnsweredWithReport {
            verdict_text: translated_messages::host::count_total_engine_verdict(
                file_report.total_engine_count,
                file_report.detecting_engine_count,
            ),
            report_url: file_report.report_url.clone(),
        },
    }
}

pub fn describe_virustotal_failure(virustotal_failure: &VirusTotalFailure) -> String {
    match virustotal_failure {
        VirusTotalFailure::ApiKeyRejected => translated_messages::host::api_key_rejected(),
        VirusTotalFailure::RequestLimitReached => {
            translated_messages::host::request_limit_reached()
        }
        VirusTotalFailure::UnexpectedStatus { status_code } => {
            translated_messages::host::virustotal_answered_status(status_code)
        }
        VirusTotalFailure::RequestFailed(request_failure) => {
            translated_messages::host::virustotal_not_reached_error(request_failure)
        }
        VirusTotalFailure::AnswerUnreadable => {
            translated_messages::host::virustotal_sent_answer_lazaret()
        }
    }
}

fn describe_circl_failure(circl_failure: &CirclLookupFailure) -> String {
    match circl_failure {
        CirclLookupFailure::RequestFailed(request_failure) => {
            translated_messages::host::request_failed_circl(request_failure)
        }
        CirclLookupFailure::AnswerUnreadable => {
            translated_messages::host::answer_unreadable_circl()
        }
        CirclLookupFailure::UnexpectedStatus { status_code } => {
            translated_messages::host::unexpected_status_circl(status_code)
        }
    }
}

pub fn describe_bazaar_failure(bazaar_failure: &MalwareBazaarFailure) -> String {
    match bazaar_failure {
        MalwareBazaarFailure::RequestFailed(request_failure) => {
            translated_messages::host::request_failed_bazaar(request_failure)
        }
        MalwareBazaarFailure::AnswerUnreadable => {
            translated_messages::host::answer_unreadable_bazaar()
        }
        MalwareBazaarFailure::KeyRefused => translated_messages::host::key_refused(),
        MalwareBazaarFailure::QueryRefused { query_status } => {
            translated_messages::host::query_refused(sanitize_scanner_line(query_status))
        }
        MalwareBazaarFailure::RequestLimitReached => {
            translated_messages::host::request_limit_reached_bazaar()
        }
        MalwareBazaarFailure::UnexpectedStatus { status_code } => {
            translated_messages::host::unexpected_status_bazaar(status_code)
        }
    }
}

fn describe_lookup_failure(lookup_failure: &ReputationLookupFailure) -> String {
    match lookup_failure {
        ReputationLookupFailure::KeyUnavailable(key_failure) => describe_key_failure(key_failure),
        ReputationLookupFailure::CirclLookupFailed(circl_failure) => {
            describe_circl_failure(circl_failure)
        }
        ReputationLookupFailure::BazaarLookupFailed(bazaar_failure) => {
            describe_bazaar_failure(bazaar_failure)
        }
        ReputationLookupFailure::VirusTotalLookupFailed(virustotal_failure) => {
            describe_virustotal_failure(virustotal_failure)
        }
    }
}

enum StoredServiceKey {
    KeyAvailable { api_key: String },
    KeyNotConfigured,
}

async fn stored_key_for(
    threat_intel_service: ThreatIntelService,
) -> Result<StoredServiceKey, ReputationLookupFailure> {
    Ok(match read_api_key(threat_intel_service).await? {
        ResolvedApiKey::FromKeyFile(api_key) | ResolvedApiKey::FromKeyring(api_key) => {
            StoredServiceKey::KeyAvailable { api_key }
        }
        ResolvedApiKey::NotConfigured => StoredServiceKey::KeyNotConfigured,
    })
}

async fn look_up_circl(content_hash: &str) -> Result<SourceAnswer, ReputationLookupFailure> {
    Ok(describe_known_file(
        &look_up_known_file(content_hash).await?,
    ))
}

async fn look_up_bazaar(content_hash: &str) -> Result<SourceAnswer, ReputationLookupFailure> {
    let StoredServiceKey::KeyAvailable { api_key } =
        stored_key_for(ThreatIntelService::AbuseChService).await?
    else {
        return Ok(SourceAnswer::NeedsKey);
    };
    Ok(describe_known_samples(
        &look_up_file_hash(&api_key, content_hash).await?,
    ))
}

async fn look_up_virustotal(content_hash: &str) -> Result<SourceAnswer, ReputationLookupFailure> {
    let StoredServiceKey::KeyAvailable { api_key } =
        stored_key_for(ThreatIntelService::VirusTotalService).await?
    else {
        return Ok(SourceAnswer::NeedsKey);
    };
    let lookup_client = VirusTotalClient::with_api_key(api_key)?;
    Ok(describe_virustotal_report(
        &lookup_client.look_up_file_report(content_hash).await?,
    ))
}

fn answer_or_failure(source_lookup: Result<SourceAnswer, ReputationLookupFailure>) -> SourceAnswer {
    source_lookup.unwrap_or_else(SourceAnswer::from)
}

pub async fn check_hash_reputation(content_hash: String) -> ReputationAnswers {
    let (circl_answer, bazaar_answer, virustotal_answer) = futures::join!(
        look_up_circl(&content_hash),
        look_up_bazaar(&content_hash),
        look_up_virustotal(&content_hash),
    );
    ReputationAnswers {
        circl_answer: answer_or_failure(circl_answer),
        bazaar_answer: answer_or_failure(bazaar_answer),
        virustotal_answer: answer_or_failure(virustotal_answer),
    }
}

#[cfg(test)]
mod file_reputation_check_tests {
    use super::*;
    use crate::host_services::threat_intel::{
        malware_bazaar_client::SampleSummary, virustotal_api_client::VirusTotalReport,
    };

    #[test]
    fn a_known_file_names_its_source() {
        assert_eq!(
            describe_known_file(&KnownFileLookup::KnownFile {
                file_name: KnownName::NamedFile("./usr/bin/ls".to_owned()),
                source_name: ListingSource::NamedSource("NSRL".to_owned()),
            })
            .displayed_text(),
            "Known file ./usr/bin/ls, listed by NSRL"
        );
        assert_eq!(
            describe_known_file(&KnownFileLookup::KnownMalicious {
                file_name: KnownName::NamedFile("eicar.com".to_owned()),
                flagged_by: ListingSource::NamedSource("malshare.com".to_owned()),
            })
            .displayed_text(),
            "Listed as malicious by malshare.com (as eicar.com)"
        );
        assert_eq!(
            describe_known_file(&KnownFileLookup::KnownMalicious {
                file_name: KnownName::UnnamedFile,
                flagged_by: ListingSource::UnnamedSource,
            })
            .displayed_text(),
            "Listed as malicious by an unnamed source"
        );
        assert_eq!(
            describe_known_file(&KnownFileLookup::HashUnknown).displayed_text(),
            "Not in the known-files database"
        );
    }

    #[test]
    fn a_known_sample_names_its_family_and_date() {
        assert_eq!(
            describe_known_samples(&KnownSampleLookup::SamplesFound {
                known_samples: vec![SampleSummary {
                    malware_family: MalwareFamily::NamedFamily("AgentTesla".to_owned()),
                    first_seen: SampleFirstSeen::SeenAt("2026-10-01".to_owned()),
                    sample_tags: Vec::new(),
                }],
            })
            .displayed_text(),
            "Known malware: AgentTesla, first seen 2026-10-01"
        );
        assert_eq!(
            describe_known_samples(&KnownSampleLookup::SamplesFound {
                known_samples: vec![SampleSummary {
                    malware_family: MalwareFamily::UnnamedFamily,
                    first_seen: SampleFirstSeen::FirstSeenUnknown,
                    sample_tags: Vec::new(),
                }],
            })
            .displayed_text(),
            "Known malware of an unnamed family, first seen on an unknown date"
        );
    }

    #[test]
    fn a_virustotal_report_counts_engines_and_links_the_report() {
        let SourceAnswer::AnsweredWithReport {
            verdict_text,
            report_url,
        } = describe_virustotal_report(&FileReportLookup::ReportAvailable(VirusTotalReport {
            content_hash: "abc".to_owned(),
            detecting_engine_count: 3,
            total_engine_count: 71,
            engine_detections: Vec::new(),
            report_url: "https://www.virustotal.com/gui/file/abc".to_owned(),
        }))
        else {
            panic!("a VirusTotal report links to its page");
        };
        assert_eq!(verdict_text, "3 of 71 engines detect it");
        assert_eq!(report_url, "https://www.virustotal.com/gui/file/abc");
    }

    #[test]
    fn lookup_failures_are_described_by_their_kind() {
        assert_eq!(
            SourceAnswer::from(ReputationLookupFailure::from(
                VirusTotalFailure::UnexpectedStatus { status_code: 503 }
            ))
            .displayed_text(),
            "VirusTotal answered with status 503"
        );
        assert_eq!(
            SourceAnswer::from(ReputationLookupFailure::from(
                MalwareBazaarFailure::QueryRefused {
                    query_status: "illegal_hash".to_owned(),
                }
            ))
            .displayed_text(),
            "MalwareBazaar answered: illegal_hash"
        );
    }
}
