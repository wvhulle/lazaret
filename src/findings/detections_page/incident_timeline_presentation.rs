use std::collections::BTreeSet;

use crate::{
    findings::detections_page::detected_file_state::{DetectedFileState, FileAccess, FileOwner},
    host_services::{
        clamav_programs::{
            distribution_install_guides::DistributionFamily,
            signature_provider_catalogue::{provider_of_signature, signature_source_name},
        },
        clamav_scanner::scanner_output_parser::sanitize_scanner_line,
        desktop::locale_formatting::exact_local_time_text,
        detection_journal::detection_history_store::{
            DetectionSources, DetectionState, ResolutionKind, StoredDetection,
        },
        threat_intel::{
            file_reputation_check::malware_family_text,
            malware_bazaar_client::{KnownSampleLookup, MalwareBazaarFailure, SampleFirstSeen},
            service_key_storage::{KeyStorageFailure, describe_key_failure},
        },
    },
    shared_widgets::step_timeline::timeline_step_contents::{
        ActionConsequence, ActionableEvidenceRow, SettledNodeState, SettledStep, StepActionButton,
        StepChoiceRow, StepEvidenceRow, TimelineStep,
    },
    translated_messages,
};

const TEST_SIGNATURE_MARKER: &str = "Eicar-Test-Signature";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IncidentStepAction {
    QuarantineIncident,
    AskIncidentDeletion,
    DeleteIncidentFile,
    CancelIncidentDeletion,
    DismissIncident,
    LookUpIncidentSignature,
    OpenOnlineServices,
    SubmitIncidentSample,
}

impl StepActionButton for IncidentStepAction {
    fn button_label(&self) -> String {
        match self {
            Self::QuarantineIncident => translated_messages::findings::quarantine_incident(),
            Self::AskIncidentDeletion => translated_messages::findings::ask_incident_deletion(),
            Self::DeleteIncidentFile => translated_messages::findings::delete_incident_file(),
            Self::CancelIncidentDeletion => {
                translated_messages::findings::cancel_incident_deletion()
            }
            Self::DismissIncident => translated_messages::findings::dismiss_incident(),
            Self::LookUpIncidentSignature => {
                translated_messages::findings::look_up_incident_signature()
            }
            Self::OpenOnlineServices => translated_messages::findings::open_online_services(),
            Self::SubmitIncidentSample => translated_messages::findings::submit_incident_sample(),
        }
    }

    fn action_consequence(&self) -> ActionConsequence {
        match self {
            Self::DeleteIncidentFile => ActionConsequence::DestructiveAction,
            Self::QuarantineIncident
            | Self::AskIncidentDeletion
            | Self::CancelIncidentDeletion
            | Self::DismissIncident
            | Self::LookUpIncidentSignature
            | Self::OpenOnlineServices
            | Self::SubmitIncidentSample => ActionConsequence::OrdinaryAction,
        }
    }
}

pub type IncidentStep = TimelineStep<IncidentStepAction>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplayedDetection {
    pub stored_detection: StoredDetection,
    pub file_state: DetectedFileState,
}

