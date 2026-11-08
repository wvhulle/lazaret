mod harness;
mod journeys;
mod remote_nodes;

use clap::ValueEnum;
pub use harness::journey_failure::JourneyFailure;
pub use remote_nodes::{
    fake_internet_server::{FakeInternetFailure, FakeInternetServerOptions, serve_fake_internet},
    team_share_server::{TeamSharePlacementOptions, serve_team_share_placement},
};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum JourneyName {
    DesktopUserJourney,
    AdministratorJourney,
}

pub fn run_vm_journey(journey_name: JourneyName) -> Result<(), JourneyFailure> {
    match journey_name {
        JourneyName::DesktopUserJourney => {
            journeys::desktop_user_journey::run_desktop_user_journey()
        }
        JourneyName::AdministratorJourney => {
            journeys::administrator_journey::run_administrator_journey()
        }
    }
}

pub fn describe_session_widgets() -> Result<Vec<String>, JourneyFailure> {
    Ok(harness::session_widgets::SessionWidgets::connect()?.describe_shown_widgets())
}
