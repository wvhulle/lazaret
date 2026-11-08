import json
import shlex
import time
from datetime import timedelta

SAMPLE_USER = "sample-user"
SAMPLE_PASSWORD = "sample-password"
SESSION_BUS = "unix:path=/run/user/1000/bus"
DOWNLOADS = f"/home/{SAMPLE_USER}/Downloads"
LAZARET_DATABASE = f"/home/{SAMPLE_USER}/.local/share/lazaret/lazaret.db"
LAZARET_READ_QUERY = f"sqlite3 -readonly -cmd '.timeout 10000' {LAZARET_DATABASE}"
LAZARET_WRITE_QUERY = f"sqlite3 -cmd '.timeout 10000' {LAZARET_DATABASE}"


def user_command(command):
    quoted = shlex.quote(command)
    return (
        f"runuser -u {SAMPLE_USER} -- env XDG_RUNTIME_DIR=/run/user/1000 "
        f"DBUS_SESSION_BUS_ADDRESS={SESSION_BUS} SWAYSOCK=/tmp/sway-ipc.sock "
        f"sh -c {quoted}"
    )


def as_user(command):
    return machine.succeed(user_command(command))


def as_user_status(command):
    return machine.execute(user_command(command))


def wait_as_user(command, timeout=60):
    return machine.wait_until_succeeds(user_command(command), timeout=timedelta(seconds=timeout))


def wait_for_desktop():
    machine.wait_for_unit("multi-user.target")
    wait_as_user("swaymsg -t get_outputs", timeout=120)
    wait_as_user("systemctl --user is-active graphical-session.target", timeout=60)


def wait_for_lazaret_service():
    wait_as_user("systemctl --user is-active lazaret.service", timeout=60)
    wait_as_user("busctl --user status page.codeberg.wvhulle.Lazaret", timeout=60)
    wait_as_user("busctl --user list --no-legend | grep -q StatusNotifierItem", timeout=60)
    time.sleep(2)


def scanner_lines_for(unit, file_path):
    journal = machine.succeed(f"journalctl -u {unit} --no-pager --output=cat")
    return [line for line in journal.splitlines() if file_path in line and line.endswith("FOUND")]


def assert_lines_stay_at(unit, file_path, settle_seconds):
    settled_count = len(scanner_lines_for(unit, file_path))
    time.sleep(settle_seconds)
    final_count = len(scanner_lines_for(unit, file_path))
    assert final_count == settled_count, (
        f"{file_path} kept producing detections: {settled_count} -> {final_count}"
    )


def stored_detection(file_path):
    query = (
        "select signature_name, detection_sources, sighting_count, resolution_kind "
        "from background_detections left join detection_resolutions "
        "using (file_path, signature_name) "
        f"where cast(file_path as text) = '{file_path}'"
    )
    return as_user(f"{LAZARET_READ_QUERY} {shlex.quote(query)}").strip()


def wait_for_stored_detection(file_path):
    wait_as_user(
        f"{LAZARET_READ_QUERY} "
        + shlex.quote(
            f"select 1 from background_detections where cast(file_path as text) = '{file_path}'"
        )
        + " | grep -q 1"
    )


def listed_notifications():
    listed = json.loads(as_user("makoctl list -j"))
    if isinstance(listed, dict):
        return [notification for group in listed.get("data", []) for notification in group]
    return listed


def notification_text(notification):
    def text_of(field):
        field_value = notification.get(field, "")
        return field_value.get("data", "") if isinstance(field_value, dict) else field_value

    return f"{text_of('summary')} {text_of('body')}"


def notifications_mentioning(expected_text):
    return [
        notification
        for notification in listed_notifications()
        if expected_text in notification_text(notification)
    ]


def wait_for_notification(expected_text, timeout=60):
    started_waiting = time.monotonic()
    while time.monotonic() - started_waiting < timeout:
        matching = notifications_mentioning(expected_text)
        if matching:
            return matching
        time.sleep(1)
    raise AssertionError(
        f"no notification mentioning {expected_text!r}: {listed_notifications()}"
    )


def wait_for_notification_gone(expected_text, timeout=30):
    started_waiting = time.monotonic()
    while time.monotonic() - started_waiting < timeout:
        if not notifications_mentioning(expected_text):
            return
        time.sleep(1)
    raise AssertionError(f"a notification mentioning {expected_text!r} was never withdrawn")


