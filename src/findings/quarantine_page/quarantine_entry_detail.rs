use relm4::{
    FactorySender, adw,
    adw::prelude::*,
    gtk::{self, glib},
    prelude::{DynamicIndex, FactoryComponent},
};

use crate::{
    application_window::application_actions::ONLINE_SERVICES_ACTION,
    findings::quarantine_page::quarantine_page_state::{
        EntryRowState, QuarantineRowInit, RowActivity, RowReputation,
    },
    host_services::{
        clamav_admin_service::trust_requests::{FileFingerprint, TrustChoice, TrustScope},
        clamav_programs::clamav_report_submission::{ClamavReportKind, SubmittedContent},
        clamav_scanner::scanner_output_parser::{displayed_threat_name, sanitize_scanner_line},
        desktop::locale_formatting::{MomentFormatters, byte_size_text},
        quarantine_vault::{
            quarantine_entry_database::{QuarantineEntryRecord, StoredDetectionDate},
            quarantine_file_moves::RecordedPayloadHash,
        },
        threat_intel::{
            file_reputation_check::{KeyedSourceAnswer, ReputationAnswers, SourceAnswer},
            service_access::{KnownServiceAccess, ServiceAccess},
        },
    },
    shared_widgets::submission_confirm_dialog::ClamavReportProposal,
    translated_messages,
};

pub fn quarantined_false_positive_detection(
    entry_record: &QuarantineEntryRecord,
) -> Option<ClamavReportProposal> {
    entry_record
        .threat_identity
        .matched_signature_name()
        .map(|signature_name| ClamavReportProposal {
            report_kind: ClamavReportKind::FalsePositive {
                detection_name: signature_name.to_owned(),
            },
            submitted_content: SubmittedContent::QuarantinedEntry {
                entry_id: entry_record.entry_id,
            },
            shown_path: sanitize_scanner_line(&entry_record.original_path.to_string_lossy()),
        })
}

pub fn entry_summary_text(
    moment_formatters: &MomentFormatters,
    entry_record: &QuarantineEntryRecord,
) -> String {
    format!(
        "{}\n{} · {}",
        sanitize_scanner_line(&entry_record.original_path.to_string_lossy()),
        match &entry_record.detection_date {
            StoredDetectionDate::DetectedAt(detected_at) =>
                moment_formatters.exact_moment_text(*detected_at),
            StoredDetectionDate::UnreadableText(stored_text) => sanitize_scanner_line(stored_text),
        },
        byte_size_text(entry_record.file_size),
    )
}

pub struct QuarantineEntryDetail {
    pub moment_formatters: &'static MomentFormatters,
    entry_record: QuarantineEntryRecord,
    row_state: EntryRowState,
}

#[derive(Debug)]
pub enum QuarantineEntryDetailInput {
    ReputationCheckRequested,
    HashCopyRequested,
    VirusTotalReportRequested,
    TrustFileRequested,
    FalsePositiveReportRequested,
    SubmissionRequested,
    RowStateChanged(Box<EntryRowState>),
}

#[derive(Debug)]
pub enum QuarantineEntryDetailOutput {
    RestoreRequested { entry_id: i64 },
    ReputationCheckRequested { entry_id: i64, content_hash: String },
    DeletionRequested { entry_id: i64 },
    TrustRequested(TrustChoice),
    FalsePositiveReportRequested(ClamavReportProposal),
    SubmissionRequested { entry_id: i64 },
    HashCopyRequested(String),
    ReportLinkRequested(String),
}

impl QuarantineEntryDetail {
    pub const fn entry_id(&self) -> i64 {
        self.entry_record.entry_id
    }

    fn known_sha256_hex(&self) -> Option<&str> {
        match &self.entry_record.payload_hash {
            RecordedPayloadHash::KnownSha256 { sha256_hex } => Some(sha256_hex),
            RecordedPayloadHash::IntegrityUnknown => None,
        }
    }

