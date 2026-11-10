use fluent_bundle::{FluentArgs, FluentResource, concurrent::FluentBundle};
use fluent_langneg::{NegotiationStrategy, negotiate_languages};
use unic_langid::LanguageIdentifier;

use crate::host_services::desktop::locale_formatting::host_language_tags;

pub(crate) struct BundledLanguageCatalog {
    pub(crate) language_tag: &'static str,
    pub(crate) fluent_sources: &'static [&'static str],
}

include!(concat!(env!("OUT_DIR"), "/generated_messages.rs"));

pub(crate) struct MessageCatalog {
    language_chain: Vec<FluentBundle<FluentResource>>,
}

impl std::fmt::Debug for MessageCatalog {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("MessageCatalog")
            .field("language_count", &self.language_chain.len())
            .finish()
    }
}

impl MessageCatalog {
    pub(crate) fn for_language_tags(requested_tags: &[String]) -> Self {
        Self {
            language_chain: fallback_language_chain(requested_tags),
        }
    }

    pub(crate) fn for_host_messages_locale() -> Self {
        Self::for_language_tags(&host_language_tags("LC_MESSAGES"))
    }

    pub(crate) fn translated_text(
        &self,
        message_id: &str,
        message_arguments: Option<&FluentArgs<'_>>,
    ) -> String {
        formatted_in_chain(&self.language_chain, message_id, message_arguments)
            .unwrap_or_else(|| message_id.to_owned())
    }
}

fn parsed_language(language_tag: &str) -> LanguageIdentifier {
    language_tag
        .parse()
        .unwrap_or_else(|_unparsable_tag| LanguageIdentifier::default())
}

fn load_language_bundle(language_catalog: &BundledLanguageCatalog) -> FluentBundle<FluentResource> {
    let mut language_bundle =
        FluentBundle::new_concurrent(vec![parsed_language(language_catalog.language_tag)]);
    language_bundle.set_use_isolating(false);
    let _builtin_functions_added = language_bundle.add_builtins();
    for fluent_source in language_catalog.fluent_sources {
        let catalog_resource = FluentResource::try_new((*fluent_source).to_owned())
            .unwrap_or_else(|(partial_resource, _syntax_errors)| partial_resource);
        let _overridden_entries = language_bundle.add_resource(catalog_resource);
    }
    language_bundle
}

fn fallback_language_chain(requested_tags: &[String]) -> Vec<FluentBundle<FluentResource>> {
    let requested_languages = requested_tags
        .iter()
        .filter_map(|requested_tag| requested_tag.parse::<LanguageIdentifier>().ok())
        .collect::<Vec<_>>();
    let available_languages = BUNDLED_LANGUAGE_CATALOGS
        .iter()
        .map(|language_catalog| parsed_language(language_catalog.language_tag))
        .collect::<Vec<_>>();
    let source_language = parsed_language(SOURCE_LANGUAGE_TAG);
    let mut chosen_languages = negotiate_languages(
        &requested_languages,
        &available_languages,
        Some(&source_language),
        NegotiationStrategy::Filtering,
    )
    .into_iter()
    .cloned()
    .collect::<Vec<_>>();
    if let Some(source_position) = chosen_languages
        .iter()
        .position(|chosen_language| *chosen_language == source_language)
    {
        chosen_languages.truncate(source_position);
    }
    chosen_languages.push(source_language);
    chosen_languages
        .iter()
        .filter_map(|chosen_language| {
            BUNDLED_LANGUAGE_CATALOGS
                .iter()
                .find(|language_catalog| {
                    parsed_language(language_catalog.language_tag) == *chosen_language
                })
                .map(load_language_bundle)
        })
        .collect()
}

fn formatted_in_chain(
    language_chain: &[FluentBundle<FluentResource>],
    message_id: &str,
    message_arguments: Option<&FluentArgs<'_>>,
) -> Option<String> {
    language_chain.iter().find_map(|language_bundle| {
        let message_pattern = language_bundle.get_message(message_id)?.value()?;
        let mut formatting_errors = Vec::new();
        let formatted_text = language_bundle.format_pattern(
            message_pattern,
            message_arguments,
            &mut formatting_errors,
        );
        formatting_errors
            .is_empty()
            .then(|| formatted_text.into_owned())
    })
}

#[cfg(test)]
mod translated_message_tests;

#[cfg(test)]
pub(crate) fn english_catalog() -> MessageCatalog {
    MessageCatalog::for_language_tags(&[])
}
