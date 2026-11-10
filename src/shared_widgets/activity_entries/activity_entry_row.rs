use relm4::{
    FactorySender, adw,
    adw::prelude::*,
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
};

use crate::{
    host_services::{
        activity_history::activity_history_database::{
            ActivityEntryRecord, ActivityRecord, ScanActivityStatus, UpdateActivityStatus,
        },
        desktop::locale_formatting::MomentFormatters,
    },
    shared_widgets::activity_entries::activity_export_formats::{
        activity_status_text, activity_summary_text,
    },
};

fn status_icon(activity_record: &ActivityRecord) -> &'static str {
    match activity_record {
        ActivityRecord::ScanActivity { scan_status, .. } => match scan_status {
            ScanActivityStatus::ScanClean => "object-select-symbolic",
            ScanActivityStatus::ScanInfected { .. } | ScanActivityStatus::ScanFailed => {
                "dialog-warning-symbolic"
            }
            ScanActivityStatus::ScanCancelled => "content-loading-symbolic",
        },
        ActivityRecord::SkippedScan { .. } => "content-loading-symbolic",
        ActivityRecord::UpdateActivity { update_status } => match update_status {
            UpdateActivityStatus::UpdateApplied | UpdateActivityStatus::UpdateAlreadyCurrent => {
                "object-select-symbolic"
            }
            UpdateActivityStatus::UpdateFailed => "dialog-warning-symbolic",
            UpdateActivityStatus::UpdateCancelled
            | UpdateActivityStatus::UpdateRequested
            | UpdateActivityStatus::UpdateResultUnknown => "content-loading-symbolic",
        },
    }
}

fn entry_subtitle(
    moment_formatters: &MomentFormatters,
    activity_entry: &ActivityEntryRecord,
) -> String {
    let first_target = activity_entry.activity_record.scan_targets().first();
    [
        moment_formatters.exact_moment_text(activity_entry.started_at),
        activity_status_text(&activity_entry.activity_record),
    ]
    .into_iter()
    .chain(first_target.map(|scan_target| scan_target.display().to_string()))
    .collect::<Vec<_>>()
    .join(" • ")
}

pub struct ActivityEntryRow {
    pub moment_formatters: &'static MomentFormatters,
    pub activity_entry: ActivityEntryRecord,
}

pub struct ActivityEntryRowInit {
    pub activity_entry: ActivityEntryRecord,
    pub moment_formatters: &'static MomentFormatters,
}

#[relm4::factory(pub)]
impl FactoryComponent for ActivityEntryRow {
    type Init = ActivityEntryRowInit;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        adw::ActionRow {
            set_use_markup: false,
            set_activatable: true,
            set_title: &activity_summary_text(&self.activity_entry),
            set_subtitle: &entry_subtitle(self.moment_formatters, &self.activity_entry),

            add_suffix = &gtk::Image {
                set_icon_name: Some(status_icon(&self.activity_entry.activity_record)),
                set_tooltip_text: Some(&activity_status_text(&self.activity_entry.activity_record)),
            },
        }
    }

    fn init_model(
        ActivityEntryRowInit {
            activity_entry,
            moment_formatters,
        }: Self::Init,
        _row_index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            activity_entry,
            moment_formatters,
        }
    }
}