#[derive(Debug)]
pub enum SignatureLookupState {
    NotLookedUp,
    LookingUp,
    KeyMissing,
    KeyUnreadable(KeyStorageFailure),
    LookupFinished(KnownSampleLookup),
    BazaarLookupFailed(MalwareBazaarFailure),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeletionPrompt {
    NotAsked,
    AwaitingAnswer,
}

pub fn signature_is_test_pattern(signature_name: &str) -> bool {
    signature_name.contains(TEST_SIGNATURE_MARKER)
}

pub fn detection_needs_attention(displayed_detection: &DisplayedDetection) -> bool {
    displayed_detection
        .stored_detection
        .detection_state
        .awaits_user_decision()
        && displayed_detection.file_state != DetectedFileState::NoLongerThere
}

pub fn displayed_file_path(stored_detection: &StoredDetection) -> String {
    sanitize_scanner_line(&stored_detection.file_path.to_string_lossy())
}

pub fn displayed_file_name(stored_detection: &StoredDetection) -> String {
    stored_detection.file_path.file_name().map_or_else(
        || displayed_file_path(stored_detection),
        |file_name| sanitize_scanner_line(&file_name.to_string_lossy()),
    )
}

pub fn detection_state_text(displayed_detection: &DisplayedDetection) -> String {
    let stored_detection = &displayed_detection.stored_detection;
    match &stored_detection.detection_state {
        DetectionState::Resolved {
            resolution_kind: ResolutionKind::MovedToQuarantine,
            ..
        } if displayed_detection.file_state.file_is_present() => {
            translated_messages::findings::restored_quarantine()
        }
        DetectionState::Resolved {
            resolution_kind: ResolutionKind::MovedToQuarantine,
            ..
        } => translated_messages::findings::moved_quarantine(),
        DetectionState::Resolved {
            resolution_kind: ResolutionKind::DeletedByUser,
            ..
        } => translated_messages::findings::deleted_label(),
        DetectionState::Resolved {
            resolution_kind: ResolutionKind::DismissedByUser,
            ..
        } => translated_messages::findings::kept_label(),
        DetectionState::Open => match displayed_detection.file_state {
            DetectedFileState::Present {
                owner: FileOwner::OtherUser,
                ..
            } => translated_messages::findings::belongs_another_user(),
            DetectedFileState::Present {
                access: FileAccess::BlockedOnAccess,
                owner: FileOwner::CurrentUser,
            } => translated_messages::findings::blocked_needs_decision(),
            DetectedFileState::Present {
                access: FileAccess::Readable,
                owner: FileOwner::CurrentUser,
            } => translated_messages::findings::not_blocked_needs_decision(),
            DetectedFileState::Present {
                access: FileAccess::NotOpened,
                owner: FileOwner::CurrentUser,
            } => translated_messages::findings::needs_decision(),
            DetectedFileState::NoLongerThere => {
                translated_messages::findings::no_longer_there_detection()
            }
            DetectedFileState::StateUnknown => {
                translated_messages::findings::state_unknown_detection()
            }
        },
    }
}

pub fn detection_summary_text(displayed_detection: &DisplayedDetection) -> String {
    let stored_detection = &displayed_detection.stored_detection;
    let signature_text = sanitize_scanner_line(&stored_detection.signature_name);
    let state_text = detection_state_text(displayed_detection);
    if signature_is_test_pattern(&stored_detection.signature_name) {
        translated_messages::findings::signature_test_file_state(&signature_text, &state_text)
    } else {
        translated_messages::findings::signature_state(&signature_text, &state_text)
    }
}

fn evidence_row(
    evidence_name: String,
    evidence_value: String,
) -> ActionableEvidenceRow<IncidentStepAction> {
    StepEvidenceRow::current_fact(evidence_name, evidence_value).into()
}

fn plain_step(
    step_title: String,
    mechanism_details: String,
    node_state: SettledNodeState,
    status_text: String,
) -> IncidentStep {
    TimelineStep::settled(
        step_title,
        mechanism_details,
        status_text,
        SettledStep::without_offers(node_state),
    )
}

fn source_text(stored_detection: &StoredDetection) -> String {
    match stored_detection.detection_sources {
        DetectionSources::OnAccessAndSchedule => translated_messages::findings::access_schedule(),
        DetectionSources::OnAccessOnly => translated_messages::findings::access_only(),
        DetectionSources::ScheduleOnly => translated_messages::findings::schedule_only(),
    }
}

fn detected_step(stored_detection: &StoredDetection) -> IncidentStep {
    let signature_text = sanitize_scanner_line(&stored_detection.signature_name);
    let evidence_rows = vec![
        evidence_row(
            translated_messages::findings::file_label(),
            displayed_file_path(stored_detection),
        ),
        evidence_row(
            translated_messages::findings::signature_source(),
            signature_source_name(provider_of_signature(&stored_detection.signature_name)),
        ),
        evidence_row(
            translated_messages::findings::found_label(),
            source_text(stored_detection),
        ),
        evidence_row(
            translated_messages::findings::last_seen(),
            exact_local_time_text(stored_detection.last_seen),
        ),
        evidence_row(
            translated_messages::findings::times_seen(),
            stored_detection.sighting_count.to_string(),
        ),
    ];
    TimelineStep::settled(
        translated_messages::findings::detected_label(),
        if signature_is_test_pattern(&stored_detection.signature_name) {
            translated_messages::findings::signature_matches_harmless_eicar()
        } else {
            translated_messages::findings::clamav_matched_file_against()
        },
        translated_messages::findings::signature_first_seen_time(
            &signature_text,
            exact_local_time_text(stored_detection.first_seen),
        ),
        SettledStep {
            evidence_rows,
            ..SettledStep::without_offers(SettledNodeState::Completed)
        },
    )
}

fn blocked_step(displayed_detection: &DisplayedDetection) -> IncidentStep {
    let stored_detection = &displayed_detection.stored_detection;
    let (node_state, status_text) = match displayed_detection.file_state {
        DetectedFileState::Present {
            access: FileAccess::BlockedOnAccess,
            ..
        } => (
            SettledNodeState::Completed,
            translated_messages::findings::opening_file_refused_why(),
        ),
        DetectedFileState::Present {
            access: FileAccess::NotOpened,
            ..
        } => (
            SettledNodeState::Completed,
            translated_messages::findings::not_opened_again_after(),
        ),
        DetectedFileState::NoLongerThere => (
            SettledNodeState::Completed,
            translated_messages::findings::no_longer_there_blocked(),
        ),
        DetectedFileState::Present {
            access: FileAccess::Readable,
            ..
        } if stored_detection.detection_sources == DetectionSources::ScheduleOnly => (
            SettledNodeState::NeedsReview,
            translated_messages::findings::not_blocked_scheduled_scan(),
        ),
        DetectedFileState::Present {
            access: FileAccess::Readable,
            ..
        } => (
            SettledNodeState::NeedsReview,
            translated_messages::findings::not_blocked_right_now(),
        ),
        DetectedFileState::StateUnknown => (
            SettledNodeState::NeedsReview,
            translated_messages::findings::state_unknown_blocked(),
        ),
    };
    plain_step(
        translated_messages::findings::blocked_label(),
        translated_messages::findings::access_prevention_kernel_refuses(),
        node_state,
        status_text,
    )
}

fn false_positive_snippet(
    stored_detection: &StoredDetection,
    distribution_family: DistributionFamily,
) -> String {
    let shown_path = displayed_file_path(stored_detection);
    if distribution_family == DistributionFamily::NixosFamily {
        format!(
            "services.clamav.daemon.settings.OnAccessExcludePath = [ \"{}\" ];",
            shown_path.replace('\\', "\\\\").replace('"', "\\\"")
        )
    } else {
        format!("OnAccessExcludePath {shown_path}")
    }
}

fn describe_bazaar_lookup_failure(bazaar_failure: &MalwareBazaarFailure) -> String {
    match bazaar_failure {
        MalwareBazaarFailure::RequestFailed(request_failure) => {
            translated_messages::findings::request_failed(request_failure)
        }
        MalwareBazaarFailure::AnswerUnreadable => {
            translated_messages::findings::answer_unreadable()
        }
        MalwareBazaarFailure::KeyRefused => translated_messages::findings::key_refused(),
        MalwareBazaarFailure::QueryRefused { query_status } => {
            translated_messages::findings::query_refused(query_status)
        }
        MalwareBazaarFailure::RequestLimitReached => {
            translated_messages::findings::request_limit_reached()
        }
        MalwareBazaarFailure::UnexpectedStatus { status_code } => {
            translated_messages::findings::unexpected_status(status_code)
        }
    }
}

fn lookup_summary_text(lookup_state: &SignatureLookupState) -> Option<String> {
    match lookup_state {
        SignatureLookupState::NotLookedUp => None,
        SignatureLookupState::LookingUp => Some(translated_messages::findings::looking_up()),
        SignatureLookupState::KeyMissing => {
            Some(translated_messages::findings::key_missing_lookup())
        }
        SignatureLookupState::KeyUnreadable(key_failure) => {
            Some(sanitize_scanner_line(&describe_key_failure(key_failure)))
        }
        SignatureLookupState::BazaarLookupFailed(bazaar_failure) => Some(sanitize_scanner_line(
            &describe_bazaar_lookup_failure(bazaar_failure),
        )),
        SignatureLookupState::LookupFinished(KnownSampleLookup::NoKnownSamples) => {
            Some(translated_messages::findings::no_known_samples_match())
        }
        SignatureLookupState::LookupFinished(KnownSampleLookup::SamplesFound { known_samples }) => {
            let families = known_samples
                .iter()
                .map(|known_sample| &known_sample.malware_family)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .map(malware_family_text)
                .collect::<Vec<_>>()
                .join(", ");
            let earliest_seen = known_samples
                .iter()
                .filter_map(|known_sample| match &known_sample.first_seen {
                    SampleFirstSeen::SeenAt(seen_at) => Some(seen_at.as_str()),
                    SampleFirstSeen::FirstSeenUnknown => None,
                })
                .min()
                .map_or_else(
                    translated_messages::findings::unknown_label,
                    sanitize_scanner_line,
                );
            Some(translated_messages::findings::count_known_samples_families(
                known_samples.len(),
                &families,
                &earliest_seen,
            ))
        }
    }
}

fn lookup_choices(lookup_state: &SignatureLookupState) -> Vec<StepChoiceRow<IncidentStepAction>> {
    let signature_lookup = StepChoiceRow {
        choice_description: translated_messages::findings::ask_malwarebazaar_signature_description(
        ),
        chosen_action: IncidentStepAction::LookUpIncidentSignature,
    };
    match lookup_state {
        SignatureLookupState::KeyMissing => vec![
            signature_lookup,
            StepChoiceRow {
                choice_description: translated_messages::findings::key_missing_choices(),
                chosen_action: IncidentStepAction::OpenOnlineServices,
            },
        ],
        SignatureLookupState::NotLookedUp
        | SignatureLookupState::LookingUp
        | SignatureLookupState::KeyUnreadable(_)
        | SignatureLookupState::LookupFinished(_)
        | SignatureLookupState::BazaarLookupFailed(_) => vec![signature_lookup],
    }
}

fn decision_choices(
    deletion_prompt: DeletionPrompt,
    file_state: DetectedFileState,
) -> Vec<StepChoiceRow<IncidentStepAction>> {
    if !file_state.user_can_move_or_delete() {
        return vec![StepChoiceRow {
            choice_description: translated_messages::findings::stop_asking_file_description(),
            chosen_action: IncidentStepAction::DismissIncident,
        }];
    }
    match deletion_prompt {
        DeletionPrompt::AwaitingAnswer => vec![
            StepChoiceRow {
                choice_description:
                    translated_messages::findings::delete_file_permanently_description(),
                chosen_action: IncidentStepAction::DeleteIncidentFile,
            },
            StepChoiceRow {
                choice_description: translated_messages::findings::go_back_without_description(),
                chosen_action: IncidentStepAction::CancelIncidentDeletion,
            },
        ],
        DeletionPrompt::NotAsked => vec![
            StepChoiceRow {
                choice_description: translated_messages::findings::move_somewhere_safe_description(
                ),
                chosen_action: IncidentStepAction::QuarantineIncident,
            },
            StepChoiceRow {
                choice_description: translated_messages::findings::remove_good_description(),
                chosen_action: IncidentStepAction::AskIncidentDeletion,
            },
            StepChoiceRow {
                choice_description: translated_messages::findings::know_file_fine_description(),
                chosen_action: IncidentStepAction::DismissIncident,
            },
        ],
    }
}

fn review_step(
    displayed_detection: &DisplayedDetection,
    deletion_prompt: DeletionPrompt,
    lookup_state: &SignatureLookupState,
    distribution_family: DistributionFamily,
) -> IncidentStep {
    let stored_detection = &displayed_detection.stored_detection;
    let step_title = translated_messages::findings::decision_title();
    let mechanism_details = translated_messages::findings::unfamiliar_quarantine_keeping_file();
    if !detection_needs_attention(displayed_detection) {
        return plain_step(
            step_title,
            translated_messages::findings::quarantined_files_restored_quarantine(),
            SettledNodeState::Completed,
            detection_state_text(displayed_detection),
        );
    }
    let status_text = match displayed_detection.file_state {
        DetectedFileState::Present {
            owner: FileOwner::OtherUser,
            ..
        } => translated_messages::findings::file_other_owner_decision(),
        DetectedFileState::StateUnknown => translated_messages::findings::state_unknown_review(),
        DetectedFileState::Present {
            owner: FileOwner::CurrentUser,
            ..
        }
        | DetectedFileState::NoLongerThere => translated_messages::findings::choose_happens_file(),
    };
    TimelineStep::settled(
        step_title,
        mechanism_details,
        status_text,
        SettledStep {
            choice_rows: decision_choices(deletion_prompt, displayed_detection.file_state)
                .into_iter()
                .chain(lookup_choices(lookup_state))
                .collect(),
            evidence_rows: lookup_summary_text(lookup_state)
                .map(|lookup_summary| {
                    evidence_row(
                        translated_messages::findings::malwarebazaar_label(),
                        lookup_summary,
                    )
                })
                .into_iter()
                .collect(),
            ..SettledStep::without_offers(SettledNodeState::ActNext)
        },
    )
    .with_guidance_commands(vec![false_positive_snippet(
        stored_detection,
        distribution_family,
    )])
}

fn resolved_step(displayed_detection: &DisplayedDetection) -> IncidentStep {
    let stored_detection = &displayed_detection.stored_detection;
    let step_title = translated_messages::findings::resolved_title();
    let mechanism_details = translated_messages::findings::resolved_detection_opens_again();
    if detection_needs_attention(displayed_detection) {
        return plain_step(
            step_title,
            mechanism_details,
            SettledNodeState::AwaitingPrerequisite,
            translated_messages::findings::waiting_decision(),
        );
    }
    let status_text = match &stored_detection.detection_state {
        DetectionState::Resolved { resolved_at, .. } => {
            translated_messages::findings::resolved_state_step(
                detection_state_text(displayed_detection),
                exact_local_time_text(*resolved_at),
            )
        }
        DetectionState::Open => detection_state_text(displayed_detection),
    };
    let quarantined_copy_kept = matches!(
        stored_detection.detection_state,
        DetectionState::Resolved {
            resolution_kind: ResolutionKind::MovedToQuarantine,
            ..
        }
    ) && !displayed_detection.file_state.file_is_present();
    if !quarantined_copy_kept {
        return plain_step(
            step_title,
            mechanism_details,
            SettledNodeState::Completed,
            status_text,
        );
    }
    TimelineStep::settled(
        step_title,
        mechanism_details,
        status_text,
        SettledStep {
            choice_rows: vec![StepChoiceRow {
                choice_description:
                    translated_messages::findings::send_quarantined_copy_description(),
                chosen_action: IncidentStepAction::SubmitIncidentSample,
            }],
            ..SettledStep::without_offers(SettledNodeState::Completed)
        },
    )
}

pub fn incident_timeline(
    displayed_detection: &DisplayedDetection,
    deletion_prompt: DeletionPrompt,
    lookup_state: &SignatureLookupState,
    distribution_family: DistributionFamily,
) -> Vec<IncidentStep> {
    vec![
        detected_step(&displayed_detection.stored_detection),
        blocked_step(displayed_detection),
        review_step(
            displayed_detection,
            deletion_prompt,
            lookup_state,
            distribution_family,
        ),
        resolved_step(displayed_detection),
    ]
}

#[cfg(test)]
mod incident_timeline_presentation_tests {
    use std::path::PathBuf;

