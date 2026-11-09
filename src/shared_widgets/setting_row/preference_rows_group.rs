use std::collections::HashMap;

use relm4::{
    Component, ComponentParts, ComponentSender, adw, adw::prelude::*, gtk, gtk::gio,
    prelude::FactoryVecDeque,
};

use crate::{
    host_services::{
        clamav_config::typed_setting_values::TypedSettingValue,
        desktop::{
            failure_journal_log::log_failure_to_journal, preference_keys::PreferenceKey,
            preference_sources::preference_source, stored_settings::application_settings,
            stored_settings_watch::forward_setting_changes,
        },
    },
    shared_widgets::setting_row::{
        setting_row_state::{
            FollowUpAction, PreferenceApplyStage, RowChangeActivity, SettingLabel, SettingRowState,
            SettingSubject, ShownSettingValue,
        },
        setting_row_widget::{SettingRow, SettingRowInput, SettingRowOutput},
    },
    translated_messages,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreferenceRowSpec {
    pub preference_key: PreferenceKey,
    pub setting_title: String,
    pub setting_description: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreferenceApplyMode {
    AppliesImmediately,
    AppliedWithGroup,
}

pub struct PreferenceRowsInit {
    pub row_specs: Vec<PreferenceRowSpec>,
    pub apply_mode: PreferenceApplyMode,
}

pub struct PreferenceRowsGroup {
    stored_settings: gio::Settings,
    row_specs: Vec<PreferenceRowSpec>,
    apply_mode: PreferenceApplyMode,
    applied_values: HashMap<PreferenceKey, TypedSettingValue>,
    setting_rows: FactoryVecDeque<SettingRow>,
}

#[derive(Debug)]
pub enum PreferenceRowsInput {
    SettingRowReported(SettingRowOutput),
    StoredPreferenceChanged,
    GroupApplied,
}

#[derive(Debug)]
pub enum PreferenceRowsOutput {
    PreferenceStored {
        preference_key: PreferenceKey,
        stored_value: TypedSettingValue,
    },
}

pub fn apply_stage_for(
    apply_mode: PreferenceApplyMode,
    current_value: Option<&TypedSettingValue>,
    applied_value: Option<&TypedSettingValue>,
) -> PreferenceApplyStage {
    match apply_mode {
        PreferenceApplyMode::AppliesImmediately => PreferenceApplyStage::AppliesImmediately,
        PreferenceApplyMode::AppliedWithGroup => {
            if current_value == applied_value {
                PreferenceApplyStage::AppliedWithGroup
            } else {
                PreferenceApplyStage::WaitsForGroupApply
            }
        }
    }
}

fn show_toast_near(anchor_widget: &impl IsA<gtk::Widget>, toast_title: &str) {
    let shown_toast = adw::Toast::builder()
        .title(toast_title)
        .use_markup(false)
        .build();
    if let Some(toast_overlay) = anchor_widget
        .ancestor(adw::ToastOverlay::static_type())
        .and_then(|overlay_widget| overlay_widget.downcast::<adw::ToastOverlay>().ok())
    {
        toast_overlay.add_toast(shown_toast);
    } else if let Some(preferences_dialog) = anchor_widget
        .ancestor(adw::PreferencesDialog::static_type())
        .and_then(|dialog_widget| dialog_widget.downcast::<adw::PreferencesDialog>().ok())
    {
        preferences_dialog.add_toast(shown_toast);
    }
}

impl PreferenceRowsGroup {
    fn current_values(&self) -> HashMap<PreferenceKey, TypedSettingValue> {
        self.row_specs
            .iter()
            .filter_map(|row_spec| {
                row_spec
                    .preference_key
                    .read_value(&self.stored_settings)
                    .map(|current_value| (row_spec.preference_key, current_value))
            })
            .collect()
    }

    fn row_state_for(&self, row_spec: &PreferenceRowSpec) -> Option<SettingRowState> {
        let current_value = row_spec.preference_key.read_value(&self.stored_settings)?;
        Some(SettingRowState {
            setting_label: SettingLabel::DescribedSetting {
                title: row_spec.setting_title.clone(),
                description: row_spec.setting_description.clone(),
            },
            setting_subject: SettingSubject::Preference {
                preference_key: row_spec.preference_key,
                preference_source: preference_source(
                    &self.stored_settings,
                    row_spec.preference_key,
                ),
                apply_stage: apply_stage_for(
                    self.apply_mode,
                    Some(&current_value),
                    self.applied_values.get(&row_spec.preference_key),
                ),
            },
            shown_value: ShownSettingValue::ReadableValue(current_value),
            change_activity: RowChangeActivity::ChangesAllowed,
        })
    }

    fn row_states(&self) -> Vec<SettingRowState> {
        self.row_specs
            .iter()
            .filter_map(|row_spec| self.row_state_for(row_spec))
            .collect()
    }

    fn refresh_rows(&mut self) {
        let row_states = self.row_states();
        if row_states.len() == self.setting_rows.len() {
            row_states
                .into_iter()
                .enumerate()
                .for_each(|(row_position, row_state)| {
                    self.setting_rows.send(
                        row_position,
                        SettingRowInput::StateReplaced(Box::new(row_state)),
                    );
                });
        } else {
            let mut row_guard = self.setting_rows.guard();
            row_guard.clear();
            row_states.into_iter().for_each(|row_state| {
                row_guard.push_back(row_state);
            });
        }
    }

    fn store_preference(
        &self,
        preference_key: PreferenceKey,
        stored_value: TypedSettingValue,
        sender: &ComponentSender<Self>,
        root: &adw::PreferencesGroup,
    ) {
        match preference_key.store_value(&self.stored_settings, &stored_value) {
            Ok(()) => {
                let _page_closed = sender.output(PreferenceRowsOutput::PreferenceStored {
                    preference_key,
                    stored_value,
                });
            }
            Err(write_failure) => {
                log_failure_to_journal("storing a preference failed", &write_failure);
                show_toast_near(
                    root,
                    &translated_messages::widgets::preference_not_stored(write_failure),
                );
            }
        }
    }
}

#[relm4::component(pub)]
impl Component for PreferenceRowsGroup {
    type Init = PreferenceRowsInit;
    type Input = PreferenceRowsInput;
    type Output = PreferenceRowsOutput;
    type CommandOutput = ();

    view! {
        adw::PreferencesGroup {}
    }

    fn init(
        rows_init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let stored_settings = application_settings();
        let watched_keys = rows_init
            .row_specs
            .iter()
            .map(|row_spec| row_spec.preference_key.schema_key())
            .collect::<Vec<_>>();
        forward_setting_changes(
            &stored_settings,
            &watched_keys,
            sender.input_sender(),
            |_changed_settings, _changed_key| PreferenceRowsInput::StoredPreferenceChanged,
        );
        let setting_rows = FactoryVecDeque::builder().launch(root.clone()).forward(
            sender.input_sender(),
            PreferenceRowsInput::SettingRowReported,
        );
        let mut model = Self {
            stored_settings,
            row_specs: rows_init.row_specs,
            apply_mode: rows_init.apply_mode,
            applied_values: HashMap::new(),
            setting_rows,
        };
        model.applied_values = model.current_values();
        model.refresh_rows();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        rows_input: Self::Input,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match rows_input {
            PreferenceRowsInput::SettingRowReported(SettingRowOutput::ValueChangeRequested {
                setting_subject: SettingSubject::Preference { preference_key, .. },
                new_value,
            }) => self.store_preference(preference_key, new_value, &sender, root),
            PreferenceRowsInput::SettingRowReported(SettingRowOutput::FollowUpChosen {
                setting_subject: SettingSubject::Preference { preference_key, .. },
                follow_up_action: FollowUpAction::ResetToDefault,
                ..
            }) => self.stored_settings.reset(preference_key.schema_key()),
            PreferenceRowsInput::SettingRowReported(SettingRowOutput::FollowUpChosen {
                setting_subject: SettingSubject::Preference { .. },
                follow_up_action:
                    FollowUpAction::RevertRuntimeChange | FollowUpAction::CopyConfigurationLine,
                ..
            }) => {}
            PreferenceRowsInput::SettingRowReported(
                SettingRowOutput::ValueChangeRequested {
                    setting_subject: SettingSubject::ClamavSetting { .. },
                    ..
                }
                | SettingRowOutput::FollowUpChosen {
                    setting_subject: SettingSubject::ClamavSetting { .. },
                    ..
                },
            ) => {}
            PreferenceRowsInput::StoredPreferenceChanged => self.refresh_rows(),
            PreferenceRowsInput::GroupApplied => {
                self.applied_values = self.current_values();
                self.refresh_rows();
            }
        }
    }
}

#[cfg(test)]
mod preference_rows_group_tests {
    use super::*;
    use crate::host_services::clamav_config::typed_setting_values::SwitchPosition;

    const fn switch_value(switched_on: bool) -> TypedSettingValue {
        TypedSettingValue::BooleanValue(SwitchPosition::from_switch_active(switched_on))
    }

    #[test]
    fn immediate_preferences_never_wait_for_an_apply() {
        assert_eq!(
            apply_stage_for(
                PreferenceApplyMode::AppliesImmediately,
                Some(&switch_value(true)),
                Some(&switch_value(false))
            ),
            PreferenceApplyStage::AppliesImmediately
        );
    }

    #[test]
    fn grouped_preferences_wait_until_their_value_is_applied() {
        assert_eq!(
            apply_stage_for(
                PreferenceApplyMode::AppliedWithGroup,
                Some(&switch_value(true)),
                Some(&switch_value(false))
            ),
            PreferenceApplyStage::WaitsForGroupApply
        );
        assert_eq!(
            apply_stage_for(
                PreferenceApplyMode::AppliedWithGroup,
                Some(&switch_value(true)),
                Some(&switch_value(true))
            ),
            PreferenceApplyStage::AppliedWithGroup
        );
    }
}
