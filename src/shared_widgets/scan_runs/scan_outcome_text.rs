use std::path::Path;

use crate::{
    host_services::clamav_scanner::{
        scan_progress_events::{
            CompletedScanOutcome, ScanProblem, ThreatFindings, UnscannedFileCause,
        },
        scan_target_enumeration::ScanTargetFailure,
        scanner_output_parser::{
            ScanFailureReason, ScanNotice, sanitize_scanner_line, sanitize_scanner_text,
        },
    },
    shared_widgets::{
        scan_runs::{
            scan_results_section::scan_results_state::{ProblemPathGroup, ScanResultsStatus},
            scan_run_progress::ScanEnding,
        },
        scanner_exit_text::abnormal_scanner_exit_text,
        scanner_start_failure_text::describe_start_failure,
    },
    translated_messages,
};

fn describe_target_failure(target_failure: &ScanTargetFailure) -> String {
    match target_failure {
        ScanTargetFailure::TargetMissing => translated_messages::widgets::scan_target_not_exist(),
        ScanTargetFailure::TargetNotScannable => {
            translated_messages::widgets::scan_target_not_regular()
        }
        ScanTargetFailure::TargetDirectoryUnreadable {
            target_path,
            directory_failure,
        } => translated_messages::widgets::unable_enumerate_scan_target(
            sanitize_scanner_line(&target_path.to_string_lossy()),
            directory_failure,
        ),
        ScanTargetFailure::FileListUnwritable(list_failure) => {
            translated_messages::widgets::unable_write_private_clamd(list_failure)
        }
        ScanTargetFailure::EnumerationInterrupted => {
            translated_messages::widgets::listing_scan_target_stopped()
        }
    }
}

fn describe_scan_problem(scan_problem: &ScanProblem) -> String {
    match scan_problem {
        ScanProblem::ScannerFailed(ScanFailureReason::NoFilesScanned) => {
            translated_messages::widgets::no_files_scanned()
        }
        ScanProblem::ScannerFailed(ScanFailureReason::ScannerReported { scanner_message }) => {
            sanitize_scanner_text(scanner_message)
        }
        ScanProblem::ScannerFailed(ScanFailureReason::ScannerExitedAbnormally(abnormal_exit)) => {
            abnormal_scanner_exit_text(abnormal_exit)
        }
        ScanProblem::FilesNotScanned {
            unscanned_paths,
            unscanned_cause,
        } => format!(
            "{} {}",
            unscanned_files_title(unscanned_paths.len()),
            describe_unscanned_cause(*unscanned_cause)
        ),
        ScanProblem::ScanPhaseUnstartable(start_failure) => describe_start_failure(start_failure),
        ScanProblem::TargetRejected(target_failure) => describe_target_failure(target_failure),
    }
}

fn unscanned_files_title(unscanned_count: usize) -> String {
    translated_messages::widgets::count_file_not_scanned(unscanned_count)
}

fn describe_unscanned_cause(unscanned_cause: UnscannedFileCause) -> String {
    match unscanned_cause {
        UnscannedFileCause::ClamdConnectionLost => {
            translated_messages::widgets::connection_clamd_lost_during()
        }
        UnscannedFileCause::ScannerStoppedReporting => {
            translated_messages::widgets::scanner_not_report_result()
        }
    }
}

fn is_listed_as_path_group(scan_problem: &ScanProblem) -> bool {
    matches!(scan_problem, ScanProblem::FilesNotScanned { .. })
}

pub fn scan_target_name(scan_target: &Path) -> String {
    sanitize_scanner_line(
        &scan_target
            .file_name()
            .unwrap_or(scan_target.as_os_str())
            .to_string_lossy(),
    )
}

fn describe_scan_notice(scan_notice: &ScanNotice) -> String {
    match scan_notice {
        ScanNotice::NoNotice => String::new(),
        ScanNotice::FilesInaccessible { file_count } => {
            translated_messages::widgets::count_not_accessed_title(*file_count)
        }
        ScanNotice::FilesUnreadable { file_count } => {
            translated_messages::widgets::count_file_not_read(*file_count)
        }
        ScanNotice::NonfatalWarnings { warning_count } => {
            translated_messages::widgets::count_non_fatal_warning(*warning_count)
        }
    }
}

