use std::{fs, path::Path};

use tempfile::TempDir;

use crate::{CatalogProblem, generate_message_functions};

fn locales_with(catalog_files: &[(&str, &str)]) -> TempDir {
    let locales_directory = tempfile::tempdir().expect("temporary locales directory");
    for (relative_path, fluent_source) in catalog_files {
        let catalog_path = locales_directory.path().join(relative_path);
        fs::create_dir_all(catalog_path.parent().expect("catalog directory"))
            .expect("create language directory");
        fs::write(catalog_path, fluent_source).expect("write catalog");
    }
    locales_directory
}

fn problems_for(locales_directory: &Path) -> Vec<CatalogProblem> {
    generate_message_functions(locales_directory, "en").expect_err("catalog problems")
}

#[test]
fn id_prefix_becomes_module_and_variables_become_typed_parameters() {
    let locales_directory = locales_with(&[(
        "en/lazaret.ftl",
        "scans-files-left = { $count ->\n    [one] {$count} file left in { $folder }\n   *[other] \
         {$count} files left in { $folder }\n}\nscans-finished-title = Scan \
         finished\nfindings-page-title = Findings\n",
    )]);
    let generated_messages =
        generate_message_functions(locales_directory.path(), "en").expect("valid catalogs");
    let rust_source = &generated_messages.rust_source;
    assert!(rust_source.contains("pub(crate) mod scans {"));
    assert!(rust_source.contains("pub(crate) mod findings {"));
    assert!(rust_source.contains(
        "pub(crate) fn files_left(count: impl \
         ::core::convert::Into<::fluent_bundle::types::FluentNumber>, folder: impl \
         ::core::fmt::Display) -> ::std::string::String"
    ));
    assert!(rust_source.contains("pub(crate) fn finished_title() -> ::std::string::String"));
}

#[test]
fn catalogs_split_over_files_share_one_namespace() {
    let locales_directory = locales_with(&[
        (
            "en/first_part.ftl",
            "-brand-name = Lazaret\nscans-finished-title = Scan finished\n",
        ),
        (
            "en/second_part.ftl",
            "scans-finished-title = Duplicate\nscans-about-brand = About { -brand-name }\n",
        ),
    ]);
    assert!(matches!(
        problems_for(locales_directory.path()).as_slice(),
        [CatalogProblem::DuplicateMessage { message_id, .. }] if message_id == "scans-finished-title"
    ));
}

#[test]
fn missing_translations_are_counted_not_rejected() {
    let locales_directory = locales_with(&[
        (
            "en/lazaret.ftl",
            "scans-finished-title = Scan finished\nscans-failed-title = Scan failed\n",
        ),
        (
            "de/lazaret.ftl",
            "scans-finished-title = Scan abgeschlossen\n",
        ),
    ]);
    let generated_messages =
        generate_message_functions(locales_directory.path(), "en").expect("valid catalogs");
    assert_eq!(generated_messages.untranslated_counts.len(), 1);
    assert_eq!(
        generated_messages.untranslated_counts[0].untranslated_count,
        1
    );
}

#[test]
fn stale_translation_is_rejected() {
    let locales_directory = locales_with(&[
        ("en/lazaret.ftl", "scans-finished-title = Scan finished\n"),
        ("de/lazaret.ftl", "scans-removed-title = Entfernt\n"),
    ]);
    assert!(matches!(
        problems_for(locales_directory.path()).as_slice(),
        [CatalogProblem::StaleTranslation { message_id, .. }] if message_id == "scans-removed-title"
    ));
}

#[test]
fn unknown_translation_variable_is_rejected() {
    let locales_directory = locales_with(&[
        (
            "en/lazaret.ftl",
            "scans-target-label = Scanning { $folder }\n",
        ),
        (
            "fr/lazaret.ftl",
            "scans-target-label = Analyse de { $dossier }\n",
        ),
    ]);
    assert!(matches!(
        problems_for(locales_directory.path()).as_slice(),
        [CatalogProblem::UnknownVariable { variable_name, .. }] if variable_name == "dossier"
    ));
}

#[test]
fn selecting_on_text_variable_is_rejected() {
    let locales_directory = locales_with(&[
        (
            "en/lazaret.ftl",
            "scans-found-threats = Found { $count } threats\n",
        ),
        (
            "de/lazaret.ftl",
            "scans-found-threats = { $count ->\n    [one] Eine Bedrohung\n   *[other] { $count } \
             Bedrohungen\n}\n",
        ),
    ]);
    assert!(matches!(
        problems_for(locales_directory.path()).as_slice(),
        [CatalogProblem::SelectorOnTextVariable { variable_name, .. }] if variable_name == "count"
    ));
}

#[test]
fn syntax_error_reports_its_line() {
    let locales_directory = locales_with(&[(
        "en/lazaret.ftl",
        "scans-finished-title = Scan finished\nbroken entry without equals\n",
    )]);
    assert!(matches!(
        problems_for(locales_directory.path()).as_slice(),
        [CatalogProblem::SyntaxError { line_number: 2, .. }]
    ));
}

#[test]
fn message_id_without_module_prefix_is_rejected() {
    let locales_directory = locales_with(&[("en/lazaret.ftl", "finished = Finished\n")]);
    assert!(matches!(
        problems_for(locales_directory.path()).as_slice(),
        [CatalogProblem::MessageIdWithoutModule { message_id, .. }] if message_id == "finished"
    ));
}

#[test]
fn keyword_function_name_is_rejected() {
    let locales_directory = locales_with(&[("en/lazaret.ftl", "scans-match = Match\n")]);
    assert!(matches!(
        problems_for(locales_directory.path()).as_slice(),
        [CatalogProblem::UnusableRustName { fluent_name, .. }] if fluent_name == "scans-match"
    ));
}

#[test]
fn non_canonical_language_directory_is_rejected() {
    let locales_directory = locales_with(&[
        ("en/lazaret.ftl", "scans-finished-title = Scan finished\n"),
        ("pt_BR/lazaret.ftl", "scans-finished-title = Concluído\n"),
    ]);
    assert!(matches!(
        problems_for(locales_directory.path()).as_slice(),
        [CatalogProblem::InvalidLanguageDirectory { .. }]
    ));
}

#[test]
fn translation_must_define_the_terms_it_uses() {
    let locales_directory = locales_with(&[
        (
            "en/lazaret.ftl",
            "-brand-name = Lazaret\nwindow-about-title = About { -brand-name }\n",
        ),
        (
            "de/lazaret.ftl",
            "window-about-title = Über { -brand-name }\n",
        ),
    ]);
    assert!(matches!(
        problems_for(locales_directory.path()).as_slice(),
        [CatalogProblem::UnresolvedTermReference { referenced_id, .. }] if referenced_id == "brand-name"
    ));
}
