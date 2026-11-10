use ksni::{TrayMethods, menu::StandardItem};

use crate::{
    APPLICATION_ID, TRAY_ATTENTION_ICON_NAME, TRAY_ICON_NAME,
    application_window::application_window_shell::ApplicationWindowShellInput,
    findings::detections_page::detection_alerts::OPEN_DETECTIONS_ACTION,
    host_services::desktop::running_instance_forwarding::SHOW_WINDOW_ACTION, translated_messages,
    translated_messages::MessageCatalog,
};

pub struct TrayMenuLabels {
    show_window_label: String,
    show_detections_label: String,
    quick_scan_label: String,
    update_definitions_label: String,
    quit_label: String,
}

impl From<&MessageCatalog> for TrayMenuLabels {
    fn from(message_catalog: &MessageCatalog) -> Self {
        Self {
            show_window_label: translated_messages::window::show_lazaret(message_catalog),
            show_detections_label: translated_messages::window::show_detections(message_catalog),
            quick_scan_label: translated_messages::window::quick_scan(message_catalog),
            update_definitions_label: translated_messages::window::update_definitions(
                message_catalog,
            ),
            quit_label: translated_messages::window::quit_tray_item(message_catalog),
        }
    }
}

pub struct SystemTrayMenu {
    menu_labels: TrayMenuLabels,
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
    menu_labels: TrayMenuLabels,
    shell_sender: relm4::Sender<ApplicationWindowShellInput>,
) -> Result<ksni::Handle<SystemTrayMenu>, ksni::Error> {
    SystemTrayMenu {
        menu_labels,
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
                self.menu_labels.show_window_label.clone(),
                SHOW_WINDOW_ACTION,
                TrayClickEffect::PresentsWindow,
            ),
            tray_menu_item(
                self.menu_labels.show_detections_label.clone(),
                OPEN_DETECTIONS_ACTION,
                TrayClickEffect::PresentsWindow,
            ),
            tray_menu_item(
                self.menu_labels.quick_scan_label.clone(),
                "quick-scan",
                TrayClickEffect::RunsInBackground,
            ),
            tray_menu_item(
                self.menu_labels.update_definitions_label.clone(),
                "update-definitions",
                TrayClickEffect::RunsInBackground,
            ),
            ksni::MenuItem::Separator,
            tray_menu_item(
                self.menu_labels.quit_label.clone(),
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
        let tray_handle = spawn_system_tray(
            TrayMenuLabels::from(&crate::translated_messages::english_catalog()),
            shell_sender,
        )
        .await;
        println!(
            "tray spawn: {:?}",
            tray_handle.as_ref().map(|_handle| "registered")
        );
        assert!(tray_handle.is_ok());
    }
}
