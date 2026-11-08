use relm4::{
    FactorySender, adw,
    adw::prelude::*,
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
};

use crate::{
    host_services::clamav_config::detection_options::DetectionOption,
    shared_widgets::engine_settings::config_setting_row::{
        RowChangeActivity, SettingAvailability, SettingLockReason, setting_lock_tooltip,
    },
    translated_messages,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DetectionSwitchState {
    pub detection_option: DetectionOption,
    pub switched_on: bool,
    pub switch_availability: SettingAvailability,
    pub change_activity: RowChangeActivity,
}

pub struct DetectionSwitchRow {
    switch_state: DetectionSwitchState,
}

#[derive(Debug)]
pub enum DetectionSwitchRowInput {
    SwitchStateChanged(DetectionSwitchState),
    ToggleRequested(bool),
}

#[derive(Debug)]
pub enum DetectionSwitchRowOutput {
    SwitchChangeRequested {
        detection_option: DetectionOption,
        switched_on: bool,
    },
}

fn option_title(detection_option: DetectionOption) -> String {
    match detection_option {
        DetectionOption::PotentiallyUnwantedPrograms => {
            translated_messages::widgets::detect_potentially_unwanted_programs()
        }
        DetectionOption::HeuristicAlerts => translated_messages::widgets::heuristic_detections(),
        DetectionOption::OfficeMacros => translated_messages::widgets::flag_office_files_macros(),
        DetectionOption::EncryptedContent => {
            translated_messages::widgets::flag_encrypted_archives_documents()
        }
        DetectionOption::ArchiveContents => translated_messages::widgets::look_inside_archives(),
        DetectionOption::OversizedFiles => translated_messages::widgets::flag_files_too_large(),
    }
}

fn option_description(detection_option: DetectionOption) -> String {
    match detection_option {
        DetectionOption::PotentiallyUnwantedPrograms => {
            translated_messages::widgets::adware_hacking_tools_remote()
        }
        DetectionOption::HeuristicAlerts => {
            translated_messages::widgets::suspicious_files_no_signature()
        }
        DetectionOption::OfficeMacros => translated_messages::widgets::macros_common_way_deliver(),
        DetectionOption::EncryptedContent => {
            translated_messages::widgets::clamav_cannot_look_inside()
        }
        DetectionOption::ArchiveContents => translated_messages::widgets::checks_files_inside_zip(),
        DetectionOption::OversizedFiles => {
            translated_messages::widgets::otherwise_files_above_size()
        }
    }
}

impl DetectionSwitchRow {
    fn availability_note(&self) -> String {
        match self.switch_state.switch_availability {
            SettingAvailability::ChangeableNow => String::new(),
            SettingAvailability::LockedUntilUnlocked => {
                translated_messages::widgets::unlock_change_setting()
            }
            SettingAvailability::NotAllowedAtRuntime => {
                translated_messages::widgets::administrator_not_allow_detection()
            }
        }
    }

    fn row_subtitle(&self) -> String {
        [
            option_description(self.switch_state.detection_option),
            self.availability_note(),
        ]
        .into_iter()
        .filter(|subtitle_part| !subtitle_part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
    }

    const fn lock_reason(&self) -> Option<SettingLockReason> {
        match self.switch_state.switch_availability {
            SettingAvailability::ChangeableNow => None,
            SettingAvailability::LockedUntilUnlocked => Some(SettingLockReason::UnlockRequired),
            SettingAvailability::NotAllowedAtRuntime => {
                Some(SettingLockReason::NotAllowedAtRuntime)
            }
        }
    }

    fn lock_tooltip(&self) -> String {
        self.lock_reason()
            .map(setting_lock_tooltip)
            .unwrap_or_default()
    }

    fn accepts_toggles(&self) -> bool {
        self.switch_state.switch_availability == SettingAvailability::ChangeableNow
            && self.switch_state.change_activity.accepts_changes()
    }
}

#[relm4::factory(pub)]
impl FactoryComponent for DetectionSwitchRow {
    type Init = DetectionSwitchState;
    type Input = DetectionSwitchRowInput;
    type Output = DetectionSwitchRowOutput;
    type CommandOutput = ();
    type ParentWidget = adw::PreferencesGroup;

    view! {
        adw::SwitchRow {
            set_title: &option_title(self.switch_state.detection_option),
            set_subtitle_lines: 0,
            #[watch]
            set_subtitle: &self.row_subtitle(),
            #[watch]
            set_sensitive: self.accepts_toggles(),
            #[watch]
            #[block_signal(switch_toggled_handler)]
            set_active: self.switch_state.switched_on,
            connect_active_notify[sender] => move |toggled_row| {
                sender.input(DetectionSwitchRowInput::ToggleRequested(toggled_row.is_active()));
            } @switch_toggled_handler,

            add_prefix = &gtk::Image {
                set_icon_name: Some("changes-prevent-symbolic"),
                set_valign: gtk::Align::Center,
                add_css_class: "dim-label",
                #[watch]
                set_visible: self.lock_reason().is_some(),
                #[watch]
                set_tooltip_text: Some(&self.lock_tooltip()),
            },
        }
    }

    fn init_model(
        switch_state: DetectionSwitchState,
        _row_index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self { switch_state }
    }

    fn update(&mut self, row_input: DetectionSwitchRowInput, sender: FactorySender<Self>) {
        match row_input {
            DetectionSwitchRowInput::SwitchStateChanged(switch_state) => {
                self.switch_state = switch_state;
            }
            DetectionSwitchRowInput::ToggleRequested(switched_on) => {
                if switched_on != self.switch_state.switched_on {
                    let _page_closed =
                        sender.output(DetectionSwitchRowOutput::SwitchChangeRequested {
                            detection_option: self.switch_state.detection_option,
                            switched_on,
                        });
                }
            }
        }
    }
}
