use crate::{
    host_services::{
        clamav_scanner::scanner_output_parser::sanitize_scanner_line,
        desktop::locale_formatting::byte_size_text,
        quarantine_vault::quarantine_file_moves::QuarantineMoveFailure,
    },
    shared_widgets::database_migration_failure_text::describe_migration_failure,
    translated_messages,
};

pub fn describe_quarantine_failure(move_failure: &QuarantineMoveFailure) -> String {
    match move_failure {
        QuarantineMoveFailure::FileHashMismatch => {
            translated_messages::widgets::file_hash_mismatch()
        }
        QuarantineMoveFailure::RestoreRefusedIntegrityUnknown => {
            translated_messages::widgets::restore_refused_integrity_unknown()
        }
        QuarantineMoveFailure::UploadRefusedIntegrityUnknown => {
            translated_messages::widgets::upload_refused_integrity_unknown()
        }
        QuarantineMoveFailure::PayloadTooLargeToSend { maximum_size } => {
            translated_messages::widgets::payload_too_large_send(byte_size_text(*maximum_size))
        }
        QuarantineMoveFailure::BlockedFileOnOtherFilesystem => {
            translated_messages::widgets::blocked_file_other_filesystem()
        }
        QuarantineMoveFailure::EntryNotFound { entry_id } => {
            translated_messages::widgets::entry_not_found(entry_id)
        }
        QuarantineMoveFailure::RecordingFailedFileRestored { .. } => {
            translated_messages::widgets::recording_failed_file_restored()
        }
        QuarantineMoveFailure::RecordingFailedRollbackFailed {
            rollback_failure,
            payload_path,
            ..
        } => translated_messages::widgets::recording_failed_rollback(
            rollback_failure,
            sanitize_scanner_line(&payload_path.to_string_lossy()),
        ),
        QuarantineMoveFailure::EntryRecordNotRemoved => {
            translated_messages::widgets::entry_record_not_removed()
        }
        QuarantineMoveFailure::SystemCallFailed(system_call_failure) => {
            translated_messages::widgets::system_call_failed(system_call_failure)
        }
        QuarantineMoveFailure::FileOperationFailed(file_failure) => {
            translated_messages::widgets::file_operation_failed(file_failure)
        }
        QuarantineMoveFailure::DatabaseOperationFailed(database_failure) => {
            translated_messages::widgets::database_operation_failed(database_failure)
        }
        QuarantineMoveFailure::BlockingStepInterrupted(interrupted_step) => {
            translated_messages::widgets::blocking_step_interrupted(interrupted_step)
        }
        QuarantineMoveFailure::PathContainsNul => translated_messages::widgets::path_contains_nul(),
        QuarantineMoveFailure::PathHasNoParent => {
            translated_messages::widgets::path_has_no_parent()
        }
        QuarantineMoveFailure::PathHasNoFileName => {
            translated_messages::widgets::path_has_no_file()
        }
        QuarantineMoveFailure::DatabasePathNotUtf8 => {
            translated_messages::widgets::database_path_not_utf8()
        }
        QuarantineMoveFailure::DatabaseNotRegularFile => {
            translated_messages::widgets::database_not_regular_file()
        }
        QuarantineMoveFailure::SourceBlockedByOnAccess => {
            translated_messages::widgets::source_blocked_access()
        }
        QuarantineMoveFailure::MovedPayloadStillBlocked => {
            translated_messages::widgets::moved_payload_still_blocked()
        }
        QuarantineMoveFailure::SourceNotRegularFile => {
            translated_messages::widgets::source_not_regular_file()
        }
        QuarantineMoveFailure::PayloadNotRegularFile => {
            translated_messages::widgets::payload_not_regular_file()
        }
        QuarantineMoveFailure::NotEnoughFreeSpace => {
            translated_messages::widgets::not_enough_free_space()
        }
        QuarantineMoveFailure::FileReplacedDuringOperation => {
            translated_messages::widgets::file_replaced_during_operation()
        }
        QuarantineMoveFailure::PathOutsideQuarantine => {
            translated_messages::widgets::path_outside_quarantine()
        }
        QuarantineMoveFailure::InvalidRestorePath => {
            translated_messages::widgets::invalid_restore_path()
        }
        QuarantineMoveFailure::ProtectedRestoreDestination => {
            translated_messages::widgets::protected_restore_destination()
        }
        QuarantineMoveFailure::StoredPathUnreadable { .. } => {
            translated_messages::widgets::stored_path_unreadable()
        }
        QuarantineMoveFailure::VaultSchemaUnusable(migration_failure) => {
            describe_migration_failure(migration_failure)
        }
    }
}