    use super::*;
    use crate::{
        host_services::threat_intel::malware_bazaar_client::MalwareFamily,
        shared_widgets::step_timeline::timeline_step_contents::{
            ButtonStyle, StepNodeState, StepPhase,
        },
    };

    const BLOCKED_POSITION: usize = 1;
    const REVIEW_POSITION: usize = 2;
    const RESOLVED_POSITION: usize = 3;

    fn blocked_detection() -> DisplayedDetection {
        DisplayedDetection {
            stored_detection: StoredDetection {
                detection_id: 1,
                file_path: PathBuf::from("/home/user/Downloads/invoice.exe"),
                signature_name: "Win.Trojan.Agent-1".to_owned(),
                detection_sources: DetectionSources::OnAccessOnly,
                first_seen: jiff::Timestamp::constant(1_793_869_222, 0),
                last_seen: jiff::Timestamp::constant(1_793_869_225, 0),
                sighting_count: 6,
                detection_state: DetectionState::Open,
            },
            file_state: DetectedFileState::Present {
                access: FileAccess::BlockedOnAccess,
                owner: FileOwner::CurrentUser,
            },
        }
    }

    fn review_choices(
        displayed_detection: &DisplayedDetection,
        deletion_prompt: DeletionPrompt,
    ) -> Vec<StepChoiceRow<IncidentStepAction>> {
        let review_step = incident_timeline(
            displayed_detection,
            deletion_prompt,
            &SignatureLookupState::NotLookedUp,
            DistributionFamily::NixosFamily,
        )
        .remove(REVIEW_POSITION);
        match review_step.step_phase {
            StepPhase::StepSettled(settled_step) => settled_step.choice_rows,
            StepPhase::StepRunning(_) => panic!("the review step never runs"),
        }
    }

