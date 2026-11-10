use std::rc::Rc;

use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, adw,
    adw::prelude::*, gtk,
};

use crate::{
    host_services::desktop::{locale_formatting::MomentFormatters, preference_keys::PreferenceKey},
    shared_widgets::setting_row::preference_rows_group::{
        PreferenceApplyMode, PreferenceRowSpec, PreferenceRowsGroup, PreferenceRowsInit,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

pub struct ScanOptionsSection {
    pub message_catalog: Rc<MessageCatalog>,
    option_rows: Controller<PreferenceRowsGroup>,
}

fn scan_option_row_specs(message_catalog: &MessageCatalog) -> Vec<PreferenceRowSpec> {
    vec![
        PreferenceRowSpec {
            preference_key: PreferenceKey::ScanBackend,
            setting_title: translated_messages::scans::backend_title(message_catalog),
            setting_description: translated_messages::scans::automatic_uses_daemon_subtitle(
                message_catalog,
            ),
        },
        PreferenceRowSpec {
            preference_key: PreferenceKey::ShowLiveProgress,
            setting_title: translated_messages::scans::show_live_scan_title(message_catalog),
            setting_description: translated_messages::scans::display_files_scanned_subtitle(
                message_catalog,
            ),
        },
    ]
}

pub struct ScanOptionsSectionInit {
    pub message_catalog: Rc<MessageCatalog>,
    pub moment_formatters: Rc<MomentFormatters>,
}

#[relm4::component(pub)]
impl Component for ScanOptionsSection {
    type Init = ScanOptionsSectionInit;
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
                set_title: &translated_messages::scans::options_title(&model.message_catalog),
            },
        }
    }

    fn init(
        ScanOptionsSectionInit {
            message_catalog,
            moment_formatters,
        }: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            message_catalog: Rc::clone(&message_catalog),
            option_rows: PreferenceRowsGroup::builder()
                .launch(PreferenceRowsInit {
                    message_catalog: Rc::clone(&message_catalog),
                    moment_formatters,
                    row_specs: scan_option_row_specs(&message_catalog),
                    apply_mode: PreferenceApplyMode::AppliesImmediately,
                })
                .detach(),
        };
        let option_group = model.option_rows.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
