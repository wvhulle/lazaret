use std::{path::PathBuf, rc::Rc};

use jiff::Timestamp;
use relm4::{Component, ComponentParts, ComponentSender, adw, adw::prelude::*};

use crate::{
    host_services::desktop::locale_formatting::MomentFormatters,
    shared_widgets::detected_file::{
        detected_file_facts::FileFactsSource,
        detected_file_row::ShownFactRows,
        file_fact_rows::{FileFactRow, detected_file_rows, facts_source_description},
    },
    translated_messages,
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

pub struct DetectedFileFactsGroup {
    pub message_catalog: Rc<MessageCatalog>,
    moment_formatters: Rc<MomentFormatters>,
    group_state: FileFactsGroupState,
    shown_fact_rows: ShownFactRows,
}

#[derive(Debug)]
pub enum DetectedFileFactsGroupInput {
    FileShown(ShownDetectedFile),
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

    fn group_fact_rows(&self) -> Vec<FileFactRow> {
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

    fn show_fact_rows(&mut self, facts_group: &adw::PreferencesGroup) {
        let fact_rows = self.group_fact_rows();
        self.shown_fact_rows
            .replace_in_group(&self.message_catalog, facts_group, &fact_rows);
    }
}

#[relm4::component(pub)]
impl Component for DetectedFileFactsGroup {
    type Init = DetectedFileFactsGroupInit;
    type Input = DetectedFileFactsGroupInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::PreferencesGroup {
            set_title: &translated_messages::findings::file_label(&model.message_catalog),
            #[watch]
            set_description: Some(&model.group_description()),
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
        let mut model = Self {
            message_catalog,
            moment_formatters,
            group_state,
            shown_fact_rows: ShownFactRows::default(),
        };
        let widgets = view_output!();
        model.show_fact_rows(&root);
        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        group_input: Self::Input,
        _sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match group_input {
            DetectedFileFactsGroupInput::FileShown(shown_file) => {
                self.group_state = FileFactsGroupState::FileShown(shown_file);
                self.show_fact_rows(root);
            }
        }
    }
}