    fn choice_actions(
        displayed_detection: &DisplayedDetection,
        deletion_prompt: DeletionPrompt,
    ) -> Vec<IncidentStepAction> {
        review_choices(displayed_detection, deletion_prompt)
            .into_iter()
            .map(|choice_row| choice_row.chosen_action)
            .filter(|chosen_action| {
                !matches!(chosen_action, IncidentStepAction::LookUpIncidentSignature)
            })
            .collect()
    }

    #[test]
    fn permanent_deletion_is_always_shown_as_destructive() {
        let deletion_choices = review_choices(&blocked_detection(), DeletionPrompt::AwaitingAnswer);
        let choice_styles = deletion_choices
            .iter()
            .map(|choice_row| choice_row.offered_choice().button_style())
            .collect::<Vec<_>>();
        assert_eq!(
            choice_styles,
            vec![
                ButtonStyle::DestructiveButton,
                ButtonStyle::PlainButton,
                ButtonStyle::PlainButton
            ]
        );
        assert_eq!(
            deletion_choices[0].chosen_action.button_label(),
            "Delete Permanently"
        );
    }

    #[test]
    fn a_blocked_file_explains_itself_and_asks_for_a_decision() {
        let incident_steps = incident_timeline(
            &blocked_detection(),
            DeletionPrompt::NotAsked,
            &SignatureLookupState::NotLookedUp,
            DistributionFamily::NixosFamily,
        );
        assert_eq!(
            incident_steps[BLOCKED_POSITION].node_state(),
            StepNodeState::Completed
        );
        assert!(
            incident_steps[BLOCKED_POSITION]
                .status_text
                .contains("won't open")
        );
        assert_eq!(
            incident_steps[REVIEW_POSITION].node_state(),
            StepNodeState::ActNext
        );
        assert_eq!(
            incident_steps[RESOLVED_POSITION].node_state(),
            StepNodeState::AwaitingPrerequisite
        );
    }