pub fn scan_summary_text(scan_outcome: &CompletedScanOutcome) -> String {
    let threat_summary =
        (!scan_outcome.threat_findings.attributed_threats().is_empty()).then(|| {
            let threat_count = scan_outcome.threat_findings.attributed_threats().len();
            translated_messages::widgets::count_threat_found_outcome(threat_count)
        });
    let inspected_summary =
        translated_messages::widgets::count_file_scanned_outcome(scan_outcome.inspected_file_count);
    [inspected_summary]
        .into_iter()
        .chain(threat_summary)
        .chain(scan_notice_lines(scan_outcome))
        .collect::<Vec<_>>()
        .join("\n")
}

fn scan_notice_lines(scan_outcome: &CompletedScanOutcome) -> Vec<String> {
    notice_lines_where(scan_outcome, |_scan_problem| true, |_scan_notice| true)
}

pub fn unlisted_notice_lines(scan_outcome: &CompletedScanOutcome) -> Vec<String> {
    notice_lines_where(
        scan_outcome,
        |scan_problem| !is_listed_as_path_group(scan_problem),
        |scan_notice| {
            !matches!(
                scan_notice,
                ScanNotice::FilesInaccessible { .. } | ScanNotice::FilesUnreadable { .. }
            )
        },
    )
}

fn unlisted_skipped_paths(scan_outcome: &CompletedScanOutcome) -> Vec<String> {
    scan_outcome
        .skipped_paths
        .iter()
        .filter(|skipped_path| {
            !scan_outcome
                .threat_findings
                .attributed_threats()
                .iter()
                .any(|detected_threat| detected_threat.file_path == Path::new(skipped_path))
        })
        .cloned()
        .collect()
}

pub fn problem_path_groups(scan_outcome: &CompletedScanOutcome) -> Vec<ProblemPathGroup> {
    let unscanned_groups = scan_outcome
        .scan_problems
        .iter()
        .filter_map(|scan_problem| match scan_problem {
            ScanProblem::FilesNotScanned {
                unscanned_paths,
                unscanned_cause,
            } => Some(ProblemPathGroup {
                group_title: unscanned_files_title(unscanned_paths.len()),
                group_explanation: describe_unscanned_cause(*unscanned_cause),
                listed_paths: unscanned_paths
                    .iter()
                    .map(|unscanned_path| unscanned_path.to_string_lossy().into_owned())
                    .collect(),
            }),
            _other_problem => None,
        });
    let skipped_paths = unlisted_skipped_paths(scan_outcome);
    let skipped_group = (!skipped_paths.is_empty()).then(|| ProblemPathGroup {
        group_title: translated_messages::widgets::count_not_accessed_title(skipped_paths.len()),
        group_explanation: translated_messages::widgets::clamav_not_open_files(),
        listed_paths: skipped_paths,
    });
    unscanned_groups.chain(skipped_group).collect()
}

fn notice_lines_where(
    scan_outcome: &CompletedScanOutcome,
    include_problem: impl Fn(&ScanProblem) -> bool,
    include_notice: impl Fn(&ScanNotice) -> bool,
) -> Vec<String> {
    let unattributed_notice = match scan_outcome.threat_findings {
        ThreatFindings::UnattributedThreatsOnly => {
            Some(translated_messages::widgets::scanner_reported_threat_scan())
        }
        ThreatFindings::NoThreatsFound | ThreatFindings::AttributedThreats(_) => None,
    };
    unattributed_notice
        .into_iter()
        .chain(
            scan_outcome
                .scan_problems
                .iter()
                .filter(|scan_problem| include_problem(scan_problem))
                .map(describe_scan_problem),
        )
        .chain(
            scan_outcome
                .scan_notices
                .iter()
                .filter(|scan_notice| include_notice(scan_notice))
                .map(describe_scan_notice),
        )
        .chain(
            scan_outcome
                .nonfatal_warnings
                .iter()
                .map(|nonfatal_warning| sanitize_scanner_line(nonfatal_warning)),
        )
        .collect()
}

pub fn results_status(
    scan_outcome: &CompletedScanOutcome,
    scan_ending: ScanEnding,
) -> ScanResultsStatus {
    match scan_ending {
        _any_ending if scan_outcome.threats_were_found() => ScanResultsStatus::ThreatsDetected,
        ScanEnding::ScanCancelled => ScanResultsStatus::ScanCancelled,
        ScanEnding::ScanCompleted if scan_outcome.scan_problems.is_empty() => {
            ScanResultsStatus::NoThreatsDetected
        }
        ScanEnding::ScanCompleted => ScanResultsStatus::FinishedWithProblems,
    }
}