    fn payload_hash_text(&self) -> String {
        match &self.entry_record.payload_hash {
            RecordedPayloadHash::KnownSha256 { sha256_hex } => sha256_hex.clone(),
            RecordedPayloadHash::IntegrityUnknown => {
                translated_messages::findings::integrity_unknown_hash()
            }
        }
    }

    fn actions_available(&self) -> bool {
        self.row_state.row_activity == RowActivity::ActionsAvailable
    }

    fn reputation_check_running(&self) -> bool {
        matches!(
            self.row_state.row_reputation,
            RowReputation::CheckingReputation
        )
    }

    fn checked_answers(&self) -> Option<&ReputationAnswers> {
        match &self.row_state.row_reputation {
            RowReputation::ReputationChecked(reputation_answers) => Some(reputation_answers),
            RowReputation::NotChecked | RowReputation::CheckingReputation => None,
        }
    }

    fn source_answer_text(&self, answered_text: fn(&ReputationAnswers) -> String) -> String {
        match &self.row_state.row_reputation {
            RowReputation::NotChecked => translated_messages::findings::not_checked(),
            RowReputation::CheckingReputation => translated_messages::findings::checking_label(),
            RowReputation::ReputationChecked(reputation_answers) => {
                answered_text(reputation_answers)
            }
        }
    }

    fn keyed_source_text(
        &self,
        source_blocker: fn(&KnownServiceAccess) -> Option<&ServiceAccess>,
        keyed_source: fn(&ReputationAnswers) -> &KeyedSourceAnswer,
    ) -> String {
        match source_blocker(&self.row_state.service_access) {
            Some(ServiceAccess::KeyMissing) => translated_messages::findings::reputation_no_key(),
            Some(ServiceAccess::KeyBroken(_) | ServiceAccess::KeyReady(_)) => {
                translated_messages::findings::reputation_key_unreadable()
            }
            None => match &self.row_state.row_reputation {
                RowReputation::NotChecked => translated_messages::findings::not_checked(),
                RowReputation::CheckingReputation => {
                    translated_messages::findings::checking_label()
                }
                RowReputation::ReputationChecked(reputation_answers) => {
                    match keyed_source(reputation_answers) {
                        KeyedSourceAnswer::KeyedAnswer(source_answer) => {
                            source_answer.displayed_text()
                        }
                        KeyedSourceAnswer::SkippedWithoutKey => {
                            translated_messages::findings::not_checked()
                        }
                    }
                }
            },
        }
    }

    fn keyed_source_blocked(
        &self,
        source_blocker: fn(&KnownServiceAccess) -> Option<&ServiceAccess>,
    ) -> bool {
        source_blocker(&self.row_state.service_access).is_some()
    }

    fn keyed_source_checking(
        &self,
        source_blocker: fn(&KnownServiceAccess) -> Option<&ServiceAccess>,
    ) -> bool {
        self.reputation_check_running() && !self.keyed_source_blocked(source_blocker)
    }

    fn keyed_source_styles(
        &self,
        source_blocker: fn(&KnownServiceAccess) -> Option<&ServiceAccess>,
    ) -> &'static [&'static str] {
        if self.keyed_source_blocked(source_blocker) {
            &[]
        } else {
            &["property"]
        }
    }

    fn virustotal_report_url(&self) -> Option<&str> {
        if self.keyed_source_blocked(KnownServiceAccess::virustotal_blocker) {
            return None;
        }
        self.checked_answers().and_then(|reputation_answers| {
            match &reputation_answers.virustotal_answer {
                KeyedSourceAnswer::KeyedAnswer(SourceAnswer::AnsweredWithReport {
                    report_url,
                    ..
                }) => Some(report_url.as_str()),
                KeyedSourceAnswer::KeyedAnswer(
                    SourceAnswer::Answered { .. } | SourceAnswer::Failed(_),
                )
                | KeyedSourceAnswer::SkippedWithoutKey => None,
            }
        })
    }
}

