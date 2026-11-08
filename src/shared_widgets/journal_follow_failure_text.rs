use crate::{
    host_services::detection_journal::detection_journal_follower::JournalFollowFailure,
    translated_messages,
};

pub fn journal_follow_failure_text(follow_failure: &JournalFollowFailure) -> String {
    let failure_reason = match follow_failure {
        JournalFollowFailure::JournalReadFailed { read_failure_kind } => {
            read_failure_kind.to_string()
        }
        JournalFollowFailure::EntryTimestampInvalid => {
            translated_messages::widgets::journal_entry_timestamp_outside()
        }
        JournalFollowFailure::ReaderThreadFailed { spawn_failure_kind } => {
            translated_messages::widgets::journal_reader_not_start(spawn_failure_kind)
        }
    };
    translated_messages::widgets::journal_reading_stopped(&failure_reason)
}
