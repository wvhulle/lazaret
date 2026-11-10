use std::rc::Rc;

use relm4::{
    FactorySender, adw,
    adw::prelude::*,
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
};

use crate::{
    host_services::clamav_scanner::scanner_output_parser::sanitize_scanner_line,
    scans::manual_scan_page::scan_profiles_dialog::{
        profile_display_text::{displayed_profile_description, displayed_profile_name},
        scan_profile_store::{ProfileOrigin, ScanProfile},
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

pub struct ScanProfileRow {
    pub message_catalog: Rc<MessageCatalog>,
    pub scan_profile: ScanProfile,
}

#[derive(Debug)]
pub enum ScanProfileRowOutput {
    ScanRequested(DynamicIndex),
    EditRequested(DynamicIndex),
    DeletionRequested(DynamicIndex),
    ExportRequested(DynamicIndex),
}

fn profile_is_deletable(scan_profile: &ScanProfile) -> bool {
    match scan_profile.profile_origin {
        ProfileOrigin::Builtin(_builtin_profile) => false,
        ProfileOrigin::Custom { .. } => true,
    }
}

fn profile_subtitle(message_catalog: &MessageCatalog, scan_profile: &ScanProfile) -> String {
    let target_count = scan_profile.profile_draft.scan_targets.len();
    let target_text = translated_messages::scans::count_target(message_catalog, target_count);
    let builtin_text = match scan_profile.profile_origin {
        ProfileOrigin::Builtin(_builtin_profile) => {
            Some(translated_messages::scans::default_profile(message_catalog))
        }
        ProfileOrigin::Custom { .. } => None,
    };
    std::iter::once(sanitize_scanner_line(&displayed_profile_description(
        message_catalog,
        scan_profile,
    )))
    .filter(|description_text| !description_text.is_empty())
    .chain(std::iter::once(target_text))
    .chain(builtin_text)
    .collect::<Vec<_>>()
    .join(" • ")
}

pub struct ScanProfileRowInit {
    pub scan_profile: ScanProfile,
    pub message_catalog: Rc<MessageCatalog>,
}

#[relm4::factory(pub)]
impl FactoryComponent for ScanProfileRow {
    type Init = ScanProfileRowInit;
    type Input = ();
    type Output = ScanProfileRowOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        adw::ActionRow {
            set_use_markup: false,
            set_title: &sanitize_scanner_line(&displayed_profile_name(&self.message_catalog, &self.scan_profile)),
            set_subtitle: &profile_subtitle(&self.message_catalog, &self.scan_profile),
            set_subtitle_lines: 0,

            add_suffix = &gtk::Button {
                set_icon_name: "media-playback-start-symbolic",
                set_tooltip_text: Some(&translated_messages::scans::run_tooltip(&self.message_catalog)),
                set_valign: gtk::Align::Center,
                add_css_class: "flat",
                connect_clicked[sender, index] => move |_clicked_button| {
                    let _parent_gone = sender.output(ScanProfileRowOutput::ScanRequested(index.clone()));
                },
            },

            add_suffix = &gtk::Button {
                set_icon_name: "document-edit-symbolic",
                set_tooltip_text: Some(&translated_messages::scans::edit_tooltip(&self.message_catalog)),
                set_valign: gtk::Align::Center,
                add_css_class: "flat",
                connect_clicked[sender, index] => move |_clicked_button| {
                    let _parent_gone = sender.output(ScanProfileRowOutput::EditRequested(index.clone()));
                },
            },

            add_suffix = &gtk::Button {
                set_icon_name: "user-trash-symbolic",
                set_tooltip_text: Some(&translated_messages::scans::delete_tooltip(&self.message_catalog)),
                set_valign: gtk::Align::Center,
                add_css_class: "flat",
                set_sensitive: profile_is_deletable(&self.scan_profile),
                connect_clicked[sender, index] => move |_clicked_button| {
                    let _parent_gone = sender.output(ScanProfileRowOutput::DeletionRequested(index.clone()));
                },
            },

            add_suffix = &gtk::Button {
                set_icon_name: "document-save-symbolic",
                set_tooltip_text: Some(&translated_messages::scans::export_tooltip(&self.message_catalog)),
                set_valign: gtk::Align::Center,
                add_css_class: "flat",
                connect_clicked[sender, index] => move |_clicked_button| {
                    let _parent_gone = sender.output(ScanProfileRowOutput::ExportRequested(index.clone()));
                },
            },
        }
    }

    fn init_model(
        ScanProfileRowInit {
            scan_profile,
            message_catalog,
        }: Self::Init,
        _row_index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            scan_profile,
            message_catalog,
        }
    }
}
