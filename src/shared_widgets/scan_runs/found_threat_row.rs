use std::rc::Rc;

use relm4::{FactorySender, adw, adw::prelude::*, gtk, prelude::FactoryComponent};

use crate::{
    host_services::clamav_scanner::scanner_output_parser::{
        DetectedThreat, displayed_threat_name, sanitize_scanner_line,
    },
    translated_messages::MessageCatalog,
};

pub struct FoundThreatRow {
    pub message_catalog: Rc<MessageCatalog>,
    detected_threat: DetectedThreat,
}

impl FoundThreatRow {
    fn shown_file_name(&self) -> String {
        sanitize_scanner_line(
            &self
                .detected_threat
                .file_path
                .file_name()
                .unwrap_or(self.detected_threat.file_path.as_os_str())
                .to_string_lossy(),
        )
    }
}

pub struct FoundThreatRowInit {
    pub detected_threat: DetectedThreat,
    pub message_catalog: Rc<MessageCatalog>,
}

#[relm4::factory(pub)]
impl FactoryComponent for FoundThreatRow {
    type Init = FoundThreatRowInit;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        adw::ActionRow {
            set_use_markup: false,
            set_title: &self.shown_file_name(),
            set_subtitle: &displayed_threat_name(&self.message_catalog, &self.detected_threat.threat_identity),
            set_tooltip_text: Some(&sanitize_scanner_line(&self.detected_threat.file_path.to_string_lossy())),
        }
    }

    fn init_model(
        FoundThreatRowInit {
            detected_threat,
            message_catalog,
        }: Self::Init,
        _row_index: &Self::Index,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            detected_threat,
            message_catalog,
        }
    }
}
