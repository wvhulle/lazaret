use crate::{
    host_services::{
        activity_history::{
            activity_history_database::ActivityHistoryFailure,
            submission_history_store::{
                BazaarSampleStatus, SubmissionRecord, SubmissionState, VirusTotalSampleStatus,
            },
        },
        clamav_programs::clamav_report_submission::clamsubmit_failure_text,
        clamav_scanner::scanner_output_parser::sanitize_scanner_line,
        desktop::locale_formatting::{MomentFormatters, byte_size_text},
        sample_submissions::{
            bazaar_sample_upload::BazaarUploadAttempt, clamav_report_flow::ClamavReportAttempt,
            submission_samples::SampleReadFailure,
            virustotal_sample_upload::VirusTotalUploadAttempt,
        },
        threat_intel::file_reputation_check::{
            describe_bazaar_failure, describe_virustotal_failure,
        },
    },
    shared_widgets::{
        database_migration_failure_text::describe_history_failure,
        quarantine_failure_text::describe_quarantine_failure,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

pub fn sample_read_failure_text(
    message_catalog: &MessageCatalog,
    read_failure: &SampleReadFailure,
) -> String {
    match read_failure {
        SampleReadFailure::QuarantinedCopyUnreadable(quarantine_failure) => {
            describe_quarantine_failure(message_catalog, quarantine_failure)
        }
        SampleReadFailure::FileUnreadable(read_failure) => {
            translated_messages::widgets::file_unreadable(message_catalog, read_failure)
        }
        SampleReadFailure::FileNotRegular => {
            translated_messages::widgets::file_not_regular(message_catalog)
        }
        SampleReadFailure::FileTooLarge { maximum_size } => {
            translated_messages::widgets::file_too_large(
                message_catalog,
                byte_size_text(*maximum_size),
            )
        }
        SampleReadFailure::ReadingInterrupted(_interrupted_task) => {
            translated_messages::widgets::reading_interrupted(message_catalog)
        }
    }
}

pub fn history_failure_text(
    message_catalog: &MessageCatalog,
    history_failure: &ActivityHistoryFailure,
) -> String {
    translated_messages::widgets::history_failure(
        message_catalog,
        describe_history_failure(message_catalog, history_failure),
    )
}

pub fn earlier_submission_text(
    message_catalog: &MessageCatalog,
    moment_formatters: &MomentFormatters,
    earlier_submission: &SubmissionRecord,
) -> String {
    translated_messages::widgets::earlier_submission(
        message_catalog,
        moment_formatters.exact_moment_text(earlier_submission.submitted_at),
    )
}

pub fn clamav_report_attempt_text(
    message_catalog: &MessageCatalog,
    moment_formatters: &MomentFormatters,
    report_attempt: &ClamavReportAttempt,
) -> String {
    match report_attempt {
        ClamavReportAttempt::ReportRecorded(_recorded_submission) => {
            translated_messages::widgets::report_recorded(message_catalog)
        }
        ClamavReportAttempt::AlreadyReported(earlier_submission) => {
            earlier_submission_text(message_catalog, moment_formatters, earlier_submission)
        }
        ClamavReportAttempt::SampleUnreadable(read_failure) => {
            translated_messages::widgets::report_not_sent_error(
                message_catalog,
                sample_read_failure_text(message_catalog, read_failure),
            )
        }
        ClamavReportAttempt::ReportNotSent(clamsubmit_failure) => {
            clamsubmit_failure_text(message_catalog, clamsubmit_failure)
        }
        ClamavReportAttempt::HistoryUnavailable(history_failure) => {
            translated_messages::widgets::report_not_sent_error(
                message_catalog,
                history_failure_text(message_catalog, history_failure),
            )
        }
        ClamavReportAttempt::ReportSentWithoutRecord(history_failure) => {
            translated_messages::widgets::report_sent_without_record(
                message_catalog,
                describe_history_failure(message_catalog, history_failure),
            )
        }
    }
}

fn upload_not_sent(message_catalog: &MessageCatalog, failure_reason: &str) -> String {
    translated_messages::widgets::nothing_uploaded_error(message_catalog, failure_reason)
}

pub fn bazaar_upload_attempt_text(
    message_catalog: &MessageCatalog,
    moment_formatters: &MomentFormatters,
    upload_attempt: &BazaarUploadAttempt,
) -> String {
    match upload_attempt {
        BazaarUploadAttempt::SubmissionRecorded(SubmissionRecord {
            submission_state:
                SubmissionState::MalwareBazaarSample {
                    bazaar_status: BazaarSampleStatus::SubmissionRefused { refusal_reason },
                },
            ..
        }) => translated_messages::widgets::malwarebazaar_refused_sample_reason(
            message_catalog,
            sanitize_scanner_line(refusal_reason),
        ),
        BazaarUploadAttempt::SubmissionRecorded(_recorded_submission) => {
            translated_messages::widgets::malwarebazaar_sample_sent_samples(message_catalog)
        }
        BazaarUploadAttempt::AlreadySubmitted(earlier_submission) => {
            earlier_submission_text(message_catalog, moment_formatters, earlier_submission)
        }
        BazaarUploadAttempt::KeyNotUsable => upload_not_sent(
            message_catalog,
            &translated_messages::widgets::bazaar_key_not_usable(message_catalog),
        ),
        BazaarUploadAttempt::SampleUnreadable(read_failure) => upload_not_sent(
            message_catalog,
            &sample_read_failure_text(message_catalog, read_failure),
        ),
        BazaarUploadAttempt::NotDetectedByClamav => upload_not_sent(
            message_catalog,
            &translated_messages::widgets::not_detected_clamav(message_catalog),
        ),
        BazaarUploadAttempt::UnwantedProgramOnly => upload_not_sent(
            message_catalog,
            &translated_messages::widgets::unwanted_program_only(message_catalog),
        ),
        BazaarUploadAttempt::BazaarUnavailable(bazaar_failure) => upload_not_sent(
            message_catalog,
            &describe_bazaar_failure(message_catalog, bazaar_failure),
        ),
        BazaarUploadAttempt::HistoryUnavailable(history_failure) => upload_not_sent(
            message_catalog,
            &history_failure_text(message_catalog, history_failure),
        ),
        BazaarUploadAttempt::SentWithoutRecord(history_failure) => {
            translated_messages::widgets::sent_without_record(
                message_catalog,
                describe_history_failure(message_catalog, history_failure),
            )
        }
    }
}

pub fn virustotal_upload_attempt_text(
    message_catalog: &MessageCatalog,
    moment_formatters: &MomentFormatters,
    upload_attempt: &VirusTotalUploadAttempt,
) -> String {
    match upload_attempt {
        VirusTotalUploadAttempt::SubmissionRecorded(SubmissionRecord {
            submission_state:
                SubmissionState::VirusTotalSample {
                    virustotal_status: VirusTotalSampleStatus::SubmissionRefused { refusal_reason },
                },
            ..
        }) => translated_messages::widgets::virustotal_refused_file_reason(
            message_catalog,
            sanitize_scanner_line(refusal_reason),
        ),
        VirusTotalUploadAttempt::SubmissionRecorded(_recorded_submission) => {
            translated_messages::widgets::virustotal_analysing_file_refresh(message_catalog)
        }
        VirusTotalUploadAttempt::AlreadySubmitted(earlier_submission) => {
            earlier_submission_text(message_catalog, moment_formatters, earlier_submission)
        }
        VirusTotalUploadAttempt::ReportAlreadyAvailable(file_report) => {
            translated_messages::widgets::report_already_available(
                message_catalog,
                file_report.total_engine_count,
                file_report.detecting_engine_count,
            )
        }
        VirusTotalUploadAttempt::KeyNotUsable => upload_not_sent(
            message_catalog,
            &translated_messages::widgets::virustotal_key_not_usable(message_catalog),
        ),
        VirusTotalUploadAttempt::SampleUnreadable(read_failure) => upload_not_sent(
            message_catalog,
            &sample_read_failure_text(message_catalog, read_failure),
        ),
        VirusTotalUploadAttempt::VirusTotalUnavailable(virustotal_failure) => upload_not_sent(
            message_catalog,
            &describe_virustotal_failure(message_catalog, virustotal_failure),
        ),
        VirusTotalUploadAttempt::HistoryUnavailable(history_failure) => upload_not_sent(
            message_catalog,
            &history_failure_text(message_catalog, history_failure),
        ),
        VirusTotalUploadAttempt::SentWithoutRecord(history_failure) => {
            translated_messages::widgets::sent_without_record(
                message_catalog,
                describe_history_failure(message_catalog, history_failure),
            )
        }
    }
}
