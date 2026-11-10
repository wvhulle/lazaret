use clamd_configuration::protection_rules::ProtectionReduction;
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, adw,
    adw::prelude::*, gtk, gtk::glib, prelude::FactoryVecDeque,
};

use crate::{
    host_services::{
        clamav_admin_service::live_settings_client::{
            LiveChangeFailure, LiveChangeOutcome, LiveTuning, OverrideSetting, apply_live_change,
            change_confirmation_text, change_effect, read_live_tuning, reset_live_settings,
        },
        clamav_config::{
            clamav_host_config::{
                ConfigSetting, HostConfigView, is_on_access_setting, read_clamd_config,
                read_freshclam_config,
            },
            detection_options::{
                DETECTION_OPTIONS, DetectionOption, configured_values, effective_switch,
            },
            setting_provenance::{
                ClamavConfigKind, ConfigFileOwner, RuntimeTuningState, change_snippet,
                clamav_source, inspect_config_file_owner, managed_config_description,
                values_with_edit,
            },
            typed_setting_values::{SwitchPosition, TypedSettingValue, parse_setting_value},
        },
        clamav_scanner::scanner_output_parser::sanitize_scanner_line,
        desktop::flatpak_sandbox::SandboxKind,
    },
    shared_widgets::{
        awaited_dialogs::{
            AlertChoice, add_alert_choice, add_dismissing_choice, alert_choice,
            deliver_local_outcome,
        },
        engine_settings::config_line_copy_dialog::{
            ConfigLineCopyDialog, ConfigLineCopyDialogOutput, ConfigLineCopyInit,
        },
        setting_row::{
            setting_row_state::{
                ClamavSettingAddress, ClamavSource, CopyOnlyReason, FileEntry, FollowUpAction,
                RootOwnedConfigFile, RowChangeActivity, SettingLabel, SettingRowState,
                SettingSubject, ShownSettingValue,
            },
            setting_row_widget::{SettingRow, SettingRowInput, SettingRowOutput},
        },
    },
    translated_messages,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EngineSettingsSection {
    ScannerEngine,
    OnAccessScanning,
    SignatureUpdates,
}

impl EngineSettingsSection {
    fn group_title(self) -> String {
        match self {
            Self::ScannerEngine => translated_messages::widgets::clamd_settings(),
            Self::OnAccessScanning => translated_messages::widgets::access_scanning_settings(),
            Self::SignatureUpdates => translated_messages::widgets::update_settings(),
        }
    }

    const fn config_kind(self) -> ClamavConfigKind {
        match self {
            Self::ScannerEngine | Self::OnAccessScanning => ClamavConfigKind::ClamdDaemonConfig,
            Self::SignatureUpdates => ClamavConfigKind::FreshclamUpdaterConfig,
        }
    }

    const fn offers_live_tuning(self) -> bool {
        matches!(self, Self::ScannerEngine | Self::OnAccessScanning)
    }

    fn read_host_config(self, sandbox_kind: SandboxKind) -> HostConfigView {
        match self {
            Self::ScannerEngine | Self::OnAccessScanning => read_clamd_config(sandbox_kind),
            Self::SignatureUpdates => read_freshclam_config(sandbox_kind),
        }
    }

    fn shows_setting(self, config_setting: &ConfigSetting) -> bool {
        match self {
            Self::ScannerEngine => !is_on_access_setting(config_setting),
            Self::OnAccessScanning => is_on_access_setting(config_setting),
            Self::SignatureUpdates => true,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EngineSettingsLayout {
    SingleSection(EngineSettingsSection),
    EngineWithDetection,
}

impl EngineSettingsLayout {
    const fn config_section(self) -> EngineSettingsSection {
        match self {
            Self::SingleSection(settings_section) => settings_section,
            Self::EngineWithDetection => EngineSettingsSection::ScannerEngine,
        }
    }

    pub const fn shows_detection_switches(self) -> bool {
        matches!(self, Self::EngineWithDetection)
    }

    fn lists_in_config_group(self, config_setting: &ConfigSetting) -> bool {
        self.config_section().shows_setting(config_setting)
            && !(self.shows_detection_switches()
                && detection_option_for_key(&config_setting.setting_key).is_some())
    }
}

fn detection_option_for_key(setting_key: &str) -> Option<DetectionOption> {
    DETECTION_OPTIONS
        .into_iter()
        .find(|detection_option| detection_option.setting_key() == setting_key)
}

enum ConfigLoad {
    LoadingConfig,
    ConfigShown {
        config_path: String,
        config_file_owner: ConfigFileOwner,
    },
    ConfigMissing,
}

#[derive(Debug)]
pub enum LiveTuningLoad {
    TuningNotOffered,
    LoadingTuning,
    TuningRead(LiveTuning),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChangedRow {
    ConfigRow {
        setting_key: String,
        value_position: usize,
    },
    DetectionSwitch(DetectionOption),
    WholeSetting {
        setting_key: String,
    },
}

impl ChangedRow {
    fn covers_row(&self, setting_key: &str, value_position: usize) -> bool {
        match self {
            Self::ConfigRow {
                setting_key: changed_key,
                value_position: changed_position,
            } => changed_key == setting_key && *changed_position == value_position,
            Self::DetectionSwitch(detection_option) => {
                detection_option.setting_key() == setting_key
            }
            Self::WholeSetting {
                setting_key: changed_key,
            } => changed_key == setting_key,
        }
    }
}

enum CopyDialogState {
    NoCopyDialog,
    CopyDialogOpen(Controller<ConfigLineCopyDialog>),
}

#[derive(Debug)]
enum ChangeProgress {
    NoChangeRunning,
    ChangeRunning { changed_row: ChangedRow },
}

impl ChangeProgress {
    fn row_change_activity(&self, setting_key: &str, value_position: usize) -> RowChangeActivity {
        match self {
            Self::NoChangeRunning => RowChangeActivity::ChangesAllowed,
            Self::ChangeRunning { changed_row } => {
                if changed_row.covers_row(setting_key, value_position) {
                    RowChangeActivity::ApplyingThisRow
                } else {
                    RowChangeActivity::ApplyingOtherRow
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LiveChangeReadiness {
    ReadyToChange,
    ChangeAlreadyRunning,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ConfirmationVerdict {
    StartConfirmedChange,
    DropStaleConfirmation,
}

const fn live_change_readiness(change_progress: &ChangeProgress) -> LiveChangeReadiness {
    match change_progress {
        ChangeProgress::ChangeRunning { .. } => LiveChangeReadiness::ChangeAlreadyRunning,
        ChangeProgress::NoChangeRunning => LiveChangeReadiness::ReadyToChange,
    }
}

const fn confirmation_verdict(change_readiness: LiveChangeReadiness) -> ConfirmationVerdict {
    match change_readiness {
        LiveChangeReadiness::ReadyToChange => ConfirmationVerdict::StartConfirmedChange,
        LiveChangeReadiness::ChangeAlreadyRunning => ConfirmationVerdict::DropStaleConfirmation,
    }
}

pub struct EngineSettingsSectionsInit {
    pub settings_layout: EngineSettingsLayout,
    pub sandbox_kind: SandboxKind,
}

pub struct EngineSettingsSections {
    sandbox_kind: SandboxKind,
    settings_layout: EngineSettingsLayout,
    settings_section: EngineSettingsSection,
    config_load: ConfigLoad,
    tuning_load: LiveTuningLoad,
    change_progress: ChangeProgress,
    setting_rows: FactoryVecDeque<SettingRow>,
    detection_rows: FactoryVecDeque<SettingRow>,
    configured_settings: Vec<ConfigSetting>,
    copy_dialog: CopyDialogState,
}

#[derive(Debug)]
pub enum EngineSettingsNotice {
    ChangeSnippetCopied,
    StaleConfirmationDropped,
    LiveChangeReported(LiveChangeOutcome),
}

#[derive(Debug)]
pub enum EngineSettingsSectionsOutput {
    ToastRequested(EngineSettingsNotice),
}

#[derive(Debug)]
pub enum EngineSettingsSectionsInput {
    SettingRowReported(SettingRowOutput),
    ConfigLineCopied(String),
    CopyDialogClosed,
    LiveChangeConfirmed {
        changed_row: ChangedRow,
        setting_key: String,
        new_values: Vec<String>,
    },
}

#[derive(Debug)]
pub enum EngineSettingsSectionsCommand {
    ConfigLoaded {
        host_config: HostConfigView,
        config_file_owner: ConfigFileOwner,
        tuning_load: LiveTuningLoad,
    },
    LiveChangeFinished {
        change_outcome: LiveChangeOutcome,
        live_tuning: LiveTuning,
    },
}

pub fn describe_engine_settings_notice(settings_notice: &EngineSettingsNotice) -> String {
    match settings_notice {
        EngineSettingsNotice::ChangeSnippetCopied => {
            translated_messages::widgets::change_snippet_copied()
        }
        EngineSettingsNotice::StaleConfirmationDropped => {
            translated_messages::widgets::stale_confirmation_dropped()
        }
        EngineSettingsNotice::LiveChangeReported(change_outcome) => {
            describe_live_change_outcome(change_outcome)
        }
    }
}

pub fn engine_settings_notice_toast(settings_notice: &EngineSettingsNotice) -> adw::Toast {
    adw::Toast::builder()
        .title(describe_engine_settings_notice(settings_notice))
        .use_markup(false)
        .build()
}

fn describe_live_change_outcome(change_outcome: &LiveChangeOutcome) -> String {
    match change_outcome {
        LiveChangeOutcome::SettingsApplied => translated_messages::widgets::settings_applied(),
        LiveChangeOutcome::PreviousRestored => translated_messages::widgets::previous_restored(),
        LiveChangeOutcome::NothingChanged(change_failure) => {
            live_change_failure_text(change_failure)
        }
    }
}

fn live_change_failure_text(change_failure: &LiveChangeFailure) -> String {
    let change_not_made = |failure_reason: &str| {
        translated_messages::widgets::change_not_made_error(sanitize_scanner_line(failure_reason))
    };
    match change_failure {
        LiveChangeFailure::AuthorizationRefused => {
            translated_messages::widgets::authorization_refused()
        }
        LiveChangeFailure::AdminServiceMissing => {
            translated_messages::widgets::admin_service_missing()
        }
        LiveChangeFailure::UnknownAdminServiceAnswer {
            admin_service_answer,
        } => change_not_made(admin_service_answer),
        LiveChangeFailure::ReportedByAdminService {
            admin_service_message,
        } => change_not_made(admin_service_message),
        LiveChangeFailure::InvalidRequest { request_problem } => change_not_made(request_problem),
        LiveChangeFailure::AdminServiceCallFailed(call_failure) => {
            change_not_made(&call_failure.to_string())
        }
    }
}

struct ProposedLiveChange {
    changed_row: ChangedRow,
    setting_key: String,
    new_values: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SettingChangeChoice {
    ChangeApplied,
    ChangeCancelled,
}

impl AlertChoice for SettingChangeChoice {
    const ANSWERING_CHOICES: &'static [Self] = &[Self::ChangeApplied];
    const DISMISSING_CHOICE: Self = Self::ChangeCancelled;

    fn response_id(self) -> &'static str {
        match self {
            Self::ChangeApplied => "apply",
            Self::ChangeCancelled => "cancel",
        }
    }
}

fn present_live_change_confirmation(
    parent_widget: &impl IsA<gtk::Widget>,
    proposed_change: ProposedLiveChange,
    current_values: &[String],
    sender: &ComponentSender<EngineSettingsSections>,
) {
    let ProposedLiveChange {
        changed_row,
        setting_key,
        new_values,
    } = proposed_change;
    let effect = change_effect(&setting_key, current_values, &new_values);
    let confirmation_dialog = adw::AlertDialog::new(
        Some(&translated_messages::widgets::change_setting()),
        Some(&change_confirmation_text(
            &setting_key,
            current_values,
            &new_values,
            effect,
        )),
    );
    add_dismissing_choice::<SettingChangeChoice>(
        &confirmation_dialog,
        &translated_messages::widgets::cancel_engine(),
    );
    add_alert_choice(
        &confirmation_dialog,
        SettingChangeChoice::ChangeApplied,
        &if effect != ProtectionReduction::NoReduction {
            translated_messages::widgets::reduce_protection()
        } else {
            translated_messages::widgets::apply_label()
        },
    );
    if effect != ProtectionReduction::NoReduction {
        confirmation_dialog.set_response_appearance(
            SettingChangeChoice::ChangeApplied.response_id(),
            adw::ResponseAppearance::Destructive,
        );
    }
    confirmation_dialog
        .set_default_response(Some(SettingChangeChoice::ChangeCancelled.response_id()));
    deliver_local_outcome(
        sender.input_sender().clone(),
        alert_choice(confirmation_dialog, parent_widget.clone().upcast()),
        move |change_choice| match change_choice {
            SettingChangeChoice::ChangeApplied => {
                Some(EngineSettingsSectionsInput::LiveChangeConfirmed {
                    changed_row,
                    setting_key,
                    new_values,
                })
            }
            SettingChangeChoice::ChangeCancelled => None,
        },
    );
}

fn detection_title(detection_option: DetectionOption) -> String {
    match detection_option {
        DetectionOption::PotentiallyUnwantedPrograms => {
            translated_messages::widgets::detect_potentially_unwanted_programs()
        }
        DetectionOption::HeuristicAlerts => translated_messages::widgets::heuristic_detections(),
        DetectionOption::OfficeMacros => translated_messages::widgets::flag_office_files_macros(),
        DetectionOption::EncryptedContent => {
            translated_messages::widgets::flag_encrypted_archives_documents()
        }
        DetectionOption::ArchiveContents => translated_messages::widgets::look_inside_archives(),
        DetectionOption::OversizedFiles => translated_messages::widgets::flag_files_too_large(),
    }
}

fn detection_description(detection_option: DetectionOption) -> String {
    match detection_option {
        DetectionOption::PotentiallyUnwantedPrograms => {
            translated_messages::widgets::adware_hacking_tools_remote()
        }
        DetectionOption::HeuristicAlerts => {
            translated_messages::widgets::suspicious_files_no_signature()
        }
        DetectionOption::OfficeMacros => translated_messages::widgets::macros_common_way_deliver(),
        DetectionOption::EncryptedContent => {
            translated_messages::widgets::clamav_cannot_look_inside()
        }
        DetectionOption::ArchiveContents => translated_messages::widgets::checks_files_inside_zip(),
        DetectionOption::OversizedFiles => {
            translated_messages::widgets::otherwise_files_above_size()
        }
    }
}

fn shown_value_of(setting_key: &str, setting_value: &str) -> ShownSettingValue {
    parse_setting_value(setting_key, setting_value).map_or_else(
        |parse_failure| ShownSettingValue::UnreadableValue {
            raw_text: setting_value.to_owned(),
            parse_failure,
        },
        ShownSettingValue::ReadableValue,
    )
}

fn replace_row_states(
    row_factory: &mut FactoryVecDeque<SettingRow>,
    row_states: Vec<SettingRowState>,
) {
    let same_rows = row_factory.len() == row_states.len()
        && row_states
            .iter()
            .enumerate()
            .all(|(row_position, row_state)| {
                row_factory.get(row_position).is_some_and(|setting_row| {
                    setting_row.row_state().setting_label == row_state.setting_label
                        && setting_row
                            .row_state()
                            .setting_subject
                            .same_setting_as(&row_state.setting_subject)
                })
            });
    if same_rows {
        row_states
            .into_iter()
            .enumerate()
            .for_each(|(row_position, row_state)| {
                row_factory.send(
                    row_position,
                    SettingRowInput::StateReplaced(Box::new(row_state)),
                );
            });
        return;
    }
    let mut row_guard = row_factory.guard();
    row_guard.clear();
    row_states.into_iter().for_each(|row_state| {
        row_guard.push_back(row_state);
    });
}

impl EngineSettingsSections {
    fn report_notice(sender: &ComponentSender<Self>, settings_notice: EngineSettingsNotice) {
        let _host_closed = sender.output(EngineSettingsSectionsOutput::ToastRequested(
            settings_notice,
        ));
    }

    fn active_overrides(&self) -> &[OverrideSetting] {
        match &self.tuning_load {
            LiveTuningLoad::TuningRead(LiveTuning::Available {
                active_overrides, ..
            }) => active_overrides,
            LiveTuningLoad::TuningRead(
                LiveTuning::AdminServiceNotInstalled | LiveTuning::AdminServiceUnreachable(_),
            )
            | LiveTuningLoad::TuningNotOffered
            | LiveTuningLoad::LoadingTuning => &[],
        }
    }

    fn tuning_state(&self) -> RuntimeTuningState<'_> {
        if !self.settings_section.offers_live_tuning() {
            return RuntimeTuningState::TuningMissing(CopyOnlyReason::OutsideAllowlist);
        }
        match &self.tuning_load {
            LiveTuningLoad::TuningRead(LiveTuning::Available {
                tunable_settings,
                active_overrides,
            }) => RuntimeTuningState::TuningAvailable {
                tunable_settings,
                active_overrides,
            },
            LiveTuningLoad::TuningRead(LiveTuning::AdminServiceUnreachable(_)) => {
                RuntimeTuningState::TuningMissing(CopyOnlyReason::AdminServiceUnreachable)
            }
            LiveTuningLoad::TuningRead(LiveTuning::AdminServiceNotInstalled)
            | LiveTuningLoad::TuningNotOffered
            | LiveTuningLoad::LoadingTuning => {
                RuntimeTuningState::TuningMissing(CopyOnlyReason::AdminServiceNotInstalled)
            }
        }
    }

    fn root_owned_config_file(&self) -> RootOwnedConfigFile {
        match &self.config_load {
            ConfigLoad::ConfigShown {
                config_path,
                config_file_owner,
            } => RootOwnedConfigFile {
                file_owner: *config_file_owner,
                config_path: config_path.clone(),
            },
            ConfigLoad::LoadingConfig | ConfigLoad::ConfigMissing => RootOwnedConfigFile {
                file_owner: ConfigFileOwner::SystemPackage,
                config_path: String::new(),
            },
        }
    }

    fn source_for_key(&self, setting_key: &str, file_entry: FileEntry) -> ClamavSource {
        clamav_source(
            setting_key,
            self.root_owned_config_file(),
            file_entry,
            self.tuning_state(),
        )
    }

    fn current_setting_values(
        &self,
        setting_key: &str,
        configured_values: &[String],
    ) -> Vec<String> {
        let overridden_values = self
            .active_overrides()
            .iter()
            .filter(|active_override| active_override.setting_key == setting_key)
            .map(|active_override| sanitize_scanner_line(&active_override.setting_value))
            .collect::<Vec<_>>();
        if overridden_values.is_empty() {
            configured_values.to_vec()
        } else {
            overridden_values
        }
    }

    fn detection_values(&self, detection_option: DetectionOption) -> Vec<String> {
        self.current_setting_values(
            detection_option.setting_key(),
            &configured_values(&self.configured_settings, detection_option.setting_key()),
        )
    }

    fn detection_row_state(&self, detection_option: DetectionOption) -> SettingRowState {
        let setting_key = detection_option.setting_key();
        let file_entry = if configured_values(&self.configured_settings, setting_key).is_empty() {
            FileEntry::ClamavBuiltInDefault
        } else {
            FileEntry::WrittenInFile
        };
        let current_values = self.detection_values(detection_option);
        SettingRowState {
            setting_label: SettingLabel::DescribedSetting {
                title: detection_title(detection_option),
                description: detection_description(detection_option),
            },
            shown_value: ShownSettingValue::ReadableValue(TypedSettingValue::BooleanValue(
                SwitchPosition::from_switch_active(effective_switch(
                    &current_values,
                    detection_option,
                )),
            )),
            setting_subject: SettingSubject::ClamavSetting {
                setting_address: ClamavSettingAddress {
                    setting_key: setting_key.to_owned(),
                    value_position: 0,
                    sibling_values: current_values,
                },
                clamav_source: self.source_for_key(setting_key, file_entry),
            },
            change_activity: self.change_progress.row_change_activity(setting_key, 0),
        }
    }

    fn config_row_states(&self) -> Vec<SettingRowState> {
        let shown_settings = self
            .configured_settings
            .iter()
            .filter(|config_setting| self.settings_layout.lists_in_config_group(config_setting))
            .collect::<Vec<_>>();
        shown_settings
            .iter()
            .map(|config_setting| {
                let same_key_settings = shown_settings
                    .iter()
                    .filter(|other_setting| other_setting.setting_key == config_setting.setting_key)
                    .collect::<Vec<_>>();
                let setting_key = sanitize_scanner_line(&config_setting.setting_key);
                let setting_value = sanitize_scanner_line(&config_setting.setting_value);
                let value_position = same_key_settings
                    .iter()
                    .position(|same_key_setting| std::ptr::eq(**same_key_setting, *config_setting))
                    .unwrap_or_default();
                SettingRowState {
                    setting_label: SettingLabel::ConfigKeyOnly,
                    shown_value: shown_value_of(&setting_key, &setting_value),
                    change_activity: self
                        .change_progress
                        .row_change_activity(&setting_key, value_position),
                    setting_subject: SettingSubject::ClamavSetting {
                        clamav_source: self
                            .source_for_key(&config_setting.setting_key, FileEntry::WrittenInFile),
                        setting_address: ClamavSettingAddress {
                            sibling_values: same_key_settings
                                .iter()
                                .map(|same_key_setting| {
                                    sanitize_scanner_line(&same_key_setting.setting_value)
                                })
                                .collect(),
                            setting_key,
                            value_position,
                        },
                    },
                }
            })
            .collect()
    }

    fn config_group_description(&self) -> String {
        match &self.config_load {
            ConfigLoad::ConfigShown {
                config_path,
                config_file_owner,
            } => managed_config_description(*config_file_owner, config_path),
            ConfigLoad::LoadingConfig => translated_messages::widgets::loading_config(),
            ConfigLoad::ConfigMissing => translated_messages::widgets::config_missing(),
        }
    }

    fn shows_empty_section_row(&self) -> bool {
        matches!(self.config_load, ConfigLoad::ConfigShown { .. }) && self.setting_rows.is_empty()
    }

    const fn shows_detection_switches(&self) -> bool {
        self.settings_layout.shows_detection_switches()
            && matches!(self.config_load, ConfigLoad::ConfigShown { .. })
    }

    fn rebuild_rows(&mut self) {
        let detection_states = if self.settings_layout.shows_detection_switches() {
            DETECTION_OPTIONS
                .into_iter()
                .map(|detection_option| self.detection_row_state(detection_option))
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        replace_row_states(&mut self.detection_rows, detection_states);
        let config_states = self.config_row_states();
        replace_row_states(&mut self.setting_rows, config_states);
    }

    fn refresh_row_activity(&self) {
        [&self.detection_rows, &self.setting_rows]
            .into_iter()
            .for_each(|row_factory| {
                (0..row_factory.len()).for_each(|row_position| {
                    if let Some(setting_row) = row_factory.get(row_position) {
                        let change_activity = match &setting_row.row_state().setting_subject {
                            SettingSubject::ClamavSetting {
                                setting_address, ..
                            } => self.change_progress.row_change_activity(
                                &setting_address.setting_key,
                                setting_address.value_position,
                            ),
                            SettingSubject::Preference { .. } => RowChangeActivity::ChangesAllowed,
                        };
                        row_factory.send(
                            row_position,
                            SettingRowInput::ChangeActivityChanged(change_activity),
                        );
                    }
                });
            });
    }

    fn proposed_live_change(
        &self,
        setting_address: ClamavSettingAddress,
        written_value: String,
    ) -> (ProposedLiveChange, Vec<String>) {
        match detection_option_for_key(&setting_address.setting_key)
            .filter(|_detection_option| self.settings_layout.shows_detection_switches())
        {
            Some(detection_option) => {
                let current_values = self.detection_values(detection_option);
                let shown_current_values = if current_values.is_empty() {
                    vec![
                        if detection_option.enabled_by_default() {
                            "yes"
                        } else {
                            "no"
                        }
                        .to_owned(),
                    ]
                } else {
                    current_values
                };
                (
                    ProposedLiveChange {
                        changed_row: ChangedRow::DetectionSwitch(detection_option),
                        setting_key: setting_address.setting_key,
                        new_values: vec![written_value],
                    },
                    shown_current_values,
                )
            }
            None => {
                let current_values = self.current_setting_values(
                    &setting_address.setting_key,
                    &setting_address.sibling_values,
                );
                let new_values = values_with_edit(
                    &current_values,
                    setting_address.value_position,
                    sanitize_scanner_line(&written_value),
                );
                (
                    ProposedLiveChange {
                        changed_row: ChangedRow::ConfigRow {
                            setting_key: setting_address.setting_key.clone(),
                            value_position: setting_address.value_position,
                        },
                        setting_key: setting_address.setting_key,
                        new_values,
                    },
                    current_values,
                )
            }
        }
    }

    fn propose_value_change(
        &self,
        setting_address: ClamavSettingAddress,
        new_value: &TypedSettingValue,
        sender: &ComponentSender<Self>,
        root: &adw::PreferencesGroup,
    ) {
        if live_change_readiness(&self.change_progress) == LiveChangeReadiness::ChangeAlreadyRunning
        {
            return;
        }
        let Ok(written_value) = new_value.writable_value(&setting_address.setting_key) else {
            return;
        };
        let (proposed_change, current_values) =
            self.proposed_live_change(setting_address, written_value);
        present_live_change_confirmation(root, proposed_change, &current_values, sender);
    }

    fn start_running_change(
        &mut self,
        changed_row: ChangedRow,
        sender: &ComponentSender<Self>,
        change_work: impl Future<Output = LiveChangeOutcome> + Send + 'static,
    ) {
        let sandbox_kind = self.sandbox_kind;
        self.change_progress = ChangeProgress::ChangeRunning { changed_row };
        sender.oneshot_command(async move {
            let change_outcome = change_work.await;
            EngineSettingsSectionsCommand::LiveChangeFinished {
                change_outcome,
                live_tuning: read_live_tuning(sandbox_kind).await,
            }
        });
        self.refresh_row_activity();
    }

    fn open_copy_dialog(
        &mut self,
        setting_address: ClamavSettingAddress,
        editor_draft: TypedSettingValue,
        sender: &ComponentSender<Self>,
        root: &adw::PreferencesGroup,
    ) {
        let copy_dialog = ConfigLineCopyDialog::builder()
            .launch(ConfigLineCopyInit {
                setting_key: setting_address.setting_key,
                sibling_values: setting_address.sibling_values,
                value_position: setting_address.value_position,
                editor_draft,
                config_file_owner: self.root_owned_config_file().file_owner,
                config_kind: self.settings_section.config_kind(),
            })
            .forward(sender.input_sender(), |dialog_output| match dialog_output {
                ConfigLineCopyDialogOutput::ConfigLineCopied(config_line) => {
                    EngineSettingsSectionsInput::ConfigLineCopied(config_line)
                }
                ConfigLineCopyDialogOutput::CopyDialogClosed => {
                    EngineSettingsSectionsInput::CopyDialogClosed
                }
            });
        copy_dialog.widget().present(Some(root));
        self.copy_dialog = CopyDialogState::CopyDialogOpen(copy_dialog);
    }

    fn handle_follow_up(
        &mut self,
        setting_subject: SettingSubject,
        shown_value: ShownSettingValue,
        follow_up_action: FollowUpAction,
        sender: &ComponentSender<Self>,
        root: &adw::PreferencesGroup,
    ) {
        let SettingSubject::ClamavSetting {
            setting_address,
            clamav_source,
        } = setting_subject
        else {
            return;
        };
        match (follow_up_action, clamav_source) {
            (FollowUpAction::RevertRuntimeChange, _clamav_source) => {
                if live_change_readiness(&self.change_progress)
                    == LiveChangeReadiness::ReadyToChange
                {
                    let setting_key = setting_address.setting_key;
                    self.start_running_change(
                        ChangedRow::WholeSetting {
                            setting_key: setting_key.clone(),
                        },
                        sender,
                        reset_live_settings(vec![setting_key]),
                    );
                }
            }
            (
                FollowUpAction::CopyConfigurationLine,
                ClamavSource::SetViaAdminService {
                    config_file,
                    override_values,
                    ..
                },
            ) => {
                root.clipboard().set_text(&change_snippet(
                    config_file.file_owner,
                    self.settings_section.config_kind(),
                    &setting_address.setting_key,
                    &override_values,
                ));
                Self::report_notice(sender, EngineSettingsNotice::ChangeSnippetCopied);
            }
            (
                FollowUpAction::CopyConfigurationLine,
                ClamavSource::RootOwnedFile { .. } | ClamavSource::ProtectedSetting { .. },
            ) => {
                if let ShownSettingValue::ReadableValue(editor_draft) = shown_value {
                    self.open_copy_dialog(setting_address, editor_draft, sender, root);
                }
            }
            (FollowUpAction::ResetToDefault, _clamav_source) => {}
        }
    }
}

fn load_section_config(
    sandbox_kind: SandboxKind,
    sender: &ComponentSender<EngineSettingsSections>,
    settings_section: EngineSettingsSection,
) {
    sender.oneshot_command(async move {
        let host_config = settings_section.read_host_config(sandbox_kind);
        let config_file_owner = match &host_config {
            HostConfigView::ConfigFound { config_path, .. } => {
                inspect_config_file_owner(std::path::Path::new(config_path))
            }
            HostConfigView::ConfigNotFound => ConfigFileOwner::SystemPackage,
        };
        let tuning_load = if settings_section.offers_live_tuning() {
            LiveTuningLoad::TuningRead(read_live_tuning(sandbox_kind).await)
        } else {
            LiveTuningLoad::TuningNotOffered
        };
        EngineSettingsSectionsCommand::ConfigLoaded {
            host_config,
            config_file_owner,
            tuning_load,
        }
    });
}

#[relm4::component(pub)]
impl Component for EngineSettingsSections {
    type Init = EngineSettingsSectionsInit;
    type Input = EngineSettingsSectionsInput;
    type Output = EngineSettingsSectionsOutput;
    type CommandOutput = EngineSettingsSectionsCommand;

    view! {
        adw::PreferencesGroup {

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 24,

                #[local_ref]
                detection_row_group -> adw::PreferencesGroup {
                    set_title: &translated_messages::widgets::detection_title(),
                    set_description: Some(&translated_messages::widgets::clamav_reports_every_description()),
                    #[watch]
                    set_visible: model.shows_detection_switches(),
                },

                #[local_ref]
                setting_rows_group -> adw::PreferencesGroup {
                    set_title: &model.settings_section.group_title(),
                    #[watch]
                    set_description: Some(&glib::markup_escape_text(
                        &model.config_group_description(),
                    )),

                    adw::ActionRow {
                        set_title: &translated_messages::widgets::no_settings_section_title(),
                        #[watch]
                        set_visible: model.shows_empty_section_row(),
                    },
                },
            },
        }
    }

    fn init(
        EngineSettingsSectionsInit {
            settings_layout,
            sandbox_kind,
        }: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings_section = settings_layout.config_section();
        let setting_rows = FactoryVecDeque::builder()
            .launch(adw::PreferencesGroup::default())
            .forward(
                sender.input_sender(),
                EngineSettingsSectionsInput::SettingRowReported,
            );
        let detection_rows = FactoryVecDeque::builder()
            .launch(adw::PreferencesGroup::default())
            .forward(
                sender.input_sender(),
                EngineSettingsSectionsInput::SettingRowReported,
            );
        let model = Self {
            sandbox_kind,
            settings_layout,
            settings_section,
            config_load: ConfigLoad::LoadingConfig,
            tuning_load: if settings_section.offers_live_tuning() {
                LiveTuningLoad::LoadingTuning
            } else {
                LiveTuningLoad::TuningNotOffered
            },
            change_progress: ChangeProgress::NoChangeRunning,
            setting_rows,
            detection_rows,
            configured_settings: Vec::new(),
            copy_dialog: CopyDialogState::NoCopyDialog,
        };
        let setting_rows_group = model.setting_rows.widget();
        let detection_row_group = model.detection_rows.widget();
        let widgets = view_output!();
        load_section_config(sandbox_kind, &sender, settings_section);
        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        page_input: EngineSettingsSectionsInput,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match page_input {
            EngineSettingsSectionsInput::SettingRowReported(
                SettingRowOutput::ValueChangeRequested {
                    setting_subject:
                        SettingSubject::ClamavSetting {
                            setting_address, ..
                        },
                    new_value,
                },
            ) => {
                self.propose_value_change(setting_address, &new_value, &sender, root);
                self.refresh_row_activity();
            }
            EngineSettingsSectionsInput::SettingRowReported(
                SettingRowOutput::ValueChangeRequested {
                    setting_subject: SettingSubject::Preference { .. },
                    ..
                },
            ) => {}
            EngineSettingsSectionsInput::SettingRowReported(SettingRowOutput::FollowUpChosen {
                setting_subject,
                shown_value,
                follow_up_action,
            }) => {
                self.handle_follow_up(
                    setting_subject,
                    shown_value,
                    follow_up_action,
                    &sender,
                    root,
                );
            }
            EngineSettingsSectionsInput::ConfigLineCopied(config_line) => {
                root.clipboard().set_text(&config_line);
                Self::report_notice(&sender, EngineSettingsNotice::ChangeSnippetCopied);
                match &self.copy_dialog {
                    CopyDialogState::CopyDialogOpen(copy_dialog) => {
                        let _dialog_closed = copy_dialog.widget().close();
                    }
                    CopyDialogState::NoCopyDialog => {}
                }
            }
            EngineSettingsSectionsInput::CopyDialogClosed => {
                self.copy_dialog = CopyDialogState::NoCopyDialog;
            }
            EngineSettingsSectionsInput::LiveChangeConfirmed {
                changed_row,
                setting_key,
                new_values,
            } => match confirmation_verdict(live_change_readiness(&self.change_progress)) {
                ConfirmationVerdict::StartConfirmedChange => {
                    self.start_running_change(
                        changed_row,
                        &sender,
                        apply_live_change(setting_key, new_values),
                    );
                }
                ConfirmationVerdict::DropStaleConfirmation => {
                    Self::report_notice(&sender, EngineSettingsNotice::StaleConfirmationDropped);
                    self.refresh_row_activity();
                }
            },
        }
    }

    fn update_cmd(
        &mut self,
        page_command: EngineSettingsSectionsCommand,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        let sandbox_kind = self.sandbox_kind;
        match page_command {
            EngineSettingsSectionsCommand::ConfigLoaded {
                host_config,
                config_file_owner,
                tuning_load,
            } => {
                self.tuning_load = tuning_load;
                match host_config {
                    HostConfigView::ConfigFound {
                        config_path,
                        config_settings,
                    } => {
                        self.configured_settings = config_settings;
                        self.config_load = ConfigLoad::ConfigShown {
                            config_path,
                            config_file_owner,
                        };
                        self.rebuild_rows();
                    }
                    HostConfigView::ConfigNotFound => self.config_load = ConfigLoad::ConfigMissing,
                }
            }
            EngineSettingsSectionsCommand::LiveChangeFinished {
                change_outcome,
                live_tuning,
            } => {
                self.change_progress = ChangeProgress::NoChangeRunning;
                Self::report_notice(
                    &sender,
                    EngineSettingsNotice::LiveChangeReported(change_outcome),
                );
                self.tuning_load = LiveTuningLoad::TuningRead(live_tuning);
                self.rebuild_rows();
                load_section_config(sandbox_kind, &sender, self.settings_section);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SettingsChangeFollowUp {
    ReloadChangesAndEvidence,
    NothingToReload,
}

pub const fn settings_change_follow_up(
    settings_notice: &EngineSettingsNotice,
) -> SettingsChangeFollowUp {
    match settings_notice {
        EngineSettingsNotice::LiveChangeReported(_) => {
            SettingsChangeFollowUp::ReloadChangesAndEvidence
        }
        EngineSettingsNotice::ChangeSnippetCopied
        | EngineSettingsNotice::StaleConfirmationDropped => SettingsChangeFollowUp::NothingToReload,
    }
}

#[cfg(test)]
mod engine_settings_sections_tests {
    use super::*;
    use crate::shared_widgets::setting_row::setting_row_state::RuntimeRoute;

    #[test]
    fn a_live_change_reloads_the_audit_and_the_evidence() {
        assert_eq!(
            settings_change_follow_up(&EngineSettingsNotice::LiveChangeReported(
                LiveChangeOutcome::SettingsApplied
            )),
            SettingsChangeFollowUp::ReloadChangesAndEvidence
        );
        assert_eq!(
            settings_change_follow_up(&EngineSettingsNotice::LiveChangeReported(
                LiveChangeOutcome::PreviousRestored
            )),
            SettingsChangeFollowUp::ReloadChangesAndEvidence
        );
    }

    #[test]
    fn copying_a_snippet_reloads_nothing() {
        assert_eq!(
            settings_change_follow_up(&EngineSettingsNotice::ChangeSnippetCopied),
            SettingsChangeFollowUp::NothingToReload
        );
        assert_eq!(
            settings_change_follow_up(&EngineSettingsNotice::StaleConfirmationDropped),
            SettingsChangeFollowUp::NothingToReload
        );
    }

    #[test]
    fn every_settings_notice_describes_its_own_toast() {
        assert_eq!(
            describe_engine_settings_notice(&EngineSettingsNotice::ChangeSnippetCopied),
            "Copied. Add it to your configuration to apply it."
        );
        assert_eq!(
            describe_engine_settings_notice(&EngineSettingsNotice::LiveChangeReported(
                LiveChangeOutcome::NothingChanged(LiveChangeFailure::AuthorizationRefused)
            )),
            "Nothing changed without your password"
        );
        assert_eq!(
            describe_engine_settings_notice(&EngineSettingsNotice::LiveChangeReported(
                LiveChangeOutcome::SettingsApplied
            )),
            describe_live_change_outcome(&LiveChangeOutcome::SettingsApplied)
        );
    }

    #[test]
    fn a_second_live_change_is_refused_while_one_runs() {
        let running_change = ChangeProgress::ChangeRunning {
            changed_row: ChangedRow::DetectionSwitch(DetectionOption::HeuristicAlerts),
        };
        assert_eq!(
            confirmation_verdict(live_change_readiness(&running_change)),
            ConfirmationVerdict::DropStaleConfirmation
        );
        assert_eq!(
            confirmation_verdict(live_change_readiness(&ChangeProgress::NoChangeRunning)),
            ConfirmationVerdict::StartConfirmedChange
        );
    }

    #[test]
    fn only_the_changed_row_spins_and_a_revert_spins_every_row_of_its_key() {
        let switch_change = ChangeProgress::ChangeRunning {
            changed_row: ChangedRow::DetectionSwitch(DetectionOption::HeuristicAlerts),
        };
        assert_eq!(
            switch_change.row_change_activity("HeuristicAlerts", 0),
            RowChangeActivity::ApplyingThisRow
        );
        assert_eq!(
            switch_change.row_change_activity("AlertOLE2Macros", 0),
            RowChangeActivity::ApplyingOtherRow
        );
        let config_change = ChangeProgress::ChangeRunning {
            changed_row: ChangedRow::ConfigRow {
                setting_key: "OnAccessIncludePath".to_owned(),
                value_position: 1,
            },
        };
        assert_eq!(
            config_change.row_change_activity("OnAccessIncludePath", 0),
            RowChangeActivity::ApplyingOtherRow
        );
        let revert_change = ChangeProgress::ChangeRunning {
            changed_row: ChangedRow::WholeSetting {
                setting_key: "OnAccessIncludePath".to_owned(),
            },
        };
        assert_eq!(
            revert_change.row_change_activity("OnAccessIncludePath", 0),
            RowChangeActivity::ApplyingThisRow
        );
        assert_eq!(
            revert_change.row_change_activity("OnAccessIncludePath", 1),
            RowChangeActivity::ApplyingThisRow
        );
        assert_eq!(
            ChangeProgress::NoChangeRunning.row_change_activity("MaxThreads", 0),
            RowChangeActivity::ChangesAllowed
        );
    }

    #[test]
    fn detection_keys_are_listed_once_on_the_scan_engine_page() {
        let detection_setting = ConfigSetting {
            setting_key: "DetectPUA".to_owned(),
            setting_value: "yes".to_owned(),
        };
        assert!(
            !EngineSettingsLayout::EngineWithDetection.lists_in_config_group(&detection_setting)
        );
        assert!(
            EngineSettingsLayout::SingleSection(EngineSettingsSection::ScannerEngine)
                .lists_in_config_group(&detection_setting)
        );
    }

    #[test]
    fn unreadable_clamd_values_keep_their_raw_text() {
        assert!(matches!(
            shown_value_of("OnAccessExtraScanning", "tru"),
            ShownSettingValue::UnreadableValue { raw_text, .. } if raw_text == "tru"
        ));
        assert!(matches!(
            shown_value_of("MaxThreads", "4"),
            ShownSettingValue::ReadableValue(TypedSettingValue::CountValue { count_value: 4, .. })
        ));
        assert!(matches!(
            clamav_source(
                "MaxThreads",
                RootOwnedConfigFile {
                    file_owner: ConfigFileOwner::SystemPackage,
                    config_path: "/etc/clamav/clamd.conf".to_owned(),
                },
                FileEntry::WrittenInFile,
                RuntimeTuningState::TuningMissing(CopyOnlyReason::AdminServiceUnreachable),
            ),
            ClamavSource::RootOwnedFile {
                runtime_route: RuntimeRoute::CopyLineOnly(CopyOnlyReason::AdminServiceUnreachable),
                ..
            }
        ));
    }

    #[test]
    fn the_scan_engine_layout_shows_both_sections() {
        assert!(EngineSettingsLayout::EngineWithDetection.shows_detection_switches());
    }
}
