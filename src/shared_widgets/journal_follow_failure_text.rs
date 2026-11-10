use crate::{
    host_services::detection_journal::detection_journal_follower::JournalFollowFailure,
    translated_messages, translated_messages::MessageCatalog,
};

pub fn journal_follow_failure_text(
    message_catalog: &MessageCatalog,
    follow_failure: &JournalFollowFailure,
) -> String {
    let failure_reason = match follow_failure {
        JournalFollowFailure::JournalReadFailed { read_failure_kind } => {
            read_failure_kind.to_string()
        }
        JournalFollowFailure::EntryTimestampInvalid => {
            translated_messages::widgets::entry_timestamp_invalid(message_catalog)
        }
        JournalFollowFailure::ReaderThreadFailed { spawn_failure_kind } => {
            translated_messages::widgets::reader_thread_failed(message_catalog, spawn_failure_kind)
        }
    };
    translated_messages::widgets::journal_reading_stopped(message_catalog, &failure_reason)
}
