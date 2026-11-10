use crate::{
    host_services::{
        clamav_programs::database_directory_status::DatabaseFolderReadFailure,
        systemd_journal::unit_journal_reader::UnitJournalReadFailure,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

pub fn unit_journal_failure_text(
    message_catalog: &MessageCatalog,
    read_failure: UnitJournalReadFailure,
) -> String {
    match read_failure {
        UnitJournalReadFailure::JournalReadFailed { failure_kind } => {
            translated_messages::widgets::journal_read_failed(message_catalog, failure_kind)
        }
        UnitJournalReadFailure::ReadingTaskInterrupted => {
            translated_messages::widgets::journal_reading_incomplete(message_catalog)
        }
    }
}

pub fn database_folder_failure_text(
    message_catalog: &MessageCatalog,
    read_failure: DatabaseFolderReadFailure,
) -> String {
    match read_failure {
        DatabaseFolderReadFailure::FolderUnreadable { failure_kind } => {
            translated_messages::widgets::folder_unreadable(message_catalog, failure_kind)
        }
        DatabaseFolderReadFailure::ReadingTaskInterrupted => {
            translated_messages::widgets::reading_signature_folder_stopped(message_catalog)
        }
    }
}

#[cfg(test)]
mod host_read_failure_text_tests {
    use std::io;

    use super::*;

    #[test]
    fn read_failures_name_what_could_not_be_read() {
        let message_catalog = &crate::translated_messages::english_catalog();
        assert_eq!(
            unit_journal_failure_text(
                message_catalog,
                UnitJournalReadFailure::JournalReadFailed {
                    failure_kind: io::ErrorKind::PermissionDenied,
                }
            ),
            "The system journal could not be read: permission denied"
        );
        assert_eq!(
            database_folder_failure_text(
                message_catalog,
                DatabaseFolderReadFailure::FolderUnreadable {
                    failure_kind: io::ErrorKind::PermissionDenied,
                }
            ),
            "The signature folder could not be read: permission denied"
        );
    }
}
