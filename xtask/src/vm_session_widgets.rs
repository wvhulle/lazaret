use std::{
    env,
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

use clap::{Args, ValueEnum};
use gtk_scene_capture::{
    accessible_widget_lookup::{
        AccessibleTree, ApplicationScope, LocatedWidget, NameMatch, WidgetLookupFailure,
        WidgetRole, WidgetSensitivity, WidgetTarget,
    },
    virtual_pointer_input::{ScreenPosition, ScreenSize, VirtualPointerInput},
};

use crate::{task_failure::TaskFailure, tool_commands::required_variable};

const ACCESSIBILITY_BUS_NAME: &str = "org.a11y.Bus";
const ACCESSIBILITY_BUS_PATH: &str = "/org/a11y/bus";
const LOOKUP_RETRY_PAUSE: Duration = Duration::from_millis(500);
const DEFAULT_WAYLAND_DISPLAY: &str = "wayland-1";
const SCROLL_WHEEL_CLICK_LIMIT: usize = 40;
const VISIBLE_EDGE_MARGIN: i32 = 24;
const TESTED_APPLICATION_NAME: &str = "lazaret";

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum WidgetRoleName {
    Button,
    ToggleButton,
    CheckBox,
    ListItem,
    Label,
    Dialog,
    Alert,
    PageTab,
    MenuItem,
    Entry,
    Text,
    Frame,
    PasswordText,
    Image,
    SpinButton,
}

impl From<WidgetRoleName> for WidgetRole {
    fn from(role_name: WidgetRoleName) -> Self {
        match role_name {
            WidgetRoleName::Button => Self::Button,
            WidgetRoleName::ToggleButton => Self::ToggleButton,
            WidgetRoleName::CheckBox => Self::CheckBox,
            WidgetRoleName::ListItem => Self::ListItem,
            WidgetRoleName::Label => Self::Label,
            WidgetRoleName::Dialog => Self::Dialog,
            WidgetRoleName::Alert => Self::Alert,
            WidgetRoleName::PageTab => Self::PageTab,
            WidgetRoleName::MenuItem => Self::MenuItem,
            WidgetRoleName::Entry => Self::Entry,
            WidgetRoleName::Text => Self::Text,
            WidgetRoleName::Frame => Self::Frame,
            WidgetRoleName::PasswordText => Self::PasswordText,
            WidgetRoleName::Image => Self::Image,
            WidgetRoleName::SpinButton => Self::SpinButton,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum NameMatchMode {
    #[default]
    Exact,
    Contained,
}

impl From<NameMatchMode> for NameMatch {
    fn from(match_mode: NameMatchMode) -> Self {
        match match_mode {
            NameMatchMode::Exact => Self::ExactName,
            NameMatchMode::Contained => Self::ContainedName,
        }
    }
}

#[derive(Args, Debug)]
pub struct SessionWidgetQuery {
    #[arg(long = "role", value_enum)]
    widget_role: WidgetRoleName,
    #[arg(long = "name")]
    accessible_name: String,
    #[arg(long = "name-match", value_enum, default_value_t)]
    name_match: NameMatchMode,
    #[arg(long = "inside-role", value_enum, requires = "anchor_name")]
    anchor_role: Option<WidgetRoleName>,
    #[arg(long = "inside-name", requires = "anchor_role")]
    anchor_name: Option<String>,
    #[arg(long = "enclosing-role", value_enum, requires = "anchor_role")]
    enclosing_role: Option<WidgetRoleName>,
    #[arg(long = "timeout-seconds", default_value_t = 30)]
    timeout_seconds: u64,
    #[arg(long = "screen-width", default_value_t = 1280)]
    screen_width: u32,
    #[arg(long = "screen-height", default_value_t = 800)]
    screen_height: u32,
}

impl From<&SessionWidgetQuery> for WidgetTarget {
    fn from(widget_query: &SessionWidgetQuery) -> Self {
        let searched_widget = Self::new(
            widget_query.widget_role.into(),
            widget_query.accessible_name.clone(),
        )
        .matching_names(widget_query.name_match.into());
        let anchor_widget = widget_query
            .anchor_role
            .zip(widget_query.anchor_name.clone())
            .map(|(anchor_role, anchor_name)| Self::new(anchor_role.into(), anchor_name));
        match (anchor_widget, widget_query.enclosing_role) {
            (Some(anchor_widget), Some(enclosing_role)) => {
                searched_widget.inside_enclosing(enclosing_role.into(), anchor_widget)
            }
            (Some(anchor_widget), None) => searched_widget.inside(anchor_widget),
            (None, _unanchored_role) => searched_widget,
        }
    }
}

impl SessionWidgetQuery {
    fn lookup_deadline(&self) -> Instant {
        Instant::now() + Duration::from_secs(self.timeout_seconds)
    }

    const fn screen_size(&self) -> ScreenSize {
        ScreenSize {
            logical_width: self.screen_width,
            logical_height: self.screen_height,
        }
    }
}

#[derive(Args, Debug)]
pub struct SessionTextEntry {
    #[command(flatten)]
    widget_query: SessionWidgetQuery,
    #[arg(long = "text")]
    replacement_text: String,
}

fn session_accessibility_address() -> Result<String, TaskFailure> {
    let address_lookup = || -> zbus::Result<String> {
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
    };
    address_lookup().map_err(TaskFailure::AccessibilityBusUnreachable)
}

fn session_wayland_socket() -> Result<PathBuf, TaskFailure> {
    Ok(PathBuf::from(required_variable("XDG_RUNTIME_DIR")?).join(
        env::var("WAYLAND_DISPLAY").unwrap_or_else(|_unset| DEFAULT_WAYLAND_DISPLAY.to_owned()),
    ))
}

fn screen_position_of(located_widget: &LocatedWidget) -> ScreenPosition {
    let (center_horizontal, center_vertical) = located_widget.widget_bounds.center_point();
    ScreenPosition {
        logical_x: center_horizontal.max(0).unsigned_abs(),
        logical_y: center_vertical.max(0).unsigned_abs(),
    }
}

fn lookup_failed(lookup_failure: WidgetLookupFailure) -> TaskFailure {
    TaskFailure::SceneCaptureFailed(lookup_failure.into())
}

struct LiveSessionInput {
    async_runtime: tokio::runtime::Runtime,
    accessible_tree: AccessibleTree,
    screen_size: ScreenSize,
}

impl LiveSessionInput {
    fn connect(screen_size: ScreenSize) -> Result<Self, TaskFailure> {
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
            screen_size,
        })
    }

    fn try_find(&self, widget_target: &WidgetTarget) -> Result<LocatedWidget, WidgetLookupFailure> {
        self.async_runtime
            .block_on(self.accessible_tree.find_widget(widget_target))
    }

    fn wait_for_widget(
        &self,
        widget_target: &WidgetTarget,
        lookup_deadline: Instant,
    ) -> Result<LocatedWidget, TaskFailure> {
        loop {
            match self.try_find(widget_target) {
                Ok(located_widget) => return Ok(located_widget),
                Err(WidgetLookupFailure::WidgetMissing { .. })
                    if Instant::now() < lookup_deadline =>
                {
                    thread::sleep(LOOKUP_RETRY_PAUSE);
                }
                Err(lookup_failure) => return Err(lookup_failed(lookup_failure)),
            }
        }
    }

    fn wait_for_sensitive_widget(
        &self,
        widget_target: &WidgetTarget,
        lookup_deadline: Instant,
    ) -> Result<LocatedWidget, TaskFailure> {
        loop {
            let located_widget = self.wait_for_widget(widget_target, lookup_deadline)?;
            let widget_sensitivity = self
                .async_runtime
                .block_on(self.accessible_tree.widget_sensitivity(&located_widget))
                .map_err(lookup_failed)?;
            match widget_sensitivity {
                WidgetSensitivity::AcceptsInput => return Ok(located_widget),
                WidgetSensitivity::RefusesInput if Instant::now() < lookup_deadline => {
                    thread::sleep(LOOKUP_RETRY_PAUSE);
                }
                WidgetSensitivity::RefusesInput => {
                    return Err(TaskFailure::WidgetStayedInsensitive {
                        accessible_name: widget_target.accessible_name.clone(),
                    });
                }
            }
        }
    }

    fn wait_until_gone(
        &self,
        widget_target: &WidgetTarget,
        lookup_deadline: Instant,
    ) -> Result<(), TaskFailure> {
        loop {
            match self.try_find(widget_target) {
                Err(WidgetLookupFailure::WidgetMissing { .. }) => return Ok(()),
                Err(lookup_failure) => return Err(lookup_failed(lookup_failure)),
                Ok(_still_shown) if Instant::now() < lookup_deadline => {
                    thread::sleep(LOOKUP_RETRY_PAUSE);
                }
                Ok(_still_shown) => {
                    return Err(TaskFailure::WidgetStayedShown {
                        accessible_name: widget_target.accessible_name.clone(),
                    });
                }
            }
        }
    }

    fn is_inside_screen(&self, located_widget: &LocatedWidget) -> bool {
        let screen_height = i32::try_from(self.screen_size.logical_height).unwrap_or(i32::MAX);
        let (_center_horizontal, center_vertical) = located_widget.widget_bounds.center_point();
        (VISIBLE_EDGE_MARGIN..screen_height - VISIBLE_EDGE_MARGIN).contains(&center_vertical)
    }

    fn scroll_into_view(
        &self,
        pointer_input: &mut VirtualPointerInput,
        widget_target: &WidgetTarget,
        lookup_deadline: Instant,
    ) -> Result<LocatedWidget, TaskFailure> {
        let mut located_widget = self.wait_for_sensitive_widget(widget_target, lookup_deadline)?;
        let screen_center = ScreenPosition {
            logical_x: self.screen_size.logical_width / 2,
            logical_y: self.screen_size.logical_height / 2,
        };
        for _wheel_click in 0..SCROLL_WHEEL_CLICK_LIMIT {
            if self.is_inside_screen(&located_widget) {
                break;
            }
            pointer_input.scroll_wheel_down(screen_center)?;
            located_widget = self.wait_for_widget(widget_target, lookup_deadline)?;
        }
        Ok(located_widget)
    }

    fn press_widget(
        &self,
        widget_target: &WidgetTarget,
        lookup_deadline: Instant,
    ) -> Result<(), TaskFailure> {
        let mut pointer_input =
            VirtualPointerInput::connect(&session_wayland_socket()?, self.screen_size)?;
        let located_widget =
            self.scroll_into_view(&mut pointer_input, widget_target, lookup_deadline)?;
        pointer_input.move_to(screen_position_of(&located_widget))?;
        match self.async_runtime.block_on(
            self.accessible_tree
                .activate_widget(&located_widget, widget_target),
        ) {
            Ok(()) => Ok(()),
            Err(WidgetLookupFailure::ActionMissing { .. }) => {
                Ok(pointer_input.click_at(screen_position_of(&located_widget))?)
            }
            Err(activation_failure) => Err(lookup_failed(activation_failure)),
        }
    }

    fn replace_text(
        &self,
        widget_target: &WidgetTarget,
        replacement_text: &str,
        lookup_deadline: Instant,
    ) -> Result<(), TaskFailure> {
        let located_widget = self.wait_for_sensitive_widget(widget_target, lookup_deadline)?;
        self.async_runtime
            .block_on(self.accessible_tree.replace_widget_text(
                &located_widget,
                widget_target,
                replacement_text,
            ))
            .map_err(lookup_failed)
    }
}

pub fn press_session_widget(widget_query: &SessionWidgetQuery) -> Result<(), TaskFailure> {
    LiveSessionInput::connect(widget_query.screen_size())?.press_widget(
        &WidgetTarget::from(widget_query),
        widget_query.lookup_deadline(),
    )
}

pub fn await_session_widget(widget_query: &SessionWidgetQuery) -> Result<(), TaskFailure> {
    LiveSessionInput::connect(widget_query.screen_size())?
        .wait_for_widget(
            &WidgetTarget::from(widget_query),
            widget_query.lookup_deadline(),
        )
        .map(|_located_widget| ())
}

pub fn await_session_widget_gone(widget_query: &SessionWidgetQuery) -> Result<(), TaskFailure> {
    LiveSessionInput::connect(widget_query.screen_size())?.wait_until_gone(
        &WidgetTarget::from(widget_query),
        widget_query.lookup_deadline(),
    )
}

pub fn type_into_session_widget(text_entry: &SessionTextEntry) -> Result<(), TaskFailure> {
    let widget_query = &text_entry.widget_query;
    LiveSessionInput::connect(widget_query.screen_size())?.replace_text(
        &WidgetTarget::from(widget_query),
        &text_entry.replacement_text,
        widget_query.lookup_deadline(),
    )
}

pub fn describe_session_widgets() -> Result<Vec<String>, TaskFailure> {
    let accessibility_address = session_accessibility_address()?;
    let async_runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    async_runtime
        .block_on(async {
            AccessibleTree::connect_to(&accessibility_address)
                .await?
                .limited_to(ApplicationScope::ApplicationNamed(
                    TESTED_APPLICATION_NAME.to_owned(),
                ))
                .describe_visible_widgets()
                .await
        })
        .map_err(lookup_failed)
}

pub fn describe_widget_ancestry(name_fragment: &str) -> Result<Vec<String>, TaskFailure> {
    let accessibility_address = session_accessibility_address()?;
    let async_runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    async_runtime
        .block_on(async {
            AccessibleTree::connect_to(&accessibility_address)
                .await?
                .limited_to(ApplicationScope::ApplicationNamed(
                    TESTED_APPLICATION_NAME.to_owned(),
                ))
                .describe_ancestry_of_named(name_fragment)
                .await
        })
        .map_err(lookup_failed)
}
