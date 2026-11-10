use std::{collections::BTreeMap, fmt::Write};

use crate::{
    catalog_consistency::{
        choice_type_name, message_function_path, pascal_case_name, rust_name_for,
    },
    catalog_reading::{CatalogMessage, LanguageCatalog},
    pattern_references::VariableKind,
};

fn parameter_bound(catalog_message: &CatalogMessage, variable_name: &str) -> String {
    match catalog_message.references.variable_kind(variable_name) {
        VariableKind::PlainText => "impl ::core::fmt::Display".to_owned(),
        VariableKind::CountedNumber => {
            "impl ::core::convert::Into<::fluent_bundle::types::FluentNumber>".to_owned()
        }
        VariableKind::NamedChoice(_) => choice_type_name(variable_name),
    }
}

fn argument_value(
    catalog_message: &CatalogMessage,
    variable_name: &str,
    parameter_name: &str,
) -> String {
    match catalog_message.references.variable_kind(variable_name) {
        VariableKind::PlainText => {
            format!("::fluent_bundle::FluentValue::from({parameter_name}.to_string())")
        }
        VariableKind::CountedNumber => {
            format!("::fluent_bundle::FluentValue::Number({parameter_name}.into())")
        }
        VariableKind::NamedChoice(_) => {
            format!("::fluent_bundle::FluentValue::from({parameter_name}.fluent_key())")
        }
    }
}

const CATALOG_PARAMETER: &str = "message_catalog: &super::MessageCatalog";

fn emit_parameter_list(catalog_message: &CatalogMessage) -> String {
    std::iter::once(CATALOG_PARAMETER.to_owned())
        .chain(
            catalog_message
                .references
                .variables_in_order
                .iter()
                .filter_map(|variable_name| {
                    let parameter_name = rust_name_for(variable_name)?;
                    Some(format!(
                        "{parameter_name}: {}",
                        parameter_bound(catalog_message, variable_name)
                    ))
                }),
        )
        .collect::<Vec<_>>()
        .join(", ")
}

fn emit_choice_types(generated_code: &mut String, module_messages: &[(String, &CatalogMessage)]) {
    let mut choice_types = BTreeMap::new();
    for (_, catalog_message) in module_messages {
        for variable_name in &catalog_message.references.variables_in_order {
            if let VariableKind::NamedChoice(choice_keys) =
                catalog_message.references.variable_kind(variable_name)
            {
                choice_types
                    .entry(choice_type_name(variable_name))
                    .or_insert(choice_keys);
            }
        }
    }
    for (type_name, choice_keys) in choice_types {
        let _ = writeln!(
            generated_code,
            "    #[derive(Clone, Copy, Debug, PartialEq, Eq)]\n    pub(crate) enum {type_name} {{"
        );
        for choice_key in &choice_keys {
            let _ = writeln!(generated_code, "        {},", pascal_case_name(choice_key));
        }
        let _ = writeln!(generated_code, "    }}");
        let _ = writeln!(
            generated_code,
            "    impl {type_name} {{\n        fn fluent_key(self) -> &'static str {{\n            \
             match self {{"
        );
        for choice_key in &choice_keys {
            let _ = writeln!(
                generated_code,
                "                Self::{} => {choice_key:?},",
                pascal_case_name(choice_key)
            );
        }
        let _ = writeln!(generated_code, "            }}\n        }}\n    }}");
    }
}

fn emit_message_function(
    generated_code: &mut String,
    function_name: &str,
    catalog_message: &CatalogMessage,
) {
    let message_id = &catalog_message.message_id;
    let parameter_list = emit_parameter_list(catalog_message);
    let _ = writeln!(
        generated_code,
        "    pub(crate) fn {function_name}({parameter_list}) -> ::std::string::String {{"
    );
    if catalog_message.references.variables_in_order.is_empty() {
        let _ = writeln!(
            generated_code,
            "        message_catalog.translated_text({message_id:?}, None)"
        );
    } else {
        let _ = writeln!(
            generated_code,
            "        let mut message_arguments = ::fluent_bundle::FluentArgs::new();"
        );
        for variable_name in &catalog_message.references.variables_in_order {
            let Some(parameter_name) = rust_name_for(variable_name) else {
                continue;
            };
            let fluent_argument = argument_value(catalog_message, variable_name, &parameter_name);
            let _ = writeln!(
                generated_code,
                "        message_arguments.set({variable_name:?}, {fluent_argument});"
            );
        }
        let _ = writeln!(
            generated_code,
            "        message_catalog.translated_text({message_id:?}, Some(&message_arguments))"
        );
    }
    let _ = writeln!(generated_code, "    }}");
}

pub fn emit_generated_messages(
    language_catalogs: &[LanguageCatalog],
    source_language_tag: &str,
) -> String {
    let mut generated_code = String::new();
    let _ = writeln!(
        generated_code,
        "pub(crate) const SOURCE_LANGUAGE_TAG: &str = {source_language_tag:?};"
    );
    let _ = writeln!(
        generated_code,
        "pub(crate) const BUNDLED_LANGUAGE_CATALOGS: &[BundledLanguageCatalog] = &["
    );
    for language_catalog in language_catalogs {
        let _ = writeln!(
            generated_code,
            "    BundledLanguageCatalog {{ language_tag: {:?}, fluent_sources: &[",
            language_catalog.language_tag
        );
        for source_path in &language_catalog.source_paths {
            let _ = writeln!(
                generated_code,
                "        include_str!({:?}),",
                source_path.display().to_string()
            );
        }
        let _ = writeln!(generated_code, "    ] }},");
    }
    let _ = writeln!(generated_code, "];");
    let Some(source_catalog) = language_catalogs
        .iter()
        .find(|language_catalog| language_catalog.language_tag == source_language_tag)
    else {
        return generated_code;
    };
    let mut messages_by_module: BTreeMap<String, Vec<(String, &CatalogMessage)>> = BTreeMap::new();
    for catalog_message in &source_catalog.messages {
        if let Some(function_path) = message_function_path(&catalog_message.message_id) {
            messages_by_module
                .entry(function_path.module_name)
                .or_default()
                .push((function_path.function_name, catalog_message));
        }
    }
    for (module_name, module_messages) in &messages_by_module {
        let _ = writeln!(generated_code, "pub(crate) mod {module_name} {{");
        emit_choice_types(&mut generated_code, module_messages);
        for (function_name, catalog_message) in module_messages {
            emit_message_function(&mut generated_code, function_name, catalog_message);
        }
        let _ = writeln!(generated_code, "}}");
    }
    generated_code
}
