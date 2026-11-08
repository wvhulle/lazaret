use crate::{
    host_services::{
        clamav_programs::distribution_install_guides::DistributionFamily,
        desktop::locale_formatting::byte_size_text,
        protection_evidence::{
            host_protection_evidence::{
                ClamdConfigReadFailure, ConfiguredPrevention, OnAccessConfig, OnAccessConflict,
                OnAccessEvidence, OnAccessScanner, OnAccessSizeLimit, ProtectionEvidence,
                WatchScope, WatchedLocation, daemon_is_answering,
            },
            kernel_on_access_support::{KernelBlockingSupport, KernelConfigReadFailure},
            on_access_setup_review::{
                OnAccessBlocking, OnAccessSetupFinding, on_access_blocking,
                on_access_setup_findings,
            },
        },
    },
    shared_widgets::step_timeline::{
        runtime_reduction_steps::{ReducedSettingGroup, runtime_reduction_step},
        step_evidence_builders::{
            CheckedCondition, daemon_awaited_text, evidence_row, no_guidance_step, remedy_commands,
        },
        suggested_remedy_commands::SuggestedRemedy,
        timeline_step_contents::{
            ActionConsequence, ActionableEvidenceRow, EvidenceMoment, SettledStep,
            StepActionButton, StepActionSlot, StepEvidenceRow, TimelineStep,
        },
    },
    translated_messages,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OnAccessAction {}

impl StepActionButton for OnAccessAction {
    fn button_label(&self) -> String {
        match *self {}
    }

    fn action_consequence(&self) -> ActionConsequence {
        match *self {}
    }
}

pub type OnAccessStep = TimelineStep<OnAccessAction>;

fn on_access_setting_row<StepAction: StepActionButton>(
    evidence_name: String,
    evidence_value: String,
) -> ActionableEvidenceRow<StepAction> {
    StepEvidenceRow {
        evidence_moment: EvidenceMoment::SettingValue,
        ..evidence_row(evidence_name, evidence_value)
    }
    .into()
}

fn clamd_config_failure_text(config_failure: ClamdConfigReadFailure) -> String {
    match config_failure {
        ClamdConfigReadFailure::NoReadableConfig => {
            translated_messages::scans::no_readable_config()
        }
        ClamdConfigReadFailure::ReadingTaskAborted => {
            translated_messages::scans::reading_task_aborted_clamd()
        }
    }
}

fn watched_location_text(watched_location: &WatchedLocation) -> String {
    match watched_location {
        WatchedLocation::WatchedFolder { folder_path } => folder_path.clone(),
        WatchedLocation::WatchedMount { mount_path } => {
            translated_messages::scans::watched_mount(mount_path)
        }
    }
}

fn watch_scope_text(watch_scope: &WatchScope) -> String {
    match watch_scope {
        WatchScope::NothingWatched => translated_messages::scans::nothing_watched(),
        WatchScope::WatchedLocations { .. } => watch_scope
            .watched_locations()
            .into_iter()
            .map(watched_location_text)
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

fn size_limit_text(size_limit: &OnAccessSizeLimit) -> String {
    match size_limit {
        OnAccessSizeLimit::DefaultLimit => translated_messages::scans::default_limit(),
        OnAccessSizeLimit::ConfiguredLimit { size_limit } => byte_size_text(*size_limit),
        OnAccessSizeLimit::UnreadableLimit { limit_text } => limit_text.clone(),
    }
}

fn on_access_rows<StepAction: StepActionButton>(
    on_access_config: &OnAccessConfig,
) -> Vec<ActionableEvidenceRow<StepAction>> {
    match on_access_config {
        OnAccessConfig::ConfigUnreadable(config_failure) => vec![on_access_setting_row(
            translated_messages::scans::configuration_label(),
            clamd_config_failure_text(*config_failure),
        )],
        OnAccessConfig::ConfigRead {
            watch_scope,
            size_limit,
            ..
        } => vec![
            on_access_setting_row(
                translated_messages::scans::watched_folders(),
                watch_scope_text(watch_scope),
            ),
            on_access_setting_row(
                translated_messages::scans::largest_file_scanned(),
                size_limit_text(size_limit),
            ),
        ],
    }
}

struct OnAccessVerdict {
    checked_condition: CheckedCondition,
    status_text: String,
    suggested_remedies: Vec<SuggestedRemedy>,
}

fn kernel_config_failure_text(read_failure: KernelConfigReadFailure) -> String {
    match read_failure {
        KernelConfigReadFailure::NoKernelConfigFound => {
            translated_messages::scans::no_kernel_config_found()
        }
        KernelConfigReadFailure::KernelConfigUnreadable { .. } => {
            translated_messages::scans::kernel_config_unreadable()
        }
        KernelConfigReadFailure::CompressedConfigCorrupt => {
            translated_messages::scans::compressed_config_corrupt()
        }
        KernelConfigReadFailure::KernelReleaseUnreadable { .. } => {
            translated_messages::scans::kernel_release_unreadable()
        }
        KernelConfigReadFailure::ReadingTaskAborted => {
            translated_messages::scans::reading_task_aborted_config()
        }
    }
}

fn watched_blocking_verdict(on_access_blocking: OnAccessBlocking) -> OnAccessVerdict {
    match on_access_blocking {
        OnAccessBlocking::BlocksDetectedFiles => OnAccessVerdict {
            checked_condition: CheckedCondition::ConditionMet,
            status_text: translated_messages::scans::blocks_detected_files(),
            suggested_remedies: Vec::new(),
        },
        OnAccessBlocking::PreventionOff => OnAccessVerdict {
            checked_condition: CheckedCondition::ReviewSuggested,
            status_text: translated_messages::scans::prevention_off(),
            suggested_remedies: vec![SuggestedRemedy::OnAccessPreventionEnable],
        },
        OnAccessBlocking::PreventionConflictsWithMounts => OnAccessVerdict {
            checked_condition: CheckedCondition::ReviewSuggested,
            status_text: translated_messages::scans::prevention_conflicts_mounts(),
            suggested_remedies: Vec::new(),
        },
        OnAccessBlocking::KernelCannotBlock => OnAccessVerdict {
            checked_condition: CheckedCondition::ReviewSuggested,
            status_text: translated_messages::scans::kernel_cannot_block(),
            suggested_remedies: Vec::new(),
        },
        OnAccessBlocking::KernelSupportUnknown(read_failure) => OnAccessVerdict {
            checked_condition: CheckedCondition::EvidenceUnknown,
            status_text: translated_messages::scans::kernel_support_unknown(
                kernel_config_failure_text(read_failure),
            ),
            suggested_remedies: Vec::new(),
        },
    }
}

fn running_monitor_verdict(
    on_access_config: &OnAccessConfig,
    kernel_blocking_support: KernelBlockingSupport,
) -> OnAccessVerdict {
    match on_access_config {
        OnAccessConfig::ConfigUnreadable(_) => OnAccessVerdict {
            checked_condition: CheckedCondition::EvidenceUnknown,
            status_text: translated_messages::scans::config_unreadable_monitor(),
            suggested_remedies: Vec::new(),
        },
        OnAccessConfig::ConfigRead {
            watch_scope: WatchScope::NothingWatched,
            configured_prevention: ConfiguredPrevention::PreventionRequested,
            ..
        } => OnAccessVerdict {
            checked_condition: CheckedCondition::ReviewSuggested,
            status_text: translated_messages::scans::running_no_folders_status(),
            suggested_remedies: vec![SuggestedRemedy::OnAccessWatchedPathAdd],
        },
        OnAccessConfig::ConfigRead {
            watch_scope: WatchScope::NothingWatched,
            configured_prevention: ConfiguredPrevention::PreventionOff,
            ..
        } => OnAccessVerdict {
            checked_condition: CheckedCondition::ReviewSuggested,
            status_text: translated_messages::scans::running_detected_status(),
            suggested_remedies: vec![
                SuggestedRemedy::OnAccessWatchedPathAdd,
                SuggestedRemedy::OnAccessPreventionEnable,
            ],
        },
        OnAccessConfig::ConfigRead {
            watch_scope: WatchScope::WatchedLocations { .. },
            configured_prevention,
            setting_conflicts,
            ..
        } => watched_blocking_verdict(on_access_blocking(
            *configured_prevention,
            setting_conflicts,
            kernel_blocking_support,
        )),
    }
}

fn on_access_verdict(
    on_access_evidence: &OnAccessEvidence,
    daemon_answering: bool,
) -> OnAccessVerdict {
    match on_access_evidence.on_access_scanner {
        OnAccessScanner::UnitStateUnknown => OnAccessVerdict {
            checked_condition: CheckedCondition::EvidenceUnknown,
            status_text: translated_messages::scans::unit_state_unknown_access(),
            suggested_remedies: Vec::new(),
        },
        OnAccessScanner::NeverSetUp => OnAccessVerdict {
            checked_condition: CheckedCondition::ReviewSuggested,
            status_text: translated_messages::scans::never_set_up_access(),
            suggested_remedies: vec![SuggestedRemedy::OnAccessScanningEnable],
        },
        OnAccessScanner::UnitStopped { .. } if !daemon_answering => OnAccessVerdict {
            checked_condition: CheckedCondition::PrerequisiteMissing,
            status_text: daemon_awaited_text(),
            suggested_remedies: vec![SuggestedRemedy::OnAccessScanningEnable],
        },
        OnAccessScanner::UnitStopped { service_unit } => OnAccessVerdict {
            checked_condition: CheckedCondition::ConditionFailed,
            status_text: translated_messages::scans::unit_stopped_access(service_unit),
            suggested_remedies: vec![SuggestedRemedy::OnAccessScanningEnable],
        },
        OnAccessScanner::MonitorRunning if !daemon_answering => OnAccessVerdict {
            checked_condition: CheckedCondition::PrerequisiteMissing,
            status_text: daemon_awaited_text(),
            suggested_remedies: Vec::new(),
        },
        OnAccessScanner::MonitorRunning => running_monitor_verdict(
            &on_access_evidence.on_access_config,
            on_access_evidence.kernel_blocking_support,
        ),
    }
}

pub fn on_access_condition(
    on_access_evidence: &OnAccessEvidence,
    daemon_answering: bool,
) -> CheckedCondition {
    on_access_verdict(on_access_evidence, daemon_answering).checked_condition
}

pub fn on_access_step<StepAction: StepActionButton>(
    on_access_evidence: &OnAccessEvidence,
    daemon_answering: bool,
    distribution_family: DistributionFamily,
    header_action: StepActionSlot<StepAction>,
) -> TimelineStep<StepAction> {
    let step_title = translated_messages::scans::access_blocking_title();
    let mechanism_details = translated_messages::scans::clamonacc_checks_files_watched();
    let on_access_verdict = on_access_verdict(on_access_evidence, daemon_answering);
    TimelineStep::settled(
        step_title,
        mechanism_details,
        on_access_verdict.status_text,
        SettledStep {
            evidence_rows: on_access_rows(&on_access_evidence.on_access_config),
            header_action,
            ..SettledStep::without_offers(on_access_verdict.checked_condition.node_state())
        },
    )
    .with_guidance_commands(
        on_access_verdict
            .suggested_remedies
            .into_iter()
            .flat_map(|suggested_remedy| remedy_commands(suggested_remedy, distribution_family))
            .collect(),
    )
}

fn conflicting_options_text(setting_conflicts: &[OnAccessConflict]) -> String {
    setting_conflicts
        .iter()
        .map(|setting_conflict| setting_conflict.conflicting_option())
        .collect::<Vec<_>>()
        .join(", ")
}

fn setup_finding_step<StepAction: StepActionButton>(
    setup_finding: &OnAccessSetupFinding,
) -> TimelineStep<StepAction> {
    let checked_condition = if setup_finding.evidence_is_missing() {
        CheckedCondition::EvidenceUnknown
    } else {
        CheckedCondition::ReviewSuggested
    };
    let (step_title, mechanism_details, status_text) = match setup_finding {
        OnAccessSetupFinding::MountWatchConflicts { setting_conflicts } => (
            translated_messages::scans::mount_watching(),
            translated_messages::scans::onaccessmountpath_watches_whole_mount(),
            translated_messages::scans::onaccessmountpath_cannot_combined_options(
                conflicting_options_text(setting_conflicts),
            ),
        ),
        OnAccessSetupFinding::ClamdUserNotExcluded => (
            translated_messages::scans::loop_protection(),
            translated_messages::scans::clamd_opens_every_file(),
            translated_messages::scans::clamd_not_excluded_clamonacc(),
        ),
        OnAccessSetupFinding::WholeRootIncluded => (
            translated_messages::scans::watching_whole_file_system(),
            translated_messages::scans::clamonacc_refuses_onaccessincludepath_include(),
            translated_messages::scans::onaccessincludepath_rejected_clamonacc(),
        ),
        OnAccessSetupFinding::WatchLimitLow {
            max_user_watches,
            large_tree_path,
        } => (
            translated_messages::scans::folder_watch_limit(),
            watch_limit_details(),
            translated_messages::scans::path_watched_fs_inotify(large_tree_path, max_user_watches),
        ),
        OnAccessSetupFinding::WatchLimitUnknown {
            large_tree_path, ..
        } => (
            translated_messages::scans::folder_watch_limit(),
            watch_limit_details(),
            translated_messages::scans::unknown_whether_path_fits(large_tree_path),
        ),
    };
    no_guidance_step(
        step_title,
        mechanism_details,
        checked_condition.node_state(),
        status_text,
    )
}

fn watch_limit_details() -> String {
    translated_messages::scans::clamonacc_watches_every_folder()
}

pub fn on_access_setup_steps<StepAction: StepActionButton>(
    on_access_evidence: &OnAccessEvidence,
) -> Vec<TimelineStep<StepAction>> {
    on_access_setup_findings(on_access_evidence)
        .iter()
        .map(setup_finding_step)
        .collect()
}

pub fn on_access_page_steps(
    protection_evidence: &ProtectionEvidence,
    distribution_family: DistributionFamily,
) -> Vec<OnAccessStep> {
    std::iter::once(on_access_step(
        &protection_evidence.on_access_evidence,
        daemon_is_answering(&protection_evidence.daemon_evidence),
        distribution_family,
        StepActionSlot::NoAction,
    ))
    .chain(on_access_setup_steps(
        &protection_evidence.on_access_evidence,
    ))
    .chain(runtime_reduction_step(
        &protection_evidence.runtime_reductions,
        ReducedSettingGroup::OnAccessSettings,
        StepActionSlot::NoAction,
    ))
    .collect()
}

#[cfg(test)]
mod on_access_presentation_tests {
    use super::*;
    use crate::{
        host_services::protection_evidence::{
            host_protection_evidence::{
                ClamdUserExclusion, DaemonEvidence, OverridesChangeTime, ReducedSetting,
                RuntimeReductions,
            },
            protection_evidence_fixtures::healthy_evidence,
        },
        shared_widgets::step_timeline::timeline_step_contents::StepNodeState,
    };

    fn on_access_with(on_access_config: OnAccessConfig) -> OnAccessEvidence {
        OnAccessEvidence {
            on_access_scanner: OnAccessScanner::MonitorRunning,
            on_access_config,
            ..healthy_evidence().on_access_evidence
        }
    }

    fn page_step_of(on_access_evidence: OnAccessEvidence) -> OnAccessStep {
        on_access_page_steps(
            &ProtectionEvidence {
                on_access_evidence,
                ..healthy_evidence()
            },
            DistributionFamily::NixosFamily,
        )
        .remove(0)
    }

    fn row_value<'step>(on_access_step: &'step OnAccessStep, evidence_name: &str) -> &'step str {
        &on_access_step
            .shown_evidence_rows()
            .into_iter()
            .find(|step_evidence| step_evidence.evidence_name == evidence_name)
            .unwrap()
            .evidence_value
    }

    #[test]
    fn running_without_prevention_is_never_reported_as_blocking() {
        let on_access_step = page_step_of(on_access_with(OnAccessConfig::ConfigRead {
            watch_scope: WatchScope::WatchedLocations {
                first_location: WatchedLocation::WatchedFolder {
                    folder_path: "/home/user/Downloads".to_owned(),
                },
                other_locations: Vec::new(),
            },
            configured_prevention: ConfiguredPrevention::PreventionOff,
            setting_conflicts: Vec::new(),
            clamd_user_exclusion: ClamdUserExclusion::ClamdUserExcluded,
            size_limit: OnAccessSizeLimit::DefaultLimit,
        }));
        assert_eq!(on_access_step.node_state(), StepNodeState::NeedsReview);
        assert_eq!(
            on_access_step.status_text,
            "Files are checked, but detected files are not blocked"
        );
    }

    #[test]
    fn an_on_access_unit_systemd_could_not_report_is_unknown_rather_than_not_set_up() {
        let on_access_step = page_step_of(OnAccessEvidence {
            on_access_scanner: OnAccessScanner::UnitStateUnknown,
            ..healthy_evidence().on_access_evidence
        });
        assert_eq!(on_access_step.node_state(), StepNodeState::NeedsReview);
        assert!(on_access_step.status_text.starts_with("Unknown"));
        assert!(!on_access_step.status_text.contains("not set up"));
    }

    #[test]
    fn an_unreadable_daemon_config_claims_no_missing_blocking() {
        let on_access_step = page_step_of(on_access_with(OnAccessConfig::ConfigUnreadable(
            ClamdConfigReadFailure::NoReadableConfig,
        )));
        assert_eq!(on_access_step.node_state(), StepNodeState::NeedsReview);
        assert!(
            on_access_step
                .status_text
                .contains("clamd.conf could not be read")
        );
        assert!(!on_access_step.status_text.contains("not blocked"));
        assert!(on_access_step.guidance_commands.is_empty());
        assert_eq!(
            row_value(&on_access_step, "Configuration"),
            "No readable clamd.conf was found"
        );
    }

    #[test]
    fn prevention_without_watched_locations_suggests_adding_a_watched_folder() {
        let on_access_step = page_step_of(on_access_with(OnAccessConfig::ConfigRead {
            watch_scope: WatchScope::NothingWatched,
            configured_prevention: ConfiguredPrevention::PreventionRequested,
            setting_conflicts: Vec::new(),
            clamd_user_exclusion: ClamdUserExclusion::ClamdUserExcluded,
            size_limit: OnAccessSizeLimit::DefaultLimit,
        }));
        assert_eq!(on_access_step.node_state(), StepNodeState::NeedsReview);
        assert_eq!(
            on_access_step.status_text,
            "Running, but no folders or mounts are watched"
        );
        assert_eq!(
            on_access_step.guidance_commands,
            remedy_commands(
                SuggestedRemedy::OnAccessWatchedPathAdd,
                DistributionFamily::NixosFamily
            )
        );
    }

    #[test]
    fn a_watched_mount_is_shown_as_a_whole_mount() {
        let on_access_step = page_step_of(on_access_with(OnAccessConfig::ConfigRead {
            watch_scope: WatchScope::WatchedLocations {
                first_location: WatchedLocation::WatchedMount {
                    mount_path: "/".to_owned(),
                },
                other_locations: Vec::new(),
            },
            configured_prevention: ConfiguredPrevention::PreventionRequested,
            setting_conflicts: Vec::new(),
            clamd_user_exclusion: ClamdUserExclusion::ClamdUserExcluded,
            size_limit: OnAccessSizeLimit::DefaultLimit,
        }));
        assert_eq!(on_access_step.node_state(), StepNodeState::Completed);
        assert_eq!(
            row_value(&on_access_step, "Watched folders"),
            "/ (whole mount)"
        );
    }

    #[test]
    fn setting_rows_on_the_page_link_nowhere_because_the_settings_are_below() {
        let on_access_step = page_step_of(healthy_evidence().on_access_evidence);
        assert!(on_access_step.offered_actions().is_empty());
    }

    #[test]
    fn a_stopped_daemon_leaves_on_access_waiting() {
        let page_steps = on_access_page_steps(
            &ProtectionEvidence {
                daemon_evidence: DaemonEvidence::UnitStopped {
                    service_unit: "clamav-daemon.service",
                },
                ..healthy_evidence()
            },
            DistributionFamily::NixosFamily,
        );
        assert_eq!(
            page_steps[0].node_state(),
            StepNodeState::AwaitingPrerequisite
        );
    }

    #[test]
    fn only_on_access_key_reductions_appear_on_this_page() {
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
            on_access_page_steps(
                &reductions_of("OnAccessPrevention"),
                DistributionFamily::NixosFamily
            )
            .len(),
            2
        );
        assert_eq!(
            on_access_page_steps(&reductions_of("DetectPUA"), DistributionFamily::NixosFamily)
                .len(),
            1
        );
    }

    fn watched_downloads() -> WatchScope {
        WatchScope::WatchedLocations {
            first_location: WatchedLocation::WatchedFolder {
                folder_path: "/home/user/Downloads".to_owned(),
            },
            other_locations: Vec::new(),
        }
    }

    #[test]
    fn mount_watching_with_prevention_is_shown_as_not_blocking_with_its_conflict() {
        let page_steps = on_access_page_steps(
            &ProtectionEvidence {
                on_access_evidence: on_access_with(OnAccessConfig::ConfigRead {
                    watch_scope: WatchScope::WatchedLocations {
                        first_location: WatchedLocation::WatchedMount {
                            mount_path: "/media".to_owned(),
                        },
                        other_locations: Vec::new(),
                    },
                    configured_prevention: ConfiguredPrevention::PreventionRequested,
                    setting_conflicts: vec![OnAccessConflict::PreventionWithMountPath],
                    clamd_user_exclusion: ClamdUserExclusion::ClamdUserExcluded,
                    size_limit: OnAccessSizeLimit::DefaultLimit,
                }),
                ..healthy_evidence()
            },
            DistributionFamily::NixosFamily,
        );
        assert_eq!(page_steps[0].node_state(), StepNodeState::NeedsReview);
        assert!(page_steps[0].status_text.contains("not blocked"));
        assert_eq!(page_steps[1].node_state(), StepNodeState::NeedsReview);
        assert_eq!(
            page_steps[1].status_text,
            "OnAccessMountPath can't be combined with OnAccessPrevention"
        );
    }

    #[test]
    fn an_unreadable_kernel_config_is_unknown_and_never_blocking() {
        let on_access_step = page_step_of(OnAccessEvidence {
            kernel_blocking_support: KernelBlockingSupport::KernelConfigUnreadable(
                KernelConfigReadFailure::NoKernelConfigFound,
            ),
            ..healthy_evidence().on_access_evidence
        });
        assert_eq!(on_access_step.node_state(), StepNodeState::NeedsReview);
        assert!(on_access_step.status_text.contains("unknown"));
        assert_ne!(on_access_step.status_text, "Detected files are blocked");
    }

    #[test]
    fn a_notify_only_kernel_says_it_cannot_block() {
        let on_access_step = page_step_of(OnAccessEvidence {
            kernel_blocking_support: KernelBlockingSupport::NotifyOnly,
            ..healthy_evidence().on_access_evidence
        });
        assert_eq!(on_access_step.node_state(), StepNodeState::NeedsReview);
        assert!(on_access_step.status_text.contains("kernel can't block"));
    }

    #[test]
    fn a_missing_clamd_exclusion_gets_its_own_review_step() {
        let page_steps = on_access_page_steps(
            &ProtectionEvidence {
                on_access_evidence: on_access_with(OnAccessConfig::ConfigRead {
                    watch_scope: watched_downloads(),
                    configured_prevention: ConfiguredPrevention::PreventionRequested,
                    setting_conflicts: Vec::new(),
                    clamd_user_exclusion: ClamdUserExclusion::ClamdUserNotExcluded,
                    size_limit: OnAccessSizeLimit::DefaultLimit,
                }),
                ..healthy_evidence()
            },
            DistributionFamily::NixosFamily,
        );
        assert_eq!(page_steps[0].node_state(), StepNodeState::Completed);
        assert_eq!(page_steps[1].node_state(), StepNodeState::NeedsReview);
        assert_eq!(
            page_steps[1].status_text,
            "clamd isn't excluded, so clamonacc can scan its own reads endlessly"
        );
    }

    #[test]
    fn a_healthy_setup_adds_no_review_steps() {
        assert_eq!(
            on_access_page_steps(&healthy_evidence(), DistributionFamily::NixosFamily).len(),
            1
        );
    }
}
