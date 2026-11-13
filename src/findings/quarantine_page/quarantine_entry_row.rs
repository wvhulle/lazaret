use std::rc::Rc;

use relm4::{
    FactorySender, adw,
    adw::prelude::*,
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
};

use crate::{
    findings::quarantine_page::quarantine_page_state::{
        EntryRowState, QuarantineRowInit, RowRecheckState,
    },
    host_services::{
        clamav_scanner::{
            content_verdict::ContentVerdict, scanner_output_parser::displayed_threat_name,
        },
        desktop::locale_formatting::MomentFormatters,
        quarantine_vault::{
            quarantine_entry_database::QuarantineEntryRecord,
            quarantine_file_moves::RecordedPayloadHash,
        },
        sample_submissions::submission_samples::shown_file_name,
    },
    shared_widgets::detected_file::{
        detected_file_facts::{FileFactsSource, RecordedQuarantineFacts, file_fact_rows},
        detected_file_row::{ShownFactRows, path_then_fact_rows},
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

pub struct QuarantineEntryRow {
    pub message_catalog: Rc<MessageCatalog>,
    pub moment_formatters: Rc<MomentFormatters>,
    entry_record: QuarantineEntryRecord,
    row_state: EntryRowState,
    shown_fact_rows: ShownFactRows,
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
        let message_catalog = Rc::clone(&self.message_catalog);
        match &self.row_state.recheck_state {
            RowRecheckState::Rechecking => {
                translated_messages::findings::checking_label(&message_catalog)
            }
            RowRecheckState::Rechecked(ContentVerdict::NoLongerDetected) => {
                translated_messages::findings::no_longer_detected(&message_catalog)
            }
            RowRecheckState::Rechecked(ContentVerdict::StillDetected { .. }) => {
                translated_messages::findings::still_detected(&message_catalog)
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
        #[name = "entry_expander_row"]
        adw::ExpanderRow {
            set_use_markup: false,
            set_title: &displayed_threat_name(&self.message_catalog, &self.entry_record.threat_identity),
            set_subtitle: &shown_file_name(&self.entry_record.original_path),
            set_subtitle_lines: 0,

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
                set_label: &translated_messages::findings::integrity_unknown_label(&self.message_catalog),
                set_valign: gtk::Align::Center,
                set_css_classes: &["caption-heading", "warning"],
                set_visible: self.integrity_unknown(),
            },

            add_suffix = &gtk::Button {
                set_icon_name: "go-next-symbolic",
                set_tooltip_text: Some(&translated_messages::findings::open_details(&self.message_catalog)),
                update_property: &[gtk::accessible::Property::Label(&translated_messages::findings::open_details(&self.message_catalog))],
                set_valign: gtk::Align::Center,
                add_css_class: "flat",
                connect_clicked[sender, entry_id = self.entry_record.entry_id] => move |_clicked_button| {
                    let _parent_gone = sender.output(QuarantineEntryRowOutput::EntryOpened { entry_id });
                },
            },
        }
    }

    fn init_model(
        QuarantineRowInit {
            message_catalog,
            moment_formatters,
            entry_record,
            row_state,
        }: QuarantineRowInit,
        _row_index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            message_catalog,
            moment_formatters,
            entry_record,
            row_state,
            shown_fact_rows: ShownFactRows::default(),
        }
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();
        let fact_rows = path_then_fact_rows(
            &self.message_catalog,
            &self.entry_record.original_path,
            file_fact_rows(
                &self.message_catalog,
                &self.moment_formatters,
                &FileFactsSource::RecordedAtQuarantine(RecordedQuarantineFacts::from(
                    &self.entry_record,
                )),
            ),
        );
        self.shown_fact_rows
            .replace_in_expander(&widgets.entry_expander_row, &fact_rows);
        widgets
    }

    fn update(&mut self, row_input: QuarantineEntryRowInput, _sender: FactorySender<Self>) {
        match row_input {
            QuarantineEntryRowInput::RowStateChanged(row_state) => {
                self.row_state = *row_state;
            }
        }
    }
}
