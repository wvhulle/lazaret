use std::panic;

use crate::vm_journeys::harness::{session_widgets::SessionWidgets, sway_windows::take_screenshot};

pub const SAMPLE_PASSWORD: &str = "sample-password";
pub const SAMPLE_HOME: &str = "/home/sample-user";

pub fn announce_step(step_description: &str) {
    eprintln!("step: {step_description}");
}

pub fn visible_widget_descriptions() -> Vec<String> {
    SessionWidgets::try_connect()
        .map(|session_widgets| session_widgets.describe_shown_widgets())
        .unwrap_or_else(|connection_failure| {
            vec![format!(
                "the accessibility tree could not be reached: {connection_failure}"
            )]
        })
}

pub fn record_evidence_on_failure() {
    let default_panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_information| {
        default_panic_hook(panic_information);
        take_screenshot("failure");
        eprintln!("visible widgets:");
        visible_widget_descriptions()
            .iter()
            .for_each(|widget_description| eprintln!("  {widget_description}"));
    }));
}
