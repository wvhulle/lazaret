SCAN_UNIT = "clamdscan.service"
SCANNED_FILE = "/var/lib/sample-scan/dropper.bin"
ENABLE_SCHEDULE_TITLE = "Enable Scheduled Scans"
APPLY_SCHEDULE_LABEL = "Apply"
TRUSTED_FILES_LIST = "/var/lib/clamav/lazaret-local.sfp"
IGNORED_SIGNATURES_LIST = "/var/lib/clamav/lazaret-local.ign2"
IGNORED_SIGNATURE = "Doc.Fixture.FalsePositive-1"
TRUST_CANDIDATE = f"{DOCUMENTS}/invoice.pdf.exe"
PROMOTED_CANDIDATE = f"{DOCUMENTS}/payroll-macro.docm"
ROOT_SCAN_COPY = "/root/invoice-copy.exe"


UNLOCK_LABEL = "Unlock"
CONFIRM_APPLY_LABEL = "Apply"
MAX_THREADS_TITLE = "MaxThreads"
COPY_DIALOG_NOTICE = "Nothing is applied"
COPY_LINE_LABEL = "Copy Line"
LIVE_CONFIG_DESCRIPTION = "combines your system configuration"
MERGED_CONFIG = "/run/lazaret/clamd.conf"
OVERRIDES_FILE = "/var/lib/lazaret/live-overrides.toml"
CHANGE_DOCUMENT_FILE = "/tmp/admin-service-change.toml"
USER_ANSWER_FILE = "/tmp/admin-service-answer"
USER_REQUEST_FILE = "/tmp/admin-service-request"


def change_document(key, toml_value):
    return f'change = "set-values"\n\n[setting_values]\n{key} = {toml_value}\n'


def reset_document(key):
    return f'change = "back-to-system"\nsetting_names = ["{key}"]\n'


def call_with_document(document):
    machine.succeed(f"printf %s {shlex.quote(document)} > {CHANGE_DOCUMENT_FILE}; chmod 644 {CHANGE_DOCUMENT_FILE}")
    return (
        "busctl --timeout=400 call page.codeberg.wvhulle.Lazaret.ClamavAdmin "
        "/page/codeberg/wvhulle/Lazaret/ClamavAdmin "
        "page.codeberg.wvhulle.Lazaret.ClamavAdmin ApplyChanges "
        f's "$(cat {CHANGE_DOCUMENT_FILE})" 2>&1'
    )


def apply_command(key, toml_value):
    return call_with_document(change_document(key, toml_value))


def apply_as_root(key, toml_value):
    reset_start_limits()
    return machine.execute(apply_command(key, toml_value))


def reset_as_root(key):
    reset_start_limits()
    return machine.execute(call_with_document(reset_document(key)))


def config_seen_by_clamd():
    clamd_pid = machine.succeed("systemctl show --property=MainPID --value clamav-daemon.service").strip()
    return machine.succeed(f"nsenter --target {clamd_pid} --mount cat /etc/clamav/clamd.conf")


def start_apply_as_user(key, toml_value):
    machine.succeed(f"rm -f {USER_ANSWER_FILE}")
    request_body = f"#!/bin/sh\n({apply_command(key, toml_value)}; echo finished) > {USER_ANSWER_FILE}\n"
    as_user(f"printf %s {shlex.quote(request_body)} > {USER_REQUEST_FILE}; chmod +x {USER_REQUEST_FILE}")
    as_user(f"swaymsg exec {USER_REQUEST_FILE}")


def wait_for_user_answer():
    machine.wait_until_succeeds(f"grep -q finished {USER_ANSWER_FILE}", timeout=timedelta(seconds=240))
    return machine.succeed(f"cat {USER_ANSWER_FILE}")


CHANGE_CONFIRMATION_HEADING = "Change this setting?"


def show_scan_engine_page():
    show_page("scan-engine")
    await_widget(WidgetQuery("list-item", MAX_THREADS_TITLE))


def enter_max_threads(new_value):
    machine.send_key("ctrl-a")
    machine.send_chars(f"{new_value}\n")


def copy_max_threads_line():
    press(button(MAX_THREADS_TITLE, inside=("list-item", MAX_THREADS_TITLE)))
    await_label(COPY_DIALOG_NOTICE)
    press(button(COPY_LINE_LABEL))


