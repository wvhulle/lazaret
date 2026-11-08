use futures::{StreamExt, stream};
use jiff::{SignedDuration, Timestamp};
use relm4::{
    Component, ComponentParts, ComponentSender, abstractions::Toaster, adw, adw::prelude::*, gtk,
    prelude::FactoryVecDeque,
};

use crate::{
    findings::quarantine_page::{
        quarantine_entry_row::{
            QuarantineEntryRow, QuarantineEntryRowInput, QuarantineEntryRowOutput,
        },
        quarantine_page_state::{
            EntryOperationKind, QuarantineEntryCount, QuarantineListing, QuarantinePageState,
        },
    },
    host_services::{
        activity_history::activity_history_database::{ActivityHistory, ActivityHistoryFailure},
        clamav_admin_service::trust_requests::{
            TrustChoice, TrustOutcome, request_trust_change, trust_outcome_text,
        },
        clamav_programs::{
            clamav_report_submission::ClamavReportRequest, trusted_program_lookup::ProgramRefusal,
        },
        clamav_scanner::{
            content_verdict::{
                ContentVerdict, VerdictFailure, scan_content_with_current_signatures,
            },
            scanner_output_parser::{displayed_threat_name, sanitize_scanner_line},
        },
        desktop::{
            failure_journal_log::{log_failed_outcome, log_failure_to_journal},
            locale_formatting::byte_size_text,
        },
        quarantine_vault::{
            quarantine_entry_database::{
                QuarantineEntryRecord, QuarantineVault, StoredDetectionDate,
            },
            quarantine_file_moves::QuarantineMoveFailure,
        },
        sample_submissions::clamav_report_review::{
            current_database_version, settle_clamav_reviews,
        },
        threat_intel::file_reputation_check::{ReputationAnswers, check_hash_reputation},
    },
    shared_widgets::{
        awaited_dialogs::{
            AlertChoice, add_alert_choice, add_dismissing_choice, alert_choice,
            deliver_local_outcome,
        },
        component_streams::{run_stream, run_stream_to_completion},
        loading_indicators::{
            LOAD_FAILED_PAGE_NAME, empty_placeholder_row, loading_placeholder_stack,
        },
        page_widget_templates::ClampedPage,
        quarantine_failure_text::describe_quarantine_failure,
        scanner_exit_text::abnormal_scanner_exit_text,
        submission_confirm_dialog::{ClamavReportProposal, present_clamav_report_confirmation},
        submission_outcome_text::history_failure_text,
        trust_confirmation_dialog::present_trust_confirmation,
    },
    translated_messages,
};

const OLD_ENTRY_AGE: SignedDuration = SignedDuration::from_hours(30 * 24);

pub fn entry_matches_search(entry_record: &QuarantineEntryRecord, search_text: &str) -> bool {
    let lowercase_search = search_text.trim().to_lowercase();
    displayed_threat_name(&entry_record.threat_identity)
        .to_lowercase()
        .contains(&lowercase_search)
        || entry_record
            .original_path
            .to_string_lossy()
            .to_lowercase()
            .contains(&lowercase_search)
}

