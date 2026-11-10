use std::rc::Rc;

use relm4::{Component, ComponentParts, ComponentSender, adw, adw::prelude::*, gtk};

use crate::{
    host_services::threat_intel::{
        service_access::{KeyProblem, ServiceAccess, read_service_access},
        service_key_storage::{
            KeySource, KeyStorageFailure, ThreatIntelService, delete_stored_api_key, key_source,
            save_api_key,
        },
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

const VIRUSTOTAL_KEY_PAGE_URL: &str = "https://www.virustotal.com/gui/my-apikey";
const ABUSE_CH_KEY_PAGE_URL: &str = "https://auth.abuse.ch/";

fn service_group_title(threat_intel_service: ThreatIntelService) -> &'static str {
    match threat_intel_service {
        ThreatIntelService::VirusTotalService => "VirusTotal",
        ThreatIntelService::AbuseChService => "MalwareBazaar (abuse.ch)",
    }
}

fn service_description(
    message_catalog: &MessageCatalog,
    threat_intel_service: ThreatIntelService,
) -> String {
    match threat_intel_service {
        ThreatIntelService::VirusTotalService => {
            translated_messages::preferences::virustotal_receives_summary(message_catalog)
        }
        ThreatIntelService::AbuseChService => {
            translated_messages::preferences::malwarebazaar_receives_summary(message_catalog)
        }
    }
}

fn service_key_hint(
    message_catalog: &MessageCatalog,
    threat_intel_service: ThreatIntelService,
) -> String {
    match threat_intel_service {
        ThreatIntelService::VirusTotalService => {
            translated_messages::preferences::virustotal_service_hint(message_catalog)
        }
        ThreatIntelService::AbuseChService => {
            translated_messages::preferences::abuse_ch_service_hint(message_catalog)
        }
    }
}

const fn service_key_page_url(threat_intel_service: ThreatIntelService) -> &'static str {
    match threat_intel_service {
        ThreatIntelService::VirusTotalService => VIRUSTOTAL_KEY_PAGE_URL,
        ThreatIntelService::AbuseChService => ABUSE_CH_KEY_PAGE_URL,
    }
}

fn key_problem_hint(message_catalog: &MessageCatalog, key_problem: KeyProblem) -> String {
    match key_problem {
        KeyProblem::KeyFileNotPermitted => {
            translated_messages::preferences::key_file_not_permitted_hint(message_catalog)
        }
        KeyProblem::KeyFileMissing => {
            translated_messages::preferences::key_file_missing_hint(message_catalog)
        }
        KeyProblem::KeyFileUnreadable => {
            translated_messages::preferences::key_file_unreadable_hint(message_catalog)
        }
        KeyProblem::KeyNotValid => {
            translated_messages::preferences::key_not_valid_hint(message_catalog)
        }
        KeyProblem::KeyringUnavailable => {
            translated_messages::preferences::keyring_unavailable_hint(message_catalog)
        }
        KeyProblem::StoredKeyUnreadable => {
            translated_messages::preferences::stored_key_unreadable_hint(message_catalog)
        }
    }
}

fn key_problem_text(message_catalog: &MessageCatalog, key_problem: KeyProblem) -> String {
    match key_problem {
        KeyProblem::KeyFileNotPermitted => {
            translated_messages::preferences::key_file_not_permitted(message_catalog)
        }
        KeyProblem::KeyFileMissing => {
            translated_messages::preferences::key_file_missing(message_catalog)
        }
        KeyProblem::KeyFileUnreadable => {
            translated_messages::preferences::key_file_unreadable(message_catalog)
        }
        KeyProblem::KeyNotValid => translated_messages::preferences::key_not_valid(message_catalog),
        KeyProblem::KeyringUnavailable => {
            translated_messages::preferences::keyring_unavailable(message_catalog)
        }
        KeyProblem::StoredKeyUnreadable => {
            translated_messages::preferences::stored_key_unreadable(message_catalog)
        }
    }
}

enum KeyReading {
    CheckingKey,
    KeyRead(ServiceAccess),
}

enum KeySaveOutcome {
    NothingSaved,
    SaveRefused(KeyProblem),
}

pub struct ServiceKeySection {
    pub message_catalog: Rc<MessageCatalog>,
    threat_intel_service: ThreatIntelService,
    key_source: KeySource,
    key_reading: KeyReading,
    key_save_outcome: KeySaveOutcome,
    entered_key_cleared: bool,
}

