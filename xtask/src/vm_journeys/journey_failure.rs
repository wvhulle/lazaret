use std::{io, process::ExitStatus};

use gtk_scene_capture::scene_capture_failure::SceneCaptureFailure;

#[derive(Debug, thiserror::Error)]
pub enum JourneyFailure {
    #[error("{program} could not be started: {start_failure}")]
    CommandNotStarted {
        program: String,
        start_failure: io::Error,
    },
    #[error("`{command_line}` failed with {exit_status}:\n{command_output}")]
    CommandFailed {
        command_line: String,
        exit_status: ExitStatus,
        command_output: String,
    },
    #[error("the widget could not be used: {0}")]
    WidgetUnavailable(#[from] SceneCaptureFailure),
    #[error("{0} never happened")]
    ConditionTimedOut(String),
    #[error("{0}")]
    ExpectationFailed(String),
    #[error("the Lazaret database could not be read: {0}")]
    DatabaseUnreadable(#[from] rusqlite::Error),
    #[error("file operation failed: {0}")]
    FileOperationFailed(#[from] io::Error),
    #[error("JSON could not be read: {0}")]
    JsonUnreadable(#[from] serde_json::Error),
    #[error("a D-Bus call failed: {0}")]
    BusCallFailed(#[from] zbus::Error),
}

pub fn ensure(
    condition_holds: bool,
    failure_text: impl FnOnce() -> String,
) -> Result<(), JourneyFailure> {
    if condition_holds {
        Ok(())
    } else {
        Err(JourneyFailure::ExpectationFailed(failure_text()))
    }
}
