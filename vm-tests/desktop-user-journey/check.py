ON_ACCESS_UNIT = "clamav-clamonacc.service"
ABUSE_CH_KEY = "FIXTUREKEYMUSTNEVERAPPEAR"
VIRUSTOTAL_KEY = "0123456789abcdef" * 4
REPORTER_NAME = "Sample User"
REPORTER_EMAIL = "sample-user@example.org"

BLOCKED_FILE = f"{DOWNLOADS}/invoice.pdf.exe"
ARCHIVE_FILE = f"{DOCUMENTS}/holiday photos.zip"
NON_UTF8_NAME_BYTES = b"r\xe9sum\xe9-copy.exe"
NON_UTF8_FILE_SHELL = f"\"$(printf '{DOCUMENTS}/r\\351sum\\351-copy.exe')\""
FALSE_POSITIVE_FILE = f"{DOCUMENTS}/payroll-macro.docm"
UNWANTED_PROGRAM_FILE = f"{DOCUMENTS}/toolbar-setup.exe"
MISSED_MALWARE_FILE = f"{DOCUMENTS}/dropper-2026.bin"
UNREMOVABLE_FILE = "/srv/shared/shared-report.exe"


def quarantined_signature(path_fragment):
    signature_rows = wait_for_lazaret_rows(
        "select signature_name from quarantine join quarantine_signatures using (entry_id) "
        f"where cast(original_path as text) like '%{path_fragment}%'"
    )
    return signature_rows[0][0]


def quarantined_entry_count():
    return int(lazaret_rows("select count(*) from quarantine")[0][0])


def submission_status(content_sha256, submission_service, submission_kind):
    status_rows = lazaret_rows(
        "select submission_status from submissions "
        f"where content_sha256 = '{content_sha256}' and submission_service = '{submission_service}' "
        f"and submission_kind = '{submission_kind}'"
    )
    return status_rows[0][0] if status_rows else None


def wait_for_submission_status(content_sha256, submission_service, submission_kind, expected_status, timeout=90):
    started_waiting = time.monotonic()
    while time.monotonic() - started_waiting < timeout:
        if submission_status(content_sha256, submission_service, submission_kind) == expected_status:
            return
        time.sleep(2)
    raise AssertionError(
        f"{submission_service} {submission_kind} of {content_sha256} stayed "
        f"{submission_status(content_sha256, submission_service, submission_kind)}, expected {expected_status}"
    )


QUARANTINE_SEARCH = WidgetQuery("entry", "", name_match="contained")


def show_quarantine_entry(path_fragment, search_text):
    show_page("quarantine")
    press(button("Refresh"))
    if not widget_is_shown(QUARANTINE_SEARCH):
        press(WidgetQuery("toggle-button", "Search"))
    type_into(QUARANTINE_SEARCH, search_text)
    entry_title = quarantined_signature(path_fragment)
    entry_trust_button = quarantine_entry_part("Trust…", entry_title)
    if not widget_is_shown(entry_trust_button):
        press(WidgetQuery("list-item", entry_title))
    await_widget(entry_trust_button)
    return entry_title


def submit_quarantined_entry(entry_title, target_label, confirm_label):
    press(quarantine_entry_part("Submit…", entry_title))
    press(button(target_label, inside=in_alert("Where Should This File Go?")))
    press(button(confirm_label, inside=in_alert(f"{target_label}?")))


def type_reporter_contact(dialog_heading):
    type_into(WidgetQuery("text", "Your Name", inside=in_alert(dialog_heading)), REPORTER_NAME)
    type_into(WidgetQuery("text", "Your Email Address", inside=in_alert(dialog_heading)), REPORTER_EMAIL)


def quarantine_scan_results(threat_count):
    press(button(f"Quarantine {threat_count} {'Threat' if threat_count == 1 else 'Threats'}"), timeout=120)
    press(button("Quarantine", inside=in_alert("Move to Quarantine?")))


def wait_until_blocked(file_path):
    machine.wait_until_succeeds(
        user_command(f"cat {shlex.quote(file_path)} 2>&1 | grep -q 'not permitted'"), timeout=timedelta(seconds=60)
    )


