use std::rc::Rc;

use relm4::{
    FactorySender, adw,
    adw::prelude::*,
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
};

use crate::{
    application_window::{
        application_pages::ApplicationPage,
        page_links::{SHOW_PAGE_ACTION, page_target},
        sidebar_navigation::sidebar_entry_presentation::{
            EntrySummary, SidebarRowPresentation, sidebar_row_presentation,
        },
    },
    translated_messages::MessageCatalog,
};

pub struct SidebarEntryRow {
    pub message_catalog: Rc<MessageCatalog>,
    pub application_page: ApplicationPage,
    pub entry_summary: EntrySummary,
}

impl SidebarEntryRow {
    fn row_presentation(&self) -> SidebarRowPresentation {
        let message_catalog = Rc::clone(&self.message_catalog);
        sidebar_row_presentation(&message_catalog, self.application_page, &self.entry_summary)
    }
}

pub struct SidebarEntryRowInit {
    pub application_page: ApplicationPage,
    pub message_catalog: Rc<MessageCatalog>,
}

#[relm4::factory(pub)]
impl FactoryComponent for SidebarEntryRow {
    type Init = SidebarEntryRowInit;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        adw::ActionRow {
            set_use_markup: false,
            set_activatable: true,
            set_selectable: true,
            set_action_name: Some(SHOW_PAGE_ACTION),
            set_action_target_value: Some(&page_target(self.application_page)),
            #[watch]
            set_title: &self.row_presentation().row_title,
            #[watch]
            set_subtitle: &self.row_presentation().row_subtitle,

            add_prefix = &gtk::Image {
                #[watch]
                set_icon_name: Some(self.row_presentation().prefix_icon),
            },

            add_suffix = &gtk::Image {
                set_valign: gtk::Align::Center,
                #[watch]
                set_visible: self.row_presentation().status_image.is_shown(),
                #[watch]
                set_icon_name: Some(self.row_presentation().status_image.icon_name()),
                #[watch]
                set_css_classes: self.row_presentation().status_image.css_classes(),
                #[watch]
                set_tooltip_text: Some(&self.row_presentation().status_image.tooltip_text(&self.message_catalog)),
            },
        }
    }

    fn init_model(
        SidebarEntryRowInit {
            application_page,
            message_catalog,
        }: Self::Init,
        _row_index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            message_catalog,
            application_page,
            entry_summary: EntrySummary::SummaryPending,
        }
    }
}