fn circl_answer_text(reputation_answers: &ReputationAnswers) -> String {
    reputation_answers.circl_answer.displayed_text()
}

const fn bazaar_source(reputation_answers: &ReputationAnswers) -> &KeyedSourceAnswer {
    &reputation_answers.bazaar_answer
}

const fn virustotal_source(reputation_answers: &ReputationAnswers) -> &KeyedSourceAnswer {
    &reputation_answers.virustotal_answer
}

fn source_row(source_name: &str) -> adw::ActionRow {
    adw::ActionRow::builder()
        .use_markup(false)
        .title(source_name)
        .subtitle_lines(0)
        .css_classes(["property"])
        .build()
}

#[relm4::factory(pub)]
impl FactoryComponent for QuarantineEntryDetail {
    type Init = QuarantineRowInit;
    type Input = QuarantineEntryDetailInput;
    type Output = QuarantineEntryDetailOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 18,

            adw::PreferencesGroup {
                set_title: &glib::markup_escape_text(&displayed_threat_name(&self.entry_record.threat_identity)),
                set_description: Some(&glib::markup_escape_text(&entry_summary_text(self.moment_formatters, &self.entry_record))),

                #[wrap(Some)]
                set_header_suffix = &gtk::Box {
                    set_spacing: 6,
                    set_valign: gtk::Align::Start,

                    gtk::Button {
                        set_label: &translated_messages::findings::delete_entry_label(),
                        add_css_class: "destructive-action",
                        #[watch]
                        set_sensitive: self.actions_available(),
                        connect_clicked[sender, entry_id = self.entry_record.entry_id] => move |_clicked_button| {
                            let _parent_gone = sender.output(QuarantineEntryDetailOutput::DeletionRequested { entry_id });
                        },
                    },

                    gtk::Button {
                        set_label: &translated_messages::findings::restore_label(),
                        #[watch]
                        set_sensitive: self.known_sha256_hex().is_some() && self.actions_available(),
                        connect_clicked[sender, entry_id = self.entry_record.entry_id] => move |_clicked_button| {
                            let _parent_gone = sender.output(QuarantineEntryDetailOutput::RestoreRequested { entry_id });
                        },
                    },
                },
            },

