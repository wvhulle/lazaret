use std::path::Path;

use rustix::process::geteuid;

use crate::{
    host_services::clamav_scanner::scanner_output_parser::is_blocked_by_on_access,
    shared_widgets::detected_file::detected_file_facts::{
        FileFactsSource, LiveFileFacts, LiveFileInspection, OwnerRelation, read_live_file_facts,
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
    pub fn facts_source(&self) -> FileFactsSource {
        match self {
            Self::Present { live_facts, .. } => FileFactsSource::LiveSnapshot(live_facts.clone()),
            Self::NoLongerThere => FileFactsSource::NoLongerThere,
            Self::StateUnknown => FileFactsSource::Unreadable,
        }
    }
}

const fn file_owner(owner_relation: OwnerRelation) -> FileOwner {
    match owner_relation {
        OwnerRelation::ViewingUser => FileOwner::CurrentUser,
        OwnerRelation::SystemAdministrator | OwnerRelation::AnotherUser => FileOwner::OtherUser,
    }
}

pub fn classify_file_state(
    live_inspection: LiveFileInspection,
    file_access: FileAccess,
) -> DetectedFileState {
    match live_inspection {
        LiveFileInspection::NoLongerThere => DetectedFileState::NoLongerThere,
        LiveFileInspection::Unreadable => DetectedFileState::StateUnknown,
        LiveFileInspection::Present(live_facts) => DetectedFileState::Present {
            access: file_access,
            owner: file_owner(live_facts.owner_relation),
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
        read_live_file_facts(file_path, geteuid().as_raw()),
        file_access,
    )
}

#[cfg(test)]
pub mod detected_file_state_fixtures {
    use bytesize::ByteSize;
    use jiff::Timestamp;

    use crate::shared_widgets::detected_file::detected_file_facts::{
        AccountIdentity, LiveFileFacts, ModificationMoment, OwnerRelation,
    };

    pub fn live_facts_owned_by(owner_relation: OwnerRelation) -> LiveFileFacts {
        LiveFileFacts {
            owner_account: AccountIdentity::NumberedAccount { numeric_id: 1000 },
            owner_relation,
            group_account: AccountIdentity::NumberedAccount { numeric_id: 100 },
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

    #[test]
    fn a_blocked_file_keeps_its_owner() {
        let administrator_facts = live_facts_owned_by(OwnerRelation::SystemAdministrator);
        assert_eq!(
            classify_file_state(
                LiveFileInspection::Present(administrator_facts.clone()),
                FileAccess::BlockedOnAccess
            ),
            DetectedFileState::Present {
                access: FileAccess::BlockedOnAccess,
                owner: FileOwner::OtherUser,
                live_facts: administrator_facts,
            }
        );
        assert!(matches!(
            classify_file_state(
                LiveFileInspection::Present(live_facts_owned_by(OwnerRelation::ViewingUser)),
                FileAccess::BlockedOnAccess
            ),
            DetectedFileState::Present {
                owner: FileOwner::CurrentUser,
                ..
            }
        ));
    }

    #[test]
    fn missing_unknown_and_present_files_are_told_apart() {
        assert_eq!(
            classify_file_state(LiveFileInspection::NoLongerThere, FileAccess::Readable),
            DetectedFileState::NoLongerThere
        );
        assert_eq!(
            classify_file_state(LiveFileInspection::Unreadable, FileAccess::Readable),
            DetectedFileState::StateUnknown
        );
        assert_eq!(
            classify_file_state(LiveFileInspection::NoLongerThere, FileAccess::Readable)
                .facts_source(),
            FileFactsSource::NoLongerThere
        );
    }
}