def edit_max_threads(new_value):
    press(WidgetQuery("list-item", MAX_THREADS_TITLE))
    enter_max_threads(new_value)
    await_widget(WidgetQuery("alert", CHANGE_CONFIRMATION_HEADING))


def wait_for_clamd():
    machine.wait_until_succeeds(f"clamdscan --config-file={MERGED_CONFIG} --ping 1", timeout=timedelta(seconds=120))


def reset_start_limits():
    machine.succeed("systemctl reset-failed clamav-daemon.service lazaret-settings-merge.service")


def merge_again():
    reset_start_limits()
    machine.succeed("systemctl restart lazaret-settings-merge.service")
    reset_start_limits()
    machine.succeed("systemctl start clamav-daemon.service")
    wait_for_clamd()



def root_scan_finds(file_path):
    return "FOUND" in machine.execute(f"clamdscan --no-summary --fdpass {shlex.quote(file_path)}")[1]


def quarantine_with_lazaret(file_path):
    scan_with_lazaret(file_path)
    press(button("Quarantine 1 Threat"), timeout=120)
    press(button("Quarantine", inside=in_alert("Move to Quarantine?")))
    machine.wait_until_fails(f"test -e {shlex.quote(file_path)}", timeout=timedelta(seconds=30))


def open_quarantined_entry(path_fragment):
    show_page("detections")
    show_page("quarantine")
    entry_title = wait_for_lazaret_rows(
        "select signature_name from quarantine join quarantine_signatures using (entry_id) "
        f"where cast(original_path as text) like '%{path_fragment}%'"
    )[0][0]
    entry_trust_button = quarantine_entry_part("Trust…", entry_title)
    if not widget_is_shown(entry_trust_button):
        press(WidgetQuery("list-item", entry_title))
    await_widget(entry_trust_button)
    return entry_title


def ask_to_trust_for_all_users(path_fragment):
    entry_title = open_quarantined_entry(path_fragment)
    press(quarantine_entry_part("Trust…", entry_title))
    press(button("Trust for All Users…", inside=in_alert("Trust This File?")))


machine.start()
wait_for_desktop()
machine.wait_for_unit("lazaret-settings-merge.service")
wait_for_clamd_ready()
wait_for_clamd()
wait_for_lazaret_service()
EICAR_SHA256 = sample_sha256("invoice.pdf.exe")
FALSE_POSITIVE_SHA256 = sample_sha256("payroll-macro.docm")
machine.succeed(f"cp {SAMPLES_DIRECTORY}/invoice.pdf.exe {ROOT_SCAN_COPY}")

with subtest("an allowlist entry for all users is listed with its warning, next to a foreign allowlist"):
    machine.succeed(f"cp {SAMPLES_DIRECTORY}/payroll-macro.docm /root/payroll-copy.docm")
    assert not root_scan_finds("/root/payroll-copy.docm"), "the ignored detection was reported"
    show_page("trusted-detections")
    await_widget(WidgetQuery("list-item", IGNORED_SIGNATURE))
    await_widget(WidgetQuery("list-item", "local.fp"))
    machine.screenshot("allowlists_for_all_users")

with subtest("reporting an ignored detection again needs the password and shrinks the list"):
    press(button("Remove", inside=("list-item", IGNORED_SIGNATURE)))
    press(button("Report Again", inside=in_alert("Report This Detection Again?")))
    answer_password_prompt()
    machine.wait_until_succeeds(
        f"! grep -q {IGNORED_SIGNATURE} {IGNORED_SIGNATURES_LIST} 2>/dev/null", timeout=timedelta(seconds=60)
    )
    wait_for_root_scan_verdict('/root/payroll-copy.docm', expect_found=True)
    machine.succeed("test -e /var/lib/clamav/local.fp")

with subtest("cancelling or failing the password prompt trusts nothing"):
    place_sample("invoice.pdf.exe", TRUST_CANDIDATE)
    quarantine_with_lazaret(TRUST_CANDIDATE)
    ask_to_trust_for_all_users("invoice.pdf.exe")
    machine.screenshot("trust_password_prompt")
    dismiss_password_prompt()
    machine.fail(f"test -e {TRUSTED_FILES_LIST}")
    ask_to_trust_for_all_users("invoice.pdf.exe")
    answer_password_prompt("not-the-password")
    dismiss_password_prompt()
    machine.fail(f"test -e {TRUSTED_FILES_LIST}")
    assert root_scan_finds(ROOT_SCAN_COPY), "a refused trust change still hid the file"

