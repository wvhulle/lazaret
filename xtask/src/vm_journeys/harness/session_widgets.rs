use std::{env, error::Error, path::PathBuf, time::Duration};

use gtk_scene_capture::{
    accessible_widget_lookup::{
        AccessibleTree, ApplicationScope, LocatedWidget, NameMatch, SearchScope,
        WidgetLookupFailure, WidgetRole, WidgetSensitivity, WidgetTarget,
    },
    scene_capture_failure::SceneCaptureFailure,
    virtual_pointer_input::{ScreenPosition, ScreenSize, VirtualPointerInput},
};

use crate::vm_journeys::harness::system_commands::{poll_until, poll_within};

const ACCESSIBILITY_BUS_NAME: &str = "org.a11y.Bus";
const ACCESSIBILITY_BUS_PATH: &str = "/org/a11y/bus";
const TESTED_APPLICATION_NAME: &str = "lazaret";
const DEFAULT_WAYLAND_DISPLAY: &str = "wayland-1";
const SLOW_LOOKUP: Duration = Duration::from_secs(1);
const SCROLL_WHEEL_CLICK_LIMIT: usize = 40;
const VISIBLE_EDGE_MARGIN: i32 = 24;
const SESSION_SCREEN_SIZE: ScreenSize = ScreenSize {
    logical_width: 1280,
    logical_height: 800,
};
const PRESS_EFFECT_WAIT: Duration = Duration::from_secs(3);
pub const DEFAULT_WIDGET_TIMEOUT: Duration = Duration::from_secs(30);

pub fn button(accessible_name: impl Into<String>) -> WidgetTarget {
    WidgetTarget::button(accessible_name)
}

pub fn label_containing(shown_text: impl Into<String>) -> WidgetTarget {
    WidgetTarget::label(shown_text).matching_names(NameMatch::ContainedName)
}

pub fn row_titled(row_title: impl Into<String>) -> WidgetTarget {
    WidgetTarget::list_item(row_title)
}

pub fn in_alert(widget_target: WidgetTarget, alert_heading: impl Into<String>) -> WidgetTarget {
    widget_target.inside(WidgetTarget::alert(alert_heading))
}

pub fn in_row(widget_target: WidgetTarget, row_title: impl Into<String>) -> WidgetTarget {
    widget_target.inside(WidgetTarget::list_item(row_title))
}

pub fn in_expanded_row(widget_target: WidgetTarget, row_title: impl Into<String>) -> WidgetTarget {
    widget_target.inside_enclosing(WidgetRole::ListItem, WidgetTarget::list_item(row_title))
}

fn session_accessibility_address() -> zbus::Result<String> {
    zbus::blocking::Connection::session()?
        .call_method(
            Some(ACCESSIBILITY_BUS_NAME),
            ACCESSIBILITY_BUS_PATH,
            Some(ACCESSIBILITY_BUS_NAME),
            "GetAddress",
            &(),
        )?
        .body()
        .deserialize::<String>()
}

fn session_wayland_socket() -> PathBuf {
    PathBuf::from(env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_unset| "/run/user/1000".to_owned()))
        .join(
            env::var("WAYLAND_DISPLAY").unwrap_or_else(|_unset| DEFAULT_WAYLAND_DISPLAY.to_owned()),
        )
}

fn screen_position_of(located_widget: &LocatedWidget) -> ScreenPosition {
    let (center_horizontal, center_vertical) = located_widget.widget_bounds.center_point();
    ScreenPosition {
        logical_x: center_horizontal.max(0).unsigned_abs(),
        logical_y: center_vertical.max(0).unsigned_abs(),
    }
}

fn lies_below_screen(located_widget: &LocatedWidget) -> bool {
    let screen_height = i32::try_from(SESSION_SCREEN_SIZE.logical_height).unwrap_or(i32::MAX);
    let (_center_horizontal, center_vertical) = located_widget.widget_bounds.center_point();
    center_vertical >= screen_height - VISIBLE_EDGE_MARGIN
}

pub struct SessionWidgets {
    async_runtime: tokio::runtime::Runtime,
    accessible_tree: AccessibleTree,
}

impl SessionWidgets {
    pub fn try_connect() -> Result<Self, Box<dyn Error>> {
        let accessibility_address = session_accessibility_address()?;
        let async_runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let accessible_tree = async_runtime
            .block_on(AccessibleTree::connect_to(&accessibility_address))?
            .limited_to(ApplicationScope::ApplicationNamed(
                TESTED_APPLICATION_NAME.to_owned(),
            ));
        Ok(Self {
            async_runtime,
            accessible_tree,
        })
    }

