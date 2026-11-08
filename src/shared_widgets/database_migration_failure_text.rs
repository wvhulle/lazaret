use crate::{
    host_services::{
        activity_history::activity_history_database::ActivityHistoryFailure,
        lazaret_database::database_migration_runner::DatabaseMigrationFailure,
    },
    translated_messages,
};

pub fn describe_migration_failure(migration_failure: &DatabaseMigrationFailure) -> String {
    match migration_failure {
        DatabaseMigrationFailure::DatabaseFromNewerRelease {
            found_version,
            known_version,
        } => {
            translated_messages::widgets::database_made_newer_lazaret(found_version, known_version)
        }
        DatabaseMigrationFailure::MigrationStepFailed {
            migration_name,
            database_failure,
        } => translated_messages::widgets::updating_database_layout_failed(
            migration_name,
            database_failure,
        ),
        DatabaseMigrationFailure::VersionCheckFailed(database_failure) => {
            translated_messages::widgets::database_layout_not_checked(database_failure)
        }
    }
}

pub fn describe_history_failure(history_failure: &ActivityHistoryFailure) -> String {
    match history_failure {
        ActivityHistoryFailure::HistorySchemaUnusable(migration_failure) => {
            describe_migration_failure(migration_failure)
        }
        ActivityHistoryFailure::DatabaseFailed(_)
        | ActivityHistoryFailure::HistoryFileUnavailable(_)
        | ActivityHistoryFailure::HistoryFileCheckInterrupted(_)
        | ActivityHistoryFailure::DetectedFileCheckInterrupted(_)
        | ActivityHistoryFailure::DatabasePathNotUtf8
        | ActivityHistoryFailure::StoredStatusUnknown
        | ActivityHistoryFailure::StoredOriginUnknown
        | ActivityHistoryFailure::StoredScanDetailsInconsistent
        | ActivityHistoryFailure::StoredDetectionUnreadable
        | ActivityHistoryFailure::StoredSubmissionUnreadable
        | ActivityHistoryFailure::StoredTimeInvalid(_)
        | ActivityHistoryFailure::HistoryLockUnavailable(_) => history_failure.to_string(),
    }
}

#[cfg(test)]
mod database_migration_failure_text_tests {
    use super::*;

    #[test]
    fn a_newer_database_names_both_layout_versions() {
        let newer_database = DatabaseMigrationFailure::DatabaseFromNewerRelease {
            found_version: 4,
            known_version: 1,
        };
        let history_text = describe_history_failure(
            &ActivityHistoryFailure::HistorySchemaUnusable(newer_database),
        );
        assert!(history_text.contains("layout version 4"));
        assert!(history_text.contains("up to 1"));
    }
}
