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
        desktop::locale_formatting::{byte_size_text, exact_local_time_text},
        sample_submissions::{
            bazaar_sample_upload::BazaarUploadAttempt, clamav_report_flow::ClamavReportAttempt,
            submission_samples::SampleReadFailure,
            virustotal_sample_upload::VirusTotalUploadAttempt,
        },
        threat_intel::{
            file_reputation_check::{describe_bazaar_failure, describe_virustotal_failure},
            service_key_storage::describe_key_failure,
        },
    },
    shared_widgets::{
        database_migration_failure_text::describe_history_failure,
        quarantine_failure_text::describe_quarantine_failure,
    },
    translated_messages,
};

pub fn sample_read_failure_text(read_failure: &SampleReadFailure) -> String {
    match read_failure {
        SampleReadFailure::QuarantinedCopyUnreadable(quarantine_failure) => {
            describe_quarantine_failure(quarantine_failure)
        }
        SampleReadFailure::FileUnreadable(read_failure) => {
            translated_messages::widgets::file_unreadable(read_failure)
        }
        SampleReadFailure::FileNotRegular => translated_messages::widgets::file_not_regular(),
        SampleReadFailure::FileTooLarge { maximum_size } => {
            translated_messages::widgets::file_too_large(byte_size_text(*maximum_size))
        }
        SampleReadFailure::ReadingInterrupted(_interrupted_task) => {
            translated_messages::widgets::reading_interrupted()
        }
    }
}

pub fn history_failure_text(history_failure: &ActivityHistoryFailure) -> String {
    translated_messages::widgets::history_failure(describe_history_failure(history_failure))
}

pub fn earlier_submission_text(earlier_submission: &SubmissionRecord) -> String {
    translated_messages::widgets::earlier_submission(exact_local_time_text(
        earlier_submission.submitted_at,
    ))
}

pub fn clamav_report_attempt_text(report_attempt: &ClamavReportAttempt) -> String {
    match report_attempt {
        ClamavReportAttempt::ReportRecorded(_recorded_submission) => {
            translated_messages::widgets::report_recorded()
        }
        ClamavReportAttempt::AlreadyReported(earlier_submission) => {
            earlier_submission_text(earlier_submission)
        }
        ClamavReportAttempt::SampleUnreadable(read_failure) => {
            translated_messages::widgets::report_not_sent_error(sample_read_failure_text(
                read_failure,
            ))
        }
        ClamavReportAttempt::ReportNotSent(clamsubmit_failure) => {
            clamsubmit_failure_text(clamsubmit_failure)
        }
        ClamavReportAttempt::HistoryUnavailable(history_failure) => {
            translated_messages::widgets::report_not_sent_error(history_failure_text(
                history_failure,
            ))
        }
        ClamavReportAttempt::ReportSentWithoutRecord(history_failure) => {
            translated_messages::widgets::report_sent_without_record(describe_history_failure(
                history_failure,
            ))
        }
    }
}

fn upload_not_sent(failure_reason: &str) -> String {
    translated_messages::widgets::nothing_uploaded_error(failure_reason)
}

pub fn bazaar_upload_attempt_text(upload_attempt: &BazaarUploadAttempt) -> String {
    match upload_attempt {
        BazaarUploadAttempt::SubmissionRecorded(SubmissionRecord {
            submission_state:
                SubmissionState::MalwareBazaarSample {
                    bazaar_status: BazaarSampleStatus::SubmissionRefused { refusal_reason },
                },
            ..
        }) => translated_messages::widgets::malwarebazaar_refused_sample_reason(
            sanitize_scanner_line(refusal_reason),
        ),
        BazaarUploadAttempt::SubmissionRecorded(_recorded_submission) => {
            translated_messages::widgets::malwarebazaar_sample_sent_samples()
        }
        BazaarUploadAttempt::AlreadySubmitted(earlier_submission) => {
            earlier_submission_text(earlier_submission)
        }
        BazaarUploadAttempt::KeyMissing => {
            upload_not_sent(&translated_messages::widgets::add_abuse_ch_key())
        }
        BazaarUploadAttempt::KeyUnreadable(key_failure) => {
            upload_not_sent(&describe_key_failure(key_failure))
        }
        BazaarUploadAttempt::SampleUnreadable(read_failure) => {
            upload_not_sent(&sample_read_failure_text(read_failure))
        }
        BazaarUploadAttempt::NotDetectedByClamav => {
            upload_not_sent(&translated_messages::widgets::not_detected_clamav())
        }
        BazaarUploadAttempt::UnwantedProgramOnly => {
            upload_not_sent(&translated_messages::widgets::unwanted_program_only())
        }
        BazaarUploadAttempt::BazaarUnavailable(bazaar_failure) => {
            upload_not_sent(&describe_bazaar_failure(bazaar_failure))
        }
        BazaarUploadAttempt::HistoryUnavailable(history_failure) => {
            upload_not_sent(&history_failure_text(history_failure))
        }
        BazaarUploadAttempt::SentWithoutRecord(history_failure) => {
            translated_messages::widgets::sent_without_record(describe_history_failure(
                history_failure,
            ))
        }
    }
}

pub fn virustotal_upload_attempt_text(upload_attempt: &VirusTotalUploadAttempt) -> String {
    match upload_attempt {
        VirusTotalUploadAttempt::SubmissionRecorded(SubmissionRecord {
            submission_state:
                SubmissionState::VirusTotalSample {
                    virustotal_status: VirusTotalSampleStatus::SubmissionRefused { refusal_reason },
                },
            ..
        }) => translated_messages::widgets::virustotal_refused_file_reason(sanitize_scanner_line(
            refusal_reason,
        )),
        VirusTotalUploadAttempt::SubmissionRecorded(_recorded_submission) => {
            translated_messages::widgets::virustotal_analysing_file_refresh()
        }
        VirusTotalUploadAttempt::AlreadySubmitted(earlier_submission) => {
            earlier_submission_text(earlier_submission)
        }
        VirusTotalUploadAttempt::ReportAlreadyAvailable(file_report) => {
            translated_messages::widgets::report_already_available(
                file_report.total_engine_count,
                file_report.detecting_engine_count,
            )
        }
        VirusTotalUploadAttempt::KeyMissing => {
            upload_not_sent(&translated_messages::widgets::add_virustotal_key_preferences())
        }
        VirusTotalUploadAttempt::KeyUnreadable(key_failure) => {
            upload_not_sent(&describe_key_failure(key_failure))
        }
        VirusTotalUploadAttempt::SampleUnreadable(read_failure) => {
            upload_not_sent(&sample_read_failure_text(read_failure))
        }
        VirusTotalUploadAttempt::VirusTotalUnavailable(virustotal_failure) => {
            upload_not_sent(&describe_virustotal_failure(virustotal_failure))
        }
        VirusTotalUploadAttempt::HistoryUnavailable(history_failure) => {
            upload_not_sent(&history_failure_text(history_failure))
        }
        VirusTotalUploadAttempt::SentWithoutRecord(history_failure) => {
            translated_messages::widgets::sent_without_record(describe_history_failure(
                history_failure,
            ))
        }
    }
}
