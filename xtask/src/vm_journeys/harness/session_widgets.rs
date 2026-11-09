use std::{
    env,
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

use gtk_scene_capture::{
    accessible_widget_lookup::{
        AccessibleTree, ApplicationScope, LocatedWidget, NameMatch, WidgetLookupFailure,
        WidgetRole, WidgetSensitivity, WidgetTarget,
    },
    scene_capture_failure::SceneCaptureFailure,
    virtual_pointer_input::{ScreenPosition, ScreenSize, VirtualPointerInput},
};

use crate::vm_journeys::harness::journey_failure::JourneyFailure;

const ACCESSIBILITY_BUS_NAME: &str = "org.a11y.Bus";
const ACCESSIBILITY_BUS_PATH: &str = "/org/a11y/bus";
const TESTED_APPLICATION_NAME: &str = "lazaret";
const DEFAULT_WAYLAND_DISPLAY: &str = "wayland-1";
const LOOKUP_RETRY_PAUSE: Duration = Duration::from_millis(300);
const SLOW_LOOKUP: Duration = Duration::from_secs(1);
const SCROLL_WHEEL_CLICK_LIMIT: usize = 40;
const VISIBLE_EDGE_MARGIN: i32 = 24;
const SESSION_SCREEN_SIZE: ScreenSize = ScreenSize {
    logical_width: 1280,
    logical_height: 800,
};
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

fn lookup_failed(lookup_failure: WidgetLookupFailure) -> JourneyFailure {
    JourneyFailure::WidgetUnavailable(SceneCaptureFailure::from(lookup_failure))
}

fn session_accessibility_address() -> Result<String, JourneyFailure> {
    Ok(zbus::blocking::Connection::session()?
        .call_method(
            Some(ACCESSIBILITY_BUS_NAME),
            ACCESSIBILITY_BUS_PATH,
            Some(ACCESSIBILITY_BUS_NAME),
            "GetAddress",
            &(),
        )?
        .body()
        .deserialize::<String>()?)
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

fn text_position_of(located_widget: &LocatedWidget) -> ScreenPosition {
    let widget_bounds = located_widget.widget_bounds;
    let (_center_horizontal, center_vertical) = widget_bounds.center_point();
    ScreenPosition {
        logical_x: (widget_bounds.left_edge + widget_bounds.widget_width / 4)
            .max(0)
            .unsigned_abs(),
        logical_y: center_vertical.max(0).unsigned_abs(),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PressMethod {
    AccessibleActionFirst,
    PointerClickIntoText,
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
    pub fn connect() -> Result<Self, JourneyFailure> {
        let accessibility_address = session_accessibility_address()?;
        let async_runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let accessible_tree = async_runtime
            .block_on(AccessibleTree::connect_to(&accessibility_address))
            .map_err(lookup_failed)?
            .limited_to(ApplicationScope::ApplicationNamed(
                TESTED_APPLICATION_NAME.to_owned(),
            ));
        Ok(Self {
            async_runtime,
            accessible_tree,
        })
    }

    fn try_find(&self, widget_target: &WidgetTarget) -> Result<LocatedWidget, WidgetLookupFailure> {
        let lookup_started = Instant::now();
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

    fn wait_for_widget(
        &self,
        widget_target: &WidgetTarget,
        timeout: Duration,
    ) -> Result<LocatedWidget, JourneyFailure> {
        let deadline = Instant::now() + timeout;
        loop {
            match self.try_find(widget_target) {
                Ok(located_widget) => return Ok(located_widget),
                Err(_vanished_or_missing) if Instant::now() < deadline => {
                    thread::sleep(LOOKUP_RETRY_PAUSE);
                }
                Err(lookup_failure) => return Err(lookup_failed(lookup_failure)),
            }
        }
    }

    fn wait_for_sensitive_widget(
        &self,
        widget_target: &WidgetTarget,
        timeout: Duration,
    ) -> Result<LocatedWidget, JourneyFailure> {
        let deadline = Instant::now() + timeout;
        loop {
            let located_widget = self.wait_for_widget(widget_target, timeout)?;
            match self
                .async_runtime
                .block_on(self.accessible_tree.widget_sensitivity(&located_widget))
                .map_err(lookup_failed)?
            {
                WidgetSensitivity::AcceptsInput => return Ok(located_widget),
                WidgetSensitivity::RefusesInput if Instant::now() < deadline => {
                    thread::sleep(LOOKUP_RETRY_PAUSE);
                }
                WidgetSensitivity::RefusesInput => {
                    return Err(JourneyFailure::ExpectationFailed(format!(
                        "{widget_target:?} never accepted input"
                    )));
                }
            }
        }
    }

    pub fn await_shown(&self, widget_target: &WidgetTarget) -> Result<(), JourneyFailure> {
        self.await_shown_within(widget_target, DEFAULT_WIDGET_TIMEOUT)
    }

    pub fn await_shown_within(
        &self,
        widget_target: &WidgetTarget,
        timeout: Duration,
    ) -> Result<(), JourneyFailure> {
        self.wait_for_widget(widget_target, timeout)
            .map(|_located_widget| ())
    }

    pub fn await_gone(&self, widget_target: &WidgetTarget) -> Result<(), JourneyFailure> {
        let deadline = Instant::now() + DEFAULT_WIDGET_TIMEOUT;
        loop {
            match self.try_find(widget_target) {
                Err(WidgetLookupFailure::WidgetMissing { .. }) => return Ok(()),
                Err(_) | Ok(_) if Instant::now() < deadline => {
                    thread::sleep(LOOKUP_RETRY_PAUSE);
                }
                Err(lookup_failure) => return Err(lookup_failed(lookup_failure)),
                Ok(_still_shown) => {
                    return Err(JourneyFailure::ExpectationFailed(format!(
                        "{widget_target:?} was still shown"
                    )));
                }
            }
        }
    }

    pub fn is_shown(&self, widget_target: &WidgetTarget) -> bool {
        self.try_find(widget_target).is_ok()
    }

    pub fn press(&self, widget_target: &WidgetTarget) -> Result<(), JourneyFailure> {
        self.press_within(widget_target, DEFAULT_WIDGET_TIMEOUT)
    }

    pub fn press_within(
        &self,
        widget_target: &WidgetTarget,
        timeout: Duration,
    ) -> Result<(), JourneyFailure> {
        self.press_by(widget_target, timeout, PressMethod::AccessibleActionFirst)
    }

    pub fn click_into(&self, widget_target: &WidgetTarget) -> Result<(), JourneyFailure> {
        self.press_by(
            widget_target,
            DEFAULT_WIDGET_TIMEOUT,
            PressMethod::PointerClickIntoText,
        )
    }

    fn press_by(
        &self,
        widget_target: &WidgetTarget,
        timeout: Duration,
        press_method: PressMethod,
    ) -> Result<(), JourneyFailure> {
        let deadline = Instant::now() + timeout;
        loop {
            match self.press_attempt(widget_target, timeout, press_method) {
                Err(JourneyFailure::WidgetUnavailable(_vanished_widget))
                    if Instant::now() < deadline =>
                {
                    thread::sleep(LOOKUP_RETRY_PAUSE);
                }
                Err(press_failure) => return Err(press_failure),
                Ok(()) => {
                    if let gtk_scene_capture::accessible_widget_lookup::SearchScope::InsideWidget(
                        container_target,
                    ) = &widget_target.search_scope
                        && container_target.widget_role == WidgetRole::Alert
                    {
                        self.await_gone(container_target)?;
                    }
                    return Ok(());
                }
            }
        }
    }

    fn press_attempt(
        &self,
        widget_target: &WidgetTarget,
        timeout: Duration,
        press_method: PressMethod,
    ) -> Result<(), JourneyFailure> {
        let mut pointer_input =
            VirtualPointerInput::connect(&session_wayland_socket(), SESSION_SCREEN_SIZE)?;
        let mut located_widget = self.wait_for_sensitive_widget(widget_target, timeout)?;
        let screen_center = ScreenPosition {
            logical_x: SESSION_SCREEN_SIZE.logical_width / 2,
            logical_y: SESSION_SCREEN_SIZE.logical_height / 2,
        };
        for _wheel_click in 0..SCROLL_WHEEL_CLICK_LIMIT {
            if !lies_below_screen(&located_widget) {
                break;
            }
            pointer_input.scroll_wheel_down(screen_center)?;
            let scrolled_widget = self.wait_for_widget(widget_target, timeout)?;
            let scroll_moved_widget = scrolled_widget.widget_bounds != located_widget.widget_bounds;
            located_widget = scrolled_widget;
            if !scroll_moved_widget {
                break;
            }
        }
        if press_method == PressMethod::PointerClickIntoText {
            return Ok(pointer_input.click_at(text_position_of(&located_widget))?);
        }
        pointer_input.move_to(screen_position_of(&located_widget))?;
        let activation_outcome = self.async_runtime.block_on(
            self.accessible_tree
                .activate_widget(&located_widget, widget_target),
        );
        match activation_outcome {
            Ok(()) => {}
            Err(WidgetLookupFailure::ActionMissing { .. }) => {
                pointer_input.click_at(screen_position_of(&located_widget))?;
            }
            Err(activation_failure) => {
                if !matches!(
                    self.try_find(widget_target),
                    Err(WidgetLookupFailure::WidgetMissing { .. })
                ) {
                    return Err(lookup_failed(activation_failure));
                }
            }
        }
        Ok(())
    }

    pub fn type_into(
        &self,
        widget_target: &WidgetTarget,
        typed_text: &str,
    ) -> Result<(), JourneyFailure> {
        let located_widget =
            self.wait_for_sensitive_widget(widget_target, DEFAULT_WIDGET_TIMEOUT)?;
        self.async_runtime
            .block_on(self.accessible_tree.replace_widget_text(
                &located_widget,
                widget_target,
                typed_text,
            ))
            .map_err(lookup_failed)
    }

    pub fn describe_widget_input(&self, widget_target: &WidgetTarget) -> String {
        self.wait_for_widget(widget_target, DEFAULT_WIDGET_TIMEOUT)
            .and_then(|located_widget| {
                self.async_runtime
                    .block_on(self.accessible_tree.describe_widget_input(&located_widget))
                    .map_err(lookup_failed)
            })
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
