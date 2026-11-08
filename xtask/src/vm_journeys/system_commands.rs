use std::{
    process::{Command, ExitStatus, Stdio},
    thread,
    time::{Duration, Instant},
};

use crate::vm_journeys::journey_failure::JourneyFailure;

const POLL_PAUSE: Duration = Duration::from_millis(500);

pub struct CommandOutcome {
    pub exit_status: ExitStatus,
    pub combined_output: String,
}

fn command_line_text(program: &str, arguments: &[&str]) -> String {
    std::iter::once(program)
        .chain(arguments.iter().copied())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn run_command(program: &str, arguments: &[&str]) -> Result<CommandOutcome, JourneyFailure> {
    let command_result = Command::new(program)
        .args(arguments)
        .stdin(Stdio::null())
        .output()
        .map_err(|start_failure| JourneyFailure::CommandNotStarted {
            program: program.to_owned(),
            start_failure,
        })?;
    Ok(CommandOutcome {
        exit_status: command_result.status,
        combined_output: format!(
            "{}{}",
            String::from_utf8_lossy(&command_result.stdout),
            String::from_utf8_lossy(&command_result.stderr)
        ),
    })
}

pub fn succeed(program: &str, arguments: &[&str]) -> Result<String, JourneyFailure> {
    let command_outcome = run_command(program, arguments)?;
    if command_outcome.exit_status.success() {
        Ok(command_outcome.combined_output)
    } else {
        Err(JourneyFailure::CommandFailed {
            command_line: command_line_text(program, arguments),
            exit_status: command_outcome.exit_status,
            command_output: command_outcome.combined_output,
        })
    }
}

pub fn as_root(arguments: &[&str]) -> Result<String, JourneyFailure> {
    let sudo_arguments = std::iter::once("-n")
        .chain(arguments.iter().copied())
        .collect::<Vec<_>>();
    succeed("sudo", &sudo_arguments)
}

pub fn as_root_outcome(arguments: &[&str]) -> Result<CommandOutcome, JourneyFailure> {
    let sudo_arguments = std::iter::once("-n")
        .chain(arguments.iter().copied())
        .collect::<Vec<_>>();
    run_command("sudo", &sudo_arguments)
}

pub fn root_file_text(file_path: &str) -> Result<String, JourneyFailure> {
    as_root(&["cat", file_path])
}

pub fn root_path_exists(file_path: &str) -> Result<bool, JourneyFailure> {
    Ok(as_root_outcome(&["test", "-e", file_path])?
        .exit_status
        .success())
}

pub fn write_root_file(file_path: &str, file_text: &str) -> Result<(), JourneyFailure> {
    let mut tee_process = Command::new("sudo")
        .args(["-n", "tee", file_path])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .map_err(|start_failure| JourneyFailure::CommandNotStarted {
            program: "sudo".to_owned(),
            start_failure,
        })?;
    if let Some(mut tee_input) = tee_process.stdin.take() {
        std::io::Write::write_all(&mut tee_input, file_text.as_bytes())?;
    }
    let exit_status = tee_process.wait()?;
    if exit_status.success() {
        Ok(())
    } else {
        Err(JourneyFailure::CommandFailed {
            command_line: format!("sudo tee {file_path}"),
            exit_status,
            command_output: String::new(),
        })
    }
}

pub fn wait_until(
    awaited_condition: &str,
    timeout: Duration,
    mut condition_holds: impl FnMut() -> Result<bool, JourneyFailure>,
) -> Result<(), JourneyFailure> {
    let deadline = Instant::now() + timeout;
    loop {
        if condition_holds()? {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(JourneyFailure::ConditionTimedOut(
                awaited_condition.to_owned(),
            ));
        }
        thread::sleep(POLL_PAUSE);
    }
}

pub fn settle(settle_duration: Duration) {
    thread::sleep(settle_duration);
}

pub fn journal_text(arguments: &[&str]) -> Result<String, JourneyFailure> {
    let journal_arguments = ["--no-pager", "--output=cat"]
        .into_iter()
        .chain(arguments.iter().copied())
        .collect::<Vec<_>>();
    succeed("journalctl", &journal_arguments)
}

pub fn unit_journal_lines_mentioning(
    unit_name: &str,
    mentioned_text: &str,
) -> Result<Vec<String>, JourneyFailure> {
    Ok(journal_text(&["-u", unit_name])?
        .lines()
        .filter(|journal_line| journal_line.contains(mentioned_text))
        .map(str::to_owned)
        .collect())
}
