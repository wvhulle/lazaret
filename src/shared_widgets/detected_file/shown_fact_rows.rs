use std::path::Path;

use relm4::{
    adw,
    adw::prelude::*,
    gtk,
    gtk::{gio, glib},
};

use crate::{
    host_services::desktop::failure_journal_log::log_failure_to_journal,
    shared_widgets::detected_file::file_fact_rows::{
        FileFactRow, FolderActions, HoverDetail, RowEmphasis,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

fn icon_button(icon_name: &str, button_label: &str) -> gtk::Button {
    let icon_button = gtk::Button::builder()
        .icon_name(icon_name)
        .tooltip_text(button_label)
        .valign(gtk::Align::Center)
        .css_classes(["flat"])
        .build();
    icon_button.update_property(&[gtk::accessible::Property::Label(button_label)]);
    icon_button
}

fn copy_path_button(message_catalog: &MessageCatalog, file_path: &Path) -> gtk::Button {
    let copy_button = icon_button(
        "edit-copy-symbolic",
        &translated_messages::widgets::copy_path(message_catalog),
    );
    let copied_path = file_path.to_string_lossy().into_owned();
    copy_button.connect_clicked(move |clicked_button| {
        clicked_button.clipboard().set_text(&copied_path);
    });
    copy_button
}

fn log_reveal_outcome(reveal_outcome: Result<(), glib::Error>) {
    if let Err(reveal_failure) = reveal_outcome {
        log_failure_to_journal("showing a detected file in Files failed", &reveal_failure);
    }
}

fn reveal_in_files_button(message_catalog: &MessageCatalog, file_path: &Path) -> gtk::Button {
    let reveal_button = icon_button(
        "folder-open-symbolic",
        &translated_messages::findings::show_in_files(message_catalog),
    );
    let revealed_file = gio::File::for_path(file_path);
    reveal_button.connect_clicked(move |clicked_button| {
        let parent_window = clicked_button.root().and_downcast::<gtk::Window>();
        gtk::FileLauncher::new(Some(&revealed_file)).open_containing_folder(
            parent_window.as_ref(),
            gio::Cancellable::NONE,
            log_reveal_outcome,
        );
    });
    reveal_button
}

pub fn fact_action_row(message_catalog: &MessageCatalog, fact_row: &FileFactRow) -> adw::ActionRow {
    let action_row = adw::ActionRow::builder()
        .use_markup(false)
        .title(&fact_row.fact_label)
        .subtitle(&fact_row.fact_value)
        .subtitle_lines(0)
        .subtitle_selectable(true)
        .build();
    action_row.add_css_class("property");
    if let HoverDetail::HoverText(hover_text) = &fact_row.hover_detail {
        action_row.set_tooltip_text(Some(hover_text));
    }
    match fact_row.row_emphasis {
        RowEmphasis::WarningRow => action_row.add_css_class("warning"),
        RowEmphasis::PlainRow => {}
    }
    match &fact_row.folder_actions {
        FolderActions::NoFolderActions => {}
        FolderActions::CopyPath(file_path) => {
            action_row.add_suffix(&copy_path_button(message_catalog, file_path));
        }
        FolderActions::CopyAndReveal(file_path) => {
            action_row.add_suffix(&reveal_in_files_button(message_catalog, file_path));
            action_row.add_suffix(&copy_path_button(message_catalog, file_path));
        }
    }
    action_row
}

#[derive(Debug, Default)]
pub struct ShownFactRows {
    action_rows: Vec<adw::ActionRow>,
}

impl ShownFactRows {
    fn rebuilt_rows(
        &mut self,
        message_catalog: &MessageCatalog,
        fact_rows: &[FileFactRow],
    ) -> Vec<adw::ActionRow> {
        std::mem::replace(
            &mut self.action_rows,
            fact_rows
                .iter()
                .map(|fact_row| fact_action_row(message_catalog, fact_row))
                .collect(),
        )
    }

    pub fn replace_in_group(
        &mut self,
        message_catalog: &MessageCatalog,
        preferences_group: &adw::PreferencesGroup,
        fact_rows: &[FileFactRow],
    ) {
        self.rebuilt_rows(message_catalog, fact_rows)
            .iter()
            .for_each(|removed_row| preferences_group.remove(removed_row));
        self.action_rows
            .iter()
            .for_each(|shown_row| preferences_group.add(shown_row));
    }
}
