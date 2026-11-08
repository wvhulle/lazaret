use std::collections::VecDeque;

use atspi::{
    AccessibilityConnection, CoordType, ObjectRefOwned, Role, State, StateSet,
    proxy::{
        accessible::{AccessibleProxy, ObjectRefExt},
        action::ActionProxy,
        component::ComponentProxy,
        editable_text::EditableTextProxy,
    },
    zbus,
};
use futures::future::join_all;

const SEARCHED_NODE_LIMIT: usize = 20_000;
const ANCESTOR_STEP_LIMIT: usize = 32;

#[derive(Debug, thiserror::Error)]
pub enum WidgetLookupFailure {
    #[error("the accessibility bus could not be reached: {0}")]
    BusUnreachable(#[from] atspi::AtspiError),
    #[error("an accessibility call failed: {0}")]
    CallFailed(#[from] zbus::Error),
    #[error("no visible {widget_role:?} named {accessible_name:?} was found")]
    WidgetMissing {
        widget_role: Role,
        accessible_name: String,
    },
    #[error("{accessible_name:?} offers no action to activate")]
    ActionMissing { accessible_name: String },
    #[error("{accessible_name:?} refused the new text")]
    TextRefused { accessible_name: String },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NameMatch {
    #[default]
    ExactName,
    ContainedName,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WidgetSensitivity {
    AcceptsInput,
    RefusesInput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SearchScope {
    WholeScreen,
    InsideWidget(Box<WidgetTarget>),
    InsideEnclosingWidget {
        enclosing_role: Role,
        anchor_target: Box<WidgetTarget>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WidgetTarget {
    pub widget_role: Role,
    pub accessible_name: String,
    pub name_match: NameMatch,
    pub search_scope: SearchScope,
}

pub use atspi::Role as WidgetRole;

impl WidgetTarget {
    pub fn new(widget_role: Role, accessible_name: impl Into<String>) -> Self {
        Self {
            widget_role,
            accessible_name: accessible_name.into(),
            name_match: NameMatch::ExactName,
            search_scope: SearchScope::WholeScreen,
        }
    }

    pub fn matching_names(self, name_match: NameMatch) -> Self {
        Self { name_match, ..self }
    }

    pub fn inside(self, container_target: Self) -> Self {
        Self {
            search_scope: SearchScope::InsideWidget(Box::new(container_target)),
            ..self
        }
    }

    pub fn inside_enclosing(self, enclosing_role: Role, anchor_target: Self) -> Self {
        Self {
            search_scope: SearchScope::InsideEnclosingWidget {
                enclosing_role,
                anchor_target: Box::new(anchor_target),
            },
            ..self
        }
    }

    pub fn dialog(accessible_name: impl Into<String>) -> Self {
        Self::new(Role::Dialog, accessible_name)
    }

    pub fn alert(accessible_name: impl Into<String>) -> Self {
        Self::new(Role::Alert, accessible_name)
    }

    pub fn button(accessible_name: impl Into<String>) -> Self {
        Self::new(Role::Button, accessible_name)
    }

    pub fn toggle_button(accessible_name: impl Into<String>) -> Self {
        Self::new(Role::ToggleButton, accessible_name)
    }

    pub fn list_item(accessible_name: impl Into<String>) -> Self {
        Self::new(Role::ListItem, accessible_name)
    }

    pub fn label(accessible_name: impl Into<String>) -> Self {
        Self::new(Role::Label, accessible_name)
    }

    pub fn page_tab(accessible_name: impl Into<String>) -> Self {
        Self::new(Role::PageTab, accessible_name)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WidgetBounds {
    pub left_edge: i32,
    pub top_edge: i32,
    pub widget_width: i32,
    pub widget_height: i32,
}

impl WidgetBounds {
    pub const fn center_point(self) -> (i32, i32) {
        (
            self.left_edge + self.widget_width / 2,
            self.top_edge + self.widget_height / 2,
        )
    }
}

pub struct LocatedWidget {
    object_reference: ObjectRefOwned,
    pub widget_bounds: WidgetBounds,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum ApplicationScope {
    #[default]
    EveryApplication,
    ApplicationNamed(String),
}

pub struct AccessibleTree {
    accessibility_connection: AccessibilityConnection,
    application_scope: ApplicationScope,
}

async fn proxy_for<'reference>(
    connection: &'reference zbus::Connection,
    object_reference: &'reference ObjectRefOwned,
) -> Result<AccessibleProxy<'reference>, WidgetLookupFailure> {
    Ok(object_reference.as_accessible_proxy(connection).await?)
}

struct InspectedNode {
    object_reference: ObjectRefOwned,
    widget_role: Role,
    accessible_name: String,
    widget_state: StateSet,
    child_references: Vec<ObjectRefOwned>,
}

impl InspectedNode {
    fn is_showing(&self) -> bool {
        self.widget_state.contains(State::Showing)
    }

    fn matches(&self, widget_target: &WidgetTarget) -> bool {
        let name_matches = match widget_target.name_match {
            NameMatch::ExactName => self.accessible_name == widget_target.accessible_name,
            NameMatch::ContainedName => self
                .accessible_name
                .contains(&widget_target.accessible_name),
        };
        self.widget_role == widget_target.widget_role && name_matches && self.is_showing()
    }

    fn may_hold_showing_widgets(&self) -> bool {
        self.widget_role == Role::Application || self.is_showing()
    }
}

async fn inspect_node(
    bus_connection: &zbus::Connection,
    object_reference: ObjectRefOwned,
) -> Option<InspectedNode> {
    if object_reference.is_null() {
        return None;
    }
    let accessible_proxy = proxy_for(bus_connection, &object_reference).await.ok()?;
    let (widget_role, accessible_name, widget_state, child_references) = tokio::join!(
        accessible_proxy.get_role(),
        accessible_proxy.name(),
        accessible_proxy.get_state(),
        accessible_proxy.get_children(),
    );
    Some(InspectedNode {
        widget_role: widget_role.ok()?,
        accessible_name: accessible_name.unwrap_or_default(),
        widget_state: widget_state.unwrap_or_default(),
        child_references: child_references.unwrap_or_default(),
        object_reference,
    })
}

async fn inspect_level(
    bus_connection: &zbus::Connection,
    level_references: Vec<ObjectRefOwned>,
) -> Vec<InspectedNode> {
    join_all(
        level_references
            .into_iter()
            .map(|object_reference| inspect_node(bus_connection, object_reference)),
    )
    .await
    .into_iter()
    .flatten()
    .collect()
}

impl AccessibleTree {
    pub async fn connect_to(accessibility_bus_address: &str) -> Result<Self, WidgetLookupFailure> {
        Ok(Self {
            accessibility_connection: AccessibilityConnection::from_address(
                accessibility_bus_address.parse()?,
            )
            .await?,
            application_scope: ApplicationScope::EveryApplication,
        })
    }

    fn bus_connection(&self) -> &zbus::Connection {
        self.accessibility_connection.connection()
    }

    pub fn limited_to(self, application_scope: ApplicationScope) -> Self {
        Self {
            application_scope,
            ..self
        }
    }

    async fn top_level_nodes(&self) -> Result<Vec<ObjectRefOwned>, WidgetLookupFailure> {
        let application_nodes = self
            .accessibility_connection
            .root_accessible_on_registry()
            .await?
            .get_children()
            .await?;
        let ApplicationScope::ApplicationNamed(application_name) = &self.application_scope else {
            return Ok(application_nodes);
        };
        let mut scoped_nodes = Vec::new();
        for application_node in application_nodes {
            let Ok(application_proxy) = proxy_for(self.bus_connection(), &application_node).await
            else {
                continue;
            };
            if application_proxy
                .name()
                .await
                .is_ok_and(|shown_name| shown_name.eq_ignore_ascii_case(application_name))
            {
                scoped_nodes.push(application_node);
            }
        }
        Ok(scoped_nodes)
    }

    async fn search_from(
        &self,
        starting_nodes: Vec<ObjectRefOwned>,
        widget_target: &WidgetTarget,
    ) -> Option<ObjectRefOwned> {
        let mut level_references = starting_nodes;
        let mut searched_nodes = 0;
        while !level_references.is_empty() && searched_nodes <= SEARCHED_NODE_LIMIT {
            searched_nodes += level_references.len();
            let mut next_level_references = Vec::new();
            for inspected_node in inspect_level(self.bus_connection(), level_references).await {
                if inspected_node.matches(widget_target) {
                    return Some(inspected_node.object_reference);
                }
                if inspected_node.may_hold_showing_widgets() {
                    next_level_references.extend(inspected_node.child_references);
                }
            }
            level_references = next_level_references;
        }
        None
    }

    async fn scope_nodes(
        &self,
        search_scope: &SearchScope,
    ) -> Result<Vec<ObjectRefOwned>, WidgetLookupFailure> {
        match search_scope {
            SearchScope::WholeScreen => self.top_level_nodes().await,
            SearchScope::InsideWidget(container_target) => {
                let container_reference = Box::pin(self.locate_reference(container_target)).await?;
                Ok(proxy_for(self.bus_connection(), &container_reference)
                    .await?
                    .get_children()
                    .await?)
            }
            SearchScope::InsideEnclosingWidget {
                enclosing_role,
                anchor_target,
            } => {
                let anchor_reference = Box::pin(self.locate_reference(anchor_target)).await?;
                let enclosing_reference = self
                    .enclosing_reference(anchor_reference, *enclosing_role)
                    .await
                    .ok_or_else(|| WidgetLookupFailure::WidgetMissing {
                        widget_role: *enclosing_role,
                        accessible_name: anchor_target.accessible_name.clone(),
                    })?;
                Ok(proxy_for(self.bus_connection(), &enclosing_reference)
                    .await?
                    .get_children()
                    .await?)
            }
        }
    }

    async fn enclosing_reference(
        &self,
        anchor_reference: ObjectRefOwned,
        enclosing_role: Role,
    ) -> Option<ObjectRefOwned> {
        let mut current_reference = anchor_reference;
        for _ancestor_step in 0..ANCESTOR_STEP_LIMIT {
            let parent_reference = proxy_for(self.bus_connection(), &current_reference)
                .await
                .ok()?
                .parent()
                .await
                .ok()?;
            if parent_reference.is_null() {
                return None;
            }
            let parent_role = proxy_for(self.bus_connection(), &parent_reference)
                .await
                .ok()?
                .get_role()
                .await
                .ok()?;
            if parent_role == enclosing_role {
                return Some(parent_reference);
            }
            current_reference = parent_reference;
        }
        None
    }

    async fn locate_reference(
        &self,
        widget_target: &WidgetTarget,
    ) -> Result<ObjectRefOwned, WidgetLookupFailure> {
        let starting_nodes = self.scope_nodes(&widget_target.search_scope).await?;
        self.search_from(starting_nodes, widget_target)
            .await
            .ok_or_else(|| WidgetLookupFailure::WidgetMissing {
                widget_role: widget_target.widget_role,
                accessible_name: widget_target.accessible_name.clone(),
            })
    }

    pub async fn find_widget(
        &self,
        widget_target: &WidgetTarget,
    ) -> Result<LocatedWidget, WidgetLookupFailure> {
        let object_reference = self.locate_reference(widget_target).await?;
        let widget_bounds = self.widget_bounds(&object_reference).await?;
        Ok(LocatedWidget {
            object_reference,
            widget_bounds,
        })
    }

    pub async fn describe_visible_widgets(&self) -> Result<Vec<String>, WidgetLookupFailure> {
        let mut level_references = self.top_level_nodes().await?;
        let mut widget_descriptions = Vec::new();
        let mut searched_nodes = 0;
        while !level_references.is_empty() && searched_nodes <= SEARCHED_NODE_LIMIT {
            searched_nodes += level_references.len();
            let mut next_level_references = Vec::new();
            for inspected_node in inspect_level(self.bus_connection(), level_references).await {
                if inspected_node.is_showing() {
                    widget_descriptions.push(format!(
                        "{:?} {:?}",
                        inspected_node.widget_role, inspected_node.accessible_name
                    ));
                }
                if inspected_node.may_hold_showing_widgets() {
                    next_level_references.extend(inspected_node.child_references);
                }
            }
            level_references = next_level_references;
        }
        Ok(widget_descriptions)
    }

    pub async fn describe_ancestry_of_named(
        &self,
        name_fragment: &str,
    ) -> Result<Vec<String>, WidgetLookupFailure> {
        #[derive(Clone, Copy)]
        enum ParentLink {
            TopLevel,
            ChildOf(usize),
        }
        struct VisitedNode {
            parent_link: ParentLink,
            node_description: String,
        }
        let mut visited_nodes: Vec<VisitedNode> = Vec::new();
        let mut pending_nodes = self
            .top_level_nodes()
            .await?
            .into_iter()
            .map(|object_reference| (ParentLink::TopLevel, object_reference))
            .collect::<VecDeque<_>>();
        let mut ancestry_descriptions = Vec::new();
        while let Some((parent_link, object_reference)) = pending_nodes.pop_front() {
            if visited_nodes.len() > SEARCHED_NODE_LIMIT || object_reference.is_null() {
                continue;
            }
            let Ok(accessible_proxy) = proxy_for(self.bus_connection(), &object_reference).await
            else {
                continue;
            };
            let accessible_name = accessible_proxy.name().await.unwrap_or_default();
            let widget_role = accessible_proxy.get_role().await.ok();
            let showing_marker = if accessible_proxy
                .get_state()
                .await
                .is_ok_and(|widget_state| widget_state.contains(State::Showing))
            {
                "showing"
            } else {
                "hidden"
            };
            let node_position = visited_nodes.len();
            visited_nodes.push(VisitedNode {
                parent_link,
                node_description: format!("{widget_role:?} {accessible_name:?} {showing_marker}"),
            });
            if accessible_name.contains(name_fragment) {
                let mut ancestry = Vec::new();
                let mut current_link = ParentLink::ChildOf(node_position);
                while let ParentLink::ChildOf(position) = current_link {
                    ancestry.push(visited_nodes[position].node_description.clone());
                    current_link = visited_nodes[position].parent_link;
                }
                ancestry.reverse();
                ancestry_descriptions.push(ancestry.join(" > "));
            }
            if let Ok(child_references) = accessible_proxy.get_children().await {
                pending_nodes.extend(
                    child_references.into_iter().map(|child_reference| {
                        (ParentLink::ChildOf(node_position), child_reference)
                    }),
                );
            }
        }
        Ok(ancestry_descriptions)
    }

    async fn widget_bounds(
        &self,
        object_reference: &ObjectRefOwned,
    ) -> Result<WidgetBounds, WidgetLookupFailure> {
        let component_proxy = ComponentProxy::builder(self.bus_connection())
            .destination(
                object_reference
                    .name()
                    .cloned()
                    .ok_or(atspi::AtspiError::MissingName)?,
            )?
            .path(object_reference.path().clone())?
            .cache_properties(zbus::proxy::CacheProperties::No)
            .build()
            .await?;
        let (left_edge, top_edge, widget_width, widget_height) =
            component_proxy.get_extents(CoordType::Window).await?;
        Ok(WidgetBounds {
            left_edge,
            top_edge,
            widget_width,
            widget_height,
        })
    }

    pub async fn activate_widget(
        &self,
        located_widget: &LocatedWidget,
        widget_target: &WidgetTarget,
    ) -> Result<(), WidgetLookupFailure> {
        let object_reference = &located_widget.object_reference;
        let action_proxy = ActionProxy::builder(self.bus_connection())
            .destination(
                object_reference
                    .name()
                    .cloned()
                    .ok_or(atspi::AtspiError::MissingName)?,
            )?
            .path(object_reference.path().clone())?
            .cache_properties(zbus::proxy::CacheProperties::No)
            .build()
            .await?;
        if action_proxy.n_actions().await.unwrap_or(0) == 0 || !action_proxy.do_action(0).await? {
            return Err(WidgetLookupFailure::ActionMissing {
                accessible_name: widget_target.accessible_name.clone(),
            });
        }
        Ok(())
    }

    pub async fn widget_sensitivity(
        &self,
        located_widget: &LocatedWidget,
    ) -> Result<WidgetSensitivity, WidgetLookupFailure> {
        let widget_state = proxy_for(self.bus_connection(), &located_widget.object_reference)
            .await?
            .get_state()
            .await?;
        Ok(if widget_state.contains(State::Sensitive) {
            WidgetSensitivity::AcceptsInput
        } else {
            WidgetSensitivity::RefusesInput
        })
    }

    pub async fn replace_widget_text(
        &self,
        located_widget: &LocatedWidget,
        widget_target: &WidgetTarget,
        replacement_text: &str,
    ) -> Result<(), WidgetLookupFailure> {
        let object_reference = &located_widget.object_reference;
        let editable_text_proxy = EditableTextProxy::builder(self.bus_connection())
            .destination(
                object_reference
                    .name()
                    .cloned()
                    .ok_or(atspi::AtspiError::MissingName)?,
            )?
            .path(object_reference.path().clone())?
            .cache_properties(zbus::proxy::CacheProperties::No)
            .build()
            .await?;
        if editable_text_proxy
            .set_text_contents(replacement_text)
            .await?
        {
            Ok(())
        } else {
            Err(WidgetLookupFailure::TextRefused {
                accessible_name: widget_target.accessible_name.clone(),
            })
        }
    }
}
