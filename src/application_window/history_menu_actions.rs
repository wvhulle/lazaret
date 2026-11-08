use relm4::{adw, adw::prelude::*};

use crate::{
    host_services::activity_history::activity_history_database::ActivityEntryRecord,
    shared_widgets::{
        activity_entries::activity_export_formats::{
            activity_entries_as_csv, activity_entries_as_json,
        },
        awaited_dialogs::{AlertChoice, add_alert_choice, add_dismissing_choice},
    },
    translated_messages,
};

pub const EXPORT_HISTORY_ACTION: &str = "win.export-history";
pub const CLEAR_HISTORY_ACTION: &str = "win.clear-history";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HistoryExportFormat {
    CsvExport,
    JsonExport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HistoryExportChoice {
    FormatChosen(HistoryExportFormat),
    ExportDismissed,
}

impl AlertChoice for HistoryExportChoice {
    const ANSWERING_CHOICES: &'static [Self] = &[
        Self::FormatChosen(HistoryExportFormat::CsvExport),
        Self::FormatChosen(HistoryExportFormat::JsonExport),
    ];
    const DISMISSING_CHOICE: Self = Self::ExportDismissed;

    fn response_id(self) -> &'static str {
        match self {
            Self::FormatChosen(HistoryExportFormat::CsvExport) => "csv",
            Self::FormatChosen(HistoryExportFormat::JsonExport) => "json",
            Self::ExportDismissed => "cancel",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClearHistoryChoice {
    ClearAllChosen,
    ClearCancelled,
}

impl AlertChoice for ClearHistoryChoice {
    const ANSWERING_CHOICES: &'static [Self] = &[Self::ClearAllChosen];
    const DISMISSING_CHOICE: Self = Self::ClearCancelled;

    fn response_id(self) -> &'static str {
        match self {
            Self::ClearAllChosen => "clear",
            Self::ClearCancelled => "cancel",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportedHistoryFile {
    pub suggested_name: &'static str,
    pub exported_text: String,
}

pub fn exported_history_file(
    export_format: HistoryExportFormat,
    activity_entries: &[ActivityEntryRecord],
) -> serde_json::Result<ExportedHistoryFile> {
    Ok(match export_format {
        HistoryExportFormat::CsvExport => ExportedHistoryFile {
            suggested_name: "lazaret-history.csv",
            exported_text: activity_entries_as_csv(activity_entries),
        },
        HistoryExportFormat::JsonExport => ExportedHistoryFile {
            suggested_name: "lazaret-history.json",
            exported_text: activity_entries_as_json(activity_entries)?,
        },
    })
}

pub fn primary_menu_entries() -> Vec<(String, &'static str)> {
    vec![
        (
            translated_messages::window::export_history_menu_item(),
            EXPORT_HISTORY_ACTION,
        ),
        (
            translated_messages::window::clear_history_menu_item(),
            CLEAR_HISTORY_ACTION,
        ),
        (
            translated_messages::window::preferences_label(),
            "app.preferences",
        ),
        (translated_messages::window::about_menu_item(), "app.about"),
        (translated_messages::window::quit_menu_item(), "app.quit"),
    ]
}

pub fn history_export_dialog() -> adw::AlertDialog {
    let export_dialog = adw::AlertDialog::new(
        Some(&translated_messages::window::export_history_heading()),
        Some(&translated_messages::window::export_history_body()),
    );
    add_dismissing_choice::<HistoryExportChoice>(
        &export_dialog,
        &translated_messages::window::cancel_history_button(),
    );
    add_alert_choice(
        &export_dialog,
        HistoryExportChoice::FormatChosen(HistoryExportFormat::JsonExport),
        &translated_messages::window::json_label(),
    );
    add_alert_choice(
        &export_dialog,
        HistoryExportChoice::FormatChosen(HistoryExportFormat::CsvExport),
        &translated_messages::window::csv_label(),
    );
    export_dialog
}

pub fn clear_history_dialog() -> adw::AlertDialog {
    let confirmation_dialog = adw::AlertDialog::new(
        Some(&translated_messages::window::clear_history_heading()),
        Some(&translated_messages::window::clear_history_body()),
    );
    add_dismissing_choice::<ClearHistoryChoice>(
        &confirmation_dialog,
        &translated_messages::window::cancel_history_button(),
    );
    add_alert_choice(
        &confirmation_dialog,
        ClearHistoryChoice::ClearAllChosen,
        &translated_messages::window::clear_label(),
    );
    confirmation_dialog.set_response_appearance(
        ClearHistoryChoice::ClearAllChosen.response_id(),
        adw::ResponseAppearance::Destructive,
    );
    confirmation_dialog
}

#[cfg(test)]
mod history_menu_actions_tests {
    use super::*;
    use crate::shared_widgets::awaited_dialogs::alert_choice_for_response;

    #[test]
    fn the_primary_menu_offers_export_and_clear_of_the_whole_history() {
        let menu_actions = primary_menu_entries()
            .into_iter()
            .map(|(_entry_label, action_name)| action_name)
            .collect::<Vec<_>>();
        assert_eq!(
            menu_actions,
            vec![
                EXPORT_HISTORY_ACTION,
                CLEAR_HISTORY_ACTION,
                "app.preferences",
                "app.about",
                "app.quit"
            ]
        );
        assert_eq!(primary_menu_entries()[0].0, "Export History…");
        assert_eq!(primary_menu_entries()[1].0, "Clear History…");
    }

    #[test]
    fn each_export_answer_picks_its_format_and_anything_else_cancels() {
        assert_eq!(
            alert_choice_for_response::<HistoryExportChoice>("csv"),
            HistoryExportChoice::FormatChosen(HistoryExportFormat::CsvExport)
        );
        assert_eq!(
            alert_choice_for_response::<HistoryExportChoice>("json"),
            HistoryExportChoice::FormatChosen(HistoryExportFormat::JsonExport)
        );
        assert_eq!(
            alert_choice_for_response::<HistoryExportChoice>("close"),
            HistoryExportChoice::ExportDismissed
        );
    }

    #[test]
    fn only_the_clear_answer_clears_the_history() {
        assert_eq!(
            alert_choice_for_response::<ClearHistoryChoice>("clear"),
            ClearHistoryChoice::ClearAllChosen
        );
        assert_eq!(
            alert_choice_for_response::<ClearHistoryChoice>("cancel"),
            ClearHistoryChoice::ClearCancelled
        );
    }

    #[test]
    fn the_export_writes_the_same_csv_and_json_as_the_activity_formats() {
        let csv_file = exported_history_file(HistoryExportFormat::CsvExport, &[]).unwrap();
        assert_eq!(csv_file.suggested_name, "lazaret-history.csv");
        assert_eq!(csv_file.exported_text, activity_entries_as_csv(&[]));
        let json_file = exported_history_file(HistoryExportFormat::JsonExport, &[]).unwrap();
        assert_eq!(json_file.suggested_name, "lazaret-history.json");
        assert_eq!(
            json_file.exported_text,
            activity_entries_as_json(&[]).unwrap()
        );
    }
}
