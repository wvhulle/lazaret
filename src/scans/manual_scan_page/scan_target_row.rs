use std::{path::PathBuf, rc::Rc};

use relm4::{
    FactorySender, adw,
    adw::prelude::*,
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
};

use crate::{
    host_services::clamav_scanner::scanner_output_parser::sanitize_scanner_line,
    translated_messages, translated_messages::MessageCatalog,
};

pub struct ScanTargetRow {
    pub message_catalog: Rc<MessageCatalog>,
    pub target_path: PathBuf,
}

#[derive(Debug)]
pub enum ScanTargetRowOutput {
    RemoveRequested(DynamicIndex),
}

pub struct ScanTargetRowInit {
    pub target_path: PathBuf,
    pub message_catalog: Rc<MessageCatalog>,
}

#[relm4::factory(pub)]
impl FactoryComponent for ScanTargetRow {
    type Init = ScanTargetRowInit;
    type Input = ();
    type Output = ScanTargetRowOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        adw::ActionRow {
            set_use_markup: false,
            set_title: &sanitize_scanner_line(&self.target_path.to_string_lossy()),
            set_title_lines: 0,

            add_prefix = &gtk::Image {
                set_icon_name: Some(if self.target_path.is_dir() {
                    "folder-symbolic"
                } else {
                    "text-x-generic-symbolic"
                }),
            },

            add_suffix = &gtk::Button {
                set_icon_name: "edit-delete-symbolic",
                set_tooltip_text: Some(&translated_messages::scans::remove_target_tooltip(&self.message_catalog)),
                set_valign: gtk::Align::Center,
                add_css_class: "flat",
                connect_clicked[sender, index] => move |_clicked_button| {
                    let _parent_gone = sender.output(ScanTargetRowOutput::RemoveRequested(index.clone()));
                },
            },
        }
    }

    fn init_model(
        ScanTargetRowInit {
            target_path,
            message_catalog,
        }: Self::Init,
        _row_index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            target_path,
            message_catalog,
        }
    }
}
