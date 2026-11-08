use std::{env, fs, path::PathBuf};

use lazaret_translation_codegen::generate_message_functions;

const SOURCE_LANGUAGE_TAG: &str = "en";

fn main() {
    let manifest_directory =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR"));
    let output_directory = PathBuf::from(env::var_os("OUT_DIR").expect("cargo sets OUT_DIR"));
    let locales_directory = manifest_directory.join("../locales");
    println!("cargo:rerun-if-changed={}", locales_directory.display());
    let generated_messages = generate_message_functions(&locales_directory, SOURCE_LANGUAGE_TAG)
        .unwrap_or_else(|catalog_problems| {
            panic!(
                "{} problem(s) in the translation catalogs under {}",
                catalog_problems.len(),
                locales_directory.display()
            )
        });
    fs::write(
        output_directory.join("generated_messages.rs"),
        generated_messages.rust_source,
    )
    .expect("write generated message functions");
}