    #[test]
    fn deleting_takes_two_explicit_choices() {
        let first_choices = choice_actions(&blocked_detection(), DeletionPrompt::NotAsked);
        assert!(first_choices.contains(&IncidentStepAction::AskIncidentDeletion));
        assert!(!first_choices.contains(&IncidentStepAction::DeleteIncidentFile));
        assert_eq!(
            choice_actions(&blocked_detection(), DeletionPrompt::AwaitingAnswer),
            vec![
                IncidentStepAction::DeleteIncidentFile,
                IncidentStepAction::CancelIncidentDeletion
            ]
        );
    }

    #[test]
    fn a_file_found_only_by_the_schedule_is_not_reported_as_blocked() {
        let displayed_detection = DisplayedDetection {
            stored_detection: StoredDetection {
                detection_sources: DetectionSources::ScheduleOnly,
                ..blocked_detection().stored_detection
            },
            file_state: DetectedFileState::Present {
                access: FileAccess::Readable,
                owner: FileOwner::CurrentUser,
            },
        };
        let incident_steps = incident_timeline(
            &displayed_detection,
            DeletionPrompt::NotAsked,
            &SignatureLookupState::NotLookedUp,
            DistributionFamily::NixosFamily,
        );
        assert_eq!(
            incident_steps[BLOCKED_POSITION].node_state(),
            StepNodeState::NeedsReview
        );
        assert!(
            incident_steps[BLOCKED_POSITION]
                .status_text
                .contains("Not blocked")
        );
    }

