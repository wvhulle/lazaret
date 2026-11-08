mod administrator_journey;
mod desktop_environment;
mod desktop_user_journey;
mod fake_internet;
mod fixture_corpus;
mod journey_failure;
mod journey_run;
mod lazaret_database;
mod session_widgets;
mod system_commands;
mod ui_messages;

use clap::ValueEnum;
pub use journey_failure::JourneyFailure;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum JourneyName {
    DesktopUserJourney,
    AdministratorJourney,
}

pub fn run_vm_journey(journey_name: JourneyName) -> Result<(), JourneyFailure> {
    match journey_name {
        JourneyName::DesktopUserJourney => desktop_user_journey::run_desktop_user_journey(),
        JourneyName::AdministratorJourney => administrator_journey::run_administrator_journey(),
    }
}

pub fn describe_session_widgets() -> Result<Vec<String>, JourneyFailure> {
    Ok(session_widgets::SessionWidgets::connect()?.describe_shown_widgets())
}
