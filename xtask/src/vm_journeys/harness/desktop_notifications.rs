use std::time::Duration;

use serde_json::Value;

use crate::vm_journeys::harness::system_commands::{succeed, wait_until};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NotificationUrgency {
    Critical,
    OtherUrgency,
}

pub struct DesktopNotification {
    pub shown_text: String,
    pub urgency: NotificationUrgency,
}

pub fn json_text(json_value: &Value) -> String {
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

pub fn listed_notifications() -> Vec<DesktopNotification> {
    let listing_text = succeed("makoctl", &["list", "-j"]);
    let listing: Value = serde_json::from_str(&listing_text).unwrap_or_else(|parse_failure| {
        panic!("makoctl listed unreadable JSON: {parse_failure}\n{listing_text}")
    });
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
    notification_values.iter().map(notification_from).collect()
}

pub fn notifications_mentioning(mentioned_text: &str) -> Vec<DesktopNotification> {
    listed_notifications()
        .into_iter()
        .filter(|desktop_notification| desktop_notification.shown_text.contains(mentioned_text))
        .collect()
}

pub fn wait_for_notification(mentioned_text: &str) -> Vec<DesktopNotification> {
    wait_until(
        &format!("a notification mentioning {mentioned_text:?}"),
        Duration::from_secs(60),
        || !notifications_mentioning(mentioned_text).is_empty(),
    );
    notifications_mentioning(mentioned_text)
}

pub fn wait_for_notification_withdrawn(mentioned_text: &str) {
    wait_until(
        &format!("the withdrawal of the notification mentioning {mentioned_text:?}"),
        Duration::from_secs(30),
        || notifications_mentioning(mentioned_text).is_empty(),
    );
}

pub fn invoke_latest_notification() {
    succeed("makoctl", &["invoke"]);
}

pub fn dismiss_notifications() {
    succeed("makoctl", &["dismiss", "--all"]);
}
