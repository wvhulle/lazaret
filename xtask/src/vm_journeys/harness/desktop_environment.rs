use std::{collections::HashMap, fs, time::Duration};

use serde_json::Value;
use zbus::zvariant;

use crate::vm_journeys::harness::{
    journey_failure::JourneyFailure,
    system_commands::{settle, succeed, wait_until},
};

const APPLICATION_BUS_NAME: &str = "page.codeberg.wvhulle.Lazaret";
const APPLICATION_OBJECT_PATH: &str = "/page/codeberg/wvhulle/Lazaret";
const WINDOW_OBJECT_PATH: &str = "/page/codeberg/wvhulle/Lazaret/window/1";
const GTK_ACTIONS_INTERFACE: &str = "org.gtk.Actions";
const TRAY_ITEM_PATH: &str = "/StatusNotifierItem";
const TRAY_ITEM_INTERFACE: &str = "org.kde.StatusNotifierItem";
const PASSWORD_PROMPT_TITLE: &str = "Authenticat";
pub const ARTIFACTS_DIRECTORY: &str = "/tmp/vm-journey-artifacts";
const WINDOW_SETTLE: Duration = Duration::from_secs(1);
const PROMPT_TIMEOUT: Duration = Duration::from_secs(60);

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NotificationUrgency {
    Critical,
    OtherUrgency,
}

pub struct DesktopNotification {
    pub shown_text: String,
    pub urgency: NotificationUrgency,
}

fn json_text(json_value: &Value) -> String {
    match json_value {
        Value::String(plain_text) => plain_text.clone(),
        Value::Object(typed_value) => typed_value.get("data").map(json_text).unwrap_or_default(),
        Value::Number(number_value) => number_value.to_string(),
        _other_value => String::new(),
    }
}

fn notification_from(notification_json: &Value) -> DesktopNotification {
    let urgency_text = notification_json
        .get("urgency")
        .map(json_text)
        .unwrap_or_default();
    DesktopNotification {
        shown_text: format!(
            "{} {}",
            notification_json
                .get("summary")
                .map(json_text)
                .unwrap_or_default(),
            notification_json
                .get("body")
                .map(json_text)
                .unwrap_or_default()
        ),
        urgency: if urgency_text == "2" || urgency_text == "critical" {
            NotificationUrgency::Critical
        } else {
            NotificationUrgency::OtherUrgency
        },
    }
}