with subtest("trusting a file for all users tells clamd and survives as a hash line"):
    ask_to_trust_for_all_users("invoice.pdf.exe")
    answer_password_prompt()
    machine.wait_until_succeeds(f"grep -qx '{EICAR_SHA256}:68:Lazaret.TrustedFile' {TRUSTED_FILES_LIST}", timeout=timedelta(seconds=60))
    wait_for_root_scan_verdict(ROOT_SCAN_COPY, expect_found=False)
    assert machine.succeed(f"stat -c '%a %U' {TRUSTED_FILES_LIST}").strip() == "644 root"
    machine.succeed("journalctl --no-pager -o verbose -u lazaret-clamav-admin.service | grep -q " + EICAR_SHA256)
    show_page("trusted-detections")
    await_widget(WidgetQuery("list-item", EICAR_SHA256))
    machine.screenshot("trusted_for_all_users")

with subtest("removing the trust for all users reports the file again and drops the empty list"):
    press(button("Remove", inside=("list-item", EICAR_SHA256)))
    press(button("Stop Trusting", inside=in_alert("Stop Trusting This File?")))
    answer_password_prompt()
    machine.wait_until_fails(f"test -e {TRUSTED_FILES_LIST}", timeout=timedelta(seconds=60))
    wait_for_root_scan_verdict(ROOT_SCAN_COPY, expect_found=True)

with subtest("a file trusted for me can be promoted to all users from Trusted Detections"):
    place_sample("payroll-macro.docm", PROMOTED_CANDIDATE)
    quarantine_with_lazaret(PROMOTED_CANDIDATE)
    entry_title = open_quarantined_entry("payroll")
    press(quarantine_entry_part("Trust…", entry_title))
    press(button("Trust for Me", inside=in_alert("Trust This File?")))
    wait_for_lazaret_rows(f"select 1 from trusted_contents where content_sha256 = '{FALSE_POSITIVE_SHA256}'")
    show_page("trusted-detections")
    press(button("Trust for All Users…", inside=("list-item", FALSE_POSITIVE_SHA256)))
    press(button("Trust for All Users…", inside=in_alert("Trust This File?")))
    answer_password_prompt()
    machine.wait_until_succeeds(f"grep -q '^{FALSE_POSITIVE_SHA256}:' {TRUSTED_FILES_LIST}", timeout=timedelta(seconds=60))
    wait_for_root_scan_verdict('/root/payroll-copy.docm', expect_found=False)

with subtest("Run Now asks for a password and runs the system scan"):
    machine.succeed(f"cp {SAMPLES_DIRECTORY}/invoice.pdf.exe {SCANNED_FILE}")
    show_page("scheduled-scans")
    press(button("Run Now"))
    answer_password_prompt()
    machine.wait_until_succeeds(
        f"journalctl -u {SCAN_UNIT} --no-pager --output=cat | grep -q '{SCANNED_FILE}.*FOUND'",
        timeout=timedelta(seconds=120),
    )
    machine.wait_until_succeeds(f"systemctl show -p Result --value {SCAN_UNIT} | grep -q exit-code", timeout=timedelta(seconds=60))
    exit_status = machine.succeed(f"systemctl show -p ExecMainStatus --value {SCAN_UNIT}").strip()
    assert exit_status == "1", f"a detection should exit 1: {exit_status}"

with subtest("finding malware is not shown as a scanner failure"):
    machine.screenshot("scan_finished_toast")
    await_label("found malware")
    assert_no_label("did not finish cleanly")

with subtest("the scan's journal line becomes a detection that is not called blocked"):
    wait_for_stored_detection(SCANNED_FILE)
    detection = stored_detection(SCANNED_FILE).split("|")
    assert detection[0].startswith("Win.Fixture.Agent-1"), detection
    assert detection[1] == "scheduled_scan", f"wrong source: {detection}"
    matching = wait_for_notification("Malware found")
    assert not notifications_mentioning("Malware blocked"), matching
    assert_no_label("ended with an error")

with subtest("the Lazaret schedule is locked because the system has a timer"):
    show_page("scheduled-scans")
    await_label("these settings are locked")
    run_widget_command("press-widget", WidgetQuery("label", ENABLE_SCHEDULE_TITLE), timeout=5)
    run_widget_command("press-widget", button(APPLY_SCHEDULE_LABEL), timeout=5)
    as_user(f"test ! -e /home/{SAMPLE_USER}/.config/systemd/user/lazaret-scheduled-scan.timer")