machine.start()
wait_for_desktop()

EICAR_SHA256 = sample_sha256("invoice.pdf.exe")
ARCHIVE_SHA256 = sample_sha256("holiday photos.zip")
FALSE_POSITIVE_SHA256 = sample_sha256("payroll-macro.docm")
UNWANTED_PROGRAM_SHA256 = sample_sha256("toolbar-setup.exe")
MISSED_MALWARE_SHA256 = sample_sha256("dropper-2026.bin")

with subtest("the fake internet answers over TLS the VM trusts"):
    machine.wait_for_unit("fake-internet.service")
    machine.wait_until_succeeds(
        "curl --fail --silent https://signatures.fixture.test/lazaret-fixture.hdb | grep -q Win.Fixture.Agent-1",
        timeout=timedelta(seconds=60),
    )
    machine.succeed(f"rm -f {FAKE_INTERNET_DIRECTORY}/requests.jsonl")

with subtest("logging in starts Lazaret through the module, without a window"):
    wait_for_lazaret_service()
    assert lazaret_window() is None or not lazaret_window().get("visible"), "started with a window"

with subtest("on-access scanning is ready"):
    wait_for_clamd_ready()
    machine.succeed(f"systemctl restart {ON_ACCESS_UNIT}")
    machine.wait_until_succeeds(
        f"journalctl -u {ON_ACCESS_UNIT} --no-pager | grep -q 'ClamInotif: watching'", timeout=timedelta(seconds=120)
    )

with subtest("a downloaded invoice is blocked and raises one urgent alert"):
    place_sample("invoice.pdf.exe", BLOCKED_FILE)
    wait_until_blocked(BLOCKED_FILE)
    matching = wait_for_notification("Win.Fixture.Agent-1")
    assert len(matching) == 1, f"expected one notification: {matching}"
    assert BLOCKED_FILE in notification_text(matching[0]), matching
    assert "Malware blocked" in notification_text(matching[0]), matching
    urgency = matching[0].get("urgency")
    urgency = urgency.get("data") if isinstance(urgency, dict) else urgency
    assert urgency in (2, "critical"), f"not urgent: {matching[0]}"
    wait_for_stored_detection(BLOCKED_FILE)
    detection = stored_detection(BLOCKED_FILE).split("|")
    assert detection[0].startswith("Win.Fixture.Agent-1"), detection
    assert detection[1] in ("on_access", "on_access_and_scheduled_scan"), detection
    assert detection[3] == "", f"should be unresolved: {detection}"
    assert_lines_stay_at(ON_ACCESS_UNIT, BLOCKED_FILE, settle_seconds=8)
    wait_for_tray_status("NeedsAttention")

with subtest("the alert opens the detection, and Look Up sends MalwareBazaar only the signature name"):
    as_user("makoctl invoke")
    wait_for_lazaret_window()
    press(button("Look Up"))
    lookup_request = wait_for_fake_internet_request("bazaar_get_clamavinfo")[0]
    assert lookup_request["form_fields"]["clamav"].startswith("Win.Fixture.Agent-1"), lookup_request
    assert lookup_request["headers"].get("auth-key") == ABUSE_CH_KEY, lookup_request["headers"]
    assert lookup_request["body_byte_size"] < 200, f"the lookup sent more than a name: {lookup_request}"
    await_label("FixtureLoader", timeout=30)
    machine.screenshot("detection_looked_up")

with subtest("quarantining from the timeline neutralises the file"):
    press(button("Quarantine"))
    machine.wait_until_fails(f"test -e {BLOCKED_FILE}", timeout=timedelta(seconds=30))
    wait_as_user(
        f"{LAZARET_READ_QUERY} "
        "\"select resolution_kind from detection_resolutions\" | grep -q quarantined"
    )
    wait_for_tray_status("Active")
    assert quarantined_signature("invoice.pdf.exe").startswith("Win.Fixture.Agent-1")
    machine.fail(f"grep -rq EICAR-STANDARD {QUARANTINE_DIRECTORY}")
    assert "FOUND" not in clamdscan_reports(QUARANTINE_DIRECTORY), "a quarantined payload is still detectable"

