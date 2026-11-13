use crate::{
    shared_widgets::detected_file::{
        file_access_summary::{AccessAudience, ProgramRunning, file_access_summary},
        file_fact_rows::{FileFactRow, RowEmphasis, hovered_fact_row},
        file_fact_texts::{OwnerMention, audience_text, running_text},
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

const fn audience_emphasis(access_audience: AccessAudience) -> RowEmphasis {
    match access_audience {
        AccessAudience::Everyone => RowEmphasis::WarningRow,
        AccessAudience::OwnerAndGroup
        | AccessAudience::OnlyOwner
        | AccessAudience::OnlyAdministrator => RowEmphasis::PlainRow,
    }
}

const fn running_emphasis(program_running: ProgramRunning) -> RowEmphasis {
    match program_running {
        ProgramRunning::RunsWithOwnerRights | ProgramRunning::RunsWithGroupRights => {
            RowEmphasis::WarningRow
        }
        ProgramRunning::NotRunnable | ProgramRunning::Runnable(_) => RowEmphasis::PlainRow,
    }
}

pub fn access_rows(
    message_catalog: &MessageCatalog,
    owner_mention: &OwnerMention,
    permission_mode: u32,
) -> Vec<FileFactRow> {
    let raw_mode_text = unix_mode::to_string(permission_mode);
    if unix_mode::Type::from(permission_mode) == unix_mode::Type::Symlink {
        return vec![hovered_fact_row(
            translated_messages::findings::kind_label(message_catalog),
            translated_messages::findings::kind_link(message_catalog),
            raw_mode_text,
        )];
    }
    let access_summary = file_access_summary(permission_mode);
    vec![
        FileFactRow {
            row_emphasis: audience_emphasis(access_summary.opening),
            ..hovered_fact_row(
                translated_messages::findings::who_can_open(message_catalog),
                audience_text(message_catalog, owner_mention, access_summary.opening),
                raw_mode_text.clone(),
            )
        },
        FileFactRow {
            row_emphasis: audience_emphasis(access_summary.changing),
            ..hovered_fact_row(
                translated_messages::findings::who_can_change(message_catalog),
                audience_text(message_catalog, owner_mention, access_summary.changing),
                raw_mode_text.clone(),
            )
        },
        FileFactRow {
            row_emphasis: running_emphasis(access_summary.running),
            ..hovered_fact_row(
                translated_messages::findings::can_run_program(message_catalog),
                running_text(message_catalog, owner_mention, access_summary.running),
                raw_mode_text,
            )
        },
    ]
}

#[cfg(test)]
mod access_fact_rows_tests {
    use crate::shared_widgets::detected_file::{
        detected_file_facts::{FileFactsSource, live_facts_fixtures::alice_live_facts},
        file_fact_rows::{
            RowEmphasis,
            file_fact_rows_tests::{labels_and_values, rows_for},
        },
    };

    #[test]
    fn world_writable_and_setuid_files_are_emphasised() {
        let fact_rows = rows_for(
            "/srv/share/tool",
            &FileFactsSource::LiveSnapshot(alice_live_facts(0o104_757)),
        );
        let warned_labels = fact_rows
            .iter()
            .filter(|shown_row| shown_row.row_emphasis == RowEmphasis::WarningRow)
            .map(|shown_row| shown_row.fact_label.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            warned_labels,
            [
                "Who can open it",
                "Who can change it",
                "Can run as a program"
            ]
        );
        assert_eq!(
            fact_rows[4].fact_value,
            "Yes, with the rights of alice (you)"
        );
    }

    #[test]
    fn a_symlink_is_named_instead_of_its_meaningless_mode() {
        let fact_rows = rows_for(
            "/home/alice/link",
            &FileFactsSource::LiveSnapshot(alice_live_facts(0o120_777)),
        );
        assert_eq!(
            labels_and_values(&fact_rows)[2],
            ("Kind", "Link to another file")
        );
        assert!(
            fact_rows
                .iter()
                .all(|shown_row| shown_row.fact_label != "Who can open it")
        );
    }
}