pub fn entry_is_older_than(
    entry_record: &QuarantineEntryRecord,
    maximum_age: SignedDuration,
    current_time: Timestamp,
) -> bool {
    match entry_record.detection_date {
        StoredDetectionDate::DetectedAt(detected_at) => {
            current_time.duration_since(detected_at) > maximum_age
        }
        StoredDetectionDate::UnreadableText(_) => false,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RestoreDialogChoice {
    RestoreChosen,
    RestoreCancelled,
}

impl AlertChoice for RestoreDialogChoice {
    const ANSWERING_CHOICES: &'static [Self] = &[Self::RestoreChosen];
    const DISMISSING_CHOICE: Self = Self::RestoreCancelled;

    fn response_id(self) -> &'static str {
        match self {
            Self::RestoreChosen => "restore",
            Self::RestoreCancelled => "cancel",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DeletionDialogChoice {
    DeletionConfirmed,
    DeletionCancelled,
}

impl AlertChoice for DeletionDialogChoice {
    const ANSWERING_CHOICES: &'static [Self] = &[Self::DeletionConfirmed];
    const DISMISSING_CHOICE: Self = Self::DeletionCancelled;

    fn response_id(self) -> &'static str {
        match self {
            Self::DeletionConfirmed => "delete",
            Self::DeletionCancelled => "cancel",
        }
    }
}

fn add_deletion_choices(confirmation_dialog: &adw::AlertDialog) {
    add_dismissing_choice::<DeletionDialogChoice>(
        confirmation_dialog,
        &translated_messages::findings::cancel_quarantine(),
    );
    add_alert_choice(
        confirmation_dialog,
        DeletionDialogChoice::DeletionConfirmed,
        &translated_messages::findings::delete_entries(),
    );
    confirmation_dialog.set_response_appearance(
        DeletionDialogChoice::DeletionConfirmed.response_id(),
        adw::ResponseAppearance::Destructive,
    );
}

pub struct QuarantineEntriesPageInit {
    pub quarantine_vault: QuarantineVault,
    pub activity_history: ActivityHistory,
}

pub struct QuarantineEntriesPage {
    quarantine_vault: QuarantineVault,
    activity_history: ActivityHistory,
    page_state: QuarantinePageState,
    search_text: String,
    entry_rows: FactoryVecDeque<QuarantineEntryRow>,
    search_mode_enabled: bool,
    toaster: Toaster,
}

#[derive(Debug)]
pub enum QuarantineEntriesPageInput {
    ReloadEntriesRequested,
    RestoreEntryRequested { entry_id: i64 },
    RestoreEntryConfirmed { entry_id: i64 },
    RestoreEntryCancelled { entry_id: i64 },
    RecheckAllRequested,
    DeleteEntryRequested { entry_id: i64 },
    DeleteEntryConfirmed { entry_id: i64 },
    SearchTextChanged(String),
    SearchModeToggled(bool),
    ClearOldEntriesRequested,
    ClearOldEntriesConfirmed,
    OperationFailuresDismissed,
    ReputationCheckRequested { entry_id: i64, content_hash: String },
    HashCopyRequested(String),
    ReportLinkRequested(String),
    TrustRequested(TrustChoice),
    TrustConfirmed(TrustChoice),
    FalsePositiveReportRequested(ClamavReportProposal),
    FalsePositiveReportConfirmed(ClamavReportRequest),
    SubmissionRequested { entry_id: i64 },
}

#[derive(Debug)]
pub enum QuarantineEntriesPageOutput {
    EntriesCounted(QuarantineEntryCount),
    SubmissionsChanged,
    ClamavReportConfirmed(ClamavReportRequest),
    SubmissionRequested { entry_id: i64 },
}

#[derive(Debug)]
pub enum QuarantineEntriesPageCommand {
    EntriesLoaded(Result<Vec<QuarantineEntryRecord>, QuarantineMoveFailure>),
    EntryOperationFinished {
        entry_id: i64,
        operation_outcome: Result<QuarantineEntryRecord, QuarantineMoveFailure>,
    },
    ReputationCheckFinished {
        entry_id: i64,
        reputation_answers: ReputationAnswers,
    },
    TrustChangeFinished(TrustOutcome),
    RestoreCheckFinished {
        entry_id: i64,
        content_verdict: ContentVerdict,
    },
    RecheckVerdictArrived {
        entry_id: i64,
        content_verdict: ContentVerdict,
    },
    RecheckFinished,
    ClamavReviewsSettled(Result<usize, ActivityHistoryFailure>),
}

async fn verdict_for_entry(quarantine_vault: &QuarantineVault, entry_id: i64) -> ContentVerdict {
    match quarantine_vault.read_payload_plaintext(entry_id).await {
        Ok(payload_plaintext) => scan_content_with_current_signatures(payload_plaintext).await,
        Err(read_failure) => VerdictFailure::PayloadUnreadable(read_failure).into(),
    }
}

fn verdict_failure_text(verdict_failure: &VerdictFailure) -> String {
    match verdict_failure {
        VerdictFailure::ScannerProgramRefused(ProgramRefusal::ProgramMissing) => {
            translated_messages::findings::scanner_program_not_installed()
        }
        VerdictFailure::ScannerProgramRefused(ProgramRefusal::ProgramReplaceable {
            candidate_path,
            replaceable_path,
        }) => translated_messages::findings::scanner_program_replaced_account(
            sanitize_scanner_line(&candidate_path.to_string_lossy()),
            sanitize_scanner_line(&replaceable_path.to_string_lossy()),
        ),
        VerdictFailure::ContentPipeFailed(pipe_failure) => {
            translated_messages::findings::content_pipe_failed(pipe_failure)
        }
        VerdictFailure::ScannerSpawnFailed(spawn_failure) => {
            translated_messages::findings::scanner_spawn_failed(spawn_failure)
        }
        VerdictFailure::VerdictTimedOut => translated_messages::findings::verdict_timed_out(),
        VerdictFailure::ScannerReported { scanner_message } => {
            sanitize_scanner_line(scanner_message)
        }
        VerdictFailure::ScannerExitedAbnormally(abnormal_exit) => {
            abnormal_scanner_exit_text(abnormal_exit)
        }
        VerdictFailure::DetectionSettingsUnreadable(settings_failure) => {
            translated_messages::findings::detection_settings_unreadable(settings_failure)
        }
        VerdictFailure::ProgramLookupFailed(lookup_failure) => {
            translated_messages::findings::program_lookup_failed(lookup_failure)
        }
        VerdictFailure::PayloadUnreadable(read_failure) => {
            translated_messages::findings::payload_unreadable(read_failure)
        }
    }
}

struct RestoreConfirmationText {
    heading: String,
    verdict_text: String,
    confirm_label: String,
    restores_detected_file: bool,
}

fn restore_confirmation_text(content_verdict: &ContentVerdict) -> RestoreConfirmationText {
    match content_verdict {
        ContentVerdict::NoLongerDetected => RestoreConfirmationText {
            heading: translated_messages::findings::restore_file_heading(),
            verdict_text: translated_messages::findings::clamav_no_longer_verdict(),
            confirm_label: translated_messages::findings::restore_button(),
            restores_detected_file: false,
        },
        ContentVerdict::StillDetected { detection_name } => RestoreConfirmationText {
            heading: translated_messages::findings::restore_file_clamav_heading(),
            verdict_text: translated_messages::findings::clamav_still_detects_verdict(
                detection_name,
            ),
            confirm_label: translated_messages::findings::restore_anyway_button(),
            restores_detected_file: true,
        },
        ContentVerdict::VerdictUnavailable(verdict_failure) => RestoreConfirmationText {
            heading: translated_messages::findings::restore_unchecked_file_heading(),
            verdict_text: translated_messages::findings::file_not_checked_verdict(
                verdict_failure_text(verdict_failure),
            ),
            confirm_label: translated_messages::findings::restore_anyway_button(),
            restores_detected_file: true,
        },
    }
}

fn restore_destination_note(entry_record: &QuarantineEntryRecord) -> Option<String> {
    (!entry_record
        .original_path
        .starts_with(gtk::glib::home_dir()))
    .then(translated_messages::findings::location_outside_home_folder)
}

fn plain_text_toast(toast_text: &str) -> adw::Toast {
    adw::Toast::builder()
        .title(toast_text)
        .use_markup(false)
        .build()
}

impl QuarantineEntriesPage {
    fn send_row_states(&self) {
        self.entry_rows
            .iter()
            .enumerate()
            .for_each(|(row_position, entry_row)| {
                self.entry_rows.send(
                    row_position,
                    QuarantineEntryRowInput::RowStateChanged(
                        self.page_state.row_state(entry_row.entry_id()),
                    ),
                );
            });
    }

    fn listing_failure_text(&self) -> String {
        match self.page_state.listing() {
            QuarantineListing::ListingFailed(load_failure) => {
                describe_quarantine_failure(load_failure)
            }
            QuarantineListing::ListingLoading | QuarantineListing::ListingLoaded(_) => {
                String::new()
            }
        }
    }

    fn operation_failures_text(&self) -> String {
        self.page_state
            .operation_failures()
            .map(describe_quarantine_failure)
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn start_entry_operation(&mut self, entry_id: i64, operation_kind: EntryOperationKind) -> bool {
        let operation_started = self
            .page_state
            .begin_entry_operation(entry_id, operation_kind)
            .is_ok();
        self.send_row_states();
        operation_started
    }

    fn finish_entry_operation(
        &mut self,
        entry_id: i64,
        operation_outcome: Result<QuarantineEntryRecord, QuarantineMoveFailure>,
        sender: &ComponentSender<Self>,
    ) {
        self.page_state
            .finish_entry_operation(entry_id, operation_outcome);
        self.send_row_states();
        if !self.page_state.operations_running() {
            self.reload_entries(sender);
        }
    }

    fn present_restore_confirmation(
        &self,
        entry_id: i64,
        content_verdict: &ContentVerdict,
        root: &adw::ToastOverlay,
        sender: &ComponentSender<Self>,
    ) {
        let Some(entry_record) = self
            .page_state
            .listed_entries()
            .iter()
            .find(|listed_entry| listed_entry.entry_id == entry_id)
        else {
            sender.input(QuarantineEntriesPageInput::RestoreEntryCancelled { entry_id });
            return;
        };
        let RestoreConfirmationText {
            heading,
            verdict_text,
            confirm_label,
            restores_detected_file,
        } = restore_confirmation_text(content_verdict);
        let restore_dialog = adw::AlertDialog::new(
            Some(&heading),
            Some(
                &[
                    verdict_text,
                    translated_messages::findings::file_goes_back(),
                ]
                .join("\n\n"),
            ),
        );
        let destination_label = gtk::Label::builder()
            .label(sanitize_scanner_line(
                &entry_record.original_path.to_string_lossy(),
            ))
            .use_markup(false)
            .selectable(true)
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::Char)
            .css_classes(["monospace", "card"])
            .build();
        let destination_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
        destination_box.append(&destination_label);
        if let Some(destination_note) = restore_destination_note(entry_record) {
            destination_box.append(
                &gtk::Label::builder()
                    .label(destination_note)
                    .wrap(true)
                    .css_classes(["warning"])
                    .build(),
            );
        }
        restore_dialog.set_extra_child(Some(&destination_box));
        add_dismissing_choice::<RestoreDialogChoice>(
            &restore_dialog,
            &translated_messages::findings::cancel_quarantine(),
        );
        add_alert_choice(
            &restore_dialog,
            RestoreDialogChoice::RestoreChosen,
            &confirm_label,
        );
        restore_dialog.set_response_appearance(
            RestoreDialogChoice::RestoreChosen.response_id(),
            if restores_detected_file {
                adw::ResponseAppearance::Destructive
            } else {
                adw::ResponseAppearance::Suggested
            },
        );
        restore_dialog
            .set_default_response(Some(RestoreDialogChoice::RestoreCancelled.response_id()));
        deliver_local_outcome(
            sender.input_sender().clone(),
            alert_choice(restore_dialog, root.clone().upcast()),
            move |restore_choice| {
                Some(match restore_choice {
                    RestoreDialogChoice::RestoreChosen => {
                        QuarantineEntriesPageInput::RestoreEntryConfirmed { entry_id }
                    }
                    RestoreDialogChoice::RestoreCancelled => {
                        QuarantineEntriesPageInput::RestoreEntryCancelled { entry_id }
                    }
                })
            },
        );
    }

    fn start_recheck(&mut self, sender: &ComponentSender<Self>) {
        let quarantine_vault = self.quarantine_vault.clone();
        let _recheck_refused = self.page_state.start_recheck_run(|rechecked_ids| {
            let recheck_commands = stream::iter(rechecked_ids)
                .then(move |entry_id| {
                    let quarantine_vault = quarantine_vault.clone();
                    async move {
                        QuarantineEntriesPageCommand::RecheckVerdictArrived {
                            entry_id,
                            content_verdict: verdict_for_entry(&quarantine_vault, entry_id).await,
                        }
                    }
                })
                .chain(stream::iter([
                    QuarantineEntriesPageCommand::RecheckFinished,
                ]));
            run_stream(sender, recheck_commands)
        });
        self.send_row_states();
    }

    fn reload_entries(&self, sender: &ComponentSender<Self>) {
        let quarantine_vault = self.quarantine_vault.clone();
        sender.oneshot_command(async move {
            QuarantineEntriesPageCommand::EntriesLoaded(
                quarantine_vault.list_quarantine_entries().await,
            )
        });
    }

    fn old_entries(&self) -> Vec<&QuarantineEntryRecord> {
        let current_time = Timestamp::now();
        self.page_state
            .listed_entries()
            .iter()
            .filter(|entry_record| entry_is_older_than(entry_record, OLD_ENTRY_AGE, current_time))
            .collect()
    }

    fn total_stored_size(&self) -> String {
        byte_size_text(
            self.page_state
                .listed_entries()
                .iter()
                .map(|entry_record| entry_record.file_size)
                .sum(),
        )
    }

    fn entry_count_text(&self) -> String {
        let listed_count = self.page_state.listed_entries().len();
        translated_messages::findings::entry_count(listed_count)
    }

    fn show_matching_entries(&mut self) {
        let mut entry_rows_guard = self.entry_rows.guard();
        entry_rows_guard.clear();
        self.page_state
            .matching_rows(&self.search_text)
            .into_iter()
            .for_each(|row_init| {
                entry_rows_guard.push_back(row_init);
            });
    }

    fn confirm_clearing_old_entries(
        &self,
        root: &adw::ToastOverlay,
        sender: &ComponentSender<Self>,
    ) {
        let old_entry_count = self.old_entries().len();
        let confirmation_dialog = adw::AlertDialog::new(
            Some(&translated_messages::findings::clear_old_items()),
            Some(&translated_messages::findings::count_quarantined_file_older(old_entry_count)),
        );
        add_deletion_choices(&confirmation_dialog);
        deliver_local_outcome(
            sender.input_sender().clone(),
            alert_choice(confirmation_dialog, root.clone().upcast()),
            |deletion_choice| match deletion_choice {
                DeletionDialogChoice::DeletionConfirmed => {
                    Some(QuarantineEntriesPageInput::ClearOldEntriesConfirmed)
                }
                DeletionDialogChoice::DeletionCancelled => None,
            },
        );
    }

    fn confirm_entry_deletion(
        &self,
        entry_id: i64,
        root: &adw::ToastOverlay,
        sender: &ComponentSender<Self>,
    ) {
        let confirmation_dialog = adw::AlertDialog::new(
            Some(&translated_messages::findings::delete_quarantined_file()),
            Some(&translated_messages::findings::file_permanently_deleted()),
        );
        add_deletion_choices(&confirmation_dialog);
        deliver_local_outcome(
            sender.input_sender().clone(),
            alert_choice(confirmation_dialog, root.clone().upcast()),
            move |deletion_choice| match deletion_choice {
                DeletionDialogChoice::DeletionConfirmed => {
                    Some(QuarantineEntriesPageInput::DeleteEntryConfirmed { entry_id })
                }
                DeletionDialogChoice::DeletionCancelled => None,
            },
        );
    }
}

#[relm4::component(pub)]
impl Component for QuarantineEntriesPage {
    type Init = QuarantineEntriesPageInit;
    type Input = QuarantineEntriesPageInput;
    type Output = QuarantineEntriesPageOutput;
    type CommandOutput = QuarantineEntriesPageCommand;

    view! {
        adw::ToastOverlay {
            add_controller = gtk::ShortcutController {
                set_scope: gtk::ShortcutScope::Managed,
                add_shortcut: gtk::Shortcut::new(
                    gtk::ShortcutTrigger::parse_string("<Control>f"),
                    Some(gtk::CallbackAction::new({
                        let shortcut_sender = sender.input_sender().clone();
                        move |_shortcut_widget, _arguments| {
                            shortcut_sender.emit(QuarantineEntriesPageInput::SearchModeToggled(true));
                            gtk::glib::Propagation::Stop
                        }
                    })),
                ),
            },

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: &translated_messages::findings::quarantine_title(),
                        #[watch]
                        set_subtitle: &model.entry_count_text(),
                    },

                    pack_end = &gtk::MenuButton {
                        set_icon_name: "view-more-symbolic",
                        set_tooltip_text: Some(&translated_messages::findings::more_tooltip()),
                        #[wrap(Some)]
                        set_popover = &gtk::Popover {
                            gtk::Button {
                                set_label: &translated_messages::findings::clear_old_items_label(),
                                add_css_class: "flat",
                                #[watch]
                                set_sensitive: !model.old_entries().is_empty(),
                                connect_clicked[sender] => move |clicked_button| {
                                    if let Some(enclosing_popover) = clicked_button
                                        .ancestor(gtk::Popover::static_type())
                                        .and_downcast::<gtk::Popover>()
                                    {
                                        enclosing_popover.popdown();
                                    }
                                    sender.input(QuarantineEntriesPageInput::ClearOldEntriesRequested);
                                },
                            },
                        },
                    },

                    pack_end = &gtk::Button {
                        set_label: &translated_messages::findings::check_again_label(),
                        set_tooltip_text: Some(&translated_messages::findings::scan_every_quarantined_tooltip()),
                        #[watch]
                        set_sensitive: !model.page_state.recheck_running()
                            && !model.page_state.listed_entries().is_empty(),
                        connect_clicked => QuarantineEntriesPageInput::RecheckAllRequested,
                    },

                    pack_end = &gtk::Button {
                        set_icon_name: "view-refresh-symbolic",
                        set_tooltip_text: Some(&translated_messages::findings::refresh_tooltip()),
                        connect_clicked => QuarantineEntriesPageInput::ReloadEntriesRequested,
                    },

                    pack_end = &gtk::ToggleButton {
                        set_icon_name: "system-search-symbolic",
                        set_tooltip_text: Some(&translated_messages::findings::search_tooltip()),
                        #[watch]
                        #[block_signal(search_toggle_handler)]
                        set_active: model.search_mode_enabled,
                        connect_toggled[sender] => move |search_toggle| {
                            sender.input(QuarantineEntriesPageInput::SearchModeToggled(search_toggle.is_active()));
                        } @search_toggle_handler,
                    },
                },

                add_top_bar = &gtk::SearchBar {
                    set_key_capture_widget: Some(&root),
                    #[watch]
                    #[block_signal(search_bar_handler)]
                    set_search_mode: model.search_mode_enabled,
                    connect_search_mode_enabled_notify[sender] => move |search_bar| {
                        sender.input(QuarantineEntriesPageInput::SearchModeToggled(search_bar.is_search_mode()));
                    } @search_bar_handler,

                    #[wrap(Some)]
                    set_child = &gtk::SearchEntry {
                        set_placeholder_text: Some(&translated_messages::findings::search_threat_name_placeholder()),
                        connect_search_changed[sender] => move |search_entry| {
                            sender.input(QuarantineEntriesPageInput::SearchTextChanged(
                                search_entry.text().to_string(),
                            ));
                        },
                    },
                },

                #[wrap(Some)]
                #[template]
                set_content = &ClampedPage {
                    #[template_child]
                    page_content {
                        adw::PreferencesGroup {
                            set_title: &translated_messages::findings::quarantine_storage_title(),
                            set_description: Some(&translated_messages::findings::detected_threats_isolated_description()),

                            adw::ActionRow {
                                set_title: &translated_messages::findings::total_size_title(),
                                #[watch]
                                set_subtitle: &model.total_stored_size(),
                                add_prefix = &gtk::Image { set_icon_name: Some("drive-harddisk-symbolic") },
                            },
                        },

                        gtk::Box {
                            set_spacing: 12,
                            #[watch]
                            set_visible: model.page_state.operation_failures().next().is_some(),

                            gtk::Label {
                                set_xalign: 0.0,
                                set_hexpand: true,
                                set_wrap: true,
                                set_selectable: true,
                                add_css_class: "error",
                                #[watch]
                                set_label: &model.operation_failures_text(),
                            },

                            gtk::Button {
                                set_label: &translated_messages::findings::dismiss_label(),
                                set_valign: gtk::Align::Center,
                                add_css_class: "flat",
                                connect_clicked => QuarantineEntriesPageInput::OperationFailuresDismissed,
                            },
                        },

                        #[local_ref]
                        entry_list -> gtk::ListBox {
                            add_css_class: "boxed-list",
                            set_selection_mode: gtk::SelectionMode::None,
                            #[wrap(Some)]
                            #[local_ref]
                            set_placeholder = placeholder_stack -> gtk::Stack {
                                add_named[Some(LOAD_FAILED_PAGE_NAME)] = &adw::ActionRow {
                                    set_title: &translated_messages::findings::quarantine_cannot_read_title(),
                                    set_use_markup: false,
                                    set_subtitle_lines: 0,
                                    add_css_class: "dim-label",
                                    #[watch]
                                    set_subtitle: &model.listing_failure_text(),
                                },
                                #[watch]
                                set_visible_child_name: model.page_state.listing().placeholder_page_name(),
                            },
                        },
                    },
                },
            },
        }
    }

    fn init(
        QuarantineEntriesPageInit {
            quarantine_vault,
            activity_history,
        }: QuarantineEntriesPageInit,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let entry_rows = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .forward(sender.input_sender(), |row_output| match row_output {
                QuarantineEntryRowOutput::RestoreRequested { entry_id } => {
                    QuarantineEntriesPageInput::RestoreEntryRequested { entry_id }
                }
                QuarantineEntryRowOutput::DeletionRequested { entry_id } => {
                    QuarantineEntriesPageInput::DeleteEntryRequested { entry_id }
                }
                QuarantineEntryRowOutput::ReputationCheckRequested {
                    entry_id,
                    content_hash,
                } => QuarantineEntriesPageInput::ReputationCheckRequested {
                    entry_id,
                    content_hash,
                },
                QuarantineEntryRowOutput::HashCopyRequested(file_hash) => {
                    QuarantineEntriesPageInput::HashCopyRequested(file_hash)
                }
                QuarantineEntryRowOutput::ReportLinkRequested(report_url) => {
                    QuarantineEntriesPageInput::ReportLinkRequested(report_url)
                }
                QuarantineEntryRowOutput::TrustRequested(trust_choice) => {
                    QuarantineEntriesPageInput::TrustRequested(trust_choice)
                }
                QuarantineEntryRowOutput::FalsePositiveReportRequested(reported_detection) => {
                    QuarantineEntriesPageInput::FalsePositiveReportRequested(reported_detection)
                }
                QuarantineEntryRowOutput::SubmissionRequested { entry_id } => {
                    QuarantineEntriesPageInput::SubmissionRequested { entry_id }
                }
            });
        let model = Self {
            quarantine_vault,
            activity_history,
            page_state: QuarantinePageState::new(),
            search_text: String::new(),
            entry_rows,
            search_mode_enabled: false,
            toaster: Toaster::new(&root),
        };
        model.reload_entries(&sender);
        let entry_list = model.entry_rows.widget();
        let placeholder_stack = &loading_placeholder_stack(
            &translated_messages::findings::loading_quarantine(),
            &empty_placeholder_row(&translated_messages::findings::quarantine_empty()),
        );
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        page_input: QuarantineEntriesPageInput,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        let quarantine_vault = self.quarantine_vault.clone();
        match page_input {
            QuarantineEntriesPageInput::HashCopyRequested(file_hash) => {
                root.clipboard().set_text(&file_hash);
                self.toaster.add_toast(plain_text_toast(
                    &translated_messages::findings::hash_copied_clipboard(),
                ));
            }
            QuarantineEntriesPageInput::ReportLinkRequested(report_url) => {
                gtk::UriLauncher::new(&report_url).launch(
                    root.root().and_downcast::<gtk::Window>().as_ref(),
                    gtk::gio::Cancellable::NONE,
                    |_launch_outcome| {},
                );
            }
            QuarantineEntriesPageInput::SearchModeToggled(search_mode_enabled) => {
                self.search_mode_enabled = search_mode_enabled;
            }
            QuarantineEntriesPageInput::TrustRequested(trust_choice) => {
                present_trust_confirmation(
                    root,
                    trust_choice,
                    sender.input_sender().clone(),
                    QuarantineEntriesPageInput::TrustConfirmed,
                );
            }
            QuarantineEntriesPageInput::TrustConfirmed(confirmed_choice) => {
                sender.oneshot_command(async move {
                    QuarantineEntriesPageCommand::TrustChangeFinished(
                        request_trust_change(confirmed_choice).await,
                    )
                });
            }
            QuarantineEntriesPageInput::FalsePositiveReportRequested(report_proposal) => {
                present_clamav_report_confirmation(
                    root,
                    report_proposal,
                    sender.input_sender().clone(),
                    QuarantineEntriesPageInput::FalsePositiveReportConfirmed,
                );
            }
            QuarantineEntriesPageInput::FalsePositiveReportConfirmed(report_request) => {
                let _shell_closed = sender.output(
                    QuarantineEntriesPageOutput::ClamavReportConfirmed(report_request),
                );
            }
            QuarantineEntriesPageInput::SubmissionRequested { entry_id } => {
                let _shell_closed =
                    sender.output(QuarantineEntriesPageOutput::SubmissionRequested { entry_id });
            }
            QuarantineEntriesPageInput::ReloadEntriesRequested => self.reload_entries(&sender),
            QuarantineEntriesPageInput::RestoreEntryRequested { entry_id } => {
                if !self.start_entry_operation(entry_id, EntryOperationKind::VerifyingBeforeRestore)
                {
                    return;
                }
                sender.oneshot_command(async move {
                    QuarantineEntriesPageCommand::RestoreCheckFinished {
                        entry_id,
                        content_verdict: verdict_for_entry(&quarantine_vault, entry_id).await,
                    }
                });
            }
            QuarantineEntriesPageInput::RecheckAllRequested => self.start_recheck(&sender),
            QuarantineEntriesPageInput::RestoreEntryConfirmed { entry_id } => {
                if self.page_state.confirm_verified_restore(entry_id).is_err() {
                    return;
                }
                self.send_row_states();
                run_stream_to_completion(
                    &sender,
                    stream::once(async move {
                        QuarantineEntriesPageCommand::EntryOperationFinished {
                            entry_id,
                            operation_outcome: quarantine_vault
                                .restore_quarantined_file(entry_id)
                                .await,
                        }
                    }),
                );
            }
            QuarantineEntriesPageInput::RestoreEntryCancelled { entry_id } => {
                self.page_state.cancel_verified_restore(entry_id);
                self.send_row_states();
            }
            QuarantineEntriesPageInput::OperationFailuresDismissed => {
                self.page_state.dismiss_operation_failures();
            }
            QuarantineEntriesPageInput::ReputationCheckRequested {
                entry_id,
                content_hash,
            } => {
                if self.page_state.begin_reputation_check(entry_id).is_err() {
                    return;
                }
                self.send_row_states();
                sender.oneshot_command(async move {
                    QuarantineEntriesPageCommand::ReputationCheckFinished {
                        entry_id,
                        reputation_answers: check_hash_reputation(content_hash).await,
                    }
                });
            }
            QuarantineEntriesPageInput::SearchTextChanged(search_text) => {
                self.search_text = search_text;
                self.show_matching_entries();
            }
            QuarantineEntriesPageInput::ClearOldEntriesRequested => {
                self.confirm_clearing_old_entries(root, &sender);
            }
            QuarantineEntriesPageInput::ClearOldEntriesConfirmed => {
                let old_entry_ids = self
                    .old_entries()
                    .into_iter()
                    .map(|entry_record| entry_record.entry_id)
                    .collect::<Vec<_>>();
                let started_entry_ids = old_entry_ids
                    .into_iter()
                    .filter(|entry_id| {
                        self.page_state
                            .begin_entry_operation(*entry_id, EntryOperationKind::DeletingEntry)
                            .is_ok()
                    })
                    .collect::<Vec<_>>();
                self.send_row_states();
                run_stream_to_completion(
                    &sender,
                    stream::iter(started_entry_ids).then(move |entry_id| {
                        let quarantine_vault = quarantine_vault.clone();
                        async move {
                            QuarantineEntriesPageCommand::EntryOperationFinished {
                                entry_id,
                                operation_outcome: quarantine_vault
                                    .delete_quarantined_file(entry_id)
                                    .await,
                            }
                        }
                    }),
                );
            }
            QuarantineEntriesPageInput::DeleteEntryRequested { entry_id } => {
                self.confirm_entry_deletion(entry_id, root, &sender);
            }
            QuarantineEntriesPageInput::DeleteEntryConfirmed { entry_id } => {
                if !self.start_entry_operation(entry_id, EntryOperationKind::DeletingEntry) {
                    return;
                }
                run_stream_to_completion(
                    &sender,
                    stream::once(async move {
                        QuarantineEntriesPageCommand::EntryOperationFinished {
                            entry_id,
                            operation_outcome: quarantine_vault
                                .delete_quarantined_file(entry_id)
                                .await,
                        }
                    }),
                );
            }
        }
    }

    fn update_cmd(
        &mut self,
        page_command: QuarantineEntriesPageCommand,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match page_command {
            QuarantineEntriesPageCommand::RestoreCheckFinished {
                entry_id,
                content_verdict,
            } => {
                self.page_state
                    .record_recheck_verdict(entry_id, content_verdict.clone());
                self.send_row_states();
                self.present_restore_confirmation(entry_id, &content_verdict, root, &sender);
            }
            QuarantineEntriesPageCommand::RecheckVerdictArrived {
                entry_id,
                content_verdict,
            } => {
                self.page_state
                    .record_recheck_verdict(entry_id, content_verdict);
                self.send_row_states();
            }
            QuarantineEntriesPageCommand::ReputationCheckFinished {
                entry_id,
                reputation_answers,
            } => {
                self.page_state
                    .record_reputation_answers(entry_id, reputation_answers);
                self.send_row_states();
            }
            QuarantineEntriesPageCommand::RecheckFinished => {
                let rechecked_contents = self.page_state.rechecked_contents();
                let activity_history = self.activity_history.clone();
                sender.oneshot_command(async move {
                    QuarantineEntriesPageCommand::ClamavReviewsSettled(
                        settle_clamav_reviews(
                            &activity_history,
                            &rechecked_contents,
                            current_database_version().await,
                        )
                        .await,
                    )
                });
                let cleared_count = self.page_state.finish_recheck_run();
                self.toaster
                    .add_toast(plain_text_toast(&if cleared_count == 0 {
                        translated_messages::findings::every_quarantined_file_still()
                    } else {
                        translated_messages::findings::count_quarantined_file_no(cleared_count)
                    }));
            }
            QuarantineEntriesPageCommand::ClamavReviewsSettled(Ok(0)) => {}
            QuarantineEntriesPageCommand::ClamavReviewsSettled(Ok(settled_count)) => {
                self.toaster.add_toast(plain_text_toast(
                    &translated_messages::findings::count_report_clamav_settled(settled_count),
                ));
                let _shell_closed = sender.output(QuarantineEntriesPageOutput::SubmissionsChanged);
            }
            QuarantineEntriesPageCommand::ClamavReviewsSettled(Err(history_failure)) => {
                log_failure_to_journal("settling ClamAV reviews failed", &history_failure);
                self.toaster
                    .add_toast(plain_text_toast(&history_failure_text(&history_failure)));
            }
            QuarantineEntriesPageCommand::TrustChangeFinished(trust_outcome) => {
                self.toaster
                    .add_toast(plain_text_toast(&trust_outcome_text(&trust_outcome)));
            }
            QuarantineEntriesPageCommand::EntriesLoaded(load_outcome) => {
                log_failed_outcome("loading the quarantine failed", &load_outcome);
                self.page_state.record_loaded_entries(load_outcome);
                self.show_matching_entries();
                let _shell_closed = sender.output(QuarantineEntriesPageOutput::EntriesCounted(
                    self.page_state.listing().entry_count(),
                ));
            }
            QuarantineEntriesPageCommand::EntryOperationFinished {
                entry_id,
                operation_outcome,
            } => self.finish_entry_operation(entry_id, operation_outcome, &sender),
        }
    }
}

