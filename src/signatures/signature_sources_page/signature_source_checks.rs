use std::{io, path::Path};

use tokio::task::spawn_blocking;

use crate::{
    host_services::{
        clamav_programs::{
            database_directory_status::{DatabaseFolderReadFailure, configured_database_directory},
            distribution_install_guides::DistributionFamily,
            signature_provider_catalogue::{
                ProviderCredential, SIGNATURE_PROVIDERS, SettingsOnlyCredential,
                SignatureFileSource, SignatureProvider, provider_of_signature_file,
            },
        },
        desktop::flatpak_sandbox::SandboxKind,
    },
    shared_widgets::host_read_failure_text::database_folder_failure_text,
    signatures::signature_sources_page::source_check_results::{AuditCheck, AuditStatus},
    translated_messages,
    translated_messages::MessageCatalog,
};

const FANGFRISCH_CONFIG_PATH: &str = "/etc/fangfrisch/fangfrisch.conf";
const FANGFRISCH_CONFIG_PROTECTION_COMMAND: &str = "sudo chmod 600 /etc/fangfrisch/fangfrisch.conf";

fn settings_credential_check(
    message_catalog: &MessageCatalog,
    inactive_check: AuditCheck,
    provider_name: &str,
    settings_credential: SettingsOnlyCredential,
    distribution_family: DistributionFamily,
) -> AuditCheck {
    let credential_choice = settings_credential.credential_label.translated_choice();
    if distribution_family == DistributionFamily::NixosFamily {
        return inactive_check
            .with_recommendation(
                translated_messages::signatures::provider_needs_credential_fangfrisch(
                    message_catalog,
                    credential_choice,
                    provider_name,
                ),
            )
            .with_documentation(settings_credential.signup_url);
    }
    inactive_check
        .with_recommendation(
            translated_messages::signatures::provider_needs_credential_website(
                message_catalog,
                credential_choice,
                provider_name,
                FANGFRISCH_CONFIG_PATH,
            ),
        )
        .with_suggested_commands([
            settings_credential.fangfrisch_credential_lines,
            FANGFRISCH_CONFIG_PROTECTION_COMMAND,
        ])
        .with_documentation(settings_credential.signup_url)
}

fn inactive_provider_check(
    message_catalog: &MessageCatalog,
    signature_provider: &SignatureProvider,
    distribution_family: DistributionFamily,
) -> AuditCheck {
    let provider_name = signature_provider.third_party_provider.provider_name();
    let inactive_check = AuditCheck::new(
        provider_name.to_owned(),
        AuditStatus::CheckSkipped,
        translated_messages::signatures::not_use(message_catalog),
    )
    .starting_collapsed();
    match signature_provider.credential {
        ProviderCredential::CredentialInSettingsOnly(settings_credential) => {
            settings_credential_check(
                message_catalog,
                inactive_check,
                provider_name,
                settings_credential,
                distribution_family,
            )
        }
        ProviderCredential::KeyFileSupported(enabling_snippets)
        | ProviderCredential::NoCredential(enabling_snippets)
            if distribution_family == DistributionFamily::NixosFamily =>
        {
            inactive_check.with_suggested_commands([enabling_snippets.nixos_snippet])
        }
        ProviderCredential::NoCredential(enabling_snippets) => inactive_check
            .with_recommendation(
                translated_messages::signatures::add_lines_fangfrisch_settings(
                    message_catalog,
                    FANGFRISCH_CONFIG_PATH,
                ),
            )
            .with_suggested_commands([enabling_snippets.fangfrisch_lines]),
        ProviderCredential::KeyFileSupported(enabling_snippets) => inactive_check
            .with_recommendation(translated_messages::signatures::provider_needs_free_abuse(
                message_catalog,
                provider_name,
                FANGFRISCH_CONFIG_PATH,
            ))
            .with_suggested_commands([
                enabling_snippets.fangfrisch_lines,
                FANGFRISCH_CONFIG_PROTECTION_COMMAND,
            ])
            .with_documentation("https://auth.abuse.ch/"),
    }
}

