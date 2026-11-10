use jiff::{SignedDuration, Timestamp};

use crate::{
    host_services::{
        clamav_programs::{
            distribution_install_guides::DistributionFamily,
            signature_provider_catalogue::{
                provider_of_signature_file, signature_file_source_name,
            },
        },
        desktop::locale_formatting::{MomentFormatters, byte_size_text},
        protection_evidence::{
            clamav_engine_evidence::DatabaseEvidence,
            host_protection_evidence::{
                ProtectionEvidence, SignatureEvidence, SignatureLineCount, ThirdPartySignatureFile,
                ThirdPartySignatureListing, UpdaterEvidence, UpdaterUnitResult,
            },
            service_run_journal::UpdaterRunHistory,
        },
    },
    shared_widgets::{
        host_read_failure_text::database_folder_failure_text,
        step_timeline::{
            program_trust_steps::{TrustedProgramGroup, program_trust_step},
            step_evidence_builders::{
                CheckedCondition, evidence_row, plain_rows, remedy_commands, timed_evidence_row,
                unknown_run_text,
            },
            suggested_remedy_commands::SuggestedRemedy,
            timeline_step_contents::{
                ActionConsequence, ActionableEvidenceRow, CancelSlot, EvidenceMoment,
                EvidenceRowControl, OfferedAction, RunningStep, SettledNodeState, SettledStep,
                StepActionButton, StepEvidenceRow, TimelineStep,
            },
        },
    },
    signatures::database_updates_page::signature_update_run::signature_update_session::{
        LastManualUpdate, SignatureUpdateProgress, SignatureUpdateView,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

const DATABASE_FRESH_LIMIT: SignedDuration = SignedDuration::from_hours(4 * 24);
const DATABASE_STALE_LIMIT: SignedDuration = SignedDuration::from_hours(8 * 24);
const THIRD_PARTY_STALE_LIMIT: SignedDuration = SignedDuration::from_hours(48);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DatabaseUpdateAction {
    UpdateSignatures,
    CancelSignatureUpdate,
}

impl StepActionButton for DatabaseUpdateAction {
    fn button_label(&self, message_catalog: &MessageCatalog) -> String {
        match self {
            Self::UpdateSignatures => {
                translated_messages::signatures::update_signatures(message_catalog)
            }
            Self::CancelSignatureUpdate => {
                translated_messages::signatures::cancel_signature_update(message_catalog)
            }
        }
    }

    fn action_consequence(&self) -> ActionConsequence {
        match self {
            Self::UpdateSignatures | Self::CancelSignatureUpdate => {
                ActionConsequence::OrdinaryAction
            }
        }
    }
}

pub type DatabaseUpdateStep = TimelineStep<DatabaseUpdateAction>;

fn database_condition(
    database_evidence: &DatabaseEvidence,
    reference_moment: Timestamp,
) -> CheckedCondition {
    match database_evidence {
        DatabaseEvidence::BuiltAt { built_at } if *built_at > reference_moment => {
            CheckedCondition::ReviewSuggested
        }
        DatabaseEvidence::BuiltAt { built_at }
            if reference_moment.duration_since(*built_at) < DATABASE_FRESH_LIMIT =>
        {
            CheckedCondition::ConditionMet
        }
        DatabaseEvidence::BuiltAt { built_at }
            if reference_moment.duration_since(*built_at) < DATABASE_STALE_LIMIT =>
        {
            CheckedCondition::ReviewSuggested
        }
        DatabaseEvidence::BuiltAt { .. } | DatabaseEvidence::DatabaseMissing => {
            CheckedCondition::ConditionFailed
        }
        DatabaseEvidence::AgeUndetermined | DatabaseEvidence::InspectionInterrupted => {
            CheckedCondition::EvidenceUnknown
        }
    }
}

pub fn database_update_condition(
    signature_evidence: &SignatureEvidence,
    reference_moment: Timestamp,
) -> CheckedCondition {
    let database_state =
        database_condition(&signature_evidence.database_evidence, reference_moment);
    if database_state == CheckedCondition::ConditionMet
        && signature_evidence
            .updater_evidence
            .iter()
            .any(updater_failed)
    {
        CheckedCondition::ReviewSuggested
    } else {
        database_state
    }
}

pub fn third_party_source_condition(
    third_party_listing: &ThirdPartySignatureListing,
    reference_moment: Timestamp,
) -> CheckedCondition {
    match third_party_listing {
        ThirdPartySignatureListing::FolderUnreadable(_) => CheckedCondition::EvidenceUnknown,
        ThirdPartySignatureListing::FilesListed(signature_files) => {
            if signature_files.iter().any(|signature_file| {
                reference_moment.duration_since(signature_file.modified_at)
                    > THIRD_PARTY_STALE_LIMIT
            }) {
                CheckedCondition::ReviewSuggested
            } else {
                CheckedCondition::ConditionMet
            }
        }
    }
}

pub fn signature_condition(
    signature_evidence: &SignatureEvidence,
    reference_moment: Timestamp,
) -> CheckedCondition {
    let update_state = database_update_condition(signature_evidence, reference_moment);
    if update_state == CheckedCondition::ConditionMet
        && third_party_source_condition(&signature_evidence.third_party_listing, reference_moment)
            != CheckedCondition::ConditionMet
    {
        CheckedCondition::ReviewSuggested
    } else {
        update_state
    }
}

fn database_status_text(
    message_catalog: &MessageCatalog,
    database_evidence: &DatabaseEvidence,
    moment_formatters: &MomentFormatters,
    reference_moment: Timestamp,
) -> String {
    match database_evidence {
        DatabaseEvidence::BuiltAt { built_at } if *built_at <= reference_moment => {
            translated_messages::signatures::definitions_built_date(
                message_catalog,
                moment_formatters.relative_moment_text(*built_at, reference_moment),
                moment_formatters.calendar_date_text(*built_at),
            )
        }
        DatabaseEvidence::BuiltAt { .. } | DatabaseEvidence::AgeUndetermined => {
            translated_messages::signatures::definitions_exist_not_determined(message_catalog)
        }
        DatabaseEvidence::DatabaseMissing => {
            translated_messages::signatures::no_virus_definitions_found(message_catalog)
        }
        DatabaseEvidence::InspectionInterrupted => {
            translated_messages::signatures::age_definitions_unknown_checking(message_catalog)
        }
    }
}

pub fn updater_failed(updater_evidence: &UpdaterEvidence) -> bool {
    match updater_evidence.last_result {
        UpdaterUnitResult::UpdateSucceeded | UpdaterUnitResult::UpdateAlreadyCurrent => false,
        UpdaterUnitResult::UpdaterError { .. } | UpdaterUnitResult::ServiceFailed { .. } => true,
    }
}

impl From<UpdaterRunHistory> for EvidenceMoment {
    fn from(run_history: UpdaterRunHistory) -> Self {
        match run_history {
            UpdaterRunHistory::CompletedUpdate { finished_at } => Self::PastEvent {
                happened_at: finished_at,
            },
            UpdaterRunHistory::NoCompletedUpdate => Self::NeverHappened,
            UpdaterRunHistory::JournalUnreadable(_) => Self::CurrentFact,
        }
    }
}

fn updater_row(
    message_catalog: &MessageCatalog,
    updater_evidence: &UpdaterEvidence,
    moment_formatters: &MomentFormatters,
    reference_moment: Timestamp,
) -> StepEvidenceRow {
    if updater_evidence.updater_is_running() {
        return StepEvidenceRow {
            evidence_moment: EvidenceMoment::HappeningNow,
            ..evidence_row(
                translated_messages::signatures::updater_updated(
                    message_catalog,
                    updater_evidence.updater_program.program_name(),
                ),
                translated_messages::signatures::updater_running_now(message_catalog),
            )
        };
    }
    let evidence_moment = EvidenceMoment::from(updater_evidence.last_completed_run);
    let evidence_value = if updater_failed(updater_evidence) {
        translated_messages::signatures::last_update_failed(message_catalog)
    } else {
        match updater_evidence.last_completed_run {
            UpdaterRunHistory::JournalUnreadable(read_failure) => {
                unknown_run_text(message_catalog, read_failure)
            }
            UpdaterRunHistory::NoCompletedUpdate => {
                translated_messages::signatures::no_completed_update(message_catalog)
            }
            UpdaterRunHistory::CompletedUpdate { .. } => String::new(),
        }
    };
    timed_evidence_row(
        message_catalog,
        translated_messages::signatures::updater_updated(
            message_catalog,
            updater_evidence.updater_program.program_name(),
        ),
        evidence_moment,
        evidence_value,
        moment_formatters,
        reference_moment,
    )
}

fn third_party_file_row(
    message_catalog: &MessageCatalog,
    signature_file: &ThirdPartySignatureFile,
    moment_formatters: &MomentFormatters,
    reference_moment: Timestamp,
) -> StepEvidenceRow {
    let file_size_text = byte_size_text(signature_file.file_size);
    timed_evidence_row(
        message_catalog,
        translated_messages::signatures::provider_fetched(
            message_catalog,
            signature_file_source_name(
                message_catalog,
                provider_of_signature_file(&signature_file.file_name),
            ),
        ),
        EvidenceMoment::PastEvent {
            happened_at: signature_file.modified_at,
        },
        match signature_file.signature_lines {
            SignatureLineCount::LinesCounted { signature_count } => {
                translated_messages::signatures::count_signature_size_file(
                    message_catalog,
                    signature_count,
                    &file_size_text,
                    &signature_file.file_name,
                )
            }
            SignatureLineCount::FileUnreadable { failure_kind } => {
                translated_messages::signatures::file_unreadable(
                    message_catalog,
                    failure_kind,
                    &file_size_text,
                    &signature_file.file_name,
                )
            }
        },
        moment_formatters,
        reference_moment,
    )
}

fn third_party_rows(
    message_catalog: &MessageCatalog,
    third_party_listing: &ThirdPartySignatureListing,
    moment_formatters: &MomentFormatters,
    reference_moment: Timestamp,
) -> Vec<StepEvidenceRow> {
    match third_party_listing {
        ThirdPartySignatureListing::FilesListed(signature_files) => signature_files
            .iter()
            .map(|signature_file| {
                third_party_file_row(
                    message_catalog,
                    signature_file,
                    moment_formatters,
                    reference_moment,
                )
            })
            .collect(),
        ThirdPartySignatureListing::FolderUnreadable(read_failure) => vec![evidence_row(
            translated_messages::signatures::other_signature_sources(message_catalog),
            translated_messages::signatures::unknown_reason_database(
                message_catalog,
                database_folder_failure_text(message_catalog, *read_failure),
            ),
        )],
    }
}

fn update_progress_of(signature_update: &SignatureUpdateView) -> SignatureUpdateProgress {
    match signature_update {
        SignatureUpdateView::HostStatusLoading => SignatureUpdateProgress::UpdateUnavailable,
        SignatureUpdateView::HostStatusKnown(known_host) => known_host.update_progress.clone(),
    }
}

fn manual_update_row(
    message_catalog: &MessageCatalog,
    signature_update: &SignatureUpdateView,
    moment_formatters: &MomentFormatters,
    reference_moment: Timestamp,
) -> StepEvidenceRow {
    let last_manual_update = match signature_update {
        SignatureUpdateView::HostStatusLoading => &LastManualUpdate::NotRunThisSession,
        SignatureUpdateView::HostStatusKnown(known_host) => &known_host.last_manual_update,
    };
    let (evidence_moment, evidence_value) = match last_manual_update {
        LastManualUpdate::FinishedAt {
            summary_line,
            finished_at,
        } => (
            EvidenceMoment::PastEvent {
                happened_at: *finished_at,
            },
            summary_line.clone(),
        ),
        LastManualUpdate::NotRunThisSession => (
            EvidenceMoment::CurrentFact,
            translated_messages::signatures::fetch_newest_signatures(message_catalog),
        ),
    };
    timed_evidence_row(
        message_catalog,
        translated_messages::signatures::manual_update(message_catalog),
        evidence_moment,
        evidence_value,
        moment_formatters,
        reference_moment,
    )
}

fn update_host_rows(
    message_catalog: &MessageCatalog,
    signature_update: &SignatureUpdateView,
) -> Vec<StepEvidenceRow> {
    match signature_update {
        SignatureUpdateView::HostStatusLoading => Vec::new(),
        SignatureUpdateView::HostStatusKnown(known_host) => vec![
            evidence_row(
                translated_messages::signatures::update_service(message_catalog),
                known_host.update_service_text.clone(),
            ),
            evidence_row(
                translated_messages::signatures::database_folder(message_catalog),
                known_host.database_folder_text.clone(),
            ),
        ],
    }
}

fn updater_output_rows(
    message_catalog: &MessageCatalog,
    update_progress: &SignatureUpdateProgress,
) -> Vec<StepEvidenceRow> {
    match update_progress {
        SignatureUpdateProgress::CancellableUpdateRunning { latest_output_line }
        | SignatureUpdateProgress::UncancellableUpdateRunning { latest_output_line } => {
            vec![StepEvidenceRow {
                evidence_moment: EvidenceMoment::HappeningNow,
                ..evidence_row(
                    translated_messages::signatures::updater_output(message_catalog),
                    latest_output_line.clone(),
                )
            }]
        }
        SignatureUpdateProgress::UpdateCancelling
        | SignatureUpdateProgress::ReadyToUpdate
        | SignatureUpdateProgress::UpdateUnavailable => Vec::new(),
    }
}

fn signature_step_title(message_catalog: &MessageCatalog) -> String {
    translated_messages::signatures::current_label(message_catalog)
}

fn signature_mechanism_details(message_catalog: &MessageCatalog) -> String {
    translated_messages::signatures::signature_mechanism(message_catalog)
}

fn signature_status_rows(
    message_catalog: &MessageCatalog,
    signature_evidence: &SignatureEvidence,
    moment_formatters: &MomentFormatters,
    reference_moment: Timestamp,
) -> Vec<StepEvidenceRow> {
    signature_evidence
        .updater_evidence
        .iter()
        .map(|updater_evidence| {
            updater_row(
                message_catalog,
                updater_evidence,
                moment_formatters,
                reference_moment,
            )
        })
        .chain(third_party_rows(
            message_catalog,
            &signature_evidence.third_party_listing,
            moment_formatters,
            reference_moment,
        ))
        .collect()
}

fn signature_guidance_commands(
    signature_evidence: &SignatureEvidence,
    distribution_family: DistributionFamily,
) -> Vec<String> {
    if signature_evidence.updater_evidence.is_empty() {
        remedy_commands(SuggestedRemedy::FreshclamUpdatesEnable, distribution_family)
    } else {
        Vec::new()
    }
}

fn signature_node_state(
    signature_evidence: &SignatureEvidence,
    reference_moment: Timestamp,
) -> SettledNodeState {
    signature_condition(signature_evidence, reference_moment).node_state()
}

fn system_updater_running(signature_evidence: &SignatureEvidence) -> bool {
    signature_evidence
        .updater_evidence
        .iter()
        .any(UpdaterEvidence::updater_is_running)
}

pub fn signature_update_step(
    message_catalog: &MessageCatalog,
    signature_evidence: &SignatureEvidence,
    signature_update: &SignatureUpdateView,
    moment_formatters: &MomentFormatters,
    reference_moment: Timestamp,
    distribution_family: DistributionFamily,
) -> DatabaseUpdateStep {
    let evidence_state = signature_node_state(signature_evidence, reference_moment);
    let update_progress = update_progress_of(signature_update);
    let manual_row = manual_update_row(
        message_catalog,
        signature_update,
        moment_formatters,
        reference_moment,
    );
    let other_rows = updater_output_rows(message_catalog, &update_progress)
        .into_iter()
        .chain(signature_status_rows(
            message_catalog,
            signature_evidence,
            moment_formatters,
            reference_moment,
        ))
        .chain(update_host_rows(message_catalog, signature_update))
        .collect::<Vec<_>>();
    let running_signature_step = |status_text: String, cancel_slot| {
        TimelineStep::running(
            signature_step_title(message_catalog),
            signature_mechanism_details(message_catalog),
            status_text,
            RunningStep {
                evidence_rows: std::iter::once(manual_row.clone())
                    .chain(other_rows.clone())
                    .collect(),
                cancel_slot,
            },
        )
    };
    let settled_signature_step = |manual_control| {
        TimelineStep::settled(
            signature_step_title(message_catalog),
            signature_mechanism_details(message_catalog),
            database_status_text(
                message_catalog,
                &signature_evidence.database_evidence,
                moment_formatters,
                reference_moment,
            ),
            SettledStep {
                evidence_rows: std::iter::once(ActionableEvidenceRow {
                    shown_evidence: manual_row.clone(),
                    row_control: manual_control,
                })
                .chain(plain_rows(other_rows.clone()))
                .collect(),
                ..SettledStep::without_offers(evidence_state)
            },
        )
    };
    let signature_step = match &update_progress {
        SignatureUpdateProgress::CancellableUpdateRunning { .. } => running_signature_step(
            translated_messages::signatures::updating_signatures(message_catalog),
            CancelSlot::Cancellable {
                cancel_action: DatabaseUpdateAction::CancelSignatureUpdate,
            },
        ),
        SignatureUpdateProgress::UncancellableUpdateRunning { .. } => running_signature_step(
            translated_messages::signatures::updating_signatures(message_catalog),
            CancelSlot::NotCancellable,
        ),
        SignatureUpdateProgress::UpdateCancelling => running_signature_step(
            translated_messages::signatures::update_cancelling(message_catalog),
            CancelSlot::NotCancellable,
        ),
        SignatureUpdateProgress::ReadyToUpdate | SignatureUpdateProgress::UpdateUnavailable
            if system_updater_running(signature_evidence) =>
        {
            running_signature_step(
                translated_messages::signatures::updating_signatures(message_catalog),
                CancelSlot::NotCancellable,
            )
        }
        SignatureUpdateProgress::ReadyToUpdate => settled_signature_step(
            EvidenceRowControl::RowButton(if evidence_state == SettledNodeState::Completed {
                OfferedAction::plain(DatabaseUpdateAction::UpdateSignatures)
            } else {
                OfferedAction::suggested(DatabaseUpdateAction::UpdateSignatures)
            }),
        ),
        SignatureUpdateProgress::UpdateUnavailable => {
            settled_signature_step(EvidenceRowControl::NoRowControl)
        }
    };
    signature_step.with_guidance_commands(signature_guidance_commands(
        signature_evidence,
        distribution_family,
    ))
}

pub fn database_update_steps(
    message_catalog: &MessageCatalog,
    protection_evidence: &ProtectionEvidence,
    signature_update: &SignatureUpdateView,
    moment_formatters: &MomentFormatters,
    reference_moment: Timestamp,
    distribution_family: DistributionFamily,
) -> Vec<DatabaseUpdateStep> {
    program_trust_step(
        message_catalog,
        &protection_evidence.program_trust,
        TrustedProgramGroup::UpdaterPrograms,
    )
    .into_iter()
    .chain([signature_update_step(
        message_catalog,
        &protection_evidence.signature_evidence,
        signature_update,
        moment_formatters,
        reference_moment,
        distribution_family,
    )])
    .collect()
}

#[cfg(test)]
mod database_updates_presentation_tests {
    use super::*;
    use crate::{
        host_services::{
            clamav_programs::database_directory_status::DatabaseFolderReadFailure,
            protection_evidence::{
                host_protection_evidence::{ProgramTrustFinding, ProgramTrustInspection},
                protection_evidence_fixtures::{
                    REFERENCE_MOMENT, english_formatters, healthy_evidence,
                },
            },
            systemd_bus::host_systemd_units::{ServiceRunState, UnitTimestamp},
        },
        shared_widgets::step_timeline::timeline_step_contents::{StepNodeState, StepPhase},
        signatures::database_updates_page::signature_update_run::{
            signature_update_session::KnownUpdateHost, updater_exit_status::SignatureUpdaterProgram,
        },
    };

    fn known_update_host(update_progress: SignatureUpdateProgress) -> SignatureUpdateView {
        SignatureUpdateView::HostStatusKnown(KnownUpdateHost {
            update_progress,
            last_manual_update: LastManualUpdate::NotRunThisSession,
            update_service_text: "Active".to_owned(),
            database_folder_text: "/var/lib/clamav".to_owned(),
        })
    }

    fn update_step_with(
        message_catalog: &MessageCatalog,
        signature_evidence: &SignatureEvidence,
        update_progress: SignatureUpdateProgress,
    ) -> DatabaseUpdateStep {
        signature_update_step(
            message_catalog,
            signature_evidence,
            &known_update_host(update_progress),
            &english_formatters(),
            REFERENCE_MOMENT,
            DistributionFamily::NixosFamily,
        )
    }

    fn ready_step_with(
        message_catalog: &MessageCatalog,
        signature_evidence: &SignatureEvidence,
    ) -> DatabaseUpdateStep {
        update_step_with(
            message_catalog,
            signature_evidence,
            SignatureUpdateProgress::ReadyToUpdate,
        )
    }

    fn row_named<'step>(
        timeline_step: &'step DatabaseUpdateStep,
        evidence_name: &str,
    ) -> &'step StepEvidenceRow {
        timeline_step
            .shown_evidence_rows()
            .into_iter()
            .find(|step_evidence| step_evidence.evidence_name == evidence_name)
            .unwrap()
    }

    fn manual_row_control(
        timeline_step: &DatabaseUpdateStep,
    ) -> &EvidenceRowControl<DatabaseUpdateAction> {
        let StepPhase::StepSettled(settled_step) = &timeline_step.step_phase else {
            panic!("{} is running", timeline_step.step_title);
        };
        &settled_step
            .evidence_rows
            .iter()
            .find(|actionable_row| actionable_row.shown_evidence.evidence_name == "Manual update")
            .unwrap()
            .row_control
    }

    fn step_titles(update_steps: &[DatabaseUpdateStep]) -> Vec<&str> {
        update_steps
            .iter()
            .map(|update_step| update_step.step_title.as_str())
            .collect()
    }

    fn update_steps_with_trust(
        message_catalog: &MessageCatalog,
        program_trust: ProgramTrustInspection,
    ) -> Vec<DatabaseUpdateStep> {
        database_update_steps(
            message_catalog,
            &ProtectionEvidence {
                program_trust,
                ..healthy_evidence()
            },
            &known_update_host(SignatureUpdateProgress::ReadyToUpdate),
            &english_formatters(),
            REFERENCE_MOMENT,
            DistributionFamily::NixosFamily,
        )
    }

    #[test]
    fn a_ready_update_offers_update_now_on_the_manual_update_row() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let signature_step =
            ready_step_with(message_catalog, &healthy_evidence().signature_evidence);
        assert_eq!(signature_step.node_state(), StepNodeState::Completed);
        assert_eq!(signature_step.header_offer(), None);
        assert_eq!(
            manual_row_control(&signature_step),
            &EvidenceRowControl::RowButton(OfferedAction::plain(
                DatabaseUpdateAction::UpdateSignatures
            ))
        );
        assert!(
            signature_step
                .status_text
                .starts_with("Definitions built 5 hours ago (")
        );
        assert_eq!(
            row_named(&signature_step, "Update service").evidence_value,
            "Active"
        );
    }

    #[test]
    fn a_running_update_shows_its_progress_and_can_be_cancelled() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let signature_step = update_step_with(
            message_catalog,
            &healthy_evidence().signature_evidence,
            SignatureUpdateProgress::CancellableUpdateRunning {
                latest_output_line: "daily.cld updated".to_owned(),
            },
        );
        assert_eq!(signature_step.node_state(), StepNodeState::InProgress);
        let output_row = row_named(&signature_step, "Updater output");
        assert_eq!(output_row.evidence_moment, EvidenceMoment::HappeningNow);
        assert_eq!(output_row.evidence_value, "daily.cld updated");
        assert_eq!(
            signature_step.offered_actions(),
            vec![OfferedAction::plain(
                DatabaseUpdateAction::CancelSignatureUpdate
            )]
        );
    }

    #[test]
    fn an_update_the_system_timer_started_shows_in_progress_without_update_now() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let running_updater = SignatureEvidence {
            updater_evidence: vec![UpdaterEvidence {
                updater_program: SignatureUpdaterProgram::Freshclam,
                service_unit: "clamav-freshclam.service",
                last_result: UpdaterUnitResult::UpdateSucceeded,
                last_completed_run: UpdaterRunHistory::NoCompletedUpdate,
                run_state: ServiceRunState::ServiceRunning {
                    started_at: UnitTimestamp::NeverRecorded,
                },
            }],
            ..healthy_evidence().signature_evidence
        };
        let signature_step = ready_step_with(message_catalog, &running_updater);
        assert_eq!(signature_step.node_state(), StepNodeState::InProgress);
        assert_eq!(signature_step.status_text, "Updating signatures…");
        assert!(signature_step.offered_actions().is_empty());
        let updater_row = row_named(&signature_step, "Updated by freshclam");
        assert_eq!(updater_row.evidence_moment, EvidenceMoment::HappeningNow);
    }

    #[test]
    fn a_failed_updater_or_stale_source_warns_even_with_fresh_definitions() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let failed_updater = SignatureEvidence {
            updater_evidence: vec![UpdaterEvidence {
                updater_program: SignatureUpdaterProgram::Fangfrisch,
                service_unit: "clamav-fangfrisch.service",
                last_result: UpdaterUnitResult::UpdaterError { exit_status: 1 },
                last_completed_run: UpdaterRunHistory::NoCompletedUpdate,
                run_state: ServiceRunState::ServiceIdle,
            }],
            ..healthy_evidence().signature_evidence
        };
        let signature_step = ready_step_with(message_catalog, &failed_updater);
        assert_eq!(signature_step.node_state(), StepNodeState::NeedsReview);
        assert_eq!(
            manual_row_control(&signature_step),
            &EvidenceRowControl::RowButton(OfferedAction::suggested(
                DatabaseUpdateAction::UpdateSignatures
            ))
        );
    }

    #[test]
    fn an_unreadable_signature_folder_is_unknown_rather_than_no_other_sources() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let signature_step = ready_step_with(
            message_catalog,
            &SignatureEvidence {
                third_party_listing: ThirdPartySignatureListing::FolderUnreadable(
                    DatabaseFolderReadFailure::ReadingTaskInterrupted,
                ),
                ..healthy_evidence().signature_evidence
            },
        );
        assert_eq!(signature_step.node_state(), StepNodeState::NeedsReview);
        assert_eq!(
            row_named(&signature_step, "Other signature sources").evidence_value,
            "Unknown. Reading the signature folder stopped before it finished"
        );
    }

    #[test]
    fn an_interrupted_database_check_is_unknown_rather_than_missing() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let signature_step = ready_step_with(
            message_catalog,
            &SignatureEvidence {
                database_evidence: DatabaseEvidence::InspectionInterrupted,
                ..healthy_evidence().signature_evidence
            },
        );
        assert_eq!(signature_step.node_state(), StepNodeState::NeedsReview);
        assert!(signature_step.status_text.contains("unknown"));
        assert!(!signature_step.status_text.contains("No virus definitions"));
    }

    #[test]
    fn only_a_replaceable_freshclam_adds_a_trust_step_before_the_signatures() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let refused_freshclam = update_steps_with_trust(
            message_catalog,
            ProgramTrustInspection::ProgramsChecked(vec![
                ProgramTrustFinding::OnlyReplaceableProgram {
                    program_name: "freshclam",
                    replaceable_paths: vec!["/home/user/.local/bin/freshclam".to_owned()],
                },
            ]),
        );
        assert_eq!(
            step_titles(&refused_freshclam),
            vec!["freshclam can be trusted", "Signatures current"]
        );
        assert_eq!(refused_freshclam[0].node_state(), StepNodeState::HasFailed);
        let refused_scanner = update_steps_with_trust(
            message_catalog,
            ProgramTrustInspection::ProgramsChecked(vec![
                ProgramTrustFinding::OnlyReplaceableProgram {
                    program_name: "clamdscan",
                    replaceable_paths: vec!["/home/user/.local/bin/clamdscan".to_owned()],
                },
            ]),
        );
        assert_eq!(step_titles(&refused_scanner), vec!["Signatures current"]);
    }
}
