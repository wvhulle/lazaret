mod catalog_consistency;
mod catalog_problems;
mod catalog_reading;
mod pattern_references;
mod rust_emission;

use std::path::Path;

pub use catalog_consistency::UntranslatedMessageCount;
pub use catalog_problems::CatalogProblem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedMessages {
    pub rust_source: String,
    pub untranslated_counts: Vec<UntranslatedMessageCount>,
}

pub fn generate_message_functions(
    locales_directory: &Path,
    source_language_tag: &str,
) -> Result<GeneratedMessages, Vec<CatalogProblem>> {
    let mut problems = Vec::new();
    let language_catalogs =
        catalog_reading::read_language_catalogs(locales_directory, &mut problems);
    let untranslated_counts = catalog_consistency::check_catalog_consistency(
        &language_catalogs,
        source_language_tag,
        &mut problems,
    );
    if !problems.is_empty() {
        return Err(problems);
    }
    Ok(GeneratedMessages {
        rust_source: rust_emission::emit_generated_messages(
            &language_catalogs,
            source_language_tag,
        ),
        untranslated_counts,
    })
}

#[cfg(test)]
mod catalog_generation_tests;