#[derive(Debug)]
pub enum ServiceKeySectionInput {
    KeyEntered(String),
    KeyDeletionRequested,
    KeyWebsiteRequested,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceKeySectionOutput {
    StoredKeyChanged,
}

#[derive(Debug)]
pub enum ServiceKeySectionCommand {
    KeyAccessRead(ServiceAccess),
    KeyChanged(Result<(), KeyStorageFailure>),
}

impl ServiceKeySection {
    fn refresh_key_access(&self, sender: &ComponentSender<Self>) {
        let threat_intel_service = self.threat_intel_service;
        sender.oneshot_command(async move {
            ServiceKeySectionCommand::KeyAccessRead(read_service_access(threat_intel_service).await)
        });
    }

    const fn uses_system_key_file(&self) -> bool {
        matches!(self.key_source, KeySource::SystemKeyFile { .. })
    }

    fn key_file_path_text(&self) -> String {
        match &self.key_source {
            KeySource::SystemKeyFile { key_file_path } => key_file_path.display().to_string(),
            KeySource::UserKeyring => String::new(),
        }
    }

    const fn key_problem(&self) -> Option<KeyProblem> {
        match (&self.key_save_outcome, &self.key_reading) {
            (KeySaveOutcome::SaveRefused(key_problem), _)
            | (
                KeySaveOutcome::NothingSaved,
                KeyReading::KeyRead(ServiceAccess::KeyBroken(key_problem)),
            ) => Some(*key_problem),
            (
                KeySaveOutcome::NothingSaved,
                KeyReading::CheckingKey
                | KeyReading::KeyRead(ServiceAccess::KeyReady(_) | ServiceAccess::KeyMissing),
            ) => None,
        }
    }

    fn key_problem_banner_text(&self) -> String {
        let message_catalog = Rc::clone(&self.message_catalog);
        self.key_problem()
            .map(|key_problem| key_problem_text(&message_catalog, key_problem))
            .unwrap_or_default()
    }

    fn key_problem_hint_text(&self) -> String {
        let message_catalog = Rc::clone(&self.message_catalog);
        self.key_problem()
            .map(|key_problem| key_problem_hint(&message_catalog, key_problem))
            .unwrap_or_default()
    }

    fn key_status_text(&self) -> String {
        let message_catalog = Rc::clone(&self.message_catalog);
        match &self.key_reading {
            KeyReading::CheckingKey => {
                translated_messages::preferences::checking_label(&message_catalog)
            }
            KeyReading::KeyRead(ServiceAccess::KeyReady(api_key)) => api_key.masked_text(),
            KeyReading::KeyRead(ServiceAccess::KeyMissing) => {
                translated_messages::preferences::not_configured(&message_catalog)
            }
            KeyReading::KeyRead(ServiceAccess::KeyBroken(_)) => {
                translated_messages::preferences::key_unreadable_status(&message_catalog)
            }
        }
    }

    const fn keyring_key_stored(&self) -> bool {
        matches!(self.key_source, KeySource::UserKeyring)
            && matches!(
                self.key_reading,
                KeyReading::KeyRead(ServiceAccess::KeyReady(_))
            )
    }
}

pub struct ServiceKeySectionInit {
    pub threat_intel_service: ThreatIntelService,
    pub message_catalog: Rc<MessageCatalog>,
}

#[relm4::component(pub)]
impl Component for ServiceKeySection {
    type Init = ServiceKeySectionInit;
    type Input = ServiceKeySectionInput;
    type Output = ServiceKeySectionOutput;
    type CommandOutput = ServiceKeySectionCommand;

