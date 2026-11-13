use crate::{
    shared_widgets::detected_file::{
        detected_file_facts::{AccountIdentity, LiveFileFacts, OwnerRelation},
        file_access_summary::{AccessAudience, ProgramRunning},
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OwnerMention {
    NamedOwner {
        owner_text: String,
        group_text: String,
    },
    UnrecordedOwner,
}

pub fn user_account_text(
    message_catalog: &MessageCatalog,
    user_account: &AccountIdentity,
) -> String {
    match user_account {
        AccountIdentity::NamedAccount { account_name, .. } => account_name.clone(),
        AccountIdentity::NumberedAccount { numeric_id } => {
            translated_messages::findings::numbered_user(message_catalog, numeric_id.to_string())
        }
    }
}

pub fn group_account_text(
    message_catalog: &MessageCatalog,
    group_account: &AccountIdentity,
) -> String {
    match group_account {
        AccountIdentity::NamedAccount { account_name, .. } => account_name.clone(),
        AccountIdentity::NumberedAccount { numeric_id } => {
            translated_messages::findings::numbered_group(message_catalog, numeric_id.to_string())
        }
    }
}

pub const fn account_numeric_id(account: &AccountIdentity) -> u32 {
    match account {
        AccountIdentity::NamedAccount { numeric_id, .. }
        | AccountIdentity::NumberedAccount { numeric_id } => *numeric_id,
    }
}

pub fn owner_text(message_catalog: &MessageCatalog, live_facts: &LiveFileFacts) -> String {
    let account_text = user_account_text(message_catalog, &live_facts.owner_account);
    match live_facts.owner_relation {
        OwnerRelation::ViewingUser => {
            translated_messages::findings::owner_you(message_catalog, &account_text)
        }
        OwnerRelation::SystemAdministrator => {
            translated_messages::findings::owner_administrator(message_catalog, &account_text)
        }
        OwnerRelation::AnotherUser => {
            translated_messages::findings::owner_other(message_catalog, &account_text)
        }
    }
}

pub fn audience_text(
    message_catalog: &MessageCatalog,
    owner_mention: &OwnerMention,
    access_audience: AccessAudience,
) -> String {
    match (access_audience, owner_mention) {
        (AccessAudience::Everyone, _) => {
            translated_messages::findings::audience_everyone(message_catalog)
        }
        (AccessAudience::OnlyAdministrator, _) => {
            translated_messages::findings::audience_only_administrator(message_catalog)
        }
        (
            AccessAudience::OwnerAndGroup,
            OwnerMention::NamedOwner {
                owner_text,
                group_text,
            },
        ) => translated_messages::findings::audience_owner_and_group(
            message_catalog,
            owner_text,
            group_text,
        ),
        (AccessAudience::OwnerAndGroup, OwnerMention::UnrecordedOwner) => {
            translated_messages::findings::audience_its_owner_and_group(message_catalog)
        }
        (AccessAudience::OnlyOwner, OwnerMention::NamedOwner { owner_text, .. }) => {
            translated_messages::findings::audience_only_owner(message_catalog, owner_text)
        }
        (AccessAudience::OnlyOwner, OwnerMention::UnrecordedOwner) => {
            translated_messages::findings::audience_only_its_owner(message_catalog)
        }
    }
}

pub fn running_text(
    message_catalog: &MessageCatalog,
    owner_mention: &OwnerMention,
    program_running: ProgramRunning,
) -> String {
    match (program_running, owner_mention) {
        (ProgramRunning::NotRunnable, _) => {
            translated_messages::findings::running_no(message_catalog)
        }
        (ProgramRunning::Runnable(running_audience), _) => {
            translated_messages::findings::running_yes(
                message_catalog,
                audience_text(message_catalog, owner_mention, running_audience),
            )
        }
        (ProgramRunning::RunsWithOwnerRights, OwnerMention::NamedOwner { owner_text, .. }) => {
            translated_messages::findings::running_owner_rights(message_catalog, owner_text)
        }
        (ProgramRunning::RunsWithOwnerRights, OwnerMention::UnrecordedOwner) => {
            translated_messages::findings::running_its_owner_rights(message_catalog)
        }
        (ProgramRunning::RunsWithGroupRights, OwnerMention::NamedOwner { group_text, .. }) => {
            translated_messages::findings::running_group_rights(message_catalog, group_text)
        }
        (ProgramRunning::RunsWithGroupRights, OwnerMention::UnrecordedOwner) => {
            translated_messages::findings::running_its_group_rights(message_catalog)
        }
    }
}

#[cfg(test)]
mod file_fact_texts_tests {
    use super::*;
    use crate::shared_widgets::detected_file::detected_file_facts::live_facts_fixtures::alice_live_facts;

    #[test]
    fn owners_are_described_relative_to_the_viewer() {
        let message_catalog = translated_messages::english_catalog();
        let administrator_facts = LiveFileFacts {
            owner_account: AccountIdentity::NamedAccount {
                account_name: "root".to_owned(),
                numeric_id: 0,
            },
            owner_relation: OwnerRelation::SystemAdministrator,
            ..alice_live_facts(0o100_644)
        };
        assert_eq!(
            owner_text(&message_catalog, &administrator_facts),
            "root (system administrator)"
        );
        let stranger_facts = LiveFileFacts {
            owner_account: AccountIdentity::NumberedAccount { numeric_id: 4242 },
            owner_relation: OwnerRelation::AnotherUser,
            ..alice_live_facts(0o100_644)
        };
        assert_eq!(
            owner_text(&message_catalog, &stranger_facts),
            "user 4242 (another user)"
        );
    }
}
