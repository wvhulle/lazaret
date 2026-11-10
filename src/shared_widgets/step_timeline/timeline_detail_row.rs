use std::rc::Rc;

use relm4::{
    FactorySender, adw,
    adw::prelude::*,
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
};

use crate::{
    shared_widgets::{
        loading_indicators::inline_row_spinner,
        step_timeline::timeline_step_contents::{
            ActionableEvidenceRow, EvidenceMoment, EvidenceRowControl, MomentTooltip,
            OfferedAction, StepActionButton, StepChoiceRow, StepEvidenceRow, StepPhase,
            TimelineStep, TimelineStepRowOutput,
        },
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

pub enum TimelineDetailKind<StepAction> {
    StepChoice(StepChoiceRow<StepAction>),
    ActionableEvidence(ActionableEvidenceRow<StepAction>),
    RunningEvidence(StepEvidenceRow),
    GuidanceCommand(String),
}

pub struct TimelineDetailRow<StepAction> {
    detail_kind: TimelineDetailKind<StepAction>,
    message_catalog: Rc<MessageCatalog>,
}

pub fn detail_rows_of<StepAction: StepActionButton>(
    message_catalog: &Rc<MessageCatalog>,
    timeline_step: &TimelineStep<StepAction>,
) -> Vec<TimelineDetailRow<StepAction>> {
    let phase_rows = match &timeline_step.step_phase {
        StepPhase::StepSettled(settled_step) => settled_step
            .choice_rows
            .iter()
            .cloned()
            .map(TimelineDetailKind::StepChoice)
            .chain(
                settled_step
                    .evidence_rows
                    .iter()
                    .cloned()
                    .map(TimelineDetailKind::ActionableEvidence),
            )
            .collect::<Vec<_>>(),
        StepPhase::StepRunning(running_step) => running_step
            .evidence_rows
            .iter()
            .cloned()
            .map(TimelineDetailKind::RunningEvidence)
            .collect(),
    };
    phase_rows
        .into_iter()
        .chain(
            timeline_step
                .guidance_commands
                .iter()
                .cloned()
                .map(TimelineDetailKind::GuidanceCommand),
        )
        .map(|detail_kind| TimelineDetailRow {
            detail_kind,
            message_catalog: Rc::clone(message_catalog),
        })
        .collect()
}

fn moment_prefix(evidence_moment: EvidenceMoment) -> Option<gtk::Widget> {
    let icon_name = match evidence_moment {
        EvidenceMoment::CurrentFact => return None,
        EvidenceMoment::HappeningNow => return Some(inline_row_spinner().upcast()),
        EvidenceMoment::PastEvent { .. } | EvidenceMoment::NeverHappened => {
            "document-open-recent-symbolic"
        }
        EvidenceMoment::UpcomingEvent { .. } | EvidenceMoment::NothingUpcoming => "alarm-symbolic",
        EvidenceMoment::SettingValue => "emblem-system-symbolic",
    };
    let moment_icon = gtk::Image::from_icon_name(icon_name);
    moment_icon.add_css_class("dim-label");
    Some(moment_icon.upcast())
}

fn evidence_row(step_evidence: &StepEvidenceRow) -> adw::ActionRow {
    let evidence_row = adw::ActionRow::builder()
        .use_markup(false)
        .title(&step_evidence.evidence_name)
        .subtitle(&step_evidence.evidence_value)
        .subtitle_selectable(true)
        .subtitle_lines(0)
        .css_classes(["property"])
        .build();
    match &step_evidence.moment_tooltip {
        MomentTooltip::ExactMomentTooltip(exact_moment) => {
            evidence_row.set_tooltip_text(Some(exact_moment));
        }
        MomentTooltip::NoMomentTooltip => {}
    }
    if let Some(moment_widget) = moment_prefix(step_evidence.evidence_moment) {
        evidence_row.add_prefix(&moment_widget);
    }
    evidence_row
}

pub fn offered_action_button<StepAction: StepActionButton>(
    message_catalog: &MessageCatalog,
    offered_action: &OfferedAction<StepAction>,
) -> gtk::Button {
    let action_button = gtk::Button::builder()
        .label(offered_action.offered_action.button_label(message_catalog))
        .valign(gtk::Align::Center)
        .build();
    if let Some(style_class) = offered_action.button_style().css_class() {
        action_button.add_css_class(style_class);
    }
    action_button
}

fn action_choosing_button<StepAction: StepActionButton>(
    message_catalog: &MessageCatalog,
    offered_action: &OfferedAction<StepAction>,
    sender: FactorySender<TimelineDetailRow<StepAction>>,
) -> gtk::Button {
    let action_button = offered_action_button(message_catalog, offered_action);
    let chosen_action = offered_action.offered_action.clone();
    action_button.connect_clicked(move |_clicked_button| {
        let _step_closed =
            sender.output(TimelineStepRowOutput::ActionChosen(chosen_action.clone()));
    });
    action_button
}

fn attach_row_control<StepAction: StepActionButton>(
    message_catalog: &MessageCatalog,
    evidence_row: &adw::ActionRow,
    row_control: &EvidenceRowControl<StepAction>,
    sender: FactorySender<TimelineDetailRow<StepAction>>,
) {
    match row_control {
        EvidenceRowControl::NoRowControl => {}
        EvidenceRowControl::RowButton(offered_action) => {
            evidence_row.add_suffix(&action_choosing_button(
                message_catalog,
                offered_action,
                sender,
            ));
        }
    }
}

fn guidance_command_row(guidance_command: &str) -> adw::ActionRow {
    let command_label = gtk::Label::builder()
        .label(guidance_command)
        .use_markup(false)
        .selectable(true)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::WordChar)
        .hexpand(true)
        .xalign(0.0)
        .css_classes(["monospace"])
        .build();
    let command_row = adw::ActionRow::new();
    command_row.add_prefix(&command_label);
    command_row
}

fn copy_button<StepAction: StepActionButton>(
    message_catalog: &MessageCatalog,
    copied_command: String,
    sender: FactorySender<TimelineDetailRow<StepAction>>,
) -> gtk::Button {
    let copy_button = gtk::Button::builder()
        .icon_name("edit-copy-symbolic")
        .tooltip_text(translated_messages::widgets::copy_timeline_tooltip(
            message_catalog,
        ))
        .valign(gtk::Align::Center)
        .css_classes(["flat"])
        .build();
    copy_button.connect_clicked(move |_clicked_button| {
        let _step_closed =
            sender.output(TimelineStepRowOutput::CopyRequested(copied_command.clone()));
    });
    copy_button
}

fn choice_row<StepAction>(step_choice: &StepChoiceRow<StepAction>) -> adw::ActionRow {
    adw::ActionRow::builder()
        .use_markup(false)
        .title(&step_choice.choice_description)
        .title_lines(0)
        .build()
}

impl<StepAction: StepActionButton> FactoryComponent for TimelineDetailRow<StepAction> {
    type ParentWidget = adw::ExpanderRow;
    type CommandOutput = ();
    type Input = ();
    type Output = TimelineStepRowOutput<StepAction>;
    type Init = Self;
    type Root = adw::ActionRow;
    type Widgets = ();
    type Index = DynamicIndex;

    fn init_model(
        detail_row: Self::Init,
        _row_index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        detail_row
    }

    fn init_root(&self) -> Self::Root {
        match &self.detail_kind {
            TimelineDetailKind::StepChoice(step_choice) => choice_row(step_choice),
            TimelineDetailKind::ActionableEvidence(actionable_row) => {
                evidence_row(&actionable_row.shown_evidence)
            }
            TimelineDetailKind::RunningEvidence(step_evidence) => evidence_row(step_evidence),
            TimelineDetailKind::GuidanceCommand(guidance_command) => {
                guidance_command_row(guidance_command)
            }
        }
    }

    fn init_widgets(
        &mut self,
        _row_index: &DynamicIndex,
        root: Self::Root,
        _returned_widget: &gtk::Widget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let message_catalog = Rc::clone(&self.message_catalog);
        match &self.detail_kind {
            TimelineDetailKind::StepChoice(step_choice) => {
                root.add_suffix(&action_choosing_button(
                    &message_catalog,
                    &step_choice.offered_choice(),
                    sender,
                ));
            }
            TimelineDetailKind::GuidanceCommand(guidance_command) => {
                root.add_suffix(&copy_button(
                    &self.message_catalog,
                    guidance_command.clone(),
                    sender,
                ));
            }
            TimelineDetailKind::ActionableEvidence(actionable_row) => {
                attach_row_control(&message_catalog, &root, &actionable_row.row_control, sender);
            }
            TimelineDetailKind::RunningEvidence(_) => {}
        }
    }
}
