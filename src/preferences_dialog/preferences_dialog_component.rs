use relm4::{
    Component, ComponentParts, ComponentSender, adw,
    adw::prelude::*,
    prelude::{ComponentController, Controller, FactoryVecDeque},
};

use crate::{
    host_services::{
        clamav_config::typed_setting_values::TypedSettingValue,
        desktop::{
            file_manager_actions::SUPPORTED_FILE_MANAGERS,
            flatpak_sandbox::{request_background_autostart, running_in_flatpak},
            preference_keys::PreferenceKey,
        },
        threat_intel::service_key_storage::ThreatIntelService,
    },
    preferences_dialog::{
        file_manager_row::FileManagerRow,
        service_key_section::{ServiceKeySection, ServiceKeySectionOutput},
    },
    shared_widgets::setting_row::preference_rows_group::{
        PreferenceApplyMode, PreferenceRowSpec, PreferenceRowsGroup, PreferenceRowsInit,
        PreferenceRowsOutput,
    },
    translated_messages,
};

pub const ONLINE_SERVICES_PAGE_NAME: &str = "online-services";

pub struct PreferencesDialog {
    window_behavior_rows: Controller<PreferenceRowsGroup>,
    file_manager_rows: FactoryVecDeque<FileManagerRow>,
    abuse_ch_section: Controller<ServiceKeySection>,
    virustotal_section: Controller<ServiceKeySection>,
}

#[derive(Debug)]
pub enum PreferencesDialogInput {
    PreferenceStored {
        preference_key: PreferenceKey,
        stored_value: TypedSettingValue,
    },
    StoredServiceKeyChanged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreferencesDialogOutput {
    StoredServiceKeyChanged,
}

const fn dialog_input_for_key_output(
    key_output: ServiceKeySectionOutput,
) -> PreferencesDialogInput {
    match key_output {
        ServiceKeySectionOutput::StoredKeyChanged => {
            PreferencesDialogInput::StoredServiceKeyChanged
        }
    }
}

#[derive(Debug)]
pub enum PreferencesDialogCommand {
    AutostartRequestFinished(zbus::Result<()>),
}

fn dialog_input_for_preference_output(
    preference_output: PreferenceRowsOutput,
) -> PreferencesDialogInput {
    match preference_output {
        PreferenceRowsOutput::PreferenceStored {
            preference_key,
            stored_value,
        } => PreferencesDialogInput::PreferenceStored {
            preference_key,
            stored_value,
        },
    }
}

fn window_behavior_row_specs() -> Vec<PreferenceRowSpec> {
    vec![
        PreferenceRowSpec {
            preference_key: PreferenceKey::CloseAction,
            setting_title: translated_messages::preferences::closing_window_title(),
            setting_description: String::new(),
        },
        PreferenceRowSpec {
            preference_key: PreferenceKey::StartInBackground,
            setting_title: translated_messages::preferences::start_background_title(),
            setting_description:
                translated_messages::preferences::launch_background_without_subtitle(),
        },
        PreferenceRowSpec {
            preference_key: PreferenceKey::NotificationsEnabled,
            setting_title: translated_messages::preferences::desktop_notifications_title(),
            setting_description:
                translated_messages::preferences::notify_detections_finished_subtitle(),
        },
    ]
}

#[relm4::component(pub)]
impl Component for PreferencesDialog {
    type Init = ();
    type Input = PreferencesDialogInput;
    type Output = PreferencesDialogOutput;
    type CommandOutput = PreferencesDialogCommand;

    view! {
        adw::PreferencesDialog {
            set_title: &translated_messages::preferences::dialog_title(),
            set_search_enabled: true,

            add = &adw::PreferencesPage {
                set_title: &translated_messages::preferences::general_title(),
                set_icon_name: Some("preferences-system-symbolic"),

                #[local_ref]
                window_behavior_group -> adw::PreferencesGroup {
                    set_title: &translated_messages::preferences::window_behavior_title(),
                    set_description: Some(&translated_messages::preferences::detection_alerts_work_description()),
                },

                #[local_ref]
                file_manager_group -> adw::PreferencesGroup {
                    set_title: &translated_messages::preferences::file_manager_integration_title(),
                    set_description: Some(&translated_messages::preferences::add_context_menu_description()),
                    set_visible: !running_in_flatpak(),
                },
            },

            add = &adw::PreferencesPage {
                set_name: Some(ONLINE_SERVICES_PAGE_NAME),
                set_title: &translated_messages::preferences::online_services_title(),
                set_icon_name: Some("network-server-symbolic"),
                set_description: &translated_messages::preferences::online_services_summary(),

                adw::PreferencesGroup {
                    set_title: "CIRCL hashlookup",
                    set_description: Some(&translated_messages::preferences::circl_receives_summary()),
                },

                #[local_ref]
                abuse_ch_group -> adw::PreferencesGroup {},

                #[local_ref]
                virustotal_group -> adw::PreferencesGroup {},

                adw::PreferencesGroup {
                    set_title: "ClamAV (Cisco Talos)",
                    set_description: Some(&translated_messages::preferences::talos_receives_summary()),
                },
            },

        }
    }

    fn init(
        _dialog_init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut file_manager_rows = FactoryVecDeque::builder()
            .launch(adw::PreferencesGroup::default())
            .detach();
        if !running_in_flatpak() {
            let mut row_guard = file_manager_rows.guard();
            SUPPORTED_FILE_MANAGERS
                .into_iter()
                .for_each(|file_manager| {
                    row_guard.push_back(file_manager);
                });
        }
        let model = Self {
            window_behavior_rows: PreferenceRowsGroup::builder()
                .launch(PreferenceRowsInit {
                    row_specs: window_behavior_row_specs(),
                    apply_mode: PreferenceApplyMode::AppliesImmediately,
                })
                .forward(sender.input_sender(), dialog_input_for_preference_output),
            file_manager_rows,
            abuse_ch_section: ServiceKeySection::builder()
                .launch(ThreatIntelService::AbuseChService)
                .forward(sender.input_sender(), dialog_input_for_key_output),
            virustotal_section: ServiceKeySection::builder()
                .launch(ThreatIntelService::VirusTotalService)
                .forward(sender.input_sender(), dialog_input_for_key_output),
        };
        let window_behavior_group = model.window_behavior_rows.widget();
        let file_manager_group = model.file_manager_rows.widget();
        let abuse_ch_group = model.abuse_ch_section.widget();
        let virustotal_group = model.virustotal_section.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        dialog_input: PreferencesDialogInput,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match dialog_input {
            PreferencesDialogInput::PreferenceStored {
                preference_key: PreferenceKey::StartInBackground,
                stored_value: TypedSettingValue::BooleanValue(switch_position),
            } => {
                if running_in_flatpak() {
                    let start_in_background = switch_position.is_switched_on();
                    sender.oneshot_command(async move {
                        PreferencesDialogCommand::AutostartRequestFinished(
                            request_background_autostart(start_in_background).await,
                        )
                    });
                }
            }
            PreferencesDialogInput::PreferenceStored { .. } => {}
            PreferencesDialogInput::StoredServiceKeyChanged => {
                let _shell_closed = sender.output(PreferencesDialogOutput::StoredServiceKeyChanged);
            }
        }
    }

    fn update_cmd(
        &mut self,
        dialog_command: PreferencesDialogCommand,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match dialog_command {
            PreferencesDialogCommand::AutostartRequestFinished(Ok(())) => {}
            PreferencesDialogCommand::AutostartRequestFinished(Err(portal_failure)) => {
                eprintln!("lazaret: the Background portal refused autostart: {portal_failure}");
            }
        }
    }
}
