use std::collections::BTreeSet;

use crate::{
    catalog_problems::CatalogProblem,
    catalog_reading::{CatalogMessage, LanguageCatalog},
};

const RUST_KEYWORDS: &[&str] = &[
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
    "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "gen", "if", "impl",
    "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof",
    "union", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntranslatedMessageCount {
    pub language_tag: String,
    pub untranslated_count: usize,
    pub source_message_count: usize,
}

pub fn rust_name_for(fluent_name: &str) -> Option<String> {
    let rust_name = fluent_name.replace('-', "_");
    let starts_with_letter = rust_name
        .chars()
        .next()
        .is_some_and(|first_character| first_character.is_ascii_lowercase());
    let uses_plain_characters = rust_name.chars().all(|name_character| {
        name_character.is_ascii_lowercase()
            || name_character.is_ascii_digit()
            || name_character == '_'
    });
    (starts_with_letter && uses_plain_characters && !RUST_KEYWORDS.contains(&rust_name.as_str()))
        .then_some(rust_name)
}

pub struct MessageFunctionPath {
    pub module_name: String,
    pub function_name: String,
}

pub fn message_function_path(message_id: &str) -> Option<MessageFunctionPath> {
    let (module_prefix, message_name) = message_id.split_once('-')?;
    Some(MessageFunctionPath {
        module_name: rust_name_for(module_prefix)?,
        function_name: rust_name_for(message_name)?,
    })
}

fn check_references_resolve(
    language_catalog: &LanguageCatalog,
    catalog_message: &CatalogMessage,
    problems: &mut Vec<CatalogProblem>,
) {
    for referenced_id in &catalog_message.references.message_references {
        if language_catalog.message_named(referenced_id).is_none() {
            problems.push(CatalogProblem::UnresolvedMessageReference {
                path: catalog_message.source_path.clone(),
                line_number: catalog_message.line_number,
                message_id: catalog_message.message_id.clone(),
                referenced_id: referenced_id.clone(),
            });
        }
    }
    for referenced_id in &catalog_message.references.term_references {
        if !language_catalog.term_ids.contains(referenced_id) {
            problems.push(CatalogProblem::UnresolvedTermReference {
                path: catalog_message.source_path.clone(),
                line_number: catalog_message.line_number,
                message_id: catalog_message.message_id.clone(),
                referenced_id: referenced_id.clone(),
            });
        }
    }
}

fn check_source_message_shape(
    catalog_message: &CatalogMessage,
    function_paths: &mut BTreeSet<(String, String)>,
    problems: &mut Vec<CatalogProblem>,
) {
    let path = catalog_message.source_path.clone();
    let line_number = catalog_message.line_number;
    let message_id = catalog_message.message_id.clone();
    if !catalog_message.has_value {
        problems.push(CatalogProblem::SourceMessageWithoutValue {
            path: path.clone(),
            line_number,
            message_id: message_id.clone(),
        });
    }
    if catalog_message.attribute_count > 0 {
        problems.push(CatalogProblem::SourceMessageWithAttributes {
            path: path.clone(),
            line_number,
            message_id: message_id.clone(),
        });
    }
    if !message_id.contains('-') {
        problems.push(CatalogProblem::MessageIdWithoutModule {
            path: path.clone(),
            line_number,
            message_id: message_id.clone(),
        });
    } else {
        match message_function_path(&message_id) {
            Some(function_path) => {
                let qualified_name = format!(
                    "{}::{}",
                    function_path.module_name, function_path.function_name
                );
                if !function_paths.insert((function_path.module_name, function_path.function_name))
                {
                    problems.push(CatalogProblem::CollidingFunctionName {
                        path: path.clone(),
                        line_number,
                        function_name: qualified_name,
                    });
                }
            }
            None => problems.push(CatalogProblem::UnusableRustName {
                path: path.clone(),
                line_number,
                fluent_name: message_id.clone(),
            }),
        }
    }
    let mut parameter_names = BTreeSet::new();
    for variable_name in &catalog_message.references.variables_in_order {
        let parameter_is_usable = rust_name_for(variable_name)
            .is_some_and(|parameter_name| parameter_names.insert(parameter_name));
        if !parameter_is_usable {
            problems.push(CatalogProblem::UnusableRustName {
                path: path.clone(),
                line_number,
                fluent_name: format!("${variable_name}"),
            });
        }
    }
}

fn check_translated_message(
    translated_catalog: &LanguageCatalog,
    translated_message: &CatalogMessage,
    source_catalog: &LanguageCatalog,
    problems: &mut Vec<CatalogProblem>,
) -> bool {
    let Some(source_message) = source_catalog.message_named(&translated_message.message_id) else {
        problems.push(CatalogProblem::StaleTranslation {
            path: translated_message.source_path.clone(),
            line_number: translated_message.line_number,
            message_id: translated_message.message_id.clone(),
        });
        return false;
    };
    let source_variables = &source_message.references.variables_in_order;
    for variable_name in &translated_message.references.variables_in_order {
        if !source_variables.contains(variable_name) {
            problems.push(CatalogProblem::UnknownVariable {
                path: translated_message.source_path.clone(),
                line_number: translated_message.line_number,
                message_id: translated_message.message_id.clone(),
                variable_name: variable_name.clone(),
            });
        }
    }
    for variable_name in &translated_message.references.selector_variables {
        if source_variables.contains(variable_name)
            && !source_message
                .references
                .selector_variables
                .contains(variable_name)
        {
            problems.push(CatalogProblem::SelectorOnTextVariable {
                path: translated_message.source_path.clone(),
                line_number: translated_message.line_number,
                message_id: translated_message.message_id.clone(),
                variable_name: variable_name.clone(),
            });
        }
    }
    check_references_resolve(translated_catalog, translated_message, problems);
    true
}

pub fn check_catalog_consistency(
    language_catalogs: &[LanguageCatalog],
    source_language_tag: &str,
    problems: &mut Vec<CatalogProblem>,
) -> Vec<UntranslatedMessageCount> {
    let Some(source_catalog) = language_catalogs
        .iter()
        .find(|language_catalog| language_catalog.language_tag == source_language_tag)
    else {
        problems.push(CatalogProblem::MissingSourceLanguage {
            source_language_tag: source_language_tag.to_owned(),
        });
        return Vec::new();
    };
    let mut function_paths = BTreeSet::new();
    for source_message in &source_catalog.messages {
        check_source_message_shape(source_message, &mut function_paths, problems);
        check_references_resolve(source_catalog, source_message, problems);
    }
    language_catalogs
        .iter()
        .filter(|language_catalog| language_catalog.language_tag != source_language_tag)
        .map(|translated_catalog| {
            let translated_count = translated_catalog
                .messages
                .iter()
                .filter(|translated_message| {
                    check_translated_message(
                        translated_catalog,
                        translated_message,
                        source_catalog,
                        problems,
                    )
                })
                .count();
            UntranslatedMessageCount {
                language_tag: translated_catalog.language_tag.clone(),
                untranslated_count: source_catalog.messages.len() - translated_count,
                source_message_count: source_catalog.messages.len(),
            }
        })
        .collect()
}
