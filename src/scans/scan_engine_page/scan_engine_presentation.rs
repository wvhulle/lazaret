use crate::{
    host_services::{
        clamav_programs::{
            component_availability::ProbeFailure,
            distribution_install_guides::{DistributionFamily, engine_install_guides},
        },
        clamav_scanner::scanner_output_parser::sanitize_scanner_line,
        desktop::locale_formatting::MomentFormatters,
        protection_evidence::host_protection_evidence::{
            DaemonEvidence, EngineInstallation, LoadedSignatures, ProtectionEvidence,
            daemon_is_answering,
        },
    },
    scans::scan_engine_page::{
        clamd_socket_ownership::ClamdSocketOwnership, scan_engine_state::ClamdSocketView,
    },
    shared_widgets::step_timeline::{
        program_trust_steps::{TrustedProgramGroup, program_trust_step},
        runtime_reduction_steps::{ReducedSettingGroup, runtime_reduction_step},
        step_evidence_builders::{CheckedCondition, evidence_row, plain_rows, remedy_commands},
        suggested_remedy_commands::SuggestedRemedy,
        timeline_step_contents::{
            ActionConsequence, CancelSlot, RunningStep, SettledStep, StepActionButton,
            StepActionSlot, StepEvidenceRow, TimelineStep,
        },
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScanEngineAction {}

impl StepActionButton for ScanEngineAction {
    fn button_label(&self, _message_catalog: &MessageCatalog) -> String {
        match *self {}
    }

    fn action_consequence(&self) -> ActionConsequence {
        match *self {}
    }
}

pub type ScanEngineStep = TimelineStep<ScanEngineAction>;

fn probe_failure_text(message_catalog: &MessageCatalog, probe_failure: &ProbeFailure) -> String {
    match probe_failure {
        ProbeFailure::ProgramPrintedFailure { program_stderr } => {
            sanitize_scanner_line(program_stderr)
        }
        ProbeFailure::ProgramStartFailed { start_failure_kind } => {
            translated_messages::scans::program_start_failed(message_catalog, start_failure_kind)
        }
        ProbeFailure::ProbeTimedOut => translated_messages::scans::probe_timed_out(message_catalog),
    }
}

pub const fn engine_condition(engine_installation: &EngineInstallation) -> CheckedCondition {
    match engine_installation {
        EngineInstallation::VersionReported { .. } => CheckedCondition::ConditionMet,
        EngineInstallation::ScannerRefused { .. } => CheckedCondition::ReviewSuggested,
        EngineInstallation::VersionUnverified(_) => CheckedCondition::EvidenceUnknown,
        EngineInstallation::EngineMissing => CheckedCondition::SetupSuggested,
    }
}

fn engine_status_text(
    message_catalog: &MessageCatalog,
    engine_installation: &EngineInstallation,
) -> String {
    match engine_installation {
        EngineInstallation::VersionReported { version_banner } => version_banner.clone(),
        EngineInstallation::ScannerRefused { refused_path } => {
            translated_messages::scans::scanner_refused(message_catalog, refused_path)
        }
        EngineInstallation::VersionUnverified(probe_failure) => {
            probe_failure_text(message_catalog, probe_failure)
        }
        EngineInstallation::EngineMissing => {
            translated_messages::scans::engine_missing(message_catalog)
        }
    }
}

fn engine_guidance_commands(
    engine_installation: &EngineInstallation,
    distribution_family: DistributionFamily,
) -> Vec<String> {
    match engine_installation {
        EngineInstallation::EngineMissing => engine_install_guides()
            .iter()
            .filter(|install_guide| install_guide.distribution_family == distribution_family)
            .map(|install_guide| install_guide.install_command.to_owned())
            .collect(),
        EngineInstallation::VersionReported { .. }
        | EngineInstallation::ScannerRefused { .. }
        | EngineInstallation::VersionUnverified(_) => Vec::new(),
    }
}

pub fn engine_step<StepAction: StepActionButton>(
    message_catalog: &MessageCatalog,
    engine_installation: &EngineInstallation,
    distribution_family: DistributionFamily,
    header_action: StepActionSlot<StepAction>,
) -> TimelineStep<StepAction> {
    TimelineStep::settled(
        translated_messages::scans::clamav_installed(message_catalog),
        translated_messages::scans::clamav_engine_recognizes_malware(message_catalog),
        engine_status_text(message_catalog, engine_installation),
        SettledStep {
            header_action,
            ..SettledStep::without_offers(engine_condition(engine_installation).node_state())
        },
    )
    .with_guidance_commands(engine_guidance_commands(
        engine_installation,
        distribution_family,
    ))
}

pub const fn daemon_condition(daemon_evidence: &DaemonEvidence) -> CheckedCondition {
    match daemon_evidence {
        DaemonEvidence::ClamdAnswering {
            loaded_signatures: LoadedSignatures::BehindDisk { .. },
        } => CheckedCondition::ReviewSuggested,
        DaemonEvidence::ClamdAnswering { .. } => CheckedCondition::ConditionMet,
        DaemonEvidence::UnitActiveNotAnswering { .. } | DaemonEvidence::UnitStopped { .. } => {
            CheckedCondition::ConditionFailed
        }
        DaemonEvidence::UnitStateUnknown => CheckedCondition::EvidenceUnknown,
        DaemonEvidence::NeverSetUp => CheckedCondition::SetupSuggested,
    }
}

fn daemon_status_text(
    message_catalog: &MessageCatalog,
    daemon_evidence: &DaemonEvidence,
) -> String {
    match daemon_evidence {
        DaemonEvidence::ClamdAnswering {
            loaded_signatures:
                LoadedSignatures::BehindDisk {
                    loaded_version,
                    disk_version,
                },
        } => translated_messages::scans::clamd_still_uses_signatures(
            message_catalog,
            loaded_version,
            disk_version,
        ),
        DaemonEvidence::ClamdAnswering { .. } => {
            translated_messages::scans::clamd_answering(message_catalog)
        }
        DaemonEvidence::UnitActiveNotAnswering { service_unit } => {
            translated_messages::scans::unit_active_not_answering(message_catalog, service_unit)
        }
        DaemonEvidence::UnitStateUnknown => {
            translated_messages::scans::unit_state_unknown_daemon(message_catalog)
        }
        DaemonEvidence::UnitStopped { service_unit } => {
            translated_messages::scans::unit_stopped_daemon(message_catalog, service_unit)
        }
        DaemonEvidence::NeverSetUp => {
            translated_messages::scans::never_set_up_daemon(message_catalog)
        }
    }
}

fn loaded_signature_rows(
    message_catalog: &MessageCatalog,
    daemon_evidence: &DaemonEvidence,
) -> Vec<StepEvidenceRow> {
    let loaded_value = match daemon_evidence {
        DaemonEvidence::ClamdAnswering {
            loaded_signatures: LoadedSignatures::MatchesDisk,
        } => translated_messages::scans::same_version_disk(message_catalog),
        DaemonEvidence::ClamdAnswering {
            loaded_signatures:
                LoadedSignatures::BehindDisk {
                    loaded_version,
                    disk_version,
                },
        } => translated_messages::scans::loaded_disk(message_catalog, loaded_version, disk_version),
        DaemonEvidence::ClamdAnswering {
            loaded_signatures: LoadedSignatures::ComparisonUnavailable,
        } => translated_messages::scans::unknown_loaded_stored_versions(message_catalog),
        DaemonEvidence::UnitActiveNotAnswering { .. }
        | DaemonEvidence::UnitStopped { .. }
        | DaemonEvidence::UnitStateUnknown
        | DaemonEvidence::NeverSetUp => return Vec::new(),
    };
    vec![evidence_row(
        translated_messages::scans::loaded_signatures(message_catalog),
        loaded_value,
    )]
}

fn daemon_guidance_commands(
    daemon_evidence: &DaemonEvidence,
    distribution_family: DistributionFamily,
) -> Vec<String> {
    match daemon_evidence {
        DaemonEvidence::UnitActiveNotAnswering { .. } => {
            remedy_commands(SuggestedRemedy::ClamavDaemonRestart, distribution_family)
        }
        DaemonEvidence::UnitStopped { .. } | DaemonEvidence::NeverSetUp => {
            remedy_commands(SuggestedRemedy::ClamavDaemonStart, distribution_family)
        }
        DaemonEvidence::ClamdAnswering { .. } | DaemonEvidence::UnitStateUnknown => Vec::new(),
    }
}

pub fn daemon_step<StepAction: StepActionButton>(
    message_catalog: &MessageCatalog,
    daemon_evidence: &DaemonEvidence,
    distribution_family: DistributionFamily,
    header_action: StepActionSlot<StepAction>,
) -> TimelineStep<StepAction> {
    TimelineStep::settled(
        translated_messages::scans::daemon_running(message_catalog),
        translated_messages::scans::clamd_keeps_signatures_loaded(message_catalog),
        daemon_status_text(message_catalog, daemon_evidence),
        SettledStep {
            evidence_rows: plain_rows(loaded_signature_rows(message_catalog, daemon_evidence)),
            header_action,
            ..SettledStep::without_offers(daemon_condition(daemon_evidence).node_state())
        },
    )
    .with_guidance_commands(daemon_guidance_commands(
        daemon_evidence,
        distribution_family,
    ))
}

fn shown_socket_path(socket_path: &std::path::Path) -> String {
    sanitize_scanner_line(&socket_path.to_string_lossy())
}

pub const fn socket_condition(
    socket_ownership: &ClamdSocketOwnership,
    daemon_answering: bool,
) -> CheckedCondition {
    match socket_ownership {
        ClamdSocketOwnership::SocketProtected { .. } => CheckedCondition::ConditionMet,
        ClamdSocketOwnership::SocketReplaceable { .. } => CheckedCondition::ConditionFailed,
        ClamdSocketOwnership::SocketMissing { .. } if !daemon_answering => {
            CheckedCondition::PrerequisiteMissing
        }
        ClamdSocketOwnership::SocketMissing { .. } | ClamdSocketOwnership::SocketNotCheckable => {
            CheckedCondition::EvidenceUnknown
        }
    }
}

fn socket_status_text(
    message_catalog: &MessageCatalog,
    socket_ownership: &ClamdSocketOwnership,
) -> String {
    match socket_ownership {
        ClamdSocketOwnership::SocketProtected { socket_path } => {
            translated_messages::scans::socket_protected(
                message_catalog,
                shown_socket_path(socket_path),
            )
        }
        ClamdSocketOwnership::SocketReplaceable {
            socket_path,
            replaceable_path,
        } => translated_messages::scans::socket_replaceable(
            message_catalog,
            shown_socket_path(replaceable_path),
            shown_socket_path(socket_path),
        ),
        ClamdSocketOwnership::SocketMissing { socket_path } => {
            translated_messages::scans::socket_missing(
                message_catalog,
                shown_socket_path(socket_path),
            )
        }
        ClamdSocketOwnership::SocketNotCheckable => {
            translated_messages::scans::socket_not_checkable(message_catalog)
        }
    }
}

fn socket_step_title(message_catalog: &MessageCatalog) -> String {
    translated_messages::scans::clamd_socket_protected(message_catalog)
}

fn socket_mechanism_details(message_catalog: &MessageCatalog) -> String {
    translated_messages::scans::socket_mechanism(message_catalog)
}

pub fn socket_step(
    message_catalog: &MessageCatalog,
    clamd_socket_view: &ClamdSocketView,
    daemon_answering: bool,
) -> ScanEngineStep {
    match clamd_socket_view {
        ClamdSocketView::SocketInspecting => TimelineStep::running(
            socket_step_title(message_catalog),
            socket_mechanism_details(message_catalog),
            translated_messages::scans::socket_inspecting(message_catalog),
            RunningStep {
                evidence_rows: Vec::new(),
                cancel_slot: CancelSlot::NotCancellable,
            },
        ),
        ClamdSocketView::SocketInspected(socket_ownership) => TimelineStep::settled(
            socket_step_title(message_catalog),
            socket_mechanism_details(message_catalog),
            socket_status_text(message_catalog, socket_ownership),
            SettledStep::without_offers(
                socket_condition(socket_ownership, daemon_answering).node_state(),
            ),
        ),
    }
}

pub fn scan_engine_steps(
    message_catalog: &MessageCatalog,
    moment_formatters: &MomentFormatters,
    protection_evidence: &ProtectionEvidence,
    clamd_socket_view: &ClamdSocketView,
    distribution_family: DistributionFamily,
) -> Vec<ScanEngineStep> {
    std::iter::once(engine_step(
        message_catalog,
        &protection_evidence.engine_installation,
        distribution_family,
        StepActionSlot::NoAction,
    ))
    .chain(program_trust_step(
        message_catalog,
        &protection_evidence.program_trust,
        TrustedProgramGroup::ScannerPrograms,
    ))
    .chain([
        daemon_step(
            message_catalog,
            &protection_evidence.daemon_evidence,
            distribution_family,
            StepActionSlot::NoAction,
        ),
        socket_step(
            message_catalog,
            clamd_socket_view,
            daemon_is_answering(&protection_evidence.daemon_evidence),
        ),
    ])
    .chain(runtime_reduction_step(
        message_catalog,
        moment_formatters,
        &protection_evidence.runtime_reductions,
        ReducedSettingGroup::EngineSettings,
        StepActionSlot::NoAction,
    ))
    .collect()
}

#[cfg(test)]
mod scan_engine_presentation_tests {
    use std::path::PathBuf;

    use super::*;
    use crate::{
        host_services::protection_evidence::{
            host_protection_evidence::{
                OverridesChangeTime, ProgramTrustFinding, ProgramTrustInspection, ReducedSetting,
                RuntimeReductions,
            },
            protection_evidence_fixtures::{english_formatters, healthy_evidence},
        },
        shared_widgets::step_timeline::timeline_step_contents::StepNodeState,
    };

    fn protected_socket() -> ClamdSocketView {
        ClamdSocketView::SocketInspected(ClamdSocketOwnership::SocketProtected {
            socket_path: PathBuf::from("/run/clamav/clamd.ctl"),
        })
    }

    fn engine_steps_with(
        message_catalog: &MessageCatalog,
        moment_formatters: &MomentFormatters,
        protection_evidence: &ProtectionEvidence,
    ) -> Vec<ScanEngineStep> {
        scan_engine_steps(
            message_catalog,
            moment_formatters,
            protection_evidence,
            &protected_socket(),
            DistributionFamily::NixosFamily,
        )
    }

    fn step_titles(engine_steps: &[ScanEngineStep]) -> Vec<&str> {
        engine_steps
            .iter()
            .map(|engine_step| engine_step.step_title.as_str())
            .collect()
    }

    fn daemon_step_state(
        message_catalog: &MessageCatalog,
        daemon_evidence: DaemonEvidence,
    ) -> StepNodeState {
        daemon_step::<ScanEngineAction>(
            message_catalog,
            &daemon_evidence,
            DistributionFamily::NixosFamily,
            StepActionSlot::NoAction,
        )
        .node_state()
    }

    #[test]
    fn a_healthy_engine_completes_every_step_in_order() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let moment_formatters = &english_formatters();
        let engine_steps =
            engine_steps_with(message_catalog, moment_formatters, &healthy_evidence());
        assert_eq!(
            step_titles(&engine_steps),
            vec![
                "ClamAV installed",
                "Daemon running",
                "clamd socket protected"
            ]
        );
        assert!(
            engine_steps
                .iter()
                .all(|engine_step| engine_step.node_state() == StepNodeState::Completed)
        );
    }

    #[test]
    fn an_active_daemon_that_does_not_answer_suggests_a_restart() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let daemon_step = daemon_step::<ScanEngineAction>(
            message_catalog,
            &DaemonEvidence::UnitActiveNotAnswering {
                service_unit: "clamav-daemon.service",
            },
            DistributionFamily::DebianFamily,
            StepActionSlot::NoAction,
        );
        assert_eq!(daemon_step.node_state(), StepNodeState::HasFailed);
        assert!(
            daemon_step
                .status_text
                .starts_with("clamav-daemon.service is running, but clamd does not answer")
        );
        assert_eq!(
            daemon_step.guidance_commands,
            vec!["sudo systemctl restart clamav-daemon".to_owned()]
        );
        assert_eq!(
            daemon_step_state(message_catalog, DaemonEvidence::UnitStateUnknown),
            StepNodeState::NeedsReview
        );
    }

    #[test]
    fn the_loaded_signature_version_is_compared_with_the_disk() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let behind_step = daemon_step::<ScanEngineAction>(
            message_catalog,
            &DaemonEvidence::ClamdAnswering {
                loaded_signatures: LoadedSignatures::BehindDisk {
                    loaded_version: "27100".to_owned(),
                    disk_version: "27102".to_owned(),
                },
            },
            DistributionFamily::NixosFamily,
            StepActionSlot::NoAction,
        );
        assert_eq!(behind_step.node_state(), StepNodeState::NeedsReview);
        assert_eq!(
            behind_step.shown_evidence_rows()[0].evidence_value,
            "27100, while 27102 is on disk"
        );
        let stopped_step = daemon_step::<ScanEngineAction>(
            message_catalog,
            &DaemonEvidence::UnitStopped {
                service_unit: "clamav-daemon.service",
            },
            DistributionFamily::NixosFamily,
            StepActionSlot::NoAction,
        );
        assert!(stopped_step.shown_evidence_rows().is_empty());
        assert_eq!(stopped_step.node_state(), StepNodeState::HasFailed);
    }

    #[test]
    fn a_missing_engine_suggests_installing_it() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let missing_step = engine_step::<ScanEngineAction>(
            message_catalog,
            &EngineInstallation::EngineMissing,
            DistributionFamily::DebianFamily,
            StepActionSlot::NoAction,
        );
        assert_eq!(missing_step.node_state(), StepNodeState::ActNext);
        assert!(!missing_step.guidance_commands.is_empty());
    }

    #[test]
    fn only_a_replaceable_scanner_adds_a_trust_step() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let moment_formatters = &english_formatters();
        let refused_scanner = engine_steps_with(
            message_catalog,
            moment_formatters,
            &ProtectionEvidence {
                program_trust: ProgramTrustInspection::ProgramsChecked(vec![
                    ProgramTrustFinding::OnlyReplaceableProgram {
                        program_name: "clamdscan",
                        replaceable_paths: vec!["/home/user/.local/bin/clamdscan".to_owned()],
                    },
                ]),
                ..healthy_evidence()
            },
        );
        assert_eq!(
            refused_scanner[1].step_title,
            "Scanner programs can be trusted"
        );
        assert_eq!(refused_scanner[1].node_state(), StepNodeState::HasFailed);
        let refused_updater = engine_steps_with(
            message_catalog,
            moment_formatters,
            &ProtectionEvidence {
                program_trust: ProgramTrustInspection::ProgramsChecked(vec![
                    ProgramTrustFinding::OnlyReplaceableProgram {
                        program_name: "freshclam",
                        replaceable_paths: vec!["/home/user/.local/bin/freshclam".to_owned()],
                    },
                ]),
                ..healthy_evidence()
            },
        );
        assert_eq!(refused_updater.len(), 3);
    }

    #[test]
    fn only_engine_key_reductions_appear_on_the_engine_page() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let moment_formatters = &english_formatters();
        let reductions_of = |setting_key: &str| ProtectionEvidence {
            runtime_reductions: RuntimeReductions::SettingsReduced {
                change_time: OverridesChangeTime::ChangeTimeUnknown,
                reduced_settings: vec![ReducedSetting {
                    setting_key: setting_key.to_owned(),
                    base_value: "yes".to_owned(),
                    changed_value: "no".to_owned(),
                }],
            },
            ..healthy_evidence()
        };
        assert_eq!(
            step_titles(&engine_steps_with(
                message_catalog,
                moment_formatters,
                &reductions_of("DetectPUA")
            ))
            .last(),
            Some(&"Protection reduced on purpose")
        );
        assert_eq!(
            engine_steps_with(
                message_catalog,
                moment_formatters,
                &reductions_of("OnAccessPrevention")
            )
            .len(),
            3
        );
    }

    #[test]
    fn a_socket_your_account_can_replace_fails_and_a_missing_one_waits_on_the_daemon() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let replaceable_step = socket_step(
            message_catalog,
            &ClamdSocketView::SocketInspected(ClamdSocketOwnership::SocketReplaceable {
                socket_path: PathBuf::from("/tmp/clamd.ctl"),
                replaceable_path: PathBuf::from("/tmp"),
            }),
            true,
        );
        assert_eq!(replaceable_step.node_state(), StepNodeState::HasFailed);
        let missing_socket =
            ClamdSocketView::SocketInspected(ClamdSocketOwnership::SocketMissing {
                socket_path: PathBuf::from("/run/clamav/clamd.ctl"),
            });
        assert_eq!(
            socket_step(message_catalog, &missing_socket, false).node_state(),
            StepNodeState::AwaitingPrerequisite
        );
        assert_eq!(
            socket_step(message_catalog, &missing_socket, true).node_state(),
            StepNodeState::NeedsReview
        );
        assert_eq!(
            socket_step(message_catalog, &ClamdSocketView::SocketInspecting, true).node_state(),
            StepNodeState::InProgress
        );
    }
}
