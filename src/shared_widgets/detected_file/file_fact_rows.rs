use std::path::{Path, PathBuf};

use bytesize::ByteSize;
use jiff::Timestamp;

use crate::{
    host_services::{
        clamav_scanner::scanner_output_parser::sanitize_scanner_line,
        desktop::locale_formatting::{MomentFormatters, byte_size_text, exact_byte_size_text},
        quarantine_vault::quarantine_entry_database::StoredDetectionDate,
    },
    shared_widgets::detected_file::{
        access_fact_rows::access_rows,
        detected_file_facts::{
            FileFactsSource, LiveFileFacts, ModificationMoment, RecordedQuarantineFacts,
        },
        file_fact_texts::{OwnerMention, account_numeric_id, group_account_text, owner_text},
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HoverDetail {
    NoHoverDetail,
    HoverText(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RowEmphasis {
    PlainRow,
    WarningRow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FolderActions {
    NoFolderActions,
    CopyPath(PathBuf),
    CopyAndReveal(PathBuf),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FactSection {
    LocationSection,
    AccessSection,
    DetailsSection,
}

pub const FACT_SECTIONS: [FactSection; 3] = [
    FactSection::LocationSection,
    FactSection::AccessSection,
    FactSection::DetailsSection,
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileFactRow {
    pub fact_section: FactSection,
    pub fact_label: String,
    pub fact_value: String,
    pub hover_detail: HoverDetail,
    pub row_emphasis: RowEmphasis,
    pub folder_actions: FolderActions,
}

pub fn fact_row(fact_section: FactSection, fact_label: String, fact_value: String) -> FileFactRow {
    FileFactRow {
        fact_section,
        fact_label,
        fact_value,
        hover_detail: HoverDetail::NoHoverDetail,
        row_emphasis: RowEmphasis::PlainRow,
        folder_actions: FolderActions::NoFolderActions,
    }
}

pub fn hovered_fact_row(
    fact_section: FactSection,
    fact_label: String,
    fact_value: String,
    hover_text: String,
) -> FileFactRow {
    FileFactRow {
        hover_detail: HoverDetail::HoverText(hover_text),
        ..fact_row(fact_section, fact_label, fact_value)
    }
}

pub fn section_title(message_catalog: &MessageCatalog, fact_section: FactSection) -> String {
    match fact_section {
        FactSection::LocationSection => {
            translated_messages::findings::location_section(message_catalog)
        }
        FactSection::AccessSection => {
            translated_messages::findings::access_section(message_catalog)
        }
        FactSection::DetailsSection => {
            translated_messages::findings::details_section(message_catalog)
        }
    }
}

pub fn rows_in_section(fact_rows: &[FileFactRow], fact_section: FactSection) -> Vec<FileFactRow> {
    fact_rows
        .iter()
        .filter(|fact_row| fact_row.fact_section == fact_section)
        .cloned()
        .collect()
}

fn size_row(message_catalog: &MessageCatalog, byte_size: ByteSize) -> FileFactRow {
    hovered_fact_row(
        FactSection::DetailsSection,
        translated_messages::findings::file_size(message_catalog),
        byte_size_text(byte_size),
        exact_byte_size_text(message_catalog, byte_size),
    )
}

fn moment_row(
    fact_label: String,
    moment_formatters: &MomentFormatters,
    shown_moment: Timestamp,
    reference_moment: Timestamp,
) -> FileFactRow {
    hovered_fact_row(
        FactSection::DetailsSection,
        fact_label,
        moment_formatters.relative_moment_text(shown_moment, reference_moment),
        moment_formatters.exact_moment_text(shown_moment),
    )
}

fn live_fact_rows(
    message_catalog: &MessageCatalog,
    moment_formatters: &MomentFormatters,
    reference_moment: Timestamp,
    live_facts: &LiveFileFacts,
) -> Vec<FileFactRow> {
    let shown_owner = owner_text(message_catalog, live_facts);
    let group_text = group_account_text(message_catalog, &live_facts.group_account);
    let owner_hover = translated_messages::findings::owner_hover(
        message_catalog,
        account_numeric_id(&live_facts.owner_account).to_string(),
        group_text.clone(),
        account_numeric_id(&live_facts.group_account).to_string(),
    );
    let owner_mention = OwnerMention::NamedOwner {
        owner_text: shown_owner.clone(),
        group_text,
    };
    let modified_label = translated_messages::findings::file_modified(message_catalog);
    let modified_row = match live_facts.modification_moment {
        ModificationMoment::ModifiedAt(modified_at) => moment_row(
            modified_label,
            moment_formatters,
            modified_at,
            reference_moment,
        ),
        ModificationMoment::OutsideCalendarRange => fact_row(
            FactSection::DetailsSection,
            modified_label,
            translated_messages::findings::unknown_label(message_catalog),
        ),
    };
    std::iter::once(hovered_fact_row(
        FactSection::AccessSection,
        translated_messages::findings::file_owner(message_catalog),
        shown_owner,
        owner_hover,
    ))
    .chain(access_rows(
        message_catalog,
        &owner_mention,
        live_facts.permission_mode,
    ))
    .chain([
        size_row(message_catalog, live_facts.byte_size),
        modified_row,
    ])
    .collect()
}

fn recorded_fact_rows(
    message_catalog: &MessageCatalog,
    moment_formatters: &MomentFormatters,
    reference_moment: Timestamp,
    recorded_facts: &RecordedQuarantineFacts,
) -> Vec<FileFactRow> {
    let detected_label = translated_messages::findings::detected_label(message_catalog);
    let detected_row = match &recorded_facts.detection_date {
        StoredDetectionDate::DetectedAt(detected_at) => moment_row(
            detected_label,
            moment_formatters,
            *detected_at,
            reference_moment,
        ),
        StoredDetectionDate::UnreadableText(stored_text) => fact_row(
            FactSection::DetailsSection,
            detected_label,
            sanitize_scanner_line(stored_text),
        ),
    };
    std::iter::once(detected_row)
        .chain(access_rows(
            message_catalog,
            &OwnerMention::UnrecordedOwner,
            recorded_facts.permission_mode,
        ))
        .chain([size_row(message_catalog, recorded_facts.byte_size)])
        .collect()
}

fn shown_folder(file_path: &Path) -> String {
    let folder_path = file_path
        .parent()
        .filter(|parent_path| !parent_path.as_os_str().is_empty())
        .unwrap_or(file_path);
    sanitize_scanner_line(&folder_path.to_string_lossy())
}

pub fn folder_row(
    message_catalog: &MessageCatalog,
    file_path: &Path,
    folder_actions: FolderActions,
) -> FileFactRow {
    FileFactRow {
        folder_actions,
        ..hovered_fact_row(
            FactSection::LocationSection,
            translated_messages::findings::file_folder(message_catalog),
            shown_folder(file_path),
            sanitize_scanner_line(&file_path.to_string_lossy()),
        )
    }
}

pub fn detected_file_rows(
    message_catalog: &MessageCatalog,
    moment_formatters: &MomentFormatters,
    reference_moment: Timestamp,
    file_path: &Path,
    facts_source: &FileFactsSource,
) -> Vec<FileFactRow> {
    let copied_path = file_path.to_path_buf();
    let (folder_actions, fact_rows) = match facts_source {
        FileFactsSource::LiveSnapshot(live_facts) => (
            FolderActions::CopyAndReveal(copied_path),
            live_fact_rows(
                message_catalog,
                moment_formatters,
                reference_moment,
                live_facts,
            ),
        ),
        FileFactsSource::RecordedAtQuarantine(recorded_facts) => (
            FolderActions::CopyPath(copied_path),
            recorded_fact_rows(
                message_catalog,
                moment_formatters,
                reference_moment,
                recorded_facts,
            ),
        ),
        FileFactsSource::NoLongerThere => (
            FolderActions::CopyPath(copied_path),
            vec![fact_row(
                FactSection::LocationSection,
                translated_messages::findings::file_status(message_catalog),
                translated_messages::findings::no_longer_there_detection(message_catalog),
            )],
        ),
        FileFactsSource::Unreadable => (
            FolderActions::CopyPath(copied_path),
            vec![fact_row(
                FactSection::LocationSection,
                translated_messages::findings::file_status(message_catalog),
                translated_messages::findings::state_unknown_detection(message_catalog),
            )],
        ),
    };
    std::iter::once(folder_row(message_catalog, file_path, folder_actions))
        .chain(fact_rows)
        .collect()
}

pub fn facts_source_description(
    message_catalog: &MessageCatalog,
    facts_source: &FileFactsSource,
) -> String {
    match facts_source {
        FileFactsSource::LiveSnapshot(_)
        | FileFactsSource::NoLongerThere
        | FileFactsSource::Unreadable => {
            translated_messages::findings::file_facts_live(message_catalog)
        }
        FileFactsSource::RecordedAtQuarantine(_) => {
            translated_messages::findings::file_facts_recorded(message_catalog)
        }
    }
}

#[cfg(test)]
pub mod file_fact_rows_tests {
    use super::*;
    use crate::{
        host_services::protection_evidence::protection_evidence_fixtures::english_formatters,
        shared_widgets::detected_file::detected_file_facts::live_facts_fixtures::alice_live_facts,
    };

    const REFERENCE_MOMENT: Timestamp = Timestamp::constant(1_793_955_622, 0);

    pub fn rows_for(file_path: &str, facts_source: &FileFactsSource) -> Vec<FileFactRow> {
        detected_file_rows(
            &translated_messages::english_catalog(),
            &english_formatters(),
            REFERENCE_MOMENT,
            Path::new(file_path),
            facts_source,
        )
    }

    pub fn labels_and_values(fact_rows: &[FileFactRow]) -> Vec<(&str, &str)> {
        fact_rows
            .iter()
            .map(|shown_row| (shown_row.fact_label.as_str(), shown_row.fact_value.as_str()))
            .collect()
    }

    #[test]
    fn a_live_file_reads_in_plain_language() {
        let fact_rows = rows_for(
            "/home/alice/Downloads/invoice.exe",
            &FileFactsSource::LiveSnapshot(alice_live_facts(0o100_640)),
        );
        assert_eq!(
            labels_and_values(&fact_rows)[..5],
            [
                ("Folder", "/home/alice/Downloads"),
                ("Owner", "alice (you)"),
                ("Who can open it", "alice (you) and members of 'users'"),
                ("Who can change it", "Only alice (you)"),
                ("Can run as a program", "No"),
            ]
        );
        assert_eq!(
            fact_rows[0].hover_detail,
            HoverDetail::HoverText("/home/alice/Downloads/invoice.exe".to_owned())
        );
        assert_eq!(
            fact_rows[0].folder_actions,
            FolderActions::CopyAndReveal(PathBuf::from("/home/alice/Downloads/invoice.exe"))
        );
        assert_eq!(
            fact_rows[2].hover_detail,
            HoverDetail::HoverText("-rw-r-----".to_owned())
        );
        assert_eq!(fact_rows[5].fact_label, "Size");
        assert_eq!(fact_rows[6].fact_label, "Modified");
    }

    #[test]
    fn a_quarantined_file_speaks_of_its_owner_and_cannot_be_revealed() {
        let fact_rows = rows_for(
            "/home/user/old.bin",
            &FileFactsSource::RecordedAtQuarantine(RecordedQuarantineFacts {
                permission_mode: 0o600,
                byte_size: ByteSize::b(68),
                detection_date: StoredDetectionDate::UnreadableText("yesterday".to_owned()),
            }),
        );
        assert_eq!(
            labels_and_values(&fact_rows),
            [
                ("Folder", "/home/user"),
                ("Detected", "yesterday"),
                ("Who can open it", "Only its owner"),
                ("Who can change it", "Only its owner"),
                ("Can run as a program", "No"),
                ("Size", "68 B"),
            ]
        );
        assert_eq!(
            fact_rows[0].folder_actions,
            FolderActions::CopyPath(PathBuf::from("/home/user/old.bin"))
        );
    }

    #[test]
    fn missing_and_unreadable_files_show_their_folder_and_status() {
        assert_eq!(
            labels_and_values(&rows_for("/tmp/gone.exe", &FileFactsSource::NoLongerThere)),
            [("Folder", "/tmp"), ("Status", "No longer there")]
        );
        assert_eq!(rows_for("/tmp/x", &FileFactsSource::Unreadable).len(), 2);
    }
}
