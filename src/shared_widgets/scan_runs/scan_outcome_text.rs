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
    translated_messages::MessageCatalog,
};

fn describe_target_failure(
    message_catalog: &MessageCatalog,
    target_failure: &ScanTargetFailure,
) -> String {
    match target_failure {
        ScanTargetFailure::TargetMissing => {
            translated_messages::widgets::target_missing(message_catalog)
        }
        ScanTargetFailure::TargetNotScannable => {
            translated_messages::widgets::target_not_scannable(message_catalog)
        }
        ScanTargetFailure::TargetDirectoryUnreadable {
            target_path,
            directory_failure,
        } => translated_messages::widgets::target_directory_unreadable(
            message_catalog,
            sanitize_scanner_line(&target_path.to_string_lossy()),
            directory_failure,
        ),
        ScanTargetFailure::FileListUnwritable(list_failure) => {
            translated_messages::widgets::file_list_unwritable(message_catalog, list_failure)
        }
        ScanTargetFailure::EnumerationInterrupted => {
            translated_messages::widgets::enumeration_interrupted(message_catalog)
        }
    }
}

fn describe_scan_problem(message_catalog: &MessageCatalog, scan_problem: &ScanProblem) -> String {
    match scan_problem {
        ScanProblem::ScannerFailed(ScanFailureReason::NoFilesScanned) => {
            translated_messages::widgets::no_files_scanned(message_catalog)
        }
        ScanProblem::ScannerFailed(ScanFailureReason::ScannerReported { scanner_message }) => {
            sanitize_scanner_text(scanner_message)
        }
        ScanProblem::ScannerFailed(ScanFailureReason::ScannerExitedAbnormally(abnormal_exit)) => {
            abnormal_scanner_exit_text(message_catalog, abnormal_exit)
        }
        ScanProblem::FilesNotScanned {
            unscanned_paths,
            unscanned_cause,
        } => format!(
            "{} {}",
            unscanned_files_title(message_catalog, unscanned_paths.len()),
            describe_unscanned_cause(message_catalog, *unscanned_cause)
        ),
        ScanProblem::ScanPhaseUnstartable(start_failure) => {
            describe_start_failure(message_catalog, start_failure)
        }
        ScanProblem::TargetRejected(target_failure) => {
            describe_target_failure(message_catalog, target_failure)
        }
        ScanProblem::UserTrustUnreadable { failure_text } => {
            translated_messages::widgets::user_trust_unreadable(
                message_catalog,
                sanitize_scanner_text(failure_text),
            )
        }
    }
}

fn unscanned_files_title(message_catalog: &MessageCatalog, unscanned_count: usize) -> String {
    translated_messages::widgets::unscanned_files(message_catalog, unscanned_count)
}

fn describe_unscanned_cause(
    message_catalog: &MessageCatalog,
    unscanned_cause: UnscannedFileCause,
) -> String {
    match unscanned_cause {
        UnscannedFileCause::ClamdConnectionLost => {
            translated_messages::widgets::clamd_connection_lost(message_catalog)
        }
        UnscannedFileCause::ScannerStoppedReporting => {
            translated_messages::widgets::scanner_stopped_reporting(message_catalog)
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

fn describe_scan_notice(message_catalog: &MessageCatalog, scan_notice: &ScanNotice) -> String {
    match scan_notice {
        ScanNotice::NoNotice => String::new(),
        ScanNotice::FilesInaccessible { file_count } => {
            translated_messages::widgets::count_not_accessed_title(message_catalog, *file_count)
        }
        ScanNotice::FilesUnreadable { file_count } => {
            translated_messages::widgets::files_unreadable(message_catalog, *file_count)
        }
        ScanNotice::NonfatalWarnings { warning_count } => {
            translated_messages::widgets::nonfatal_warnings(message_catalog, *warning_count)
        }
    }
}

pub fn scan_summary_text(
    message_catalog: &MessageCatalog,
    scan_outcome: &CompletedScanOutcome,
) -> String {
    let threat_summary =
        (!scan_outcome.threat_findings.attributed_threats().is_empty()).then(|| {
            let threat_count = scan_outcome.threat_findings.attributed_threats().len();
            translated_messages::widgets::count_threat_found_outcome(message_catalog, threat_count)
        });
    let inspected_summary = translated_messages::widgets::count_file_scanned_outcome(
        message_catalog,
        scan_outcome.inspected_file_count,
    );
    [inspected_summary]
        .into_iter()
        .chain(threat_summary)
        .chain(scan_notice_lines(message_catalog, scan_outcome))
        .collect::<Vec<_>>()
        .join("\n")
}

fn scan_notice_lines(
    message_catalog: &MessageCatalog,
    scan_outcome: &CompletedScanOutcome,
) -> Vec<String> {
    notice_lines_where(
        message_catalog,
        scan_outcome,
        |_scan_problem| true,
        |_scan_notice| true,
    )
}

pub fn unlisted_notice_lines(
    message_catalog: &MessageCatalog,
    scan_outcome: &CompletedScanOutcome,
) -> Vec<String> {
    notice_lines_where(
        message_catalog,
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

pub fn problem_path_groups(
    message_catalog: &MessageCatalog,
    scan_outcome: &CompletedScanOutcome,
) -> Vec<ProblemPathGroup> {
    let unscanned_groups = scan_outcome
        .scan_problems
        .iter()
        .filter_map(|scan_problem| match scan_problem {
            ScanProblem::FilesNotScanned {
                unscanned_paths,
                unscanned_cause,
            } => Some(ProblemPathGroup {
                group_title: unscanned_files_title(message_catalog, unscanned_paths.len()),
                group_explanation: describe_unscanned_cause(message_catalog, *unscanned_cause),
                listed_paths: unscanned_paths
                    .iter()
                    .map(|unscanned_path| unscanned_path.to_string_lossy().into_owned())
                    .collect(),
            }),
            _other_problem => None,
        });
    let skipped_paths = unlisted_skipped_paths(scan_outcome);
    let skipped_group = (!skipped_paths.is_empty()).then(|| ProblemPathGroup {
        group_title: translated_messages::widgets::count_not_accessed_title(
            message_catalog,
            skipped_paths.len(),
        ),
        group_explanation: translated_messages::widgets::clamav_not_open_files(message_catalog),
        listed_paths: skipped_paths,
    });
    unscanned_groups.chain(skipped_group).collect()
}

fn notice_lines_where(
    message_catalog: &MessageCatalog,
    scan_outcome: &CompletedScanOutcome,
    include_problem: impl Fn(&ScanProblem) -> bool,
    include_notice: impl Fn(&ScanNotice) -> bool,
) -> Vec<String> {
    let unattributed_notice = match scan_outcome.threat_findings {
        ThreatFindings::UnattributedThreatsOnly => Some(
            translated_messages::widgets::unattributed_threats_only(message_catalog),
        ),
        ThreatFindings::NoThreatsFound | ThreatFindings::AttributedThreats(_) => None,
    };
    unattributed_notice
        .into_iter()
        .chain(
            scan_outcome
                .scan_problems
                .iter()
                .filter(|scan_problem| include_problem(scan_problem))
                .map(|scan_problem| describe_scan_problem(message_catalog, scan_problem)),
        )
        .chain(
            scan_outcome
                .scan_notices
                .iter()
                .filter(|scan_notice| include_notice(scan_notice))
                .map(|scan_notice| describe_scan_notice(message_catalog, scan_notice)),
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
