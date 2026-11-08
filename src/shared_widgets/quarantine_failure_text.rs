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
            translated_messages::widgets::file_hash_mismatch_corrupted()
        }
        QuarantineMoveFailure::RestoreRefusedIntegrityUnknown => {
            translated_messages::widgets::quarantine_record_verified()
        }
        QuarantineMoveFailure::UploadRefusedIntegrityUnknown => {
            translated_messages::widgets::quarantine_record_lazaret()
        }
        QuarantineMoveFailure::PayloadTooLargeToSend { maximum_size } => {
            translated_messages::widgets::file_larger_size_quarantine(byte_size_text(*maximum_size))
        }
        QuarantineMoveFailure::BlockedFileOnOtherFilesystem => {
            translated_messages::widgets::access_scanning_blocks_file()
        }
        QuarantineMoveFailure::EntryNotFound { entry_id } => {
            translated_messages::widgets::quarantine_entry_not_found(entry_id)
        }
        QuarantineMoveFailure::RecordingFailedFileRestored { .. } => {
            translated_messages::widgets::failed_record_restored()
        }
        QuarantineMoveFailure::RecordingFailedRollbackFailed {
            rollback_failure,
            payload_path,
            ..
        } => translated_messages::widgets::failed_record_rollback(
            rollback_failure,
            sanitize_scanner_line(&payload_path.to_string_lossy()),
        ),
        QuarantineMoveFailure::EntryRecordNotRemoved => {
            translated_messages::widgets::file_moved_quarantine_record()
        }
        QuarantineMoveFailure::SystemCallFailed(system_call_failure) => {
            translated_messages::widgets::system_call_failed_error(system_call_failure)
        }
        QuarantineMoveFailure::FileOperationFailed(file_failure) => {
            translated_messages::widgets::file_operation_failed_error(file_failure)
        }
        QuarantineMoveFailure::DatabaseOperationFailed(database_failure) => {
            translated_messages::widgets::quarantine_database_operation_failed(database_failure)
        }
        QuarantineMoveFailure::BlockingStepInterrupted(interrupted_step) => {
            translated_messages::widgets::background_quarantine_step_stopped(interrupted_step)
        }
        QuarantineMoveFailure::PathContainsNul => {
            translated_messages::widgets::path_contains_nul_byte()
        }
        QuarantineMoveFailure::PathHasNoParent => {
            translated_messages::widgets::path_no_parent_folder()
        }
        QuarantineMoveFailure::PathHasNoFileName => {
            translated_messages::widgets::path_no_file_name()
        }
        QuarantineMoveFailure::DatabasePathNotUtf8 => {
            translated_messages::widgets::quarantine_database_not_valid()
        }
        QuarantineMoveFailure::DatabaseNotRegularFile => {
            translated_messages::widgets::quarantine_database_not_regular()
        }
        QuarantineMoveFailure::SourceBlockedByOnAccess => {
            translated_messages::widgets::access_scanning_blocks_reading()
        }
        QuarantineMoveFailure::MovedPayloadStillBlocked => {
            translated_messages::widgets::blocked_file_still_unreadable()
        }
        QuarantineMoveFailure::SourceNotRegularFile => {
            translated_messages::widgets::file_not_regular()
        }
        QuarantineMoveFailure::PayloadNotRegularFile => {
            translated_messages::widgets::quarantined_file_not_regular()
        }
        QuarantineMoveFailure::NotEnoughFreeSpace => {
            translated_messages::widgets::not_enough_free_space()
        }
        QuarantineMoveFailure::FileReplacedDuringOperation => {
            translated_messages::widgets::file_replaced_not_removed()
        }
        QuarantineMoveFailure::PathOutsideQuarantine => {
            translated_messages::widgets::path_not_inside_quarantine()
        }
        QuarantineMoveFailure::InvalidRestorePath => {
            translated_messages::widgets::restore_path_not_valid()
        }
        QuarantineMoveFailure::ProtectedRestoreDestination => {
            translated_messages::widgets::restoring_protected_system_folder()
        }
        QuarantineMoveFailure::StoredPathUnreadable { .. } => {
            translated_messages::widgets::path_stored_quarantine_database()
        }
        QuarantineMoveFailure::VaultSchemaUnusable(migration_failure) => {
            describe_migration_failure(migration_failure)
        }
    }
}
