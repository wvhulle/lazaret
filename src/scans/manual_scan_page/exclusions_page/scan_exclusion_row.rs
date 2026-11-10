use std::rc::Rc;

use relm4::{FactorySender, adw, adw::prelude::*, gtk, prelude::FactoryComponent};

use crate::{
    host_services::clamav_scanner::{
        scan_exclusion_rules::{ExclusionEntry, PRESET_DIRECTORY_NAMES},
        scanner_output_parser::sanitize_scanner_line,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

pub fn is_preset_pattern(exclusion_pattern: &str) -> bool {
    PRESET_DIRECTORY_NAMES.contains(&exclusion_pattern)
}

fn preset_description(message_catalog: &MessageCatalog, exclusion_pattern: &str) -> String {
    match exclusion_pattern {
        "node_modules" => translated_messages::scans::node_js_dependencies(message_catalog),
        ".git" => translated_messages::scans::git_repository_data(message_catalog),
        ".venv" => translated_messages::scans::python_virtual_environment(message_catalog),
        "build" => translated_messages::scans::build_output_directory(message_catalog),
        "dist" => translated_messages::scans::distribution_output_directory(message_catalog),
        "__pycache__" => translated_messages::scans::python_bytecode_cache(message_catalog),
        _other_pattern => String::new(),
    }
}

pub struct ScanExclusionRow {
    pub message_catalog: Rc<MessageCatalog>,
    exclusion_entry: ExclusionEntry,
}

#[derive(Debug)]
pub enum ScanExclusionRowOutput {
    EnabledToggled {
        exclusion_pattern: String,
        is_enabled: bool,
    },
    RemovalRequested {
        exclusion_pattern: String,
    },
}

impl ScanExclusionRow {
    fn is_preset(&self) -> bool {
        is_preset_pattern(&self.exclusion_entry.exclusion_pattern)
    }

    fn shown_title(&self) -> String {
        if self.is_preset() {
            self.exclusion_entry.exclusion_pattern.clone()
        } else {
            sanitize_scanner_line(&self.exclusion_entry.exclusion_pattern)
        }
    }
}

pub struct ScanExclusionRowInit {
    pub exclusion_entry: ExclusionEntry,
    pub message_catalog: Rc<MessageCatalog>,
}

#[relm4::factory(pub)]
impl FactoryComponent for ScanExclusionRow {
    type Init = ScanExclusionRowInit;
    type Input = ();
    type Output = ScanExclusionRowOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        adw::SwitchRow {
            set_use_markup: false,
            set_title: &self.shown_title(),
            set_subtitle: &preset_description(&self.message_catalog, &self.exclusion_entry.exclusion_pattern),
            set_active: self.exclusion_entry.is_enabled,
            connect_active_notify[sender, exclusion_pattern = self.exclusion_entry.exclusion_pattern.clone()] => move |changed_row| {
                let _page_closed = sender.output(ScanExclusionRowOutput::EnabledToggled {
                    exclusion_pattern: exclusion_pattern.clone(),
                    is_enabled: changed_row.is_active(),
                });
            },

            add_suffix = &gtk::Button {
                set_icon_name: "user-trash-symbolic",
                set_tooltip_text: Some(&translated_messages::scans::remove_exclusion_tooltip(&self.message_catalog)),
                set_valign: gtk::Align::Center,
                add_css_class: "flat",
                set_visible: !self.is_preset(),
                connect_clicked[sender, exclusion_pattern = self.exclusion_entry.exclusion_pattern.clone()] => move |_clicked_button| {
                    let _page_closed = sender.output(ScanExclusionRowOutput::RemovalRequested {
                        exclusion_pattern: exclusion_pattern.clone(),
                    });
                },
            },
        }
    }

    fn init_model(
        ScanExclusionRowInit {
            exclusion_entry,
            message_catalog,
        }: Self::Init,
        _row_index: &Self::Index,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            exclusion_entry,
            message_catalog,
        }
    }
}
