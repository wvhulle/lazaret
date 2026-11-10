use std::rc::Rc;

use relm4::{FactorySender, adw, adw::prelude::*, gtk, prelude::FactoryComponent};

use crate::{
    host_services::clamav_scanner::scanner_output_parser::sanitize_scanner_line,
    translated_messages, translated_messages::MessageCatalog,
};

pub struct ProfileValueRow {
    pub message_catalog: Rc<MessageCatalog>,
    pub listed_value: String,
}

#[derive(Debug)]
pub enum ProfileValueRowOutput {
    RemovalRequested(String),
}

pub struct ProfileValueRowInit {
    pub listed_value: String,
    pub message_catalog: Rc<MessageCatalog>,
}

#[relm4::factory(pub)]
impl FactoryComponent for ProfileValueRow {
    type Init = ProfileValueRowInit;
    type Input = ();
    type Output = ProfileValueRowOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        adw::ActionRow {
            set_use_markup: false,
            set_title: &sanitize_scanner_line(&self.listed_value),

            add_suffix = &gtk::Button {
                set_icon_name: "user-trash-symbolic",
                set_tooltip_text: Some(&translated_messages::scans::remove_profile_tooltip(&self.message_catalog)),
                set_valign: gtk::Align::Center,
                add_css_class: "flat",
                connect_clicked[sender, removed_value = self.listed_value.clone()] => move |_clicked_button| {
                    let _editor_closed = sender.output(ProfileValueRowOutput::RemovalRequested(removed_value.clone()));
                },
            },
        }
    }

    fn init_model(
        ProfileValueRowInit {
            listed_value,
            message_catalog,
        }: Self::Init,
        _row_index: &Self::Index,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            listed_value,
            message_catalog,
        }
    }
}
