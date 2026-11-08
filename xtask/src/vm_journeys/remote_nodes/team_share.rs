use crate::vm_journeys::harness::{
    fixture_corpus::FixtureSample, journey_failure::JourneyFailure, system_commands::succeed,
};

const PLACEMENT_ADDRESS: &str = "http://fileserver:8081/place";
pub const TEAM_SHARE_MOUNT_POINT: &str = "/srv/team-share";

pub fn colleague_places_on_team_share(
    fixture_sample: FixtureSample,
) -> Result<String, JourneyFailure> {
    succeed(
        "curl",
        &[
            "--fail",
            "--silent",
            "--show-error",
            "--request",
            "PUT",
            "--data-binary",
            fixture_sample.file_name(),
            PLACEMENT_ADDRESS,
        ],
    )?;
    Ok(format!(
        "{TEAM_SHARE_MOUNT_POINT}/{}",
        fixture_sample.file_name()
    ))
}
