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
            service_access::{ApiKey, OnlineServiceAccess},
            virustotal_api_client::{FileReportLookup, VirusTotalClient, VirusTotalFailure},
        },
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

#[derive(Debug, thiserror::Error)]
pub enum ReputationLookupFailure {
    #[error(transparent)]
    CirclLookupFailed(#[from] CirclLookupFailure),
    #[error(transparent)]
    BazaarLookupFailed(#[from] MalwareBazaarFailure),
    #[error(transparent)]
    VirusTotalLookupFailed(#[from] VirusTotalFailure),
}

#[derive(Clone, Debug)]
pub enum SourceAnswer {
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
    pub fn displayed_text(&self, message_catalog: &MessageCatalog) -> String {
        match self {
            Self::Answered { verdict_text } | Self::AnsweredWithReport { verdict_text, .. } => {
                verdict_text.clone()
            }
            Self::Failed(lookup_failure) => {
                describe_lookup_failure(message_catalog, lookup_failure)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum KeyedSourceAnswer {
    SkippedWithoutKey,
    KeyedAnswer(SourceAnswer),
}

#[derive(Clone, Debug)]
pub struct ReputationAnswers {
    pub circl_answer: SourceAnswer,
    pub bazaar_answer: KeyedSourceAnswer,
    pub virustotal_answer: KeyedSourceAnswer,
}

pub fn describe_known_file(
    message_catalog: &MessageCatalog,
    known_file_lookup: &KnownFileLookup,
) -> SourceAnswer {
    let verdict_text = match known_file_lookup {
        KnownFileLookup::HashUnknown => translated_messages::host::hash_unknown(message_catalog),
        KnownFileLookup::KnownMalicious {
            file_name: KnownName::NamedFile(file_name),
            flagged_by: ListingSource::NamedSource(source_name),
        } => translated_messages::host::listed_malicious_source_name(
            message_catalog,
            sanitize_scanner_line(source_name),
            sanitize_scanner_line(file_name),
        ),
        KnownFileLookup::KnownMalicious {
            file_name: KnownName::NamedFile(file_name),
            flagged_by: ListingSource::UnnamedSource,
        } => translated_messages::host::listed_malicious_unnamed_source_name(
            message_catalog,
            sanitize_scanner_line(file_name),
        ),
        KnownFileLookup::KnownMalicious {
            file_name: KnownName::UnnamedFile,
            flagged_by: ListingSource::NamedSource(source_name),
        } => translated_messages::host::listed_malicious_source(
            message_catalog,
            sanitize_scanner_line(source_name),
        ),
        KnownFileLookup::KnownMalicious {
            file_name: KnownName::UnnamedFile,
            flagged_by: ListingSource::UnnamedSource,
        } => translated_messages::host::listed_malicious_unnamed_source(message_catalog),
        KnownFileLookup::KnownFile {
            file_name: KnownName::UnnamedFile,
            source_name: ListingSource::NamedSource(source_name),
        } => translated_messages::host::listed_known_file_source(
            message_catalog,
            sanitize_scanner_line(source_name),
        ),
        KnownFileLookup::KnownFile {
            file_name: KnownName::UnnamedFile,
            source_name: ListingSource::UnnamedSource,
        } => translated_messages::host::listed_known_file_unnamed_source(message_catalog),
        KnownFileLookup::KnownFile {
            file_name: KnownName::NamedFile(file_name),
            source_name: ListingSource::NamedSource(source_name),
        } => translated_messages::host::known_file_name_listed(
            message_catalog,
            sanitize_scanner_line(file_name),
            sanitize_scanner_line(source_name),
        ),
        KnownFileLookup::KnownFile {
            file_name: KnownName::NamedFile(file_name),
            source_name: ListingSource::UnnamedSource,
        } => translated_messages::host::known_file_name_unnamed_source(
            message_catalog,
            sanitize_scanner_line(file_name),
        ),
    };
    SourceAnswer::Answered { verdict_text }
}

pub fn malware_family_text(
    message_catalog: &MessageCatalog,
    malware_family: &MalwareFamily,
) -> String {
    match malware_family {
        MalwareFamily::NamedFamily(family_name) => sanitize_scanner_line(family_name),
        MalwareFamily::UnnamedFamily => translated_messages::host::unnamed_family(message_catalog),
    }
}

pub fn describe_known_samples(
    message_catalog: &MessageCatalog,
    sample_lookup: &KnownSampleLookup,
) -> SourceAnswer {
    let verdict_text = match sample_lookup {
        KnownSampleLookup::NoKnownSamples => {
            translated_messages::host::no_known_samples(message_catalog)
        }
        KnownSampleLookup::SamplesFound { known_samples } => known_samples.first().map_or_else(
            || translated_messages::host::known_malware_sample(message_catalog),
            |first_sample| match (&first_sample.malware_family, &first_sample.first_seen) {
                (MalwareFamily::NamedFamily(family_name), SampleFirstSeen::SeenAt(seen_at)) => {
                    translated_messages::host::samples_found(
                        message_catalog,
                        sanitize_scanner_line(family_name),
                        sanitize_scanner_line(seen_at),
                    )
                }
                (MalwareFamily::NamedFamily(family_name), SampleFirstSeen::FirstSeenUnknown) => {
                    translated_messages::host::samples_found_date_unknown(
                        message_catalog,
                        sanitize_scanner_line(family_name),
                    )
                }
                (MalwareFamily::UnnamedFamily, SampleFirstSeen::SeenAt(seen_at)) => {
                    translated_messages::host::unnamed_family_samples_found(
                        message_catalog,
                        sanitize_scanner_line(seen_at),
                    )
                }
                (MalwareFamily::UnnamedFamily, SampleFirstSeen::FirstSeenUnknown) => {
                    translated_messages::host::unnamed_family_date_unknown(message_catalog)
                }
            },
        ),
    };
    SourceAnswer::Answered { verdict_text }
}

pub fn describe_virustotal_report(
    message_catalog: &MessageCatalog,
    report_lookup: &FileReportLookup,
) -> SourceAnswer {
    match report_lookup {
        FileReportLookup::FileUnknownToVirusTotal => SourceAnswer::Answered {
            verdict_text: translated_messages::host::file_unknown_virustotal(message_catalog),
        },
        FileReportLookup::ReportAvailable(file_report) => SourceAnswer::AnsweredWithReport {
            verdict_text: translated_messages::host::count_total_engine_verdict(
                message_catalog,
                file_report.total_engine_count,
                file_report.detecting_engine_count,
            ),
            report_url: file_report.report_url.clone(),
        },
    }
}

pub fn describe_virustotal_failure(
    message_catalog: &MessageCatalog,
    virustotal_failure: &VirusTotalFailure,
) -> String {
    match virustotal_failure {
        VirusTotalFailure::ApiKeyRejected => {
            translated_messages::host::api_key_rejected(message_catalog)
        }
        VirusTotalFailure::RequestLimitReached => {
            translated_messages::host::request_limit_reached(message_catalog)
        }
        VirusTotalFailure::UnexpectedStatus { status_code } => {
            translated_messages::host::virustotal_answered_status(message_catalog, status_code)
        }
        VirusTotalFailure::RequestFailed(request_failure) => {
            translated_messages::host::virustotal_not_reached_error(
                message_catalog,
                request_failure,
            )
        }
        VirusTotalFailure::AnswerUnreadable => {
            translated_messages::host::virustotal_sent_answer_lazaret(message_catalog)
        }
    }
}

fn describe_circl_failure(
    message_catalog: &MessageCatalog,
    circl_failure: &CirclLookupFailure,
) -> String {
    match circl_failure {
        CirclLookupFailure::RequestFailed(request_failure) => {
            translated_messages::host::request_failed_circl(message_catalog, request_failure)
        }
        CirclLookupFailure::AnswerUnreadable => {
            translated_messages::host::answer_unreadable_circl(message_catalog)
        }
        CirclLookupFailure::UnexpectedStatus { status_code } => {
            translated_messages::host::unexpected_status_circl(message_catalog, status_code)
        }
    }
}

pub fn describe_bazaar_failure(
    message_catalog: &MessageCatalog,
    bazaar_failure: &MalwareBazaarFailure,
) -> String {
    match bazaar_failure {
        MalwareBazaarFailure::RequestFailed(request_failure) => {
            translated_messages::host::request_failed_bazaar(message_catalog, request_failure)
        }
        MalwareBazaarFailure::AnswerUnreadable => {
            translated_messages::host::answer_unreadable_bazaar(message_catalog)
        }
        MalwareBazaarFailure::KeyRefused => translated_messages::host::key_refused(message_catalog),
        MalwareBazaarFailure::QueryRefused { query_status } => {
            translated_messages::host::query_refused(
                message_catalog,
                sanitize_scanner_line(query_status),
            )
        }
        MalwareBazaarFailure::RequestLimitReached => {
            translated_messages::host::request_limit_reached_bazaar(message_catalog)
        }
        MalwareBazaarFailure::UnexpectedStatus { status_code } => {
            translated_messages::host::unexpected_status_bazaar(message_catalog, status_code)
        }
    }
}

fn describe_lookup_failure(
    message_catalog: &MessageCatalog,
    lookup_failure: &ReputationLookupFailure,
) -> String {
    match lookup_failure {
        ReputationLookupFailure::CirclLookupFailed(circl_failure) => {
            describe_circl_failure(message_catalog, circl_failure)
        }
        ReputationLookupFailure::BazaarLookupFailed(bazaar_failure) => {
            describe_bazaar_failure(message_catalog, bazaar_failure)
        }
        ReputationLookupFailure::VirusTotalLookupFailed(virustotal_failure) => {
            describe_virustotal_failure(message_catalog, virustotal_failure)
        }
    }
}

#[derive(Debug)]
pub enum KeyedLookup<LookupResult> {
    SkippedWithoutKey,
    LookedUp(Result<LookupResult, ReputationLookupFailure>),
}

#[derive(Debug)]
pub struct ReputationLookups {
    circl_lookup: Result<KnownFileLookup, ReputationLookupFailure>,
    bazaar_lookup: KeyedLookup<KnownSampleLookup>,
    virustotal_lookup: KeyedLookup<FileReportLookup>,
}

async fn look_up_circl(content_hash: &str) -> Result<KnownFileLookup, ReputationLookupFailure> {
    Ok(look_up_known_file(content_hash).await?)
}

async fn look_up_bazaar(
    auth_key: &ApiKey,
    content_hash: &str,
) -> Result<KnownSampleLookup, ReputationLookupFailure> {
    Ok(look_up_file_hash(auth_key.as_str(), content_hash).await?)
}

async fn look_up_virustotal(
    api_key: &ApiKey,
    content_hash: &str,
) -> Result<FileReportLookup, ReputationLookupFailure> {
    let lookup_client = VirusTotalClient::with_api_key(api_key.as_str().to_owned())?;
    Ok(lookup_client.look_up_file_report(content_hash).await?)
}

async fn keyed_lookup<'lookup, LookupResult, LookupFuture>(
    ready_key: Option<&'lookup ApiKey>,
    run_lookup: impl FnOnce(&'lookup ApiKey) -> LookupFuture,
) -> KeyedLookup<LookupResult>
where
    LookupFuture: Future<Output = Result<LookupResult, ReputationLookupFailure>>,
{
    match ready_key {
        Some(api_key) => KeyedLookup::LookedUp(run_lookup(api_key).await),
        None => KeyedLookup::SkippedWithoutKey,
    }
}

pub async fn look_up_hash_reputation(
    content_hash: String,
    service_access: OnlineServiceAccess,
) -> ReputationLookups {
    let (circl_lookup, bazaar_lookup, virustotal_lookup) = futures::join!(
        look_up_circl(&content_hash),
        keyed_lookup(
            service_access.malware_bazaar_access.ready_key(),
            |auth_key| look_up_bazaar(auth_key, &content_hash),
        ),
        keyed_lookup(service_access.virustotal_access.ready_key(), |api_key| {
            look_up_virustotal(api_key, &content_hash)
        }),
    );
    ReputationLookups {
        circl_lookup,
        bazaar_lookup,
        virustotal_lookup,
    }
}

fn described_lookup<LookupResult>(
    source_lookup: Result<LookupResult, ReputationLookupFailure>,
    describe_found: impl FnOnce(&LookupResult) -> SourceAnswer,
) -> SourceAnswer {
    match source_lookup {
        Ok(found_result) => describe_found(&found_result),
        Err(lookup_failure) => SourceAnswer::from(lookup_failure),
    }
}

fn described_keyed_lookup<LookupResult>(
    keyed_lookup: KeyedLookup<LookupResult>,
    describe_found: impl FnOnce(&LookupResult) -> SourceAnswer,
) -> KeyedSourceAnswer {
    match keyed_lookup {
        KeyedLookup::SkippedWithoutKey => KeyedSourceAnswer::SkippedWithoutKey,
        KeyedLookup::LookedUp(source_lookup) => {
            KeyedSourceAnswer::KeyedAnswer(described_lookup(source_lookup, describe_found))
        }
    }
}

pub fn describe_reputation(
    message_catalog: &MessageCatalog,
    reputation_lookups: ReputationLookups,
) -> ReputationAnswers {
    ReputationAnswers {
        circl_answer: described_lookup(reputation_lookups.circl_lookup, |known_file_lookup| {
            describe_known_file(message_catalog, known_file_lookup)
        }),
        bazaar_answer: described_keyed_lookup(reputation_lookups.bazaar_lookup, |sample_lookup| {
            describe_known_samples(message_catalog, sample_lookup)
        }),
        virustotal_answer: described_keyed_lookup(
            reputation_lookups.virustotal_lookup,
            |report_lookup| describe_virustotal_report(message_catalog, report_lookup),
        ),
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
        let message_catalog = &crate::translated_messages::english_catalog();
        assert_eq!(
            describe_known_file(
                message_catalog,
                &KnownFileLookup::KnownFile {
                    file_name: KnownName::NamedFile("./usr/bin/ls".to_owned()),
                    source_name: ListingSource::NamedSource("NSRL".to_owned()),
                }
            )
            .displayed_text(message_catalog),
            "Known file ./usr/bin/ls, listed by NSRL"
        );
        assert_eq!(
            describe_known_file(
                message_catalog,
                &KnownFileLookup::KnownMalicious {
                    file_name: KnownName::NamedFile("eicar.com".to_owned()),
                    flagged_by: ListingSource::NamedSource("malshare.com".to_owned()),
                }
            )
            .displayed_text(message_catalog),
            "Listed as malicious by malshare.com (as eicar.com)"
        );
        assert_eq!(
            describe_known_file(
                message_catalog,
                &KnownFileLookup::KnownMalicious {
                    file_name: KnownName::UnnamedFile,
                    flagged_by: ListingSource::UnnamedSource,
                }
            )
            .displayed_text(message_catalog),
            "Listed as malicious by an unnamed source"
        );
        assert_eq!(
            describe_known_file(message_catalog, &KnownFileLookup::HashUnknown)
                .displayed_text(message_catalog),
            "Not in the known-files database"
        );
    }

    #[test]
    fn a_known_sample_names_its_family_and_date() {
        let message_catalog = &crate::translated_messages::english_catalog();
        assert_eq!(
            describe_known_samples(
                message_catalog,
                &KnownSampleLookup::SamplesFound {
                    known_samples: vec![SampleSummary {
                        malware_family: MalwareFamily::NamedFamily("AgentTesla".to_owned()),
                        first_seen: SampleFirstSeen::SeenAt("2026-10-01".to_owned()),
                        sample_tags: Vec::new(),
                    }],
                }
            )
            .displayed_text(message_catalog),
            "Known malware: AgentTesla, first seen 2026-10-01"
        );
        assert_eq!(
            describe_known_samples(
                message_catalog,
                &KnownSampleLookup::SamplesFound {
                    known_samples: vec![SampleSummary {
                        malware_family: MalwareFamily::UnnamedFamily,
                        first_seen: SampleFirstSeen::FirstSeenUnknown,
                        sample_tags: Vec::new(),
                    }],
                }
            )
            .displayed_text(message_catalog),
            "Known malware of an unnamed family, first seen on an unknown date"
        );
    }

    #[test]
    fn a_virustotal_report_counts_engines_and_links_the_report() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let SourceAnswer::AnsweredWithReport {
            verdict_text,
            report_url,
        } = describe_virustotal_report(
            message_catalog,
            &FileReportLookup::ReportAvailable(VirusTotalReport {
                content_hash: "abc".to_owned(),
                detecting_engine_count: 3,
                total_engine_count: 71,
                engine_detections: Vec::new(),
                report_url: "https://www.virustotal.com/gui/file/abc".to_owned(),
            }),
        )
        else {
            panic!("a VirusTotal report links to its page");
        };
        assert_eq!(verdict_text, "3 of 71 engines detect it");
        assert_eq!(report_url, "https://www.virustotal.com/gui/file/abc");
    }

    #[test]
    fn lookup_failures_are_described_by_their_kind() {
        let message_catalog = &crate::translated_messages::english_catalog();
        assert_eq!(
            SourceAnswer::from(ReputationLookupFailure::from(
                VirusTotalFailure::UnexpectedStatus { status_code: 503 }
            ))
            .displayed_text(message_catalog),
            "VirusTotal answered with status 503"
        );
        assert_eq!(
            SourceAnswer::from(ReputationLookupFailure::from(
                MalwareBazaarFailure::QueryRefused {
                    query_status: "illegal_hash".to_owned(),
                }
            ))
            .displayed_text(message_catalog),
            "MalwareBazaar answered: illegal_hash"
        );
    }
}
