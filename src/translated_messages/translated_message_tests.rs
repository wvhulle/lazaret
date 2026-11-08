use fluent_bundle::{FluentArgs, FluentValue};

use super::{fallback_language_chain, formatted_in_chain};

#[test]
fn requested_language_is_used_when_translated() {
    let german_chain = fallback_language_chain(&["de-DE".to_owned(), "de".to_owned()]);
    assert_eq!(
        formatted_in_chain(&german_chain, "window-cancel-history-button", None).as_deref(),
        Some("Abbrechen")
    );
}

#[test]
fn unknown_language_falls_back_to_english() {
    let unknown_chain = fallback_language_chain(&["xx".to_owned()]);
    assert_eq!(
        formatted_in_chain(&unknown_chain, "window-cancel-history-button", None).as_deref(),
        Some("Cancel")
    );
}

#[test]
fn brand_term_renders_application_name() {
    let english_chain = fallback_language_chain(&[]);
    assert_eq!(
        formatted_in_chain(&english_chain, "window-about-menu-item", None).as_deref(),
        Some("About Lazaret")
    );
}

#[test]
fn plural_variant_follows_count() {
    let english_chain = fallback_language_chain(&[]);
    let item_count_text = |item_count: u64| {
        let mut message_arguments = FluentArgs::new();
        message_arguments.set("count", FluentValue::from(item_count));
        formatted_in_chain(
            &english_chain,
            "window-count-item",
            Some(&message_arguments),
        )
    };
    assert_eq!(item_count_text(1).as_deref(), Some("1 item"));
    assert_eq!(item_count_text(1234).as_deref(), Some("1234 items"));
}

#[test]
fn choice_selects_the_sentence_for_that_credential() {
    let german_chain = fallback_language_chain(&["de".to_owned()]);
    let credential_text = |credential_key: &str| {
        let mut message_arguments = FluentArgs::new();
        message_arguments.set("credential", FluentValue::from(credential_key.to_owned()));
        message_arguments.set("provider", FluentValue::from("SecuriteInfo"));
        message_arguments.set("path", FluentValue::from("/etc/fangfrisch/fangfrisch.conf"));
        formatted_in_chain(
            &german_chain,
            "signatures-provider-needs-credential-website",
            Some(&message_arguments),
        )
        .expect("message renders")
    };
    assert!(credential_text("purchase-receipt").contains("Kaufbelegnummer"));
    assert!(credential_text("customer-identifier").contains("Kundennummer"));
}

#[test]
fn translation_agrees_with_the_second_count() {
    let german_chain = fallback_language_chain(&["de".to_owned()]);
    let verdict_text = |detecting_count: u64| {
        let mut message_arguments = FluentArgs::new();
        message_arguments.set("total", FluentValue::from(70));
        message_arguments.set("count", FluentValue::from(detecting_count));
        formatted_in_chain(
            &german_chain,
            "host-count-total-engine-verdict",
            Some(&message_arguments),
        )
        .expect("message renders")
    };
    assert!(verdict_text(1).contains("erkennt"));
    assert!(verdict_text(3).contains("erkennen"));
}