def lazaret_window():
    tree = json.loads(as_user("swaymsg -t get_tree"))
    pending = [tree]
    while pending:
        node = pending.pop()
        if node.get("app_id") == "page.codeberg.wvhulle.Lazaret":
            return node
        pending += node.get("nodes", []) + node.get("floating_nodes", [])
    return None


def wait_for_lazaret_window(timeout=30):
    started_waiting = time.monotonic()
    while time.monotonic() - started_waiting < timeout:
        window = lazaret_window()
        if window is not None and window.get("visible"):
            return window
        time.sleep(1)
    raise AssertionError("the Lazaret window never became visible")


def activate_application_action(action_name):
    as_user(
        "gdbus call --session --dest page.codeberg.wvhulle.Lazaret "
        "--object-path /page/codeberg/wvhulle/Lazaret "
        "--method org.gtk.Actions.Activate "
        f"{action_name} '[]' '{{}}'"
    )


def activate_window_action(action_name, action_target):
    as_user(
        "gdbus call --session --dest page.codeberg.wvhulle.Lazaret "
        "--object-path /page/codeberg/wvhulle/Lazaret/window/1 "
        "--method org.gtk.Actions.Activate "
        f"{action_name} \"[<'{action_target}'>]\" '{{}}'"
    )


def tray_status():
    names = as_user("busctl --user list --no-legend")
    tray_names = [
        line.split()[0] for line in names.splitlines() if "StatusNotifierItem" in line.split()[0]
    ]
    assert tray_names, f"Lazaret registered no tray item:\n{names}"
    status = as_user(
        f"busctl --user get-property {tray_names[0]} /StatusNotifierItem "
        "org.kde.StatusNotifierItem Status"
    )
    return status.split('"')[1]


def wait_for_tray_status(expected_status, timeout=30):
    started_waiting = time.monotonic()
    while time.monotonic() - started_waiting < timeout:
        if tray_status() == expected_status:
            return
        time.sleep(1)
    raise AssertionError(f"tray status stayed {tray_status()}, expected {expected_status}")


FAKE_INTERNET_DIRECTORY = "/var/lib/fake-internet"
SAMPLES_DIRECTORY = "/etc/lazaret-test/samples"
DOCUMENTS = f"/home/{SAMPLE_USER}/Documents"
QUARANTINE_DIRECTORY = f"/home/{SAMPLE_USER}/.local/share/lazaret/quarantine"


class WidgetQuery:
    def __init__(self, role, name, name_match="exact", inside=None, enclosing_role=None):
        self.role = role
        self.name = name
        self.name_match = name_match
        self.inside = inside
        self.enclosing_role = enclosing_role

    def arguments(self, timeout):
        query_arguments = (
            f"--role {self.role} --name {shlex.quote(self.name)} "
            f"--name-match {self.name_match} --timeout-seconds {timeout}"
        )
        if self.inside is not None:
            anchor_role, anchor_name = self.inside
            query_arguments += f" --inside-role {anchor_role} --inside-name {shlex.quote(anchor_name)}"
        if self.enclosing_role is not None:
            query_arguments += f" --enclosing-role {self.enclosing_role}"
        return query_arguments

    def __repr__(self):
        return f"{self.role} {self.name!r} ({self.name_match}) inside {self.inside} enclosed by {self.enclosing_role}"


def button(name, inside=None, enclosing_role=None):
    return WidgetQuery("button", name, inside=inside, enclosing_role=enclosing_role)


def label_containing(text):
    return WidgetQuery("label", text, name_match="contained")


def in_alert(heading):
    return ("alert", heading)


def quarantine_entry_part(name, entry_title):
    return button(name, inside=("list-item", entry_title), enclosing_role="list-item")


def report_visible_widgets(screenshot_name):
    machine.screenshot(screenshot_name)
    _status, widget_listing = as_user_status("xtask describe-session-widgets")
    print(f"visible widgets:\n{widget_listing}")


def run_widget_command(command_name, widget_query, timeout, extra_arguments=""):
    return as_user_status(f"xtask {command_name} {widget_query.arguments(timeout)} {extra_arguments}")


def require_widget_command(command_name, widget_query, timeout, extra_arguments=""):
    command_status, command_output = run_widget_command(command_name, widget_query, timeout, extra_arguments)
    if command_status != 0:
        report_visible_widgets(f"{command_name}_failed")
        raise AssertionError(f"{command_name} {widget_query} failed: {command_output}")


