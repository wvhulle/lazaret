use crate::{
    host_services::{
        clamav_programs::distribution_install_guides::DistributionFamily,
        detection_journal::detection_journal_follower::JournalAccess,
        protection_evidence::{
            host_protection_evidence::ProtectionEvidence,
            process_attach_scope::{ProcessAttachReadFailure, ProcessAttachScope},
        },
        systemd_bus::host_systemd_units::{UnitActivity, UnitState},
    },
    shared_widgets::{
        journal_follow_failure_text::journal_follow_failure_text,
        step_timeline::{
            step_evidence_builders::{CheckedCondition, evidence_row, plain_rows, remedy_commands},
            suggested_remedy_commands::SuggestedRemedy,
            timeline_step_contents::{
                ActionConsequence, ActionableEvidenceRow, OfferedAction, SettledStep,
                StepActionButton, StepActionSlot, TimelineStep,
            },
        },
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

const NIXOS_PTRACE_SNIPPET: &str = "boot.kernel.sysctl.\"kernel.yama.ptrace_scope\" = 1;";
const SYSCTL_PTRACE_COMMAND: &str = "echo 'kernel.yama.ptrace_scope = 1' | sudo tee \
                                     /etc/sysctl.d/10-ptrace.conf && sudo sysctl --system";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlertStepAction {
    OpenPreferences,
}

impl StepActionButton for AlertStepAction {
    fn button_label(&self, message_catalog: &MessageCatalog) -> String {
        match self {
            Self::OpenPreferences => {
                translated_messages::findings::open_preferences(message_catalog)
            }
        }
    }

    fn action_consequence(&self) -> ActionConsequence {
        match self {
            Self::OpenPreferences => ActionConsequence::OrdinaryAction,
        }
    }
}

pub type AlertStep = TimelineStep<AlertStepAction>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlertReadiness {
    pub journal_access: JournalAccess,
    pub notifications_enabled: bool,
}

const fn background_service_runs(background_service_state: UnitState) -> bool {
    matches!(
        background_service_state,
        UnitState::UnitLoaded(UnitActivity::UnitActive)
    )
}

pub fn alerts_condition(
    alert_readiness: &AlertReadiness,
    background_service_state: UnitState,
) -> CheckedCondition {
    match &alert_readiness.journal_access {
        JournalAccess::JournalUnreadable | JournalAccess::FollowingFailed(_) => {
            CheckedCondition::ConditionFailed
        }
        JournalAccess::FollowingJournal
            if !alert_readiness.notifications_enabled
                || !background_service_runs(background_service_state) =>
        {
            CheckedCondition::ReviewSuggested
        }
        JournalAccess::FollowingJournal => CheckedCondition::ConditionMet,
    }
}

fn alerts_status_text(
    message_catalog: &MessageCatalog,
    alert_readiness: &AlertReadiness,
    background_service_state: UnitState,
) -> String {
    match &alert_readiness.journal_access {
        JournalAccess::JournalUnreadable => {
            translated_messages::findings::journal_unreadable_alerts(message_catalog)
        }
        JournalAccess::FollowingFailed(follow_failure) => {
            journal_follow_failure_text(message_catalog, follow_failure)
        }
        JournalAccess::FollowingJournal if !alert_readiness.notifications_enabled => {
            translated_messages::findings::notifications_turned_off_preferences(message_catalog)
        }
        JournalAccess::FollowingJournal if !background_service_runs(background_service_state) => {
            translated_messages::findings::alerts_stop_quit_lazaret(message_catalog)
        }
        JournalAccess::FollowingJournal => {
            translated_messages::findings::running_background_following_journal(message_catalog)
        }
    }
}

fn alerts_timeline_step<StepAction: StepActionButton>(
    message_catalog: &MessageCatalog,
    alert_readiness: &AlertReadiness,
    background_service_state: UnitState,
    header_action: StepActionSlot<StepAction>,
    evidence_rows: Vec<ActionableEvidenceRow<StepAction>>,
) -> TimelineStep<StepAction> {
    TimelineStep::settled(
        translated_messages::findings::alerts_label(message_catalog),
        translated_messages::findings::lazaret_follows_system_journal(message_catalog),
        alerts_status_text(message_catalog, alert_readiness, background_service_state),
        SettledStep {
            header_action,
            evidence_rows,
            ..SettledStep::without_offers(
                alerts_condition(alert_readiness, background_service_state).node_state(),
            )
        },
    )
}

fn notifications_text(message_catalog: &MessageCatalog, notifications_enabled: bool) -> String {
    if notifications_enabled {
        translated_messages::findings::notifications_on(message_catalog)
    } else {
        translated_messages::findings::notifications_off(message_catalog)
    }
}

fn background_service_text(
    message_catalog: &MessageCatalog,
    background_service_state: UnitState,
) -> String {
    match background_service_state {
        UnitState::UnitLoaded(UnitActivity::UnitActive) => {
            translated_messages::findings::running_label(message_catalog)
        }
        UnitState::UnitLoaded(UnitActivity::UnitInactive) => {
            translated_messages::findings::not_running(message_catalog)
        }
        UnitState::UnitNotFound => translated_messages::findings::unit_not_found(message_catalog),
        UnitState::SystemdUnavailable => {
            translated_messages::findings::systemd_unavailable(message_catalog)
        }
    }
}

fn journal_access_text(message_catalog: &MessageCatalog, journal_access: &JournalAccess) -> String {
    match journal_access {
        JournalAccess::FollowingJournal => {
            translated_messages::findings::following_journal_access(message_catalog)
        }
        JournalAccess::JournalUnreadable => {
            translated_messages::findings::journal_unreadable_access(message_catalog)
        }
        JournalAccess::FollowingFailed(_) => {
            translated_messages::findings::following_failed(message_catalog)
        }
    }
}

pub fn alert_delivery_step(
    message_catalog: &MessageCatalog,
    alert_readiness: &AlertReadiness,
    background_service_state: UnitState,
    distribution_family: DistributionFamily,
) -> AlertStep {
    let preferences_offer = if alert_readiness.notifications_enabled {
        StepActionSlot::NoAction
    } else {
        StepActionSlot::ActionOffered(OfferedAction::suggested(AlertStepAction::OpenPreferences))
    };
    let guidance_commands = if alert_readiness.journal_access == JournalAccess::FollowingJournal
        && alert_readiness.notifications_enabled
        && !background_service_runs(background_service_state)
    {
        remedy_commands(SuggestedRemedy::BackgroundAlertsEnable, distribution_family)
    } else {
        Vec::new()
    };
    alerts_timeline_step(
        message_catalog,
        alert_readiness,
        background_service_state,
        preferences_offer,
        plain_rows([
            evidence_row(
                translated_messages::findings::notifications_label(message_catalog),
                notifications_text(message_catalog, alert_readiness.notifications_enabled),
            ),
            evidence_row(
                translated_messages::findings::background_service(message_catalog),
                background_service_text(message_catalog, background_service_state),
            ),
            evidence_row(
                translated_messages::findings::system_journal(message_catalog),
                journal_access_text(message_catalog, &alert_readiness.journal_access),
            ),
        ]),
    )
    .with_guidance_commands(guidance_commands)
}

pub const fn process_attach_condition(
    process_attach_scope: ProcessAttachScope,
) -> CheckedCondition {
    match process_attach_scope {
        ProcessAttachScope::AttachRestricted => CheckedCondition::ConditionMet,
        ProcessAttachScope::AnyProgramMayAttach | ProcessAttachScope::YamaNotLoaded => {
            CheckedCondition::ReviewSuggested
        }
        ProcessAttachScope::ScopeUnreadable(_) => CheckedCondition::EvidenceUnknown,
    }
}

fn process_attach_failure_text(
    message_catalog: &MessageCatalog,
    read_failure: ProcessAttachReadFailure,
) -> String {
    match read_failure {
        ProcessAttachReadFailure::ScopeFileUnreadable { failure_kind } => {
            translated_messages::findings::attach_unknown_setting(message_catalog, failure_kind)
        }
        ProcessAttachReadFailure::ScopeValueUnrecognized => {
            translated_messages::findings::attach_unknown_reports(message_catalog)
        }
        ProcessAttachReadFailure::ReadingTaskInterrupted => {
            translated_messages::findings::attach_unknown_check(message_catalog)
        }
    }
}

fn process_attach_status_text(
    message_catalog: &MessageCatalog,
    process_attach_scope: ProcessAttachScope,
) -> String {
    match process_attach_scope {
        ProcessAttachScope::AttachRestricted => {
            translated_messages::findings::attach_restricted(message_catalog)
        }
        ProcessAttachScope::AnyProgramMayAttach => {
            translated_messages::findings::any_program_may_attach(message_catalog)
        }
        ProcessAttachScope::YamaNotLoaded => {
            translated_messages::findings::yama_not_loaded(message_catalog)
        }
        ProcessAttachScope::ScopeUnreadable(read_failure) => {
            process_attach_failure_text(message_catalog, read_failure)
        }
    }
}

const fn ptrace_scope_command(distribution_family: DistributionFamily) -> &'static str {
    match distribution_family {
        DistributionFamily::NixosFamily => NIXOS_PTRACE_SNIPPET,
        DistributionFamily::DebianFamily
        | DistributionFamily::FedoraWorkstation
        | DistributionFamily::FedoraAtomic
        | DistributionFamily::ArchFamily
        | DistributionFamily::UnrecognizedFamily => SYSCTL_PTRACE_COMMAND,
    }
}

pub fn process_attach_step(
    message_catalog: &MessageCatalog,
    process_attach_scope: ProcessAttachScope,
    distribution_family: DistributionFamily,
) -> AlertStep {
    let guidance_commands = match process_attach_scope {
        ProcessAttachScope::AnyProgramMayAttach => {
            vec![ptrace_scope_command(distribution_family).to_owned()]
        }
        ProcessAttachScope::AttachRestricted
        | ProcessAttachScope::YamaNotLoaded
        | ProcessAttachScope::ScopeUnreadable(_) => Vec::new(),
    };
    TimelineStep::settled(
        translated_messages::findings::other_programs_cannot_attach(message_catalog),
        translated_messages::findings::program_attaches_lazaret_hide(message_catalog),
        process_attach_status_text(message_catalog, process_attach_scope),
        SettledStep::without_offers(process_attach_condition(process_attach_scope).node_state()),
    )
    .with_guidance_commands(guidance_commands)
}

pub fn alert_group_steps(
    message_catalog: &MessageCatalog,
    protection_evidence: &ProtectionEvidence,
    alert_readiness: &AlertReadiness,
    distribution_family: DistributionFamily,
) -> Vec<AlertStep> {
    vec![
        alert_delivery_step(
            message_catalog,
            alert_readiness,
            protection_evidence.background_service_state,
            distribution_family,
        ),
        process_attach_step(
            message_catalog,
            protection_evidence.process_attach_scope,
            distribution_family,
        ),
    ]
}

#[cfg(test)]
mod alert_delivery_presentation_tests {
    use super::*;
    use crate::{
        host_services::protection_evidence::protection_evidence_fixtures::healthy_evidence,
        shared_widgets::step_timeline::timeline_step_contents::StepNodeState,
    };

    fn ready_alerts() -> AlertReadiness {
        AlertReadiness {
            journal_access: JournalAccess::FollowingJournal,
            notifications_enabled: true,
        }
    }

    fn alert_step_state(
        message_catalog: &MessageCatalog,
        alert_readiness: &AlertReadiness,
        background_service_state: UnitState,
    ) -> StepNodeState {
        alert_delivery_step(
            message_catalog,
            alert_readiness,
            background_service_state,
            DistributionFamily::NixosFamily,
        )
        .node_state()
    }

    #[test]
    fn alerts_are_only_complete_when_they_will_actually_fire() {
        let message_catalog = &crate::translated_messages::english_catalog();
        assert_eq!(
            alert_step_state(
                message_catalog,
                &ready_alerts(),
                UnitState::UnitLoaded(UnitActivity::UnitActive)
            ),
            StepNodeState::Completed
        );
        assert_eq!(
            alert_step_state(message_catalog, &ready_alerts(), UnitState::UnitNotFound),
            StepNodeState::NeedsReview
        );
        assert_eq!(
            alert_step_state(
                message_catalog,
                &AlertReadiness {
                    notifications_enabled: false,
                    ..ready_alerts()
                },
                UnitState::UnitLoaded(UnitActivity::UnitActive)
            ),
            StepNodeState::NeedsReview
        );
        assert_eq!(
            alert_step_state(
                message_catalog,
                &AlertReadiness {
                    journal_access: JournalAccess::JournalUnreadable,
                    ..ready_alerts()
                },
                UnitState::UnitLoaded(UnitActivity::UnitActive)
            ),
            StepNodeState::HasFailed
        );
    }

    #[test]
    fn the_alert_step_shows_notifications_service_and_journal() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let delivery_step = alert_delivery_step(
            message_catalog,
            &ready_alerts(),
            UnitState::UnitNotFound,
            DistributionFamily::NixosFamily,
        );
        let shown_rows = delivery_step
            .shown_evidence_rows()
            .into_iter()
            .map(|step_evidence| {
                (
                    step_evidence.evidence_name.as_str(),
                    step_evidence.evidence_value.as_str(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            shown_rows,
            vec![
                ("Notifications", "On"),
                ("Background service", "Not set up"),
                ("System journal", "Followed"),
            ]
        );
        assert_eq!(
            delivery_step.guidance_commands,
            vec!["services.lazaret.enable = true;".to_owned()]
        );
        assert_eq!(delivery_step.header_offer(), None);
    }

    #[test]
    fn turned_off_notifications_offer_the_preferences() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let delivery_step = alert_delivery_step(
            message_catalog,
            &AlertReadiness {
                notifications_enabled: false,
                ..ready_alerts()
            },
            UnitState::UnitLoaded(UnitActivity::UnitActive),
            DistributionFamily::NixosFamily,
        );
        assert_eq!(
            delivery_step.header_offer(),
            Some(OfferedAction::suggested(AlertStepAction::OpenPreferences))
        );
        assert!(delivery_step.guidance_commands.is_empty());
    }

    #[test]
    fn the_process_attach_step_follows_the_ptrace_scope() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let restricted_step = process_attach_step(
            message_catalog,
            ProcessAttachScope::AttachRestricted,
            DistributionFamily::NixosFamily,
        );
        assert_eq!(restricted_step.node_state(), StepNodeState::Completed);
        assert_eq!(
            restricted_step.step_title,
            "Other programs can't attach to Lazaret"
        );
        assert!(restricted_step.guidance_commands.is_empty());
        let open_step = process_attach_step(
            message_catalog,
            ProcessAttachScope::AnyProgramMayAttach,
            DistributionFamily::DebianFamily,
        );
        assert_eq!(open_step.node_state(), StepNodeState::NeedsReview);
        assert_eq!(
            open_step.guidance_commands,
            vec![SYSCTL_PTRACE_COMMAND.to_owned()]
        );
        assert_eq!(
            process_attach_step(
                message_catalog,
                ProcessAttachScope::YamaNotLoaded,
                DistributionFamily::NixosFamily
            )
            .node_state(),
            StepNodeState::NeedsReview
        );
        let unknown_step = process_attach_step(
            message_catalog,
            ProcessAttachScope::ScopeUnreadable(ProcessAttachReadFailure::ReadingTaskInterrupted),
            DistributionFamily::NixosFamily,
        );
        assert_eq!(unknown_step.node_state(), StepNodeState::NeedsReview);
        assert!(
            unknown_step
                .status_text
                .starts_with("Whether other programs")
        );
    }

    #[test]
    fn the_alert_group_lists_delivery_then_process_attach() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let group_steps = alert_group_steps(
            message_catalog,
            &healthy_evidence(),
            &ready_alerts(),
            DistributionFamily::NixosFamily,
        );
        assert_eq!(
            group_steps
                .iter()
                .map(|alert_step| alert_step.step_title.as_str())
                .collect::<Vec<_>>(),
            vec!["Alerts", "Other programs can't attach to Lazaret"]
        );
        assert!(
            group_steps
                .iter()
                .all(|alert_step| alert_step.node_state() == StepNodeState::Completed)
        );
    }
}