    view! {
        adw::PreferencesGroup {
            set_title: service_group_title(model.threat_intel_service),
            set_description: Some(&service_description(&model.message_catalog, model.threat_intel_service)),

            adw::ActionRow {
                set_use_markup: false,
                add_css_class: "warning",
                #[watch]
                set_visible: model.key_problem().is_some(),
                set_subtitle_lines: 0,
                #[watch]
                set_title: &model.key_problem_banner_text(),
                #[watch]
                set_subtitle: &model.key_problem_hint_text(),
                add_prefix = &gtk::Image { set_icon_name: Some("dialog-warning-symbolic") },
            },

            adw::ActionRow {
                set_title: &translated_messages::preferences::key_file_title(&model.message_catalog),
                set_use_markup: false,
                set_subtitle: &model.key_file_path_text(),
                set_tooltip_text: Some(&translated_messages::preferences::set_system_configuration_title(&model.message_catalog)),
                set_visible: model.uses_system_key_file(),
                add_prefix = &gtk::Image { set_icon_name: Some("changes-prevent-symbolic") },
                add_suffix = &gtk::Label {
                    add_css_class: "dim-label",
                    #[watch]
                    set_visible: model.key_problem().is_none(),
                    #[watch]
                    set_label: &model.key_status_text(),
                },
            },

            adw::ActionRow {
                set_title: &translated_messages::preferences::key_keyring_title(&model.message_catalog),
                set_visible: !model.uses_system_key_file(),
                #[watch]
                set_subtitle: &model.key_status_text(),

                add_suffix = &gtk::Button {
                    set_label: &translated_messages::preferences::delete_key_label(&model.message_catalog),
                    set_valign: gtk::Align::Center,
                    #[watch]
                    set_visible: model.keyring_key_stored(),
                    add_css_class: "flat",
                    connect_clicked => ServiceKeySectionInput::KeyDeletionRequested,
                },
            },

            adw::PasswordEntryRow {
                set_title: &translated_messages::preferences::paste_api_key_title(&model.message_catalog),
                set_show_apply_button: true,
                set_visible: !model.uses_system_key_file(),
                #[track(model.entered_key_cleared)]
                set_text: "",
                connect_apply[sender] => move |applied_row| {
                    sender.input(ServiceKeySectionInput::KeyEntered(
                        applied_row.text().trim().to_owned(),
                    ));
                },
            },

            adw::ActionRow {
                set_title: &translated_messages::preferences::get_free_api_title(&model.message_catalog),
                set_subtitle: &service_key_hint(&model.message_catalog, model.threat_intel_service),
                set_activatable: true,
                set_visible: !model.uses_system_key_file(),
                add_suffix = &gtk::Image { set_icon_name: Some("adw-external-link-symbolic") },
                connect_activated => ServiceKeySectionInput::KeyWebsiteRequested,
            },
        }
    }

    fn init(
        ServiceKeySectionInit {
            threat_intel_service,
            message_catalog,
        }: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            message_catalog,
            threat_intel_service,
            key_source: key_source(threat_intel_service),
            key_reading: KeyReading::CheckingKey,
            key_save_outcome: KeySaveOutcome::NothingSaved,
            entered_key_cleared: false,
        };
        let widgets = view_output!();
        model.refresh_key_access(&sender);
        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        group_input: ServiceKeySectionInput,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        let threat_intel_service = self.threat_intel_service;
        self.entered_key_cleared = false;
        match group_input {
            ServiceKeySectionInput::KeyEntered(entered_key) => {
                self.entered_key_cleared = true;
                sender.oneshot_command(async move {
                    ServiceKeySectionCommand::KeyChanged(
                        save_api_key(threat_intel_service, entered_key).await,
                    )
                });
            }
            ServiceKeySectionInput::KeyDeletionRequested => {
                sender.oneshot_command(async move {
                    ServiceKeySectionCommand::KeyChanged(
                        delete_stored_api_key(threat_intel_service).await,
                    )
                });
            }
            ServiceKeySectionInput::KeyWebsiteRequested => {
                gtk::UriLauncher::new(service_key_page_url(threat_intel_service)).launch(
                    root.root().and_downcast::<gtk::Window>().as_ref(),
                    gtk::gio::Cancellable::NONE,
                    |_launch_outcome| {},
                );
            }
        }
    }

    fn update_cmd(
        &mut self,
        group_command: ServiceKeySectionCommand,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        self.entered_key_cleared = false;
        match group_command {
            ServiceKeySectionCommand::KeyAccessRead(service_access) => {
                self.key_reading = KeyReading::KeyRead(service_access);
            }
            ServiceKeySectionCommand::KeyChanged(Err(key_failure)) => {
                self.key_save_outcome = KeySaveOutcome::SaveRefused(KeyProblem::from(&key_failure));
            }
            ServiceKeySectionCommand::KeyChanged(Ok(())) => {
                self.key_save_outcome = KeySaveOutcome::NothingSaved;
                self.refresh_key_access(&sender);
                let _page_closed = sender.output(ServiceKeySectionOutput::StoredKeyChanged);
            }
        }
    }
}
