mod harness;
mod journeys;
mod remote_nodes;

use clap::ValueEnum;
pub use remote_nodes::{
    fake_internet_server::{FakeInternetFailure, FakeInternetServerOptions, serve_fake_internet},
    team_share_server::{TeamSharePlacementOptions, serve_team_share_placement},
};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum JourneyName {
    DesktopUserJourney,
    AdministratorJourney,
}

pub fn run_vm_journey(journey_name: JourneyName) {
    harness::journey_progress::record_evidence_on_failure();
    match journey_name {
        JourneyName::DesktopUserJourney => {
            journeys::desktop_user_journey::run_desktop_user_journey();
        }
        JourneyName::AdministratorJourney => {
            journeys::administrator_journey::run_administrator_journey();
        }
    }
    eprintln!("every step of {journey_name:?} passed");
}

pub fn describe_session_widgets() -> Vec<String> {
    harness::journey_progress::visible_widget_descriptions()
}
