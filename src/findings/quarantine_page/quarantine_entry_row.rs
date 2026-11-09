use relm4::{
    FactorySender, adw,
    adw::prelude::*,
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
};

use crate::{
    findings::quarantine_page::{
        quarantine_entry_detail::entry_summary_text,
        quarantine_page_state::{EntryRowState, QuarantineRowInit, RowRecheckState},
    },
    host_services::{
        clamav_scanner::{
            content_verdict::ContentVerdict, scanner_output_parser::displayed_threat_name,
        },
        quarantine_vault::{
            quarantine_entry_database::QuarantineEntryRecord,
            quarantine_file_moves::RecordedPayloadHash,
        },
    },
    translated_messages,
};

pub struct QuarantineEntryRow {
    entry_record: QuarantineEntryRecord,
    row_state: EntryRowState,
}

#[derive(Debug)]
pub enum QuarantineEntryRowInput {
    RowStateChanged(Box<EntryRowState>),
}

#[derive(Debug)]
pub enum QuarantineEntryRowOutput {
    EntryOpened { entry_id: i64 },
}

impl QuarantineEntryRow {
    pub const fn entry_id(&self) -> i64 {
        self.entry_record.entry_id
    }

    fn recheck_label(&self) -> String {
        match &self.row_state.recheck_state {
            RowRecheckState::Rechecking => translated_messages::findings::checking_label(),
            RowRecheckState::Rechecked(ContentVerdict::NoLongerDetected) => {
                translated_messages::findings::no_longer_detected()
            }
            RowRecheckState::Rechecked(ContentVerdict::StillDetected { .. }) => {
                translated_messages::findings::still_detected()
            }
            RowRecheckState::Rechecked(ContentVerdict::VerdictUnavailable(_))
            | RowRecheckState::NotRechecked => String::new(),
        }
    }

    fn recheck_style(&self) -> &'static str {
        match &self.row_state.recheck_state {
            RowRecheckState::Rechecked(ContentVerdict::NoLongerDetected) => "success",
            RowRecheckState::Rechecked(ContentVerdict::StillDetected { .. }) => "warning",
            RowRecheckState::Rechecking
            | RowRecheckState::Rechecked(ContentVerdict::VerdictUnavailable(_))
            | RowRecheckState::NotRechecked => "dim-label",
        }
    }

    fn integrity_unknown(&self) -> bool {
        self.entry_record.payload_hash == RecordedPayloadHash::IntegrityUnknown
    }
}

#[relm4::factory(pub)]
impl FactoryComponent for QuarantineEntryRow {
    type Init = QuarantineRowInit;
    type Input = QuarantineEntryRowInput;
    type Output = QuarantineEntryRowOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        #[root]
        adw::ActionRow {
            set_use_markup: false,
            set_title: &displayed_threat_name(&self.entry_record.threat_identity),
            set_subtitle: &entry_summary_text(&self.entry_record),
            set_subtitle_lines: 0,
            set_activatable: true,
            connect_activated[sender, entry_id = self.entry_record.entry_id] => move |_activated_row| {
                let _parent_gone = sender.output(QuarantineEntryRowOutput::EntryOpened { entry_id });
            },

            add_suffix = &gtk::Label {
                set_valign: gtk::Align::Center,
                #[watch]
                set_visible: !self.recheck_label().is_empty(),
                #[watch]
                set_label: &self.recheck_label(),
                #[watch]
                set_css_classes: &["caption-heading", self.recheck_style()],
            },

            add_suffix = &gtk::Label {
                set_label: &translated_messages::findings::integrity_unknown_label(),
                set_valign: gtk::Align::Center,
                set_css_classes: &["caption-heading", "warning"],
                set_visible: self.integrity_unknown(),
            },

            add_suffix = &gtk::Image {
                set_icon_name: Some("go-next-symbolic"),
            },
        }
    }

    fn init_model(
        QuarantineRowInit {
            entry_record,
            row_state,
        }: QuarantineRowInit,
        _row_index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            entry_record,
            row_state,
        }
    }

    fn update(&mut self, row_input: QuarantineEntryRowInput, _sender: FactorySender<Self>) {
        match row_input {
            QuarantineEntryRowInput::RowStateChanged(row_state) => {
                self.row_state = *row_state;
            }
        }
    }
}
