use std::rc::Rc;

use crate::{
    host_services::desktop::{flatpak_sandbox::SandboxKind, locale_formatting::MomentFormatters},
    translated_messages::MessageCatalog,
};
mod application_window;
mod findings;
mod headless_modes;
mod host_services;
#[cfg(test)]
mod host_verification_tests;
mod preferences_dialog;
mod scans;
mod shared_widgets;
mod signatures;
mod translated_messages;

use std::process::ExitCode;

use relm4::{adw, gtk::gio::prelude::SettingsExt};

use crate::{
    application_window::{
        application_window_shell::{ApplicationWindowShell, ApplicationWindowShellInit},
        startup_arguments::{
            StartupPageRequest, known_startup_page_names, parse_startup_arguments,
        },
    },
    headless_modes::headless_scheduled_scan::{parse_scheduled_scan_options, run_scheduled_scan},
    host_services::desktop::{
        running_instance_forwarding::{another_instance_is_running, forward_to_running_instance},
        stored_settings::{application_settings, settings_schema_is_installed},
    },
};

pub(crate) const RELEASE_APPLICATION_ID: &str = "page.codeberg.wvhulle.Lazaret";
pub(crate) const SETTINGS_SCHEMA_ID: &str = RELEASE_APPLICATION_ID;
pub(crate) const APPLICATION_ICON_NAME: &str = RELEASE_APPLICATION_ID;
pub(crate) const TRAY_ICON_NAME: &str = "page.codeberg.wvhulle.Lazaret-symbolic";
pub(crate) const TRAY_ATTENTION_ICON_NAME: &str =
    "page.codeberg.wvhulle.Lazaret-attention-symbolic";
#[cfg(not(debug_assertions))]
pub(crate) const APPLICATION_ID: &str = RELEASE_APPLICATION_ID;
#[cfg(debug_assertions)]
pub(crate) const APPLICATION_ID: &str = "page.codeberg.wvhulle.Lazaret.Devel";
pub(crate) const APPLICATION_DATA_NAME: &str = "lazaret";
fn main() -> ExitCode {
    let message_catalog = Rc::new(MessageCatalog::for_host_messages_locale());
    let sandbox_kind = SandboxKind::detect();
    let moment_formatters = Rc::new(MomentFormatters::for_host_locale());
    if !settings_schema_is_installed() {
        eprintln!(
            "{}",
            translated_messages::startup::settings_schema_missing(&message_catalog)
        );
        return ExitCode::FAILURE;
    }
    if let Some(scan_options) = parse_scheduled_scan_options(std::env::args()) {
        return run_scheduled_scan(&message_catalog, sandbox_kind, scan_options);
    }
    let startup_request = parse_startup_arguments(std::env::args());
    if let StartupPageRequest::UnrecognizedPage {
        requested_page_name,
    } = &startup_request.startup_page
    {
        eprintln!(
            "{}",
            translated_messages::startup::unknown_page_name(
                &message_catalog,
                requested_page_name,
                known_startup_page_names().join(", ")
            )
        );
    }
    if startup_request.start_in_background && another_instance_is_running() {
        return ExitCode::SUCCESS;
    }
    if !startup_request.start_in_background
        && forward_to_running_instance(&startup_request.scan_targets)
    {
        return ExitCode::SUCCESS;
    }
    let application = adw::Application::builder()
        .application_id(APPLICATION_ID)
        .build();
    let start_hidden = startup_request.start_in_background
        || application_settings().boolean("start-in-background");
    relm4::RelmApp::from_app(application)
        .with_args(Vec::new())
        .visible_on_activate(!start_hidden)
        .run_async::<ApplicationWindowShell>(ApplicationWindowShellInit {
            message_catalog,
            startup_request,
            sandbox_kind,
            moment_formatters,
        });
    ExitCode::SUCCESS
}