with subtest("the tray action opens the Detections page"):
    activate_application_action("open-detections")
    await_label("What on-access scanning")

with subtest("deleting takes two choices and withdraws the alert that is still showing"):
    as_user("makoctl dismiss --all")
    deleted_file = f"{DOWNLOADS}/throwaway.exe"
    place_sample("invoice.pdf.exe", deleted_file)
    wait_until_blocked(deleted_file)
    wait_for_notification(deleted_file)
    press(WidgetQuery("list-item", "throwaway.exe"))
    press(button("Delete…"))
    machine.succeed(f"test -e {deleted_file}")
    press(button("Delete Permanently"))
    machine.wait_until_fails(f"test -e {deleted_file}", timeout=timedelta(seconds=30))
    wait_for_notification_gone(deleted_file)
    wait_for_tray_status("Active")

with subtest("a manual scan of Documents finds the archive, the odd name, the false positive and the toolbar"):
    place_sample("holiday photos.zip", ARCHIVE_FILE)
    place_sample("payroll-macro.docm", FALSE_POSITIVE_FILE)
    place_sample("toolbar-setup.exe", UNWANTED_PROGRAM_FILE)
    as_user(f"cp {SAMPLES_DIRECTORY}/invoice.pdf.exe {NON_UTF8_FILE_SHELL}")
    entries_before_scan = quarantined_entry_count()
    scan_with_lazaret(DOCUMENTS)
    quarantine_scan_results(4)
    wait_as_user(
        f"test \"$({LAZARET_READ_QUERY} 'select count(*) from quarantine')\" = {entries_before_scan + 4}",
        timeout=60,
    )
    for moved_file in (ARCHIVE_FILE, FALSE_POSITIVE_FILE, UNWANTED_PROGRAM_FILE):
        machine.fail(f"test -e {shlex.quote(moved_file)}")
    machine.fail(f"test -e {NON_UTF8_FILE_SHELL}")
    stored_paths = [row[0] for row in lazaret_rows("select hex(original_path) from quarantine")]
    assert NON_UTF8_NAME_BYTES.hex().upper() in "".join(stored_paths), stored_paths
    assert quarantined_signature("payroll").startswith("Doc.Fixture.FalsePositive-1")
    assert quarantined_signature("toolbar").startswith("PUA.Win.Fixture.Toolbar-1")
    machine.screenshot("documents_quarantined")

with subtest("the reputation check sends each service only the hash"):
    eicar_title = show_quarantine_entry("invoice.pdf.exe", "invoice")
    press(quarantine_entry_part("Check", eicar_title))
    circl_request = wait_for_fake_internet_request("circl_lookup")[0]
    bazaar_request = wait_for_fake_internet_request("bazaar_get_info")[0]
    virustotal_request = wait_for_fake_internet_request("virustotal_file_report")[0]
    assert circl_request["path"].endswith(EICAR_SHA256), circl_request
    assert "auth-key" not in circl_request["headers"] and "x-apikey" not in circl_request["headers"]
    assert bazaar_request["form_fields"] == {"query": "get_info", "hash": EICAR_SHA256}, bazaar_request
    assert virustotal_request["path"] == f"/api/v3/files/{EICAR_SHA256}", virustotal_request
    assert virustotal_request["headers"].get("x-apikey") == VIRUSTOTAL_KEY, virustotal_request["headers"]
    for lookup_request in (circl_request, bazaar_request, virustotal_request):
        assert lookup_request["body_byte_size"] < 200, f"a lookup sent the file: {lookup_request}"
    await_label("Unknown to VirusTotal", timeout=30)
    machine.screenshot("reputation_checked")

with subtest("a VirusTotal rate limit and a rejected key are explained"):
    set_fake_internet_scenario(virustotal_file_report={"status": 429, "body": {"error": {"code": "QuotaExceededError"}}})
    press(quarantine_entry_part("Check", eicar_title))
    await_label("request limit was reached", timeout=30)
    set_fake_internet_scenario(virustotal_file_report={"status": 401, "body": {"error": {"code": "WrongCredentialsError"}}})
    press(quarantine_entry_part("Check", eicar_title))
    await_label("rejected the API key", timeout=30)
    set_fake_internet_scenario()

