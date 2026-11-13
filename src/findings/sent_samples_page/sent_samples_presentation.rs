use crate::{
    findings::sent_samples_page::{
        sent_samples_state::{
            CandidateListing, ProgressFilter, SendAvailability, ServiceFilter, SubmissionFilters,
            SubmissionListing, offered_target_activities, send_availability,
        },
        uploads_in_flight::UploadsInFlight,
    },
    host_services::{
        activity_history::submission_history_store::{
            BazaarSampleStatus, FalsePositiveReview, MissedMalwareReview, RemoteEntryLink,
            SignatureDatabaseVersion, SubmissionKind, SubmissionProgress, SubmissionRecord,
            SubmissionService, SubmissionState, SubmissionTarget, VirusTotalSampleStatus,
        },
        clamav_scanner::scanner_output_parser::{displayed_threat_name, sanitize_scanner_line},
        desktop::locale_formatting::MomentFormatters,
        quarantine_vault::{
            quarantine_entry_database::QuarantineEntryRecord,
            quarantine_file_moves::RecordedPayloadHash,
        },
        sample_submissions::{
            remote_status_refresh::{RemoteRefreshFailure, RemoteRefreshSummary},
            submission_samples::shown_file_name,
        },
        threat_intel::file_reputation_check::{
            describe_bazaar_failure, describe_virustotal_failure,
        },
    },
    shared_widgets::{
        quarantine_failure_text::describe_quarantine_failure,
        submission_outcome_text::history_failure_text,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

const SHORT_HASH_LENGTH: usize = 12;

pub const fn service_name(submission_service: SubmissionService) -> &'static str {
    match submission_service {
        SubmissionService::ClamavProject => "ClamAV",
        SubmissionService::MalwareBazaar => "MalwareBazaar",
        SubmissionService::VirusTotal => "VirusTotal",
    }
}

pub fn kind_text(message_catalog: &MessageCatalog, submission_kind: SubmissionKind) -> String {
    match submission_kind {
        SubmissionKind::FalsePositive => {
            translated_messages::findings::false_positive(message_catalog)
        }
        SubmissionKind::MissedMalware => {
            translated_messages::findings::missed_malware(message_catalog)
        }
        SubmissionKind::MalwareSample => {
            translated_messages::findings::malware_sample(message_catalog)
        }
    }
}

pub fn target_choice_label(
    message_catalog: &MessageCatalog,
    submission_target: SubmissionTarget,
) -> String {
    match submission_target {
        SubmissionTarget::ClamavFalsePositive => {
            translated_messages::findings::clamav_false_positive(message_catalog)
        }
        SubmissionTarget::ClamavMissedMalware => {
            translated_messages::findings::clamav_missed_malware(message_catalog)
        }
        SubmissionTarget::MalwareBazaarSample => {
            translated_messages::findings::malwarebazaar_sample_choice(message_catalog)
        }
        SubmissionTarget::VirusTotalSample => {
            translated_messages::findings::virustotal_sample_choice(message_catalog)
        }
    }
}

pub fn short_hash(content_sha256: &str) -> String {
    content_sha256.chars().take(SHORT_HASH_LENGTH).collect()
}

fn settled_version_text(
    database_version: SignatureDatabaseVersion,
    versioned_text: impl FnOnce(&str) -> String,
    unversioned_text: String,
) -> String {
    match database_version {
        SignatureDatabaseVersion::DailyVersion { version_number } => {
            versioned_text(&version_number.to_string())
        }
        SignatureDatabaseVersion::VersionUnreadable => unversioned_text,
    }
}

fn awaiting_clamav_text(message_catalog: &MessageCatalog) -> String {
    translated_messages::findings::awaiting_clamav(message_catalog)
}

pub fn status_text(message_catalog: &MessageCatalog, submission_state: &SubmissionState) -> String {
    match submission_state {
        SubmissionState::ClamavFalsePositive {
            review_status: FalsePositiveReview::AwaitingClamavReview,
            ..
        }
        | SubmissionState::ClamavMissedMalware {
            review_status: MissedMalwareReview::AwaitingClamavReview,
        } => awaiting_clamav_text(message_catalog),
        SubmissionState::ClamavFalsePositive {
            review_status: FalsePositiveReview::FixedInDatabase { database_version },
            ..
        } => settled_version_text(
            *database_version,
            |version_text| {
                translated_messages::findings::fixed_database_version(message_catalog, version_text)
            },
            translated_messages::findings::fixed_current_signature_database(message_catalog),
        ),
        SubmissionState::ClamavMissedMalware {
            review_status: MissedMalwareReview::NowDetected { database_version },
        } => settled_version_text(
            *database_version,
            |version_text| {
                translated_messages::findings::detected_since_database_version(
                    message_catalog,
                    version_text,
                )
            },
            translated_messages::findings::detected_current_signature_database(message_catalog),
        ),
        SubmissionState::MalwareBazaarSample {
            bazaar_status: BazaarSampleStatus::UploadAccepted,
        } => translated_messages::findings::uploaded_waiting_malwarebazaar_list(message_catalog),
        SubmissionState::MalwareBazaarSample {
            bazaar_status: BazaarSampleStatus::ListedOnMalwareBazaar,
        } => translated_messages::findings::listed_malwarebazaar(message_catalog),
        SubmissionState::MalwareBazaarSample {
            bazaar_status: BazaarSampleStatus::SubmissionRefused { refusal_reason },
        } => translated_messages::findings::malwarebazaar_refused_reason(
            message_catalog,
            sanitize_scanner_line(refusal_reason),
        ),
        SubmissionState::VirusTotalSample {
            virustotal_status: VirusTotalSampleStatus::VirusTotalAnalysisQueued { .. },
        } => translated_messages::findings::queued_analysis_virustotal(message_catalog),
        SubmissionState::VirusTotalSample {
            virustotal_status:
                VirusTotalSampleStatus::VirusTotalAnalysed {
                    malicious_count,
                    engine_count,
                    ..
                },
        } => translated_messages::findings::count_total_engine_flags(
            message_catalog,
            *engine_count,
            malicious_count,
        ),
        SubmissionState::VirusTotalSample {
            virustotal_status: VirusTotalSampleStatus::SubmissionRefused { refusal_reason },
        } => translated_messages::findings::virustotal_refused_reason(
            message_catalog,
            sanitize_scanner_line(refusal_reason),
        ),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubmittedRowView {
    pub row_title: String,
    pub row_subtitle: String,
    pub needs_attention: bool,
    pub entry_link: RemoteEntryLink,
}

pub fn submitted_row_view(
    message_catalog: &MessageCatalog,
    submission_record: &SubmissionRecord,
    moment_formatters: &MomentFormatters,
) -> SubmittedRowView {
    let submission_target = submission_record.submission_state.submission_target();
    let summary_line = translated_messages::findings::service_kind_hash_date(
        message_catalog,
        service_name(submission_target.submission_service()),
        kind_text(message_catalog, submission_target.submission_kind()),
        short_hash(&submission_record.submitted_file.content_sha256),
        moment_formatters.exact_moment_text(submission_record.submitted_at),
    );
    SubmittedRowView {
        row_title: sanitize_scanner_line(&submission_record.submitted_file.file_name),
        row_subtitle: format!(
            "{summary_line}\n{}",
            status_text(message_catalog, &submission_record.submission_state)
        ),
        needs_attention: submission_record.submission_state.submission_progress()
            == SubmissionProgress::RefusedByService,
        entry_link: submission_record
            .submission_state
            .remote_entry_link(&submission_record.submitted_file.content_sha256),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateRowView {
    pub entry_id: i64,
    pub row_title: String,
    pub row_subtitle: String,
    pub send_availability: SendAvailability,
}

impl CandidateRowView {
    pub const fn allows_sending(&self) -> bool {
        matches!(self.send_availability, SendAvailability::SendableNow)
    }
}

pub fn candidate_row_view(
    message_catalog: &MessageCatalog,
    entry_record: &QuarantineEntryRecord,
    uploads_in_flight: &UploadsInFlight,
) -> CandidateRowView {
    let detection_text = displayed_threat_name(message_catalog, &entry_record.threat_identity);
    CandidateRowView {
        entry_id: entry_record.entry_id,
        row_title: shown_file_name(&entry_record.original_path),
        row_subtitle: match &entry_record.payload_hash {
            RecordedPayloadHash::KnownSha256 { sha256_hex } => {
                format!("{detection_text} · {}", short_hash(sha256_hex))
            }
            RecordedPayloadHash::IntegrityUnknown => {
                translated_messages::findings::integrity_unknown_candidate(
                    message_catalog,
                    &detection_text,
                )
            }
        },
        send_availability: send_availability(&offered_target_activities(
            entry_record,
            uploads_in_flight,
        )),
    }
}

pub fn service_filter_label(
    message_catalog: &MessageCatalog,
    service_filter: ServiceFilter,
) -> String {
    match service_filter {
        ServiceFilter::AllServices => translated_messages::findings::all_services(message_catalog),
        ServiceFilter::OnlyService(submission_service) => {
            service_name(submission_service).to_owned()
        }
    }
}

pub fn progress_filter_label(
    message_catalog: &MessageCatalog,
    progress_filter: ProgressFilter,
) -> String {
    match progress_filter {
        ProgressFilter::AllProgress => {
            translated_messages::findings::any_status_option(message_catalog)
        }
        ProgressFilter::OnlyProgress(SubmissionProgress::AwaitingOutcome) => {
            translated_messages::findings::waiting_label(message_catalog)
        }
        ProgressFilter::OnlyProgress(SubmissionProgress::OutcomeKnown) => {
            translated_messages::findings::settled_label(message_catalog)
        }
        ProgressFilter::OnlyProgress(SubmissionProgress::RefusedByService) => {
            translated_messages::findings::refused_label(message_catalog)
        }
    }
}

pub fn filter_summary_text(
    message_catalog: &MessageCatalog,
    submission_filters: &SubmissionFilters,
) -> String {
    match (
        submission_filters.service_filter,
        submission_filters.progress_filter,
    ) {
        (ServiceFilter::AllServices, ProgressFilter::AllProgress) => {
            translated_messages::findings::all_submissions_option(message_catalog)
        }
        (ServiceFilter::AllServices, progress_filter) => {
            progress_filter_label(message_catalog, progress_filter)
        }
        (service_filter, ProgressFilter::AllProgress) => {
            service_filter_label(message_catalog, service_filter)
        }
        (service_filter, progress_filter) => format!(
            "{} · {}",
            service_filter_label(message_catalog, service_filter),
            progress_filter_label(message_catalog, progress_filter)
        ),
    }
}

pub fn empty_submissions_text(
    message_catalog: &MessageCatalog,
    submission_listing: &SubmissionListing,
) -> String {
    match submission_listing {
        SubmissionListing::ListingLoading => {
            translated_messages::findings::listing_loading(message_catalog)
        }
        SubmissionListing::ListingLoaded(listed_submissions) if listed_submissions.is_empty() => {
            translated_messages::findings::nothing_submitted(message_catalog)
        }
        SubmissionListing::ListingLoaded(_) => {
            translated_messages::findings::no_submission_matches_filters(message_catalog)
        }
        SubmissionListing::ListingFailed(history_failure) => {
            history_failure_text(message_catalog, history_failure)
        }
    }
}

pub fn empty_candidates_text(
    message_catalog: &MessageCatalog,
    candidate_listing: &CandidateListing,
) -> String {
    match candidate_listing {
        CandidateListing::CandidatesLoading => {
            translated_messages::findings::candidates_loading(message_catalog)
        }
        CandidateListing::CandidatesLoaded(_) => {
            translated_messages::findings::quarantine_empty_sent(message_catalog)
        }
        CandidateListing::CandidatesFailed(quarantine_failure) => {
            describe_quarantine_failure(message_catalog, quarantine_failure)
        }
    }
}

fn refresh_failure_text(
    message_catalog: &MessageCatalog,
    refresh_failure: &RemoteRefreshFailure,
) -> String {
    match refresh_failure {
        RemoteRefreshFailure::BazaarLookupFailed(bazaar_failure) => {
            describe_bazaar_failure(message_catalog, bazaar_failure)
        }
        RemoteRefreshFailure::VirusTotalLookupFailed(virustotal_failure) => {
            describe_virustotal_failure(message_catalog, virustotal_failure)
        }
        RemoteRefreshFailure::HistoryFailed(history_failure) => {
            history_failure_text(message_catalog, history_failure)
        }
    }
}

pub fn refresh_summary_text(
    message_catalog: &MessageCatalog,
    refresh_summary: &RemoteRefreshSummary,
) -> String {
    let updated_text = match refresh_summary.updated_count {
        0 => translated_messages::findings::no_upload_changed(message_catalog),
        updated_count => {
            translated_messages::findings::count_upload_new_status(message_catalog, updated_count)
        }
    };
    refresh_summary
        .refresh_failures
        .iter()
        .map(|refresh_failure| refresh_failure_text(message_catalog, refresh_failure))
        .fold(updated_text, |summary_text, failure_text| {
            format!("{summary_text}. {failure_text}")
        })
}

#[cfg(test)]
mod submissions_presentation_tests {
    use std::path::PathBuf;

    use bytesize::ByteSize;
    use jiff::Timestamp;

    use super::*;
    use crate::host_services::{
        activity_history::submission_history_store::SubmittedFile,
        clamav_scanner::scanner_output_parser::ThreatIdentity,
        protection_evidence::protection_evidence_fixtures::english_formatters,
        quarantine_vault::quarantine_entry_database::StoredDetectionDate,
        threat_intel::malware_bazaar_client::MalwareBazaarFailure,
    };

    fn record_with(submission_state: SubmissionState) -> SubmissionRecord {
        SubmissionRecord {
            submitted_file: SubmittedFile {
                content_sha256: "275a021bbfb6489e54d471899f7db9d1663fc695ec2fe2a2c4538aabf651fd0f"
                    .to_owned(),
                file_name: "invoice.exe".to_owned(),
            },
            submitted_at: Timestamp::constant(1_793_869_222, 0),
            submission_state,
        }
    }

    #[test]
    fn a_submitted_row_shows_service_kind_hash_date_and_status() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let row_view = submitted_row_view(
            message_catalog,
            &record_with(SubmissionState::ClamavFalsePositive {
                reported_signature: "Win.Trojan.Agent".to_owned(),
                review_status: FalsePositiveReview::FixedInDatabase {
                    database_version: SignatureDatabaseVersion::DailyVersion {
                        version_number: 28_174,
                    },
                },
            }),
            &english_formatters(),
        );
        assert_eq!(row_view.row_title, "invoice.exe");
        assert!(
            row_view
                .row_subtitle
                .starts_with("ClamAV · False positive · 275a021bbfb6 · ")
        );
        assert!(
            row_view
                .row_subtitle
                .ends_with("\nFixed in database version 28174")
        );
        assert!(!row_view.needs_attention);
        assert_eq!(row_view.entry_link, RemoteEntryLink::NoRemoteEntry);
    }

    #[test]
    fn waiting_clamav_reports_mention_the_review_time() {
        let message_catalog = &crate::translated_messages::english_catalog();
        assert!(
            status_text(
                message_catalog,
                &SubmissionState::ClamavMissedMalware {
                    review_status: MissedMalwareReview::AwaitingClamavReview,
                }
            )
            .contains("at least 48 hours")
        );
    }

    #[test]
    fn refused_uploads_need_attention_and_link_nowhere() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let row_view = submitted_row_view(
            message_catalog,
            &record_with(SubmissionState::MalwareBazaarSample {
                bazaar_status: BazaarSampleStatus::SubmissionRefused {
                    refusal_reason: "file_expected".to_owned(),
                },
            }),
            &english_formatters(),
        );
        assert!(row_view.needs_attention);
        assert!(row_view.row_subtitle.contains("refused it: file_expected"));
        assert_eq!(row_view.entry_link, RemoteEntryLink::NoRemoteEntry);
    }

    #[test]
    fn analysed_uploads_show_the_engine_counts_and_link_to_the_report() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let row_view = submitted_row_view(
            message_catalog,
            &record_with(SubmissionState::VirusTotalSample {
                virustotal_status: VirusTotalSampleStatus::VirusTotalAnalysed {
                    analysis_id: "analysis-1".to_owned(),
                    malicious_count: 41,
                    engine_count: 68,
                },
            }),
            &english_formatters(),
        );
        assert!(
            row_view
                .row_subtitle
                .contains("41 of 68 engines flag it as malicious")
        );
        assert!(matches!(
            row_view.entry_link,
            RemoteEntryLink::RemoteEntryAt { entry_url } if entry_url.starts_with("https://www.virustotal.com/gui/file/")
        ));
    }

    #[test]
    fn an_unhashed_candidate_cannot_be_sent() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let candidate_view = candidate_row_view(
            message_catalog,
            &QuarantineEntryRecord {
                entry_id: 5,
                original_path: PathBuf::from("/home/user/old.bin"),
                quarantine_path: PathBuf::from("/vault/old"),
                threat_identity: ThreatIdentity::Signature {
                    signature_name: "Legacy.Test".to_owned(),
                },
                detection_date: StoredDetectionDate::from("2020-01-01T00:00:00Z".to_owned()),
                file_size: ByteSize::b(3),
                payload_hash: RecordedPayloadHash::IntegrityUnknown,
                original_permissions: 0o644,
            },
            &UploadsInFlight::default(),
        );
        assert_eq!(candidate_view.row_title, "old.bin");
        assert_eq!(
            candidate_view.send_availability,
            SendAvailability::NothingToSend
        );
        assert!(candidate_view.row_subtitle.contains("No usable hash"));
    }

    #[test]
    fn a_refresh_summary_lists_what_changed_and_what_failed() {
        let message_catalog = &crate::translated_messages::english_catalog();
        assert_eq!(
            refresh_summary_text(
                message_catalog,
                &RemoteRefreshSummary {
                    updated_count: 2,
                    refresh_failures: vec![RemoteRefreshFailure::BazaarLookupFailed(
                        MalwareBazaarFailure::UnexpectedStatus { status_code: 503 },
                    )],
                }
            ),
            "2 uploads have a new status. MalwareBazaar answered with status 503"
        );
    }
}
