use std::rc::Rc;

use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller,
    abstractions::Toaster, adw, adw::prelude::*, gtk, prelude::FactoryVecDeque,
};

use crate::{
    application_window::protection_evidence_refresh::{EvidenceRefresh, GatheredEvidence},
    findings::trusted_detections_page::{
        trusted_detections_presentation::{
            TrustedDetectionsStepAction, trusted_detections_status_steps,
        },
        trusted_detections_section::trusted_detections_group::{
            TrustedDetectionsSection, TrustedDetectionsSectionInit, TrustedDetectionsSectionOutput,
        },
    },
    host_services::desktop::{flatpak_sandbox::SandboxKind, locale_formatting::MomentFormatters},
    shared_widgets::{
        loading_indicators::{
            BUSY_PAGE_NAME, FirstLoad, IDLE_PAGE_NAME, LOADED_PAGE_NAME, LOADING_PAGE_NAME,
            busy_page_name, loading_status_page,
        },
        step_timeline::{
            timeline_step_contents::TimelineStepRowOutput,
            timeline_step_row::{TimelineStepRow, show_timeline_steps},
        },
        system_changes::{
            system_changes_group::{
                SystemChangesGroup, SystemChangesGroupInit, SystemChangesGroupInput,
            },
            system_changes_state::AuditTopic,
        },
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

pub struct TrustedDetectionsPage {
    pub message_catalog: Rc<MessageCatalog>,
    toaster: Toaster,
    status_rows: FactoryVecDeque<TimelineStepRow<TrustedDetectionsStepAction>>,
    evidence_refresh: EvidenceRefresh,
    gathered_evidence: GatheredEvidence,
    trusted_detections_section: Controller<TrustedDetectionsSection>,
    trust_changes_group: Controller<SystemChangesGroup>,
}

#[derive(Debug)]
pub enum TrustedDetectionsPageInput {
    RefreshRequested,
    EvidenceRefreshChanged(EvidenceRefresh),
    EvidenceGathered(GatheredEvidence),
    StepRequested(TimelineStepRowOutput<TrustedDetectionsStepAction>),
    ToastRequested(String),
    TrustListChanged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrustedDetectionsPageOutput {
    EvidenceRefreshRequested,
}

pub fn page_input_for_trusted_output(
    trusted_output: TrustedDetectionsSectionOutput,
) -> TrustedDetectionsPageInput {
    match trusted_output {
        TrustedDetectionsSectionOutput::ToastRequested(toast_text) => {
            TrustedDetectionsPageInput::ToastRequested(toast_text)
        }
        TrustedDetectionsSectionOutput::TrustListChanged => {
            TrustedDetectionsPageInput::TrustListChanged
        }
    }
}

fn plain_text_toast(toast_text: &str) -> adw::Toast {
    adw::Toast::builder()
        .title(toast_text)
        .use_markup(false)
        .build()
}

impl TrustedDetectionsPage {
    const fn first_load(&self) -> FirstLoad {
        match self.gathered_evidence {
            GatheredEvidence::NotGatheredYet => FirstLoad::StillLoading,
            GatheredEvidence::Gathered { .. } => FirstLoad::LoadFinished,
        }
    }

    fn report_output(sender: &ComponentSender<Self>, page_output: TrustedDetectionsPageOutput) {
        let _shell_closed = sender.output(page_output);
    }

    fn show_status_steps(&mut self) {
        let message_catalog = Rc::clone(&self.message_catalog);
        let GatheredEvidence::Gathered {
            protection_evidence,
            ..
        } = &self.gathered_evidence
        else {
            return;
        };
        show_timeline_steps(
            &message_catalog,
            &mut self.status_rows,
            trusted_detections_status_steps(&message_catalog, &protection_evidence.trust_list),
        );
    }
}

pub struct TrustedDetectionsPageInit {
    pub message_catalog: Rc<MessageCatalog>,
    pub sandbox_kind: SandboxKind,
    pub moment_formatters: Rc<MomentFormatters>,
}

#[relm4::component(pub)]
impl Component for TrustedDetectionsPage {
    type Init = TrustedDetectionsPageInit;
    type Input = TrustedDetectionsPageInput;
    type Output = TrustedDetectionsPageOutput;
    type CommandOutput = ();

    view! {
        adw::ToastOverlay {
            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    pack_end = &gtk::Stack {
                        add_named[Some(IDLE_PAGE_NAME)] = &gtk::Button {
                            set_icon_name: "view-refresh-symbolic",
                            set_tooltip_text: Some(&translated_messages::findings::check_again_trusted_tooltip(&model.message_catalog)),
                            connect_clicked => TrustedDetectionsPageInput::RefreshRequested,
                        },
                        add_named[Some(BUSY_PAGE_NAME)] = &adw::Spinner {
                            set_halign: gtk::Align::Center,
                            set_valign: gtk::Align::Center,
                            set_tooltip_text: Some(&translated_messages::findings::checking_trusted_tooltip(&model.message_catalog)),
                        },
                        #[watch]
                        set_visible_child_name: busy_page_name(model.evidence_refresh.is_running()),
                    },
                },

                #[wrap(Some)]
                set_content = &gtk::Stack {
                    #[local_ref]
                    add_named[Some(LOADING_PAGE_NAME)] = status_loading_page -> adw::StatusPage {},
                    add_named[Some(LOADED_PAGE_NAME)] = &adw::PreferencesPage {
                        add: status_group,
                        add: model.trusted_detections_section.widget(),
                        add: model.trust_changes_group.widget(),
                    },
                    #[watch]
                    set_visible_child_name: model.first_load().shown_page_name(),
                },
            },
        }
    }

    fn init(
        TrustedDetectionsPageInit {
            message_catalog,
            sandbox_kind,
            moment_formatters,
        }: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            message_catalog: Rc::clone(&message_catalog),
            toaster: Toaster::new(&root),
            status_rows: FactoryVecDeque::builder()
                .launch(
                    adw::PreferencesGroup::builder()
                        .title(translated_messages::findings::status_title(
                            &message_catalog,
                        ))
                        .build(),
                )
                .forward(
                    sender.input_sender(),
                    TrustedDetectionsPageInput::StepRequested,
                ),
            evidence_refresh: EvidenceRefresh::NothingRunning,
            gathered_evidence: GatheredEvidence::NotGatheredYet,
            trusted_detections_section: TrustedDetectionsSection::builder()
                .launch(TrustedDetectionsSectionInit {
                    message_catalog: Rc::clone(&message_catalog),
                    sandbox_kind,
                    moment_formatters: Rc::clone(&moment_formatters),
                })
                .forward(sender.input_sender(), page_input_for_trusted_output),
            trust_changes_group: SystemChangesGroup::builder()
                .launch(SystemChangesGroupInit {
                    message_catalog: Rc::clone(&message_catalog),
                    moment_formatters,
                    audit_topic: AuditTopic::TrustChanges,
                    group_description:
                        translated_messages::findings::files_trusted_detections_description(
                            &message_catalog,
                        ),
                    empty_title: translated_messages::findings::no_trusted_detections_title(
                        &message_catalog,
                    ),
                })
                .detach(),
        };
        let status_group = &model.status_rows.widget().clone();
        let status_loading_page = &loading_status_page(
            &translated_messages::findings::checking_trusted_detections(&message_catalog),
        );
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        page_input: TrustedDetectionsPageInput,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        let message_catalog = Rc::clone(&self.message_catalog);
        match page_input {
            TrustedDetectionsPageInput::RefreshRequested => {
                Self::report_output(
                    &sender,
                    TrustedDetectionsPageOutput::EvidenceRefreshRequested,
                );
            }
            TrustedDetectionsPageInput::EvidenceRefreshChanged(evidence_refresh) => {
                self.evidence_refresh = evidence_refresh;
            }
            TrustedDetectionsPageInput::EvidenceGathered(gathered_evidence) => {
                self.gathered_evidence = gathered_evidence;
                self.show_status_steps();
            }
            TrustedDetectionsPageInput::StepRequested(TimelineStepRowOutput::ActionChosen(
                chosen_action,
            )) => match chosen_action {},
            TrustedDetectionsPageInput::StepRequested(TimelineStepRowOutput::CopyRequested(
                copied_command,
            )) => {
                root.clipboard().set_text(&copied_command);
                self.toaster.add_toast(adw::Toast::new(
                    &translated_messages::findings::command_copied_clipboard(&message_catalog),
                ));
            }
            TrustedDetectionsPageInput::ToastRequested(toast_text) => {
                self.toaster.add_toast(plain_text_toast(&toast_text));
            }
            TrustedDetectionsPageInput::TrustListChanged => {
                self.trust_changes_group
                    .emit(SystemChangesGroupInput::ReloadRequested);
                Self::report_output(
                    &sender,
                    TrustedDetectionsPageOutput::EvidenceRefreshRequested,
                );
            }
        }
    }
}

#[cfg(test)]
mod trusted_detections_component_tests {
    use super::*;

    #[test]
    fn trusted_detections_requests_become_page_inputs() {
        assert!(matches!(
            page_input_for_trusted_output(TrustedDetectionsSectionOutput::ToastRequested(
                "Removed".to_owned()
            )),
            TrustedDetectionsPageInput::ToastRequested(_)
        ));
        assert!(matches!(
            page_input_for_trusted_output(TrustedDetectionsSectionOutput::TrustListChanged),
            TrustedDetectionsPageInput::TrustListChanged
        ));
    }
}
