use relm4::{
    Component, ComponentParts, ComponentSender, adw, adw::prelude::*, gtk, prelude::FactoryVecDeque,
};

use crate::{
    host_services::{
        clamav_admin_service::admin_audit_records::AdminAuditEntry,
        desktop::{
            failure_journal_log::log_failure_to_journal, locale_formatting::MomentFormatters,
        },
    },
    shared_widgets::{
        loading_indicators::{
            LOAD_FAILED_PAGE_NAME, empty_placeholder_row, loading_placeholder_stack,
        },
        system_changes::{
            system_change_row::{SystemChangeRow, SystemChangeRowInit},
            system_changes_state::{
                AuditTopic, SystemChangesReadFailure, SystemChangesView, audit_entries_for_topic,
                read_system_changes,
            },
        },
    },
    translated_messages,
};

pub struct SystemChangesGroupInit {
    pub moment_formatters: &'static MomentFormatters,
    pub audit_topic: AuditTopic,
    pub group_description: String,
    pub empty_title: String,
}

pub struct SystemChangesGroup {
    pub moment_formatters: &'static MomentFormatters,
    audit_topic: AuditTopic,
    system_changes_view: SystemChangesView,
    system_change_rows: FactoryVecDeque<SystemChangeRow>,
}

#[derive(Debug)]
pub enum SystemChangesGroupInput {
    ReloadRequested,
}

#[derive(Debug)]
pub enum SystemChangesGroupCommand {
    SystemChangesLoaded(Result<Vec<AdminAuditEntry>, SystemChangesReadFailure>),
}

pub fn system_changes_failure_text(system_changes_view: &SystemChangesView) -> String {
    match system_changes_view {
        SystemChangesView::SystemChangesFailed(read_failure) => {
            translated_messages::widgets::system_changes_failed(read_failure)
        }
        SystemChangesView::SystemChangesLoading | SystemChangesView::SystemChangesLoaded => {
            String::new()
        }
    }
}

impl SystemChangesGroup {
    fn reload_system_changes(sender: &ComponentSender<Self>) {
        sender.oneshot_command(async {
            SystemChangesGroupCommand::SystemChangesLoaded(read_system_changes().await)
        });
    }

    fn replace_rows(&mut self, audit_entries: Vec<AdminAuditEntry>) {
        let moment_formatters = self.moment_formatters;
        let mut row_guard = self.system_change_rows.guard();
        row_guard.clear();
        audit_entries.into_iter().for_each(|audit_entry| {
            row_guard.push_back(SystemChangeRowInit {
                audit_entry,
                moment_formatters,
            });
        });
    }
}

#[relm4::component(pub)]
impl Component for SystemChangesGroup {
    type Init = SystemChangesGroupInit;
    type Input = SystemChangesGroupInput;
    type Output = ();
    type CommandOutput = SystemChangesGroupCommand;

    view! {
        adw::PreferencesGroup {
            set_title: &translated_messages::widgets::system_changes_title(),
            set_description: Some(&group_init.group_description),
            #[wrap(Some)]
            set_header_suffix = &gtk::Button {
                set_icon_name: "view-refresh-symbolic",
                set_tooltip_text: Some(&translated_messages::widgets::refresh_tooltip()),
                set_valign: gtk::Align::Center,
                add_css_class: "flat",
                connect_clicked => SystemChangesGroupInput::ReloadRequested,
            },

            #[local_ref]
            system_change_list -> gtk::ListBox {
                add_css_class: "boxed-list",
                set_selection_mode: gtk::SelectionMode::None,
                #[wrap(Some)]
                #[local_ref]
                set_placeholder = system_change_placeholder -> gtk::Stack {
                    add_named[Some(LOAD_FAILED_PAGE_NAME)] = &adw::ActionRow {
                        set_title: &translated_messages::widgets::system_changes_cannot_title(),
                        set_use_markup: false,
                        set_subtitle_lines: 0,
                        add_css_class: "dim-label",
                        #[watch]
                        set_subtitle: &system_changes_failure_text(&model.system_changes_view),
                    },
                    #[watch]
                    set_visible_child_name: model.system_changes_view.placeholder_page_name(),
                },
            },
        }
    }

    fn init(
        group_init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let moment_formatters = group_init.moment_formatters;
        let model = Self {
            moment_formatters,
            audit_topic: group_init.audit_topic,
            system_changes_view: SystemChangesView::SystemChangesLoading,
            system_change_rows: FactoryVecDeque::builder()
                .launch(gtk::ListBox::default())
                .detach(),
        };
        let system_change_list = model.system_change_rows.widget();
        let system_change_placeholder = &loading_placeholder_stack(
            &translated_messages::widgets::loading_system_changes(),
            &empty_placeholder_row(&group_init.empty_title),
        );
        let widgets = view_output!();
        Self::reload_system_changes(&sender);
        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        group_input: SystemChangesGroupInput,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match group_input {
            SystemChangesGroupInput::ReloadRequested => Self::reload_system_changes(&sender),
        }
    }

    fn update_cmd(
        &mut self,
        group_command: SystemChangesGroupCommand,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match group_command {
            SystemChangesGroupCommand::SystemChangesLoaded(Ok(audit_entries)) => {
                self.system_changes_view = SystemChangesView::SystemChangesLoaded;
                let grouped_entries = audit_entries_for_topic(audit_entries, self.audit_topic);
                self.replace_rows(grouped_entries);
            }
            SystemChangesGroupCommand::SystemChangesLoaded(Err(read_failure)) => {
                log_failure_to_journal("reading the system changes failed", &read_failure);
                self.system_changes_view = SystemChangesView::SystemChangesFailed(read_failure);
                self.replace_rows(Vec::new());
            }
        }
    }
}