    #[test]
    fn a_foreign_file_offers_no_move_or_delete_even_when_blocked() {
        [FileAccess::BlockedOnAccess, FileAccess::Readable]
            .into_iter()
            .for_each(|file_access| {
                let displayed_detection = DisplayedDetection {
                    file_state: DetectedFileState::Present {
                        access: file_access,
                        owner: FileOwner::OtherUser,
                    },
                    ..blocked_detection()
                };
                assert_eq!(
                    choice_actions(&displayed_detection, DeletionPrompt::NotAsked),
                    vec![IncidentStepAction::DismissIncident]
                );
            });
    }

    #[test]
    fn an_unresolved_file_that_could_not_be_checked_still_needs_attention() {
        let displayed_detection = DisplayedDetection {
            file_state: DetectedFileState::StateUnknown,
            ..blocked_detection()
        };
        assert!(detection_needs_attention(&displayed_detection));
        assert_eq!(
            choice_actions(&displayed_detection, DeletionPrompt::NotAsked),
            vec![IncidentStepAction::DismissIncident]
        );
    }

    #[test]
    fn a_vanished_or_resolved_file_needs_no_attention() {
        assert!(detection_needs_attention(&blocked_detection()));
        assert!(!detection_needs_attention(&DisplayedDetection {
            file_state: DetectedFileState::NoLongerThere,
            ..blocked_detection()
        }));
        assert!(!detection_needs_attention(&DisplayedDetection {
            stored_detection: StoredDetection {
                detection_state: DetectionState::Resolved {
                    resolution_kind: ResolutionKind::DismissedByUser,
                    resolved_at: jiff::Timestamp::constant(1_793_869_300, 0),
                },
                ..blocked_detection().stored_detection
            },
            ..blocked_detection()
        }));
    }