pub fn signature_source_checks(
    message_catalog: &MessageCatalog,
    signature_file_names: &[String],
    distribution_family: DistributionFamily,
) -> Vec<AuditCheck> {
    SIGNATURE_PROVIDERS
        .iter()
        .map(|signature_provider| {
            let provider_in_use = signature_file_names.iter().any(|file_name| {
                provider_of_signature_file(file_name)
                    == SignatureFileSource::ThirdPartyFile(signature_provider.third_party_provider)
            });
            if provider_in_use {
                AuditCheck::new(
                    signature_provider
                        .third_party_provider
                        .provider_name()
                        .to_owned(),
                    AuditStatus::CheckPassed,
                    translated_messages::signatures::use_label(message_catalog),
                )
            } else {
                inactive_provider_check(message_catalog, signature_provider, distribution_family)
            }
        })
        .collect()
}

pub fn unreadable_source_checks(
    message_catalog: &MessageCatalog,
    read_failure: DatabaseFolderReadFailure,
) -> Vec<AuditCheck> {
    SIGNATURE_PROVIDERS
        .iter()
        .map(|signature_provider| {
            AuditCheck::new(
                signature_provider
                    .third_party_provider
                    .provider_name()
                    .to_owned(),
                AuditStatus::StatusUnknown,
                translated_messages::signatures::unknown_reason_source(
                    message_catalog,
                    database_folder_failure_text(message_catalog, read_failure),
                ),
            )
        })
        .collect()
}

fn signature_file_names(
    database_directory: &Path,
) -> Result<Vec<String>, DatabaseFolderReadFailure> {
    match std::fs::read_dir(database_directory) {
        Ok(directory_entries) => Ok(directory_entries
            .filter_map(Result::ok)
            .map(|directory_entry| directory_entry.file_name().to_string_lossy().into_owned())
            .collect()),
        Err(read_failure) if read_failure.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(read_failure) => Err(read_failure.into()),
    }
}

pub async fn read_signature_file_names(
    sandbox_kind: SandboxKind,
) -> Result<Vec<String>, DatabaseFolderReadFailure> {
    spawn_blocking(move || signature_file_names(&configured_database_directory(sandbox_kind)))
        .await
        .map_err(DatabaseFolderReadFailure::from)
        .flatten()
}

pub fn audit_checks_of(
    message_catalog: &MessageCatalog,
    file_names_read: Result<Vec<String>, DatabaseFolderReadFailure>,
    distribution_family: DistributionFamily,
) -> Vec<AuditCheck> {
    match file_names_read {
        Ok(signature_files) => {
            signature_source_checks(message_catalog, &signature_files, distribution_family)
        }
        Err(read_failure) => unreadable_source_checks(message_catalog, read_failure),
    }
}

#[cfg(test)]
mod signature_source_checks_tests {
    use super::*;
    use crate::{
        host_services::clamav_programs::signature_provider_catalogue::{
            EnablingSnippets, FANGFRISCH_SANESECURITY_LINES, FANGFRISCH_SECURITEINFO_LINES,
            NIXOS_SANESECURITY_SNIPPET, ThirdPartyProvider,
        },
        signatures::signature_sources_page::source_check_results::CheckGuidance,
    };