with subtest("settings the system owns are locked and copy a snippet"):
    show_page("scan-engine")
    await_widget(WidgetQuery("image", "doesn't allow changing this setting", name_match="contained"))
    machine.screenshot("scanner_engine_settings")

with subtest("signature sources have their own page"):
    show_page("signature-sources")
    await_label("Signature Sources", timeout=60)

with subtest("the privileged units run sandboxed"):
    for privileged_unit in ["lazaret-clamav-admin.service", "lazaret-settings-merge.service", "lazaret-reduction-expiry.service"]:
        exposure_report = machine.succeed(f"systemd-analyze security --no-pager {privileged_unit}")
        exposure_score = float(exposure_report.strip().splitlines()[-1].split("level for")[-1].split(":")[1].split()[0])
        assert exposure_score <= 3.0, f"{privileged_unit} exposure {exposure_score}:\n{exposure_report}"

with subtest("the socket survives a daemon restart"):
    machine.succeed("test -S /run/clamav/clamd.ctl")
    machine.succeed("systemctl restart clamav-daemon.service")
    machine.succeed("test -S /run/clamav/clamd.ctl")
    wait_for_clamd()

with subtest("live tuning is on by default and the daemon sees the merged configuration"):
    machine.succeed("grep -qx DetectPUA /etc/lazaret/tunable-settings")
    machine.succeed(f"systemctl cat clamav-daemon.service | grep -- 'BindReadOnlyPaths=-{MERGED_CONFIG}:/etc/clamav/clamd.conf'")
    machine.fail("systemctl cat clamav-daemon.service | grep -- --config-file")
    assert "DatabaseDirectory" in machine.succeed(f"cat {MERGED_CONFIG}")
    assert config_seen_by_clamd() == machine.succeed(f"cat {MERGED_CONFIG}")

with subtest("a user change asks for a password, shows the change, and applies"):
    start_apply_as_user("MaxThreads", "7")
    answer_password_prompt()
    answer = wait_for_user_answer()
    assert "applied" in answer, answer
    assert "MaxThreads 7" in machine.succeed(f"cat {MERGED_CONFIG}")
    wait_for_clamd()

with subtest("authorization is not retained, so a second change asks again"):
    start_apply_as_user("MaxThreads", "9")
    dismiss_password_prompt()
    answer = wait_for_user_answer()
    assert "applied" not in answer, f"applied without a new password: {answer}"
    assert "MaxThreads 9" not in machine.succeed(f"cat {MERGED_CONFIG}")

with subtest("the Scanner Engine settings offer to unlock allowed settings"):
    wait_for_lazaret_service()
    show_scan_engine_page()
    await_label(LIVE_CONFIG_DESCRIPTION)
    machine.screenshot("scanner_engine_locked")
    await_widget(WidgetQuery("toggle-button", UNLOCK_LABEL))

with subtest("while locked, Copy configuration line only copies the line"):
    merged_before_copy = machine.succeed(f"cat {MERGED_CONFIG}")
    overrides_before_copy = machine.succeed(f"cat {OVERRIDES_FILE} 2>/dev/null || true")
    copy_max_threads_line()
    await_label("Copied", timeout=60)
    machine.screenshot("locked_copy")
    assert machine.succeed(f"cat {MERGED_CONFIG}") == merged_before_copy, "a locked copy changed clamd"
    assert machine.succeed(f"cat {OVERRIDES_FILE} 2>/dev/null || true") == overrides_before_copy, (
        "a locked copy changed the stored changes"
    )

with subtest("unlocking exposes the allowed settings to assistive technology"):
    show_scan_engine_page()
    press(WidgetQuery("toggle-button", UNLOCK_LABEL))
    await_widget(WidgetQuery("toggle-button", "Lock"))
    machine.screenshot("preferences_unlocked")
    unlocked_editor = WidgetQuery("list-item", MAX_THREADS_TITLE)
    print(as_user_status(f"xtask describe-widget-ancestry {MAX_THREADS_TITLE}")[1])
    expect_behaviour(
        "an unlocked setting can be reached by its name",
        lambda: await_widget(unlocked_editor, timeout=15),
    )

