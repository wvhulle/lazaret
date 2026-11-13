use std::rc::Rc;

use relm4::{FactorySender, adw, adw::prelude::*, gtk, prelude::FactoryComponent};

use crate::{
    findings::detections_page::incident_timeline_presentation::{
        DisplayedDetection, detection_needs_attention, detection_summary_text, displayed_file_name,
    },
    host_services::desktop::locale_formatting::MomentFormatters,
    shared_widgets::detected_file::{
        detected_file_facts::file_fact_rows,
        detected_file_row::{ShownFactRows, path_then_fact_rows},
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

pub struct DetectionListRow {
    pub message_catalog: Rc<MessageCatalog>,
    moment_formatters: Rc<MomentFormatters>,
    displayed_detection: DisplayedDetection,
    shown_fact_rows: ShownFactRows,
}

#[derive(Debug)]
pub enum DetectionListRowOutput {
    IncidentOpened(i64),
}

impl DetectionListRow {
    fn needs_attention(&self) -> bool {
        detection_needs_attention(&self.displayed_detection)
    }

    const fn attention_icon(&self, needs_attention: bool) -> &'static str {
        if needs_attention {
            "dialog-warning-symbolic"
        } else {
            "object-select-symbolic"
        }
    }

    fn attention_style(&self, needs_attention: bool) -> &'static str {
        if needs_attention {
            "warning"
        } else {
            "success"
        }
    }

    fn row_subtitle(&self) -> String {
        detection_summary_text(&self.message_catalog, &self.displayed_detection)
    }
}

pub struct DetectionListRowInit {
    pub displayed_detection: DisplayedDetection,
    pub message_catalog: Rc<MessageCatalog>,
    pub moment_formatters: Rc<MomentFormatters>,
}

#[relm4::factory(pub)]
impl FactoryComponent for DetectionListRow {
    type Init = DetectionListRowInit;
    type Input = ();
    type Output = DetectionListRowOutput;
    type CommandOutput = ();
    type ParentWidget = adw::PreferencesGroup;

    view! {
        #[name = "detection_expander_row"]
        adw::ExpanderRow {
            set_use_markup: false,
            set_title: &displayed_file_name(&self.displayed_detection.stored_detection),
            set_subtitle: &self.row_subtitle(),
            set_subtitle_lines: 0,

            add_prefix = &gtk::Image {
                set_icon_name: Some(self.attention_icon(self.needs_attention())),
                add_css_class: self.attention_style(self.needs_attention()),
            },
            add_suffix = &gtk::Button {
                set_icon_name: "go-next-symbolic",
                set_tooltip_text: Some(&translated_messages::findings::open_details(&self.message_catalog)),
                update_property: &[gtk::accessible::Property::Label(&translated_messages::findings::open_details(&self.message_catalog))],
                set_valign: gtk::Align::Center,
                add_css_class: "flat",
                connect_clicked[sender, detection_id = self.displayed_detection.stored_detection.detection_id] => move |_clicked_button| {
                    let _page_closed = sender.output(DetectionListRowOutput::IncidentOpened(detection_id));
                },
            },
        }
    }

    fn init_model(
        DetectionListRowInit {
            displayed_detection,
            message_catalog,
            moment_formatters,
        }: Self::Init,
        _row_index: &Self::Index,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            message_catalog,
            moment_formatters,
            displayed_detection,
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
        let fact_rows = path_then_fact_rows(
            &self.message_catalog,
            &self.displayed_detection.stored_detection.file_path,
            file_fact_rows(
                &self.message_catalog,
                &self.moment_formatters,
                &self.displayed_detection.file_state.facts_source(),
            ),
        );
        self.shown_fact_rows
            .replace_in_expander(&widgets.detection_expander_row, &fact_rows);
        widgets
    }
}
