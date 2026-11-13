use std::{
    io::Write,
    process::{Command, ExitStatus, Stdio},
    thread,
    time::{Duration, Instant},
};

const POLL_PAUSE: Duration = Duration::from_millis(300);

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

pub fn run_command(program: &str, arguments: &[&str]) -> CommandOutcome {
    let command_output = Command::new(program)
        .args(arguments)
        .stdin(Stdio::null())
        .output()
        .unwrap_or_else(|start_failure| panic!("{program} could not be started: {start_failure}"));
    CommandOutcome {
        exit_status: command_output.status,
        combined_output: format!(
            "{}{}",
            String::from_utf8_lossy(&command_output.stdout),
            String::from_utf8_lossy(&command_output.stderr)
        ),
    }
}

pub fn command_succeeds(program: &str, arguments: &[&str]) -> bool {
    run_command(program, arguments).exit_status.success()
}

pub fn succeed(program: &str, arguments: &[&str]) -> String {
    let command_outcome = run_command(program, arguments);
    assert!(
        command_outcome.exit_status.success(),
        "`{}` failed with {}:\n{}",
        command_line_text(program, arguments),
        command_outcome.exit_status,
        command_outcome.combined_output
    );
    command_outcome.combined_output
}

fn sudo_arguments<'argument>(arguments: &[&'argument str]) -> Vec<&'argument str> {
    std::iter::once("-n")
        .chain(arguments.iter().copied())
        .collect()
}

pub fn as_root(arguments: &[&str]) -> String {
    succeed("sudo", &sudo_arguments(arguments))
}

pub fn as_root_outcome(arguments: &[&str]) -> CommandOutcome {
    run_command("sudo", &sudo_arguments(arguments))
}

pub fn root_file_text(file_path: &str) -> String {
    as_root(&["cat", file_path])
}

pub fn root_file_text_or_empty(file_path: &str) -> String {
    let reading_outcome = as_root_outcome(&["cat", file_path]);
    if reading_outcome.exit_status.success() {
        reading_outcome.combined_output
    } else {
        String::new()
    }
}

pub fn root_path_exists(file_path: &str) -> bool {
    as_root_outcome(&["test", "-e", file_path])
        .exit_status
        .success()
}

pub fn write_root_file(file_path: &str, file_text: &str) {
    let mut tee_process = Command::new("sudo")
        .args(["-n", "tee", file_path])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .unwrap_or_else(|start_failure| panic!("sudo could not be started: {start_failure}"));
    if let Some(mut tee_input) = tee_process.stdin.take() {
        tee_input
            .write_all(file_text.as_bytes())
            .unwrap_or_else(|write_failure| panic!("writing {file_path} failed: {write_failure}"));
    }
    let exit_status = tee_process
        .wait()
        .unwrap_or_else(|wait_failure| panic!("sudo tee {file_path} vanished: {wait_failure}"));
    assert!(
        exit_status.success(),
        "`sudo tee {file_path}` failed with {exit_status}"
    );
}

pub fn poll_within<Found>(
    timeout: Duration,
    mut attempt: impl FnMut() -> Option<Found>,
) -> Option<Found> {
    let deadline = Instant::now() + timeout;
    loop {
        if let Some(found) = attempt() {
            return Some(found);
        }
        if Instant::now() >= deadline {
            return None;
        }
        thread::sleep(POLL_PAUSE);
    }
}

pub fn poll_until<Found>(
    awaited_condition: &str,
    timeout: Duration,
    attempt: impl FnMut() -> Option<Found>,
) -> Found {
    poll_within(timeout, attempt).unwrap_or_else(|| {
        panic!(
            "{awaited_condition} never happened within {}s",
            timeout.as_secs()
        )
    })
}

pub fn holds_within(timeout: Duration, mut condition_holds: impl FnMut() -> bool) -> bool {
    poll_within(timeout, || condition_holds().then_some(())).is_some()
}

pub fn wait_until(
    awaited_condition: &str,
    timeout: Duration,
    mut condition_holds: impl FnMut() -> bool,
) {
    poll_until(awaited_condition, timeout, || {
        condition_holds().then_some(())
    });
}

pub fn settle(settle_duration: Duration) {
    thread::sleep(settle_duration);
}

pub fn journal_text(arguments: &[&str]) -> String {
    let journal_arguments = ["--no-pager", "--output=cat"]
        .into_iter()
        .chain(arguments.iter().copied())
        .collect::<Vec<_>>();
    succeed("journalctl", &journal_arguments)
}

pub fn unit_journal_lines_mentioning(unit_name: &str, mentioned_text: &str) -> Vec<String> {
    journal_text(&["-u", unit_name])
        .lines()
        .filter(|journal_line| journal_line.contains(mentioned_text))
        .map(str::to_owned)
        .collect()
}
