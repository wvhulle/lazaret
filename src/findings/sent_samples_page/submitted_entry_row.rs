use std::rc::Rc;

use relm4::{FactorySender, adw, adw::prelude::*, gtk, prelude::FactoryComponent};

use crate::{
    findings::sent_samples_page::sent_samples_presentation::SubmittedRowView,
    host_services::activity_history::submission_history_store::RemoteEntryLink,
    translated_messages, translated_messages::MessageCatalog,
};

#[derive(Debug)]
pub enum SubmittedEntryRowOutput {
    RemoteEntryRequested(String),
}

pub struct SubmittedEntryRow {
    pub message_catalog: Rc<MessageCatalog>,
    row_view: SubmittedRowView,
}

impl SubmittedEntryRow {
    const fn has_remote_entry(&self) -> bool {
        matches!(
            self.row_view.entry_link,
            RemoteEntryLink::RemoteEntryAt { .. }
        )
    }
}

pub struct SubmittedEntryRowInit {
    pub row_view: SubmittedRowView,
    pub message_catalog: Rc<MessageCatalog>,
}

#[relm4::factory(pub)]
impl FactoryComponent for SubmittedEntryRow {
    type Init = SubmittedEntryRowInit;
    type Input = ();
    type Output = SubmittedEntryRowOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        adw::ActionRow {
            set_use_markup: false,
            set_title: &self.row_view.row_title,
            set_subtitle: &self.row_view.row_subtitle,
            set_subtitle_lines: 0,

            add_prefix = &gtk::Image {
                set_icon_name: Some("dialog-warning-symbolic"),
                add_css_class: "warning",
                set_visible: self.row_view.needs_attention,
            },

            add_suffix = &gtk::Button {
                set_icon_name: "adw-external-link-symbolic",
                set_tooltip_text: Some(&translated_messages::findings::open_remote_entry_tooltip(&self.message_catalog)),
                set_valign: gtk::Align::Center,
                add_css_class: "flat",
                set_visible: self.has_remote_entry(),
                connect_clicked[sender, entry_link = self.row_view.entry_link.clone()] => move |_clicked_button| {
                    if let RemoteEntryLink::RemoteEntryAt { entry_url } = &entry_link {
                        let _page_closed = sender.output(SubmittedEntryRowOutput::RemoteEntryRequested(entry_url.clone()));
                    }
                },
            },
        }
    }

    fn init_model(
        SubmittedEntryRowInit {
            row_view,
            message_catalog,
        }: Self::Init,
        _row_index: &Self::Index,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            row_view,
            message_catalog,
        }
    }
}
