use std::{ffi::OsStr, fs, io, os::unix::fs::MetadataExt, path::Path};

use bytesize::ByteSize;
use jiff::Timestamp;
use rustix::process::geteuid;

use crate::host_services::{
    clamav_scanner::scanner_output_parser::sanitize_scanner_line,
    quarantine_vault::quarantine_entry_database::{QuarantineEntryRecord, StoredDetectionDate},
};

const SYSTEM_ADMINISTRATOR_ID: u32 = 0;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OwnerRelation {
    ViewingUser,
    SystemAdministrator,
    AnotherUser,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModificationMoment {
    ModifiedAt(Timestamp),
    OutsideCalendarRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveFileFacts {
    pub owner_account: AccountIdentity,
    pub owner_relation: OwnerRelation,
    pub group_account: AccountIdentity,
    pub permission_mode: u32,
    pub byte_size: ByteSize,
    pub modification_moment: ModificationMoment,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveFileInspection {
    Present(LiveFileFacts),
    NoLongerThere,
    Unreadable,
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

impl From<LiveFileInspection> for FileFactsSource {
    fn from(live_inspection: LiveFileInspection) -> Self {
        match live_inspection {
            LiveFileInspection::Present(live_facts) => Self::LiveSnapshot(live_facts),
            LiveFileInspection::NoLongerThere => Self::NoLongerThere,
            LiveFileInspection::Unreadable => Self::Unreadable,
        }
    }
}

fn named_account(numeric_id: u32, account_name: &OsStr) -> AccountIdentity {
    AccountIdentity::NamedAccount {
        account_name: sanitize_scanner_line(&account_name.to_string_lossy()),
        numeric_id,
    }
}

fn looked_up_owner(owner_id: u32) -> AccountIdentity {
    match uzers::get_user_by_uid(owner_id) {
        Some(owner_user) => named_account(owner_id, owner_user.name()),
        None => AccountIdentity::NumberedAccount {
            numeric_id: owner_id,
        },
    }
}

fn looked_up_group(group_id: u32) -> AccountIdentity {
    match uzers::get_group_by_gid(group_id) {
        Some(owner_group) => named_account(group_id, owner_group.name()),
        None => AccountIdentity::NumberedAccount {
            numeric_id: group_id,
        },
    }
}

const fn owner_relation(owner_id: u32, viewing_user_id: u32) -> OwnerRelation {
    if owner_id == viewing_user_id {
        OwnerRelation::ViewingUser
    } else if owner_id == SYSTEM_ADMINISTRATOR_ID {
        OwnerRelation::SystemAdministrator
    } else {
        OwnerRelation::AnotherUser
    }
}

fn modification_moment(file_metadata: &fs::Metadata) -> ModificationMoment {
    let Ok(modified_nanoseconds) = i32::try_from(file_metadata.mtime_nsec()) else {
        return ModificationMoment::OutsideCalendarRange;
    };
    match Timestamp::new(file_metadata.mtime(), modified_nanoseconds) {
        Ok(modified_at) => ModificationMoment::ModifiedAt(modified_at),
        Err(_out_of_range) => ModificationMoment::OutsideCalendarRange,
    }
}

pub fn read_live_file_facts(file_path: &Path, viewing_user_id: u32) -> LiveFileInspection {
    let file_metadata = match fs::symlink_metadata(file_path) {
        Ok(file_metadata) => file_metadata,
        Err(metadata_failure) if metadata_failure.kind() == io::ErrorKind::NotFound => {
            return LiveFileInspection::NoLongerThere;
        }
        Err(_other_failure) => return LiveFileInspection::Unreadable,
    };
    LiveFileInspection::Present(LiveFileFacts {
        owner_account: looked_up_owner(file_metadata.uid()),
        owner_relation: owner_relation(file_metadata.uid(), viewing_user_id),
        group_account: looked_up_group(file_metadata.gid()),
        permission_mode: file_metadata.mode(),
        byte_size: ByteSize::b(file_metadata.size()),
        modification_moment: modification_moment(&file_metadata),
    })
}

pub fn inspect_live_file_facts(file_path: &Path) -> FileFactsSource {
    FileFactsSource::from(read_live_file_facts(file_path, geteuid().as_raw()))
}

#[cfg(test)]
pub mod live_facts_fixtures {
    use super::*;

    pub fn alice_live_facts(permission_mode: u32) -> LiveFileFacts {
        LiveFileFacts {
            owner_account: AccountIdentity::NamedAccount {
                account_name: "alice".to_owned(),
                numeric_id: 1000,
            },
            owner_relation: OwnerRelation::ViewingUser,
            group_account: AccountIdentity::NamedAccount {
                account_name: "users".to_owned(),
                numeric_id: 100,
            },
            permission_mode,
            byte_size: ByteSize::b(2048),
            modification_moment: ModificationMoment::ModifiedAt(Timestamp::constant(
                1_793_869_222,
                0,
            )),
        }
    }
}

#[cfg(test)]
mod detected_file_facts_tests {
    use super::*;

    #[test]
    fn a_missing_path_is_reported_as_gone() {
        assert_eq!(
            read_live_file_facts(Path::new("/nonexistent/lazaret/detected.bin"), 1000),
            LiveFileInspection::NoLongerThere
        );
    }
}
