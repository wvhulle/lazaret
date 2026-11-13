use std::rc::Rc;

use relm4::{FactorySender, adw, adw::prelude::*, gtk, prelude::FactoryComponent};

use crate::{
    host_services::{
        clamav_scanner::scanner_output_parser::{DetectedThreat, displayed_threat_name},
        desktop::locale_formatting::MomentFormatters,
        sample_submissions::submission_samples::shown_file_name,
    },
    shared_widgets::detected_file::{
        detected_file_facts::{FileFactsSource, file_fact_rows},
        detected_file_row::{
            InspectionRequest, LiveFactsLoading, ShownFactRows, inspect_live_facts_in_background,
            path_then_fact_rows,
        },
    },
    translated_messages::MessageCatalog,
};

pub struct FoundThreatRow {
    pub message_catalog: Rc<MessageCatalog>,
    moment_formatters: Rc<MomentFormatters>,
    detected_threat: DetectedThreat,
    facts_loading: LiveFactsLoading,
    shown_fact_rows: ShownFactRows,
}

impl FoundThreatRow {
    fn row_subtitle(&self) -> String {
        displayed_threat_name(&self.message_catalog, &self.detected_threat.threat_identity)
    }
}

#[derive(Debug)]
pub enum FoundThreatRowInput {
    FileFactsRequested,
}

pub struct FoundThreatRowInit {
    pub detected_threat: DetectedThreat,
    pub message_catalog: Rc<MessageCatalog>,
    pub moment_formatters: Rc<MomentFormatters>,
}

#[relm4::factory(pub)]
impl FactoryComponent for FoundThreatRow {
    type Init = FoundThreatRowInit;
    type Input = FoundThreatRowInput;
    type Output = ();
    type CommandOutput = FileFactsSource;
    type ParentWidget = gtk::ListBox;

    view! {
        #[name = "found_threat_expander_row"]
        adw::ExpanderRow {
            set_use_markup: false,
            set_title: &shown_file_name(&self.detected_threat.file_path),
            set_subtitle: &self.row_subtitle(),
            set_subtitle_lines: 0,
            connect_expanded_notify[sender] => move |expander_row| {
                if expander_row.is_expanded() {
                    sender.input(FoundThreatRowInput::FileFactsRequested);
                }
            },
        }
    }

    fn init_model(
        FoundThreatRowInit {
            detected_threat,
            message_catalog,
            moment_formatters,
        }: Self::Init,
        _row_index: &Self::Index,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            message_catalog,
            moment_formatters,
            detected_threat,
            facts_loading: LiveFactsLoading::NotRequested,
            shown_fact_rows: ShownFactRows::default(),
        }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();
        self.shown_fact_rows.replace_in_expander(
            &widgets.found_threat_expander_row,
            &path_then_fact_rows(
                &self.message_catalog,
                &self.detected_threat.file_path,
                Vec::new(),
            ),
        );
        widgets
    }

    fn update(&mut self, row_input: FoundThreatRowInput, sender: FactorySender<Self>) {
        match row_input {
            FoundThreatRowInput::FileFactsRequested => {
                if self.facts_loading.request_inspection() == InspectionRequest::StartInspection {
                    sender.oneshot_command(inspect_live_facts_in_background(
                        self.detected_threat.file_path.clone(),
                    ));
                }
            }
        }
    }

    fn update_cmd_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        facts_source: FileFactsSource,
        sender: FactorySender<Self>,
    ) {
        let fact_rows = file_fact_rows(
            &self.message_catalog,
            &self.moment_formatters,
            &facts_source,
        );
        self.shown_fact_rows
            .replace_in_expander(&widgets.found_threat_expander_row, &fact_rows);
        self.facts_loading = LiveFactsLoading::Inspected;
        self.update_view(widgets, sender);
    }
}
