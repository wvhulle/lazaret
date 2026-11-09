use relm4::gtk::{gio, gio::prelude::*, glib};

use crate::{
    host_services::clamav_config::typed_setting_values::{
        SettingParseFailure, SwitchPosition, TypedSettingValue, WriteRefusal,
    },
    scans::scheduled_scans_page::scan_schedule_units::parse_schedule_time,
    translated_messages,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PreferenceKey {
    CloseAction,
    StartInBackground,
    NotificationsEnabled,
    ShowLiveProgress,
    ScanBackend,
    DeviceScanEnabled,
    DeviceScanMaxGigabytes,
    DeviceScanSkipOnBattery,
    ScheduleEnabled,
    ScheduleFrequency,
    ScheduleTime,
    ScheduleDayOfWeek,
    ScheduleDayOfMonth,
    ScheduleSkipOnBattery,
    ScheduleAutoQuarantine,
}

pub const PREFERENCE_KEYS: [PreferenceKey; 15] = [
    PreferenceKey::CloseAction,
    PreferenceKey::StartInBackground,
    PreferenceKey::NotificationsEnabled,
    PreferenceKey::ShowLiveProgress,
    PreferenceKey::ScanBackend,
    PreferenceKey::DeviceScanEnabled,
    PreferenceKey::DeviceScanMaxGigabytes,
    PreferenceKey::DeviceScanSkipOnBattery,
    PreferenceKey::ScheduleEnabled,
    PreferenceKey::ScheduleFrequency,
    PreferenceKey::ScheduleTime,
    PreferenceKey::ScheduleDayOfWeek,
    PreferenceKey::ScheduleDayOfMonth,
    PreferenceKey::ScheduleSkipOnBattery,
    PreferenceKey::ScheduleAutoQuarantine,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PreferenceValueKind {
    SwitchPreference,
    UnsignedCountPreference { minimum: u64, maximum: u64 },
    SignedCountPreference { minimum: u64, maximum: u64 },
    TextChoicePreference { stored_texts: &'static [&'static str] },
    NumberChoicePreference,
    ClockTimePreference,
}

#[derive(Debug, thiserror::Error)]
pub enum PreferenceWriteFailure {
    #[error("the value doesn't fit this preference")]
    ValueKindMismatch,
    #[error("the preference is locked: {0}")]
    PreferenceNotWritable(#[from] glib::BoolError),
}

const CLOSE_ACTION_TEXTS: &[&str] = &["quit", "minimize", "ask"];
const SCAN_BACKEND_TEXTS: &[&str] = &["auto", "clamd", "clamscan"];
const SCHEDULE_FREQUENCY_TEXTS: &[&str] = &["hourly", "daily", "weekly", "monthly"];

fn close_action_labels() -> Vec<String> {
    vec![
        translated_messages::window::quit_completely(),
        translated_messages::window::hide_tray(),
        translated_messages::window::ask_each_time(),
    ]
}

fn scan_backend_labels() -> Vec<String> {
    vec![
        translated_messages::scans::prefer_clamd_when_available(),
        translated_messages::scans::always_clamd(),
        translated_messages::scans::always_clamscan(),
    ]
}

fn schedule_frequency_labels() -> Vec<String> {
    vec![
        translated_messages::scans::hourly_label(),
        translated_messages::scans::daily_label(),
        translated_messages::scans::weekly_label(),
        translated_messages::scans::monthly_label(),
    ]
}

fn weekday_labels() -> Vec<String> {
    vec![
        translated_messages::scans::monday_label(),
        translated_messages::scans::tuesday_label(),
        translated_messages::scans::wednesday_label(),
        translated_messages::scans::thursday_label(),
        translated_messages::scans::friday_label(),
        translated_messages::scans::saturday_label(),
        translated_messages::scans::sunday_label(),
    ]
}

impl PreferenceKey {
    pub const fn schema_key(self) -> &'static str {
        match self {
            Self::CloseAction => "close-action",
            Self::StartInBackground => "start-in-background",
            Self::NotificationsEnabled => "notifications-enabled",
            Self::ShowLiveProgress => "show-live-progress",
            Self::ScanBackend => "scan-backend",
            Self::DeviceScanEnabled => "device-scan-enabled",
            Self::DeviceScanMaxGigabytes => "device-scan-max-gigabytes",
            Self::DeviceScanSkipOnBattery => "device-scan-skip-on-battery",
            Self::ScheduleEnabled => "schedule-enabled",
            Self::ScheduleFrequency => "schedule-frequency",
            Self::ScheduleTime => "schedule-time",
            Self::ScheduleDayOfWeek => "schedule-day-of-week",
            Self::ScheduleDayOfMonth => "schedule-day-of-month",
            Self::ScheduleSkipOnBattery => "schedule-skip-on-battery",
            Self::ScheduleAutoQuarantine => "schedule-auto-quarantine",
        }
    }

    pub fn from_schema_key(schema_key: &str) -> Option<Self> {
        PREFERENCE_KEYS
            .into_iter()
            .find(|preference_key| preference_key.schema_key() == schema_key)
    }

    const fn value_kind(self) -> PreferenceValueKind {
        match self {
            Self::StartInBackground
            | Self::NotificationsEnabled
            | Self::ShowLiveProgress
            | Self::DeviceScanEnabled
            | Self::DeviceScanSkipOnBattery
            | Self::ScheduleEnabled
            | Self::ScheduleSkipOnBattery
            | Self::ScheduleAutoQuarantine => PreferenceValueKind::SwitchPreference,
            Self::CloseAction => PreferenceValueKind::TextChoicePreference {
                stored_texts: CLOSE_ACTION_TEXTS,
            },
            Self::ScanBackend => PreferenceValueKind::TextChoicePreference {
                stored_texts: SCAN_BACKEND_TEXTS,
            },
            Self::ScheduleFrequency => PreferenceValueKind::TextChoicePreference {
                stored_texts: SCHEDULE_FREQUENCY_TEXTS,
            },
            Self::DeviceScanMaxGigabytes => PreferenceValueKind::UnsignedCountPreference {
                minimum: 0,
                maximum: 10_000,
            },
            Self::ScheduleDayOfMonth => PreferenceValueKind::SignedCountPreference {
                minimum: 1,
                maximum: 28,
            },
            Self::ScheduleDayOfWeek => PreferenceValueKind::NumberChoicePreference,
            Self::ScheduleTime => PreferenceValueKind::ClockTimePreference,
        }
    }

    fn choice_labels(self) -> Vec<String> {
        match self {
            Self::CloseAction => close_action_labels(),
            Self::ScanBackend => scan_backend_labels(),
            Self::ScheduleFrequency => schedule_frequency_labels(),
            Self::ScheduleDayOfWeek => weekday_labels(),
            Self::StartInBackground
            | Self::NotificationsEnabled
            | Self::ShowLiveProgress
            | Self::DeviceScanEnabled
            | Self::DeviceScanMaxGigabytes
            | Self::DeviceScanSkipOnBattery
            | Self::ScheduleEnabled
            | Self::ScheduleTime
            | Self::ScheduleDayOfMonth
            | Self::ScheduleSkipOnBattery
            | Self::ScheduleAutoQuarantine => Vec::new(),
        }
    }

    fn choice_value(self, chosen_position: u32) -> TypedSettingValue {
        TypedSettingValue::ChoiceValue {
            chosen_position,
            choice_labels: self.choice_labels(),
        }
    }

    pub fn typed_value_of(self, stored_variant: &glib::Variant) -> Option<TypedSettingValue> {
        match self.value_kind() {
            PreferenceValueKind::SwitchPreference => stored_variant
                .get::<bool>()
                .map(|switched_on| {
                    TypedSettingValue::BooleanValue(SwitchPosition::from_switch_active(switched_on))
                }),
            PreferenceValueKind::UnsignedCountPreference { minimum, maximum } => stored_variant
                .get::<u32>()
                .map(|stored_count| TypedSettingValue::CountValue {
                    count_value: u64::from(stored_count).clamp(minimum, maximum),
                    minimum,
                    maximum,
                }),
            PreferenceValueKind::SignedCountPreference { minimum, maximum } => stored_variant
                .get::<i32>()
                .and_then(|stored_count| u64::try_from(stored_count).ok())
                .map(|stored_count| TypedSettingValue::CountValue {
                    count_value: stored_count.clamp(minimum, maximum),
                    minimum,
                    maximum,
                }),
            PreferenceValueKind::TextChoicePreference { stored_texts } => stored_variant
                .get::<String>()
                .and_then(|stored_text| {
                    stored_texts
                        .iter()
                        .position(|choice_text| *choice_text == stored_text)
                })
                .and_then(|choice_index| u32::try_from(choice_index).ok())
                .map(|chosen_position| self.choice_value(chosen_position)),
            PreferenceValueKind::NumberChoicePreference => stored_variant
                .get::<i32>()
                .and_then(|stored_number| u32::try_from(stored_number).ok())
                .map(|chosen_position| self.choice_value(chosen_position)),
            PreferenceValueKind::ClockTimePreference => stored_variant
                .get::<String>()
                .map(TypedSettingValue::TextValue),
        }
    }

    pub fn typed_value_of_json(self, declared_json: &serde_json::Value) -> Option<TypedSettingValue> {
        let declared_variant = match self.value_kind() {
            PreferenceValueKind::SwitchPreference => declared_json.as_bool()?.to_variant(),
            PreferenceValueKind::UnsignedCountPreference { .. } => {
                u32::try_from(declared_json.as_u64()?).ok()?.to_variant()
            }
            PreferenceValueKind::SignedCountPreference { .. }
            | PreferenceValueKind::NumberChoicePreference => {
                i32::try_from(declared_json.as_i64()?).ok()?.to_variant()
            }
            PreferenceValueKind::TextChoicePreference { .. }
            | PreferenceValueKind::ClockTimePreference => declared_json.as_str()?.to_variant(),
        };
        self.typed_value_of(&declared_variant)
    }

    pub fn variant_of(self, typed_value: &TypedSettingValue) -> Option<glib::Variant> {
        match (self.value_kind(), typed_value) {
            (
                PreferenceValueKind::SwitchPreference,
                TypedSettingValue::BooleanValue(switch_position),
            ) => Some(switch_position.is_switched_on().to_variant()),
            (
                PreferenceValueKind::UnsignedCountPreference { .. },
                TypedSettingValue::CountValue { count_value, .. },
            ) => u32::try_from(*count_value).ok().map(|count| count.to_variant()),
            (
                PreferenceValueKind::SignedCountPreference { .. },
                TypedSettingValue::CountValue { count_value, .. },
            ) => i32::try_from(*count_value).ok().map(|count| count.to_variant()),
            (
                PreferenceValueKind::TextChoicePreference { stored_texts },
                TypedSettingValue::ChoiceValue {
                    chosen_position, ..
                },
            ) => usize::try_from(*chosen_position)
                .ok()
                .and_then(|choice_index| stored_texts.get(choice_index))
                .map(|choice_text| choice_text.to_variant()),
            (
                PreferenceValueKind::NumberChoicePreference,
                TypedSettingValue::ChoiceValue {
                    chosen_position, ..
                },
            ) => i32::try_from(*chosen_position)
                .ok()
                .map(|chosen_number| chosen_number.to_variant()),
            (PreferenceValueKind::ClockTimePreference, TypedSettingValue::TextValue(time_text)) => {
                parse_schedule_time(time_text).map(|_clock_time| time_text.trim().to_variant())
            }
            (
                PreferenceValueKind::SwitchPreference
                | PreferenceValueKind::UnsignedCountPreference { .. }
                | PreferenceValueKind::SignedCountPreference { .. }
                | PreferenceValueKind::TextChoicePreference { .. }
                | PreferenceValueKind::NumberChoicePreference
                | PreferenceValueKind::ClockTimePreference,
                _mismatched_value,
            ) => None,
        }
    }

    pub fn accepts_value(self, typed_value: &TypedSettingValue) -> Result<(), WriteRefusal> {
        match (self.value_kind(), typed_value) {
            (PreferenceValueKind::ClockTimePreference, TypedSettingValue::TextValue(time_text))
                if parse_schedule_time(time_text).is_none() =>
            {
                Err(WriteRefusal::ValueNotParsed(
                    SettingParseFailure::NotAClockTime,
                ))
            }
            (_value_kind, _typed_value) => self
                .variant_of(typed_value)
                .map(|_stored_variant| ())
                .ok_or(WriteRefusal::ValueNotParsed(SettingParseFailure::UnknownOption)),
        }
    }

    pub fn read_value(self, stored_settings: &gio::Settings) -> Option<TypedSettingValue> {
        self.typed_value_of(&stored_settings.value(self.schema_key()))
    }

    pub fn store_value(
        self,
        stored_settings: &gio::Settings,
        typed_value: &TypedSettingValue,
    ) -> Result<(), PreferenceWriteFailure> {
        let stored_variant = self
            .variant_of(typed_value)
            .ok_or(PreferenceWriteFailure::ValueKindMismatch)?;
        stored_settings.set_value(self.schema_key(), &stored_variant)?;
        Ok(())
    }

    pub fn home_manager_line(self, typed_value: &TypedSettingValue) -> String {
        let nix_value = self
            .variant_of(typed_value)
            .map(|stored_variant| {
                stored_variant
                    .str()
                    .map(|stored_text| format!("\"{stored_text}\""))
                    .or_else(|| stored_variant.get::<bool>().map(|switched_on| switched_on.to_string()))
                    .unwrap_or_else(|| stored_variant.print(false).to_string())
            })
            .unwrap_or_default();
        format!(
            "services.lazaret.settings.\"{}\" = {nix_value};",
            self.schema_key()
        )
    }
}

#[cfg(test)]
mod preference_keys_tests {
    use super::*;

    #[test]
    fn every_preference_key_is_found_by_its_schema_name() {
        PREFERENCE_KEYS.into_iter().for_each(|preference_key| {
            assert_eq!(
                PreferenceKey::from_schema_key(preference_key.schema_key()),
                Some(preference_key)
            );
        });
    }

    #[test]
    fn stored_values_round_trip_through_their_typed_value() {
        [
            (PreferenceKey::CloseAction, "minimize".to_variant()),
            (PreferenceKey::NotificationsEnabled, false.to_variant()),
            (PreferenceKey::DeviceScanMaxGigabytes, 64_u32.to_variant()),
            (PreferenceKey::ScheduleDayOfMonth, 12_i32.to_variant()),
            (PreferenceKey::ScheduleDayOfWeek, 3_i32.to_variant()),
            (PreferenceKey::ScheduleTime, "03:30".to_variant()),
        ]
        .into_iter()
        .for_each(|(preference_key, stored_variant)| {
            let typed_value = preference_key
                .typed_value_of(&stored_variant)
                .expect("a stored value of the right type");
            assert_eq!(
                preference_key.variant_of(&typed_value),
                Some(stored_variant),
                "{preference_key:?}"
            );
        });
    }

    #[test]
    fn an_unreadable_clock_time_is_refused() {
        assert_eq!(
            PreferenceKey::ScheduleTime.accepts_value(&TypedSettingValue::TextValue(
                "25:00".to_owned()
            )),
            Err(WriteRefusal::ValueNotParsed(
                SettingParseFailure::NotAClockTime
            ))
        );
        assert_eq!(
            PreferenceKey::ScheduleTime
                .accepts_value(&TypedSettingValue::TextValue("02:15".to_owned())),
            Ok(())
        );
    }

    #[test]
    fn home_manager_declarations_read_like_stored_values() {
        assert_eq!(
            PreferenceKey::CloseAction.typed_value_of_json(&serde_json::json!("quit")),
            PreferenceKey::CloseAction.typed_value_of(&"quit".to_variant())
        );
        assert_eq!(
            PreferenceKey::NotificationsEnabled.typed_value_of_json(&serde_json::json!(true)),
            Some(TypedSettingValue::BooleanValue(SwitchPosition::SwitchedOn))
        );
        assert_eq!(
            PreferenceKey::CloseAction.home_manager_line(
                &PreferenceKey::CloseAction
                    .typed_value_of(&"quit".to_variant())
                    .expect("quit is a close action")
            ),
            "services.lazaret.settings.\"close-action\" = \"quit\";"
        );
        assert_eq!(
            PreferenceKey::NotificationsEnabled.home_manager_line(&TypedSettingValue::BooleanValue(
                SwitchPosition::SwitchedOff
            )),
            "services.lazaret.settings.\"notifications-enabled\" = false;"
        );
    }
}
