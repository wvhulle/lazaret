use std::{io, path::Path};

use rustix::process::geteuid;

use crate::{
    host_services::clamav_scanner::scanner_output_parser::is_blocked_by_on_access,
    shared_widgets::detected_file::detected_file_facts::{
        FileFactsSource, LiveFileFacts, live_file_metadata,
    },
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileAccess {
    BlockedOnAccess,
    Readable,
    NotOpened,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileOwner {
    CurrentUser,
    OtherUser,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DetectedFileState {
    Present {
        access: FileAccess,
        owner: FileOwner,
        live_facts: LiveFileFacts,
    },
    NoLongerThere,
    StateUnknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpeningCheck {
    OpenToCheckBlocking,
    InspectWithoutOpening,
}

impl DetectedFileState {
    pub const fn file_is_present(&self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub const fn user_can_move_or_delete(&self) -> bool {
        matches!(
            self,
            Self::Present {
                owner: FileOwner::CurrentUser,
                ..
            }
        )
    }

    pub fn facts_source(&self) -> FileFactsSource {
        match self {
            Self::Present { live_facts, .. } => FileFactsSource::LiveSnapshot(live_facts.clone()),
            Self::NoLongerThere => FileFactsSource::NoLongerThere,
            Self::StateUnknown => FileFactsSource::Unreadable,
        }
    }
}

pub fn classify_file_state(
    metadata_outcome: Result<LiveFileFacts, io::ErrorKind>,
    current_user_id: u32,
    file_access: FileAccess,
) -> DetectedFileState {
    match metadata_outcome {
        Err(io::ErrorKind::NotFound) => DetectedFileState::NoLongerThere,
        Err(_other_failure) => DetectedFileState::StateUnknown,
        Ok(live_facts) => DetectedFileState::Present {
            access: file_access,
            owner: if live_facts.owner_account.numeric_id() == current_user_id {
                FileOwner::CurrentUser
            } else {
                FileOwner::OtherUser
            },
            live_facts,
        },
    }
}

pub fn inspect_detected_file(file_path: &Path, opening_check: OpeningCheck) -> DetectedFileState {
    let file_access = match opening_check {
        OpeningCheck::InspectWithoutOpening => FileAccess::NotOpened,
        OpeningCheck::OpenToCheckBlocking if is_blocked_by_on_access(file_path) => {
            FileAccess::BlockedOnAccess
        }
        OpeningCheck::OpenToCheckBlocking => FileAccess::Readable,
    };
    classify_file_state(
        live_file_metadata(file_path).map(|file_metadata| LiveFileFacts::from(&file_metadata)),
        geteuid().as_raw(),
        file_access,
    )
}

#[cfg(test)]
pub mod detected_file_state_fixtures {
    use bytesize::ByteSize;
    use jiff::Timestamp;

    use crate::shared_widgets::detected_file::detected_file_facts::{
        AccountIdentity, LiveFileFacts, ModificationMoment,
    };

    pub fn live_facts_owned_by(owner_id: u32) -> LiveFileFacts {
        LiveFileFacts {
            owner_account: AccountIdentity::NumberedAccount {
                numeric_id: owner_id,
            },
            group_account: AccountIdentity::NumberedAccount {
                numeric_id: owner_id,
            },
            permission_mode: 0o100_644,
            byte_size: ByteSize::b(68),
            modification_moment: ModificationMoment::ModifiedAt(Timestamp::constant(
                1_793_869_000,
                0,
            )),
        }
    }
}

#[cfg(test)]
mod detected_file_state_tests {
    use super::{detected_file_state_fixtures::live_facts_owned_by, *};

    const OWN_USER: u32 = 1000;

    #[test]
    fn a_blocked_file_keeps_its_owner() {
        assert_eq!(
            classify_file_state(
                Ok(live_facts_owned_by(0)),
                OWN_USER,
                FileAccess::BlockedOnAccess
            ),
            DetectedFileState::Present {
                access: FileAccess::BlockedOnAccess,
                owner: FileOwner::OtherUser,
                live_facts: live_facts_owned_by(0),
            }
        );
        assert!(
            !classify_file_state(
                Ok(live_facts_owned_by(0)),
                OWN_USER,
                FileAccess::BlockedOnAccess
            )
            .user_can_move_or_delete()
        );
        assert!(
            classify_file_state(
                Ok(live_facts_owned_by(OWN_USER)),
                OWN_USER,
                FileAccess::BlockedOnAccess
            )
            .user_can_move_or_delete()
        );
    }

    #[test]
    fn missing_unknown_and_present_files_are_told_apart() {
        assert_eq!(
            classify_file_state(Err(io::ErrorKind::NotFound), OWN_USER, FileAccess::Readable),
            DetectedFileState::NoLongerThere
        );
        assert_eq!(
            classify_file_state(
                Err(io::ErrorKind::PermissionDenied),
                OWN_USER,
                FileAccess::Readable
            ),
            DetectedFileState::StateUnknown
        );
        assert_eq!(
            classify_file_state(Err(io::ErrorKind::NotFound), OWN_USER, FileAccess::Readable)
                .facts_source(),
            FileFactsSource::NoLongerThere
        );
        assert!(
            !classify_file_state(
                Err(io::ErrorKind::PermissionDenied),
                OWN_USER,
                FileAccess::Readable
            )
            .user_can_move_or_delete()
        );
    }
}
