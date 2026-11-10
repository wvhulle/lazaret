use std::rc::Rc;

use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, adw,
    adw::prelude::*, gtk::gio,
};

use crate::{
    host_services::desktop::{
        locale_formatting::MomentFormatters, preference_keys::PreferenceKey,
        stored_settings::application_settings, stored_settings_watch::forward_setting_changes,
    },
    scans::removable_drives_page::drive_mount_checks::{
        DEVICE_SCAN_SETTING_KEYS, DeviceAutoScanSettings,
    },
    shared_widgets::setting_row::preference_rows_group::{
        PreferenceApplyMode, PreferenceRowSpec, PreferenceRowsGroup, PreferenceRowsInit,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

pub struct DeviceAutoScanSection {
    pub message_catalog: Rc<MessageCatalog>,
    stored_settings: gio::Settings,
    device_scan_rows: Controller<PreferenceRowsGroup>,
}

#[derive(Debug)]
pub enum DeviceAutoScanSectionInput {
    StoredSettingChanged,
}

#[derive(Debug)]
pub enum DeviceAutoScanSectionOutput {
    SettingsChanged(DeviceAutoScanSettings),
}

fn device_scan_row_specs(message_catalog: &MessageCatalog) -> Vec<PreferenceRowSpec> {
    vec![
        PreferenceRowSpec {
            preference_key: PreferenceKey::DeviceScanEnabled,
            setting_title: translated_messages::scans::enable_device_auto_title(message_catalog),
            setting_description: String::new(),
        },
        PreferenceRowSpec {
            preference_key: PreferenceKey::DeviceScanMaxGigabytes,
            setting_title: translated_messages::scans::max_device_size_title(message_catalog),
            setting_description: translated_messages::scans::no_size_limit_subtitle(
                message_catalog,
            ),
        },
        PreferenceRowSpec {
            preference_key: PreferenceKey::DeviceScanSkipOnBattery,
            setting_title: translated_messages::scans::skip_battery_device_title(message_catalog),
            setting_description: String::new(),
        },
    ]
}

pub struct DeviceAutoScanSectionInit {
    pub message_catalog: Rc<MessageCatalog>,
    pub moment_formatters: Rc<MomentFormatters>,
}

#[relm4::component(pub)]
impl Component for DeviceAutoScanSection {
    type Init = DeviceAutoScanSectionInit;
    type Input = DeviceAutoScanSectionInput;
    type Output = DeviceAutoScanSectionOutput;
    type CommandOutput = ();

    view! {
        adw::PreferencesGroup {
            #[local_ref]
            device_scan_group -> adw::PreferencesGroup {
                set_title: &translated_messages::scans::device_auto_scan_title(&model.message_catalog),
                set_description: Some(&translated_messages::scans::automatically_scan_removable_description(&model.message_catalog)),
            },
        }
    }

    fn init(
        DeviceAutoScanSectionInit {
            message_catalog,
            moment_formatters,
        }: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let stored_settings = application_settings();
        forward_setting_changes(
            &stored_settings,
            &DEVICE_SCAN_SETTING_KEYS,
            sender.input_sender(),
            |_changed_settings, _changed_key| DeviceAutoScanSectionInput::StoredSettingChanged,
        );
        let model = Self {
            message_catalog: Rc::clone(&message_catalog),
            stored_settings,
            device_scan_rows: PreferenceRowsGroup::builder()
                .launch(PreferenceRowsInit {
                    message_catalog: Rc::clone(&message_catalog),
                    moment_formatters,
                    row_specs: device_scan_row_specs(&message_catalog),
                    apply_mode: PreferenceApplyMode::AppliesImmediately,
                })
                .detach(),
        };
        let device_scan_group = model.device_scan_rows.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        section_input: DeviceAutoScanSectionInput,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match section_input {
            DeviceAutoScanSectionInput::StoredSettingChanged => {
                let _page_closed = sender.output(DeviceAutoScanSectionOutput::SettingsChanged(
                    DeviceAutoScanSettings::read_from(&self.stored_settings),
                ));
            }
        }
    }
}
