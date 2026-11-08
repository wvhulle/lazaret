use crate::{
    host_services::{
        clamav_programs::trusted_program_lookup::ClamavProgram,
        protection_evidence::host_protection_evidence::{
            ProgramTrustFinding, ProgramTrustInspection,
        },
    },
    shared_widgets::step_timeline::{
        step_evidence_builders::{CheckedCondition, evidence_row, no_guidance_step, plain_rows},
        timeline_step_contents::{SettledStep, StepActionButton, StepEvidenceRow, TimelineStep},
    },
    translated_messages,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrustedProgramGroup {
    ScannerPrograms,
    UpdaterPrograms,
}

impl TrustedProgramGroup {
    fn includes_program(self, program_name: &str) -> bool {
        let scanner_names = [
            ClamavProgram::ClamdClient.program_name(),
            ClamavProgram::StandaloneScanner.program_name(),
        ];
        match self {
            Self::ScannerPrograms => scanner_names.contains(&program_name),
            Self::UpdaterPrograms => program_name == ClamavProgram::SignatureUpdater.program_name(),
        }
    }
}

fn program_trust_rows(program_trust_finding: &ProgramTrustFinding) -> Vec<StepEvidenceRow> {
    match program_trust_finding {
        ProgramTrustFinding::ReplaceableProgramIgnored {
            program_name,
            trusted_path,
            ignored_paths,
        } => std::iter::once(evidence_row(
            translated_messages::widgets::program_use(program_name),
            trusted_path.clone(),
        ))
        .chain(ignored_paths.iter().map(|ignored_path| {
            evidence_row(
                translated_messages::widgets::ignored_program(program_name),
                ignored_path.clone(),
            )
        }))
        .collect(),
        ProgramTrustFinding::OnlyReplaceableProgram {
            program_name,
            replaceable_paths,
        } => replaceable_paths
            .iter()
            .map(|replaceable_path| {
                evidence_row(
                    translated_messages::widgets::refused_program(program_name),
                    replaceable_path.clone(),
                )
            })
            .collect(),
    }
}

const fn finding_program_name(program_trust_finding: &ProgramTrustFinding) -> &'static str {
    match program_trust_finding {
        ProgramTrustFinding::ReplaceableProgramIgnored { program_name, .. }
        | ProgramTrustFinding::OnlyReplaceableProgram { program_name, .. } => program_name,
    }
}

fn program_findings_condition<'finding>(
    program_trust_findings: impl IntoIterator<Item = &'finding ProgramTrustFinding>,
) -> CheckedCondition {
    let found_conditions = program_trust_findings
        .into_iter()
        .map(|program_trust_finding| match program_trust_finding {
            ProgramTrustFinding::OnlyReplaceableProgram { .. } => CheckedCondition::ConditionFailed,
            ProgramTrustFinding::ReplaceableProgramIgnored { .. } => {
                CheckedCondition::ReviewSuggested
            }
        })
        .collect::<Vec<_>>();
    if found_conditions.contains(&CheckedCondition::ConditionFailed) {
        CheckedCondition::ConditionFailed
    } else if found_conditions.is_empty() {
        CheckedCondition::ConditionMet
    } else {
        CheckedCondition::ReviewSuggested
    }
}

pub fn program_trust_condition(
    program_trust: &ProgramTrustInspection,
    program_group: TrustedProgramGroup,
) -> CheckedCondition {
    match program_trust {
        ProgramTrustInspection::ProgramsChecked(program_trust_findings) => {
            program_findings_condition(program_trust_findings.iter().filter(
                |program_trust_finding| {
                    program_group.includes_program(finding_program_name(program_trust_finding))
                },
            ))
        }
        ProgramTrustInspection::CheckInterrupted => CheckedCondition::EvidenceUnknown,
    }
}

fn program_trust_title(program_group: TrustedProgramGroup) -> String {
    match program_group {
        TrustedProgramGroup::ScannerPrograms => {
            translated_messages::widgets::scanner_programs_trusted()
        }
        TrustedProgramGroup::UpdaterPrograms => translated_messages::widgets::freshclam_trusted(),
    }
}

fn program_trust_details(program_group: TrustedProgramGroup) -> String {
    match program_group {
        TrustedProgramGroup::ScannerPrograms => {
            translated_messages::widgets::malware_running_clamdscan()
        }
        TrustedProgramGroup::UpdaterPrograms => {
            translated_messages::widgets::malware_running_freshclam()
        }
    }
}

pub fn program_trust_step<StepAction: StepActionButton>(
    program_trust: &ProgramTrustInspection,
    program_group: TrustedProgramGroup,
) -> Option<TimelineStep<StepAction>> {
    match program_trust {
        ProgramTrustInspection::ProgramsChecked(program_trust_findings) => program_findings_step(
            &program_trust_findings
                .iter()
                .filter(|program_trust_finding| {
                    program_group.includes_program(finding_program_name(program_trust_finding))
                })
                .collect::<Vec<_>>(),
            program_group,
        ),
        ProgramTrustInspection::CheckInterrupted => Some(no_guidance_step(
            program_trust_title(program_group),
            program_trust_details(program_group),
            program_trust_condition(program_trust, program_group).node_state(),
            translated_messages::widgets::unknown_checking_clamav_programs(),
        )),
    }
}

fn program_findings_step<StepAction: StepActionButton>(
    program_trust_findings: &[&ProgramTrustFinding],
    program_group: TrustedProgramGroup,
) -> Option<TimelineStep<StepAction>> {
    if program_trust_findings.is_empty() {
        return None;
    }
    let any_program_refused = program_trust_findings.iter().any(|program_trust_finding| {
        matches!(
            program_trust_finding,
            ProgramTrustFinding::OnlyReplaceableProgram { .. }
        )
    });
    let node_state =
        program_findings_condition(program_trust_findings.iter().copied()).node_state();
    let status_text = if any_program_refused {
        translated_messages::widgets::clamav_program_replaced_account()
    } else {
        translated_messages::widgets::clamav_program_account_shadows()
    };
    Some(TimelineStep::settled(
        program_trust_title(program_group),
        program_trust_details(program_group),
        status_text,
        SettledStep {
            evidence_rows: plain_rows(
                program_trust_findings
                    .iter()
                    .flat_map(|program_trust_finding| program_trust_rows(program_trust_finding)),
            ),
            ..SettledStep::without_offers(node_state)
        },
    ))
}
