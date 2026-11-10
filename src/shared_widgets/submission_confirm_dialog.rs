use relm4::{Sender, adw, adw::prelude::*, gtk};

use crate::{
    host_services::{
        clamav_programs::clamav_report_submission::{
            ClamavReport, ClamavReportKind, ClamavReportRequest, ReporterContact, SubmittedContent,
            contact_is_complete,
        },
        threat_intel::malware_bazaar_client::bazaar_signature_tag,
    },
    shared_widgets::awaited_dialogs::{
        AlertChoice, add_alert_choice, add_dismissing_choice, alert_choice, deliver_local_outcome,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SubmissionDialogChoice {
    SendChosen,
    SendingCancelled,
}

impl AlertChoice for SubmissionDialogChoice {
    const ANSWERING_CHOICES: &'static [Self] = &[Self::SendChosen];
    const DISMISSING_CHOICE: Self = Self::SendingCancelled;

    fn response_id(self) -> &'static str {
        match self {
            Self::SendChosen => "send",
            Self::SendingCancelled => "cancel",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClamavReportProposal {
    pub report_kind: ClamavReportKind,
    pub submitted_content: SubmittedContent,
    pub shown_path: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UploadService {
    MalwareBazaarUpload { signature_name: String },
    VirusTotalUpload,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SampleUploadProposal {
    pub upload_service: UploadService,
    pub entry_id: i64,
    pub shown_path: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubmissionProposal {
    ClamavReportProposed(ClamavReportProposal),
    SampleUploadProposed(SampleUploadProposal),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfirmationText {
    pub heading: String,
    pub body_text: String,
    pub confirm_label: String,
}

fn sent_items_text(sent_items: &[String]) -> String {
    sent_items
        .iter()
        .map(|sent_item| format!("• {sent_item}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn assembled_body(
    message_catalog: &MessageCatalog,
    file_line: String,
    sent_items: &[String],
    audience_text: String,
    shown_path: &str,
) -> String {
    [
        file_line,
        shown_path.to_owned(),
        translated_messages::widgets::what_is_sent_label(message_catalog),
        sent_items_text(sent_items),
        audience_text,
    ]
    .join("\n\n")
}

fn clamav_audience_text(message_catalog: &MessageCatalog) -> String {
    translated_messages::widgets::clamav_audience(message_catalog)
}

pub fn confirmation_text(
    message_catalog: &MessageCatalog,
    submission_proposal: &SubmissionProposal,
) -> ConfirmationText {
    match submission_proposal {
        SubmissionProposal::ClamavReportProposed(report_proposal) => {
            let (heading, mut sent_items) = match &report_proposal.report_kind {
                ClamavReportKind::FalsePositive { detection_name } => (
                    translated_messages::widgets::report_false_positive_clamav(message_catalog),
                    vec![
                        translated_messages::widgets::file_content(message_catalog),
                        translated_messages::widgets::detection_name(
                            message_catalog,
                            detection_name,
                        ),
                    ],
                ),
                ClamavReportKind::MissedMalware => (
                    translated_messages::widgets::report_missed_malware_clamav(message_catalog),
                    vec![translated_messages::widgets::file_content(message_catalog)],
                ),
            };
            sent_items.push(translated_messages::widgets::name_email_address_talos(
                message_catalog,
            ));
            ConfirmationText {
                heading,
                body_text: assembled_body(
                    message_catalog,
                    translated_messages::widgets::lazaret_sends_file_clamav(message_catalog),
                    &sent_items,
                    clamav_audience_text(message_catalog),
                    &report_proposal.shown_path,
                ),
                confirm_label: translated_messages::widgets::send_report_button(message_catalog),
            }
        }
        SubmissionProposal::SampleUploadProposed(upload_proposal) => {
            sample_upload_text(message_catalog, upload_proposal)
        }
    }
}

fn sample_upload_text(
    message_catalog: &MessageCatalog,
    upload_proposal: &SampleUploadProposal,
) -> ConfirmationText {
    let hashed_content = translated_messages::widgets::file_content_named_sha(message_catalog);
    match &upload_proposal.upload_service {
        UploadService::MalwareBazaarUpload { signature_name } => ConfirmationText {
            heading: translated_messages::widgets::upload_malwarebazaar_heading(message_catalog),
            body_text: assembled_body(
                message_catalog,
                translated_messages::widgets::lazaret_uploads_file_malwarebazaar(message_catalog),
                &[
                    hashed_content,
                    translated_messages::widgets::tag_clamav_detection(
                        message_catalog,
                        bazaar_signature_tag(signature_name),
                    ),
                    translated_messages::widgets::abuse_ch_key_sample(message_catalog),
                ],
                translated_messages::widgets::malwarebazaar_public_anyone_download(message_catalog),
                &upload_proposal.shown_path,
            ),
            confirm_label: translated_messages::widgets::upload_publicly_button(message_catalog),
        },
        UploadService::VirusTotalUpload => ConfirmationText {
            heading: translated_messages::widgets::upload_virustotal_heading(message_catalog),
            body_text: assembled_body(
                message_catalog,
                translated_messages::widgets::lazaret_uploads_file_virustotal(message_catalog),
                &[
                    hashed_content,
                    translated_messages::widgets::virustotal_key_links_upload(message_catalog),
                ],
                translated_messages::widgets::virustotal_shares_uploaded_files(message_catalog),
                &upload_proposal.shown_path,
            ),
            confirm_label: translated_messages::widgets::upload_button(message_catalog),
        },
    }
}

fn contact_from_rows(name_row: &adw::EntryRow, email_row: &adw::EntryRow) -> ReporterContact {
    ReporterContact {
        reporter_name: name_row.text().to_string(),
        reporter_email: email_row.text().to_string(),
    }
}

fn confirmation_dialog(
    message_catalog: &MessageCatalog,
    confirmation: &ConfirmationText,
) -> adw::AlertDialog {
    let submission_dialog =
        adw::AlertDialog::new(Some(&confirmation.heading), Some(&confirmation.body_text));
    add_dismissing_choice::<SubmissionDialogChoice>(
        &submission_dialog,
        &translated_messages::widgets::cancel_submission(message_catalog),
    );
    add_alert_choice(
        &submission_dialog,
        SubmissionDialogChoice::SendChosen,
        &confirmation.confirm_label,
    );
    submission_dialog.set_response_appearance(
        SubmissionDialogChoice::SendChosen.response_id(),
        adw::ResponseAppearance::Suggested,
    );
    submission_dialog
        .set_default_response(Some(SubmissionDialogChoice::SendingCancelled.response_id()));
    submission_dialog
}

pub fn present_clamav_report_confirmation<Message: 'static>(
    message_catalog: &MessageCatalog,
    parent_widget: &impl IsA<gtk::Widget>,
    report_proposal: ClamavReportProposal,
    message_sender: Sender<Message>,
    confirmed_message: fn(ClamavReportRequest) -> Message,
) {
    let report_dialog = confirmation_dialog(
        message_catalog,
        &confirmation_text(
            message_catalog,
            &SubmissionProposal::ClamavReportProposed(report_proposal.clone()),
        ),
    );
    let name_row = adw::EntryRow::builder()
        .title(translated_messages::widgets::name_title(message_catalog))
        .build();
    let email_row = adw::EntryRow::builder()
        .title(translated_messages::widgets::email_address_title(
            message_catalog,
        ))
        .input_purpose(gtk::InputPurpose::Email)
        .build();
    let contact_list = gtk::ListBox::builder()
        .css_classes(["boxed-list"])
        .selection_mode(gtk::SelectionMode::None)
        .build();
    contact_list.append(&name_row);
    contact_list.append(&email_row);
    report_dialog.set_extra_child(Some(&contact_list));
    let send_response = SubmissionDialogChoice::SendChosen.response_id();
    report_dialog.set_response_enabled(send_response, false);
    [&name_row, &email_row].into_iter().for_each(|contact_row| {
        let watched_dialog = report_dialog.clone();
        let watched_name = name_row.clone();
        let watched_email = email_row.clone();
        contact_row.connect_changed(move |_edited_row| {
            watched_dialog.set_response_enabled(
                send_response,
                contact_is_complete(&contact_from_rows(&watched_name, &watched_email)),
            );
        });
    });
    deliver_local_outcome(
        message_sender,
        alert_choice(report_dialog, parent_widget.clone().upcast()),
        move |report_choice| match report_choice {
            SubmissionDialogChoice::SendChosen => Some(confirmed_message(ClamavReportRequest {
                clamav_report: ClamavReport {
                    reporter_contact: contact_from_rows(&name_row, &email_row),
                    report_kind: report_proposal.report_kind,
                },
                submitted_content: report_proposal.submitted_content,
            })),
            SubmissionDialogChoice::SendingCancelled => None,
        },
    );
}

pub fn present_sample_upload_confirmation<Message: 'static>(
    message_catalog: &MessageCatalog,
    parent_widget: &impl IsA<gtk::Widget>,
    upload_proposal: SampleUploadProposal,
    message_sender: Sender<Message>,
    confirmed_message: fn(SampleUploadProposal) -> Message,
) {
    let upload_dialog = confirmation_dialog(
        message_catalog,
        &confirmation_text(
            message_catalog,
            &SubmissionProposal::SampleUploadProposed(upload_proposal.clone()),
        ),
    );
    upload_dialog.set_response_appearance(
        SubmissionDialogChoice::SendChosen.response_id(),
        adw::ResponseAppearance::Destructive,
    );
    deliver_local_outcome(
        message_sender,
        alert_choice(upload_dialog, parent_widget.clone().upcast()),
        move |upload_choice| match upload_choice {
            SubmissionDialogChoice::SendChosen => Some(confirmed_message(upload_proposal)),
            SubmissionDialogChoice::SendingCancelled => None,
        },
    );
}

#[cfg(test)]
mod submission_confirm_dialog_tests {
    use std::path::PathBuf;

    use super::*;

    fn clamav_proposal(report_kind: ClamavReportKind) -> SubmissionProposal {
        SubmissionProposal::ClamavReportProposed(ClamavReportProposal {
            report_kind,
            submitted_content: SubmittedContent::FileOnDisk {
                file_path: PathBuf::from("/home/alex/Downloads/invoice.exe"),
            },
            shown_path: "/home/alex/Downloads/invoice.exe".to_owned(),
        })
    }

    #[test]
    fn a_clamav_report_names_the_file_the_service_and_its_privacy() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let false_positive_text = confirmation_text(
            message_catalog,
            &clamav_proposal(ClamavReportKind::FalsePositive {
                detection_name: "Win.Trojan.Agent-1".to_owned(),
            }),
        );
        assert!(false_positive_text.heading.contains("ClamAV"));
        assert!(
            false_positive_text
                .body_text
                .contains("/home/alex/Downloads/invoice.exe")
        );
        assert!(false_positive_text.body_text.contains("Win.Trojan.Agent-1"));
        assert!(false_positive_text.body_text.contains("Cisco Talos"));
        assert!(false_positive_text.body_text.contains("never made public"));
        assert!(false_positive_text.body_text.contains("at least 48 hours"));
        assert!(false_positive_text.body_text.contains("email address"));
    }

    fn upload_proposal(upload_service: UploadService) -> SubmissionProposal {
        SubmissionProposal::SampleUploadProposed(SampleUploadProposal {
            upload_service,
            entry_id: 7,
            shown_path: "/home/alex/Downloads/invoice.exe".to_owned(),
        })
    }

    #[test]
    fn a_malware_bazaar_upload_says_the_sample_becomes_public() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let bazaar_text = confirmation_text(
            message_catalog,
            &upload_proposal(UploadService::MalwareBazaarUpload {
                signature_name: "Win.Test.EICAR_HDB-1".to_owned(),
            }),
        );
        assert!(bazaar_text.heading.contains("MalwareBazaar"));
        assert!(
            bazaar_text
                .body_text
                .contains("/home/alex/Downloads/invoice.exe")
        );
        assert!(bazaar_text.body_text.contains("public"));
        assert!(bazaar_text.body_text.contains("anyone can download"));
        assert!(bazaar_text.body_text.contains("Win.Test.EICAR-HDB-1"));
        assert!(!bazaar_text.body_text.contains("Cisco Talos"));
    }

    #[test]
    fn a_virustotal_upload_says_paying_customers_get_the_file() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let virustotal_text = confirmation_text(
            message_catalog,
            &upload_proposal(UploadService::VirusTotalUpload),
        );
        assert!(virustotal_text.heading.contains("VirusTotal"));
        assert!(virustotal_text.body_text.contains("invoice.exe"));
        assert!(virustotal_text.body_text.contains("paying customers"));
        assert!(!virustotal_text.body_text.contains("never made public"));
    }

    #[test]
    fn a_missed_malware_report_sends_no_detection_name() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let missed_text = confirmation_text(
            message_catalog,
            &clamav_proposal(ClamavReportKind::MissedMalware),
        );
        assert!(missed_text.heading.contains("Missed Malware"));
        assert!(!missed_text.body_text.contains("detection name"));
        assert!(missed_text.body_text.contains("invoice.exe"));
    }
}