with subtest("an unavailable MalwareBazaar is reported as unavailable, not as an unreadable answer"):
    set_fake_internet_scenario(
        bazaar_get_info={"status": 503, "content_type": "text/html", "body": "<html>Service Unavailable</html>"}
    )
    bazaar_lookups_before = len(fake_internet_requests("bazaar_get_info"))
    press(quarantine_entry_part("Check", eicar_title))
    wait_for_fake_internet_request("bazaar_get_info", previous_count=bazaar_lookups_before)
    await_label("MalwareBazaar ")
    machine.screenshot("bazaar_unavailable")
    set_fake_internet_scenario()
    expect_behaviour(
        "an HTTP 503 from MalwareBazaar is reported as unavailable",
        lambda: assert_no_label("sent an answer that could not be read"),
    )

with subtest("uploading the invoice to MalwareBazaar names it by its hash and is never repeated"):
    submit_quarantined_entry(eicar_title, "Upload to MalwareBazaar", "Upload Publicly")
    upload_request = wait_for_fake_internet_request("bazaar_upload")[0]
    uploaded_file = upload_request["form_fields"]["file"]
    assert uploaded_file["uploaded_name"] == EICAR_SHA256, uploaded_file
    assert uploaded_file["content_sha256"] == EICAR_SHA256, "the upload is not the quarantined file"
    upload_options = json.loads(upload_request["form_fields"]["json_data"])
    assert upload_options["anonymous"] == 1, upload_options
    assert upload_options["tags"][0].startswith("Win.Fixture.Agent-1"), upload_options
    wait_for_submission_status(EICAR_SHA256, "malware_bazaar", "malware_sample", "upload_accepted")
    eicar_title = show_quarantine_entry("invoice.pdf.exe", "invoice")
    submit_quarantined_entry(eicar_title, "Upload to MalwareBazaar", "Upload Publicly")
    assert_request_count_stays("bazaar_upload", 1)

with subtest("the archive goes to MalwareBazaar too"):
    archive_title = show_quarantine_entry("holiday", "holiday")
    submit_quarantined_entry(archive_title, "Upload to MalwareBazaar", "Upload Publicly")
    wait_for_fake_internet_request("bazaar_upload", previous_count=1)
    wait_for_submission_status(ARCHIVE_SHA256, "malware_bazaar", "malware_sample", "upload_accepted")

with subtest("a potentially unwanted program is never offered to MalwareBazaar"):
    toolbar_title = show_quarantine_entry("toolbar", "toolbar")
    press(quarantine_entry_part("Submit…", toolbar_title))
    await_widget(WidgetQuery("alert", "Where Should This File Go?"))
    assert not widget_is_shown(button("Upload to MalwareBazaar", inside=in_alert("Where Should This File Go?"))), (
        "MalwareBazaar was offered a potentially unwanted program"
    )
    press(button("Cancel", inside=in_alert("Where Should This File Go?")))
    assert len(fake_internet_requests("bazaar_upload")) == 2

with subtest("uploading the invoice to VirusTotal queues an analysis"):
    eicar_title = show_quarantine_entry("invoice.pdf.exe", "invoice")
    submit_quarantined_entry(eicar_title, "Upload to VirusTotal", "Upload")
    virustotal_upload = wait_for_fake_internet_request("virustotal_upload")[0]
    assert virustotal_upload["form_fields"]["file"]["uploaded_name"] == EICAR_SHA256, virustotal_upload
    assert virustotal_upload["headers"].get("x-apikey") == VIRUSTOTAL_KEY
    wait_for_submission_status(EICAR_SHA256, "virustotal", "malware_sample", "analysis_queued")

with subtest("one failed MalwareBazaar lookup doesn't stop the refresh of the other uploads"):
    set_fake_internet_scenario(
        bazaar_get_info={"status": 500, "content_type": "text/html", "body": "<html>Internal error</html>", "only_hash": EICAR_SHA256}
    )
    show_page("sent-samples")
    press(button("Check MalwareBazaar and VirusTotal for news"))
    wait_for_fake_internet_request("virustotal_analysis")
    press(button("Check MalwareBazaar and VirusTotal for news"))
    wait_for_submission_status(EICAR_SHA256, "virustotal", "malware_sample", "analysed")
    set_fake_internet_scenario()
    machine.screenshot("sent_samples_after_refresh")
    expect_behaviour(
        "one failed MalwareBazaar lookup leaves the other uploads refreshed",
        lambda: wait_for_submission_status(ARCHIVE_SHA256, "malware_bazaar", "malware_sample", "listed", timeout=30),
    )