    pub fn connect() -> Self {
        Self::try_connect().unwrap_or_else(|connection_failure| {
            panic!("the session's accessibility tree could not be reached: {connection_failure}")
        })
    }

    fn try_find(&self, widget_target: &WidgetTarget) -> Result<LocatedWidget, WidgetLookupFailure> {
        let lookup_started = std::time::Instant::now();
        let lookup_outcome = self
            .async_runtime
            .block_on(self.accessible_tree.find_widget(widget_target));
        let lookup_duration = lookup_started.elapsed();
        if lookup_duration > SLOW_LOOKUP {
            eprintln!(
                "slow lookup ({:.1}s, {}): {widget_target:?}",
                lookup_duration.as_secs_f64(),
                if lookup_outcome.is_ok() {
                    "found"
                } else {
                    "missing"
                }
            );
        }
        lookup_outcome
    }

    fn located_within(
        &self,
        widget_target: &WidgetTarget,
        timeout: Duration,
    ) -> Option<LocatedWidget> {
        poll_within(timeout, || self.try_find(widget_target).ok())
    }

    fn wait_for_widget(&self, widget_target: &WidgetTarget, timeout: Duration) -> LocatedWidget {
        poll_until(&format!("showing {widget_target:?}"), timeout, || {
            self.try_find(widget_target).ok()
        })
    }

    fn accepts_input(&self, located_widget: &LocatedWidget) -> bool {
        matches!(
            self.async_runtime
                .block_on(self.accessible_tree.widget_sensitivity(located_widget)),
            Ok(WidgetSensitivity::AcceptsInput)
        )
    }

    fn wait_for_sensitive_widget(
        &self,
        widget_target: &WidgetTarget,
        timeout: Duration,
    ) -> LocatedWidget {
        poll_until(
            &format!("{widget_target:?} accepting input"),
            timeout,
            || {
                let located_widget = self.try_find(widget_target).ok()?;
                self.accepts_input(&located_widget)
                    .then_some(located_widget)
            },
        )
    }

    pub fn await_shown(&self, widget_target: &WidgetTarget) {
        self.await_shown_within(widget_target, DEFAULT_WIDGET_TIMEOUT);
    }

    pub fn await_shown_within(&self, widget_target: &WidgetTarget, timeout: Duration) {
        self.wait_for_widget(widget_target, timeout);
    }

    pub fn is_shown_within(&self, widget_target: &WidgetTarget, timeout: Duration) -> bool {
        self.located_within(widget_target, timeout).is_some()
    }

    pub fn await_gone(&self, widget_target: &WidgetTarget) {
        poll_until(
            &format!("{widget_target:?} disappearing"),
            DEFAULT_WIDGET_TIMEOUT,
            || {
                matches!(
                    self.try_find(widget_target),
                    Err(WidgetLookupFailure::WidgetMissing { .. })
                )
                .then_some(())
            },
        );
    }

    pub fn is_shown(&self, widget_target: &WidgetTarget) -> bool {
        self.try_find(widget_target).is_ok()
    }

    pub fn press(&self, widget_target: &WidgetTarget) {
        self.press_within(widget_target, DEFAULT_WIDGET_TIMEOUT);
    }

    pub fn press_within(&self, widget_target: &WidgetTarget, timeout: Duration) {
        poll_until(&format!("pressing {widget_target:?}"), timeout, || {
            self.press_attempt(widget_target, timeout)
                .inspect_err(|press_failure| {
                    eprintln!("pressing {widget_target:?} failed, retrying: {press_failure}");
                })
                .ok()
        });
        if let SearchScope::InsideWidget(container_target) = &widget_target.search_scope
            && container_target.widget_role == WidgetRole::Alert
        {
            self.await_gone(container_target);
        }
    }

    pub fn press_until_shown(&self, pressed_target: &WidgetTarget, awaited_target: &WidgetTarget) {
        poll_until(
            &format!("pressing {pressed_target:?} showing {awaited_target:?}"),
            DEFAULT_WIDGET_TIMEOUT,
            || {
                self.press(pressed_target);
                self.is_shown_within(awaited_target, PRESS_EFFECT_WAIT)
                    .then_some(())
            },
        );
    }

