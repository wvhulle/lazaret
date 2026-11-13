use std::{path::PathBuf, rc::Rc};

use relm4::{Component, ComponentParts, ComponentSender, adw, adw::prelude::*, gtk};

use crate::{
    host_services::desktop::locale_formatting::MomentFormatters,
    shared_widgets::detected_file::{
        detected_file_facts::{FileFactsSource, facts_source_description, file_fact_rows},
        detected_file_row::{ShownFactRows, shown_file_path},
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShownDetectedFile {
    pub file_path: PathBuf,
    pub facts_source: FileFactsSource,
}

pub struct DetectedFileFactsGroupInit {
    pub message_catalog: Rc<MessageCatalog>,
    pub moment_formatters: Rc<MomentFormatters>,
    pub shown_file: ShownDetectedFile,
}

pub struct DetectedFileFactsGroup {
    pub message_catalog: Rc<MessageCatalog>,
    moment_formatters: Rc<MomentFormatters>,
    shown_file: ShownDetectedFile,
    shown_fact_rows: ShownFactRows,
}

#[derive(Debug)]
pub enum DetectedFileFactsGroupInput {
    FileShown(ShownDetectedFile),
    PathCopyRequested,
}

impl DetectedFileFactsGroup {
    fn show_fact_rows(&mut self, facts_group: &adw::PreferencesGroup) {
        let fact_rows = file_fact_rows(
            &self.message_catalog,
            &self.moment_formatters,
            &self.shown_file.facts_source,
        );
        self.shown_fact_rows
            .replace_in_group(facts_group, &fact_rows);
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
            set_description: Some(&facts_source_description(&model.message_catalog, &model.shown_file.facts_source)),

            add = &adw::ActionRow {
                set_use_markup: false,
                set_title: &translated_messages::findings::file_path(&model.message_catalog),
                #[watch]
                set_subtitle: &shown_file_path(&model.shown_file.file_path),
                set_subtitle_lines: 0,
                set_subtitle_selectable: true,
                add_css_class: "property",

                add_suffix = &gtk::Button {
                    set_icon_name: "edit-copy-symbolic",
                    set_tooltip_text: Some(&translated_messages::widgets::copy_path(&model.message_catalog)),
                    update_property: &[gtk::accessible::Property::Label(&translated_messages::widgets::copy_path(&model.message_catalog))],
                    set_valign: gtk::Align::Center,
                    add_css_class: "flat",
                    connect_clicked => DetectedFileFactsGroupInput::PathCopyRequested,
                },
            },
        }
    }

    fn init(
        DetectedFileFactsGroupInit {
            message_catalog,
            moment_formatters,
            shown_file,
        }: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = Self {
            message_catalog,
            moment_formatters,
            shown_file,
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
                self.shown_file = shown_file;
                self.show_fact_rows(root);
            }
            DetectedFileFactsGroupInput::PathCopyRequested => {
                root.clipboard()
                    .set_text(&self.shown_file.file_path.to_string_lossy());
            }
        }
    }
}
