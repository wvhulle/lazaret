use clamd_configuration::override_reports::OverrideLifetime;

use crate::{
    host_services::{
        clamav_config::{
            setting_provenance::ConfigFileOwner,
            typed_setting_values::{SettingParseFailure, TypedSettingValue, WriteRefusal},
        },
        desktop::{locale_formatting::exact_local_time_text, preference_keys::PreferenceKey},
    },
    translated_messages,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RowChangeActivity {
    ChangesAllowed,
    ApplyingThisRow,
    ApplyingOtherRow,
}

impl RowChangeActivity {
    pub const fn accepts_changes(self) -> bool {
        matches!(self, Self::ChangesAllowed)
    }

    pub const fn shows_applying_spinner(self) -> bool {
        matches!(self, Self::ApplyingThisRow)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreferenceSource {
    SchemaDefault,
    ChosenByYou,
    DeclaredByHomeManager,
    ChangedAfterHomeManager { declared_value: TypedSettingValue },
    AdministratorDefault,
    LockedByAdministrator,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreferenceApplyStage {
    AppliesImmediately,
    AppliedWithGroup,
    WaitsForGroupApply,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RootOwnedConfigFile {
    pub file_owner: ConfigFileOwner,
    pub config_path: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileEntry {
    WrittenInFile,
    ClamavBuiltInDefault,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CopyOnlyReason {
    OutsideAllowlist,
    AdminServiceNotInstalled,
    AdminServiceUnreachable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeRoute {
    ChangeableWithPassword,
    CopyLineOnly(CopyOnlyReason),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClamavSource {
    RootOwnedFile {
        config_file: RootOwnedConfigFile,
        file_entry: FileEntry,
        runtime_route: RuntimeRoute,
    },
    SetViaAdminService {
        config_file: RootOwnedConfigFile,
        override_lifetime: OverrideLifetime,
        override_values: Vec<String>,
    },
    ProtectedSetting {
        config_file: RootOwnedConfigFile,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClamavSettingAddress {
    pub setting_key: String,
    pub value_position: usize,
    pub sibling_values: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SettingSubject {
    Preference {
        preference_key: PreferenceKey,
        preference_source: PreferenceSource,
        apply_stage: PreferenceApplyStage,
    },
    ClamavSetting {
        setting_address: ClamavSettingAddress,
        clamav_source: ClamavSource,
    },
}

impl SettingSubject {
    pub fn same_setting_as(&self, other_subject: &Self) -> bool {
        match (self, other_subject) {
            (
                Self::Preference { preference_key, .. },
                Self::Preference {
                    preference_key: other_key,
                    ..
                },
            ) => preference_key == other_key,
            (
                Self::ClamavSetting {
                    setting_address, ..
                },
                Self::ClamavSetting {
                    setting_address: other_address,
                    ..
                },
            ) => {
                setting_address.setting_key == other_address.setting_key
                    && setting_address.value_position == other_address.value_position
            }
            (Self::Preference { .. }, Self::ClamavSetting { .. })
            | (Self::ClamavSetting { .. }, Self::Preference { .. }) => false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShownSettingValue {
    ReadableValue(TypedSettingValue),
    UnreadableValue {
        raw_text: String,
        parse_failure: SettingParseFailure,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SettingLabel {
    DescribedSetting { title: String, description: String },
    ConfigKeyOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettingRowState {
    pub setting_label: SettingLabel,
    pub setting_subject: SettingSubject,
    pub shown_value: ShownSettingValue,
    pub change_activity: RowChangeActivity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChangeRoute {
    ChangesImmediately,
    AsksAdministratorPassword,
    TakesEffectOnGroupApply,
    CopyLineForConfiguration(CopyOnlyReason),
    LockedByAdministrator,
    NeverChangedHere,
}

impl ChangeRoute {
    pub const fn allows_editing(self) -> bool {
        matches!(
            self,
            Self::ChangesImmediately | Self::AsksAdministratorPassword | Self::TakesEffectOnGroupApply
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceEmphasis {
    PlainSource,
    NeedsAttention,
    FixedValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceTexts {
    pub short_tag: String,
    pub explanation: String,
    pub source_emphasis: SourceEmphasis,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FollowUpAction {
    ResetToDefault,
    RevertRuntimeChange,
    RestoreHomeManagerValue,
    CopyConfigurationLine,
    CopyHomeManagerLine,
}

pub const fn change_route(setting_subject: &SettingSubject) -> ChangeRoute {
    match setting_subject {
        SettingSubject::Preference {
            preference_source: PreferenceSource::LockedByAdministrator,
            ..
        } => ChangeRoute::LockedByAdministrator,
        SettingSubject::Preference {
            apply_stage: PreferenceApplyStage::AppliesImmediately,
            ..
        } => ChangeRoute::ChangesImmediately,
        SettingSubject::Preference {
            apply_stage:
                PreferenceApplyStage::AppliedWithGroup | PreferenceApplyStage::WaitsForGroupApply,
            ..
        } => ChangeRoute::TakesEffectOnGroupApply,
        SettingSubject::ClamavSetting { clamav_source, .. } => match clamav_source {
            ClamavSource::RootOwnedFile {
                runtime_route: RuntimeRoute::ChangeableWithPassword,
                ..
            }
            | ClamavSource::SetViaAdminService { .. } => ChangeRoute::AsksAdministratorPassword,
            ClamavSource::RootOwnedFile {
                runtime_route: RuntimeRoute::CopyLineOnly(copy_only_reason),
                ..
            } => ChangeRoute::CopyLineForConfiguration(*copy_only_reason),
            ClamavSource::ProtectedSetting { .. } => ChangeRoute::NeverChangedHere,
        },
    }
}

fn config_file_name(config_file: &RootOwnedConfigFile) -> String {
    std::path::Path::new(&config_file.config_path)
        .file_name()
        .map_or_else(
            || config_file.config_path.clone(),
            |file_name| file_name.to_string_lossy().into_owned(),
        )
}

fn file_source_texts(config_file: &RootOwnedConfigFile, file_entry: FileEntry) -> SourceTexts {
    let (short_tag, explanation) = match (config_file.file_owner, file_entry) {
        (ConfigFileOwner::NixOsConfiguration, FileEntry::WrittenInFile) => (
            translated_messages::widgets::source_nixos_tag(),
            translated_messages::widgets::source_nixos_explanation(),
        ),
        (ConfigFileOwner::NixOsConfiguration, FileEntry::ClamavBuiltInDefault) => (
            translated_messages::widgets::source_clamav_default_tag(),
            translated_messages::widgets::source_nixos_default_explanation(),
        ),
        (ConfigFileOwner::SystemPackage, FileEntry::WrittenInFile) => (
            translated_messages::widgets::source_config_file_tag(config_file_name(config_file)),
            translated_messages::widgets::source_config_file_explanation(&config_file.config_path),
        ),
        (ConfigFileOwner::SystemPackage, FileEntry::ClamavBuiltInDefault) => (
            translated_messages::widgets::source_clamav_default_tag(),
            translated_messages::widgets::source_config_file_default_explanation(
                &config_file.config_path,
            ),
        ),
    };
    SourceTexts {
        short_tag,
        explanation,
        source_emphasis: SourceEmphasis::PlainSource,
    }
}

fn override_source_texts(
    config_file: &RootOwnedConfigFile,
    override_lifetime: OverrideLifetime,
) -> SourceTexts {
    match override_lifetime {
        OverrideLifetime::Permanent => SourceTexts {
            short_tag: translated_messages::widgets::source_changed_here_tag(),
            explanation: match config_file.file_owner {
                ConfigFileOwner::NixOsConfiguration => {
                    translated_messages::widgets::source_runtime_nixos_explanation()
                }
                ConfigFileOwner::SystemPackage => {
                    translated_messages::widgets::source_runtime_file_explanation(
                        &config_file.config_path,
                    )
                }
            },
            source_emphasis: SourceEmphasis::NeedsAttention,
        },
        OverrideLifetime::ExpiresAt { expires_at } => SourceTexts {
            short_tag: translated_messages::widgets::source_reduced_until_tag(
                exact_local_time_text(expires_at),
            ),
            explanation: translated_messages::widgets::source_reduced_until_explanation(
                exact_local_time_text(expires_at),
            ),
            source_emphasis: SourceEmphasis::NeedsAttention,
        },
    }
}

fn preference_source_texts(
    preference_source: &PreferenceSource,
    declared_value_text: impl FnOnce(&TypedSettingValue) -> String,
) -> SourceTexts {
    match preference_source {
        PreferenceSource::SchemaDefault => SourceTexts {
            short_tag: translated_messages::widgets::source_default_tag(),
            explanation: String::new(),
            source_emphasis: SourceEmphasis::PlainSource,
        },
        PreferenceSource::ChosenByYou => SourceTexts {
            short_tag: translated_messages::widgets::source_your_choice_tag(),
            explanation: String::new(),
            source_emphasis: SourceEmphasis::PlainSource,
        },
        PreferenceSource::DeclaredByHomeManager => SourceTexts {
            short_tag: translated_messages::widgets::source_home_manager_tag(),
            explanation: translated_messages::widgets::source_home_manager_explanation(),
            source_emphasis: SourceEmphasis::PlainSource,
        },
        PreferenceSource::ChangedAfterHomeManager { declared_value } => SourceTexts {
            short_tag: translated_messages::widgets::source_changed_here_tag(),
            explanation: translated_messages::widgets::source_home_manager_changed_explanation(
                declared_value_text(declared_value),
            ),
            source_emphasis: SourceEmphasis::NeedsAttention,
        },
        PreferenceSource::AdministratorDefault => SourceTexts {
            short_tag: translated_messages::widgets::source_administrator_default_tag(),
            explanation: translated_messages::widgets::source_administrator_default_explanation(),
            source_emphasis: SourceEmphasis::PlainSource,
        },
        PreferenceSource::LockedByAdministrator => SourceTexts {
            short_tag: translated_messages::widgets::source_locked_tag(),
            explanation: translated_messages::widgets::source_locked_explanation(),
            source_emphasis: SourceEmphasis::FixedValue,
        },
    }
}

pub fn shown_value_text(typed_value: &TypedSettingValue) -> String {
    match typed_value {
        TypedSettingValue::BooleanValue(switch_position) => {
            if switch_position.is_switched_on() {
                translated_messages::widgets::switched_on_value()
            } else {
                translated_messages::widgets::switched_off_value()
            }
        }
        TypedSettingValue::CountValue { .. }
        | TypedSettingValue::SizeValue(_)
        | TypedSettingValue::PathValue(_)
        | TypedSettingValue::TextValue(_)
        | TypedSettingValue::ChoiceValue { .. } => typed_value.config_value_text(),
    }
}

pub fn source_texts(setting_subject: &SettingSubject) -> SourceTexts {
    match setting_subject {
        SettingSubject::Preference {
            apply_stage: PreferenceApplyStage::WaitsForGroupApply,
            preference_source,
            ..
        } if *preference_source != PreferenceSource::LockedByAdministrator => SourceTexts {
            short_tag: translated_messages::widgets::source_not_applied_tag(),
            explanation: translated_messages::widgets::source_not_applied_explanation(),
            source_emphasis: SourceEmphasis::NeedsAttention,
        },
        SettingSubject::Preference {
            preference_source, ..
        } => preference_source_texts(preference_source, shown_value_text),
        SettingSubject::ClamavSetting { clamav_source, .. } => match clamav_source {
            ClamavSource::RootOwnedFile {
                config_file,
                file_entry,
                ..
            } => file_source_texts(config_file, *file_entry),
            ClamavSource::SetViaAdminService {
                config_file,
                override_lifetime,
                ..
            } => override_source_texts(config_file, *override_lifetime),
            ClamavSource::ProtectedSetting { .. } => SourceTexts {
                short_tag: translated_messages::widgets::source_protected_tag(),
                explanation: translated_messages::widgets::source_protected_explanation(),
                source_emphasis: SourceEmphasis::FixedValue,
            },
        },
    }
}

pub fn route_explanation(route: ChangeRoute) -> String {
    match route {
        ChangeRoute::ChangesImmediately
        | ChangeRoute::LockedByAdministrator
        | ChangeRoute::NeverChangedHere => String::new(),
        ChangeRoute::AsksAdministratorPassword => {
            translated_messages::widgets::route_administrator_password_explanation()
        }
        ChangeRoute::TakesEffectOnGroupApply => {
            translated_messages::widgets::route_group_apply_explanation()
        }
        ChangeRoute::CopyLineForConfiguration(CopyOnlyReason::OutsideAllowlist) => {
            translated_messages::widgets::route_outside_allowlist_explanation()
        }
        ChangeRoute::CopyLineForConfiguration(CopyOnlyReason::AdminServiceNotInstalled) => {
            translated_messages::widgets::route_admin_service_missing_explanation()
        }
        ChangeRoute::CopyLineForConfiguration(CopyOnlyReason::AdminServiceUnreachable) => {
            translated_messages::widgets::route_admin_service_unreachable_explanation()
        }
    }
}

pub fn follow_up_actions(
    setting_subject: &SettingSubject,
    shown_value: &ShownSettingValue,
) -> Vec<FollowUpAction> {
    match setting_subject {
        SettingSubject::Preference {
            preference_source, ..
        } => match preference_source {
            PreferenceSource::ChosenByYou => vec![FollowUpAction::ResetToDefault],
            PreferenceSource::ChangedAfterHomeManager { .. } => vec![
                FollowUpAction::RestoreHomeManagerValue,
                FollowUpAction::CopyHomeManagerLine,
            ],
            PreferenceSource::SchemaDefault
            | PreferenceSource::DeclaredByHomeManager
            | PreferenceSource::AdministratorDefault
            | PreferenceSource::LockedByAdministrator => Vec::new(),
        },
        SettingSubject::ClamavSetting { clamav_source, .. } => match (clamav_source, shown_value) {
            (ClamavSource::SetViaAdminService { .. }, _shown_value) => vec![
                FollowUpAction::RevertRuntimeChange,
                FollowUpAction::CopyConfigurationLine,
            ],
            (
                ClamavSource::RootOwnedFile {
                    runtime_route: RuntimeRoute::CopyLineOnly(_),
                    ..
                },
                ShownSettingValue::ReadableValue(_),
            ) => vec![FollowUpAction::CopyConfigurationLine],
            (
                ClamavSource::RootOwnedFile {
                    runtime_route: RuntimeRoute::CopyLineOnly(_),
                    ..
                },
                ShownSettingValue::UnreadableValue { .. },
            )
            | (
                ClamavSource::RootOwnedFile {
                    runtime_route: RuntimeRoute::ChangeableWithPassword,
                    ..
                }
                | ClamavSource::ProtectedSetting { .. },
                _,
            ) => Vec::new(),
        },
    }
}

pub fn row_needs_attention(setting_subject: &SettingSubject) -> bool {
    source_texts(setting_subject).source_emphasis == SourceEmphasis::NeedsAttention
}

pub fn draft_refusal(
    setting_subject: &SettingSubject,
    editor_draft: &TypedSettingValue,
) -> Result<(), WriteRefusal> {
    match setting_subject {
        SettingSubject::Preference { preference_key, .. } => {
            preference_key.accepts_value(editor_draft)
        }
        SettingSubject::ClamavSetting {
            setting_address, ..
        } => editor_draft
            .writable_value(&setting_address.setting_key)
            .map(|_writable_text| ()),
    }
}

pub fn describe_parse_failure(parse_failure: SettingParseFailure) -> String {
    match parse_failure {
        SettingParseFailure::UnknownOption => translated_messages::widgets::unknown_option(),
        SettingParseFailure::NotYesOrNo => translated_messages::widgets::not_yes_no(),
        SettingParseFailure::NotAWholeNumber => translated_messages::widgets::not_whole_number(),
        SettingParseFailure::CountOutOfRange { minimum, maximum } => {
            translated_messages::widgets::count_out_range(minimum, maximum)
        }
        SettingParseFailure::NotASize => translated_messages::widgets::not_size(),
        SettingParseFailure::PathNotAbsolute => translated_messages::widgets::path_not_absolute(),
        SettingParseFailure::EmptyText => translated_messages::widgets::empty_text(),
        SettingParseFailure::NotAClockTime => translated_messages::widgets::not_clock_time(),
    }
}

#[cfg(test)]
mod setting_row_state_tests {
    use jiff::Timestamp;

    use super::*;
    use crate::host_services::clamav_config::typed_setting_values::SwitchPosition;

    fn nixos_file() -> RootOwnedConfigFile {
        RootOwnedConfigFile {
            file_owner: ConfigFileOwner::NixOsConfiguration,
            config_path: "/etc/clamav/clamd.conf".to_owned(),
        }
    }

    fn clamav_subject(clamav_source: ClamavSource) -> SettingSubject {
        SettingSubject::ClamavSetting {
            setting_address: ClamavSettingAddress {
                setting_key: "DetectPUA".to_owned(),
                value_position: 0,
                sibling_values: vec!["no".to_owned()],
            },
            clamav_source,
        }
    }

    fn preference_subject(
        preference_source: PreferenceSource,
        apply_stage: PreferenceApplyStage,
    ) -> SettingSubject {
        SettingSubject::Preference {
            preference_key: PreferenceKey::NotificationsEnabled,
            preference_source,
            apply_stage,
        }
    }

    fn switch_value() -> ShownSettingValue {
        ShownSettingValue::ReadableValue(TypedSettingValue::BooleanValue(
            SwitchPosition::SwitchedOn,
        ))
    }

    fn runtime_override(override_lifetime: OverrideLifetime) -> SettingSubject {
        clamav_subject(ClamavSource::SetViaAdminService {
            config_file: nixos_file(),
            override_lifetime,
            override_values: vec!["yes".to_owned()],
        })
    }

    fn file_setting(runtime_route: RuntimeRoute) -> SettingSubject {
        clamav_subject(ClamavSource::RootOwnedFile {
            config_file: nixos_file(),
            file_entry: FileEntry::WrittenInFile,
            runtime_route,
        })
    }

    #[test]
    fn a_runtime_change_is_reverted_and_copied_from_its_own_row() {
        let changed_subject = runtime_override(OverrideLifetime::Permanent);
        assert_eq!(
            change_route(&changed_subject),
            ChangeRoute::AsksAdministratorPassword
        );
        assert_eq!(
            follow_up_actions(&changed_subject, &switch_value()),
            vec![
                FollowUpAction::RevertRuntimeChange,
                FollowUpAction::CopyConfigurationLine
            ]
        );
        assert!(row_needs_attention(&changed_subject));
        assert!(row_needs_attention(&runtime_override(
            OverrideLifetime::ExpiresAt {
                expires_at: Timestamp::UNIX_EPOCH
            }
        )));
    }

    #[test]
    fn an_allowed_setting_is_editable_without_unlocking_and_needs_no_follow_up() {
        let allowed_subject = file_setting(RuntimeRoute::ChangeableWithPassword);
        assert!(change_route(&allowed_subject).allows_editing());
        assert!(follow_up_actions(&allowed_subject, &switch_value()).is_empty());
        assert!(!row_needs_attention(&allowed_subject));
    }

    #[test]
    fn settings_outside_the_allowlist_offer_only_their_configuration_line() {
        [
            CopyOnlyReason::OutsideAllowlist,
            CopyOnlyReason::AdminServiceNotInstalled,
            CopyOnlyReason::AdminServiceUnreachable,
        ]
        .into_iter()
        .for_each(|copy_only_reason| {
            let copy_subject = file_setting(RuntimeRoute::CopyLineOnly(copy_only_reason));
            assert_eq!(
                change_route(&copy_subject),
                ChangeRoute::CopyLineForConfiguration(copy_only_reason)
            );
            assert!(!change_route(&copy_subject).allows_editing());
            assert_eq!(
                follow_up_actions(&copy_subject, &switch_value()),
                vec![FollowUpAction::CopyConfigurationLine]
            );
            assert!(!route_explanation(change_route(&copy_subject)).is_empty());
        });
    }

    #[test]
    fn protected_settings_and_administrator_locks_are_fixed() {
        let protected_subject = clamav_subject(ClamavSource::ProtectedSetting {
            config_file: nixos_file(),
        });
        assert_eq!(
            change_route(&protected_subject),
            ChangeRoute::NeverChangedHere
        );
        assert!(follow_up_actions(&protected_subject, &switch_value()).is_empty());
        let locked_subject = preference_subject(
            PreferenceSource::LockedByAdministrator,
            PreferenceApplyStage::WaitsForGroupApply,
        );
        assert_eq!(
            change_route(&locked_subject),
            ChangeRoute::LockedByAdministrator
        );
        assert_eq!(
            source_texts(&locked_subject).source_emphasis,
            SourceEmphasis::FixedValue
        );
    }

    #[test]
    fn preferences_offer_a_reset_only_when_you_chose_the_value() {
        assert_eq!(
            follow_up_actions(
                &preference_subject(
                    PreferenceSource::ChosenByYou,
                    PreferenceApplyStage::AppliesImmediately
                ),
                &switch_value()
            ),
            vec![FollowUpAction::ResetToDefault]
        );
        [
            PreferenceSource::SchemaDefault,
            PreferenceSource::DeclaredByHomeManager,
            PreferenceSource::AdministratorDefault,
        ]
        .into_iter()
        .for_each(|preference_source| {
            let quiet_subject =
                preference_subject(preference_source, PreferenceApplyStage::AppliesImmediately);
            assert!(follow_up_actions(&quiet_subject, &switch_value()).is_empty());
            assert!(!row_needs_attention(&quiet_subject));
        });
    }

    #[test]
    fn a_value_changed_after_home_manager_can_be_restored() {
        let drifted_subject = preference_subject(
            PreferenceSource::ChangedAfterHomeManager {
                declared_value: TypedSettingValue::BooleanValue(SwitchPosition::SwitchedOff),
            },
            PreferenceApplyStage::AppliesImmediately,
        );
        assert!(row_needs_attention(&drifted_subject));
        assert_eq!(
            follow_up_actions(&drifted_subject, &switch_value()),
            vec![
                FollowUpAction::RestoreHomeManagerValue,
                FollowUpAction::CopyHomeManagerLine
            ]
        );
    }

    #[test]
    fn a_staged_preference_waits_for_the_group_apply() {
        let staged_subject = preference_subject(
            PreferenceSource::ChosenByYou,
            PreferenceApplyStage::WaitsForGroupApply,
        );
        assert_eq!(
            change_route(&staged_subject),
            ChangeRoute::TakesEffectOnGroupApply
        );
        assert!(row_needs_attention(&staged_subject));
        assert!(!row_needs_attention(&preference_subject(
            PreferenceSource::ChosenByYou,
            PreferenceApplyStage::AppliedWithGroup
        )));
    }
}
