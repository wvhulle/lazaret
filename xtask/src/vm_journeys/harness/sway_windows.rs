use std::{fs, process::Command, time::Duration};

use serde_json::Value;

use crate::vm_journeys::harness::{
    desktop_notifications::json_text,
    system_commands::{holds_within, settle, succeed, wait_until},
};

pub const ARTIFACTS_DIRECTORY: &str = "/tmp/vm-journey-artifacts";
const PASSWORD_PROMPT_TITLE: &str = "Authenticat";
const WINDOW_SETTLE: Duration = Duration::from_secs(1);
const PROMPT_TIMEOUT: Duration = Duration::from_secs(60);
const FOLLOW_UP_PROMPT_WAIT: Duration = Duration::from_secs(10);
const ENTER_KEYCODE: u16 = 28;
const ESCAPE_KEYCODE: u16 = 1;
const DOWN_KEYCODE: u16 = 108;
const TYPING_DELAY_MILLISECONDS: &str = "20";

pub struct SwayWindow {
    pub window_title: String,
    pub is_focused: bool,
    pub is_visible: bool,
    pub application_identifier: String,
}

fn window_from(sway_node: &Value) -> SwayWindow {
    SwayWindow {
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
    }
}

pub fn sway_windows() -> Vec<SwayWindow> {
    let tree_text = succeed("swaymsg", &["-t", "get_tree"]);
    let sway_tree: Value = serde_json::from_str(&tree_text).unwrap_or_else(|parse_failure| {
        panic!("swaymsg returned an unreadable tree: {parse_failure}")
    });
    let mut pending_nodes = vec![sway_tree];
    let mut found_windows = Vec::new();
    while let Some(sway_node) = pending_nodes.pop() {
        if sway_node.get("pid").and_then(Value::as_u64).is_some() {
            found_windows.push(window_from(&sway_node));
        }
        for child_key in ["nodes", "floating_nodes"] {
            if let Some(child_nodes) = sway_node.get(child_key).and_then(Value::as_array) {
                pending_nodes.extend(child_nodes.iter().cloned());
            }
        }
    }
    found_windows
}

fn window_titled_is_open(title_fragment: &str) -> bool {
    sway_windows()
        .iter()
        .any(|sway_window| sway_window.window_title.contains(title_fragment))
}

fn focused_window_is_titled(title_fragment: &str) -> bool {
    sway_windows().iter().any(|sway_window| {
        sway_window.is_focused && sway_window.window_title.contains(title_fragment)
    })
}

pub fn wait_for_focused_window(title_fragment: &str) {
    wait_until(
        &format!("a focused window titled {title_fragment:?}"),
        PROMPT_TIMEOUT,
        || focused_window_is_titled(title_fragment),
    );
    settle(WINDOW_SETTLE);
}

pub fn wait_for_window_closed(title_fragment: &str) {
    wait_until(
        &format!("the window titled {title_fragment:?} closing"),
        PROMPT_TIMEOUT,
        || !window_titled_is_open(title_fragment),
    );
}

#[derive(Clone, Copy, Debug)]
pub enum KeyInput<'typed> {
    TypedText(&'typed str),
    Enter,
    Escape,
    Down,
}

fn pressed_and_released(keycode: u16) -> [String; 3] {
    [
        "key".to_owned(),
        format!("{keycode}:1"),
        format!("{keycode}:0"),
    ]
}

fn send_key_input(key_input: KeyInput<'_>) {
    let ydotool_arguments = match key_input {
        KeyInput::TypedText(typed_text) => vec![
            "type".to_owned(),
            "--key-delay".to_owned(),
            TYPING_DELAY_MILLISECONDS.to_owned(),
            typed_text.to_owned(),
        ],
        KeyInput::Enter => pressed_and_released(ENTER_KEYCODE).to_vec(),
        KeyInput::Escape => pressed_and_released(ESCAPE_KEYCODE).to_vec(),
        KeyInput::Down => pressed_and_released(DOWN_KEYCODE).to_vec(),
    };
    let argument_texts = ydotool_arguments
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    succeed("ydotool", &argument_texts);
}

pub fn send_keys(key_inputs: &[KeyInput<'_>]) {
    key_inputs.iter().copied().for_each(send_key_input);
}

pub fn answer_password_prompt(password: &str) {
    wait_for_focused_window(PASSWORD_PROMPT_TITLE);
    send_keys(&[KeyInput::TypedText(password), KeyInput::Enter]);
}

pub fn answer_every_password_prompt(password: &str) {
    answer_password_prompt(password);
    loop {
        wait_for_window_closed(PASSWORD_PROMPT_TITLE);
        if !holds_within(FOLLOW_UP_PROMPT_WAIT, || {
            focused_window_is_titled(PASSWORD_PROMPT_TITLE)
        }) {
            return;
        }
        settle(WINDOW_SETTLE);
        send_keys(&[KeyInput::TypedText(password), KeyInput::Enter]);
    }
}

pub fn answer_password_prompts_if_asked(password: &str) {
    if holds_within(FOLLOW_UP_PROMPT_WAIT, || {
        focused_window_is_titled(PASSWORD_PROMPT_TITLE)
    }) {
        answer_every_password_prompt(password);
    }
}

pub fn dismiss_password_prompt() {
    wait_for_focused_window(PASSWORD_PROMPT_TITLE);
    send_keys(&[KeyInput::Escape]);
    wait_for_window_closed(PASSWORD_PROMPT_TITLE);
}

pub fn take_screenshot(screenshot_name: &str) {
    if fs::create_dir_all(ARTIFACTS_DIRECTORY).is_ok() {
        let screenshot_status = Command::new("grim")
            .arg(format!("{ARTIFACTS_DIRECTORY}/{screenshot_name}.png"))
            .status();
        if let Err(screenshot_failure) = screenshot_status {
            eprintln!("the screenshot {screenshot_name} failed: {screenshot_failure}");
        }
    }
}
