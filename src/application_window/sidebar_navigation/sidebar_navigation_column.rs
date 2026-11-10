use std::rc::Rc;

use relm4::{
    Component, ComponentParts, ComponentSender, adw, adw::prelude::*, gtk, gtk::gio,
    prelude::FactoryVecDeque,
};

use crate::{
    application_window::{
        application_pages::ApplicationPage,
        history_menu_actions::primary_menu_entries,
        sidebar_area_status::SidebarAreaSummary,
        sidebar_navigation::{
            sidebar_entry_presentation::{EntrySummary, SidebarGroup},
            sidebar_entry_row::{SidebarEntryRow, SidebarEntryRowInit},
        },
    },
    shared_widgets::named_menu_items::NamedMenuItems,
    translated_messages,
    translated_messages::MessageCatalog,
};

pub struct SidebarNavigationColumn {
    pub message_catalog: Rc<MessageCatalog>,
    selected_page: ApplicationPage,
    signatures_rows: FactoryVecDeque<SidebarEntryRow>,
    scans_rows: FactoryVecDeque<SidebarEntryRow>,
    findings_rows: FactoryVecDeque<SidebarEntryRow>,
}

#[derive(Debug)]
pub enum SidebarNavigationColumnInput {
    SelectedPageChanged(ApplicationPage),
    AreaSummariesChanged(Vec<SidebarAreaSummary>),
}

fn primary_menu(message_catalog: &MessageCatalog) -> gio::Menu {
    let primary_menu = gio::Menu::new();
    primary_menu_entries(message_catalog)
        .into_iter()
        .for_each(|(entry_label, action_name)| {
            primary_menu.append(Some(&entry_label), Some(action_name));
        });
    primary_menu
}

fn launched_group_rows(
    message_catalog: &Rc<MessageCatalog>,
    sidebar_group: SidebarGroup,
) -> FactoryVecDeque<SidebarEntryRow> {
    let mut group_rows = FactoryVecDeque::builder()
        .launch(gtk::ListBox::default())
        .detach();
    sidebar_group
        .group_rows()
        .into_iter()
        .for_each(|application_page| {
            group_rows.guard().push_back(SidebarEntryRowInit {
                application_page,
                message_catalog: Rc::clone(message_catalog),
            });
        });
    group_rows
}

fn selected_group_row(
    group_list: &gtk::ListBox,
    sidebar_group: SidebarGroup,
    selected_page: ApplicationPage,
) -> Option<gtk::ListBoxRow> {
    sidebar_group
        .row_position_of(selected_page)
        .and_then(|row_position| group_list.row_at_index(row_position))
}

fn show_area_summaries(
    group_rows: &mut FactoryVecDeque<SidebarEntryRow>,
    area_summaries: &[SidebarAreaSummary],
) {
    group_rows.guard().iter_mut().for_each(|entry_row| {
        entry_row.entry_summary =
            EntrySummary::for_page(entry_row.application_page, area_summaries);
    });
}

pub struct SidebarNavigationColumnInit {
    pub selected_page: ApplicationPage,
    pub message_catalog: Rc<MessageCatalog>,
}

#[relm4::component(pub)]
impl Component for SidebarNavigationColumn {
    type Init = SidebarNavigationColumnInit;
    type Input = SidebarNavigationColumnInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::NavigationPage {
            set_title: "Lazaret",

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_tooltip_text: Some(&translated_messages::window::menu_tooltip(&model.message_catalog)),
                        set_named_menu_model: &primary_menu(&model.message_catalog),
                        set_primary: true,
                    },
                },

                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    set_hscrollbar_policy: gtk::PolicyType::Never,
                    set_vexpand: true,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 18,
                        set_margin_top: 6,
                        set_margin_bottom: 12,
                        set_margin_start: 12,
                        set_margin_end: 12,

                        adw::PreferencesGroup {
                            set_title: &SidebarGroup::SignaturesGroup.group_title(&model.message_catalog),
                            set_description: Some(&SidebarGroup::SignaturesGroup.group_description(&model.message_catalog)),

                            #[local_ref]
                            signatures_list -> gtk::ListBox {
                                add_css_class: "boxed-list",
                                set_selection_mode: gtk::SelectionMode::Single,
                                #[watch]
                                select_row: selected_group_row(signatures_list, SidebarGroup::SignaturesGroup, model.selected_page).as_ref(),
                            },
                        },

                        adw::PreferencesGroup {
                            set_title: &SidebarGroup::ScansGroup.group_title(&model.message_catalog),
                            set_description: Some(&SidebarGroup::ScansGroup.group_description(&model.message_catalog)),

                            #[local_ref]
                            scans_list -> gtk::ListBox {
                                add_css_class: "boxed-list",
                                set_selection_mode: gtk::SelectionMode::Single,
                                #[watch]
                                select_row: selected_group_row(scans_list, SidebarGroup::ScansGroup, model.selected_page).as_ref(),
                            },
                        },

                        adw::PreferencesGroup {
                            set_title: &SidebarGroup::FindingsGroup.group_title(&model.message_catalog),
                            set_description: Some(&SidebarGroup::FindingsGroup.group_description(&model.message_catalog)),

                            #[local_ref]
                            findings_list -> gtk::ListBox {
                                add_css_class: "boxed-list",
                                set_selection_mode: gtk::SelectionMode::Single,
                                #[watch]
                                select_row: selected_group_row(findings_list, SidebarGroup::FindingsGroup, model.selected_page).as_ref(),
                            },
                        },
                    },
                },
            },
        }
    }

    fn init(
        SidebarNavigationColumnInit {
            selected_page,
            message_catalog,
        }: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            message_catalog: Rc::clone(&message_catalog),
            selected_page,
            signatures_rows: launched_group_rows(&message_catalog, SidebarGroup::SignaturesGroup),
            scans_rows: launched_group_rows(&message_catalog, SidebarGroup::ScansGroup),
            findings_rows: launched_group_rows(&message_catalog, SidebarGroup::FindingsGroup),
        };
        let signatures_list = model.signatures_rows.widget();
        let scans_list = model.scans_rows.widget();
        let findings_list = model.findings_rows.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        column_input: SidebarNavigationColumnInput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match column_input {
            SidebarNavigationColumnInput::SelectedPageChanged(selected_page) => {
                self.selected_page = selected_page;
            }
            SidebarNavigationColumnInput::AreaSummariesChanged(area_summaries) => {
                [
                    &mut self.signatures_rows,
                    &mut self.scans_rows,
                    &mut self.findings_rows,
                ]
                .into_iter()
                .for_each(|group_rows| show_area_summaries(group_rows, &area_summaries));
            }
        }
    }
}