    #[test]
    fn a_readable_file_found_on_access_is_not_blamed_on_the_schedule() {
        let displayed_detection = DisplayedDetection {
            stored_detection: StoredDetection {
                detection_sources: DetectionSources::OnAccessOnly,
                ..blocked_detection().stored_detection
            },
            file_state: DetectedFileState::Present {
                access: FileAccess::Readable,
                owner: FileOwner::CurrentUser,
            },
        };
        let incident_steps = incident_timeline(
            &displayed_detection,
            DeletionPrompt::NotAsked,
            &SignatureLookupState::NotLookedUp,
            DistributionFamily::NixosFamily,
        );
        assert!(
            !incident_steps[BLOCKED_POSITION]
                .status_text
                .contains("scheduled scan")
        );
    }

    #[test]
    fn a_quarantined_file_that_came_back_reads_as_restored() {
        let displayed_detection = DisplayedDetection {
            stored_detection: StoredDetection {
                detection_state: DetectionState::Resolved {
                    resolution_kind: ResolutionKind::MovedToQuarantine,
                    resolved_at: jiff::Timestamp::constant(1_793_869_300, 0),
                },
                ..blocked_detection().stored_detection
            },
            file_state: DetectedFileState::Present {
                access: FileAccess::Readable,
                owner: FileOwner::CurrentUser,
            },
        };
        assert_eq!(
            detection_state_text(&displayed_detection),
            "Restored from quarantine"
        );
    }