            adw::PreferencesGroup {
                set_title: &translated_messages::findings::reputation_section_title(),
                set_description: Some(&translated_messages::findings::reputation_section_description()),

                add = &adw::ActionRow {
                    set_use_markup: false,
                    set_title: "SHA-256",
                    set_subtitle: &self.payload_hash_text(),
                    set_subtitle_lines: 0,
                    set_subtitle_selectable: true,
                    add_css_class: "property",

                    add_suffix = &gtk::Button {
                        set_icon_name: "edit-copy-symbolic",
                        set_tooltip_text: Some(&translated_messages::findings::copy_hash_tooltip()),
                        set_valign: gtk::Align::Center,
                        set_visible: self.known_sha256_hex().is_some(),
                        add_css_class: "flat",
                        connect_clicked => QuarantineEntryDetailInput::HashCopyRequested,
                    },
                },

                add = &adw::ButtonRow {
                    set_title: &translated_messages::findings::look_up_hash_button(),
                    set_start_icon_name: Some("system-search-symbolic"),
                    #[watch]
                    set_sensitive: self.known_sha256_hex().is_some() && !self.reputation_check_running(),
                    connect_activated => QuarantineEntryDetailInput::ReputationCheckRequested,
                },

                add = &source_row("CIRCL hashlookup") -> adw::ActionRow {
                    #[watch]
                    set_subtitle: &self.source_answer_text(circl_answer_text),

                    add_suffix = &adw::Spinner {

                        set_halign: gtk::Align::Center,

                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_visible: self.reputation_check_running(),
                    },
                },

                add = &source_row("MalwareBazaar") -> adw::ActionRow {
                    #[watch]
                    set_subtitle: &self.keyed_source_text(KnownServiceAccess::malware_bazaar_blocker, bazaar_source),
                    #[watch]
                    set_css_classes: self.keyed_source_styles(KnownServiceAccess::malware_bazaar_blocker),

                    add_suffix = &adw::Spinner {

                        set_halign: gtk::Align::Center,

                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_visible: self.keyed_source_checking(KnownServiceAccess::malware_bazaar_blocker),
                    },

                    add_suffix = &gtk::Button {
                        set_label: &translated_messages::findings::set_up_source_button(),
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_visible: self.keyed_source_blocked(KnownServiceAccess::malware_bazaar_blocker),
                        set_action_name: Some(ONLINE_SERVICES_ACTION),
                    },
                },

                add = &source_row("VirusTotal") -> adw::ActionRow {
                    #[watch]
                    set_subtitle: &self.keyed_source_text(KnownServiceAccess::virustotal_blocker, virustotal_source),
                    #[watch]
                    set_css_classes: self.keyed_source_styles(KnownServiceAccess::virustotal_blocker),

                    add_suffix = &adw::Spinner {

                        set_halign: gtk::Align::Center,

                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_visible: self.keyed_source_checking(KnownServiceAccess::virustotal_blocker),
                    },

                    add_suffix = &gtk::Button {
                        set_label: &translated_messages::findings::set_up_source_button(),
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_visible: self.keyed_source_blocked(KnownServiceAccess::virustotal_blocker),
                        set_action_name: Some(ONLINE_SERVICES_ACTION),
                    },

                    add_suffix = &gtk::Button {
                        set_label: &translated_messages::findings::view_report_label(),
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_visible: self.virustotal_report_url().is_some(),
                        connect_clicked => QuarantineEntryDetailInput::VirusTotalReportRequested,
                    },
                },

                add = &adw::ActionRow {
                    set_title: &translated_messages::findings::submit_sample_row_title(),
                    set_subtitle: &translated_messages::findings::send_clamav_malwarebazaar_tooltip(),
                    set_subtitle_lines: 0,

                    add_suffix = &gtk::Button {
                        set_label: &translated_messages::findings::submit_quarantine_label(),
                        set_valign: gtk::Align::Center,
                        set_sensitive: self.known_sha256_hex().is_some(),
                        connect_clicked => QuarantineEntryDetailInput::SubmissionRequested,
                    },
                },
            },