if widget_is_shown(WidgetQuery("list-item", MAX_THREADS_TITLE)):
    with subtest("editing and confirming applies through a real password prompt"):
        edit_max_threads(8)
        await_label("MaxThreads 8")
        machine.screenshot("confirm_change")
        press(button(CONFIRM_APPLY_LABEL, inside=in_alert(CHANGE_CONFIRMATION_HEADING)))
        answer_password_prompt()
        machine.wait_until_succeeds(f"grep -q 'MaxThreads 8' {MERGED_CONFIG}", timeout=timedelta(seconds=120))
        wait_for_clamd()
        show_scan_engine_page()
        await_label("MaxThreads = 8", timeout=60)
        machine.screenshot("after_gui_apply")

    with subtest("a second change starts from the value just applied"):
        show_scan_engine_page()
        edit_max_threads(10)
        await_label("Now: 8", timeout=60)
        machine.screenshot("confirm_second_change")
        press(button("Cancel", inside=in_alert(CHANGE_CONFIRMATION_HEADING)))

with subtest("protected and unlisted settings are refused by the admin service"):
    for key, toml_value in (("VirusEvent", '"/bin/sh"'), ("User", '"root"'), ("MaxFileSize", '"1M"'), ("MaxThreads", '"many"')):
        status, output = apply_as_root(key, toml_value)
        assert status != 0, output
    merged = machine.succeed(f"cat {MERGED_CONFIG}")
    assert "VirusEvent" not in merged and "User root" not in merged and "MaxFileSize" not in merged

with subtest("a protected key planted in the stored changes makes the merge fall back to the base"):
    machine.succeed(
        f"printf '[VirusEvent]\\nstate = \"permanent\"\\nvalue = \"/bin/sh\"\\n\\n[MaxThreads]\\nstate = \"permanent\"\\nvalue = 5\\n' > {OVERRIDES_FILE}"
    )
    merge_again()
    merged = machine.succeed(f"cat {MERGED_CONFIG}")
    assert "VirusEvent" not in merged and "MaxThreads 5" not in merged, merged
    assert "MaxThreads 4" in merged, merged

with subtest("clamd still starts when the stored changes cannot be read"):
    machine.succeed(f"printf 'not toml at all\\n\\0\\0' > {OVERRIDES_FILE}")
    merge_again()

with subtest("a restart that fails restores the previous settings"):
    machine.succeed(f"printf '[MaxThreads]\\nstate = \"permanent\"\\nvalue = 6\\n' > {OVERRIDES_FILE}")
    merge_again()
    status, output = apply_as_root("MaxThreads", "3")
    assert status == 0 and "rolled-back" in output, output
    assert "value = 6" in machine.succeed(f"cat {OVERRIDES_FILE}")
    assert "MaxThreads 3" not in machine.succeed(f"cat {MERGED_CONFIG}")
    wait_for_clamd()

with subtest("a change that reduces protection ends on its own"):
    status, output = apply_as_root("DetectPUA", "false")
    assert status == 0 and "applied" in output, output
    stored_overrides = machine.succeed(f"cat {OVERRIDES_FILE}")
    base_detect_pua = machine.succeed("grep -i DetectPUA /etc/clamav/clamd.conf || echo 'DetectPUA unset in the base'")
    assert "temporary-reduction" in stored_overrides, f"stored:\n{stored_overrides}\nbase: {base_detect_pua}"
    assert "DetectPUA no" in config_seen_by_clamd()
    time.sleep(35)
    machine.succeed("systemctl start lazaret-reduction-expiry.service")
    wait_for_clamd()
    assert "DetectPUA no" not in config_seen_by_clamd()
    machine.succeed("journalctl _SYSTEMD_UNIT=lazaret-reduction-expiry.service LAZARET_ACTION=expire-reductions | grep -q DetectPUA")

with subtest("going back to the system configuration removes the change"):
    status, output = reset_as_root("MaxThreads")
    assert status == 0 and "applied" in output, output
    wait_for_clamd()
    assert "MaxThreads 4" in config_seen_by_clamd()

with subtest("a rebuild that changes the base reaches clamd without a reboot"):
    machine.succeed("/run/current-system/specialisation/newBase/bin/switch-to-configuration test")
    machine.wait_until_succeeds(f"grep -q 'MaxThreads 11' {MERGED_CONFIG}", timeout=timedelta(seconds=120))
    wait_for_clamd()
    assert "MaxThreads 11" in config_seen_by_clamd()

with subtest("every product behaviour held"):
    report_deferred_product_failures()
