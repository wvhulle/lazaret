use std::time::Duration;

use gtk_scene_capture::accessible_widget_lookup::{WidgetRole, WidgetTarget};

use super::{
    LONG_WAIT, SHORT_WAIT,
    clamd_settings::{MAX_THREADS, merged_config, stored_overrides, wait_for_clamd},
};
use crate::vm_journeys::harness::{
    journey_progress::{SAMPLE_PASSWORD, announce_step},
    lazaret_application::{ApplicationPage, show_page, wait_for_lazaret_service},
    session_widgets::{
        DEFAULT_WIDGET_TIMEOUT, SessionWidgets, button, in_alert, in_expanded_row, in_row,
        label_containing, row_titled,
    },
    sway_windows::{answer_password_prompt, take_screenshot},
    system_commands::wait_until,
    ui_messages::{MessageCatalog, host, widgets},
};

const EDITOR_WAIT: Duration = Duration::from_secs(15);

fn max_threads_editor() -> WidgetTarget {
    WidgetTarget::new(WidgetRole::SpinButton, MAX_THREADS)
}

fn change_confirmation() -> WidgetTarget {
    WidgetTarget::alert(widgets::change_setting(&MessageCatalog::source_language()))
}

fn show_scan_engine_page(session_widgets: &SessionWidgets) {
    show_page(ApplicationPage::ScanEngine);
    wait_until(
        "the MaxThreads setting on the Scan Engine page",
        SHORT_WAIT,
        || {
            session_widgets.is_shown(&row_titled(MAX_THREADS))
                || session_widgets.is_shown(&max_threads_editor())
        },
    );
}

fn edit_max_threads(session_widgets: &SessionWidgets, new_value: u32) {
    session_widgets.set_spin_value(&max_threads_editor(), f64::from(new_value));
    session_widgets.press(&in_expanded_row(
        button(widgets::apply_tooltip(&MessageCatalog::source_language())),
        MAX_THREADS,
    ));
    assert!(
        session_widgets.is_shown_within(&change_confirmation(), DEFAULT_WIDGET_TIMEOUT),
        "the MaxThreads editor showed no confirmation: {}",
        session_widgets.describe_widget_input(&max_threads_editor())
    );
}

fn wait_for_merged_max_threads(thread_count: u32) {
    let merged_line = format!("{MAX_THREADS} {thread_count}");
    wait_until(
        &format!("{merged_line} in the merged configuration"),
        LONG_WAIT,
        || merged_config().contains(&merged_line),
    );
    wait_for_clamd();
}

pub fn engine_settings_show_their_source(session_widgets: &SessionWidgets) {
    announce_step("each Scanner Engine setting shows where its value comes from");
    wait_for_lazaret_service();
    show_scan_engine_page(session_widgets);
    session_widgets.await_shown(&label_containing(widgets::source_nixos_tag(
        &MessageCatalog::source_language(),
    )));
    take_screenshot("scanner_engine_sources");
}

pub fn disallowed_setting_offers_only_configuration_line(session_widgets: &SessionWidgets) {
    announce_step("a setting the system doesn't allow offers only its configuration line");
    let merged_before_copy = merged_config();
    let overrides_before_copy = stored_overrides();
    let heuristic_title = widgets::heuristic_detections(&MessageCatalog::source_language());
    session_widgets.press(&row_titled(heuristic_title.clone()));
    session_widgets.await_shown(&label_containing(
        widgets::route_outside_allowlist_explanation(&MessageCatalog::source_language()),
    ));
    session_widgets.press(&in_expanded_row(
        button(widgets::copy_configuration_line_button(
            &MessageCatalog::source_language(),
        )),
        heuristic_title,
    ));
    session_widgets.await_shown(&label_containing(
        widgets::nothing_applied_choose_description(&MessageCatalog::source_language()),
    ));
    session_widgets.press(&button(widgets::copy_line_label(
        &MessageCatalog::source_language(),
    )));
    session_widgets.await_shown_within(
        &label_containing(widgets::change_snippet_copied(
            &MessageCatalog::source_language(),
        )),
        SHORT_WAIT,
    );
    take_screenshot("copy_only_setting");
    assert_eq!(
        merged_config(),
        merged_before_copy,
        "copying a line changed clamd"
    );
    assert_eq!(
        stored_overrides(),
        overrides_before_copy,
        "copying a line changed the stored changes"
    );
}

pub fn allowed_settings_are_editable(session_widgets: &SessionWidgets) {
    announce_step("allowed settings are editable without unlocking anything");
    show_scan_engine_page(session_widgets);
    let editor_shown = session_widgets.is_shown_within(&max_threads_editor(), EDITOR_WAIT);
    take_screenshot("allowed_setting_editable");
    assert!(
        editor_shown,
        "the MaxThreads editor cannot be reached by its name; its ancestry:\n{}",
        session_widgets
            .describe_ancestry_of_named(MAX_THREADS)
            .join("\n")
    );
}

pub fn confirmed_edit_applies_through_password_prompt(session_widgets: &SessionWidgets) {
    announce_step("editing and confirming applies through a real password prompt");
    edit_max_threads(session_widgets, 8);
    take_screenshot("confirm_change");
    session_widgets.press(&in_alert(
        button(widgets::apply_label(&MessageCatalog::source_language())),
        widgets::change_setting(&MessageCatalog::source_language()),
    ));
    answer_password_prompt(SAMPLE_PASSWORD);
    wait_for_merged_max_threads(8);
}

pub fn second_edit_starts_from_applied_value(session_widgets: &SessionWidgets) {
    announce_step("a second change starts from the value just applied");
    show_scan_engine_page(session_widgets);
    edit_max_threads(session_widgets, 10);
    session_widgets.await_shown(&label_containing(host::key_now_current_new(
        &MessageCatalog::source_language(),
        MAX_THREADS,
        8,
        10,
    )));
    take_screenshot("confirm_second_change");
    session_widgets.press(&in_alert(
        button(widgets::cancel_engine(&MessageCatalog::source_language())),
        widgets::change_setting(&MessageCatalog::source_language()),
    ));
}

pub fn changed_row_reverts_in_place(session_widgets: &SessionWidgets) {
    announce_step("the changed row shows its runtime change and reverts it in place");
    show_scan_engine_page(session_widgets);
    session_widgets.await_shown(&in_expanded_row(
        label_containing(widgets::source_runtime_nixos_explanation(
            &MessageCatalog::source_language(),
        )),
        MAX_THREADS,
    ));
    take_screenshot("runtime_change_in_row");
    session_widgets.press(&in_expanded_row(
        button(widgets::revert_change_button(
            &MessageCatalog::source_language(),
        )),
        MAX_THREADS,
    ));
    answer_password_prompt(SAMPLE_PASSWORD);
    wait_for_merged_max_threads(4);
    session_widgets.await_shown(&in_row(
        label_containing(widgets::source_nixos_tag(&MessageCatalog::source_language())),
        MAX_THREADS,
    ));
}
