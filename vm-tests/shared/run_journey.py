from datetime import timedelta

JOURNEY_ENVIRONMENT = (
    "XDG_RUNTIME_DIR=/run/user/1000 "
    "DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus "
    "SWAYSOCK=/tmp/sway-ipc.sock "
    "WAYLAND_DISPLAY=wayland-1 "
    "YDOTOOL_SOCKET=/run/ydotoold/socket "
    "PATH=/run/wrappers/bin:/run/current-system/sw/bin"
)
ARTIFACTS_DIRECTORY = "/tmp/vm-journey-artifacts"


def as_sample_user(command):
    return f"runuser -u sample-user -- env {JOURNEY_ENVIRONMENT} {command}"


def run_journey(journey_name):
    machine.start()
    machine.wait_for_unit("multi-user.target")
    machine.wait_until_succeeds(as_sample_user("swaymsg -t get_outputs"), timeout=timedelta(seconds=120))
    machine.wait_until_succeeds(
        as_sample_user("systemctl --user is-active graphical-session.target"), timeout=timedelta(seconds=60)
    )
    machine.succeed(f"install -d -o sample-user {ARTIFACTS_DIRECTORY}")
    with subtest(journey_name):
        journey_status, _journey_output = machine.execute(
            as_sample_user(f"systemd-cat --identifier=vm-journey xtask vm-journey {journey_name}"),
            timeout=5400,
        )
        machine.copy_from_machine(ARTIFACTS_DIRECTORY, "artifacts")
        assert journey_status == 0, f"{journey_name} failed; its steps are in the log under vm-journey"
