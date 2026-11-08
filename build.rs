use std::{env, fs, path::PathBuf};

use lazaret_translation_codegen::generate_message_functions;

const SOURCE_LANGUAGE_TAG: &str = "en";

fn main() {
    let manifest_directory =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR"));
    let output_directory = PathBuf::from(env::var_os("OUT_DIR").expect("cargo sets OUT_DIR"));
    let locales_directory = manifest_directory.join("locales");
    println!("cargo:rerun-if-changed={}", locales_directory.display());
    let generated_messages =
        match generate_message_functions(&locales_directory, SOURCE_LANGUAGE_TAG) {
            Ok(generated_messages) => generated_messages,
            Err(catalog_problems) => {
                for catalog_problem in &catalog_problems {
                    eprintln!("error: {catalog_problem}");
                }
                panic!(
                    "{} problem(s) in the translation catalogs under {}",
                    catalog_problems.len(),
                    locales_directory.display()
                );
            }
        };
    let incomplete_languages = generated_messages
        .untranslated_counts
        .iter()
        .filter(|language_count| language_count.untranslated_count > 0)
        .map(|language_count| {
            format!(
                "{} {}/{}",
                language_count.language_tag,
                language_count.untranslated_count,
                language_count.source_message_count
            )
        })
        .collect::<Vec<_>>();
    if !incomplete_languages.is_empty() {
        println!(
            "cargo:warning=untranslated messages fall back to English: {}",
            incomplete_languages.join(", ")
        );
    }
    fs::write(
        output_directory.join("generated_messages.rs"),
        generated_messages.rust_source,
    )
    .expect("write generated message functions");
}