with subtest("reporting the false positive runs clamsubmit against clamav.net"):
    false_positive_title = show_quarantine_entry("payroll", "payroll")
    press(quarantine_entry_part("Report…", false_positive_title))
    type_reporter_contact("Report a False Positive to ClamAV?")
    press(button("Send Report", inside=in_alert("Report a False Positive to ClamAV?")))
    submit_request = wait_for_fake_internet_request("clamav_submit", timeout=120)[0]
    report_fields = submit_request["form_fields"]
    assert report_fields["type"] == "fp", report_fields
    assert report_fields["sendername"] == REPORTER_NAME and report_fields["email"] == REPORTER_EMAIL, report_fields
    assert report_fields["virusname"].startswith("Doc.Fixture.FalsePositive-1"), report_fields
    sample_upload = fake_internet_requests("clamav_sample_upload")[-1]
    assert sample_upload["form_fields"]["file"]["content_sha256"] == FALSE_POSITIVE_SHA256, sample_upload
    wait_for_submission_status(FALSE_POSITIVE_SHA256, "clamav", "false_positive", "awaiting_review")

with subtest("missed malware picked from disk is reported to ClamAV"):
    place_sample("dropper-2026.bin", MISSED_MALWARE_FILE)
    show_page("sent-samples")
    press(button("Report Missed Malware…"))
    choose_file_in_dialog("Choose the File ClamAV Misses", MISSED_MALWARE_FILE)
    type_reporter_contact("Report Missed Malware to ClamAV?")
    press(button("Send Report", inside=in_alert("Report Missed Malware to ClamAV?")))
    malware_report = wait_for_fake_internet_request("clamav_submit", previous_count=1, timeout=120)[0]
    assert malware_report["form_fields"]["type"] == "malware", malware_report
    assert fake_internet_requests("clamav_sample_upload")[-1]["form_fields"]["file"]["content_sha256"] == MISSED_MALWARE_SHA256
    wait_for_submission_status(MISSED_MALWARE_SHA256, "clamav", "missed_malware", "awaiting_review")

with subtest("trusting the toolbar for me hides it from my scans, but not a changed copy"):
    toolbar_title = show_quarantine_entry("toolbar", "toolbar")
    press(quarantine_entry_part("Trust…", toolbar_title))
    press(button("Trust for Me", inside=in_alert("Trust This File?")))
    wait_for_lazaret_rows(f"select 1 from trusted_contents where content_sha256 = '{UNWANTED_PROGRAM_SHA256}'")
    trusted_copy = f"{DOCUMENTS}/trusted/toolbar-setup.exe"
    changed_copy = f"{DOCUMENTS}/changed/toolbar-setup.exe"
    place_sample("toolbar-setup.exe", trusted_copy)
    place_sample("toolbar-setup.exe", changed_copy)
    as_user(f"printf 'tampered' >> {changed_copy}")
    scan_with_lazaret(trusted_copy, changed_copy)
    quarantine_scan_results(1)
    machine.wait_until_fails(f"test -e {changed_copy}", timeout=timedelta(seconds=30))
    machine.succeed(f"test -e {trusted_copy}")

with subtest("a file trusted only for me is still blocked on access and still alerts, as the dialog says"):
    as_user("makoctl dismiss --all")
    trusted_download = f"{DOWNLOADS}/toolbar-setup.exe"
    place_sample("toolbar-setup.exe", trusted_download)
    wait_until_blocked(trusted_download)
    wait_for_notification(trusted_download)

