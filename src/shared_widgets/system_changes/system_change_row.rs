use relm4::{FactorySender, adw, adw::prelude::*, gtk, prelude::FactoryComponent};

use crate::{
    host_services::{
        clamav_admin_service::admin_audit_records::{AdminAuditEntry, Requester},
        desktop::locale_formatting::MomentFormatters,
    },
    translated_messages,
};

pub struct SystemChangeRow {
    pub moment_formatters: &'static MomentFormatters,
    audit_entry: AdminAuditEntry,
}

impl SystemChangeRow {
    fn row_subtitle(&self) -> String {
        let moment_formatters = self.moment_formatters;
        let recorded_time_text = moment_formatters.exact_moment_text(self.audit_entry.recorded_at);
        let outcome_text = self.audit_entry.audited_outcome.described();
        match self.audit_entry.requester {
            Requester::KnownUser { user_id } => translated_messages::widgets::known_user(
                &recorded_time_text,
                &outcome_text,
                user_id,
            ),
            Requester::UnknownUser => {
                translated_messages::widgets::unknown_user(&recorded_time_text, &outcome_text)
            }
        }
    }
}

pub struct SystemChangeRowInit {
    pub audit_entry: AdminAuditEntry,
    pub moment_formatters: &'static MomentFormatters,
}

#[relm4::factory(pub)]
impl FactoryComponent for SystemChangeRow {
    type Init = SystemChangeRowInit;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        adw::ActionRow {
            set_use_markup: false,
            set_title: &self.audit_entry.action_subject,
            set_title_lines: 0,
            set_subtitle: &self.row_subtitle(),

            add_prefix = &gtk::Image {
                set_icon_name: Some("dialog-warning-symbolic"),
                set_tooltip_text: Some(&translated_messages::widgets::change_lowered_protection_tooltip()),
                add_css_class: "warning",
                set_visible: self.audit_entry.lowered_protection(),
            },
        }
    }

    fn init_model(
        SystemChangeRowInit {
            audit_entry,
            moment_formatters,
        }: Self::Init,
        _row_index: &Self::Index,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            audit_entry,
            moment_formatters,
        }
    }
}
