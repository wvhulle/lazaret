use std::{collections::HashMap, fs, path::PathBuf};

use relm4::gtk::{gio, gio::prelude::*, glib};

use crate::{
    host_services::{
        clamav_config::typed_setting_values::TypedSettingValue,
        desktop::preference_keys::PreferenceKey,
    },
    shared_widgets::setting_row::setting_row_state::PreferenceSource,
};

const HOME_MANAGER_DECLARATION_FILE: &str = "lazaret/home-manager-settings.json";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HomeManagerDeclarations {
    declared_values: HashMap<PreferenceKey, TypedSettingValue>,
}

impl HomeManagerDeclarations {
    pub fn from_declaration_json(declaration_text: &str) -> Self {
        let declared_values = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(
            declaration_text,
        )
        .map(|declared_settings| {
            declared_settings
                .iter()
                .filter_map(|(schema_key, declared_json)| {
                    let preference_key = PreferenceKey::from_schema_key(schema_key)?;
                    let declared_value = preference_key.typed_value_of_json(declared_json)?;
                    Some((preference_key, declared_value))
                })
                .collect()
        })
        .unwrap_or_default();
        Self { declared_values }
    }

    pub fn declared_value(&self, preference_key: PreferenceKey) -> Option<&TypedSettingValue> {
        self.declared_values.get(&preference_key)
    }
}

pub fn home_manager_declaration_path() -> PathBuf {
    glib::user_config_dir().join(HOME_MANAGER_DECLARATION_FILE)
}

pub fn read_home_manager_declarations() -> HomeManagerDeclarations {
    fs::read_to_string(home_manager_declaration_path())
        .map(|declaration_text| HomeManagerDeclarations::from_declaration_json(&declaration_text))
        .unwrap_or_default()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyWritability {
    WritableKey,
    LockedKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreferenceObservation {
    pub key_writability: KeyWritability,
    pub user_value: Option<TypedSettingValue>,
    pub effective_default: Option<TypedSettingValue>,
    pub schema_default: Option<TypedSettingValue>,
    pub declared_value: Option<TypedSettingValue>,
}

pub fn preference_source_of(observation: PreferenceObservation) -> PreferenceSource {
    match observation {
        PreferenceObservation {
            key_writability: KeyWritability::LockedKey,
            ..
        } => PreferenceSource::LockedByAdministrator,
        PreferenceObservation {
            user_value: Some(user_value),
            declared_value: Some(declared_value),
            ..
        } => {
            if user_value == declared_value {
                PreferenceSource::DeclaredByHomeManager
            } else {
                PreferenceSource::ChangedAfterHomeManager { declared_value }
            }
        }
        PreferenceObservation {
            user_value: Some(_),
            declared_value: None,
            ..
        } => PreferenceSource::ChosenByYou,
        PreferenceObservation {
            user_value: None,
            effective_default,
            schema_default,
            ..
        } => {
            if effective_default == schema_default {
                PreferenceSource::SchemaDefault
            } else {
                PreferenceSource::AdministratorDefault
            }
        }
    }
}

pub fn observe_preference(
    stored_settings: &gio::Settings,
    preference_key: PreferenceKey,
    home_manager_declarations: &HomeManagerDeclarations,
) -> PreferenceObservation {
    let schema_key = preference_key.schema_key();
    PreferenceObservation {
        key_writability: if stored_settings.is_writable(schema_key) {
            KeyWritability::WritableKey
        } else {
            KeyWritability::LockedKey
        },
        user_value: stored_settings
            .user_value(schema_key)
            .and_then(|user_variant| preference_key.typed_value_of(&user_variant)),
        effective_default: stored_settings
            .default_value(schema_key)
            .and_then(|default_variant| preference_key.typed_value_of(&default_variant)),
        schema_default: stored_settings
            .settings_schema()
            .filter(|settings_schema| settings_schema.has_key(schema_key))
            .map(|settings_schema| settings_schema.key(schema_key).default_value())
            .and_then(|default_variant| preference_key.typed_value_of(&default_variant)),
        declared_value: home_manager_declarations
            .declared_value(preference_key)
            .cloned(),
    }
}

pub fn preference_source(
    stored_settings: &gio::Settings,
    preference_key: PreferenceKey,
    home_manager_declarations: &HomeManagerDeclarations,
) -> PreferenceSource {
    preference_source_of(observe_preference(
        stored_settings,
        preference_key,
        home_manager_declarations,
    ))
}

#[cfg(test)]
mod preference_sources_tests {
    use super::*;
    use crate::host_services::clamav_config::typed_setting_values::SwitchPosition;

    const fn switch_value(switched_on: bool) -> TypedSettingValue {
        TypedSettingValue::BooleanValue(SwitchPosition::from_switch_active(switched_on))
    }

    fn observation_with(
        user_value: Option<bool>,
        effective_default: bool,
        declared_value: Option<bool>,
    ) -> PreferenceObservation {
        PreferenceObservation {
            key_writability: KeyWritability::WritableKey,
            user_value: user_value.map(switch_value),
            effective_default: Some(switch_value(effective_default)),
            schema_default: Some(switch_value(true)),
            declared_value: declared_value.map(switch_value),
        }
    }

    #[test]
    fn an_administrator_lock_wins_over_every_other_source() {
        assert_eq!(
            preference_source_of(PreferenceObservation {
                key_writability: KeyWritability::LockedKey,
                ..observation_with(Some(false), true, Some(false))
            }),
            PreferenceSource::LockedByAdministrator
        );
    }

    #[test]
    fn a_value_you_set_is_yours_unless_home_manager_declares_the_key() {
        assert_eq!(
            preference_source_of(observation_with(Some(false), true, None)),
            PreferenceSource::ChosenByYou
        );
        assert_eq!(
            preference_source_of(observation_with(Some(false), true, Some(false))),
            PreferenceSource::DeclaredByHomeManager
        );
        assert_eq!(
            preference_source_of(observation_with(Some(true), true, Some(false))),
            PreferenceSource::ChangedAfterHomeManager {
                declared_value: switch_value(false)
            }
        );
    }

    #[test]
    fn without_your_own_value_the_default_tells_who_chose_it() {
        assert_eq!(
            preference_source_of(observation_with(None, true, None)),
            PreferenceSource::SchemaDefault
        );
        assert_eq!(
            preference_source_of(observation_with(None, false, None)),
            PreferenceSource::AdministratorDefault
        );
    }

    #[test]
    fn home_manager_declarations_skip_unknown_keys_and_mistyped_values() {
        let declarations = HomeManagerDeclarations::from_declaration_json(
            r#"{"close-action": "quit", "notifications-enabled": "yes", "not-a-key": 1}"#,
        );
        assert!(
            declarations
                .declared_value(PreferenceKey::CloseAction)
                .is_some()
        );
        assert_eq!(
            declarations.declared_value(PreferenceKey::NotificationsEnabled),
            None
        );
        assert_eq!(
            HomeManagerDeclarations::from_declaration_json("not json"),
            HomeManagerDeclarations::default()
        );
    }
}