with subtest("ignoring a detection for me works from a scan result and silences background alerts"):
    as_user("makoctl dismiss --all")
    ignored_copy = f"{DOCUMENTS}/ignored/payroll-macro.docm"
    place_sample("payroll-macro.docm", ignored_copy)
    scan_with_lazaret(ignored_copy)
    await_widget(button("Quarantine 1 Threat"), timeout=120)
    press(button("More Actions"))
    press(WidgetQuery("menu-item", "Ignore This Detection for Me…"))
    press(button("Ignore", inside=in_alert("Ignore This Detection?")))
    wait_for_lazaret_rows("select 1 from ignored_signatures where signature_name like 'Doc.Fixture.FalsePositive-1%'")
    ignored_download = f"{DOWNLOADS}/payroll-macro.docm"
    place_sample("payroll-macro.docm", ignored_download)
    machine.wait_until_succeeds(
        f"journalctl -u {ON_ACCESS_UNIT} --no-pager | grep -q {ignored_download}", timeout=timedelta(seconds=60)
    )
    time.sleep(10)
    assert not notifications_mentioning(ignored_download), "alerted about a detection the user ignored"
    assert stored_detection(ignored_download).strip() == "", "recorded a detection the user ignored"
    machine.fail("test -e /var/lib/clamav/lazaret-local.ign2")

with subtest("removing my trust and my ignored detection brings both back"):
    show_page("trusted-detections")
    press(button("Remove", inside=("list-item", UNWANTED_PROGRAM_SHA256)))
    press(button("Stop Trusting", inside=in_alert("Stop Trusting This File?")))
    ignored_signature = lazaret_rows("select signature_name from ignored_signatures")[0][0]
    press(button("Remove", inside=("list-item", ignored_signature)))
    press(button("Report Again", inside=in_alert("Report This Detection Again?")))
    wait_as_user(
        f"test \"$({LAZARET_READ_QUERY} 'select count(*) from trusted_contents') "
        f"$({LAZARET_READ_QUERY} 'select count(*) from ignored_signatures')\" = '0 0'"
    )
    scan_with_lazaret(trusted_copy)
    quarantine_scan_results(1)
    machine.wait_until_fails(f"test -e {trusted_copy}", timeout=timedelta(seconds=30))

with subtest("Update Now fetches the fixed signatures and keeps the abuse.ch key out of the UI"):
    set_fake_internet_scenario(signature_release="v2")
    activate_application_action("update-definitions")
    answer_every_password_prompt()
    wait_as_user(
        f"{LAZARET_READ_QUERY} "
        "\"select activity_status from activity_entries\" | grep -q '^update_\\(applied\\|already_current\\|failed\\|result_unknown\\)'",
        timeout=240,
    )
    machine.screenshot("update_finished")
    update_statuses = as_user(
        f"{LAZARET_READ_QUERY} \"select activity_status from activity_entries where activity_status like 'update_%'\""
    ).split()
    assert "update_applied" in update_statuses, f"the update did not apply: {update_statuses}"
    machine.succeed("grep -q Win.Fixture.Missed-1 /var/lib/clamav/lazaret-fixture.hdb")
    urlhaus_requests = wait_for_fake_internet_request("urlhaus_download")
    assert any(ABUSE_CH_KEY in urlhaus_request["path"] for urlhaus_request in urlhaus_requests), urlhaus_requests
    stored_output = as_user(f"{LAZARET_READ_QUERY} \"select activity_output from activity_entries\"")
    assert "fangfrisch" in stored_output, f"fangfrisch output was not shown:\n{stored_output}"
    assert ABUSE_CH_KEY not in stored_output, "the update log saved the abuse.ch key"
    assert VIRUSTOTAL_KEY not in stored_output, "the update log saved the VirusTotal key"
    assert "address hidden" in stored_output, f"the address was not hidden:\n{stored_output}"
    assert_no_label(ABUSE_CH_KEY)

with subtest("after the update the false positive is no longer detected and its report is settled"):
    wait_for_root_scan_verdict(f"{SAMPLES_DIRECTORY}/payroll-macro.docm", expect_found=False, timeout=120)
    show_page("quarantine")
    press(button("Check Again"))
    await_label("No longer detected", timeout=120)
    machine.screenshot("false_positive_no_longer_detected")
    expect_behaviour(
        "a signature update settles the false positive report",
        lambda: wait_for_submission_status(FALSE_POSITIVE_SHA256, "clamav", "false_positive", "fixed_in_database"),
    )

