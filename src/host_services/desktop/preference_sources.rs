use relm4::gtk::{gio, gio::prelude::*};

use crate::{
    host_services::{
        clamav_config::typed_setting_values::TypedSettingValue,
        desktop::preference_keys::PreferenceKey,
    },
    shared_widgets::setting_row::setting_row_state::PreferenceSource,
    translated_messages::MessageCatalog,
};

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
}

pub fn preference_source_of(observation: PreferenceObservation) -> PreferenceSource {
    match observation {
        PreferenceObservation {
            key_writability: KeyWritability::LockedKey,
            ..
        } => PreferenceSource::FixedByConfiguration,
        PreferenceObservation {
            user_value: Some(_),
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
    message_catalog: &MessageCatalog,
    stored_settings: &gio::Settings,
    preference_key: PreferenceKey,
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
            .and_then(|user_variant| preference_key.typed_value_of(message_catalog, &user_variant)),
        effective_default: stored_settings
            .default_value(schema_key)
            .and_then(|default_variant| {
                preference_key.typed_value_of(message_catalog, &default_variant)
            }),
        schema_default: stored_settings
            .settings_schema()
            .filter(|settings_schema| settings_schema.has_key(schema_key))
            .map(|settings_schema| settings_schema.key(schema_key).default_value())
            .and_then(|default_variant| {
                preference_key.typed_value_of(message_catalog, &default_variant)
            }),
    }
}

pub fn preference_source(
    message_catalog: &MessageCatalog,
    stored_settings: &gio::Settings,
    preference_key: PreferenceKey,
) -> PreferenceSource {
    preference_source_of(observe_preference(
        message_catalog,
        stored_settings,
        preference_key,
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
    ) -> PreferenceObservation {
        PreferenceObservation {
            key_writability: KeyWritability::WritableKey,
            user_value: user_value.map(switch_value),
            effective_default: Some(switch_value(effective_default)),
            schema_default: Some(switch_value(true)),
        }
    }

    #[test]
    fn a_locked_key_is_fixed_by_configuration_whatever_you_stored() {
        assert_eq!(
            preference_source_of(PreferenceObservation {
                key_writability: KeyWritability::LockedKey,
                ..observation_with(Some(false), true)
            }),
            PreferenceSource::FixedByConfiguration
        );
    }

    #[test]
    fn a_value_you_stored_is_your_choice() {
        assert_eq!(
            preference_source_of(observation_with(Some(false), true)),
            PreferenceSource::ChosenByYou
        );
    }

    #[test]
    fn without_your_own_value_the_default_tells_who_chose_it() {
        assert_eq!(
            preference_source_of(observation_with(None, true)),
            PreferenceSource::SchemaDefault
        );
        assert_eq!(
            preference_source_of(observation_with(None, false)),
            PreferenceSource::AdministratorDefault
        );
    }
}