    pub fn try_press_within(&self, widget_target: &WidgetTarget, timeout: Duration) -> bool {
        let Some(located_widget) = self.located_within(widget_target, timeout) else {
            return false;
        };
        self.accepts_input(&located_widget) && self.press_attempt(widget_target, timeout).is_ok()
    }

    fn scrolled_into_view(
        &self,
        widget_target: &WidgetTarget,
        pointer_input: &mut VirtualPointerInput,
        timeout: Duration,
    ) -> Result<LocatedWidget, SceneCaptureFailure> {
        let screen_center = ScreenPosition {
            logical_x: SESSION_SCREEN_SIZE.logical_width / 2,
            logical_y: SESSION_SCREEN_SIZE.logical_height / 2,
        };
        let mut located_widget = self.wait_for_sensitive_widget(widget_target, timeout);
        for _wheel_click in 0..SCROLL_WHEEL_CLICK_LIMIT {
            if !lies_below_screen(&located_widget) {
                break;
            }
            pointer_input.scroll_wheel_down(screen_center)?;
            let scrolled_widget = self.wait_for_widget(widget_target, timeout);
            let scroll_moved_widget = scrolled_widget.widget_bounds != located_widget.widget_bounds;
            located_widget = scrolled_widget;
            if !scroll_moved_widget {
                break;
            }
        }
        Ok(located_widget)
    }

    fn press_attempt(
        &self,
        widget_target: &WidgetTarget,
        timeout: Duration,
    ) -> Result<(), SceneCaptureFailure> {
        let mut pointer_input =
            VirtualPointerInput::connect(&session_wayland_socket(), SESSION_SCREEN_SIZE)?;
        let located_widget = self.scrolled_into_view(widget_target, &mut pointer_input, timeout)?;
        pointer_input.move_to(screen_position_of(&located_widget))?;
        let activation_outcome = self.async_runtime.block_on(
            self.accessible_tree
                .activate_widget(&located_widget, widget_target),
        );
        match activation_outcome {
            Ok(()) => Ok(()),
            Err(WidgetLookupFailure::ActionMissing { .. }) => {
                pointer_input.click_at(screen_position_of(&located_widget))
            }
            Err(activation_failure) => match self.try_find(widget_target) {
                Err(WidgetLookupFailure::WidgetMissing { .. }) => Ok(()),
                Err(_) | Ok(_) => Err(SceneCaptureFailure::from(activation_failure)),
            },
        }
    }

    pub fn type_into(&self, widget_target: &WidgetTarget, typed_text: &str) {
        let located_widget = self.wait_for_sensitive_widget(widget_target, DEFAULT_WIDGET_TIMEOUT);
        self.async_runtime
            .block_on(self.accessible_tree.replace_widget_text(
                &located_widget,
                widget_target,
                typed_text,
            ))
            .unwrap_or_else(|typing_failure| {
                panic!("typing into {widget_target:?} failed: {typing_failure}")
            });
    }

    pub fn set_spin_value(&self, widget_target: &WidgetTarget, assigned_value: f64) {
        let located_widget = self.wait_for_sensitive_widget(widget_target, DEFAULT_WIDGET_TIMEOUT);
        self.async_runtime
            .block_on(
                self.accessible_tree
                    .assign_spin_value(&located_widget, assigned_value),
            )
            .unwrap_or_else(|assignment_failure| {
                panic!("setting {widget_target:?} to {assigned_value} failed: {assignment_failure}")
            });
    }

    pub fn describe_widget_input(&self, widget_target: &WidgetTarget) -> String {
        let Some(located_widget) = self.located_within(widget_target, DEFAULT_WIDGET_TIMEOUT)
        else {
            return format!("unreadable: {widget_target:?} is not shown");
        };
        self.async_runtime
            .block_on(self.accessible_tree.describe_widget_input(&located_widget))
            .unwrap_or_else(|lookup_failure| format!("unreadable: {lookup_failure}"))
    }

    pub fn describe_ancestry_of_named(&self, name_fragment: &str) -> Vec<String> {
        self.async_runtime
            .block_on(
                self.accessible_tree
                    .describe_ancestry_of_named(name_fragment),
            )
            .unwrap_or_default()
    }

    pub fn describe_shown_widgets(&self) -> Vec<String> {
        self.async_runtime
            .block_on(self.accessible_tree.describe_visible_widgets())
            .unwrap_or_default()
    }
}
