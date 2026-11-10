use crate::{
    host_services::clamav_scanner::{
        scanner_output_parser::sanitize_scanner_line, scanner_process_events::ScanStartFailure,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

pub fn describe_start_failure(
    message_catalog: &MessageCatalog,
    start_failure: &ScanStartFailure,
) -> String {
    match start_failure {
        ScanStartFailure::ScannerProgramMissing => {
            translated_messages::widgets::scanner_program_missing(message_catalog)
        }
        ScanStartFailure::ScannerProgramReplaceable {
            candidate_path,
            replaceable_path,
        } => translated_messages::widgets::scanner_program_replaceable(
            message_catalog,
            sanitize_scanner_line(&candidate_path.to_string_lossy()),
            sanitize_scanner_line(&replaceable_path.to_string_lossy()),
        ),
        ScanStartFailure::ScannerSpawnFailed(spawn_failure) => {
            translated_messages::widgets::scanner_spawn_failed(message_catalog, spawn_failure)
        }
        ScanStartFailure::ScannerWorkDirectoryUnavailable(work_directory_failure) => {
            translated_messages::widgets::scanner_work_directory_unavailable(
                message_catalog,
                work_directory_failure,
            )
        }
        ScanStartFailure::ScannerOutputUncaptured => {
            translated_messages::widgets::scanner_output_uncaptured(message_catalog)
        }
        ScanStartFailure::DetectionSettingsUnreadable(settings_failure) => {
            translated_messages::widgets::detection_settings_unreadable(
                message_catalog,
                settings_failure,
            )
        }
        ScanStartFailure::ProgramLookupFailed(lookup_failure) => {
            translated_messages::widgets::program_lookup_failed(message_catalog, lookup_failure)
        }
        ScanStartFailure::ScanInputUnavailable {
            input_path,
            input_failure,
        } => translated_messages::widgets::scan_input_unavailable(
            message_catalog,
            sanitize_scanner_line(&input_path.to_string_lossy()),
            input_failure,
        ),
    }
}

#[cfg(test)]
mod scanner_start_failure_text_tests {
    use super::*;
    use crate::host_services::{
        clamav_config::detection_options::DetectionSettingsFailure,
        clamav_scanner::scanner_command_builder::ProgramLookupFailure,
    };

    #[tokio::test]
    async fn unreadable_detection_settings_stop_the_scan_with_a_reason() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let abandoned_read = tokio::spawn(std::future::pending::<()>());
        abandoned_read.abort();
        let interrupted_read = abandoned_read.await.expect_err("the read was aborted");
        let start_text = describe_start_failure(
            message_catalog,
            &ScanStartFailure::DetectionSettingsUnreadable(
                DetectionSettingsFailure::ReadingTaskInterrupted(interrupted_read),
            ),
        );
        assert!(start_text.contains("detection settings could not be read"));
    }

    #[tokio::test]
    async fn an_interrupted_program_lookup_stops_the_scan_with_a_reason() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let abandoned_lookup = tokio::spawn(std::future::pending::<()>());
        abandoned_lookup.abort();
        let interrupted_lookup = abandoned_lookup.await.expect_err("the lookup was aborted");
        let start_text = describe_start_failure(
            message_catalog,
            &ScanStartFailure::ProgramLookupFailed(ProgramLookupFailure::LookupInterrupted(
                interrupted_lookup,
            )),
        );
        assert!(start_text.contains("scanner programs could not be looked up"));
    }
}
