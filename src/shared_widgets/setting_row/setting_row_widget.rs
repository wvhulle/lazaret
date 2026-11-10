use clamd_configuration::setting_kinds::whole_number_in_text;
use relm4::{
    FactorySender, adw,
    adw::prelude::*,
    gtk,
    gtk::glib,
    prelude::{DynamicIndex, FactoryComponent},
};

use crate::{
    host_services::{
        clamav_config::typed_setting_values::{
            DraftEdit, SIZE_UNITS, SizeUnit, SwitchPosition, TypedSettingValue,
        },
        desktop::locale_formatting::{MomentFormatters, byte_size_text},
    },
    shared_widgets::setting_row::setting_row_state::{
        ChangeRoute, FollowUpAction, RowChangeActivity, SettingLabel, SettingRowState,
        SettingSubject, ShownSettingValue, SourceEmphasis, change_route, describe_parse_failure,
        draft_refusal, follow_up_actions, route_explanation, row_needs_attention, shown_value_text,
        source_texts,
    },
    translated_messages,
};

const SIZE_COUNT_CEILING: f64 = i32::MAX as f64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EditorApplyTiming {
    AppliedOnEveryEdit,
    AppliedOnRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EditorKind {
    ReadOnlySwitch,
    ReadOnlyText,
    EditableSwitch,
    EditableCount,
    EditableSize,
    EditableText,
    EditableChoice,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DraftComparison {
    SameAsShown,
    ChangedFromShown,
}

pub struct SettingRow {
    moment_formatters: &'static MomentFormatters,
    row_state: SettingRowState,
    editor_draft: TypedSettingValue,
}

#[derive(Debug)]
pub enum SettingRowInput {
    StateReplaced(Box<SettingRowState>),
    ChangeActivityChanged(RowChangeActivity),
    DraftEdited(DraftEdit),
    ApplyRequested,
    DraftEditedAndApplied(DraftEdit),
    FollowUpPressed(FollowUpAction),
}

#[derive(Debug)]
pub enum SettingRowOutput {
    ValueChangeRequested {
        setting_subject: SettingSubject,
        new_value: TypedSettingValue,
    },
    FollowUpChosen {
        setting_subject: SettingSubject,
        shown_value: ShownSettingValue,
        follow_up_action: FollowUpAction,
    },
}

enum SuffixEditor {
    ValueLabel(gtk::Label),
    SwitchEditor {
        value_switch: gtk::Switch,
        toggle_handler: glib::SignalHandlerId,
    },
    CountEditor {
        spin_button: gtk::SpinButton,
        value_handler: glib::SignalHandlerId,
        apply_button: gtk::Button,
    },
    SizeEditor {
        spin_button: gtk::SpinButton,
        value_handler: glib::SignalHandlerId,
        unit_dropdown: gtk::DropDown,
        unit_handler: glib::SignalHandlerId,
        apply_button: gtk::Button,
    },
    TextEditor {
        text_entry: gtk::Entry,
        text_handler: glib::SignalHandlerId,
        apply_button: gtk::Button,
    },
    ChoiceEditor {
        choice_dropdown: gtk::DropDown,
        choice_handler: glib::SignalHandlerId,
    },
}

pub struct SettingRowWidgets {
    expander_row: adw::ExpanderRow,
    source_icon: gtk::Image,
    editor_slot: gtk::Box,
    editor_kind: EditorKind,
    suffix_editor: SuffixEditor,
    applying_spinner: adw::Spinner,
    explanation_label: gtk::Label,
    action_box: gtk::Box,
    shown_actions: Vec<FollowUpAction>,
    attention_shown: bool,
}

fn row_title(row_state: &SettingRowState) -> String {
    match (&row_state.setting_label, &row_state.setting_subject) {
        (SettingLabel::DescribedSetting { title, .. }, _setting_subject) => title.clone(),
        (
            SettingLabel::ConfigKeyOnly,
            SettingSubject::ClamavSetting {
                setting_address, ..
            },
        ) => setting_address.setting_key.clone(),
        (SettingLabel::ConfigKeyOnly, SettingSubject::Preference { preference_key, .. }) => {
            preference_key.schema_key().to_owned()
        }
    }
}

fn explanation_text(moment_formatters: &MomentFormatters, row_state: &SettingRowState) -> String {
    let setting_description = match &row_state.setting_label {
        SettingLabel::DescribedSetting { description, .. } => description.clone(),
        SettingLabel::ConfigKeyOnly => String::new(),
    };
    let unreadable_note = match &row_state.shown_value {
        ShownSettingValue::ReadableValue(_) => String::new(),
        ShownSettingValue::UnreadableValue { parse_failure, .. } => {
            describe_parse_failure(*parse_failure)
        }
    };
    [
        setting_description,
        unreadable_note,
        source_texts(moment_formatters, &row_state.setting_subject).explanation,
        route_explanation(change_route(&row_state.setting_subject)),
    ]
    .into_iter()
    .filter(|paragraph| !paragraph.is_empty())
    .collect::<Vec<_>>()
    .join("\n\n")
}

fn editor_kind_for(row_state: &SettingRowState) -> EditorKind {
    let editable = change_route(&row_state.setting_subject).allows_editing();
    match (&row_state.shown_value, editable) {
        (ShownSettingValue::UnreadableValue { .. }, _editable) => EditorKind::ReadOnlyText,
        (ShownSettingValue::ReadableValue(TypedSettingValue::BooleanValue(_)), false) => {
            EditorKind::ReadOnlySwitch
        }
        (ShownSettingValue::ReadableValue(_), false) => EditorKind::ReadOnlyText,
        (ShownSettingValue::ReadableValue(typed_value), true) => match typed_value {
            TypedSettingValue::BooleanValue(_) => EditorKind::EditableSwitch,
            TypedSettingValue::CountValue { .. } => EditorKind::EditableCount,
            TypedSettingValue::SizeValue(_) => EditorKind::EditableSize,
            TypedSettingValue::PathValue(_) | TypedSettingValue::TextValue(_) => {
                EditorKind::EditableText
            }
            TypedSettingValue::ChoiceValue { .. } => EditorKind::EditableChoice,
        },
    }
}

fn shown_value_or_text(shown_value: &ShownSettingValue) -> TypedSettingValue {
    match shown_value {
        ShownSettingValue::ReadableValue(typed_value) => typed_value.clone(),
        ShownSettingValue::UnreadableValue { raw_text, .. } => {
            TypedSettingValue::TextValue(raw_text.clone())
        }
    }
}

fn read_only_value_text(shown_value: &ShownSettingValue) -> String {
    match shown_value {
        ShownSettingValue::ReadableValue(TypedSettingValue::SizeValue(size_amount)) => size_amount
            .byte_size()
            .map_or_else(|| size_amount.config_text(), byte_size_text),
        ShownSettingValue::ReadableValue(typed_value) => shown_value_text(typed_value),
        ShownSettingValue::UnreadableValue { raw_text, .. } => raw_text.clone(),
    }
}

const fn editor_apply_timing(setting_route: ChangeRoute) -> EditorApplyTiming {
    match setting_route {
        ChangeRoute::AsksAdministratorPassword => EditorApplyTiming::AppliedOnRequest,
        ChangeRoute::ChangesImmediately
        | ChangeRoute::TakesEffectOnGroupApply
        | ChangeRoute::CopyLineForConfiguration(_)
        | ChangeRoute::FixedByConfiguration
        | ChangeRoute::NeverChangedHere => EditorApplyTiming::AppliedOnEveryEdit,
    }
}

fn follow_up_label(follow_up_action: FollowUpAction) -> String {
    match follow_up_action {
        FollowUpAction::ResetToDefault => translated_messages::widgets::reset_default_button(),
        FollowUpAction::RevertRuntimeChange => translated_messages::widgets::revert_change_button(),
        FollowUpAction::CopyConfigurationLine => {
            translated_messages::widgets::copy_configuration_line_button()
        }
    }
}

const fn follow_up_is_primary(follow_up_action: FollowUpAction) -> bool {
    matches!(follow_up_action, FollowUpAction::RevertRuntimeChange)
}

fn apply_button() -> gtk::Button {
    gtk::Button::builder()
        .icon_name("object-select-symbolic")
        .tooltip_text(translated_messages::widgets::apply_tooltip())
        .valign(gtk::Align::Center)
        .css_classes(["suggested-action", "circular"])
        .build()
}

fn labelled_for_assistive_technology(editor_widget: &impl IsA<gtk::Widget>, editor_label: &str) {
    editor_widget
        .upcast_ref::<gtk::Widget>()
        .update_property(&[gtk::accessible::Property::Label(editor_label)]);
}

fn spin_entered_input(spin_button: &gtk::SpinButton) -> SettingRowInput {
    whole_number_in_text(spin_button.text().trim())
        .map_or(SettingRowInput::ApplyRequested, |entered_count| {
            SettingRowInput::DraftEditedAndApplied(DraftEdit::NumberSet(entered_count))
        })
}

fn count_spin_button(minimum: f64, maximum: f64, shown_count: f64) -> gtk::SpinButton {
    let spin_button = gtk::SpinButton::new(
        Some(&gtk::Adjustment::new(
            shown_count,
            minimum,
            maximum,
            1.0,
            10.0,
            0.0,
        )),
        1.0,
        0,
    );
    spin_button.set_numeric(true);
    spin_button.set_valign(gtk::Align::Center);
    spin_button.set_value(shown_count);
    spin_button
}

fn connect_count_editing(
    spin_button: &gtk::SpinButton,
    sender: &FactorySender<SettingRow>,
) -> glib::SignalHandlerId {
    let value_sender = sender.clone();
    let value_handler = spin_button.connect_value_changed(move |edited_spin| {
        value_sender.input(SettingRowInput::DraftEdited(DraftEdit::NumberSet(
            edited_spin.value() as u64,
        )));
    });
    let activation_sender = sender.clone();
    spin_button.connect_activate(move |activated_spin| {
        activation_sender.input(spin_entered_input(activated_spin));
    });
    value_handler
}

fn connected_apply_button(sender: &FactorySender<SettingRow>) -> gtk::Button {
    let apply_button = apply_button();
    let apply_sender = sender.clone();
    apply_button.connect_clicked(move |_clicked_button| {
        apply_sender.input(SettingRowInput::ApplyRequested);
    });
    apply_button
}

pub fn size_unit_labels() -> gtk::StringList {
    let unit_labels = SIZE_UNITS.map(size_unit_label);
    gtk::StringList::new(&unit_labels.each_ref().map(String::as_str))
}

pub fn size_unit_label(size_unit: SizeUnit) -> String {
    match size_unit {
        SizeUnit::PlainBytes => translated_messages::widgets::plain_bytes(),
        SizeUnit::Kibibytes => translated_messages::widgets::kibibytes_unit(),
        SizeUnit::Mebibytes => translated_messages::widgets::mebibytes_unit(),
    }
}

fn build_suffix_editor(
    editor_kind: EditorKind,
    editor_draft: &TypedSettingValue,
    shown_value: &ShownSettingValue,
    editor_label: &str,
    sender: &FactorySender<SettingRow>,
) -> (gtk::Box, SuffixEditor) {
    let editor_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .valign(gtk::Align::Center)
        .build();
    let suffix_editor = match (editor_kind, editor_draft) {
        (EditorKind::ReadOnlySwitch | EditorKind::EditableSwitch, _editor_draft) => {
            let value_switch = gtk::Switch::builder()
                .active(
                    editor_draft
                        .shown_switch_position()
                        .is_some_and(SwitchPosition::is_switched_on),
                )
                .sensitive(editor_kind == EditorKind::EditableSwitch)
                .valign(gtk::Align::Center)
                .build();
            labelled_for_assistive_technology(&value_switch, editor_label);
            let toggle_sender = sender.clone();
            let toggle_handler = value_switch.connect_active_notify(move |toggled_switch| {
                toggle_sender.input(SettingRowInput::DraftEditedAndApplied(
                    DraftEdit::SwitchSet(SwitchPosition::from_switch_active(
                        toggled_switch.is_active(),
                    )),
                ));
            });
            editor_box.append(&value_switch);
            SuffixEditor::SwitchEditor {
                value_switch,
                toggle_handler,
            }
        }
        (
            EditorKind::EditableCount,
            TypedSettingValue::CountValue {
                count_value,
                minimum,
                maximum,
            },
        ) => {
            let spin_button =
                count_spin_button(*minimum as f64, *maximum as f64, *count_value as f64);
            labelled_for_assistive_technology(&spin_button, editor_label);
            let value_handler = connect_count_editing(&spin_button, sender);
            let apply_button = connected_apply_button(sender);
            editor_box.append(&spin_button);
            editor_box.append(&apply_button);
            SuffixEditor::CountEditor {
                spin_button,
                value_handler,
                apply_button,
            }
        }
        (EditorKind::EditableSize, TypedSettingValue::SizeValue(size_amount)) => {
            let spin_button =
                count_spin_button(0.0, SIZE_COUNT_CEILING, size_amount.unit_count as f64);
            labelled_for_assistive_technology(&spin_button, editor_label);
            let value_handler = connect_count_editing(&spin_button, sender);
            let unit_dropdown = gtk::DropDown::builder()
                .model(&size_unit_labels())
                .selected(size_amount.size_unit.unit_position())
                .valign(gtk::Align::Center)
                .tooltip_text(translated_messages::widgets::unit_tooltip())
                .build();
            let unit_sender = sender.clone();
            let unit_handler = unit_dropdown.connect_selected_notify(move |chosen_unit| {
                if let Some(size_unit) = SizeUnit::unit_at_position(chosen_unit.selected()) {
                    unit_sender.input(SettingRowInput::DraftEdited(DraftEdit::SizeUnitSet(
                        size_unit,
                    )));
                }
            });
            let apply_button = connected_apply_button(sender);
            editor_box.append(&spin_button);
            editor_box.append(&unit_dropdown);
            editor_box.append(&apply_button);
            SuffixEditor::SizeEditor {
                spin_button,
                value_handler,
                unit_dropdown,
                unit_handler,
                apply_button,
            }
        }
        (
            EditorKind::EditableText,
            TypedSettingValue::PathValue(draft_text) | TypedSettingValue::TextValue(draft_text),
        ) => {
            let text_entry = gtk::Entry::builder()
                .text(draft_text)
                .valign(gtk::Align::Center)
                .width_chars(12)
                .build();
            labelled_for_assistive_technology(&text_entry, editor_label);
            let text_sender = sender.clone();
            let text_handler = text_entry.connect_changed(move |edited_entry| {
                text_sender.input(SettingRowInput::DraftEdited(DraftEdit::TextSet(
                    edited_entry.text().to_string(),
                )));
            });
            let activation_sender = sender.clone();
            text_entry.connect_activate(move |_activated_entry| {
                activation_sender.input(SettingRowInput::ApplyRequested);
            });
            let apply_button = connected_apply_button(sender);
            editor_box.append(&text_entry);
            editor_box.append(&apply_button);
            SuffixEditor::TextEditor {
                text_entry,
                text_handler,
                apply_button,
            }
        }
        (
            EditorKind::EditableChoice,
            TypedSettingValue::ChoiceValue {
                chosen_position,
                choice_labels,
            },
        ) => {
            let choice_dropdown = gtk::DropDown::builder()
                .model(&gtk::StringList::new(
                    &choice_labels.iter().map(String::as_str).collect::<Vec<_>>(),
                ))
                .selected(*chosen_position)
                .valign(gtk::Align::Center)
                .build();
            labelled_for_assistive_technology(&choice_dropdown, editor_label);
            let choice_sender = sender.clone();
            let choice_handler = choice_dropdown.connect_selected_notify(move |chosen_dropdown| {
                choice_sender.input(SettingRowInput::DraftEditedAndApplied(
                    DraftEdit::ChoiceSet(chosen_dropdown.selected()),
                ));
            });
            editor_box.append(&choice_dropdown);
            SuffixEditor::ChoiceEditor {
                choice_dropdown,
                choice_handler,
            }
        }
        (
            EditorKind::ReadOnlyText
            | EditorKind::EditableCount
            | EditorKind::EditableSize
            | EditorKind::EditableText
            | EditorKind::EditableChoice,
            _mismatched_draft,
        ) => {
            let value_label = gtk::Label::builder()
                .label(read_only_value_text(shown_value))
                .selectable(true)
                .ellipsize(gtk::pango::EllipsizeMode::Middle)
                .max_width_chars(28)
                .valign(gtk::Align::Center)
                .css_classes(["dim-label"])
                .build();
            labelled_for_assistive_technology(&value_label, editor_label);
            editor_box.append(&value_label);
            SuffixEditor::ValueLabel(value_label)
        }
    };
    (editor_box, suffix_editor)
}

impl SettingRow {
    pub const fn row_state(&self) -> &SettingRowState {
        &self.row_state
    }

    fn setting_route(&self) -> ChangeRoute {
        change_route(&self.row_state.setting_subject)
    }

    fn draft_comparison(&self, editor_draft: &TypedSettingValue) -> DraftComparison {
        if self.row_state.shown_value == ShownSettingValue::ReadableValue(editor_draft.clone()) {
            DraftComparison::SameAsShown
        } else {
            DraftComparison::ChangedFromShown
        }
    }

    fn draft_is_refused(&self) -> bool {
        draft_refusal(&self.row_state.setting_subject, &self.editor_draft).is_err()
    }

    fn apply_is_possible(&self) -> bool {
        self.setting_route().allows_editing()
            && self.row_state.change_activity.accepts_changes()
            && self.draft_comparison(&self.editor_draft) == DraftComparison::ChangedFromShown
            && !self.draft_is_refused()
    }

    fn report_draft(&self, sender: &FactorySender<Self>) {
        if self.apply_is_possible() {
            let _parent_closed = sender.output(SettingRowOutput::ValueChangeRequested {
                setting_subject: self.row_state.setting_subject.clone(),
                new_value: self.editor_draft.clone(),
            });
        }
    }

    fn edit_draft(&mut self, draft_edit: DraftEdit, sender: &FactorySender<Self>) {
        self.editor_draft = self.editor_draft.clone().with_draft_edit(draft_edit);
        match editor_apply_timing(self.setting_route()) {
            EditorApplyTiming::AppliedOnEveryEdit => self.report_draft(sender),
            EditorApplyTiming::AppliedOnRequest => {}
        }
    }

    fn header_subtitle(&self) -> String {
        let moment_formatters = self.moment_formatters;
        if self.row_state.change_activity.shows_applying_spinner() {
            translated_messages::widgets::setting_applying_subtitle()
        } else {
            source_texts(moment_formatters, &self.row_state.setting_subject).short_tag
        }
    }

    fn source_icon_name(&self) -> Option<&'static str> {
        let moment_formatters = self.moment_formatters;
        match source_texts(moment_formatters, &self.row_state.setting_subject).source_emphasis {
            SourceEmphasis::PlainSource => None,
            SourceEmphasis::NeedsAttention => Some("dialog-warning-symbolic"),
            SourceEmphasis::FixedValue => Some("changes-prevent-symbolic"),
        }
    }

    fn rebuild_follow_up_buttons(
        &self,
        widgets: &mut SettingRowWidgets,
        sender: &FactorySender<Self>,
    ) {
        let follow_ups =
            follow_up_actions(&self.row_state.setting_subject, &self.row_state.shown_value);
        while let Some(old_button) = widgets.action_box.first_child() {
            widgets.action_box.remove(&old_button);
        }
        follow_ups.iter().for_each(|follow_up_action| {
            let follow_up_button = gtk::Button::builder()
                .label(follow_up_label(*follow_up_action))
                .build();
            if follow_up_is_primary(*follow_up_action) {
                follow_up_button.add_css_class("suggested-action");
            }
            let pressed_action = *follow_up_action;
            let follow_up_sender = sender.clone();
            follow_up_button.connect_clicked(move |_clicked_button| {
                follow_up_sender.input(SettingRowInput::FollowUpPressed(pressed_action));
            });
            widgets.action_box.append(&follow_up_button);
        });
        widgets.action_box.set_visible(!follow_ups.is_empty());
        widgets.shown_actions = follow_ups;
    }

    fn replace_editor_when_kind_changed(
        &self,
        widgets: &mut SettingRowWidgets,
        sender: &FactorySender<Self>,
    ) {
        let wanted_kind = editor_kind_for(&self.row_state);
        if wanted_kind == widgets.editor_kind {
            return;
        }
        while let Some(old_editor) = widgets.editor_slot.first_child() {
            widgets.editor_slot.remove(&old_editor);
        }
        let (editor_box, suffix_editor) = build_suffix_editor(
            wanted_kind,
            &self.editor_draft,
            &self.row_state.shown_value,
            &row_title(&self.row_state),
            sender,
        );
        widgets.editor_slot.append(&editor_box);
        widgets.editor_kind = wanted_kind;
        widgets.suffix_editor = suffix_editor;
    }

    fn show_draft_in_editor(&self, widgets: &SettingRowWidgets) {
        let explicit_apply =
            editor_apply_timing(self.setting_route()) == EditorApplyTiming::AppliedOnRequest;
        match &widgets.suffix_editor {
            SuffixEditor::ValueLabel(value_label) => {
                value_label.set_label(&read_only_value_text(&self.row_state.shown_value));
            }
            SuffixEditor::SwitchEditor {
                value_switch,
                toggle_handler,
            } => {
                if let Some(switch_position) = self.editor_draft.shown_switch_position() {
                    value_switch.block_signal(toggle_handler);
                    value_switch.set_active(switch_position.is_switched_on());
                    value_switch.unblock_signal(toggle_handler);
                }
            }
            SuffixEditor::CountEditor {
                spin_button,
                value_handler,
                apply_button,
            } => {
                if let Some(shown_count) = self.editor_draft.shown_count() {
                    spin_button.block_signal(value_handler);
                    spin_button.set_value(shown_count as f64);
                    spin_button.unblock_signal(value_handler);
                }
                apply_button.set_visible(explicit_apply);
                apply_button.set_sensitive(self.apply_is_possible());
            }
            SuffixEditor::SizeEditor {
                spin_button,
                value_handler,
                unit_dropdown,
                unit_handler,
                apply_button,
            } => {
                if let Some(shown_count) = self.editor_draft.shown_count() {
                    spin_button.block_signal(value_handler);
                    spin_button.set_value(shown_count as f64);
                    spin_button.unblock_signal(value_handler);
                }
                if let Some(size_unit) = self.editor_draft.shown_size_unit() {
                    unit_dropdown.block_signal(unit_handler);
                    unit_dropdown.set_selected(size_unit.unit_position());
                    unit_dropdown.unblock_signal(unit_handler);
                }
                apply_button.set_visible(explicit_apply);
                apply_button.set_sensitive(self.apply_is_possible());
            }
            SuffixEditor::TextEditor {
                text_entry,
                text_handler,
                apply_button,
            } => {
                if let Some(draft_text) = self.editor_draft.shown_text()
                    && text_entry.text() != draft_text
                {
                    text_entry.block_signal(text_handler);
                    text_entry.set_text(draft_text);
                    text_entry.unblock_signal(text_handler);
                }
                if self.draft_is_refused() {
                    text_entry.add_css_class("error");
                } else {
                    text_entry.remove_css_class("error");
                }
                apply_button.set_visible(explicit_apply);
                apply_button.set_sensitive(self.apply_is_possible());
            }
            SuffixEditor::ChoiceEditor {
                choice_dropdown,
                choice_handler,
            } => {
                if let Some(chosen_position) = self.editor_draft.shown_choice_position() {
                    choice_dropdown.block_signal(choice_handler);
                    choice_dropdown.set_selected(chosen_position);
                    choice_dropdown.unblock_signal(choice_handler);
                }
            }
        }
    }
}

pub struct SettingRowInit {
    pub row_state: SettingRowState,
    pub moment_formatters: &'static MomentFormatters,
}

impl FactoryComponent for SettingRow {
    type ParentWidget = adw::PreferencesGroup;
    type CommandOutput = ();
    type Input = SettingRowInput;
    type Output = SettingRowOutput;
    type Init = SettingRowInit;
    type Root = adw::ExpanderRow;
    type Widgets = SettingRowWidgets;
    type Index = DynamicIndex;

    fn init_model(
        SettingRowInit {
            row_state,
            moment_formatters,
        }: Self::Init,
        _row_index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self {
            moment_formatters,
            editor_draft: shown_value_or_text(&row_state.shown_value),
            row_state,
        }
    }

    fn init_root(&self) -> Self::Root {
        adw::ExpanderRow::builder()
            .use_markup(false)
            .title(row_title(&self.row_state))
            .build()
    }

    fn init_widgets(
        &mut self,
        _row_index: &DynamicIndex,
        root: Self::Root,
        _returned_widget: &gtk::Widget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let source_icon = gtk::Image::builder().valign(gtk::Align::Center).build();
        root.add_prefix(&source_icon);
        let editor_kind = editor_kind_for(&self.row_state);
        let (editor_box, suffix_editor) = build_suffix_editor(
            editor_kind,
            &self.editor_draft,
            &self.row_state.shown_value,
            &row_title(&self.row_state),
            &sender,
        );
        let editor_slot = gtk::Box::builder().valign(gtk::Align::Center).build();
        editor_slot.append(&editor_box);
        let applying_spinner = adw::Spinner::builder()
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .tooltip_text(translated_messages::widgets::applying_restarting_clamav_tooltip())
            .visible(false)
            .build();
        root.add_suffix(&applying_spinner);
        root.add_suffix(&editor_slot);
        let explanation_label = gtk::Label::builder()
            .xalign(0.0)
            .wrap(true)
            .selectable(true)
            .use_markup(false)
            .build();
        let action_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(12)
            .build();
        let detail_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        detail_box.append(&explanation_label);
        detail_box.append(&action_box);
        root.add_row(&detail_box);
        let attention_shown =
            row_needs_attention(self.moment_formatters, &self.row_state.setting_subject);
        root.set_expanded(attention_shown);
        let mut widgets = SettingRowWidgets {
            expander_row: root,
            source_icon,
            editor_slot,
            editor_kind,
            suffix_editor,
            applying_spinner,
            explanation_label,
            action_box,
            shown_actions: Vec::new(),
            attention_shown,
        };
        self.update_view(&mut widgets, sender);
        widgets
    }

    fn update(&mut self, row_input: Self::Input, sender: FactorySender<Self>) {
        match row_input {
            SettingRowInput::StateReplaced(row_state) => {
                self.editor_draft = shown_value_or_text(&row_state.shown_value);
                self.row_state = *row_state;
            }
            SettingRowInput::ChangeActivityChanged(change_activity) => {
                self.row_state.change_activity = change_activity;
                if change_activity.accepts_changes() {
                    self.editor_draft = shown_value_or_text(&self.row_state.shown_value);
                }
            }
            SettingRowInput::DraftEdited(draft_edit) => self.edit_draft(draft_edit, &sender),
            SettingRowInput::ApplyRequested => self.report_draft(&sender),
            SettingRowInput::DraftEditedAndApplied(draft_edit) => {
                self.editor_draft = self.editor_draft.clone().with_draft_edit(draft_edit);
                self.report_draft(&sender);
            }
            SettingRowInput::FollowUpPressed(follow_up_action) => {
                let _parent_closed = sender.output(SettingRowOutput::FollowUpChosen {
                    setting_subject: self.row_state.setting_subject.clone(),
                    shown_value: self.row_state.shown_value.clone(),
                    follow_up_action,
                });
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: FactorySender<Self>) {
        let moment_formatters = self.moment_formatters;
        self.replace_editor_when_kind_changed(widgets, &sender);
        self.show_draft_in_editor(widgets);
        if follow_up_actions(&self.row_state.setting_subject, &self.row_state.shown_value)
            != widgets.shown_actions
        {
            self.rebuild_follow_up_buttons(widgets, &sender);
        }
        widgets.expander_row.set_subtitle(&self.header_subtitle());
        match self.source_icon_name() {
            Some(icon_name) => {
                widgets.source_icon.set_icon_name(Some(icon_name));
                widgets.source_icon.set_visible(true);
            }
            None => widgets.source_icon.set_visible(false),
        }
        if source_texts(moment_formatters, &self.row_state.setting_subject).source_emphasis
            == SourceEmphasis::NeedsAttention
        {
            widgets.source_icon.add_css_class("warning");
            widgets.source_icon.remove_css_class("dim-label");
        } else {
            widgets.source_icon.remove_css_class("warning");
            widgets.source_icon.add_css_class("dim-label");
        }
        let explanation = explanation_text(moment_formatters, &self.row_state);
        widgets.explanation_label.set_label(&explanation);
        widgets
            .expander_row
            .upcast_ref::<gtk::Widget>()
            .update_property(&[gtk::accessible::Property::Description(&explanation)]);
        widgets
            .expander_row
            .set_enable_expansion(!explanation.is_empty() || !widgets.shown_actions.is_empty());
        let needs_attention =
            row_needs_attention(moment_formatters, &self.row_state.setting_subject);
        if needs_attention && !widgets.attention_shown {
            widgets.expander_row.set_expanded(true);
        }
        widgets.attention_shown = needs_attention;
        let applying_here = self.row_state.change_activity.shows_applying_spinner();
        widgets.applying_spinner.set_visible(applying_here);
        widgets.editor_slot.set_visible(!applying_here);
        widgets
            .editor_slot
            .set_sensitive(self.row_state.change_activity.accepts_changes());
        widgets
            .action_box
            .set_sensitive(self.row_state.change_activity.accepts_changes());
    }
}
