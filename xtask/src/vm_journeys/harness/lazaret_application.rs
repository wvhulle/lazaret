use std::{collections::HashMap, time::Duration};

use zbus::zvariant;

use crate::vm_journeys::harness::{
    sway_windows::sway_windows,
    system_commands::{command_succeeds, settle, succeed, wait_until},
};

const APPLICATION_BUS_NAME: &str = "page.codeberg.wvhulle.Lazaret";
const APPLICATION_OBJECT_PATH: &str = "/page/codeberg/wvhulle/Lazaret";
const WINDOW_OBJECT_PATH: &str = "/page/codeberg/wvhulle/Lazaret/window/1";
const GTK_ACTIONS_INTERFACE: &str = "org.gtk.Actions";
const TRAY_ITEM_PATH: &str = "/StatusNotifierItem";
const TRAY_ITEM_INTERFACE: &str = "org.kde.StatusNotifierItem";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrayStatus {
    Active,
    NeedsAttention,
}

impl TrayStatus {
    const fn bus_value(self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::NeedsAttention => "NeedsAttention",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApplicationPage {
    Detections,
    Quarantine,
    TrustedDetections,
    ScheduledScans,
    ScanEngine,
    SignatureSources,
}

impl ApplicationPage {
    const fn page_name(self) -> &'static str {
        match self {
            Self::Detections => "detections",
            Self::Quarantine => "quarantine",
            Self::TrustedDetections => "trusted-detections",
            Self::ScheduledScans => "scheduled-scans",
            Self::ScanEngine => "scan-engine",
            Self::SignatureSources => "signature-sources",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApplicationAction {
    ShowWindow,
    OpenDetections,
    UpdateDefinitions,
}

impl ApplicationAction {
    const fn action_name(self) -> &'static str {
        match self {
            Self::ShowWindow => "show-window",
            Self::OpenDetections => "open-detections",
            Self::UpdateDefinitions => "update-definitions",
        }
    }
}

pub fn lazaret_window_is_visible() -> bool {
    sway_windows().iter().any(|sway_window| {
        sway_window.application_identifier == APPLICATION_BUS_NAME && sway_window.is_visible
    })
}

pub fn wait_for_lazaret_window() {
    wait_until(
        "a visible Lazaret window",
        Duration::from_secs(30),
        lazaret_window_is_visible,
    );
}

fn session_connection() -> zbus::blocking::Connection {
    zbus::blocking::Connection::session().unwrap_or_else(|connection_failure| {
        panic!("the session bus could not be reached: {connection_failure}")
    })
}

fn activate_gtk_action(
    object_path: &str,
    action_name: &str,
    action_parameters: Vec<zvariant::Value<'_>>,
) {
    session_connection()
        .call_method(
            Some(APPLICATION_BUS_NAME),
            object_path,
            Some(GTK_ACTIONS_INTERFACE),
            "Activate",
            &(
                action_name,
                action_parameters,
                HashMap::<&str, zvariant::Value<'_>>::new(),
            ),
        )
        .unwrap_or_else(|call_failure| {
            panic!("activating the action {action_name} on {object_path} failed: {call_failure}")
        });
}

pub fn activate_application_action(application_action: ApplicationAction) {
    activate_gtk_action(
        APPLICATION_OBJECT_PATH,
        application_action.action_name(),
        Vec::new(),
    );
}

pub fn show_page(application_page: ApplicationPage) {
    activate_application_action(ApplicationAction::ShowWindow);
    wait_for_lazaret_window();
    activate_gtk_action(
        WINDOW_OBJECT_PATH,
        "show-page",
        vec![zvariant::Value::from(application_page.page_name())],
    );
}

fn tray_bus_name() -> Option<String> {
    let listed_names: Vec<String> = session_connection()
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "ListNames",
            &(),
        )
        .and_then(|listing_reply| listing_reply.body().deserialize())
        .unwrap_or_else(|listing_failure| {
            panic!("the session bus names could not be listed: {listing_failure}")
        });
    listed_names
        .into_iter()
        .find(|listed_name| listed_name.contains("StatusNotifierItem"))
}

fn tray_status_value() -> Option<String> {
    let tray_name = tray_bus_name()?;
    let status_reply = session_connection()
        .call_method(
            Some(tray_name.as_str()),
            TRAY_ITEM_PATH,
            Some("org.freedesktop.DBus.Properties"),
            "Get",
            &(TRAY_ITEM_INTERFACE, "Status"),
        )
        .ok()?;
    let status_value: zvariant::OwnedValue = status_reply.body().deserialize().ok()?;
    String::try_from(status_value).ok()
}

pub fn wait_for_tray_status(expected_status: TrayStatus) {
    wait_until(
        &format!("the tray status {}", expected_status.bus_value()),
        Duration::from_secs(30),
        || tray_status_value().as_deref() == Some(expected_status.bus_value()),
    );
}

pub fn wait_for_lazaret_service() {
    wait_until("the Lazaret user service", Duration::from_secs(60), || {
        command_succeeds("systemctl", &["--user", "is-active", "lazaret.service"])
    });
    wait_until(
        "Lazaret on the session bus",
        Duration::from_secs(60),
        || command_succeeds("busctl", &["--user", "status", APPLICATION_BUS_NAME]),
    );
    wait_until("the Lazaret tray item", Duration::from_secs(60), || {
        tray_bus_name().is_some()
    });
    settle(Duration::from_secs(2));
}

pub fn scan_with_lazaret(scanned_paths: &[&str]) {
    succeed("lazaret", scanned_paths);
    wait_for_lazaret_window();
}
