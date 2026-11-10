use std::time::{Instant, SystemTime};

use jiff::Timestamp;

use crate::{
    host_services::{
        activity_history::activity_history_database::{
            ActivityRecord, NewActivityEntry, UpdateActivityStatus,
        },
        clamav_programs::{
            component_availability::{ProgramProbe, probe_located_program_version},
            database_directory_status::{
                DatabaseDirectoryStatus, FreshclamServiceState, configured_database_directory,
                detect_freshclam_service, inspect_database_directory,
            },
            trusted_program_lookup::{ClamavProgram, ProgramLocation, locate_trusted_program},
        },
        clamav_scanner::{
            scanner_exit_classification::AbnormalScannerExit,
            scanner_output_parser::{sanitize_scanner_line, sanitize_scanner_text},
            scanner_process_events::ScanStartFailure,
        },
        desktop::flatpak_sandbox::SandboxKind,
        systemd_bus::host_systemd_units::SystemBusAccess,
    },
    shared_widgets::{
        component_streams::RunningStream,
        host_read_failure_text::{database_folder_failure_text, unit_journal_failure_text},
        scanner_start_failure_text::describe_start_failure as describe_scan_start_failure,
    },
    signatures::database_updates_page::signature_update_run::{
        database_update_events::{
            DatabaseUpdateEvent, DatabaseUpdatePlan, UpdaterFinish, plan_database_update,
        },
        freshclam_output_parser::{
            DatabaseUpdateOutcome, FreshclamReportSummary, RateLimitCooldown, UpdateFailureReason,
            interpret_unfinished_update, interpret_updater_finish,
        },
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

const TRANSCRIPT_LIMIT: usize = 4 * 1024 * 1024;

#[derive(Debug)]
pub struct DatabaseHostStatus {
    freshclam_probe: ProgramProbe,
    service_state: FreshclamServiceState,
    directory_status: DatabaseDirectoryStatus,
}

#[cfg(test)]
impl DatabaseHostStatus {
    pub fn installed_with_service(service_state: FreshclamServiceState) -> Self {
        Self {
            freshclam_probe: ProgramProbe::VersionReported {
                version_text: "ClamAV 1.4.3".to_owned(),
            },
            service_state,
            directory_status: DatabaseDirectoryStatus::DefinitionsMissing {
                database_directory: std::path::PathBuf::from("/var/lib/clamav"),
            },
        }
    }
}

#[derive(Clone, Copy)]
struct UpdateTiming {
    started_at: Timestamp,
    started_instant: Instant,
}

impl UpdateTiming {
    fn starting_now() -> Self {
        Self {
            started_at: Timestamp::now(),
            started_instant: Instant::now(),
        }
    }
}

enum LatestUpdaterLine {
    NothingPrinted,
    LinePrinted(String),
}

struct UpdateRunOutput {
    update_transcript: String,
    latest_updater_line: LatestUpdaterLine,
}

impl UpdateRunOutput {
    const fn nothing_printed() -> Self {
        Self {
            update_transcript: String::new(),
            latest_updater_line: LatestUpdaterLine::NothingPrinted,
        }
    }

    fn append_output_line(&mut self, output_line: &str) {
        let displayed_line = sanitize_scanner_line(output_line);
        if !displayed_line.trim().is_empty() {
            self.latest_updater_line =
                LatestUpdaterLine::LinePrinted(displayed_line.trim().to_owned());
        }
        if self.update_transcript.len() + displayed_line.len() < TRANSCRIPT_LIMIT {
            self.update_transcript.push_str(&displayed_line);
            self.update_transcript.push('\n');
        }
    }

    fn latest_line_text(&self, message_catalog: &MessageCatalog) -> String {
        match &self.latest_updater_line {
            LatestUpdaterLine::NothingPrinted => {
                translated_messages::signatures::starting_update(message_catalog)
            }
            LatestUpdaterLine::LinePrinted(output_line) => output_line.clone(),
        }
    }
}

enum ActiveUpdateRun {
    UpdateRunning {
        update_plan: DatabaseUpdatePlan,
        update_timing: UpdateTiming,
        run_output: UpdateRunOutput,
        _update_stream: RunningStream,
    },
    UpdaterFinished {
        update_timing: UpdateTiming,
        run_output: UpdateRunOutput,
        updater_finish: UpdaterFinish,
        _update_stream: RunningStream,
    },
    CancellingUpdate {
        update_timing: UpdateTiming,
        run_output: UpdateRunOutput,
    },
}

struct EndedUpdateRun {
    update_timing: UpdateTiming,
    run_output: UpdateRunOutput,
    update_outcome: DatabaseUpdateOutcome,
}

impl ActiveUpdateRun {
    const fn run_output_mut(&mut self) -> &mut UpdateRunOutput {
        match self {
            Self::UpdateRunning { run_output, .. }
            | Self::UpdaterFinished { run_output, .. }
            | Self::CancellingUpdate { run_output, .. } => run_output,
        }
    }

    fn cancel_requested(self) -> Self {
        match self {
            Self::UpdateRunning {
                update_plan: DatabaseUpdatePlan::RunFreshclamDirectly,
                update_timing,
                run_output,
                ..
            } => Self::CancellingUpdate {
                update_timing,
                run_output,
            },
            uncancellable_run @ (Self::UpdateRunning {
                update_plan:
                    DatabaseUpdatePlan::SignalRunningDaemon { .. }
                    | DatabaseUpdatePlan::StartServiceUnit { .. },
                ..
            }
            | Self::UpdaterFinished { .. }
            | Self::CancellingUpdate { .. }) => uncancellable_run,
        }
    }

    fn updater_finished(self, updater_finish: UpdaterFinish) -> Self {
        match self {
            Self::UpdateRunning {
                update_timing,
                run_output,
                _update_stream,
                ..
            } => Self::UpdaterFinished {
                update_timing,
                run_output,
                updater_finish,
                _update_stream,
            },
            settled_run @ (Self::UpdaterFinished { .. } | Self::CancellingUpdate { .. }) => {
                settled_run
            }
        }
    }

    fn absorb_update_event(
        mut self,
        message_catalog: &MessageCatalog,
        update_event: DatabaseUpdateEvent,
    ) -> Self {
        match update_event {
            DatabaseUpdateEvent::UpdateOutputLine(output_line) => {
                self.run_output_mut().append_output_line(&output_line);
                self
            }
            DatabaseUpdateEvent::UpdateOutputUnreadable(read_failure) => {
                self.run_output_mut()
                    .append_output_line(&unit_journal_failure_text(message_catalog, read_failure));
                self
            }
            DatabaseUpdateEvent::UpdaterFinished(updater_finish) => {
                self.updater_finished(updater_finish)
            }
        }
    }

    fn into_ended_run(self) -> EndedUpdateRun {
        match self {
            Self::UpdateRunning {
                update_timing,
                run_output,
                ..
            } => EndedUpdateRun {
                update_outcome: interpret_unfinished_update(&run_output.update_transcript),
                update_timing,
                run_output,
            },
            Self::UpdaterFinished {
                update_timing,
                run_output,
                updater_finish,
                ..
            } => EndedUpdateRun {
                update_outcome: interpret_updater_finish(
                    &run_output.update_transcript,
                    updater_finish,
                ),
                update_timing,
                run_output,
            },
            Self::CancellingUpdate {
                update_timing,
                run_output,
            } => EndedUpdateRun {
                update_outcome: DatabaseUpdateOutcome::UpdateCancelled,
                update_timing,
                run_output,
            },
        }
    }

    fn into_finished_report(self, message_catalog: &MessageCatalog) -> FinishedUpdateReport {
        let ended_run = self.into_ended_run();
        FinishedUpdateReport {
            summary_text: describe_update_outcome(message_catalog, &ended_run.update_outcome),
            definitions_changed: definitions_changed(&ended_run.update_outcome),
            activity_entry: NewActivityEntry {
                started_at: ended_run.update_timing.started_at,
                activity_record: ActivityRecord::UpdateActivity {
                    update_status: update_status_of(&ended_run.update_outcome),
                },
                duration_millis: u64::try_from(
                    ended_run
                        .update_timing
                        .started_instant
                        .elapsed()
                        .as_millis(),
                )
                .unwrap_or(u64::MAX),
                activity_output: ended_run.run_output.update_transcript,
            },
        }
    }

    fn update_progress(&self, message_catalog: &MessageCatalog) -> SignatureUpdateProgress {
        match self {
            Self::UpdateRunning {
                update_plan: DatabaseUpdatePlan::RunFreshclamDirectly,
                run_output,
                ..
            } => SignatureUpdateProgress::CancellableUpdateRunning {
                latest_output_line: run_output.latest_line_text(message_catalog),
            },
            Self::UpdateRunning {
                update_plan:
                    DatabaseUpdatePlan::SignalRunningDaemon { .. }
                    | DatabaseUpdatePlan::StartServiceUnit { .. },
                run_output,
                ..
            }
            | Self::UpdaterFinished { run_output, .. } => {
                SignatureUpdateProgress::UncancellableUpdateRunning {
                    latest_output_line: run_output.latest_line_text(message_catalog),
                }
            }
            Self::CancellingUpdate { .. } => SignatureUpdateProgress::UpdateCancelling,
        }
    }
}

enum DatabaseUpdateState {
    NoActiveUpdate,
    ActiveUpdate(Box<ActiveUpdateRun>),
}

enum HostStatusView {
    StatusLoading,
    StatusLoaded(DatabaseHostStatus),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SignatureUpdateProgress {
    UpdateUnavailable,
    ReadyToUpdate,
    CancellableUpdateRunning { latest_output_line: String },
    UncancellableUpdateRunning { latest_output_line: String },
    UpdateCancelling,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LastManualUpdate {
    NotRunThisSession,
    FinishedAt {
        summary_line: String,
        finished_at: Timestamp,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KnownUpdateHost {
    pub update_progress: SignatureUpdateProgress,
    pub last_manual_update: LastManualUpdate,
    pub update_service_text: String,
    pub database_folder_text: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SignatureUpdateView {
    HostStatusLoading,
    HostStatusKnown(KnownUpdateHost),
}

#[derive(Debug)]
pub struct FinishedUpdateReport {
    pub summary_text: String,
    pub definitions_changed: bool,
    pub activity_entry: NewActivityEntry,
}

#[derive(Debug)]
pub enum UpdateStreamEnding {
    UpdateEnded(FinishedUpdateReport),
    NoUpdateWasRunning,
}

fn service_description(
    message_catalog: &MessageCatalog,
    service_state: FreshclamServiceState,
) -> String {
    match service_state {
        FreshclamServiceState::DaemonRunning { .. } => {
            translated_messages::signatures::daemon_running(message_catalog)
        }
        FreshclamServiceState::TimerScheduled { .. } => {
            translated_messages::signatures::timer_scheduled(message_catalog)
        }
        FreshclamServiceState::ServiceStopped { .. } => {
            translated_messages::signatures::service_stopped(message_catalog)
        }
        FreshclamServiceState::ServiceNotFound => {
            translated_messages::signatures::service_not_found(message_catalog)
        }
        FreshclamServiceState::SystemdUnreachable => {
            translated_messages::signatures::systemd_unreachable_service(message_catalog)
        }
    }
}

fn directory_description(
    message_catalog: &MessageCatalog,
    directory_status: &DatabaseDirectoryStatus,
) -> String {
    match directory_status {
        DatabaseDirectoryStatus::DefinitionsAvailable {
            newest_definition_age_days,
            ..
        } => translated_messages::signatures::count_day_old(
            message_catalog,
            *newest_definition_age_days,
        ),
        DatabaseDirectoryStatus::DefinitionsMissing { .. } => {
            translated_messages::signatures::definitions_missing(message_catalog)
        }
        DatabaseDirectoryStatus::DirectoryUnreadable { read_failure, .. } => {
            translated_messages::signatures::directory_unreadable(
                message_catalog,
                database_folder_failure_text(message_catalog, *read_failure),
            )
        }
    }
}

fn directory_path_text(directory_status: &DatabaseDirectoryStatus) -> String {
    match directory_status {
        DatabaseDirectoryStatus::DefinitionsAvailable {
            database_directory, ..
        }
        | DatabaseDirectoryStatus::DefinitionsMissing { database_directory }
        | DatabaseDirectoryStatus::DirectoryUnreadable {
            database_directory, ..
        } => sanitize_scanner_line(&database_directory.to_string_lossy()),
    }
}

fn describe_abnormal_freshclam_exit(
    message_catalog: &MessageCatalog,
    abnormal_exit: AbnormalScannerExit,
) -> String {
    match abnormal_exit {
        AbnormalScannerExit::ScannerError { exit_code } => {
            translated_messages::signatures::scanner_error(message_catalog, exit_code)
        }
        AbnormalScannerExit::KilledBySignal { signal_number } => {
            translated_messages::signatures::killed_signal(message_catalog, signal_number)
        }
        AbnormalScannerExit::UnrecognizedWaitStatus { .. } => {
            translated_messages::signatures::unrecognized_wait_status(message_catalog)
        }
        AbnormalScannerExit::WaitFailed { .. } => {
            translated_messages::signatures::wait_failed(message_catalog)
        }
    }
}

fn describe_failure_reason(
    message_catalog: &MessageCatalog,
    failure_reason: &UpdateFailureReason,
) -> String {
    match failure_reason {
        UpdateFailureReason::AuthorizationRefused => {
            translated_messages::signatures::authorization_refused(message_catalog)
        }
        UpdateFailureReason::SystemdUnreachable(_bus_failure) => {
            translated_messages::signatures::systemd_not_reached(message_catalog)
        }
        UpdateFailureReason::ServiceJobEnded { job_result } => {
            translated_messages::signatures::service_job_ended(
                message_catalog,
                sanitize_scanner_line(job_result),
            )
        }
        UpdateFailureReason::FreshclamEndedAbnormally(abnormal_exit) => {
            describe_abnormal_freshclam_exit(message_catalog, *abnormal_exit)
        }
        UpdateFailureReason::RateLimitedDatabases | UpdateFailureReason::RateLimitedByMirror => {
            translated_messages::signatures::update_rate_limited_mirror(message_catalog)
        }
        UpdateFailureReason::BlockedByCdn => {
            translated_messages::signatures::update_blocked_cdn(message_catalog)
        }
        UpdateFailureReason::MirrorUnavailable => {
            translated_messages::signatures::mirror_unavailable(message_catalog)
        }
        UpdateFailureReason::CertificateRejected => {
            translated_messages::signatures::certificate_rejected(message_catalog)
        }
        UpdateFailureReason::ConnectionTimedOut => {
            translated_messages::signatures::connection_timed_out(message_catalog)
        }
        UpdateFailureReason::DatabaseLocked => {
            translated_messages::signatures::database_locked(message_catalog)
        }
        UpdateFailureReason::PermissionDenied => {
            translated_messages::signatures::permission_denied(message_catalog)
        }
        UpdateFailureReason::ConnectionFailed => {
            translated_messages::signatures::connection_failed(message_catalog)
        }
        UpdateFailureReason::NameResolutionFailed => {
            translated_messages::signatures::name_resolution_failed(message_catalog)
        }
        UpdateFailureReason::ReportedByUpdater { diagnostic_text } => {
            sanitize_scanner_text(diagnostic_text)
        }
        UpdateFailureReason::UpdaterResultMissing => {
            translated_messages::signatures::update_failed_unknown_error(message_catalog)
        }
    }
}

fn describe_report_summary(
    message_catalog: &MessageCatalog,
    report_summary: &FreshclamReportSummary,
) -> Vec<String> {
    let updated_line = (!report_summary.updated_databases.is_empty()).then(|| {
        translated_messages::signatures::updated_databases(
            message_catalog,
            report_summary.updated_databases.join(", "),
        )
    });
    let current_line = (!report_summary.current_databases.is_empty()).then(|| {
        translated_messages::signatures::already_current_databases(
            message_catalog,
            report_summary.current_databases.join(", "),
        )
    });
    let rate_limited_lines =
        report_summary
            .rate_limited_databases
            .iter()
            .map(
                |rate_limited_database| match &rate_limited_database.rate_limit_cooldown {
                    RateLimitCooldown::CooldownUnknown => {
                        translated_messages::signatures::cooldown_unknown(
                            message_catalog,
                            &rate_limited_database.database_name,
                        )
                    }
                    RateLimitCooldown::CooldownUntil { cooldown_text } => {
                        translated_messages::signatures::cooldown_until(
                            message_catalog,
                            &rate_limited_database.database_name,
                            sanitize_scanner_line(cooldown_text),
                        )
                    }
                },
            );
    updated_line
        .into_iter()
        .chain(current_line)
        .chain(rate_limited_lines)
        .map(|summary_line| sanitize_scanner_line(&summary_line))
        .collect()
}

fn describe_update_outcome(
    message_catalog: &MessageCatalog,
    update_outcome: &DatabaseUpdateOutcome,
) -> String {
    let (headline, report_summary) = match update_outcome {
        DatabaseUpdateOutcome::DefinitionsUpdated(report_summary) => (
            translated_messages::signatures::definitions_updated(message_catalog),
            report_summary,
        ),
        DatabaseUpdateOutcome::AlreadyCurrent(report_summary) => (
            translated_messages::signatures::already_current(message_catalog),
            report_summary,
        ),
        DatabaseUpdateOutcome::ServiceRanWithoutVerdict(report_summary) => (
            translated_messages::signatures::service_ran_without_verdict(message_catalog),
            report_summary,
        ),
        DatabaseUpdateOutcome::UpdateFailed {
            failure_reason,
            report_summary,
        } => (
            describe_failure_reason(message_catalog, failure_reason),
            report_summary,
        ),
        DatabaseUpdateOutcome::UpdateRequestedFromDaemon => {
            return translated_messages::signatures::update_requested_daemon(message_catalog);
        }
        DatabaseUpdateOutcome::UpdateCancelled => {
            return translated_messages::signatures::update_cancelled(message_catalog);
        }
        DatabaseUpdateOutcome::UpdaterNeverStarted(start_failure) => {
            return describe_start_failure(message_catalog, start_failure);
        }
    };
    [headline]
        .into_iter()
        .chain(describe_report_summary(message_catalog, report_summary))
        .collect::<Vec<_>>()
        .join("\n")
}

const fn update_status_of(update_outcome: &DatabaseUpdateOutcome) -> UpdateActivityStatus {
    match update_outcome {
        DatabaseUpdateOutcome::DefinitionsUpdated(_) => UpdateActivityStatus::UpdateApplied,
        DatabaseUpdateOutcome::AlreadyCurrent(_) => UpdateActivityStatus::UpdateAlreadyCurrent,
        DatabaseUpdateOutcome::ServiceRanWithoutVerdict(_) => {
            UpdateActivityStatus::UpdateResultUnknown
        }
        DatabaseUpdateOutcome::UpdateRequestedFromDaemon => UpdateActivityStatus::UpdateRequested,
        DatabaseUpdateOutcome::UpdateCancelled => UpdateActivityStatus::UpdateCancelled,
        DatabaseUpdateOutcome::UpdateFailed { .. }
        | DatabaseUpdateOutcome::UpdaterNeverStarted(_) => UpdateActivityStatus::UpdateFailed,
    }
}

const fn definitions_changed(update_outcome: &DatabaseUpdateOutcome) -> bool {
    match update_outcome {
        DatabaseUpdateOutcome::DefinitionsUpdated(_) => true,
        DatabaseUpdateOutcome::AlreadyCurrent(_)
        | DatabaseUpdateOutcome::ServiceRanWithoutVerdict(_)
        | DatabaseUpdateOutcome::UpdateRequestedFromDaemon
        | DatabaseUpdateOutcome::UpdateCancelled
        | DatabaseUpdateOutcome::UpdateFailed { .. }
        | DatabaseUpdateOutcome::UpdaterNeverStarted(_) => false,
    }
}

fn describe_start_failure(
    message_catalog: &MessageCatalog,
    start_failure: &ScanStartFailure,
) -> String {
    match start_failure {
        ScanStartFailure::ScannerProgramMissing => {
            translated_messages::signatures::scanner_program_missing(message_catalog)
        }
        ScanStartFailure::ScannerProgramReplaceable { .. }
        | ScanStartFailure::ScannerSpawnFailed(_)
        | ScanStartFailure::ScannerOutputUncaptured
        | ScanStartFailure::ScannerWorkDirectoryUnavailable(_)
        | ScanStartFailure::DetectionSettingsUnreadable(_)
        | ScanStartFailure::ProgramLookupFailed(_)
        | ScanStartFailure::ScanInputUnavailable { .. } => {
            describe_scan_start_failure(message_catalog, start_failure)
        }
    }
}

pub async fn load_host_status(
    system_bus: &SystemBusAccess,
    sandbox_kind: SandboxKind,
) -> DatabaseHostStatus {
    let freshclam_location = tokio::task::spawn_blocking(move || {
        locate_trusted_program(sandbox_kind, ClamavProgram::SignatureUpdater)
    })
    .await
    .unwrap_or(ProgramLocation::ProgramNotFound);
    let freshclam_probe = probe_located_program_version(sandbox_kind, &freshclam_location).await;
    let service_state = detect_freshclam_service(system_bus).await;
    let directory_status = tokio::task::spawn_blocking(move || {
        inspect_database_directory(
            &configured_database_directory(sandbox_kind),
            SystemTime::now(),
        )
    })
    .await
    .unwrap_or_else(
        |interrupted_inspection| DatabaseDirectoryStatus::DirectoryUnreadable {
            database_directory: configured_database_directory(sandbox_kind),
            read_failure: interrupted_inspection.into(),
        },
    );
    DatabaseHostStatus {
        freshclam_probe,
        service_state,
        directory_status,
    }
}

pub struct SignatureUpdateSession {
    host_status: HostStatusView,
    update_state: DatabaseUpdateState,
    last_manual_update: LastManualUpdate,
}

impl SignatureUpdateSession {
    pub const fn new() -> Self {
        Self {
            host_status: HostStatusView::StatusLoading,
            update_state: DatabaseUpdateState::NoActiveUpdate,
            last_manual_update: LastManualUpdate::NotRunThisSession,
        }
    }

    const fn loaded_status(&self) -> Option<&DatabaseHostStatus> {
        match &self.host_status {
            HostStatusView::StatusLoaded(host_status) => Some(host_status),
            HostStatusView::StatusLoading => None,
        }
    }

    pub const fn host_status_known(&self) -> bool {
        self.loaded_status().is_some()
    }

    pub fn host_status_loaded(&mut self, host_status: DatabaseHostStatus) {
        self.host_status = HostStatusView::StatusLoaded(host_status);
    }

    const fn update_is_active(&self) -> bool {
        match self.update_state {
            DatabaseUpdateState::NoActiveUpdate => false,
            DatabaseUpdateState::ActiveUpdate(_) => true,
        }
    }

    fn can_request_update(&self) -> bool {
        !self.update_is_active()
            && self.loaded_status().is_some_and(|host_status| {
                !matches!(
                    host_status.service_state,
                    FreshclamServiceState::ServiceNotFound
                        | FreshclamServiceState::SystemdUnreachable
                ) || matches!(
                    host_status.freshclam_probe,
                    ProgramProbe::VersionReported { .. }
                )
            })
    }

    pub fn requested_update_plan(&self) -> Option<DatabaseUpdatePlan> {
        if !self.can_request_update() {
            return None;
        }
        self.loaded_status()
            .map(|host_status| plan_database_update(host_status.service_state))
    }

    pub fn update_started(
        &mut self,
        update_plan: DatabaseUpdatePlan,
        update_stream: RunningStream,
    ) {
        self.update_state =
            DatabaseUpdateState::ActiveUpdate(Box::new(ActiveUpdateRun::UpdateRunning {
                update_plan,
                update_timing: UpdateTiming::starting_now(),
                run_output: UpdateRunOutput::nothing_printed(),
                _update_stream: update_stream,
            }));
    }

    fn advance_active_run(
        &mut self,
        run_transition: impl FnOnce(ActiveUpdateRun) -> ActiveUpdateRun,
    ) {
        self.update_state =
            match std::mem::replace(&mut self.update_state, DatabaseUpdateState::NoActiveUpdate) {
                DatabaseUpdateState::NoActiveUpdate => DatabaseUpdateState::NoActiveUpdate,
                DatabaseUpdateState::ActiveUpdate(active_run) => {
                    DatabaseUpdateState::ActiveUpdate(Box::new(run_transition(*active_run)))
                }
            };
    }

    pub fn cancel_requested(&mut self) {
        self.advance_active_run(ActiveUpdateRun::cancel_requested);
    }

    pub fn absorb_update_batch(
        &mut self,
        message_catalog: &MessageCatalog,
        update_batch: Vec<DatabaseUpdateEvent>,
    ) {
        self.advance_active_run(|active_run| {
            update_batch
                .into_iter()
                .fold(active_run, |active_run, update_event| {
                    active_run.absorb_update_event(message_catalog, update_event)
                })
        });
    }

    pub fn update_stream_ended(&mut self, message_catalog: &MessageCatalog) -> UpdateStreamEnding {
        match std::mem::replace(&mut self.update_state, DatabaseUpdateState::NoActiveUpdate) {
            DatabaseUpdateState::NoActiveUpdate => UpdateStreamEnding::NoUpdateWasRunning,
            DatabaseUpdateState::ActiveUpdate(active_run) => {
                let finished_report = active_run.into_finished_report(message_catalog);
                self.last_manual_update = LastManualUpdate::FinishedAt {
                    summary_line: finished_report
                        .summary_text
                        .lines()
                        .next()
                        .unwrap_or_default()
                        .to_owned(),
                    finished_at: Timestamp::now(),
                };
                UpdateStreamEnding::UpdateEnded(finished_report)
            }
        }
    }

    fn update_progress(&self, message_catalog: &MessageCatalog) -> SignatureUpdateProgress {
        match &self.update_state {
            DatabaseUpdateState::ActiveUpdate(active_run) => {
                active_run.update_progress(message_catalog)
            }
            DatabaseUpdateState::NoActiveUpdate if self.can_request_update() => {
                SignatureUpdateProgress::ReadyToUpdate
            }
            DatabaseUpdateState::NoActiveUpdate => SignatureUpdateProgress::UpdateUnavailable,
        }
    }

    pub fn update_view(&self, message_catalog: &MessageCatalog) -> SignatureUpdateView {
        let Some(host_status) = self.loaded_status() else {
            return SignatureUpdateView::HostStatusLoading;
        };
        SignatureUpdateView::HostStatusKnown(KnownUpdateHost {
            update_progress: self.update_progress(message_catalog),
            last_manual_update: self.last_manual_update.clone(),
            update_service_text: service_description(message_catalog, host_status.service_state),
            database_folder_text: format!(
                "{}\n{}",
                directory_path_text(&host_status.directory_status),
                directory_description(message_catalog, &host_status.directory_status)
            ),
        })
    }
}

#[cfg(test)]
mod signature_update_session_tests {
    use super::*;
    use crate::{
        host_services::{
            clamav_programs::database_directory_status::DatabaseFolderReadFailure,
            systemd_bus::host_systemd_units::UnitStartOutcome,
            systemd_journal::unit_journal_reader::UnitJournalReadFailure,
        },
        signatures::database_updates_page::signature_update_run::updater_exit_status::FreshclamExit,
    };

    const FRESHCLAM_UNIT: &str = "clamav-freshclam";

    fn session_running(update_plan: DatabaseUpdatePlan) -> SignatureUpdateSession {
        let mut update_session = SignatureUpdateSession::new();
        update_session.update_started(update_plan, RunningStream::already_stopped());
        update_session
    }

    fn ended_report(
        message_catalog: &MessageCatalog,
        update_session: &mut SignatureUpdateSession,
    ) -> FinishedUpdateReport {
        match update_session.update_stream_ended(message_catalog) {
            UpdateStreamEnding::UpdateEnded(finished_report) => finished_report,
            UpdateStreamEnding::NoUpdateWasRunning => panic!("expected an update to end"),
        }
    }

    #[test]
    fn a_stream_end_without_an_active_update_produces_no_report() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let mut update_session = SignatureUpdateSession::new();
        assert!(matches!(
            update_session.update_stream_ended(message_catalog),
            UpdateStreamEnding::NoUpdateWasRunning
        ));
        assert_eq!(
            update_session.last_manual_update,
            LastManualUpdate::NotRunThisSession
        );
    }

    #[test]
    fn a_stream_end_is_reported_once_per_run() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let mut update_session = session_running(DatabaseUpdatePlan::RunFreshclamDirectly);
        let _first_report = ended_report(message_catalog, &mut update_session);
        assert!(matches!(
            update_session.update_stream_ended(message_catalog),
            UpdateStreamEnding::NoUpdateWasRunning
        ));
    }

    #[test]
    fn a_signalled_daemon_does_not_count_as_updated_definitions() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let mut update_session = session_running(DatabaseUpdatePlan::SignalRunningDaemon {
            service_unit: FRESHCLAM_UNIT,
        });
        update_session.absorb_update_batch(
            message_catalog,
            vec![DatabaseUpdateEvent::UpdaterFinished(
                UpdaterFinish::RunningDaemonSignalled(UnitStartOutcome::UnitFinished),
            )],
        );
        let finished_report = ended_report(message_catalog, &mut update_session);
        assert!(!finished_report.definitions_changed);
        assert_eq!(
            finished_report.activity_entry.activity_record,
            ActivityRecord::UpdateActivity {
                update_status: UpdateActivityStatus::UpdateRequested
            }
        );
    }

    #[test]
    fn an_unreadable_service_journal_is_written_into_the_update_output() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let mut update_session = session_running(DatabaseUpdatePlan::StartServiceUnit {
            service_unit: FRESHCLAM_UNIT,
        });
        update_session.absorb_update_batch(
            message_catalog,
            vec![
                DatabaseUpdateEvent::UpdateOutputUnreadable(
                    UnitJournalReadFailure::ReadingTaskInterrupted,
                ),
                DatabaseUpdateEvent::UpdaterFinished(UpdaterFinish::ServiceUnitRan(
                    UnitStartOutcome::UnitFinished,
                )),
            ],
        );
        let finished_report = ended_report(message_catalog, &mut update_session);
        assert!(
            finished_report
                .activity_entry
                .activity_output
                .contains("Reading the system journal stopped before it finished")
        );
        assert_eq!(
            finished_report.activity_entry.activity_record,
            ActivityRecord::UpdateActivity {
                update_status: UpdateActivityStatus::UpdateResultUnknown
            }
        );
    }

    #[test]
    fn an_unreadable_database_folder_is_not_reported_as_missing() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let unreadable_status = DatabaseDirectoryStatus::DirectoryUnreadable {
            database_directory: std::path::PathBuf::from("/var/lib/clamav"),
            read_failure: DatabaseFolderReadFailure::ReadingTaskInterrupted,
        };
        assert_eq!(
            directory_description(message_catalog, &unreadable_status),
            "Unknown. Reading the signature folder stopped before it finished"
        );
        assert_eq!(directory_path_text(&unreadable_status), "/var/lib/clamav");
    }

    #[test]
    fn a_unit_run_without_a_version_line_has_an_unknown_result() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let mut update_session = session_running(DatabaseUpdatePlan::StartServiceUnit {
            service_unit: FRESHCLAM_UNIT,
        });
        update_session.absorb_update_batch(
            message_catalog,
            vec![
                DatabaseUpdateEvent::UpdateOutputLine("Starting clamav-freshclam".to_owned()),
                DatabaseUpdateEvent::UpdaterFinished(UpdaterFinish::ServiceUnitRan(
                    UnitStartOutcome::UnitFinished,
                )),
            ],
        );
        let finished_report = ended_report(message_catalog, &mut update_session);
        assert!(!finished_report.definitions_changed);
        assert_eq!(
            finished_report.activity_entry.activity_record,
            ActivityRecord::UpdateActivity {
                update_status: UpdateActivityStatus::UpdateResultUnknown
            }
        );
    }

    #[test]
    fn a_refused_authorization_fails_without_writing_to_the_transcript() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let mut update_session = session_running(DatabaseUpdatePlan::StartServiceUnit {
            service_unit: FRESHCLAM_UNIT,
        });
        update_session.absorb_update_batch(
            message_catalog,
            vec![DatabaseUpdateEvent::UpdaterFinished(
                UpdaterFinish::ServiceUnitRan(UnitStartOutcome::AuthorizationRefused),
            )],
        );
        let finished_report = ended_report(message_catalog, &mut update_session);
        assert_eq!(
            finished_report.activity_entry.activity_record,
            ActivityRecord::UpdateActivity {
                update_status: UpdateActivityStatus::UpdateFailed
            }
        );
        assert_eq!(finished_report.activity_entry.activity_output, "");
        assert_eq!(
            finished_report.summary_text,
            "Authorization failed. Please try again and enter your password."
        );
    }

    #[test]
    fn an_exit_from_a_previous_run_does_not_survive_into_the_next_run() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let mut update_session = session_running(DatabaseUpdatePlan::RunFreshclamDirectly);
        update_session.absorb_update_batch(
            message_catalog,
            vec![
                DatabaseUpdateEvent::UpdateOutputLine(
                    "daily.cld updated (version: 1, sigs: 2)".to_owned(),
                ),
                DatabaseUpdateEvent::UpdaterFinished(UpdaterFinish::FreshclamExited(
                    FreshclamExit::UpdateSucceeded,
                )),
            ],
        );
        assert!(ended_report(message_catalog, &mut update_session).definitions_changed);
        update_session.update_started(
            DatabaseUpdatePlan::RunFreshclamDirectly,
            RunningStream::already_stopped(),
        );
        let next_report = ended_report(message_catalog, &mut update_session);
        assert!(!next_report.definitions_changed);
        assert_eq!(
            next_report.activity_entry.activity_record,
            ActivityRecord::UpdateActivity {
                update_status: UpdateActivityStatus::UpdateFailed
            }
        );
        assert_eq!(next_report.activity_entry.activity_output, "");
    }

    #[test]
    fn a_cancelled_run_ignores_a_late_exit() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let mut update_session = session_running(DatabaseUpdatePlan::RunFreshclamDirectly);
        update_session.cancel_requested();
        update_session.absorb_update_batch(
            message_catalog,
            vec![DatabaseUpdateEvent::UpdaterFinished(
                UpdaterFinish::FreshclamExited(FreshclamExit::UpdateSucceeded),
            )],
        );
        let finished_report = ended_report(message_catalog, &mut update_session);
        assert!(!finished_report.definitions_changed);
        assert_eq!(
            finished_report.activity_entry.activity_record,
            ActivityRecord::UpdateActivity {
                update_status: UpdateActivityStatus::UpdateCancelled
            }
        );
    }

    #[test]
    fn only_a_direct_freshclam_run_can_be_cancelled() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let mut update_session = session_running(DatabaseUpdatePlan::StartServiceUnit {
            service_unit: FRESHCLAM_UNIT,
        });
        update_session.cancel_requested();
        assert_eq!(
            update_session.update_progress(message_catalog),
            SignatureUpdateProgress::UncancellableUpdateRunning {
                latest_output_line: "Starting the update…".to_owned()
            }
        );
    }
}