    fn commands_of(audit_check: &AuditCheck) -> Vec<&'static str> {
        audit_check
            .check_guidance
            .iter()
            .filter_map(|check_guidance| match check_guidance {
                CheckGuidance::SuggestedCommand(suggested_command) => Some(*suggested_command),
                CheckGuidance::RecommendationText(_) | CheckGuidance::DocumentationLink(_) => None,
            })
            .collect()
    }

    #[test]
    fn an_unreadable_signature_folder_leaves_every_source_unknown() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let source_checks = unreadable_source_checks(
            message_catalog,
            DatabaseFolderReadFailure::ReadingTaskInterrupted,
        );
        assert_eq!(source_checks.len(), SIGNATURE_PROVIDERS.len());
        source_checks.iter().for_each(|source_check| {
            assert_eq!(source_check.check_status, AuditStatus::StatusUnknown);
            assert!(source_check.status_detail.starts_with("Unknown."));
        });
    }

    #[test]
    fn sources_in_use_are_recognized_and_offer_no_snippet() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let source_checks = signature_source_checks(
            message_catalog,
            &["urlhaus_clamav.ndb".to_owned()],
            DistributionFamily::NixosFamily,
        );
        assert_eq!(source_checks[0].check_status, AuditStatus::CheckPassed);
        assert!(commands_of(&source_checks[0]).is_empty());
        assert_eq!(source_checks[1].check_status, AuditStatus::CheckSkipped);
    }

    #[test]
    fn no_snippet_ever_contains_a_credential() {
        let message_catalog = &crate::translated_messages::english_catalog();
        signature_source_checks(message_catalog, &[], DistributionFamily::NixosFamily)
            .iter()
            .flat_map(commands_of)
            .for_each(|enabling_snippet| {
                ["auth_key", "receipt", "customer_id"]
                    .into_iter()
                    .for_each(|credential_setting| {
                        assert!(
                            !enabling_snippet.contains(credential_setting),
                            "{enabling_snippet}"
                        );
                    });
            });
    }

    #[test]
    fn sources_needing_a_store_visible_credential_warn_instead_of_suggesting() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let source_checks =
            signature_source_checks(message_catalog, &[], DistributionFamily::NixosFamily);
        let malwarepatrol_check = source_checks
            .iter()
            .find(|source_check| source_check.check_name == "Malwarepatrol")
            .expect("catalogue entry");
        assert!(commands_of(malwarepatrol_check).is_empty());
        assert!(malwarepatrol_check.check_guidance.iter().any(|check_guidance| {
            matches!(check_guidance, CheckGuidance::RecommendationText(warning_text) if warning_text.contains("Nix store") && warning_text.contains("purchase receipt number"))
        }));
    }

    #[test]
    fn other_systems_get_placeholder_lines_and_a_permission_command() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let source_checks =
            signature_source_checks(message_catalog, &[], DistributionFamily::DebianFamily);
        let securiteinfo_check = source_checks
            .iter()
            .find(|source_check| source_check.check_name == "SecuriteInfo")
            .expect("catalogue entry");
        assert_eq!(
            commands_of(securiteinfo_check),
            vec![
                FANGFRISCH_SECURITEINFO_LINES,
                FANGFRISCH_CONFIG_PROTECTION_COMMAND
            ]
        );
        assert!(securiteinfo_check.check_guidance.iter().all(|check_guidance| {
            !matches!(check_guidance, CheckGuidance::RecommendationText(advice_text) if advice_text.contains("NixOS"))
        }));
    }

    #[test]
    fn providers_without_a_settings_credential_carry_their_snippets() {
        let message_catalog = &crate::translated_messages::english_catalog();
        let sanesecurity_provider = SIGNATURE_PROVIDERS
            .iter()
            .find(|signature_provider| {
                signature_provider.third_party_provider == ThirdPartyProvider::Sanesecurity
            })
            .expect("catalogue entry");
        assert_eq!(
            sanesecurity_provider.credential,
            ProviderCredential::NoCredential(EnablingSnippets {
                nixos_snippet: NIXOS_SANESECURITY_SNIPPET,
                fangfrisch_lines: FANGFRISCH_SANESECURITY_LINES,
            })
        );
        let nixos_checks =
            signature_source_checks(message_catalog, &[], DistributionFamily::NixosFamily);
        let sanesecurity_check = nixos_checks
            .iter()
            .find(|source_check| source_check.check_name == "Sanesecurity")
            .expect("catalogue entry");
        assert_eq!(
            commands_of(sanesecurity_check),
            vec![NIXOS_SANESECURITY_SNIPPET]
        );
    }

    #[test]
    fn every_suggested_snippet_is_filled_in() {
        let message_catalog = &crate::translated_messages::english_catalog();
        [
            DistributionFamily::NixosFamily,
            DistributionFamily::DebianFamily,
        ]
        .into_iter()
        .flat_map(|distribution_family| {
            signature_source_checks(message_catalog, &[], distribution_family)
        })
        .for_each(|source_check| {
            commands_of(&source_check)
                .into_iter()
                .for_each(|suggested_command| assert!(!suggested_command.is_empty()));
        });
    }
}
