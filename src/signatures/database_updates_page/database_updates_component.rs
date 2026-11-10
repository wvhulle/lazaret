use std::rc::Rc;

use futures::StreamExt;
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller,
    abstractions::Toaster, adw, adw::prelude::*, gtk, prelude::FactoryVecDeque,
};

use crate::{
    application_window::protection_evidence_refresh::{EvidenceRefresh, GatheredEvidence},
    host_services::{
        activity_history::activity_history_database::{
            ActivityEntryRecord, ActivityHistory, ActivityHistoryFailure,
        },
        clamav_programs::distribution_install_guides::{
            DistributionFamily, detect_distribution_family,
        },
        desktop::{
            desktop_notifications::notify_outside_the_window,
            failure_journal_log::{log_failed_outcome, log_failure_to_journal},
            flatpak_sandbox::SandboxKind,
            locale_formatting::MomentFormatters,
        },
        systemd_bus::host_systemd_units::SystemBusAccess,
    },
    shared_widgets::{
        activity_entries::{
            activity_entry_row::{ActivityEntryRow, ActivityEntryRowInit},
            activity_transcript_dialog::{ActivityTranscriptDialog, ActivityTranscriptDialogInit},
        },
        component_streams::run_stream_then,
        database_migration_failure_text::describe_history_failure,
        engine_settings::engine_settings_sections::{
            EngineSettingsLayout, EngineSettingsNotice, EngineSettingsSection,
            EngineSettingsSections, EngineSettingsSectionsInit, EngineSettingsSectionsOutput,
            engine_settings_notice_toast,
        },
        loading_indicators::{
            BUSY_PAGE_NAME, FirstLoad, IDLE_PAGE_NAME, LOAD_FAILED_PAGE_NAME, LOADED_PAGE_NAME,
            LOADING_PAGE_NAME, busy_page_name, empty_placeholder_row, loading_placeholder_stack,
            loading_status_page,
        },
        step_timeline::{
            timeline_step_contents::TimelineStepRowOutput,
            timeline_step_row::{TimelineStepRow, show_timeline_steps},
        },
    },
    signatures::database_updates_page::{
        database_updates_presentation::{DatabaseUpdateAction, database_update_steps},
        database_updates_state::{
            DatabaseUpdatesPageOutput, SignatureUpdateControl, UpdateFinishEffects,
            UpdateHistoryView, UpdateStartDecision,
        },
        signature_update_run::{
            database_update_events::{DatabaseUpdateEvent, database_update_events},
            signature_update_session::{DatabaseHostStatus, load_host_status},
        },
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

const UPDATE_BATCH_SIZE: usize = 64;

enum TranscriptDialogView {
    DialogClosed,
    DialogOpen {
        _transcript_dialog: Controller<ActivityTranscriptDialog>,
    },
}

pub struct DatabaseUpdatesPageInit {
    pub message_catalog: Rc<MessageCatalog>,
    pub moment_formatters: Rc<MomentFormatters>,
    pub system_bus: SystemBusAccess,
    pub activity_history: ActivityHistory,
    pub sandbox_kind: SandboxKind,
}

pub struct DatabaseUpdatesPage {
    pub message_catalog: Rc<MessageCatalog>,
    moment_formatters: Rc<MomentFormatters>,
    system_bus: SystemBusAccess,
    sandbox_kind: SandboxKind,
    toaster: Toaster,
    status_rows: FactoryVecDeque<TimelineStepRow<DatabaseUpdateAction>>,
    history_rows: FactoryVecDeque<ActivityEntryRow>,
    update_history: UpdateHistoryView,
    activity_history: ActivityHistory,
    update_control: SignatureUpdateControl,
    distribution_family: DistributionFamily,
    evidence_refresh: EvidenceRefresh,
    gathered_evidence: GatheredEvidence,
    transcript_dialog: TranscriptDialogView,
    update_settings_sections: Controller<EngineSettingsSections>,
}

#[derive(Debug)]
pub enum DatabaseUpdatesPageInput {
    RefreshRequested,
    EvidenceRefreshChanged(EvidenceRefresh),
    EvidenceGathered(GatheredEvidence),
    SignatureUpdateRequested,
    SettingsNoticeReceived(EngineSettingsNotice),
    StepRequested(TimelineStepRowOutput<DatabaseUpdateAction>),
    HistoryEntryActivated { row_position: usize },
    HistoryReloadRequested,
}

#[derive(Debug)]
pub enum DatabaseUpdatesPageCommand {
    HostStatusLoaded(DatabaseHostStatus),
    UpdateBatchArrived(Vec<DatabaseUpdateEvent>),
    UpdateStreamFinished,
    ActivityRecordingFinished(Result<(), ActivityHistoryFailure>),
    HistoryLoaded(Result<Vec<ActivityEntryRecord>, ActivityHistoryFailure>),
}

fn plain_text_toast(toast_text: &str) -> adw::Toast {
    adw::Toast::builder()
        .title(toast_text)
        .use_markup(false)
        .build()
}

pub fn page_input_for_settings_output(
    settings_output: EngineSettingsSectionsOutput,
) -> DatabaseUpdatesPageInput {
    match settings_output {
        EngineSettingsSectionsOutput::ToastRequested(settings_notice) => {
            DatabaseUpdatesPageInput::SettingsNoticeReceived(settings_notice)
        }
    }
}

impl DatabaseUpdatesPage {
    const fn first_load(&self) -> FirstLoad {
        match self.gathered_evidence {
            GatheredEvidence::NotGatheredYet => FirstLoad::StillLoading,
            GatheredEvidence::Gathered { .. } => FirstLoad::LoadFinished,
        }
    }

    fn report_output(sender: &ComponentSender<Self>, page_output: DatabaseUpdatesPageOutput) {
        let _shell_closed = sender.output(page_output);
    }

    fn history_failure_text(&self) -> String {
        let message_catalog = Rc::clone(&self.message_catalog);
        match &self.update_history {
            UpdateHistoryView::HistoryLoadFailed(load_failure) => {
                translated_messages::signatures::history_load_failed(
                    &message_catalog,
                    describe_history_failure(&message_catalog, load_failure),
                )
            }
            UpdateHistoryView::HistoryLoading | UpdateHistoryView::HistoryLoaded(_) => {
                String::new()
            }
        }
    }

    fn show_status_steps(&mut self) {
        let message_catalog = Rc::clone(&self.message_catalog);
        let moment_formatters = Rc::clone(&self.moment_formatters);
        let GatheredEvidence::Gathered {
            protection_evidence,
            gathered_at,
        } = &self.gathered_evidence
        else {
            return;
        };
        show_timeline_steps(
            &message_catalog,
            &mut self.status_rows,
            database_update_steps(
                &message_catalog,
                protection_evidence,
                &self.update_control.update_view(&message_catalog),
                &moment_formatters,
                *gathered_at,
                self.distribution_family,
            ),
        );
    }

    fn reload_history(&self, sender: &ComponentSender<Self>) {
        let activity_history = self.activity_history.clone();
        sender.oneshot_command(async move {
            DatabaseUpdatesPageCommand::HistoryLoaded(activity_history.list_recent_entries().await)
        });
    }

    fn refresh_host_status(
        system_bus: SystemBusAccess,
        sandbox_kind: SandboxKind,
        sender: &ComponentSender<Self>,
    ) {
        sender.oneshot_command(async move {
            DatabaseUpdatesPageCommand::HostStatusLoaded(
                load_host_status(&system_bus, sandbox_kind).await,
            )
        });
    }

    fn follow_start_decision(
        &mut self,
        start_decision: UpdateStartDecision,
        sender: &ComponentSender<Self>,
    ) {
        let sandbox_kind = self.sandbox_kind;
        match start_decision {
            UpdateStartDecision::StartUpdateNow(update_plan) => {
                let update_stream = run_stream_then(
                    sender,
                    database_update_events(self.system_bus.clone(), sandbox_kind, update_plan)
                        .ready_chunks(UPDATE_BATCH_SIZE)
                        .map(DatabaseUpdatesPageCommand::UpdateBatchArrived),
                    DatabaseUpdatesPageCommand::UpdateStreamFinished,
                );
                self.update_control
                    .update_started(update_plan, update_stream);
            }
            UpdateStartDecision::UpdateQueued | UpdateStartDecision::NothingToStart => {}
        }
        self.show_status_steps();
    }

    fn finish_update(&mut self, sender: &ComponentSender<Self>) {
        let message_catalog = Rc::clone(&self.message_catalog);
        let sandbox_kind = self.sandbox_kind;
        let finished_effects = match self.update_control.update_stream_ended(&message_catalog) {
            UpdateFinishEffects::UpdateFinished(finished_effects) => finished_effects,
            UpdateFinishEffects::StaleStreamEnding => return,
        };
        notify_outside_the_window(
            "update-finished",
            &translated_messages::signatures::database_update_finished(&message_catalog),
            &finished_effects.summary_line,
        );
        self.toaster
            .add_toast(plain_text_toast(&finished_effects.summary_line));
        let activity_history = self.activity_history.clone();
        let activity_entry = finished_effects.activity_entry;
        sender.oneshot_command(async move {
            DatabaseUpdatesPageCommand::ActivityRecordingFinished(
                activity_history.record_activity(activity_entry).await,
            )
        });
        Self::refresh_host_status(self.system_bus.clone(), sandbox_kind, sender);
        self.show_status_steps();
        finished_effects
            .page_outputs
            .into_iter()
            .for_each(|page_output| Self::report_output(sender, page_output));
    }

    fn open_transcript(&mut self, row_position: usize, root: &adw::ToastOverlay) {
        let message_catalog = Rc::clone(&self.message_catalog);
        let moment_formatters = Rc::clone(&self.moment_formatters);
        let Some(activity_entry) = self.update_history.listed_entries().get(row_position) else {
            return;
        };
        let transcript_dialog = ActivityTranscriptDialog::builder()
            .launch(ActivityTranscriptDialogInit {
                message_catalog,
                activity_entry: activity_entry.clone(),
                moment_formatters: Rc::clone(&moment_formatters),
            })
            .detach();
        transcript_dialog.widget().present(Some(root));
        self.transcript_dialog = TranscriptDialogView::DialogOpen {
            _transcript_dialog: transcript_dialog,
        };
    }
}

#[relm4::component(pub)]
impl Component for DatabaseUpdatesPage {
    type Init = DatabaseUpdatesPageInit;
    type Input = DatabaseUpdatesPageInput;
    type Output = DatabaseUpdatesPageOutput;
    type CommandOutput = DatabaseUpdatesPageCommand;

    view! {
        adw::ToastOverlay {
            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    pack_end = &gtk::Stack {
                        add_named[Some(IDLE_PAGE_NAME)] = &gtk::Button {
                            set_icon_name: "view-refresh-symbolic",
                            set_tooltip_text: Some(&translated_messages::signatures::check_again_database_tooltip(&model.message_catalog)),
                            connect_clicked => DatabaseUpdatesPageInput::RefreshRequested,
                        },
                        add_named[Some(BUSY_PAGE_NAME)] = &adw::Spinner {
                            set_halign: gtk::Align::Center,
                            set_valign: gtk::Align::Center,
                            set_tooltip_text: Some(&translated_messages::signatures::checking_database_tooltip(&model.message_catalog)),
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
                        add: model.update_settings_sections.widget(),

                        add = &adw::PreferencesGroup {
                            set_title: &translated_messages::signatures::update_history_title(&model.message_catalog),
                            set_description: Some(&translated_messages::signatures::updates_started_lazaret_description(&model.message_catalog)),

                            #[local_ref]
                            history_list -> gtk::ListBox {
                                add_css_class: "boxed-list",
                                set_selection_mode: gtk::SelectionMode::None,
                                #[wrap(Some)]
                                #[local_ref]
                                set_placeholder = history_placeholder -> gtk::Stack {
                                    add_named[Some(LOAD_FAILED_PAGE_NAME)] = &adw::ActionRow {
                                        set_title: &translated_messages::signatures::historical_logs_cannot_title(&model.message_catalog),
                                        set_use_markup: false,
                                        set_subtitle_lines: 0,
                                        add_css_class: "dim-label",
                                        #[watch]
                                        set_subtitle: &model.history_failure_text(),
                                    },
                                    #[watch]
                                    set_visible_child_name: model.update_history.placeholder_page_name(),
                                },
                                connect_row_activated[sender] => move |_history_list, activated_row| {
                                    sender.input(DatabaseUpdatesPageInput::HistoryEntryActivated {
                                        row_position: usize::try_from(activated_row.index()).unwrap_or_default(),
                                    });
                                },
                            },
                        },
                    },
                    #[watch]
                    set_visible_child_name: model.first_load().shown_page_name(),
                },
            },
        }
    }

    fn init(
        DatabaseUpdatesPageInit {
            message_catalog,
            moment_formatters,
            activity_history,
            sandbox_kind,
            system_bus,
        }: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            message_catalog: Rc::clone(&message_catalog),
            moment_formatters: Rc::clone(&moment_formatters),
            system_bus,
            sandbox_kind,
            toaster: Toaster::new(&root),
            status_rows: FactoryVecDeque::builder()
                .launch(
                    adw::PreferencesGroup::builder()
                        .title(translated_messages::signatures::status_title(
                            &message_catalog,
                        ))
                        .build(),
                )
                .forward(
                    sender.input_sender(),
                    DatabaseUpdatesPageInput::StepRequested,
                ),
            history_rows: FactoryVecDeque::builder()
                .launch(gtk::ListBox::default())
                .detach(),
            update_history: UpdateHistoryView::HistoryLoading,
            activity_history,
            update_control: SignatureUpdateControl::new(),
            distribution_family: detect_distribution_family(sandbox_kind),
            evidence_refresh: EvidenceRefresh::NothingRunning,
            gathered_evidence: GatheredEvidence::NotGatheredYet,
            transcript_dialog: TranscriptDialogView::DialogClosed,
            update_settings_sections: EngineSettingsSections::builder()
                .launch(EngineSettingsSectionsInit {
                    message_catalog: Rc::clone(&message_catalog),
                    moment_formatters,
                    settings_layout: EngineSettingsLayout::SingleSection(
                        EngineSettingsSection::SignatureUpdates,
                    ),
                    sandbox_kind,
                })
                .forward(sender.input_sender(), page_input_for_settings_output),
        };
        let status_group = &model.status_rows.widget().clone();
        let history_list = model.history_rows.widget();
        let history_placeholder = &loading_placeholder_stack(
            &translated_messages::signatures::loading_history(&message_catalog),
            &empty_placeholder_row(
                &translated_messages::signatures::no_updates_started_lazaret(&message_catalog),
            ),
        );
        let status_loading_page = &loading_status_page(
            &translated_messages::signatures::checking_signatures(&message_catalog),
        );
        let widgets = view_output!();
        Self::refresh_host_status(model.system_bus.clone(), sandbox_kind, &sender);
        model.reload_history(&sender);
        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        page_input: DatabaseUpdatesPageInput,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        let message_catalog = Rc::clone(&self.message_catalog);
        match page_input {
            DatabaseUpdatesPageInput::HistoryReloadRequested => self.reload_history(&sender),
            DatabaseUpdatesPageInput::RefreshRequested => {
                Self::report_output(&sender, DatabaseUpdatesPageOutput::EvidenceRefreshRequested);
            }
            DatabaseUpdatesPageInput::EvidenceRefreshChanged(evidence_refresh) => {
                self.evidence_refresh = evidence_refresh;
            }
            DatabaseUpdatesPageInput::EvidenceGathered(gathered_evidence) => {
                self.gathered_evidence = gathered_evidence;
                self.show_status_steps();
            }
            DatabaseUpdatesPageInput::SignatureUpdateRequested => {
                let start_decision = self.update_control.update_requested();
                self.follow_start_decision(start_decision, &sender);
            }
            DatabaseUpdatesPageInput::SettingsNoticeReceived(settings_notice) => {
                self.toaster.add_toast(engine_settings_notice_toast(
                    &message_catalog,
                    &settings_notice,
                ));
            }
            DatabaseUpdatesPageInput::StepRequested(TimelineStepRowOutput::ActionChosen(
                DatabaseUpdateAction::UpdateSignatures,
            )) => {
                let start_decision = self.update_control.update_requested();
                self.follow_start_decision(start_decision, &sender);
            }
            DatabaseUpdatesPageInput::StepRequested(TimelineStepRowOutput::ActionChosen(
                DatabaseUpdateAction::CancelSignatureUpdate,
            )) => {
                self.update_control.cancel_requested();
                self.show_status_steps();
            }
            DatabaseUpdatesPageInput::StepRequested(TimelineStepRowOutput::CopyRequested(
                copied_command,
            )) => {
                root.clipboard().set_text(&copied_command);
                self.toaster.add_toast(adw::Toast::new(
                    &translated_messages::signatures::command_copied_clipboard_database(
                        &message_catalog,
                    ),
                ));
            }
            DatabaseUpdatesPageInput::HistoryEntryActivated { row_position } => {
                self.open_transcript(row_position, root);
            }
        }
    }

    fn update_cmd(
        &mut self,
        page_command: DatabaseUpdatesPageCommand,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        let message_catalog = Rc::clone(&self.message_catalog);
        let moment_formatters = Rc::clone(&self.moment_formatters);
        match page_command {
            DatabaseUpdatesPageCommand::HostStatusLoaded(host_status) => {
                let start_decision = self.update_control.host_status_loaded(host_status);
                self.follow_start_decision(start_decision, &sender);
            }
            DatabaseUpdatesPageCommand::UpdateBatchArrived(update_batch) => {
                self.update_control
                    .absorb_update_batch(&self.message_catalog, update_batch);
                self.show_status_steps();
            }
            DatabaseUpdatesPageCommand::UpdateStreamFinished => self.finish_update(&sender),
            DatabaseUpdatesPageCommand::ActivityRecordingFinished(Ok(())) => {
                self.reload_history(&sender);
                Self::report_output(&sender, DatabaseUpdatesPageOutput::ActivityRecorded);
            }
            DatabaseUpdatesPageCommand::ActivityRecordingFinished(Err(recording_failure)) => {
                log_failure_to_journal(
                    "recording the update in the history failed",
                    &recording_failure,
                );
                self.toaster.add_toast(plain_text_toast(
                    &translated_messages::signatures::update_not_added_history(
                        &message_catalog,
                        describe_history_failure(&message_catalog, &recording_failure),
                    ),
                ));
            }
            DatabaseUpdatesPageCommand::HistoryLoaded(load_outcome) => {
                log_failed_outcome("loading the update history failed", &load_outcome);
                self.update_history = UpdateHistoryView::from_load_outcome(load_outcome);
                let mut history_guard = self.history_rows.guard();
                history_guard.clear();
                self.update_history
                    .listed_entries()
                    .iter()
                    .cloned()
                    .for_each(|activity_entry| {
                        history_guard.push_back(ActivityEntryRowInit {
                            message_catalog: Rc::clone(&message_catalog),
                            activity_entry,
                            moment_formatters: Rc::clone(&moment_formatters),
                        });
                    });
            }
        }
    }
}

#[cfg(test)]
mod database_updates_component_tests {
    use super::*;
    use crate::host_services::clamav_admin_service::live_settings_client::LiveChangeOutcome;

    #[test]
    fn a_settings_toast_request_becomes_a_page_notice() {
        assert!(matches!(
            page_input_for_settings_output(EngineSettingsSectionsOutput::ToastRequested(
                EngineSettingsNotice::LiveChangeReported(LiveChangeOutcome::PreviousRestored)
            )),
            DatabaseUpdatesPageInput::SettingsNoticeReceived(
                EngineSettingsNotice::LiveChangeReported(LiveChangeOutcome::PreviousRestored)
            )
        ));
        assert!(matches!(
            page_input_for_settings_output(EngineSettingsSectionsOutput::ToastRequested(
                EngineSettingsNotice::ChangeSnippetCopied
            )),
            DatabaseUpdatesPageInput::SettingsNoticeReceived(
                EngineSettingsNotice::ChangeSnippetCopied
            )
        ));
    }
}
