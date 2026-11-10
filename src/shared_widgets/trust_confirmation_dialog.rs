use relm4::{Sender, adw, adw::prelude::*, gtk};

use crate::{
    host_services::{
        clamav_admin_service::trust_requests::{
            FileFingerprint, SystemWideTrust, TrustChoice, TrustScope, system_wide_trust,
        },
        desktop::{flatpak_sandbox::SandboxKind, locale_formatting::exact_byte_size_text},
    },
    shared_widgets::awaited_dialogs::{
        AlertChoice, add_alert_choice, add_dismissing_choice, alert_choice, deliver_local_outcome,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TrustDialogChoice {
    TrustConfirmed,
    TrustForEveryUserConfirmed,
    TrustCancelled,
}

impl AlertChoice for TrustDialogChoice {
    const ANSWERING_CHOICES: &'static [Self] =
        &[Self::TrustConfirmed, Self::TrustForEveryUserConfirmed];
    const DISMISSING_CHOICE: Self = Self::TrustCancelled;

    fn response_id(self) -> &'static str {
        match self {
            Self::TrustConfirmed => "confirm",
            Self::TrustForEveryUserConfirmed => "confirm-every-user",
            Self::TrustCancelled => "cancel",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProtectionEffect {
    LowersProtection,
    RestoresProtection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EveryUserOffer {
    OffersEveryUser,
    ThisChoiceOnly,
}

struct ConfirmationText {
    heading: String,
    body: String,
    shown_value: String,
    confirm_label: String,
    protection_effect: ProtectionEffect,
    every_user_offer: EveryUserOffer,
}

fn fingerprint_text(
    message_catalog: &MessageCatalog,
    file_fingerprint: &FileFingerprint,
) -> String {
    translated_messages::widgets::sha_256_hash_size(
        message_catalog,
        &file_fingerprint.sha256_hex,
        exact_byte_size_text(message_catalog, file_fingerprint.file_size),
    )
}

fn confirmation_text(
    message_catalog: &MessageCatalog,
    sandbox_kind: SandboxKind,
    trust_choice: &TrustChoice,
) -> ConfirmationText {
    match trust_choice {
        TrustChoice::TrustFile {
            file_fingerprint,
            shown_path,
            trust_scope: TrustScope::OnlyThisUser,
        } => ConfirmationText {
            heading: translated_messages::widgets::trust_file_heading(message_catalog),
            body: translated_messages::widgets::stop_showing_file_body(message_catalog, shown_path),
            shown_value: fingerprint_text(message_catalog, file_fingerprint),
            confirm_label: translated_messages::widgets::trust_for_me_button(message_catalog),
            protection_effect: ProtectionEffect::LowersProtection,
            every_user_offer: match system_wide_trust(sandbox_kind) {
                SystemWideTrust::OfferedOutsideSandbox => EveryUserOffer::OffersEveryUser,
                SystemWideTrust::UnavailableInFlatpak => EveryUserOffer::ThisChoiceOnly,
            },
        },
        TrustChoice::TrustFile {
            file_fingerprint,
            shown_path,
            trust_scope: TrustScope::EveryUser,
        } => ConfirmationText {
            heading: translated_messages::widgets::trust_file_heading(message_catalog),
            body: translated_messages::widgets::clamav_never_report_body(
                message_catalog,
                shown_path,
            ),
            shown_value: fingerprint_text(message_catalog, file_fingerprint),
            confirm_label: translated_messages::widgets::trust_for_all_users_button(
                message_catalog,
            ),
            protection_effect: ProtectionEffect::LowersProtection,
            every_user_offer: EveryUserOffer::ThisChoiceOnly,
        },
        TrustChoice::IgnoreSignatureForThisUser { signature_name } => ConfirmationText {
            heading: translated_messages::widgets::ignore_detection_heading(message_catalog),
            body: translated_messages::widgets::stop_showing_detection_body(message_catalog),
            shown_value: signature_name.clone(),
            confirm_label: translated_messages::widgets::ignore_button(message_catalog),
            protection_effect: ProtectionEffect::LowersProtection,
            every_user_offer: EveryUserOffer::ThisChoiceOnly,
        },
        TrustChoice::ForgetTrustedFile {
            sha256_hex,
            trust_scope,
        } => ConfirmationText {
            heading: translated_messages::widgets::stop_trusting_file_heading(message_catalog),
            body: match trust_scope {
                TrustScope::OnlyThisUser => {
                    translated_messages::widgets::show_file_again_body(message_catalog)
                }
                TrustScope::EveryUser => {
                    translated_messages::widgets::clamav_report_file_body(message_catalog)
                }
            },
            shown_value: format!("SHA-256 {sha256_hex}"),
            confirm_label: translated_messages::widgets::stop_trusting_button(message_catalog),
            protection_effect: ProtectionEffect::RestoresProtection,
            every_user_offer: EveryUserOffer::ThisChoiceOnly,
        },
        TrustChoice::StopIgnoringSignature {
            signature_name,
            trust_scope,
        } => ConfirmationText {
            heading: translated_messages::widgets::report_detection_again_heading(message_catalog),
            body: match trust_scope {
                TrustScope::OnlyThisUser => {
                    translated_messages::widgets::show_detection_again_body(message_catalog)
                }
                TrustScope::EveryUser => {
                    translated_messages::widgets::clamav_report_files_body(message_catalog)
                }
            },
            shown_value: signature_name.clone(),
            confirm_label: translated_messages::widgets::report_again_button(message_catalog),
            protection_effect: ProtectionEffect::RestoresProtection,
            every_user_offer: EveryUserOffer::ThisChoiceOnly,
        },
    }
}

fn shown_value_label(shown_value: &str) -> gtk::Label {
    gtk::Label::builder()
        .label(shown_value)
        .use_markup(false)
        .selectable(true)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::Char)
        .css_classes(["monospace", "card"])
        .margin_top(6)
        .build()
}

pub fn present_trust_confirmation<Message: 'static>(
    message_catalog: &MessageCatalog,
    sandbox_kind: SandboxKind,
    parent_widget: &impl IsA<gtk::Widget>,
    trust_choice: TrustChoice,
    message_sender: Sender<Message>,
    confirmed_message: fn(TrustChoice) -> Message,
) {
    let ConfirmationText {
        heading,
        body,
        shown_value,
        confirm_label,
        protection_effect,
        every_user_offer,
    } = confirmation_text(message_catalog, sandbox_kind, &trust_choice);
    let confirmation_dialog = adw::AlertDialog::new(Some(&heading), Some(&body));
    confirmation_dialog.set_extra_child(Some(&shown_value_label(&shown_value)));
    add_dismissing_choice::<TrustDialogChoice>(
        &confirmation_dialog,
        &translated_messages::widgets::cancel_trust(message_catalog),
    );
    match every_user_offer {
        EveryUserOffer::OffersEveryUser => add_alert_choice(
            &confirmation_dialog,
            TrustDialogChoice::TrustForEveryUserConfirmed,
            &translated_messages::widgets::trust_for_all_users_button(message_catalog),
        ),
        EveryUserOffer::ThisChoiceOnly => {}
    }
    add_alert_choice(
        &confirmation_dialog,
        TrustDialogChoice::TrustConfirmed,
        &confirm_label,
    );
    confirmation_dialog.set_response_appearance(
        TrustDialogChoice::TrustConfirmed.response_id(),
        match protection_effect {
            ProtectionEffect::LowersProtection => adw::ResponseAppearance::Destructive,
            ProtectionEffect::RestoresProtection => adw::ResponseAppearance::Suggested,
        },
    );
    confirmation_dialog.set_default_response(Some(TrustDialogChoice::TrustCancelled.response_id()));
    deliver_local_outcome(
        message_sender,
        alert_choice(confirmation_dialog, parent_widget.clone().upcast()),
        move |trust_dialog_choice| match trust_dialog_choice {
            TrustDialogChoice::TrustConfirmed => Some(confirmed_message(trust_choice)),
            TrustDialogChoice::TrustForEveryUserConfirmed => {
                Some(confirmed_message(trust_choice.for_every_user()))
            }
            TrustDialogChoice::TrustCancelled => None,
        },
    );
}

#[cfg(test)]
mod trust_confirmation_dialog_tests {
    use bytesize::ByteSize;

    use super::*;

    fn trust_file_choice(trust_scope: TrustScope) -> TrustChoice {
        TrustChoice::TrustFile {
            file_fingerprint: FileFingerprint {
                sha256_hex: "00".repeat(32),
                file_size: ByteSize::b(3),
            },
            shown_path: "/home/user/file".to_owned(),
            trust_scope,
        }
    }

    #[test]
    fn trusting_lowers_protection_and_forgetting_does_not() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let sandbox_kind = SandboxKind::HostSystem;
        assert_eq!(
            confirmation_text(
                message_catalog,
                sandbox_kind,
                &TrustChoice::IgnoreSignatureForThisUser {
                    signature_name: "Win.Test.EICAR_HDB-1".to_owned()
                }
            )
            .protection_effect,
            ProtectionEffect::LowersProtection
        );
        assert_eq!(
            confirmation_text(
                message_catalog,
                sandbox_kind,
                &TrustChoice::ForgetTrustedFile {
                    sha256_hex: "00".to_owned(),
                    trust_scope: TrustScope::OnlyThisUser,
                }
            )
            .protection_effect,
            ProtectionEffect::RestoresProtection
        );
    }

    #[test]
    fn only_trusting_a_file_for_this_user_offers_every_user() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let sandbox_kind = SandboxKind::HostSystem;
        assert_eq!(
            confirmation_text(
                message_catalog,
                sandbox_kind,
                &trust_file_choice(TrustScope::EveryUser)
            )
            .every_user_offer,
            EveryUserOffer::ThisChoiceOnly
        );
        assert_eq!(
            confirmation_text(
                message_catalog,
                sandbox_kind,
                &TrustChoice::IgnoreSignatureForThisUser {
                    signature_name: "Win.Test.EICAR_HDB-1".to_owned()
                }
            )
            .every_user_offer,
            EveryUserOffer::ThisChoiceOnly
        );
        assert_eq!(
            trust_file_choice(TrustScope::OnlyThisUser).for_every_user(),
            trust_file_choice(TrustScope::EveryUser)
        );
    }
}