def await_widget(widget_query, timeout=30):
    require_widget_command("await-widget", widget_query, timeout)


def await_widget_gone(widget_query, timeout=30):
    require_widget_command("await-widget-gone", widget_query, timeout)


def widget_is_shown(widget_query):
    shown_status, _shown_output = run_widget_command("await-widget", widget_query, timeout=0)
    return shown_status == 0


def press(widget_query, timeout=30):
    require_widget_command("press-widget", widget_query, timeout)
    if widget_query.inside is not None and widget_query.inside[0] == "alert":
        await_widget_gone(WidgetQuery("alert", widget_query.inside[1]), timeout=timeout)


def type_into(widget_query, typed_text, timeout=30):
    require_widget_command("type-into-widget", widget_query, timeout, f"--text {shlex.quote(typed_text)}")


def await_label(text, timeout=30):
    await_widget(label_containing(text), timeout=timeout)


def assert_no_label(text):
    assert not widget_is_shown(label_containing(text)), f"a label containing {text!r} is shown"


PASSWORD_PROMPT_TITLE = "Authenticat"


def sway_window_titles():
    tree = json.loads(as_user("swaymsg -t get_tree"))
    pending = [tree]
    window_titles = []
    while pending:
        node = pending.pop()
        if node.get("pid") and node.get("name"):
            window_titles.append(node["name"])
        pending += node.get("nodes", []) + node.get("floating_nodes", [])
    return window_titles


def focused_window_title():
    tree = json.loads(as_user("swaymsg -t get_tree"))
    pending = [tree]
    while pending:
        node = pending.pop()
        if node.get("focused") and node.get("pid"):
            return node.get("name") or ""
        pending += node.get("nodes", []) + node.get("floating_nodes", [])
    return ""


def wait_for_window_titled(title_fragment, timeout=30):
    started_waiting = time.monotonic()
    while time.monotonic() - started_waiting < timeout:
        if title_fragment in focused_window_title():
            time.sleep(1)
            return
        time.sleep(0.5)
    raise AssertionError(f"no focused window titled {title_fragment!r}: {sway_window_titles()}")


def wait_for_window_closed(title_fragment, timeout=30):
    started_waiting = time.monotonic()
    while time.monotonic() - started_waiting < timeout:
        if not any(title_fragment in window_title for window_title in sway_window_titles()):
            return
        time.sleep(0.5)
    raise AssertionError(f"the window titled {title_fragment!r} stayed open: {sway_window_titles()}")


def choose_file_in_dialog(dialog_title, chosen_path):
    wait_for_window_titled(dialog_title)
    machine.send_key("ctrl-l")
    machine.send_chars(chosen_path)
    for _confirmation_attempt in range(3):
        time.sleep(1)
        machine.send_key("ret")
        if not any(dialog_title in window_title for window_title in sway_window_titles()):
            return
        time.sleep(2)
    wait_for_window_closed(dialog_title, timeout=5)


def password_prompt_is_shown():
    return any(PASSWORD_PROMPT_TITLE in window_title for window_title in sway_window_titles())


def wait_for_password_prompt(expected_shown, timeout=60):
    started_waiting = time.monotonic()
    while time.monotonic() - started_waiting < timeout:
        if password_prompt_is_shown() == expected_shown:
            return True
        time.sleep(0.5)
    return False


def answer_password_prompt(password=SAMPLE_PASSWORD, timeout=60):
    assert wait_for_password_prompt(True, timeout), f"no password prompt appeared: {sway_window_titles()}"
    time.sleep(1)
    machine.send_chars(f"{password}\n")


def answer_every_password_prompt(password=SAMPLE_PASSWORD, follow_up_seconds=10):
    answer_password_prompt(password)
    while True:
        assert wait_for_password_prompt(False), "the password prompt never closed"
        if not wait_for_password_prompt(True, follow_up_seconds):
            return
        time.sleep(1)
        machine.send_chars(f"{password}\n")


def dismiss_password_prompt():
    assert wait_for_password_prompt(True), f"no password prompt appeared: {sway_window_titles()}"
    time.sleep(1)
    machine.send_key("esc")
    assert wait_for_password_prompt(False), "the password prompt stayed open"


