#![allow(dead_code)]

use fluent_bundle::{FluentArgs, FluentResource, concurrent::FluentBundle};
use unic_langid::LanguageIdentifier;

pub(crate) struct BundledLanguageCatalog {
    pub(crate) language_tag: &'static str,
    pub(crate) fluent_sources: &'static [&'static str],
}

include!(concat!(env!("OUT_DIR"), "/generated_messages.rs"));

const PLACEHOLDER_SENTINEL: &str = "\u{1f}";

pub(crate) struct MessageCatalog {
    source_language_bundle: FluentBundle<FluentResource>,
}

impl MessageCatalog {
    pub(crate) fn source_language() -> Self {
        let source_language = SOURCE_LANGUAGE_TAG
            .parse::<LanguageIdentifier>()
            .unwrap_or_default();
        let mut source_language_bundle = FluentBundle::new_concurrent(vec![source_language]);
        source_language_bundle.set_use_isolating(false);
        BUNDLED_LANGUAGE_CATALOGS
            .iter()
            .filter(|language_catalog| language_catalog.language_tag == SOURCE_LANGUAGE_TAG)
            .flat_map(|language_catalog| language_catalog.fluent_sources.iter())
            .for_each(|fluent_source| {
                let catalog_resource = FluentResource::try_new((*fluent_source).to_owned())
                    .unwrap_or_else(|(partial_resource, _syntax_errors)| partial_resource);
                let _overridden_entries = source_language_bundle.add_resource(catalog_resource);
            });
        Self {
            source_language_bundle,
        }
    }

    pub(crate) fn translated_text(
        &self,
        message_id: &str,
        message_arguments: Option<&FluentArgs<'_>>,
    ) -> String {
        let Some(message_pattern) = self
            .source_language_bundle
            .get_message(message_id)
            .and_then(|catalog_message| catalog_message.value())
        else {
            panic!("the English catalogue has no message {message_id}");
        };
        let mut formatting_errors = Vec::new();
        self.source_language_bundle
            .format_pattern(message_pattern, message_arguments, &mut formatting_errors)
            .into_owned()
    }
}

pub fn text_before_placeholder(format_message: impl FnOnce(&'static str) -> String) -> String {
    let formatted_text = format_message(PLACEHOLDER_SENTINEL);
    formatted_text
        .split(PLACEHOLDER_SENTINEL)
        .next()
        .unwrap_or_default()
        .to_owned()
}
