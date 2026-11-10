use std::{fs, path::Path};

use clamd_configuration::PROTECTED_SETTING_NAMES;

use crate::{
    host_services::clamav_admin_service::live_settings_client::OverrideSetting,
    shared_widgets::setting_row::setting_row_state::{
        ClamavSource, CopyOnlyReason, FileEntry, RootOwnedConfigFile, RuntimeRoute,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

const NIX_STORE_PREFIX: &str = "/nix/store/";
const LIVE_TUNING_RUNTIME_PREFIX: &str = "/run/lazaret/";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigFileOwner {
    NixOsConfiguration,
    SystemPackage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClamavConfigKind {
    ClamdDaemonConfig,
    FreshclamUpdaterConfig,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SettingLock {
    ChangedThroughSnippet,
    NeverChangedHere,
}

pub fn setting_lock_for_key(setting_key: &str) -> SettingLock {
    if PROTECTED_SETTING_NAMES
        .lines()
        .map(str::trim)
        .any(|protected_key| protected_key.eq_ignore_ascii_case(setting_key))
    {
        SettingLock::NeverChangedHere
    } else {
        SettingLock::ChangedThroughSnippet
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeTuningState<'tuning> {
    TuningAvailable {
        tunable_settings: &'tuning [String],
        active_overrides: &'tuning [OverrideSetting],
    },
    TuningMissing(CopyOnlyReason),
}

pub fn clamav_source(
    setting_key: &str,
    config_file: RootOwnedConfigFile,
    file_entry: FileEntry,
    tuning_state: RuntimeTuningState<'_>,
) -> ClamavSource {
    if setting_lock_for_key(setting_key) == SettingLock::NeverChangedHere {
        return ClamavSource::ProtectedSetting { config_file };
    }
    match tuning_state {
        RuntimeTuningState::TuningMissing(copy_only_reason) => ClamavSource::RootOwnedFile {
            config_file,
            file_entry,
            runtime_route: RuntimeRoute::CopyLineOnly(copy_only_reason),
        },
        RuntimeTuningState::TuningAvailable {
            tunable_settings,
            active_overrides,
        } => {
            let key_overrides = active_overrides
                .iter()
                .filter(|active_override| active_override.setting_key == setting_key)
                .collect::<Vec<_>>();
            match key_overrides.first() {
                Some(first_override) => ClamavSource::SetViaAdminService {
                    config_file,
                    override_lifetime: first_override.override_lifetime,
                    override_values: key_overrides
                        .iter()
                        .map(|key_override| key_override.setting_value.clone())
                        .collect(),
                },
                None => ClamavSource::RootOwnedFile {
                    config_file,
                    file_entry,
                    runtime_route: if tunable_settings
                        .iter()
                        .any(|tunable_key| tunable_key == setting_key)
                    {
                        RuntimeRoute::ChangeableWithPassword
                    } else {
                        RuntimeRoute::CopyLineOnly(CopyOnlyReason::OutsideAllowlist)
                    },
                },
            }
        }
    }
}

pub fn config_file_owner_of_target(resolved_target: &str) -> ConfigFileOwner {
    if resolved_target.starts_with(NIX_STORE_PREFIX)
        || resolved_target.starts_with(LIVE_TUNING_RUNTIME_PREFIX)
    {
        ConfigFileOwner::NixOsConfiguration
    } else {
        ConfigFileOwner::SystemPackage
    }
}

pub fn inspect_config_file_owner(config_path: &Path) -> ConfigFileOwner {
    fs::canonicalize(config_path).map_or(ConfigFileOwner::SystemPackage, |resolved_target| {
        config_file_owner_of_target(&resolved_target.to_string_lossy())
    })
}

fn nix_literal(setting_value: &str) -> String {
    let is_number = !setting_value.is_empty()
        && setting_value
            .chars()
            .all(|value_character| value_character.is_ascii_digit());
    match setting_value {
        "true" | "false" => setting_value.to_owned(),
        _ if is_number => setting_value.to_owned(),
        _ => format!(
            "\"{}\"",
            setting_value
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace("${", "\\${")
        ),
    }
}

fn nix_setting_prefix(config_kind: ClamavConfigKind) -> &'static str {
    match config_kind {
        ClamavConfigKind::ClamdDaemonConfig => "services.clamav.daemon.settings",
        ClamavConfigKind::FreshclamUpdaterConfig => "services.clamav.updater.settings",
    }
}

fn nix_attribute_name(setting_key: &str) -> String {
    let is_plain_identifier = setting_key
        .chars()
        .all(|key_character| key_character.is_ascii_alphanumeric() || key_character == '_');
    if is_plain_identifier {
        setting_key.to_owned()
    } else {
        nix_literal(setting_key)
    }
}

pub fn change_snippet(
    config_file_owner: ConfigFileOwner,
    config_kind: ClamavConfigKind,
    setting_key: &str,
    setting_values: &[String],
) -> String {
    match config_file_owner {
        ConfigFileOwner::NixOsConfiguration => {
            let right_hand_side = match setting_values {
                [single_value] => nix_literal(single_value),
                several_values => format!(
                    "[ {} ]",
                    several_values
                        .iter()
                        .map(|setting_value| nix_literal(setting_value))
                        .collect::<Vec<_>>()
                        .join(" ")
                ),
            };
            format!(
                "{}.{} = {right_hand_side};",
                nix_setting_prefix(config_kind),
                nix_attribute_name(setting_key)
            )
        }
        ConfigFileOwner::SystemPackage => setting_values
            .iter()
            .map(|setting_value| format!("{setting_key} {setting_value}"))
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

pub fn managed_config_description(
    message_catalog: &MessageCatalog,
    config_file_owner: ConfigFileOwner,
    config_path: &str,
) -> String {
    match config_file_owner {
        ConfigFileOwner::NixOsConfiguration => {
            translated_messages::host::nixos_configuration(message_catalog, config_path)
        }
        ConfigFileOwner::SystemPackage => {
            translated_messages::host::system_package(message_catalog, config_path)
        }
    }
}

pub fn values_with_edit(
    setting_values: &[String],
    value_position: usize,
    edited_text: String,
) -> Vec<String> {
    let mut edited_values = setting_values.to_vec();
    if let Some(edited_value) = edited_values.get_mut(value_position) {
        *edited_value = edited_text;
    }
    edited_values
}

#[cfg(test)]
mod setting_provenance_tests {
    use clamd_configuration::override_reports::OverrideLifetime;

    use super::*;

    fn single_value(setting_value: &str) -> Vec<String> {
        vec![setting_value.to_owned()]
    }

    #[test]
    fn settings_that_run_commands_or_change_identity_are_protected() {
        [
            "VirusEvent",
            "OnUpdateExecute",
            "User",
            "DatabaseDirectory",
            "virusevent",
        ]
        .into_iter()
        .for_each(|setting_key| {
            assert_eq!(
                setting_lock_for_key(setting_key),
                SettingLock::NeverChangedHere,
                "{setting_key}"
            );
        });
    }

    #[test]
    fn ordinary_tuning_settings_can_be_changed_through_a_snippet() {
        ["OnAccessMaxFileSize", "MaxFileSize", "OnAccessIncludePath"]
            .into_iter()
            .for_each(|setting_key| {
                assert_eq!(
                    setting_lock_for_key(setting_key),
                    SettingLock::ChangedThroughSnippet,
                    "{setting_key}"
                );
            });
    }

    #[test]
    fn files_resolving_into_the_store_belong_to_the_nixos_configuration() {
        assert_eq!(
            config_file_owner_of_target("/nix/store/6x2mf6-clamd.conf"),
            ConfigFileOwner::NixOsConfiguration
        );
        assert_eq!(
            config_file_owner_of_target("/etc/clamav/clamd.conf"),
            ConfigFileOwner::SystemPackage
        );
    }

    #[test]
    fn nix_snippets_use_the_right_option_and_literal_type() {
        let nix_snippet = |config_kind, setting_key, setting_value| {
            change_snippet(
                ConfigFileOwner::NixOsConfiguration,
                config_kind,
                setting_key,
                &single_value(setting_value),
            )
        };
        assert_eq!(
            nix_snippet(
                ClamavConfigKind::ClamdDaemonConfig,
                "OnAccessPrevention",
                "true"
            ),
            "services.clamav.daemon.settings.OnAccessPrevention = true;"
        );
        assert_eq!(
            nix_snippet(ClamavConfigKind::FreshclamUpdaterConfig, "Checks", "12"),
            "services.clamav.updater.settings.Checks = 12;"
        );
        assert_eq!(
            nix_snippet(
                ClamavConfigKind::ClamdDaemonConfig,
                "OnAccessMaxFileSize",
                "100M"
            ),
            "services.clamav.daemon.settings.OnAccessMaxFileSize = \"100M\";"
        );
    }

    #[test]
    fn repeated_settings_become_a_list() {
        assert_eq!(
            change_snippet(
                ConfigFileOwner::NixOsConfiguration,
                ClamavConfigKind::ClamdDaemonConfig,
                "OnAccessIncludePath",
                &["/home/user/Downloads".to_owned(), "/srv/inbox".to_owned()],
            ),
            "services.clamav.daemon.settings.OnAccessIncludePath = [ \"/home/user/Downloads\" \
             \"/srv/inbox\" ];"
        );
    }

    #[test]
    fn nix_strings_cannot_break_out_or_interpolate() {
        assert_eq!(
            change_snippet(
                ConfigFileOwner::NixOsConfiguration,
                ClamavConfigKind::ClamdDaemonConfig,
                "LogTag",
                &single_value("a\"b${c}\\"),
            ),
            "services.clamav.daemon.settings.LogTag = \"a\\\"b\\${c}\\\\\";"
        );
    }

    #[test]
    fn other_systems_get_config_file_lines() {
        assert_eq!(
            change_snippet(
                ConfigFileOwner::SystemPackage,
                ClamavConfigKind::ClamdDaemonConfig,
                "OnAccessIncludePath",
                &["/home/a".to_owned(), "/home/b".to_owned()],
            ),
            "OnAccessIncludePath /home/a\nOnAccessIncludePath /home/b"
        );
    }

    #[test]
    fn an_edit_replaces_only_the_value_at_its_position() {
        let setting_values = vec!["/home".to_owned(), "/srv".to_owned()];
        assert_eq!(
            values_with_edit(&setting_values, 1, "/data".to_owned()),
            vec!["/home".to_owned(), "/data".to_owned()]
        );
        assert_eq!(
            values_with_edit(&setting_values, 5, "/data".to_owned()),
            setting_values
        );
    }

    fn nixos_clamd_file() -> RootOwnedConfigFile {
        RootOwnedConfigFile {
            file_owner: ConfigFileOwner::NixOsConfiguration,
            config_path: "/etc/clamav/clamd.conf".to_owned(),
        }
    }

    fn source_with_tuning(setting_key: &str, tuning_state: RuntimeTuningState<'_>) -> ClamavSource {
        clamav_source(
            setting_key,
            nixos_clamd_file(),
            FileEntry::WrittenInFile,
            tuning_state,
        )
    }

    #[test]
    fn each_clamd_setting_gets_the_source_that_decides_its_value() {
        let tunable_settings = vec!["DetectPUA".to_owned(), "MaxThreads".to_owned()];
        let active_overrides = vec![OverrideSetting {
            setting_key: "MaxThreads".to_owned(),
            setting_value: "8".to_owned(),
            override_lifetime: OverrideLifetime::Permanent,
        }];
        let available_tuning = RuntimeTuningState::TuningAvailable {
            tunable_settings: &tunable_settings,
            active_overrides: &active_overrides,
        };
        assert_eq!(
            source_with_tuning("VirusEvent", available_tuning),
            ClamavSource::ProtectedSetting {
                config_file: nixos_clamd_file()
            }
        );
        assert_eq!(
            source_with_tuning("MaxThreads", available_tuning),
            ClamavSource::SetViaAdminService {
                config_file: nixos_clamd_file(),
                override_lifetime: OverrideLifetime::Permanent,
                override_values: vec!["8".to_owned()],
            }
        );
        assert!(matches!(
            source_with_tuning("DetectPUA", available_tuning),
            ClamavSource::RootOwnedFile {
                runtime_route: RuntimeRoute::ChangeableWithPassword,
                ..
            }
        ));
        assert!(matches!(
            source_with_tuning("MaxScanSize", available_tuning),
            ClamavSource::RootOwnedFile {
                runtime_route: RuntimeRoute::CopyLineOnly(CopyOnlyReason::OutsideAllowlist),
                ..
            }
        ));
        assert!(matches!(
            source_with_tuning(
                "DetectPUA",
                RuntimeTuningState::TuningMissing(CopyOnlyReason::AdminServiceNotInstalled)
            ),
            ClamavSource::RootOwnedFile {
                runtime_route: RuntimeRoute::CopyLineOnly(CopyOnlyReason::AdminServiceNotInstalled),
                ..
            }
        ));
    }
}