pub fn listed_notifications() -> Result<Vec<DesktopNotification>, JourneyFailure> {
    let listing: Value = serde_json::from_str(&succeed("makoctl", &["list", "-j"])?)?;
    let notification_values = match &listing {
        Value::Object(grouped_listing) => grouped_listing
            .get("data")
            .and_then(Value::as_array)
            .map(|notification_groups| {
                notification_groups
                    .iter()
                    .filter_map(Value::as_array)
                    .flatten()
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
        Value::Array(plain_listing) => plain_listing.clone(),
        _unexpected_listing => Vec::new(),
    };
    Ok(notification_values.iter().map(notification_from).collect())
}

pub fn notifications_mentioning(
    mentioned_text: &str,
) -> Result<Vec<DesktopNotification>, JourneyFailure> {
    Ok(listed_notifications()?
        .into_iter()
        .filter(|desktop_notification| desktop_notification.shown_text.contains(mentioned_text))
        .collect())
}

pub fn wait_for_notification(
    mentioned_text: &str,
) -> Result<Vec<DesktopNotification>, JourneyFailure> {
    wait_until(
        &format!("a notification mentioning {mentioned_text:?}"),
        Duration::from_secs(60),
        || Ok(!notifications_mentioning(mentioned_text)?.is_empty()),
    )?;
    notifications_mentioning(mentioned_text)
}

pub fn wait_for_notification_withdrawn(mentioned_text: &str) -> Result<(), JourneyFailure> {
    wait_until(
        &format!("the withdrawal of the notification mentioning {mentioned_text:?}"),
        Duration::from_secs(30),
        || Ok(notifications_mentioning(mentioned_text)?.is_empty()),
    )
}

pub fn invoke_latest_notification() -> Result<(), JourneyFailure> {
    succeed("makoctl", &["invoke"]).map(|_invoked| ())
}

pub fn dismiss_notifications() -> Result<(), JourneyFailure> {
    succeed("makoctl", &["dismiss", "--all"]).map(|_dismissed| ())
}

struct SwayWindow {
    window_title: String,
    is_focused: bool,
    is_visible: bool,
    application_identifier: String,
}

fn sway_windows() -> Result<Vec<SwayWindow>, JourneyFailure> {
    let sway_tree: Value = serde_json::from_str(&succeed("swaymsg", &["-t", "get_tree"])?)?;
    let mut pending_nodes = vec![sway_tree];
    let mut found_windows = Vec::new();
    while let Some(sway_node) = pending_nodes.pop() {
        if sway_node.get("pid").and_then(Value::as_u64).is_some() {
            found_windows.push(SwayWindow {
                window_title: sway_node.get("name").map(json_text).unwrap_or_default(),
                is_focused: sway_node
                    .get("focused")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
                is_visible: sway_node
                    .get("visible")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
                application_identifier: sway_node.get("app_id").map(json_text).unwrap_or_default(),
            });
        }
        for child_key in ["nodes", "floating_nodes"] {
            if let Some(child_nodes) = sway_node.get(child_key).and_then(Value::as_array) {
                pending_nodes.extend(child_nodes.iter().cloned());
            }
        }
    }
    Ok(found_windows)
}

pub fn lazaret_window_is_visible() -> Result<bool, JourneyFailure> {
    Ok(sway_windows()?.iter().any(|sway_window| {
        sway_window.application_identifier == APPLICATION_BUS_NAME && sway_window.is_visible
    }))
}

pub fn wait_for_lazaret_window() -> Result<(), JourneyFailure> {
    wait_until(
        "a visible Lazaret window",
        Duration::from_secs(30),
        lazaret_window_is_visible,
    )
}

fn window_titled_is_open(title_fragment: &str) -> Result<bool, JourneyFailure> {
    Ok(sway_windows()?
        .iter()
        .any(|sway_window| sway_window.window_title.contains(title_fragment)))
}

fn focused_window_is_titled(title_fragment: &str) -> Result<bool, JourneyFailure> {
    Ok(sway_windows()?.iter().any(|sway_window| {
        sway_window.is_focused && sway_window.window_title.contains(title_fragment)
    }))
}

pub fn wait_for_focused_window(title_fragment: &str) -> Result<(), JourneyFailure> {
    wait_until(
        &format!("a focused window titled {title_fragment:?}"),
        PROMPT_TIMEOUT,
        || focused_window_is_titled(title_fragment),
    )?;
    settle(WINDOW_SETTLE);
    Ok(())
}

pub fn wait_for_window_closed(title_fragment: &str) -> Result<(), JourneyFailure> {
    wait_until(
        &format!("the window titled {title_fragment:?} closing"),
        PROMPT_TIMEOUT,
        || Ok(!window_titled_is_open(title_fragment)?),
    )
}

const LEFT_CONTROL_KEYCODE: u16 = 29;
const ENTER_KEYCODE: u16 = 28;
const ESCAPE_KEYCODE: u16 = 1;
const DOWN_KEYCODE: u16 = 108;
const TYPING_DELAY_MILLISECONDS: &str = "20";

#[derive(Clone, Copy, Debug)]
pub enum ChordKey {
    SelectAll,
}

impl ChordKey {
    const fn keycode(self) -> u16 {
        match self {
            Self::SelectAll => 30,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum KeyInput<'typed> {
    TypedText(&'typed str),
    ControlChord(ChordKey),
    Enter,
    Escape,
    Down,
}

fn pressed_and_released(keycodes: &[u16]) -> Vec<String> {
    keycodes
        .iter()
        .map(|keycode| format!("{keycode}:1"))
        .chain(keycodes.iter().rev().map(|keycode| format!("{keycode}:0")))
        .collect()
}

fn send_key_input(key_input: KeyInput<'_>) -> Result<(), JourneyFailure> {
    let ydotool_arguments = match key_input {
        KeyInput::TypedText(typed_text) => vec![
            "type".to_owned(),
            "--key-delay".to_owned(),
            TYPING_DELAY_MILLISECONDS.to_owned(),
            typed_text.to_owned(),
        ],
        KeyInput::ControlChord(chord_key) => std::iter::once("key".to_owned())
            .chain(pressed_and_released(&[
                LEFT_CONTROL_KEYCODE,
                chord_key.keycode(),
            ]))
            .collect(),
        KeyInput::Enter => std::iter::once("key".to_owned())
            .chain(pressed_and_released(&[ENTER_KEYCODE]))
            .collect(),
        KeyInput::Escape => std::iter::once("key".to_owned())
            .chain(pressed_and_released(&[ESCAPE_KEYCODE]))
            .collect(),
        KeyInput::Down => std::iter::once("key".to_owned())
            .chain(pressed_and_released(&[DOWN_KEYCODE]))
            .collect(),
    };
    let argument_texts = ydotool_arguments
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    succeed("ydotool", &argument_texts).map(|_sent| ())
}

pub fn send_keys(key_inputs: &[KeyInput<'_>]) -> Result<(), JourneyFailure> {
    key_inputs.iter().copied().try_for_each(send_key_input)
}

pub fn answer_password_prompt(password: &str) -> Result<(), JourneyFailure> {
    wait_for_focused_window(PASSWORD_PROMPT_TITLE)?;
    send_keys(&[KeyInput::TypedText(password), KeyInput::Enter])
}

pub fn answer_every_password_prompt(password: &str) -> Result<(), JourneyFailure> {
    answer_password_prompt(password)?;
    loop {
        wait_for_window_closed(PASSWORD_PROMPT_TITLE)?;
        let follow_up_prompt = wait_until(
            "a follow-up password prompt",
            Duration::from_secs(10),
            || focused_window_is_titled(PASSWORD_PROMPT_TITLE),
        );
        if follow_up_prompt.is_err() {
            return Ok(());
        }
        settle(WINDOW_SETTLE);
        send_keys(&[KeyInput::TypedText(password), KeyInput::Enter])?;
    }
}

pub fn answer_password_prompts_if_asked(password: &str) -> Result<(), JourneyFailure> {
    let prompt_appeared = wait_until("a password prompt", Duration::from_secs(10), || {
        focused_window_is_titled(PASSWORD_PROMPT_TITLE)
    });
    match prompt_appeared {
        Ok(()) => answer_every_password_prompt(password),
        Err(_kept_authorization) => Ok(()),
    }
}

pub fn dismiss_password_prompt() -> Result<(), JourneyFailure> {
    wait_for_focused_window(PASSWORD_PROMPT_TITLE)?;
    send_keys(&[KeyInput::Escape])?;
    wait_for_window_closed(PASSWORD_PROMPT_TITLE)
}

pub fn take_screenshot(screenshot_name: &str) {
    if fs::create_dir_all(ARTIFACTS_DIRECTORY).is_ok() {
        let _screenshot_attempt = succeed(
            "grim",
            &[&format!("{ARTIFACTS_DIRECTORY}/{screenshot_name}.png")],
        );
    }
}

fn session_connection() -> Result<zbus::blocking::Connection, JourneyFailure> {
    Ok(zbus::blocking::Connection::session()?)
}

fn activate_gtk_action(
    object_path: &str,
    action_name: &str,
    action_parameters: Vec<zvariant::Value<'_>>,
) -> Result<(), JourneyFailure> {
    session_connection()?.call_method(
        Some(APPLICATION_BUS_NAME),
        object_path,
        Some(GTK_ACTIONS_INTERFACE),
        "Activate",
        &(
            action_name,
            action_parameters,
            HashMap::<&str, zvariant::Value<'_>>::new(),
        ),
    )?;
    Ok(())
}

pub fn activate_application_action(
    application_action: ApplicationAction,
) -> Result<(), JourneyFailure> {
    activate_gtk_action(
        APPLICATION_OBJECT_PATH,
        application_action.action_name(),
        Vec::new(),
    )
}

pub fn show_page(application_page: ApplicationPage) -> Result<(), JourneyFailure> {
    activate_application_action(ApplicationAction::ShowWindow)?;
    wait_for_lazaret_window()?;
    activate_gtk_action(
        WINDOW_OBJECT_PATH,
        "show-page",
        vec![zvariant::Value::from(application_page.page_name())],
    )
}

fn tray_bus_name() -> Result<Option<String>, JourneyFailure> {
    let listed_names: Vec<String> = session_connection()?
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "ListNames",
            &(),
        )?
        .body()
        .deserialize()?;
    Ok(listed_names
        .into_iter()
        .find(|listed_name| listed_name.contains("StatusNotifierItem")))
}

fn tray_status_value() -> Result<Option<String>, JourneyFailure> {
    let Some(tray_name) = tray_bus_name()? else {
        return Ok(None);
    };
    let status_value: zvariant::OwnedValue = session_connection()?
        .call_method(
            Some(tray_name.as_str()),
            TRAY_ITEM_PATH,
            Some("org.freedesktop.DBus.Properties"),
            "Get",
            &(TRAY_ITEM_INTERFACE, "Status"),
        )?
        .body()
        .deserialize()?;
    Ok(String::try_from(status_value).ok())
}

pub fn wait_for_tray_status(expected_status: TrayStatus) -> Result<(), JourneyFailure> {
    wait_until(
        &format!("the tray status {}", expected_status.bus_value()),
        Duration::from_secs(30),
        || Ok(tray_status_value()?.as_deref() == Some(expected_status.bus_value())),
    )
}

pub fn wait_for_lazaret_service() -> Result<(), JourneyFailure> {
    wait_until("the Lazaret user service", Duration::from_secs(60), || {
        Ok(succeed("systemctl", &["--user", "is-active", "lazaret.service"]).is_ok())
    })?;
    wait_until(
        "Lazaret on the session bus",
        Duration::from_secs(60),
        || Ok(succeed("busctl", &["--user", "status", APPLICATION_BUS_NAME]).is_ok()),
    )?;
    wait_until("the Lazaret tray item", Duration::from_secs(60), || {
        Ok(tray_bus_name()?.is_some())
    })?;
    settle(Duration::from_secs(2));
    Ok(())
}

pub fn scan_with_lazaret(scanned_paths: &[&str]) -> Result<(), JourneyFailure> {
    succeed("lazaret", scanned_paths)?;
    wait_for_lazaret_window()
}
