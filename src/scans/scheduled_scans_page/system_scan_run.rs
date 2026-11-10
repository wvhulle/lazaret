use jiff::Timestamp;

use crate::{
    host_services::{
        clamav_scanner::scanner_output_parser::sanitize_scanner_line,
        protection_evidence::host_protection_evidence::{
            DaemonEvidence, ProtectionEvidence, ScanServiceEvidence, ScanUnitResult,
            ScheduledScanEvidence,
        },
        systemd_bus::host_systemd_units::{
            ServiceFailureKind, ServiceRunState, SystemdBusFailure, UnitStartOutcome, UnitTimestamp,
        },
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SystemScanAvailability {
    NoSystemScanService,
    SystemScanService {
        service_unit: &'static str,
        scanned_folders: Vec<String>,
        daemon_answering: bool,
        scan_run_state: ServiceRunState,
    },
}

pub fn system_scan_availability(
    protection_evidence: &ProtectionEvidence,
) -> SystemScanAvailability {
    match &protection_evidence.scheduled_scan_evidence {
        ScheduledScanEvidence::SystemTimer {
            service_unit,
            scan_service:
                ScanServiceEvidence::ServiceConfigured {
                    scan_targets,
                    scan_run_state,
                    ..
                },
            ..
        } => SystemScanAvailability::SystemScanService {
            service_unit,
            scanned_folders: scan_targets.clone(),
            daemon_answering: matches!(
                protection_evidence.daemon_evidence,
                DaemonEvidence::ClamdAnswering { .. }
            ),
            scan_run_state: *scan_run_state,
        },
        ScheduledScanEvidence::SystemTimer { .. }
        | ScheduledScanEvidence::LazaretSchedule
        | ScheduledScanEvidence::NoSchedule => SystemScanAvailability::NoSystemScanService,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SystemScanRun {
    RunIdle,
    RunRequested,
    UnitRunning { started_at: UnitTimestamp },
    StopRequested { started_at: UnitTimestamp },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SystemScanStart {
    StartServiceUnit { service_unit: &'static str },
    NothingToStart,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SystemScanStop {
    StopServiceUnit { service_unit: &'static str },
    NothingToStop,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemScanControl {
    scan_availability: SystemScanAvailability,
    scan_run: SystemScanRun,
}

impl SystemScanControl {
    pub const fn new() -> Self {
        Self {
            scan_availability: SystemScanAvailability::NoSystemScanService,
            scan_run: SystemScanRun::RunIdle,
        }
    }

    const fn unit_run_state(&self) -> ServiceRunState {
        match self.scan_availability {
            SystemScanAvailability::SystemScanService { scan_run_state, .. } => scan_run_state,
            SystemScanAvailability::NoSystemScanService => ServiceRunState::ServiceIdle,
        }
    }

    pub fn evidence_gathered(&mut self, protection_evidence: &ProtectionEvidence) {
        self.scan_availability = system_scan_availability(protection_evidence);
        self.scan_run = match (self.scan_run, self.unit_run_state()) {
            (
                SystemScanRun::StopRequested { .. },
                ServiceRunState::ServiceRunning { started_at },
            ) => SystemScanRun::StopRequested { started_at },
            (_, ServiceRunState::ServiceRunning { started_at }) => {
                SystemScanRun::UnitRunning { started_at }
            }
            (SystemScanRun::RunRequested, ServiceRunState::ServiceIdle) => {
                SystemScanRun::RunRequested
            }
            (_, ServiceRunState::ServiceIdle) => SystemScanRun::RunIdle,
        };
    }

    pub const fn run_requested(&mut self) -> SystemScanStart {
        match (&self.scan_run, &self.scan_availability) {
            (
                SystemScanRun::RunIdle,
                SystemScanAvailability::SystemScanService {
                    service_unit,
                    daemon_answering: true,
                    ..
                },
            ) => {
                self.scan_run = SystemScanRun::RunRequested;
                SystemScanStart::StartServiceUnit { service_unit }
            }
            _ => SystemScanStart::NothingToStart,
        }
    }

    pub const fn stop_requested(&mut self) -> SystemScanStop {
        match (self.scan_run, &self.scan_availability) {
            (
                SystemScanRun::UnitRunning { started_at },
                SystemScanAvailability::SystemScanService { service_unit, .. },
            ) => {
                self.scan_run = SystemScanRun::StopRequested { started_at };
                SystemScanStop::StopServiceUnit { service_unit }
            }
            _ => SystemScanStop::NothingToStop,
        }
    }

    pub const fn stop_refused(&mut self) {
        if let SystemScanRun::StopRequested { started_at } = self.scan_run {
            self.scan_run = SystemScanRun::UnitRunning { started_at };
        }
    }

    pub const fn run_finished(&mut self) {
        self.scan_run = SystemScanRun::RunIdle;
    }

    pub const fn shows_system_scan(&self) -> bool {
        matches!(
            self.scan_availability,
            SystemScanAvailability::SystemScanService { .. }
        )
    }

    pub const fn shows_run_status(&self) -> bool {
        !matches!(self.scan_run, SystemScanRun::RunIdle)
    }

    pub const fn accepts_run(&self) -> bool {
        matches!(
            (&self.scan_run, &self.scan_availability),
            (
                SystemScanRun::RunIdle,
                SystemScanAvailability::SystemScanService {
                    daemon_answering: true,
                    ..
                }
            )
        )
    }

    pub const fn accepts_stop(&self) -> bool {
        matches!(self.scan_run, SystemScanRun::UnitRunning { .. })
    }

    pub fn run_status_title(&self, message_catalog: &MessageCatalog) -> String {
        match self.scan_run {
            SystemScanRun::RunIdle => String::new(),
            SystemScanRun::RunRequested => {
                translated_messages::scans::starting_scan_tooltip(message_catalog)
            }
            SystemScanRun::UnitRunning { .. } => {
                translated_messages::scans::scanning_title(message_catalog)
            }
            SystemScanRun::StopRequested { .. } => {
                translated_messages::scans::stopping_scan(message_catalog)
            }
        }
    }

    pub const fn run_started_at(&self) -> Option<Timestamp> {
        match self.scan_run {
            SystemScanRun::UnitRunning {
                started_at: UnitTimestamp::RecordedAt { recorded_at },
            }
            | SystemScanRun::StopRequested {
                started_at: UnitTimestamp::RecordedAt { recorded_at },
            } => Some(recorded_at),
            SystemScanRun::UnitRunning { .. }
            | SystemScanRun::StopRequested { .. }
            | SystemScanRun::RunRequested
            | SystemScanRun::RunIdle => None,
        }
    }

    pub fn scanned_folders_text(&self, message_catalog: &MessageCatalog) -> String {
        match &self.scan_availability {
            SystemScanAvailability::SystemScanService {
                scanned_folders, ..
            } if !scanned_folders.is_empty() => scanned_folders.join(", "),
            SystemScanAvailability::SystemScanService { .. } => {
                translated_messages::scans::unit_names_no_folders(message_catalog)
            }
            SystemScanAvailability::NoSystemScanService => String::new(),
        }
    }

    pub fn group_description(&self, message_catalog: &MessageCatalog) -> String {
        match &self.scan_availability {
            SystemScanAvailability::SystemScanService {
                service_unit,
                daemon_answering: true,
                ..
            } => {
                translated_messages::scans::system_scan_service_unit(message_catalog, service_unit)
            }
            SystemScanAvailability::SystemScanService {
                daemon_answering: false,
                ..
            } => translated_messages::scans::needs_daemon_see_scan(message_catalog),
            SystemScanAvailability::NoSystemScanService => String::new(),
        }
    }
}

const fn scan_was_stopped(
    start_outcome: &UnitStartOutcome,
    scan_service: &ScanServiceEvidence,
) -> bool {
    matches!(start_outcome, UnitStartOutcome::JobEnded { .. })
        && matches!(
            scan_service,
            ScanServiceEvidence::ServiceConfigured {
                last_result: ScanUnitResult::ServiceFailed {
                    failure_kind: ServiceFailureKind::KilledBySignal
                },
                ..
            }
        )
}

pub fn scan_found_threats(
    start_outcome: &UnitStartOutcome,
    scan_service: &ScanServiceEvidence,
) -> bool {
    matches!(start_outcome, UnitStartOutcome::JobEnded { .. })
        && matches!(
            scan_service,
            ScanServiceEvidence::ServiceConfigured {
                last_result: ScanUnitResult::ThreatsFound,
                ..
            }
        )
}

fn systemd_failure_text(
    message_catalog: &MessageCatalog,
    bus_failure: &SystemdBusFailure,
) -> String {
    match bus_failure {
        SystemdBusFailure::BusCallFailed(bus_error) => translated_messages::scans::bus_call_failed(
            message_catalog,
            sanitize_scanner_line(&bus_error.to_string()),
        ),
        SystemdBusFailure::UnrecognizedLoadState => {
            translated_messages::scans::unrecognized_load_state(message_catalog)
        }
    }
}

pub fn unit_stop_toast_text(
    message_catalog: &MessageCatalog,
    stop_outcome: &UnitStartOutcome,
) -> Option<String> {
    match stop_outcome {
        UnitStartOutcome::UnitFinished => None,
        UnitStartOutcome::AuthorizationRefused => Some(
            translated_messages::scans::stop_authorization_refused(message_catalog),
        ),
        UnitStartOutcome::BusFailure(bus_failure) => {
            Some(translated_messages::scans::not_stopped_error(
                message_catalog,
                systemd_failure_text(message_catalog, bus_failure),
            ))
        }
        UnitStartOutcome::JobEnded { job_result } => {
            Some(translated_messages::scans::not_stopped_error(
                message_catalog,
                sanitize_scanner_line(job_result),
            ))
        }
    }
}

pub fn unit_run_toast_text(
    message_catalog: &MessageCatalog,
    start_outcome: &UnitStartOutcome,
    scan_service: &ScanServiceEvidence,
) -> String {
    if scan_found_threats(start_outcome, scan_service) {
        return translated_messages::scans::found_malware(message_catalog);
    }
    if scan_was_stopped(start_outcome, scan_service) {
        return translated_messages::scans::scan_stopped(message_catalog);
    }
    match start_outcome {
        UnitStartOutcome::UnitFinished => {
            translated_messages::scans::unit_finished(message_catalog)
        }
        UnitStartOutcome::JobEnded { job_result } => {
            translated_messages::scans::not_finish_cleanly_result(
                message_catalog,
                sanitize_scanner_line(job_result),
            )
        }
        UnitStartOutcome::AuthorizationRefused => {
            translated_messages::scans::authorization_refused(message_catalog)
        }
        UnitStartOutcome::BusFailure(bus_failure) => translated_messages::scans::not_started_error(
            message_catalog,
            systemd_failure_text(message_catalog, bus_failure),
        ),
    }
}

#[cfg(test)]
mod system_scan_run_tests {
    use super::*;
    use crate::host_services::{
        protection_evidence::{
            host_protection_evidence::scan_service_evidence,
            protection_evidence_fixtures::healthy_evidence,
        },
        systemd_bus::host_systemd_units::{ServiceLastResult, ServiceUnitSnapshot, UnitActivity},
    };

    fn control_with(protection_evidence: &ProtectionEvidence) -> SystemScanControl {
        let mut system_scan_control = SystemScanControl::new();
        system_scan_control.evidence_gathered(protection_evidence);
        system_scan_control
    }

    #[test]
    fn a_system_scan_service_can_be_run_once_until_it_finishes() {
        let mut system_scan_control = control_with(&healthy_evidence());
        assert!(system_scan_control.shows_system_scan());
        assert!(system_scan_control.accepts_run());
        assert!(matches!(
            system_scan_control.run_requested(),
            SystemScanStart::StartServiceUnit { .. }
        ));
        assert!(system_scan_control.shows_run_status());
        assert!(!system_scan_control.accepts_stop());
        assert!(!system_scan_control.accepts_run());
        assert_eq!(
            system_scan_control.run_requested(),
            SystemScanStart::NothingToStart
        );
        system_scan_control.run_finished();
        assert!(system_scan_control.accepts_run());
    }

    fn evidence_with_scan_run_state(scan_run_state: ServiceRunState) -> ProtectionEvidence {
        let mut running_evidence = healthy_evidence();
        if let ScheduledScanEvidence::SystemTimer {
            scan_service:
                ScanServiceEvidence::ServiceConfigured {
                    scan_run_state: shown_run_state,
                    ..
                },
            ..
        } = &mut running_evidence.scheduled_scan_evidence
        {
            *shown_run_state = scan_run_state;
        }
        running_evidence
    }

    const SCAN_STARTED_AT: Timestamp = Timestamp::constant(1_700_000_000, 0);

    fn running_scan_evidence() -> ProtectionEvidence {
        evidence_with_scan_run_state(ServiceRunState::ServiceRunning {
            started_at: UnitTimestamp::RecordedAt {
                recorded_at: SCAN_STARTED_AT,
            },
        })
    }

    #[test]
    fn a_scan_the_timer_started_shows_as_running_and_can_be_stopped() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let mut system_scan_control = control_with(&running_scan_evidence());
        assert!(system_scan_control.shows_run_status());
        assert!(!system_scan_control.accepts_run());
        assert!(system_scan_control.accepts_stop());
        assert_eq!(
            system_scan_control.run_status_title(message_catalog),
            "Scanning"
        );
        assert_eq!(system_scan_control.run_started_at(), Some(SCAN_STARTED_AT));
        assert_eq!(
            system_scan_control.stop_requested(),
            SystemScanStop::StopServiceUnit {
                service_unit: "clamdscan.service"
            }
        );
        assert!(!system_scan_control.accepts_stop());
        system_scan_control.evidence_gathered(&running_scan_evidence());
        assert_eq!(
            system_scan_control.run_status_title(message_catalog),
            "Stopping the scan…"
        );
        system_scan_control.evidence_gathered(&healthy_evidence());
        assert!(!system_scan_control.shows_run_status());
        assert!(system_scan_control.accepts_run());
    }

    #[test]
    fn a_refused_stop_leaves_the_scan_stoppable() {
        let mut system_scan_control = control_with(&running_scan_evidence());
        let _sent_stop = system_scan_control.stop_requested();
        system_scan_control.stop_refused();
        assert!(system_scan_control.accepts_stop());
        assert_eq!(system_scan_control.run_started_at(), Some(SCAN_STARTED_AT));
    }

    #[test]
    fn a_requested_run_waits_for_authorization_while_the_unit_is_still_idle() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let mut system_scan_control = control_with(&healthy_evidence());
        let _started_unit = system_scan_control.run_requested();
        system_scan_control.evidence_gathered(&healthy_evidence());
        assert_eq!(
            system_scan_control.run_status_title(message_catalog),
            "Starting the scan…"
        );
        system_scan_control.evidence_gathered(&running_scan_evidence());
        assert!(system_scan_control.accepts_stop());
        system_scan_control.run_finished();
        assert!(!system_scan_control.shows_run_status());
    }

    #[test]
    fn lazarets_own_schedule_and_no_schedule_offer_no_system_scan() {
        [
            ScheduledScanEvidence::LazaretSchedule,
            ScheduledScanEvidence::NoSchedule,
        ]
        .into_iter()
        .for_each(|scheduled_scan_evidence| {
            let mut system_scan_control = control_with(&ProtectionEvidence {
                scheduled_scan_evidence,
                ..healthy_evidence()
            });
            assert!(!system_scan_control.shows_system_scan());
            assert_eq!(
                system_scan_control.run_requested(),
                SystemScanStart::NothingToStart
            );
        });
    }

    #[test]
    fn a_stopped_daemon_shows_the_service_but_does_not_run_it() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let mut system_scan_control = control_with(&ProtectionEvidence {
            daemon_evidence: DaemonEvidence::UnitStopped {
                service_unit: "clamav-daemon.service",
            },
            ..healthy_evidence()
        });
        assert!(system_scan_control.shows_system_scan());
        assert!(!system_scan_control.accepts_run());
        assert_eq!(
            system_scan_control.run_requested(),
            SystemScanStart::NothingToStart
        );
        assert_eq!(
            system_scan_control.group_description(message_catalog),
            "Needs the daemon, see Scan Engine"
        );
    }

    #[test]
    fn the_scanned_folders_come_from_the_scan_unit() {
        let message_catalog = &crate::translated_messages::english_catalog();
        assert_eq!(
            control_with(&healthy_evidence()).scanned_folders_text(message_catalog),
            "/tmp, /var/tmp"
        );
    }

    fn loaded_scan_service(exit_status: i32) -> ScanServiceEvidence {
        scan_service_evidence(ServiceUnitSnapshot::LoadedService {
            run_state: ServiceRunState::ServiceIdle,
            unit_activity: UnitActivity::UnitInactive,
            last_result: ServiceLastResult::ExitedWithStatus { exit_status },
            program_arguments: Vec::new(),
        })
    }

    #[test]
    fn finding_malware_is_not_reported_as_a_failed_scan() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let failed_job = UnitStartOutcome::JobEnded {
            job_result: "failed".to_owned(),
        };
        assert_eq!(
            unit_run_toast_text(message_catalog, &failed_job, &loaded_scan_service(1)),
            "The scan found malware"
        );
        assert!(
            unit_run_toast_text(message_catalog, &failed_job, &loaded_scan_service(2))
                .contains("did not finish cleanly")
        );
    }

    #[test]
    fn a_refused_stop_is_not_reported_as_a_failed_start() {
        let message_catalog = &crate::translated_messages::english_catalog();
        assert_eq!(
            unit_stop_toast_text(message_catalog, &UnitStartOutcome::UnitFinished),
            None
        );
        assert_eq!(
            unit_stop_toast_text(message_catalog, &UnitStartOutcome::AuthorizationRefused)
                .as_deref(),
            Some("Stopping the scan needs administrator authorization")
        );
        assert!(
            unit_stop_toast_text(
                message_catalog,
                &UnitStartOutcome::BusFailure(SystemdBusFailure::UnrecognizedLoadState)
            )
            .is_some_and(|stop_text| stop_text.starts_with("The scan could not be stopped"))
        );
    }

    #[test]
    fn a_scan_killed_by_a_signal_is_reported_as_stopped() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let failed_job = UnitStartOutcome::JobEnded {
            job_result: "failed".to_owned(),
        };
        let killed_service = scan_service_evidence(ServiceUnitSnapshot::LoadedService {
            run_state: ServiceRunState::ServiceIdle,
            unit_activity: UnitActivity::UnitInactive,
            last_result: ServiceLastResult::FailedWithResult {
                failure_kind: ServiceFailureKind::KilledBySignal,
            },
            program_arguments: Vec::new(),
        });
        assert_eq!(
            unit_run_toast_text(message_catalog, &failed_job, &killed_service),
            "The scan was stopped"
        );
    }

    #[test]
    fn an_unreadable_scan_service_never_claims_malware_was_found() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let failed_job = UnitStartOutcome::JobEnded {
            job_result: "failed".to_owned(),
        };
        assert!(
            unit_run_toast_text(
                message_catalog,
                &failed_job,
                &scan_service_evidence(ServiceUnitSnapshot::SystemdUnreachable(
                    SystemdBusFailure::UnrecognizedLoadState
                ))
            )
            .contains("did not finish cleanly")
        );
        assert_eq!(
            unit_run_toast_text(
                message_catalog,
                &UnitStartOutcome::BusFailure(SystemdBusFailure::UnrecognizedLoadState),
                &ScanServiceEvidence::ServiceMissing
            ),
            "The scan could not be started: systemd reported a unit state Lazaret does not \
             recognize"
        );
    }
}