def show_page(page_name):
    activate_application_action("show-window")
    wait_for_lazaret_window()
    activate_window_action("show-page", page_name)


def lazaret_rows(query):
    return [
        row_text.split("|")
        for row_text in as_user(
            f"{LAZARET_READ_QUERY} {shlex.quote(query)}"
        ).splitlines()
        if row_text
    ]


def wait_for_lazaret_rows(query, timeout=60):
    wait_as_user(
        f"test -n \"$({LAZARET_READ_QUERY} {shlex.quote(query)})\"", timeout=timeout
    )
    return lazaret_rows(query)


def sample_sha256(sample_name):
    for checksum_line in machine.succeed(f"cat {SAMPLES_DIRECTORY}/SHA256SUMS").splitlines():
        checksum, listed_name = checksum_line.split("  ", 1)
        if listed_name == sample_name:
            return checksum
    raise AssertionError(f"no fixture sample named {sample_name}")


def place_sample(sample_name, destination_path):
    destination_directory = destination_path.rsplit("/", 1)[0]
    as_user(f"mkdir -p {shlex.quote(destination_directory)}")
    as_user(f"cp {shlex.quote(f'{SAMPLES_DIRECTORY}/{sample_name}')} {shlex.quote(destination_path)}")
    as_user(f"chmod 0640 {shlex.quote(destination_path)}")


def scan_with_lazaret(*scanned_paths):
    as_user("lazaret " + " ".join(shlex.quote(scanned_path) for scanned_path in scanned_paths))
    wait_for_lazaret_window()


def fake_internet_requests(route_name=None):
    request_log = machine.succeed(f"cat {FAKE_INTERNET_DIRECTORY}/requests.jsonl 2>/dev/null || true")
    logged_requests = [json.loads(logged_line) for logged_line in request_log.splitlines() if logged_line]
    if route_name is None:
        return logged_requests
    return [logged_request for logged_request in logged_requests if logged_request["route_name"] == route_name]


def wait_for_fake_internet_request(route_name, previous_count=0, timeout=60):
    started_waiting = time.monotonic()
    while time.monotonic() - started_waiting < timeout:
        routed_requests = fake_internet_requests(route_name)
        if len(routed_requests) > previous_count:
            return routed_requests[previous_count:]
        time.sleep(1)
    raise AssertionError(f"the fake internet never received a {route_name} request: {fake_internet_requests()}")


def set_fake_internet_scenario(**route_overrides):
    scenario_text = json.dumps(route_overrides)
    machine.succeed(f"printf %s {shlex.quote(scenario_text)} > {FAKE_INTERNET_DIRECTORY}/scenario.json")


def wait_for_clamd_ready():
    machine.succeed("systemctl start clamav-daemon.service")
    machine.wait_for_unit("clamav-daemon.service")
    machine.wait_until_succeeds("clamdscan --ping 1", timeout=timedelta(seconds=120))


def clamdscan_reports(file_path):
    _scan_status, scan_output = machine.execute(f"clamdscan --no-summary --fdpass {shlex.quote(file_path)}")
    return scan_output


def wait_for_root_scan_verdict(file_path, expect_found, timeout=60):
    started_waiting = time.monotonic()
    while time.monotonic() - started_waiting < timeout:
        if ("FOUND" in clamdscan_reports(file_path)) == expect_found:
            return
        time.sleep(1)
    raise AssertionError(f"clamd kept reporting {clamdscan_reports(file_path)!r} for {file_path}")


def assert_request_count_stays(route_name, expected_count, settle_seconds=5):
    time.sleep(settle_seconds)
    routed_count = len(fake_internet_requests(route_name))
    assert routed_count == expected_count, f"{route_name} was requested {routed_count} times, expected {expected_count}"


deferred_product_failures = []


def expect_behaviour(behaviour_description, behaviour_check):
    try:
        behaviour_check()
    except AssertionError as product_failure:
        machine.screenshot(f"product_failure_{len(deferred_product_failures)}")
        deferred_product_failures.append(f"{behaviour_description}: {product_failure}")
        print(f"!!! product behaviour failed, continuing: {behaviour_description}: {product_failure}")


def require(condition_holds, failure_text):
    assert condition_holds, failure_text


def report_deferred_product_failures():
    assert not deferred_product_failures, "product behaviour failed:\n" + "\n".join(deferred_product_failures)