            adw::PreferencesGroup {
                set_title: &translated_messages::findings::false_positive_title(),

                add = &adw::ActionRow {
                    set_title: &translated_messages::findings::report_false_positive_row_title(),
                    set_subtitle: &translated_messages::findings::report_false_positive_row_subtitle(),
                    set_subtitle_lines: 0,
                    set_visible: quarantined_false_positive_detection(&self.entry_record).is_some(),

                    add_suffix = &gtk::Button {
                        set_label: &translated_messages::findings::report_label(),
                        set_valign: gtk::Align::Center,
                        connect_clicked => QuarantineEntryDetailInput::FalsePositiveReportRequested,
                    },
                },

                add = &adw::ActionRow {
                    set_title: &translated_messages::findings::trust_file_row_title(),
                    set_subtitle: &translated_messages::findings::trust_file_row_subtitle(),
                    set_subtitle_lines: 0,

                    add_suffix = &gtk::Button {
                        set_label: &translated_messages::findings::trust_label(),
                        set_valign: gtk::Align::Center,
                        set_sensitive: self.known_sha256_hex().is_some(),
                        connect_clicked => QuarantineEntryDetailInput::TrustFileRequested,
                    },
                },
            },
        }
    }

    fn init_model(
        QuarantineRowInit {
            moment_formatters,
            entry_record,
            row_state,
        }: QuarantineRowInit,
        _row_index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            moment_formatters,
            entry_record,
            row_state,
        }
    }

    fn update(&mut self, row_input: QuarantineEntryDetailInput, sender: FactorySender<Self>) {
        match row_input {
            QuarantineEntryDetailInput::ReputationCheckRequested => {
                let Some(content_hash) = self.known_sha256_hex().map(str::to_owned) else {
                    return;
                };
                let _page_closed =
                    sender.output(QuarantineEntryDetailOutput::ReputationCheckRequested {
                        entry_id: self.entry_record.entry_id,
                        content_hash,
                    });
            }
            QuarantineEntryDetailInput::HashCopyRequested => {
                if let Some(sha256_hex) = self.known_sha256_hex() {
                    let _page_closed = sender.output(
                        QuarantineEntryDetailOutput::HashCopyRequested(sha256_hex.to_owned()),
                    );
                }
            }
            QuarantineEntryDetailInput::TrustFileRequested => {
                let Some(sha256_hex) = self.known_sha256_hex().map(str::to_owned) else {
                    return;
                };
                let _page_closed = sender.output(QuarantineEntryDetailOutput::TrustRequested(
                    TrustChoice::TrustFile {
                        file_fingerprint: FileFingerprint {
                            sha256_hex,
                            file_size: self.entry_record.file_size,
                        },
                        shown_path: sanitize_scanner_line(
                            &self.entry_record.original_path.to_string_lossy(),
                        ),
                        trust_scope: TrustScope::OnlyThisUser,
                    },
                ));
            }
            QuarantineEntryDetailInput::SubmissionRequested => {
                let _page_closed =
                    sender.output(QuarantineEntryDetailOutput::SubmissionRequested {
                        entry_id: self.entry_record.entry_id,
                    });
            }
            QuarantineEntryDetailInput::RowStateChanged(row_state) => {
                self.row_state = *row_state;
            }
            QuarantineEntryDetailInput::FalsePositiveReportRequested => {
                let Some(reported_detection) =
                    quarantined_false_positive_detection(&self.entry_record)
                else {
                    return;
                };
                let _page_closed = sender.output(
                    QuarantineEntryDetailOutput::FalsePositiveReportRequested(reported_detection),
                );
            }
            QuarantineEntryDetailInput::VirusTotalReportRequested => {
                if let Some(report_url) = self.virustotal_report_url() {
                    let _page_closed = sender.output(
                        QuarantineEntryDetailOutput::ReportLinkRequested(report_url.to_owned()),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod quarantine_entry_detail_tests {
    use std::path::PathBuf;

    use super::*;
    use crate::host_services::clamav_scanner::scanner_output_parser::ThreatIdentity;

    fn quarantined_entry(threat_identity: ThreatIdentity) -> QuarantineEntryRecord {
        QuarantineEntryRecord {
            entry_id: 7,
            original_path: PathBuf::from("/home/user/Downloads/blocked.bin"),
            quarantine_path: PathBuf::from("/vault/payload"),
            threat_identity,
            detection_date: StoredDetectionDate::from("2026-11-01T00:00:00Z".to_owned()),
            file_size: bytesize::ByteSize::b(1),
            payload_hash: RecordedPayloadHash::IntegrityUnknown,
            original_permissions: 0o644,
        }
    }

    #[test]
    fn a_quarantined_on_access_block_cannot_be_reported_as_a_false_positive() {
        assert_eq!(
            quarantined_false_positive_detection(&quarantined_entry(
                ThreatIdentity::BlockedByOnAccess
            )),
            None
        );
    }

    #[test]
    fn a_quarantined_signature_is_reported_under_its_name() {
        assert_eq!(
            quarantined_false_positive_detection(&quarantined_entry(ThreatIdentity::Signature {
                signature_name: "Win.Test.EICAR_HDB-1".to_owned(),
            }))
            .map(|report_proposal| report_proposal.report_kind),
            Some(ClamavReportKind::FalsePositive {
                detection_name: "Win.Test.EICAR_HDB-1".to_owned(),
            })
        );
    }
}
