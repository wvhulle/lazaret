use std::{collections::BTreeMap, fmt::Write};

use crate::{
    catalog_consistency::{message_function_path, rust_name_for},
    catalog_reading::{CatalogMessage, LanguageCatalog},
};

fn is_selector(catalog_message: &CatalogMessage, variable_name: &str) -> bool {
    catalog_message
        .references
        .selector_variables
        .contains(variable_name)
}

fn emit_parameter_list(catalog_message: &CatalogMessage) -> String {
    catalog_message
        .references
        .variables_in_order
        .iter()
        .filter_map(|variable_name| {
            let parameter_name = rust_name_for(variable_name)?;
            let parameter_bound = if is_selector(catalog_message, variable_name) {
                "impl ::core::convert::Into<::fluent_bundle::types::FluentNumber>"
            } else {
                "impl ::core::fmt::Display"
            };
            Some(format!("{parameter_name}: {parameter_bound}"))
        })
        .collect::<Vec<_>>()
        .join(", ")
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
            "        super::translated_message_text({message_id:?}, None)"
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
            let argument_value = if is_selector(catalog_message, variable_name) {
                format!("::fluent_bundle::FluentValue::Number({parameter_name}.into())")
            } else {
                format!("::fluent_bundle::FluentValue::from({parameter_name}.to_string())")
            };
            let _ = writeln!(
                generated_code,
                "        message_arguments.set({variable_name:?}, {argument_value});"
            );
        }
        let _ = writeln!(
            generated_code,
            "        super::translated_message_text({message_id:?}, Some(&message_arguments))"
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
        for (function_name, catalog_message) in module_messages {
            emit_message_function(&mut generated_code, function_name, catalog_message);
        }
        let _ = writeln!(generated_code, "}}");
    }
    generated_code
}
