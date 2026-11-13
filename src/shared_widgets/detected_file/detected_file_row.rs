use std::path::{Path, PathBuf};

use relm4::{adw, adw::prelude::*};
use tokio::task::spawn_blocking;

use crate::{
    host_services::clamav_scanner::scanner_output_parser::sanitize_scanner_line,
    shared_widgets::detected_file::detected_file_facts::{
        FileFactRow, FileFactsSource, inspect_live_file_facts,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

pub fn shown_file_path(file_path: &Path) -> String {
    sanitize_scanner_line(&file_path.to_string_lossy())
}

pub fn path_then_fact_rows(
    message_catalog: &MessageCatalog,
    file_path: &Path,
    fact_rows: Vec<FileFactRow>,
) -> Vec<FileFactRow> {
    std::iter::once(FileFactRow {
        fact_label: translated_messages::findings::file_path(message_catalog),
        fact_value: shown_file_path(file_path),
    })
    .chain(fact_rows)
    .collect()
}

pub fn fact_action_row(fact_row: &FileFactRow) -> adw::ActionRow {
    let action_row = adw::ActionRow::builder()
        .use_markup(false)
        .title(&fact_row.fact_label)
        .subtitle(&fact_row.fact_value)
        .subtitle_lines(0)
        .subtitle_selectable(true)
        .build();
    action_row.add_css_class("property");
    action_row
}

#[derive(Debug, Default)]
pub struct ShownFactRows {
    action_rows: Vec<adw::ActionRow>,
}

impl ShownFactRows {
    pub fn replace_in_expander(
        &mut self,
        expander_row: &adw::ExpanderRow,
        fact_rows: &[FileFactRow],
    ) {
        self.action_rows
            .drain(..)
            .for_each(|shown_row| expander_row.remove(&shown_row));
        self.action_rows = fact_rows.iter().map(fact_action_row).collect();
        self.action_rows
            .iter()
            .for_each(|shown_row| expander_row.add_row(shown_row));
    }

    pub fn replace_in_group(
        &mut self,
        preferences_group: &adw::PreferencesGroup,
        fact_rows: &[FileFactRow],
    ) {
        self.action_rows
            .drain(..)
            .for_each(|shown_row| preferences_group.remove(&shown_row));
        self.action_rows = fact_rows.iter().map(fact_action_row).collect();
        self.action_rows
            .iter()
            .for_each(|shown_row| preferences_group.add(shown_row));
    }
}

#[derive(Debug)]
pub enum LiveFactsLoading {
    NotRequested,
    Inspecting,
    Inspected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InspectionRequest {
    StartInspection,
    AlreadyRequested,
}

impl LiveFactsLoading {
    pub fn request_inspection(&mut self) -> InspectionRequest {
        match self {
            Self::NotRequested => {
                *self = Self::Inspecting;
                InspectionRequest::StartInspection
            }
            Self::Inspecting | Self::Inspected => InspectionRequest::AlreadyRequested,
        }
    }
}

pub async fn inspect_live_facts_in_background(file_path: PathBuf) -> FileFactsSource {
    spawn_blocking(move || inspect_live_file_facts(&file_path))
        .await
        .unwrap_or(FileFactsSource::Unreadable)
}

#[cfg(test)]
mod detected_file_row_tests {
    use super::*;

    #[test]
    fn inspection_starts_once() {
        let mut facts_loading = LiveFactsLoading::NotRequested;
        assert_eq!(
            facts_loading.request_inspection(),
            InspectionRequest::StartInspection
        );
        assert_eq!(
            facts_loading.request_inspection(),
            InspectionRequest::AlreadyRequested
        );
    }

    #[test]
    fn the_path_comes_first_among_the_expanded_rows() {
        let message_catalog = translated_messages::english_catalog();
        let expanded_rows = path_then_fact_rows(
            &message_catalog,
            Path::new("/home/alice/Downloads/invoice.exe"),
            vec![FileFactRow {
                fact_label: "Owner".to_owned(),
                fact_value: "alice (1000)".to_owned(),
            }],
        );
        assert_eq!(
            expanded_rows
                .iter()
                .map(|expanded_row| expanded_row.fact_value.as_str())
                .collect::<Vec<_>>(),
            ["/home/alice/Downloads/invoice.exe", "alice (1000)"]
        );
        assert_eq!(expanded_rows[0].fact_label, "Path");
    }
}
