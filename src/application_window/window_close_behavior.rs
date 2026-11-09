use relm4::{
    Sender, adw,
    adw::prelude::*,
    gtk::{gio, gio::prelude::SettingsExt},
};

use crate::{
    host_services::desktop::stored_settings::CLOSE_ACTION_KEY,
    shared_widgets::awaited_dialogs::{
        AlertChoice, add_alert_choice, add_dismissing_choice, alert_choice, deliver_local_outcome,
    },
    translated_messages,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CloseAction {
    QuitCompletely,
    HideToTray,
    AskOnNextClose,
}

pub const CLOSE_ACTION_CHOICES: [CloseAction; 3] = [
    CloseAction::QuitCompletely,
    CloseAction::HideToTray,
    CloseAction::AskOnNextClose,
];

const SCHEMA_DEFAULT_CLOSE_ACTION: CloseAction = CloseAction::AskOnNextClose;

impl CloseAction {
    pub const fn stored_text(self) -> &'static str {
        match self {
            Self::QuitCompletely => "quit",
            Self::HideToTray => "minimize",
            Self::AskOnNextClose => "ask",
        }
    }

    pub fn choice_label(self) -> String {
        match self {
            Self::QuitCompletely => translated_messages::window::quit_completely(),
            Self::HideToTray => translated_messages::window::hide_tray(),
            Self::AskOnNextClose => translated_messages::window::ask_on_next_close(),
        }
    }

    pub const fn choice_position(self) -> u32 {
        match self {
            Self::QuitCompletely => 0,
            Self::HideToTray => 1,
            Self::AskOnNextClose => 2,
        }
    }

    pub fn from_choice_position(choice_position: u32) -> Option<Self> {
        usize::try_from(choice_position)
            .ok()
            .and_then(|choice_index| CLOSE_ACTION_CHOICES.get(choice_index))
            .copied()
    }

    fn from_stored_text(stored_text: &str) -> Option<Self> {
        CLOSE_ACTION_CHOICES
            .into_iter()
            .find(|close_choice| close_choice.stored_text() == stored_text)
    }

    pub fn read_from(stored_settings: &gio::Settings) -> Self {
        Self::from_stored_text(&stored_settings.string(CLOSE_ACTION_KEY))
            .unwrap_or(SCHEMA_DEFAULT_CLOSE_ACTION)
    }

    pub fn store_into(self, stored_settings: &gio::Settings) {
        let _read_only_settings = stored_settings.set_string(CLOSE_ACTION_KEY, self.stored_text());
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CloseAnswer {
    CloseCancelled,
    QuitChosen,
    KeepRunningChosen,
}

impl CloseAnswer {
    pub const fn remembered_close_action(self) -> Option<CloseAction> {
        match self {
            Self::CloseCancelled => None,
            Self::QuitChosen => Some(CloseAction::QuitCompletely),
            Self::KeepRunningChosen => Some(CloseAction::HideToTray),
        }
    }
}

impl AlertChoice for CloseAnswer {
    const ANSWERING_CHOICES: &'static [Self] = &[Self::QuitChosen, Self::KeepRunningChosen];
    const DISMISSING_CHOICE: Self = Self::CloseCancelled;

    fn response_id(self) -> &'static str {
        match self {
            Self::CloseCancelled => "cancel",
            Self::QuitChosen => "quit",
            Self::KeepRunningChosen => "minimize",
        }
    }
}

pub fn present_close_confirmation<Message: 'static>(
    window_root: &adw::ApplicationWindow,
    answer_sender: Sender<Message>,
    answered_message: fn(CloseAnswer) -> Message,
) {
    let close_dialog = adw::AlertDialog::new(
        Some(&translated_messages::window::close_lazaret()),
        Some(&translated_messages::window::detection_alerts_stop_lazaret()),
    );
    add_dismissing_choice::<CloseAnswer>(
        &close_dialog,
        &translated_messages::window::cancel_close_button(),
    );
    add_alert_choice(
        &close_dialog,
        CloseAnswer::QuitChosen,
        &translated_messages::window::quit_close_button(),
    );
    add_alert_choice(
        &close_dialog,
        CloseAnswer::KeepRunningChosen,
        &translated_messages::window::keep_running(),
    );
    close_dialog.set_default_response(Some(CloseAnswer::KeepRunningChosen.response_id()));
    deliver_local_outcome(
        answer_sender,
        alert_choice(close_dialog, window_root.clone().upcast()),
        move |close_answer| Some(answered_message(close_answer)),
    );
}

#[cfg(test)]
mod close_action_tests {
    use super::*;
    use crate::shared_widgets::awaited_dialogs::alert_choice_for_response;

    #[test]
    fn every_close_action_sits_at_its_own_choice_position() {
        CLOSE_ACTION_CHOICES.into_iter().for_each(|close_action| {
            assert_eq!(
                CloseAction::from_choice_position(close_action.choice_position()),
                Some(close_action)
            );
        });
    }

    #[test]
    fn every_close_action_reads_back_from_its_stored_text() {
        CLOSE_ACTION_CHOICES.into_iter().for_each(|close_action| {
            assert_eq!(
                CloseAction::from_stored_text(close_action.stored_text()),
                Some(close_action)
            );
        });
    }

    #[test]
    fn positions_past_the_choices_name_no_close_action() {
        assert_eq!(
            CloseAction::from_choice_position(relm4::gtk::INVALID_LIST_POSITION),
            None
        );
    }

    #[test]
    fn close_answers_remember_the_chosen_close_action() {
        assert_eq!(
            CloseAnswer::QuitChosen.remembered_close_action(),
            Some(CloseAction::QuitCompletely)
        );
        assert_eq!(
            CloseAnswer::KeepRunningChosen.remembered_close_action(),
            Some(CloseAction::HideToTray)
        );
        assert_eq!(CloseAnswer::CloseCancelled.remembered_close_action(), None);
    }

    #[test]
    fn close_dialog_responses_parse_to_close_answers() {
        assert_eq!(
            alert_choice_for_response::<CloseAnswer>("quit"),
            CloseAnswer::QuitChosen
        );
        assert_eq!(
            alert_choice_for_response::<CloseAnswer>("minimize"),
            CloseAnswer::KeepRunningChosen
        );
        assert_eq!(
            alert_choice_for_response::<CloseAnswer>("close"),
            CloseAnswer::CloseCancelled
        );
    }
}
