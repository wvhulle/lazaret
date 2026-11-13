use std::{path::PathBuf, rc::Rc};

use jiff::Timestamp;
use relm4::{Component, ComponentParts, ComponentSender, adw, adw::prelude::*, gtk};

use crate::{
    host_services::desktop::locale_formatting::MomentFormatters,
    shared_widgets::detected_file::{
        detected_file_facts::FileFactsSource,
        file_fact_rows::{
            FACT_SECTIONS, FactSection, FileFactRow, detected_file_rows, facts_source_description,
            rows_in_section, section_title,
        },
        shown_fact_rows::ShownFactRows,
    },
    translated_messages::MessageCatalog,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShownDetectedFile {
    pub file_path: PathBuf,
    pub facts_source: FileFactsSource,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileFactsGroupState {
    NothingShown,
    FileShown(ShownDetectedFile),
}

pub struct DetectedFileFactsGroupInit {
    pub message_catalog: Rc<MessageCatalog>,
    pub moment_formatters: Rc<MomentFormatters>,
    pub group_state: FileFactsGroupState,
}

struct ShownSection {
    fact_section: FactSection,
    section_group: adw::PreferencesGroup,
    shown_fact_rows: ShownFactRows,
}

pub struct DetectedFileFactsGroup {
    pub message_catalog: Rc<MessageCatalog>,
    moment_formatters: Rc<MomentFormatters>,
    group_state: FileFactsGroupState,
    shown_sections: Vec<ShownSection>,
}

#[derive(Debug)]
pub enum DetectedFileFactsGroupInput {
    FileShown(ShownDetectedFile),
}

fn section_group(message_catalog: &MessageCatalog, fact_section: FactSection) -> ShownSection {
    ShownSection {
        fact_section,
        section_group: adw::PreferencesGroup::builder()
            .title(section_title(message_catalog, fact_section))
            .build(),
        shown_fact_rows: ShownFactRows::default(),
    }
}

impl DetectedFileFactsGroup {
    fn group_description(&self) -> String {
        match &self.group_state {
            FileFactsGroupState::NothingShown => String::new(),
            FileFactsGroupState::FileShown(shown_file) => {
                facts_source_description(&self.message_catalog, &shown_file.facts_source)
            }
        }
    }

    fn file_rows(&self) -> Vec<FileFactRow> {
        match &self.group_state {
            FileFactsGroupState::NothingShown => Vec::new(),
            FileFactsGroupState::FileShown(shown_file) => detected_file_rows(
                &self.message_catalog,
                &self.moment_formatters,
                Timestamp::now(),
                &shown_file.file_path,
                &shown_file.facts_source,
            ),
        }
    }

    fn show_sections(&mut self) {
        let file_rows = self.file_rows();
        let group_description = self.group_description();
        let described_section = file_rows.first().map(|first_row| first_row.fact_section);
        self.shown_sections.iter_mut().for_each(|shown_section| {
            let section_rows = rows_in_section(&file_rows, shown_section.fact_section);
            shown_section.shown_fact_rows.replace_in_group(
                &self.message_catalog,
                &shown_section.section_group,
                &section_rows,
            );
            shown_section
                .section_group
                .set_visible(!section_rows.is_empty());
            shown_section.section_group.set_description(
                (described_section == Some(shown_section.fact_section))
                    .then_some(group_description.as_str()),
            );
        });
    }
}

#[relm4::component(pub)]
impl Component for DetectedFileFactsGroup {
    type Init = DetectedFileFactsGroupInit;
    type Input = DetectedFileFactsGroupInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 18,
        }
    }

    fn init(
        DetectedFileFactsGroupInit {
            message_catalog,
            moment_formatters,
            group_state,
        }: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let shown_sections = FACT_SECTIONS
            .into_iter()
            .map(|fact_section| section_group(&message_catalog, fact_section))
            .collect::<Vec<_>>();
        shown_sections
            .iter()
            .for_each(|shown_section| root.append(&shown_section.section_group));
        let mut model = Self {
            message_catalog,
            moment_formatters,
            group_state,
            shown_sections,
        };
        let widgets = view_output!();
        model.show_sections();
        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        group_input: Self::Input,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match group_input {
            DetectedFileFactsGroupInput::FileShown(shown_file) => {
                self.group_state = FileFactsGroupState::FileShown(shown_file);
                self.show_sections();
            }
        }
    }
}
