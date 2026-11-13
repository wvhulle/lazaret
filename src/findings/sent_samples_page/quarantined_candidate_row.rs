use std::rc::Rc;

use relm4::{FactorySender, adw, adw::prelude::*, gtk, prelude::FactoryComponent};

use crate::{
    findings::sent_samples_page::sent_samples_presentation::CandidateRowView, translated_messages,
    translated_messages::MessageCatalog,
};

#[derive(Debug)]
pub enum QuarantinedCandidateRowOutput {
    SubmissionRequested { entry_id: i64 },
}

pub struct QuarantinedCandidateRow {
    pub message_catalog: Rc<MessageCatalog>,
    row_view: CandidateRowView,
}

pub struct QuarantinedCandidateRowInit {
    pub row_view: CandidateRowView,
    pub message_catalog: Rc<MessageCatalog>,
}

#[relm4::factory(pub)]
impl FactoryComponent for QuarantinedCandidateRow {
    type Init = QuarantinedCandidateRowInit;
    type Input = ();
    type Output = QuarantinedCandidateRowOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        adw::ActionRow {
            set_use_markup: false,
            set_title: &self.row_view.row_title,
            set_subtitle: &self.row_view.row_subtitle,
            set_subtitle_lines: 0,

            add_suffix = &gtk::Button {
                set_label: &translated_messages::findings::submit_quarantined_label(&self.message_catalog),
                set_valign: gtk::Align::Center,
                #[watch]
                set_sensitive: self.row_view.allows_sending(),
                connect_clicked[sender, entry_id = self.row_view.entry_id] => move |_clicked_button| {
                    let _page_closed = sender.output(QuarantinedCandidateRowOutput::SubmissionRequested { entry_id });
                },
            },
        }
    }

    fn init_model(
        QuarantinedCandidateRowInit {
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
