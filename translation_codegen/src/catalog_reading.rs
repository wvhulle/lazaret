use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use fluent_syntax::{ast::Entry, parser};
use unic_langid::LanguageIdentifier;

use crate::{catalog_problems::CatalogProblem, pattern_references::PatternReferences};

pub const FLUENT_FILE_EXTENSION: &str = "ftl";

#[derive(Debug, Clone)]
pub struct CatalogMessage {
    pub message_id: String,
    pub source_path: PathBuf,
    pub line_number: usize,
    pub has_value: bool,
    pub attribute_count: usize,
    pub references: PatternReferences,
}

#[derive(Debug, Clone)]
pub struct LanguageCatalog {
    pub language_tag: String,
    pub source_paths: Vec<PathBuf>,
    pub messages: Vec<CatalogMessage>,
    pub term_ids: BTreeSet<String>,
}

impl LanguageCatalog {
    pub fn message_named(&self, message_id: &str) -> Option<&CatalogMessage> {
        self.messages
            .iter()
            .find(|catalog_message| catalog_message.message_id == message_id)
    }
}

fn sorted_directory_entries(directory: &Path) -> Result<Vec<PathBuf>, CatalogProblem> {
    let directory_reader =
        fs::read_dir(directory).map_err(|read_failure| CatalogProblem::UnreadablePath {
            path: directory.to_path_buf(),
            reason: read_failure.to_string(),
        })?;
    let mut entry_paths = Vec::new();
    for directory_entry in directory_reader {
        let directory_entry =
            directory_entry.map_err(|read_failure| CatalogProblem::UnreadablePath {
                path: directory.to_path_buf(),
                reason: read_failure.to_string(),
            })?;
        entry_paths.push(directory_entry.path());
    }
    entry_paths.sort();
    Ok(entry_paths)
}

fn line_number_at(source_text: &str, byte_offset: usize) -> usize {
    source_text[..byte_offset.min(source_text.len())]
        .matches('\n')
        .count()
        + 1
}

fn line_number_of_entry(source_text: &str, entry_id: &str) -> usize {
    source_text
        .lines()
        .position(|source_line| {
            source_line
                .strip_prefix(entry_id)
                .is_some_and(|after_id| after_id.trim_start().starts_with('='))
        })
        .map_or(1, |line_index| line_index + 1)
}

fn merged_references(fluent_message: &fluent_syntax::ast::Message<&str>) -> PatternReferences {
    let mut references = fluent_message
        .value
        .as_ref()
        .map(PatternReferences::collected_from)
        .unwrap_or_default();
    for message_attribute in &fluent_message.attributes {
        references.merge(PatternReferences::collected_from(&message_attribute.value));
    }
    references
}

fn read_catalog_file(
    source_path: &Path,
    language_catalog: &mut LanguageCatalog,
    problems: &mut Vec<CatalogProblem>,
) {
    let source_text = match fs::read_to_string(source_path) {
        Ok(source_text) => source_text,
        Err(read_failure) => {
            problems.push(CatalogProblem::UnreadablePath {
                path: source_path.to_path_buf(),
                reason: read_failure.to_string(),
            });
            return;
        }
    };
    let parsed_resource = match parser::parse(source_text.as_str()) {
        Ok(parsed_resource) => parsed_resource,
        Err((partial_resource, syntax_errors)) => {
            for syntax_error in syntax_errors {
                problems.push(CatalogProblem::SyntaxError {
                    path: source_path.to_path_buf(),
                    line_number: line_number_at(&source_text, syntax_error.pos.start),
                    description: syntax_error.kind.to_string(),
                });
            }
            partial_resource
        }
    };
    language_catalog
        .source_paths
        .push(source_path.to_path_buf());
    for resource_entry in &parsed_resource.body {
        match resource_entry {
            Entry::Message(fluent_message) => {
                let message_id = fluent_message.id.name.to_owned();
                let line_number = line_number_of_entry(&source_text, &message_id);
                if language_catalog.message_named(&message_id).is_some() {
                    problems.push(CatalogProblem::DuplicateMessage {
                        path: source_path.to_path_buf(),
                        line_number,
                        message_id,
                    });
                    continue;
                }
                language_catalog.messages.push(CatalogMessage {
                    message_id,
                    source_path: source_path.to_path_buf(),
                    line_number,
                    has_value: fluent_message.value.is_some(),
                    attribute_count: fluent_message.attributes.len(),
                    references: merged_references(fluent_message),
                });
            }
            Entry::Term(fluent_term) => {
                language_catalog
                    .term_ids
                    .insert(fluent_term.id.name.to_owned());
            }
            Entry::Comment(_)
            | Entry::GroupComment(_)
            | Entry::ResourceComment(_)
            | Entry::Junk { .. } => {}
        }
    }
}

pub fn read_language_catalogs(
    locales_directory: &Path,
    problems: &mut Vec<CatalogProblem>,
) -> Vec<LanguageCatalog> {
    let language_directories = match sorted_directory_entries(locales_directory) {
        Ok(language_directories) => language_directories,
        Err(directory_problem) => {
            problems.push(directory_problem);
            return Vec::new();
        }
    };
    let mut language_catalogs = Vec::new();
    for language_directory in language_directories {
        if !language_directory.is_dir() {
            continue;
        }
        let language_tag = language_directory
            .file_name()
            .map(|directory_name| directory_name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let canonical_tag = language_tag
            .parse::<LanguageIdentifier>()
            .map(|language_identifier| language_identifier.to_string());
        if canonical_tag.as_deref() != Ok(language_tag.as_str()) {
            problems.push(CatalogProblem::InvalidLanguageDirectory {
                path: language_directory.clone(),
            });
            continue;
        }
        let catalog_paths = match sorted_directory_entries(&language_directory) {
            Ok(catalog_paths) => catalog_paths,
            Err(directory_problem) => {
                problems.push(directory_problem);
                continue;
            }
        };
        let mut language_catalog = LanguageCatalog {
            language_tag,
            source_paths: Vec::new(),
            messages: Vec::new(),
            term_ids: BTreeSet::new(),
        };
        for catalog_path in catalog_paths.iter().filter(|catalog_path| {
            catalog_path
                .extension()
                .is_some_and(|extension| extension == FLUENT_FILE_EXTENSION)
        }) {
            read_catalog_file(catalog_path, &mut language_catalog, problems);
        }
        language_catalogs.push(language_catalog);
    }
    language_catalogs
}
