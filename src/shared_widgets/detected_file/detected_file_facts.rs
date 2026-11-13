use std::{ffi::OsStr, fs, io, os::unix::fs::MetadataExt, path::Path};

use bytesize::ByteSize;
use jiff::Timestamp;
use rustix::fs::FileType;

use crate::{
    host_services::{
        clamav_scanner::scanner_output_parser::sanitize_scanner_line,
        desktop::locale_formatting::{MomentFormatters, byte_size_text},
        quarantine_vault::quarantine_entry_database::{QuarantineEntryRecord, StoredDetectionDate},
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccountIdentity {
    NamedAccount {
        account_name: String,
        numeric_id: u32,
    },
    NumberedAccount {
        numeric_id: u32,
    },
}

impl AccountIdentity {
    pub const fn numeric_id(&self) -> u32 {
        match self {
            Self::NamedAccount { numeric_id, .. } | Self::NumberedAccount { numeric_id } => {
                *numeric_id
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModificationMoment {
    ModifiedAt(Timestamp),
    OutsideCalendarRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveFileFacts {
    pub owner_account: AccountIdentity,
    pub group_account: AccountIdentity,
    pub permission_mode: u32,
    pub byte_size: ByteSize,
    pub modification_moment: ModificationMoment,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordedQuarantineFacts {
    pub permission_mode: u32,
    pub byte_size: ByteSize,
    pub detection_date: StoredDetectionDate,
}

impl From<&QuarantineEntryRecord> for RecordedQuarantineFacts {
    fn from(entry_record: &QuarantineEntryRecord) -> Self {
        Self {
            permission_mode: entry_record.original_permissions,
            byte_size: entry_record.file_size,
            detection_date: entry_record.detection_date.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileFactsSource {
    LiveSnapshot(LiveFileFacts),
    RecordedAtQuarantine(RecordedQuarantineFacts),
    NoLongerThere,
    Unreadable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileFactRow {
    pub fact_label: String,
    pub fact_value: String,
}

fn named_account(numeric_id: u32, account_name: &OsStr) -> AccountIdentity {
    AccountIdentity::NamedAccount {
        account_name: sanitize_scanner_line(&account_name.to_string_lossy()),
        numeric_id,
    }
}

fn modification_moment(file_metadata: &fs::Metadata) -> ModificationMoment {
    i32::try_from(file_metadata.mtime_nsec())
        .ok()
        .and_then(|modified_nanoseconds| {
            Timestamp::new(file_metadata.mtime(), modified_nanoseconds).ok()
        })
        .map_or(
            ModificationMoment::OutsideCalendarRange,
            ModificationMoment::ModifiedAt,
        )
}

impl From<&fs::Metadata> for LiveFileFacts {
    fn from(file_metadata: &fs::Metadata) -> Self {
        let owner_id = file_metadata.uid();
        let group_id = file_metadata.gid();
        Self {
            owner_account: uzers::get_user_by_uid(owner_id).map_or(
                AccountIdentity::NumberedAccount {
                    numeric_id: owner_id,
                },
                |owner_user| named_account(owner_id, owner_user.name()),
            ),
            group_account: uzers::get_group_by_gid(group_id).map_or(
                AccountIdentity::NumberedAccount {
                    numeric_id: group_id,
                },
                |owner_group| named_account(group_id, owner_group.name()),
            ),
            permission_mode: file_metadata.mode(),
            byte_size: ByteSize::b(file_metadata.size()),
            modification_moment: modification_moment(file_metadata),
        }
    }
}

pub fn live_file_metadata(file_path: &Path) -> Result<fs::Metadata, io::ErrorKind> {
    fs::symlink_metadata(file_path).map_err(|metadata_failure| metadata_failure.kind())
}

pub fn inspect_live_file_facts(file_path: &Path) -> FileFactsSource {
    match live_file_metadata(file_path) {
        Ok(file_metadata) => FileFactsSource::LiveSnapshot(LiveFileFacts::from(&file_metadata)),
        Err(io::ErrorKind::NotFound) => FileFactsSource::NoLongerThere,
        Err(_other_failure) => FileFactsSource::Unreadable,
    }
}

const fn file_type_marker(permission_mode: u32) -> char {
    match FileType::from_raw_mode(permission_mode) {
        FileType::Directory => 'd',
        FileType::Symlink => 'l',
        FileType::CharacterDevice => 'c',
        FileType::BlockDevice => 'b',
        FileType::Fifo => 'p',
        FileType::Socket => 's',
        FileType::RegularFile | FileType::Unknown => '-',
    }
}

struct PermissionClass {
    bit_shift: u32,
    special_bit: u32,
    special_with_execute: char,
    special_without_execute: char,
}

const OWNER_CLASS: PermissionClass = PermissionClass {
    bit_shift: 6,
    special_bit: 0o4000,
    special_with_execute: 's',
    special_without_execute: 'S',
};

const GROUP_CLASS: PermissionClass = PermissionClass {
    bit_shift: 3,
    special_bit: 0o2000,
    special_with_execute: 's',
    special_without_execute: 'S',
};

const OTHER_CLASS: PermissionClass = PermissionClass {
    bit_shift: 0,
    special_bit: 0o1000,
    special_with_execute: 't',
    special_without_execute: 'T',
};

const fn permission_marker(
    permission_mode: u32,
    permission_bit: u32,
    granted_marker: char,
) -> char {
    if permission_mode & permission_bit == 0 {
        '-'
    } else {
        granted_marker
    }
}

const fn permission_triplet(permission_mode: u32, permission_class: &PermissionClass) -> [char; 3] {
    let execute_bit = 0o1 << permission_class.bit_shift;
    let execute_marker = match (
        permission_mode & permission_class.special_bit != 0,
        permission_mode & execute_bit != 0,
    ) {
        (true, true) => permission_class.special_with_execute,
        (true, false) => permission_class.special_without_execute,
        (false, true) => 'x',
        (false, false) => '-',
    };
    [
        permission_marker(permission_mode, 0o4 << permission_class.bit_shift, 'r'),
        permission_marker(permission_mode, 0o2 << permission_class.bit_shift, 'w'),
        execute_marker,
    ]
}

pub fn permission_mode_text(permission_mode: u32) -> String {
    let owner_markers = permission_triplet(permission_mode, &OWNER_CLASS);
    let group_markers = permission_triplet(permission_mode, &GROUP_CLASS);
    let other_markers = permission_triplet(permission_mode, &OTHER_CLASS);
    std::iter::once(file_type_marker(permission_mode))
        .chain(owner_markers)
        .chain(group_markers)
        .chain(other_markers)
        .collect()
}

pub fn account_identity_text(account_identity: &AccountIdentity) -> String {
    match account_identity {
        AccountIdentity::NamedAccount {
            account_name,
            numeric_id,
        } => format!("{account_name} ({numeric_id})"),
        AccountIdentity::NumberedAccount { numeric_id } => numeric_id.to_string(),
    }
}

fn fact_row(fact_label: String, fact_value: String) -> FileFactRow {
    FileFactRow {
        fact_label,
        fact_value,
    }
}

fn detection_date_text(
    moment_formatters: &MomentFormatters,
    detection_date: &StoredDetectionDate,
) -> String {
    match detection_date {
        StoredDetectionDate::DetectedAt(detected_at) => {
            moment_formatters.exact_moment_text(*detected_at)
        }
        StoredDetectionDate::UnreadableText(stored_text) => sanitize_scanner_line(stored_text),
    }
}

pub fn file_fact_rows(
    message_catalog: &MessageCatalog,
    moment_formatters: &MomentFormatters,
    facts_source: &FileFactsSource,
) -> Vec<FileFactRow> {
    match facts_source {
        FileFactsSource::LiveSnapshot(live_facts) => vec![
            fact_row(
                translated_messages::findings::file_owner(message_catalog),
                account_identity_text(&live_facts.owner_account),
            ),
            fact_row(
                translated_messages::findings::file_group(message_catalog),
                account_identity_text(&live_facts.group_account),
            ),
            fact_row(
                translated_messages::findings::file_permissions(message_catalog),
                permission_mode_text(live_facts.permission_mode),
            ),
            fact_row(
                translated_messages::findings::file_size(message_catalog),
                byte_size_text(live_facts.byte_size),
            ),
            fact_row(
                translated_messages::findings::file_modified(message_catalog),
                match live_facts.modification_moment {
                    ModificationMoment::ModifiedAt(modified_at) => {
                        moment_formatters.exact_moment_text(modified_at)
                    }
                    ModificationMoment::OutsideCalendarRange => {
                        translated_messages::findings::unknown_label(message_catalog)
                    }
                },
            ),
        ],
        FileFactsSource::RecordedAtQuarantine(recorded_facts) => vec![
            fact_row(
                translated_messages::findings::detected_label(message_catalog),
                detection_date_text(moment_formatters, &recorded_facts.detection_date),
            ),
            fact_row(
                translated_messages::findings::file_permissions(message_catalog),
                permission_mode_text(recorded_facts.permission_mode),
            ),
            fact_row(
                translated_messages::findings::file_size(message_catalog),
                byte_size_text(recorded_facts.byte_size),
            ),
        ],
        FileFactsSource::NoLongerThere => vec![fact_row(
            translated_messages::findings::file_status(message_catalog),
            translated_messages::findings::no_longer_there_detection(message_catalog),
        )],
        FileFactsSource::Unreadable => vec![fact_row(
            translated_messages::findings::file_status(message_catalog),
            translated_messages::findings::state_unknown_detection(message_catalog),
        )],
    }
}

pub fn facts_source_description(
    message_catalog: &MessageCatalog,
    facts_source: &FileFactsSource,
) -> String {
    match facts_source {
        FileFactsSource::LiveSnapshot(_)
        | FileFactsSource::NoLongerThere
        | FileFactsSource::Unreadable => {
            translated_messages::findings::file_facts_live(message_catalog)
        }
        FileFactsSource::RecordedAtQuarantine(_) => {
            translated_messages::findings::file_facts_recorded(message_catalog)
        }
    }
}

#[cfg(test)]
mod detected_file_facts_tests {
    use super::*;
    use crate::host_services::protection_evidence::protection_evidence_fixtures::english_formatters;

    fn alice_live_facts() -> LiveFileFacts {
        LiveFileFacts {
            owner_account: AccountIdentity::NamedAccount {
                account_name: "alice".to_owned(),
                numeric_id: 1000,
            },
            group_account: AccountIdentity::NumberedAccount { numeric_id: 4242 },
            permission_mode: 0o100_644,
            byte_size: ByteSize::b(2048),
            modification_moment: ModificationMoment::ModifiedAt(Timestamp::constant(
                1_793_869_222,
                0,
            )),
        }
    }

    fn row_labels_and_values(fact_rows: Vec<FileFactRow>) -> Vec<(String, String)> {
        fact_rows
            .into_iter()
            .map(|fact_row| (fact_row.fact_label, fact_row.fact_value))
            .collect()
    }

    #[test]
    fn permission_modes_read_like_ls() {
        assert_eq!(permission_mode_text(0o100_644), "-rw-r--r--");
        assert_eq!(permission_mode_text(0o040_755), "drwxr-xr-x");
        assert_eq!(permission_mode_text(0o120_777), "lrwxrwxrwx");
        assert_eq!(permission_mode_text(0o104_755), "-rwsr-xr-x");
        assert_eq!(permission_mode_text(0o041_777), "drwxrwxrwt");
        assert_eq!(permission_mode_text(0o102_640), "-rw-r-S---");
        assert_eq!(permission_mode_text(0o644), "-rw-r--r--");
    }

    #[test]
    fn accounts_show_their_name_and_number_or_only_the_number() {
        let live_facts = alice_live_facts();
        assert_eq!(
            account_identity_text(&live_facts.owner_account),
            "alice (1000)"
        );
        assert_eq!(account_identity_text(&live_facts.group_account), "4242");
    }

    #[test]
    fn a_live_file_lists_owner_group_mode_size_and_modification() {
        let message_catalog = translated_messages::english_catalog();
        let fact_rows = file_fact_rows(
            &message_catalog,
            &english_formatters(),
            &FileFactsSource::LiveSnapshot(alice_live_facts()),
        );
        let labels_and_values = row_labels_and_values(fact_rows);
        assert_eq!(
            labels_and_values
                .iter()
                .map(|(fact_label, _fact_value)| fact_label.as_str())
                .collect::<Vec<_>>(),
            ["Owner", "Group", "Permissions", "Size", "Modified"]
        );
        assert_eq!(labels_and_values[0].1, "alice (1000)");
        assert_eq!(labels_and_values[2].1, "-rw-r--r--");
        assert_eq!(labels_and_values[3].1, "2.0 KiB");
    }

    #[test]
    fn a_quarantined_file_lists_only_what_was_recorded() {
        let message_catalog = translated_messages::english_catalog();
        let fact_rows = file_fact_rows(
            &message_catalog,
            &english_formatters(),
            &FileFactsSource::RecordedAtQuarantine(RecordedQuarantineFacts {
                permission_mode: 0o600,
                byte_size: ByteSize::b(68),
                detection_date: StoredDetectionDate::UnreadableText("yesterday".to_owned()),
            }),
        );
        assert_eq!(
            row_labels_and_values(fact_rows),
            [
                ("Detected".to_owned(), "yesterday".to_owned()),
                ("Permissions".to_owned(), "-rw-------".to_owned()),
                ("Size".to_owned(), "68 B".to_owned()),
            ]
        );
    }

    #[test]
    fn missing_and_unreadable_files_show_only_their_status() {
        let message_catalog = translated_messages::english_catalog();
        let formatters = english_formatters();
        assert_eq!(
            row_labels_and_values(file_fact_rows(
                &message_catalog,
                &formatters,
                &FileFactsSource::NoLongerThere
            )),
            [("Status".to_owned(), "No longer there".to_owned())]
        );
        assert_eq!(
            file_fact_rows(&message_catalog, &formatters, &FileFactsSource::Unreadable).len(),
            1
        );
    }

    #[test]
    fn a_missing_path_is_reported_as_gone() {
        assert_eq!(
            inspect_live_file_facts(Path::new("/nonexistent/lazaret/detected.bin")),
            FileFactsSource::NoLongerThere
        );
    }
}
