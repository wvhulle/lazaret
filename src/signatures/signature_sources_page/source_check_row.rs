use std::rc::Rc;

use relm4::{
    FactorySender, adw,
    adw::prelude::*,
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
};

use crate::{
    shared_widgets::loading_indicators::inline_row_spinner,
    signatures::signature_sources_page::source_check_results::{
        AuditCheck, AuditStatus, CheckGuidance,
    },
    translated_messages,
    translated_messages::MessageCatalog,
};

pub enum AuditCheckKind {
    CheckingPlaceholder,
    CheckShown(AuditCheck),
}

pub struct AuditCheckRow {
    check_kind: AuditCheckKind,
    message_catalog: Rc<MessageCatalog>,
}

pub fn audit_check_row(
    message_catalog: &Rc<MessageCatalog>,
    check_kind: AuditCheckKind,
) -> AuditCheckRow {
    AuditCheckRow {
        check_kind,
        message_catalog: Rc::clone(message_catalog),
    }
}

#[derive(Debug)]
pub enum AuditCheckRowOutput {
    CopyCommandRequested(String),
    DocumentationRequested(&'static str),
}

fn status_indicator_image(
    message_catalog: &MessageCatalog,
    check_status: AuditStatus,
) -> gtk::Image {
    let status_label = check_status.status_label(message_catalog);
    let indicator_image = gtk::Image::builder()
        .icon_name(check_status.status_icon_name())
        .tooltip_text(&status_label)
        .valign(gtk::Align::Center)
        .css_classes([check_status.status_style_class()])
        .build();
    indicator_image.update_property(&[gtk::accessible::Property::Label(&status_label)]);
    indicator_image
}

fn recommendation_row(recommendation_text: &str) -> adw::ActionRow {
    adw::ActionRow::builder()
        .use_markup(false)
        .title(recommendation_text)
        .title_lines(0)
        .build()
}

fn suggested_command_row(
    message_catalog: &MessageCatalog,
    suggested_command: &'static str,
    sender: &FactorySender<AuditCheckRow>,
) -> adw::ActionRow {
    let command_label = gtk::Label::builder()
        .label(suggested_command)
        .use_markup(false)
        .selectable(true)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::WordChar)
        .hexpand(true)
        .xalign(0.0)
        .css_classes(["monospace"])
        .build();
    let copy_button = gtk::Button::builder()
        .icon_name("edit-copy-symbolic")
        .tooltip_text(translated_messages::signatures::copy_tooltip(
            message_catalog,
        ))
        .valign(gtk::Align::Center)
        .css_classes(["flat"])
        .build();
    let row_sender = sender.clone();
    copy_button.connect_clicked(move |_clicked_button| {
        let _section_closed = row_sender.output(AuditCheckRowOutput::CopyCommandRequested(
            suggested_command.to_owned(),
        ));
    });
    let command_row = adw::ActionRow::new();
    command_row.add_prefix(&command_label);
    command_row.add_suffix(&copy_button);
    command_row
}

fn documentation_row(
    message_catalog: &MessageCatalog,
    documentation_url: &'static str,
    sender: &FactorySender<AuditCheckRow>,
) -> adw::ActionRow {
    let documentation_row = adw::ActionRow::builder()
        .use_markup(false)
        .title(translated_messages::signatures::learn_more_title(
            message_catalog,
        ))
        .tooltip_text(translated_messages::signatures::open_browser_tooltip(
            message_catalog,
        ))
        .activatable(true)
        .build();
    documentation_row.add_suffix(&gtk::Image::from_icon_name("go-next-symbolic"));
    let row_sender = sender.clone();
    documentation_row.connect_activated(move |_activated_row| {
        let _section_closed = row_sender.output(AuditCheckRowOutput::DocumentationRequested(
            documentation_url,
        ));
    });
    documentation_row
}

fn guidance_row(
    message_catalog: &MessageCatalog,
    check_guidance: &CheckGuidance,
    sender: &FactorySender<AuditCheckRow>,
) -> adw::ActionRow {
    match check_guidance {
        CheckGuidance::RecommendationText(recommendation_text) => {
            recommendation_row(recommendation_text)
        }
        CheckGuidance::SuggestedCommand(suggested_command) => {
            suggested_command_row(message_catalog, suggested_command, sender)
        }
        CheckGuidance::DocumentationLink(documentation_url) => {
            documentation_row(message_catalog, documentation_url, sender)
        }
    }
}

fn checking_placeholder_row(message_catalog: &MessageCatalog) -> adw::PreferencesRow {
    let checking_row = adw::ActionRow::builder()
        .use_markup(false)
        .title(translated_messages::signatures::checking_title(
            message_catalog,
        ))
        .css_classes(["dim-label"])
        .build();
    checking_row.add_prefix(&inline_row_spinner());
    checking_row.upcast()
}

fn shown_check_row(
    message_catalog: &MessageCatalog,
    audit_check: &AuditCheck,
) -> adw::PreferencesRow {
    let status_image = status_indicator_image(message_catalog, audit_check.check_status);
    if audit_check.check_guidance.is_empty() {
        let plain_row = adw::ActionRow::builder()
            .use_markup(false)
            .title(&audit_check.check_name)
            .subtitle(&audit_check.status_detail)
            .subtitle_lines(0)
            .build();
        plain_row.add_suffix(&status_image);
        return plain_row.upcast();
    }
    let expander_row = adw::ExpanderRow::builder()
        .use_markup(false)
        .title(&audit_check.check_name)
        .subtitle(&audit_check.status_detail)
        .subtitle_lines(0)
        .expanded(audit_check.expanded_by_default())
        .build();
    expander_row.add_suffix(&status_image);
    expander_row.upcast()
}

impl FactoryComponent for AuditCheckRow {
    type ParentWidget = adw::PreferencesGroup;
    type CommandOutput = ();
    type Input = ();
    type Output = AuditCheckRowOutput;
    type Init = Self;
    type Root = adw::PreferencesRow;
    type Widgets = ();
    type Index = DynamicIndex;

    fn init_model(
        check_row: Self::Init,
        _row_index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        check_row
    }

    fn init_root(&self) -> Self::Root {
        let message_catalog = Rc::clone(&self.message_catalog);
        match &self.check_kind {
            AuditCheckKind::CheckingPlaceholder => checking_placeholder_row(&self.message_catalog),
            AuditCheckKind::CheckShown(audit_check) => {
                shown_check_row(&message_catalog, audit_check)
            }
        }
    }

    fn init_widgets(
        &mut self,
        _row_index: &DynamicIndex,
        root: Self::Root,
        _returned_widget: &gtk::Widget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        if let (AuditCheckKind::CheckShown(audit_check), Some(expander_row)) =
            (&self.check_kind, root.downcast_ref::<adw::ExpanderRow>())
        {
            audit_check
                .check_guidance
                .iter()
                .for_each(|check_guidance| {
                    expander_row.add_row(&guidance_row(
                        &self.message_catalog,
                        check_guidance,
                        &sender,
                    ));
                });
        }
    }
}
