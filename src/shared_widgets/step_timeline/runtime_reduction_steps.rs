use crate::{
    host_services::{
        clamav_admin_service::live_settings_client::StoredOverridesFailure,
        desktop::locale_formatting::MomentFormatters,
        protection_evidence::host_protection_evidence::{
            OverriddenSetting, OverridesChangeTime, OverridesReadFailure, ReducedSetting,
            RuntimeReductions,
        },
    },
    shared_widgets::step_timeline::{
        step_evidence_builders::{CheckedCondition, evidence_row, plain_rows},
        timeline_step_contents::{
            SettledStep, StepActionButton, StepActionSlot, StepEvidenceRow, TimelineStep,
        },
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

const ON_ACCESS_KEY_PREFIX: &str = "OnAccess";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReducedSettingGroup {
    OnAccessSettings,
    EngineSettings,
}

impl ReducedSettingGroup {
    pub fn includes_setting(self, setting_key: &str) -> bool {
        let on_access_setting = setting_key.starts_with(ON_ACCESS_KEY_PREFIX);
        match self {
            Self::OnAccessSettings => on_access_setting,
            Self::EngineSettings => !on_access_setting,
        }
    }
}

fn reduced_settings_status(
    message_catalog: &MessageCatalog,
    moment_formatters: &MomentFormatters,
    reduced_settings: &[&ReducedSetting],
    change_time: OverridesChangeTime,
) -> String {
    let setting_count = u32::try_from(reduced_settings.len()).unwrap_or(u32::MAX);
    match change_time {
        OverridesChangeTime::ChangedAt { changed_at } => {
            translated_messages::widgets::count_settings_reduced_since(
                message_catalog,
                setting_count,
                moment_formatters.exact_moment_text(changed_at),
            )
        }
        OverridesChangeTime::ChangeTimeUnknown => {
            translated_messages::widgets::count_settings_reduced_unknown_time(
                message_catalog,
                setting_count,
            )
        }
    }
}

fn override_change_time_row(
    message_catalog: &MessageCatalog,
    moment_formatters: &MomentFormatters,
    change_time: OverridesChangeTime,
) -> StepEvidenceRow {
    evidence_row(
        translated_messages::widgets::changed_label(message_catalog),
        match change_time {
            OverridesChangeTime::ChangedAt { changed_at } => {
                moment_formatters.exact_moment_text(changed_at)
            }
            OverridesChangeTime::ChangeTimeUnknown => {
                translated_messages::widgets::change_time_unknown_override(message_catalog)
            }
        },
    )
}

fn overrides_failure_text(
    message_catalog: &MessageCatalog,
    read_failure: OverridesReadFailure,
) -> String {
    match read_failure {
        OverridesReadFailure::StoredOverridesUnusable(
            StoredOverridesFailure::OverridesFileUnreadable { failure_kind },
        ) => translated_messages::widgets::reduction_unknown_unread(message_catalog, failure_kind),
        OverridesReadFailure::StoredOverridesUnusable(
            StoredOverridesFailure::OverridesFileInvalid,
        ) => translated_messages::widgets::reduction_unknown_file(message_catalog),
        OverridesReadFailure::ReadingTaskInterrupted => {
            translated_messages::widgets::reduction_unknown_reading(message_catalog)
        }
    }
}

fn reduced_settings_in_group(
    reduced_settings: &[ReducedSetting],
    setting_group: ReducedSettingGroup,
) -> Vec<&ReducedSetting> {
    reduced_settings
        .iter()
        .filter(|reduced_setting| setting_group.includes_setting(&reduced_setting.setting_key))
        .collect()
}

fn overridden_settings_in_group(
    overridden_settings: &[OverriddenSetting],
    setting_group: ReducedSettingGroup,
) -> Vec<&OverriddenSetting> {
    overridden_settings
        .iter()
        .filter(|overridden_setting| {
            setting_group.includes_setting(&overridden_setting.setting_key)
        })
        .collect()
}

pub fn runtime_reduction_condition(
    runtime_reductions: &RuntimeReductions,
    setting_group: ReducedSettingGroup,
) -> CheckedCondition {
    match runtime_reductions {
        RuntimeReductions::NothingReduced => CheckedCondition::ConditionMet,
        RuntimeReductions::OverridesUnreadable(_) => CheckedCondition::EvidenceUnknown,
        RuntimeReductions::SettingsReduced {
            reduced_settings, ..
        } => {
            if reduced_settings_in_group(reduced_settings, setting_group).is_empty() {
                CheckedCondition::ConditionMet
            } else {
                CheckedCondition::ReviewSuggested
            }
        }
        RuntimeReductions::BaseConfigUnreadable {
            overridden_settings,
            ..
        } => {
            if overridden_settings_in_group(overridden_settings, setting_group).is_empty() {
                CheckedCondition::ConditionMet
            } else {
                CheckedCondition::EvidenceUnknown
            }
        }
    }
}

pub fn runtime_reduction_step<StepAction: StepActionButton>(
    message_catalog: &MessageCatalog,
    moment_formatters: &MomentFormatters,
    runtime_reductions: &RuntimeReductions,
    setting_group: ReducedSettingGroup,
    header_action: StepActionSlot<StepAction>,
) -> Option<TimelineStep<StepAction>> {
    let node_state = runtime_reduction_condition(runtime_reductions, setting_group).node_state();
    match runtime_reductions {
        RuntimeReductions::NothingReduced => None,
        RuntimeReductions::OverridesUnreadable(read_failure) => Some(TimelineStep::settled(
            translated_messages::widgets::settings_changed_runtime(message_catalog),
            translated_messages::widgets::administrator_can_change(message_catalog),
            overrides_failure_text(message_catalog, *read_failure),
            SettledStep {
                header_action,
                ..SettledStep::without_offers(node_state)
            },
        )),
        RuntimeReductions::SettingsReduced {
            change_time,
            reduced_settings,
        } => {
            let grouped_settings = reduced_settings_in_group(reduced_settings, setting_group);
            (!grouped_settings.is_empty()).then(|| {
                TimelineStep::settled(
                    translated_messages::widgets::protection_reduced_purpose(message_catalog),
                    translated_messages::widgets::administrator_changed_runtime(message_catalog),
                    reduced_settings_status(
                        message_catalog,
                        moment_formatters,
                        &grouped_settings,
                        *change_time,
                    ),
                    SettledStep {
                        evidence_rows: plain_rows(grouped_settings.iter().map(|reduced_setting| {
                            evidence_row(
                                reduced_setting.setting_key.clone(),
                                translated_messages::widgets::base_configuration_changed_now(
                                    message_catalog,
                                    &reduced_setting.base_value,
                                    &reduced_setting.changed_value,
                                ),
                            )
                        })),
                        header_action,
                        ..SettledStep::without_offers(node_state)
                    },
                )
            })
        }
        RuntimeReductions::BaseConfigUnreadable {
            change_time,
            overridden_settings,
        } => {
            let grouped_settings = overridden_settings_in_group(overridden_settings, setting_group);
            (!grouped_settings.is_empty()).then(|| {
                TimelineStep::settled(
                    translated_messages::widgets::settings_changed_runtime(message_catalog),
                    translated_messages::widgets::administrator_changed_clamd(message_catalog),
                    translated_messages::widgets::settings_changed_runtime_clamd(message_catalog),
                    SettledStep {
                        evidence_rows: plain_rows(
                            grouped_settings
                                .iter()
                                .map(|overridden_setting| {
                                    evidence_row(
                                        overridden_setting.setting_key.clone(),
                                        translated_messages::widgets::changed_now(
                                            message_catalog,
                                            &overridden_setting.changed_value,
                                        ),
                                    )
                                })
                                .chain(std::iter::once(override_change_time_row(
                                    message_catalog,
                                    moment_formatters,
                                    *change_time,
                                ))),
                        ),
                        header_action,
                        ..SettledStep::without_offers(node_state)
                    },
                )
            })
        }
    }
}

#[cfg(test)]
mod runtime_reduction_steps_tests {
    use super::*;
    use crate::{
        host_services::protection_evidence::protection_evidence_fixtures::english_formatters,
        shared_widgets::step_timeline::timeline_step_contents::{
            ActionConsequence, OfferedAction, StepNodeState,
        },
    };

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum ReviewStepAction {
        ReviewChangedSettings,
    }

    impl StepActionButton for ReviewStepAction {
        fn button_label(&self, _message_catalog: &MessageCatalog) -> String {
            match self {
                Self::ReviewChangedSettings => "Review".to_owned(),
            }
        }

        fn action_consequence(&self) -> ActionConsequence {
            match self {
                Self::ReviewChangedSettings => ActionConsequence::OrdinaryAction,
            }
        }
    }

    fn reduced_setting(setting_key: &str) -> ReducedSetting {
        ReducedSetting {
            setting_key: setting_key.to_owned(),
            base_value: "yes".to_owned(),
            changed_value: "no".to_owned(),
        }
    }

    fn mixed_reductions() -> RuntimeReductions {
        RuntimeReductions::SettingsReduced {
            change_time: OverridesChangeTime::ChangeTimeUnknown,
            reduced_settings: vec![
                reduced_setting("OnAccessPrevention"),
                reduced_setting("DetectPUA"),
            ],
        }
    }

    fn reduction_rows(
        message_catalog: &MessageCatalog,
        moment_formatters: &MomentFormatters,
        setting_group: ReducedSettingGroup,
    ) -> Vec<String> {
        runtime_reduction_step::<ReviewStepAction>(
            message_catalog,
            moment_formatters,
            &mixed_reductions(),
            setting_group,
            StepActionSlot::NoAction,
        )
        .map(|reduction_step| {
            reduction_step
                .shown_evidence_rows()
                .into_iter()
                .map(|step_evidence| step_evidence.evidence_name.clone())
                .collect()
        })
        .unwrap_or_default()
    }

    #[test]
    fn each_group_lists_only_its_own_reduced_keys() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let moment_formatters = &english_formatters();
        assert_eq!(
            reduction_rows(
                message_catalog,
                moment_formatters,
                ReducedSettingGroup::OnAccessSettings
            ),
            vec!["OnAccessPrevention"]
        );
        assert_eq!(
            reduction_rows(
                message_catalog,
                moment_formatters,
                ReducedSettingGroup::EngineSettings
            ),
            vec!["DetectPUA"]
        );
    }

    #[test]
    fn a_group_without_reduced_keys_adds_no_step_and_stays_met() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let moment_formatters = &english_formatters();
        let on_access_only = RuntimeReductions::SettingsReduced {
            change_time: OverridesChangeTime::ChangeTimeUnknown,
            reduced_settings: vec![reduced_setting("OnAccessPrevention")],
        };
        assert_eq!(
            runtime_reduction_step::<ReviewStepAction>(
                message_catalog,
                moment_formatters,
                &on_access_only,
                ReducedSettingGroup::EngineSettings,
                StepActionSlot::NoAction,
            ),
            None
        );
        assert_eq!(
            runtime_reduction_condition(&on_access_only, ReducedSettingGroup::EngineSettings),
            CheckedCondition::ConditionMet
        );
    }

    #[test]
    fn unreadable_overrides_concern_every_group() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let moment_formatters = &english_formatters();
        let unreadable_overrides =
            RuntimeReductions::OverridesUnreadable(OverridesReadFailure::ReadingTaskInterrupted);
        [
            ReducedSettingGroup::EngineSettings,
            ReducedSettingGroup::OnAccessSettings,
        ]
        .into_iter()
        .for_each(|setting_group| {
            let reduction_step = runtime_reduction_step(
                message_catalog,
                moment_formatters,
                &unreadable_overrides,
                setting_group,
                StepActionSlot::ActionOffered(OfferedAction::plain(
                    ReviewStepAction::ReviewChangedSettings,
                )),
            )
            .unwrap();
            assert_eq!(reduction_step.node_state(), StepNodeState::NeedsReview);
            assert!(reduction_step.header_offer().is_some());
        });
    }
}