    #[test]
    fn the_exclusion_snippet_matches_the_distribution_and_escapes_quotes() {
        let stored_detection = StoredDetection {
            file_path: PathBuf::from("/home/user/a\"b.exe"),
            ..blocked_detection().stored_detection
        };
        assert_eq!(
            false_positive_snippet(&stored_detection, DistributionFamily::NixosFamily),
            "services.clamav.daemon.settings.OnAccessExcludePath = [ \"/home/user/a\\\"b.exe\" ];"
        );
        assert_eq!(
            false_positive_snippet(&stored_detection, DistributionFamily::DebianFamily),
            "OnAccessExcludePath /home/user/a\"b.exe"
        );
    }

    #[test]
    fn test_patterns_are_labelled_in_the_summary() {
        let displayed_detection = DisplayedDetection {
            stored_detection: StoredDetection {
                signature_name: "Eicar-Test-Signature".to_owned(),
                ..blocked_detection().stored_detection
            },
            ..blocked_detection()
        };
        assert!(detection_summary_text(&displayed_detection).contains("test file"));
    }

    #[test]
    fn the_only_lookup_sends_the_signature_name() {
        let lookup_actions = |lookup_state| {
            lookup_choices(lookup_state)
                .into_iter()
                .map(|choice_row| choice_row.chosen_action)
                .collect::<Vec<_>>()
        };
        assert_eq!(
            lookup_actions(&SignatureLookupState::NotLookedUp),
            vec![IncidentStepAction::LookUpIncidentSignature]
        );
        assert_eq!(
            lookup_actions(&SignatureLookupState::KeyMissing),
            vec![
                IncidentStepAction::LookUpIncidentSignature,
                IncidentStepAction::OpenOnlineServices
            ]
        );
    }

    #[test]
    fn lookup_results_summarise_families_and_the_earliest_date() {
        let known_sample = |malware_family: &str, first_seen: &str| {
            crate::host_services::threat_intel::malware_bazaar_client::SampleSummary {
                malware_family: MalwareFamily::NamedFamily(malware_family.to_owned()),
                first_seen: SampleFirstSeen::SeenAt(first_seen.to_owned()),
                sample_tags: Vec::new(),
            }
        };
        assert_eq!(
            lookup_summary_text(&SignatureLookupState::LookupFinished(
                KnownSampleLookup::SamplesFound {
                    known_samples: vec![
                        known_sample("AgentTesla", "2026-10-02"),
                        known_sample("AgentTesla", "2026-10-01"),
                    ],
                }
            )),
            Some("2 known samples. Families: AgentTesla. Earliest seen: 2026-10-01.".to_owned())
        );
        assert_eq!(
            lookup_summary_text(&SignatureLookupState::LookupFinished(
                KnownSampleLookup::SamplesFound {
                    known_samples: vec![
                        crate::host_services::threat_intel::malware_bazaar_client::SampleSummary {
                            malware_family: MalwareFamily::UnnamedFamily,
                            first_seen: SampleFirstSeen::FirstSeenUnknown,
                            sample_tags: Vec::new(),
                        }
                    ],
                }
            )),
            Some("1 known samples. Families: unnamed family. Earliest seen: unknown.".to_owned())
        );
        assert_eq!(
            lookup_summary_text(&SignatureLookupState::NotLookedUp),
            None
        );
    }
}
