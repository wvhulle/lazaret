use std::time::Duration;

use crate::{
    host_services::clamav_scanner::{
        scan_progress_estimates::elapsed_duration_text,
        scanner_output_parser::{DetectedThreat, displayed_threat_name, sanitize_scanner_line},
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScanResultsStatus {
    ThreatsDetected,
    NoThreatsDetected,
    FinishedWithProblems,
    ScanCancelled,
}

#[derive(Clone, Debug)]
pub struct ProblemPathGroup {
    pub group_title: String,
    pub group_explanation: String,
    pub listed_paths: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ScanResultsView {
    pub results_status: ScanResultsStatus,
    pub inspected_file_count: u64,
    pub planned_file_count: u64,
    pub scanned_directory_count: u64,
    pub excluded_entry_count: u64,
    pub elapsed_time: Duration,
    pub detected_threats: Vec<DetectedThreat>,
    pub problem_path_groups: Vec<ProblemPathGroup>,
    pub notice_lines: Vec<String>,
    pub noteworthy_output: String,
}

impl ScanResultsView {
    pub const fn empty_results() -> Self {
        Self {
            results_status: ScanResultsStatus::NoThreatsDetected,
            inspected_file_count: 0,
            planned_file_count: 0,
            scanned_directory_count: 0,
            excluded_entry_count: 0,
            elapsed_time: Duration::ZERO,
            detected_threats: Vec::new(),
            problem_path_groups: Vec::new(),
            notice_lines: Vec::new(),
            noteworthy_output: String::new(),
        }
    }

    pub fn headline_text(&self, message_catalog: &MessageCatalog) -> String {
        match self.results_status {
            ScanResultsStatus::ThreatsDetected if !self.detected_threats.is_empty() => {
                threats_found_text(message_catalog, self.detected_threats.len())
            }
            other_status => status_title(message_catalog, other_status),
        }
    }

    pub fn summary_text(&self, message_catalog: &MessageCatalog) -> String {
        let scanned_text = if self.inspected_file_count < self.planned_file_count {
            translated_messages::widgets::done_total_file_scanned(
                message_catalog,
                self.planned_file_count,
                self.inspected_file_count,
            )
        } else {
            translated_messages::widgets::count_file_scanned_results(
                message_catalog,
                self.inspected_file_count,
            )
        };
        let excluded_text = (self.excluded_entry_count > 0).then(|| {
            translated_messages::widgets::count_entry_skipped_exclusions(
                message_catalog,
                self.excluded_entry_count,
            )
        });
        std::iter::once(scanned_text)
            .chain(excluded_text)
            .chain(std::iter::once(elapsed_duration_text(
                message_catalog,
                self.elapsed_time,
            )))
            .collect::<Vec<_>>()
            .join(" · ")
    }

    pub fn has_problems(&self) -> bool {
        !self.problem_path_groups.is_empty()
            || !self.notice_lines.is_empty()
            || !self.noteworthy_output.is_empty()
    }
}

pub fn status_title(message_catalog: &MessageCatalog, results_status: ScanResultsStatus) -> String {
    match results_status {
        ScanResultsStatus::ThreatsDetected => {
            translated_messages::widgets::threats_detected_status(message_catalog)
        }
        ScanResultsStatus::NoThreatsDetected => {
            translated_messages::widgets::no_threats_detected_status(message_catalog)
        }
        ScanResultsStatus::FinishedWithProblems => {
            translated_messages::widgets::finished_problems_status(message_catalog)
        }
        ScanResultsStatus::ScanCancelled => {
            translated_messages::widgets::scan_cancelled_status(message_catalog)
        }
    }
}

pub fn status_style_class(results_status: ScanResultsStatus) -> &'static str {
    match results_status {
        ScanResultsStatus::ThreatsDetected => "error",
        ScanResultsStatus::FinishedWithProblems => "warning",
        ScanResultsStatus::NoThreatsDetected => "success",
        ScanResultsStatus::ScanCancelled => "dim-label",
    }
}

pub fn status_icon(results_status: ScanResultsStatus) -> &'static str {
    match results_status {
        ScanResultsStatus::ThreatsDetected | ScanResultsStatus::FinishedWithProblems => {
            "dialog-warning-symbolic"
        }
        ScanResultsStatus::NoThreatsDetected => "object-select-symbolic",
        ScanResultsStatus::ScanCancelled => "process-stop-symbolic",
    }
}

fn threats_found_text(message_catalog: &MessageCatalog, threat_count: usize) -> String {
    translated_messages::widgets::count_threat_found_results(message_catalog, threat_count)
}

pub fn format_scan_report(
    message_catalog: &MessageCatalog,
    results_view: &ScanResultsView,
) -> String {
    let statistics_lines = [
        status_title(message_catalog, results_view.results_status),
        format!(
            "{}: {}",
            translated_messages::widgets::files_scanned(message_catalog),
            results_view.inspected_file_count
        ),
        format!(
            "{}: {}",
            translated_messages::widgets::files_planned(message_catalog),
            results_view.planned_file_count
        ),
        format!(
            "{}: {}",
            translated_messages::widgets::directories_label(message_catalog),
            results_view.scanned_directory_count
        ),
        format!(
            "{}: {}",
            translated_messages::widgets::threats_found(message_catalog),
            results_view.detected_threats.len()
        ),
    ];
    let threat_lines = results_view.detected_threats.iter().map(|detected_threat| {
        format!(
            "{}: {}",
            sanitize_scanner_line(&detected_threat.file_path.to_string_lossy()),
            displayed_threat_name(message_catalog, &detected_threat.threat_identity)
        )
    });
    let problem_sections = results_view.problem_path_groups.iter().map(|path_group| {
        std::iter::once(path_group.group_title.clone())
            .chain(
                path_group
                    .listed_paths
                    .iter()
                    .map(|listed_path| sanitize_scanner_line(listed_path)),
            )
            .collect::<Vec<_>>()
            .join("\n")
    });
    let report_sections = [
        statistics_lines.join("\n"),
        results_view.notice_lines.join("\n"),
        threat_lines.collect::<Vec<_>>().join("\n"),
    ]
    .into_iter()
    .chain(problem_sections)
    .chain(std::iter::once(
        results_view.noteworthy_output.trim_end().to_owned(),
    ));
    let mut formatted_report = report_sections
        .filter(|report_section| !report_section.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n");
    formatted_report.push('\n');
    formatted_report
}

pub fn fingerprint_failure_text(
    message_catalog: &MessageCatalog,
    fingerprint_failure: &std::io::Error,
) -> String {
    match fingerprint_failure.kind() {
        std::io::ErrorKind::NotFound => {
            translated_messages::widgets::file_no_longer_quarantine(message_catalog)
        }
        std::io::ErrorKind::PermissionDenied => {
            translated_messages::widgets::permission_denied(message_catalog)
        }
        _other_kind => {
            translated_messages::widgets::file_not_read_scan(message_catalog, fingerprint_failure)
        }
    }
}

#[cfg(test)]
mod scan_results_state_tests {
    use std::path::PathBuf;

    use super::*;
    use crate::host_services::clamav_scanner::scanner_output_parser::ThreatIdentity;

    fn results_with_threat() -> ScanResultsView {
        ScanResultsView {
            results_status: ScanResultsStatus::ThreatsDetected,
            inspected_file_count: 3,
            scanned_directory_count: 2,
            excluded_entry_count: 4,
            detected_threats: vec![DetectedThreat {
                file_path: PathBuf::from("/scan/bad\u{1b}[31m.txt"),
                threat_identity: ThreatIdentity::Signature {
                    signature_name: "Eicar-Test-Signature".to_owned(),
                },
            }],
            planned_file_count: 3,
            elapsed_time: Duration::from_secs(4),
            problem_path_groups: vec![ProblemPathGroup {
                group_title: "Files not accessible".to_owned(),
                group_explanation: String::new(),
                listed_paths: vec!["/scan/locked".to_owned()],
            }],
            notice_lines: vec!["1 file could not be accessed".to_owned()],
            noteworthy_output: "Scanned files: 3\n".to_owned(),
        }
    }

    #[test]
    fn the_report_lists_counts_threats_and_skipped_paths() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let formatted_report = format_scan_report(message_catalog, &results_with_threat());
        assert!(formatted_report.contains("Files scanned: 3"));
        assert!(formatted_report.contains("Directories: 2"));
        assert!(formatted_report.contains("/scan/bad.txt: Eicar-Test-Signature"));
        assert!(formatted_report.contains("/scan/locked"));
        assert!(formatted_report.contains("Scanned files: 3"));
    }

    #[test]
    fn the_report_removes_terminal_control_sequences_from_paths() {
        let message_catalog = &crate::translated_messages::english_catalog();
        assert!(!format_scan_report(message_catalog, &results_with_threat()).contains('\u{1b}'));
    }
}