#[cfg(test)]
mod quarantine_entries_page_tests {
    use std::path::PathBuf;

    use super::*;
    use crate::host_services::{
        clamav_scanner::scanner_output_parser::ThreatIdentity,
        quarantine_vault::quarantine_file_moves::RecordedPayloadHash,
    };

    fn entry_detected_on(detection_date: &str) -> QuarantineEntryRecord {
        QuarantineEntryRecord {
            entry_id: 1,
            original_path: PathBuf::from("/home/user/Downloads/Invoice.exe"),
            quarantine_path: PathBuf::from("/vault/payload"),
            threat_identity: ThreatIdentity::Signature {
                signature_name: "Win.Trojan.Agent".to_owned(),
            },
            detection_date: StoredDetectionDate::from(detection_date.to_owned()),
            file_size: bytesize::ByteSize::b(1),
            payload_hash: RecordedPayloadHash::IntegrityUnknown,
            original_permissions: 0o644,
        }
    }

    #[test]
    fn search_matches_threat_names_and_paths_case_insensitively() {
        let entry_record = entry_detected_on("2026-11-01T00:00:00Z");
        assert!(entry_matches_search(&entry_record, "trojan"));
        assert!(entry_matches_search(&entry_record, " invoice "));
        assert!(entry_matches_search(&entry_record, ""));
        assert!(!entry_matches_search(&entry_record, "ransom"));
    }

    #[test]
    fn age_is_compared_with_the_detection_date() {
        let current_time: Timestamp = "2026-11-05T00:00:00Z"
            .parse()
            .expect("fixture current time");
        assert!(entry_is_older_than(
            &entry_detected_on("2026-10-01T00:00:00.000Z"),
            OLD_ENTRY_AGE,
            current_time
        ));
        assert!(!entry_is_older_than(
            &entry_detected_on("2026-11-01T00:00:00.000Z"),
            OLD_ENTRY_AGE,
            current_time
        ));
    }

    #[test]
    fn an_unparseable_date_is_never_treated_as_old() {
        assert!(!entry_is_older_than(
            &entry_detected_on("not a date"),
            OLD_ENTRY_AGE,
            Timestamp::now()
        ));
    }
}
