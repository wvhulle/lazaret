use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, adw,
    adw::prelude::*, gtk,
};

use crate::{
    host_services::desktop::preference_keys::PreferenceKey,
    shared_widgets::setting_row::preference_rows_group::{
        PreferenceApplyMode, PreferenceRowSpec, PreferenceRowsGroup, PreferenceRowsInit,
    },
    translated_messages,
};

pub struct ScanOptionsSection {
    option_rows: Controller<PreferenceRowsGroup>,
}

fn scan_option_row_specs() -> Vec<PreferenceRowSpec> {
    vec![
        PreferenceRowSpec {
            preference_key: PreferenceKey::ScanBackend,
            setting_title: translated_messages::scans::backend_title(),
            setting_description: translated_messages::scans::automatic_uses_daemon_subtitle(),
        },
        PreferenceRowSpec {
            preference_key: PreferenceKey::ShowLiveProgress,
            setting_title: translated_messages::scans::show_live_scan_title(),
            setting_description: translated_messages::scans::display_files_scanned_subtitle(),
        },
    ]
}

#[relm4::component(pub)]
impl Component for ScanOptionsSection {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 18,
            set_margin_top: 12,

            #[local_ref]
            option_group -> adw::PreferencesGroup {
                set_title: &translated_messages::scans::options_title(),
            },
        }
    }

    fn init(
        _section_init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            option_rows: PreferenceRowsGroup::builder()
                .launch(PreferenceRowsInit {
                    row_specs: scan_option_row_specs(),
                    apply_mode: PreferenceApplyMode::AppliesImmediately,
                })
                .detach(),
        };
        let option_group = model.option_rows.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