with subtest("the missed malware is detected since the update and its report is settled once quarantined"):
    scan_with_lazaret(MISSED_MALWARE_FILE)
    quarantine_scan_results(1)
    machine.wait_until_fails(f"test -e {MISSED_MALWARE_FILE}", timeout=timedelta(seconds=30))
    show_page("quarantine")
    press(button("Check Again"))
    expect_behaviour(
        "quarantining reported missed malware after the update settles its report",
        lambda: wait_for_submission_status(MISSED_MALWARE_SHA256, "clamav", "missed_malware", "now_detected", timeout=120),
    )

with subtest("restoring the fixed false positive gives back the same file"):
    false_positive_title = show_quarantine_entry("payroll", "payroll")
    press(quarantine_entry_part("Restore", false_positive_title))
    press(button("Restore", inside=in_alert("Restore This File?")))
    machine.wait_until_succeeds(f"test -e {FALSE_POSITIVE_FILE}", timeout=timedelta(seconds=30))
    restored_hash = machine.succeed(f"sha256sum {FALSE_POSITIVE_FILE}").split()[0]
    assert restored_hash == FALSE_POSITIVE_SHA256, "the restored file differs from the quarantined one"
    restored_mode = machine.succeed(f"stat -c %a {FALSE_POSITIVE_FILE}").strip()
    assert restored_mode == "640", f"the restored file lost its permissions: {restored_mode}"
    assert machine.succeed(f"stat -c %U {FALSE_POSITIVE_FILE}").strip() == SAMPLE_USER

with subtest("restoring over a file that came back in the meantime is refused and keeps the entry"):
    as_user(f"printf 'new holiday photos' > {shlex.quote(ARCHIVE_FILE)}")
    archive_title = show_quarantine_entry("holiday", "holiday")
    payloads_before_restore = machine.succeed(f"ls {QUARANTINE_DIRECTORY} | wc -l").strip()
    press(quarantine_entry_part("Restore", archive_title))
    press(button("Restore Anyway", inside=in_alert("Restore a File ClamAV Still Detects?")))
    time.sleep(5)
    machine.screenshot("restore_collision")
    expect_behaviour(
        "a refused restore keeps the newer file, the entry and its payload",
        lambda: (
            require(as_user(f"cat {shlex.quote(ARCHIVE_FILE)}") == "new holiday photos", "the restore overwrote a newer file"),
            require(
                bool(lazaret_rows("select 1 from quarantine where cast(original_path as text) like '%holiday%'")),
                "the entry disappeared after a refused restore",
            ),
            require(
                machine.succeed(f"ls {QUARANTINE_DIRECTORY} | wc -l").strip() == payloads_before_restore,
                "the payload count changed after a refused restore",
            ),
        ),
    )
    as_user(f"rm {shlex.quote(ARCHIVE_FILE)}")

with subtest("restoring a file ClamAV still detects into a watched folder opens the detection again"):
    eicar_title = show_quarantine_entry("invoice.pdf.exe", "invoice")
    press(quarantine_entry_part("Restore", eicar_title))
    press(button("Restore Anyway", inside=in_alert("Restore a File ClamAV Still Detects?")))
    machine.wait_until_succeeds(f"test -e {BLOCKED_FILE}", timeout=timedelta(seconds=30))
    assert not lazaret_rows(
        "select 1 from quarantine where cast(original_path as text) like '%invoice.pdf.exe'"
    ), "the restored entry is still listed"
    wait_until_blocked(BLOCKED_FILE)
    machine.wait_until_succeeds(
        user_command(
            f"test -z \"$({LAZARET_READ_QUERY} "
            f"\\\"select resolution_kind from detection_resolutions where cast(file_path as text) = '{BLOCKED_FILE}'\\\")\""
        ),
        timeout=timedelta(seconds=60),
    )
    wait_for_tray_status("NeedsAttention")

