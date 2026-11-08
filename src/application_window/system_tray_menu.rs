use ksni::{TrayMethods, menu::StandardItem};

use crate::{
    APPLICATION_ID, TRAY_ATTENTION_ICON_NAME, TRAY_ICON_NAME,
    application_window::application_window_shell::ApplicationWindowShellInput,
    findings::detections_page::detection_alerts::OPEN_DETECTIONS_ACTION,
    host_services::desktop::running_instance_forwarding::SHOW_WINDOW_ACTION, translated_messages,
};

pub struct SystemTrayMenu {
    pub unresolved_detections: usize,
    shell_sender: relm4::Sender<ApplicationWindowShellInput>,
}

#[derive(Clone, Copy)]
enum TrayClickEffect {
    PresentsWindow,
    RunsInBackground,
}

impl SystemTrayMenu {
    fn forward_tray_click(&self, action_name: &'static str, click_effect: TrayClickEffect) {
        if let TrayClickEffect::PresentsWindow = click_effect
            && let Some(activation_token) = ksni::current_xdg_activation_token()
        {
            self.shell_sender
                .emit(ApplicationWindowShellInput::ActivationTokenReceived(
                    activation_token,
                ));
        }
        self.shell_sender
            .emit(ApplicationWindowShellInput::TrayActionRequested(
                action_name,
            ));
    }
}

fn tray_menu_item(
    item_label: String,
    action_name: &'static str,
    click_effect: TrayClickEffect,
) -> ksni::MenuItem<SystemTrayMenu> {
    StandardItem {
        label: item_label,
        activate: Box::new(move |tray_menu: &mut SystemTrayMenu| {
            tray_menu.forward_tray_click(action_name, click_effect);
        }),
        ..Default::default()
    }
    .into()
}

pub async fn spawn_system_tray(
    shell_sender: relm4::Sender<ApplicationWindowShellInput>,
) -> Result<ksni::Handle<SystemTrayMenu>, ksni::Error> {
    SystemTrayMenu {
        unresolved_detections: 0,
        shell_sender,
    }
    .assume_sni_available(true)
    .spawn()
    .await
}

impl ksni::Tray for SystemTrayMenu {
    fn id(&self) -> String {
        APPLICATION_ID.to_owned()
    }

    fn title(&self) -> String {
        "Lazaret".to_owned()
    }

    fn icon_name(&self) -> String {
        TRAY_ICON_NAME.to_owned()
    }

    fn status(&self) -> ksni::Status {
        if self.unresolved_detections > 0 {
            ksni::Status::NeedsAttention
        } else {
            ksni::Status::Active
        }
    }

    fn attention_icon_name(&self) -> String {
        TRAY_ATTENTION_ICON_NAME.to_owned()
    }

    fn activate(&mut self, _horizontal: i32, _vertical: i32) {
        self.forward_tray_click(SHOW_WINDOW_ACTION, TrayClickEffect::PresentsWindow);
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        vec![
            tray_menu_item(
                translated_messages::window::show_lazaret(),
                SHOW_WINDOW_ACTION,
                TrayClickEffect::PresentsWindow,
            ),
            tray_menu_item(
                translated_messages::window::show_detections(),
                OPEN_DETECTIONS_ACTION,
                TrayClickEffect::PresentsWindow,
            ),
            tray_menu_item(
                translated_messages::window::quick_scan(),
                "quick-scan",
                TrayClickEffect::RunsInBackground,
            ),
            tray_menu_item(
                translated_messages::window::update_definitions(),
                "update-definitions",
                TrayClickEffect::RunsInBackground,
            ),
            ksni::MenuItem::Separator,
            tray_menu_item(
                translated_messages::window::quit_tray_item(),
                "quit",
                TrayClickEffect::RunsInBackground,
            ),
        ]
    }
}

#[cfg(test)]
mod system_tray_menu_tests {
    use super::*;

    #[tokio::test]
    #[ignore = "needs a real desktop session with a tray host"]
    async fn host_tray_registers_on_the_session_bus() {
        let (shell_sender, _shell_receiver) = relm4::channel::<ApplicationWindowShellInput>();
        let tray_handle = spawn_system_tray(shell_sender).await;
        println!(
            "tray spawn: {:?}",
            tray_handle.as_ref().map(|_handle| "registered")
        );
        assert!(tray_handle.is_ok());
    }
}