with subtest("deleting for good and clearing old items remove the payloads"):
    renamed_title = show_quarantine_entry("-copy.exe", "copy")
    payload_path = lazaret_rows(
        "select cast(quarantine_path as text) from quarantine where cast(original_path as text) like '%-copy.exe'"
    )[0][0]
    press(quarantine_entry_part("Delete", renamed_title))
    press(button("Delete", inside=in_alert("Delete Quarantined File?")))
    machine.wait_until_fails(f"test -e {shlex.quote(payload_path)}", timeout=timedelta(seconds=30))
    as_user(
        f"{LAZARET_WRITE_QUERY} "
        "\"update quarantine set detection_date = '2020-01-01T00:00:00.000Z' "
        "where cast(original_path as text) like '%holiday%'\""
    )
    show_page("quarantine")
    press(button("Refresh"))
    press(button("More"))
    press(button("Clear Old Items…"))
    press(button("Delete", inside=in_alert("Clear Old Items?")))
    wait_as_user(
        f"test -z \"$({LAZARET_READ_QUERY} "
        "\\\"select 1 from quarantine where cast(original_path as text) like '%holiday%'\\\")\""
    )

with subtest("a detected file Lazaret can read but not remove leaves no orphaned payload"):
    machine.succeed(f"cp {SAMPLES_DIRECTORY}/invoice.pdf.exe {UNREMOVABLE_FILE} && chmod 0644 {UNREMOVABLE_FILE}")
    payloads_before_quarantine = machine.succeed(f"ls {QUARANTINE_DIRECTORY} | wc -l").strip()
    scan_with_lazaret(UNREMOVABLE_FILE)
    quarantine_scan_results(1)
    time.sleep(5)
    machine.screenshot("unremovable_file")
    machine.succeed(f"test -e {UNREMOVABLE_FILE}")
    expect_behaviour(
        "a file that can't be removed leaves no pending record and no orphaned payload",
        lambda: (
            require(not lazaret_rows("select 1 from quarantine_pending"), "a pending quarantine record was left behind"),
            require(
                machine.succeed(f"ls {QUARANTINE_DIRECTORY} | wc -l").strip() == payloads_before_quarantine,
                "a payload was left in the quarantine for a file that was never moved",
            ),
        ),
    )

with subtest("a scheduled scan only reports while Lazaret is open, and quarantines once it is closed"):
    scheduled_target = f"{DOCUMENTS}/scheduled"
    scheduled_file = f"{scheduled_target}/invoice-copy.exe"
    place_sample("invoice.pdf.exe", scheduled_file)
    scheduled_scan_command = (
        f"systemd-run --user --wait --collect /run/current-system/sw/bin/lazaret --scheduled-scan --auto-quarantine --target {scheduled_target}"
    )
    as_user_status(scheduled_scan_command)
    machine.succeed(f"test -e {scheduled_file}")
    as_user("systemctl --user stop lazaret.service")
    as_user_status(scheduled_scan_command)
    machine.wait_until_fails(f"test -e {scheduled_file}", timeout=timedelta(seconds=60))
    assert lazaret_rows(
        "select 1 from quarantine where cast(original_path as text) like '%scheduled/invoice-copy.exe'"
    ), "the closed-app scheduled scan did not record the quarantined file"
    as_user("systemctl --user start lazaret.service")
    wait_for_lazaret_service()

with subtest("detections missed while closed are summarised once at the next start"):
    missed_while_closed = f"{DOWNLOADS}/while-closed.exe"
    as_user("makoctl dismiss --all")
    as_user("systemctl --user stop lazaret.service")
    place_sample("invoice.pdf.exe", missed_while_closed)
    wait_until_blocked(missed_while_closed)
    time.sleep(2)
    as_user("systemctl --user start lazaret.service")
    wait_for_notification("while Lazaret wasn't running")
    assert not notifications_mentioning(missed_while_closed), "alerted individually about a missed file"
    assert_lines_stay_at(ON_ACCESS_UNIT, missed_while_closed, settle_seconds=10)

with subtest("neither API key ever reached the database"):
    database_dump = as_user(f"{LAZARET_READ_QUERY} .dump")
    assert ABUSE_CH_KEY not in database_dump and VIRUSTOTAL_KEY not in database_dump, "an API key is in the database"

with subtest("every product behaviour held"):
    report_deferred_product_failures()
