-brand-name = Lazaret

startup-settings-schema-missing = The { -brand-name } settings schema is not installed. Run make install, or set GSETTINGS_SCHEMA_DIR.
startup-unknown-page-name = { -brand-name } has no page named { $page }, so it opens Protection. Pages: { $pages }

window-signature-sources = Signature Sources
window-database-updates = Database Updates
window-scan-engine = Scan Engine
window-access-scanning = On-Access Scanning
window-scheduled-scans = Scheduled Scans
window-manual-scan = Manual Scan
window-removable-drives = Removable Drives
window-detections-label = Detections
window-quarantine-label = Quarantine
window-trusted-detections = Trusted Detections
window-sent-samples = Sent Samples
window-quarantine-database-not-locked = The quarantine database could not be locked
window-quarantine-database-not-opened = The quarantine database could not be opened
window-file-operation-failed-error = A file operation failed: { $error }
window-background-quarantine-step-stopped = A background quarantine step stopped unexpectedly: { $error }
window-translator-credits = translator-credits
window-export-history-menu-item = Export History…
window-clear-history-menu-item = Clear History…
window-preferences-label = Preferences
window-about-menu-item = About { -brand-name }
window-quit-menu-item = Quit
window-export-history-heading = Export History?
window-export-history-body = Saves every recorded scan and signature update, with their details.
window-cancel-history-button = Cancel
window-json-label = JSON
window-csv-label = CSV
window-clear-history-heading = Clear History?
window-clear-history-body = This permanently deletes every recorded scan and signature update. This can't be undone.
window-clear-label = Clear All
window-scanning-drive-title = Scanning { $drive }
window-open-removable-drives-button = Open Removable Drives
window-not-export-history-toast = Could not export the history: { $error }
window-history-exported-toast = History exported
window-history-cleared-toast = History cleared
window-not-clear-history-toast = Could not clear the history: { $error }
window-other-sources-unknown = Other sources unknown
window-source-out-date = A source is out of date
window-official-signatures = Official signatures only
window-official-count-other-source =
    { $count ->
        [one] Official and { $count } other source
       *[other] Official and { $count } other sources
    }
window-no-virus-definitions = No virus definitions
window-age-definitions-unknown = Age of the definitions unknown
window-last-update-failed = Last update failed
window-definitions-dated-future = Definitions dated in the future
window-updated-label = Updated { $when }
window-definitions-label = Definitions from { $when }
window-freshclam-refused = freshclam is refused
window-unknown-whether-freshclam-trusted = Unknown whether freshclam is trusted
window-freshclam-copy-account-ignored = A freshclam copy in your account is ignored
window-clamd-uses-older-signatures = clamd uses older signatures
window-clamd-answering = clamd answering
window-clamd-not-answering = clamd not answering
window-clamd-state-unknown = clamd state unknown
window-clamd-not-running = clamd not running
window-clamd-not-set-up = clamd not set up
window-clamav-not-installed = ClamAV not installed
window-clamscan-refused = clamscan is refused
window-clamav-version-unknown = ClamAV version unknown
window-scanner-program-refused = A scanner program is refused
window-unknown-whether-scanners-trusted = Unknown whether the scanners are trusted
window-scanner-copy-account-ignored = A scanner copy in your account is ignored
window-runtime-changes-unknown = Runtime changes unknown
window-settings-reduced-runtime = Settings reduced at runtime
window-waiting-clamd = Waiting for clamd
window-state-unknown = State unknown
window-not-set-up = Not set up
window-clamonacc-not-running = clamonacc not running
window-clamd-conf-unreadable = clamd.conf unreadable
window-nothing-watched = Nothing watched
window-checks-files-not-block = Checks files, doesn't block
window-checks-files-kernel-cannot = Checks files, kernel can't block
window-checks-files-blocking-unknown = Checks files, blocking unknown
window-blocking-paths = Blocking { $paths }
window-folder-watch-limit-unknown = Folder watch limit unknown
window-mount-watching-conflicts = Mount watching conflicts
window-clamd-not-excluded = clamd not excluded
window-watches-directly = Watches / directly
window-folder-watch-limit-low = Folder watch limit low
window-timer-not-active = Timer not active
window-last-scan-failed = Last scan failed
window-next-scan = Next scan { $when }
window-no-next-scan-planned = No next scan planned
window-scan-service-missing = Scan service missing
window-scan-service-unknown = Scan service unknown
window-scheduled-lazaret = Scheduled by { -brand-name }
window-no-scan-scheduled = No scan scheduled
window-schedule-changed-outside-lazaret = Schedule changed outside { -brand-name }
window-ready-scan = Ready to scan
window-uses-slower-clamscan = Uses the slower clamscan
window-exclusion-skip-malware = An exclusion may skip malware
window-scanning-drive = Scanning { $drive }
window-threats-found-drive = Threats found on { $drive }
window-journal-unreadable = Journal unreadable
window-following-journal-stopped = Following the journal stopped
window-notifications-off = Notifications off
window-alerts-stop-lazaret-quits = Alerts stop when { -brand-name } quits
window-other-programs-attach-lazaret = Other programs can attach to { -brand-name }
window-unknown-whether-programs-attach = Unknown whether programs can attach
window-nothing-decide = Nothing to decide
window-count-needs-decision =
    { $count ->
        [one] { $count } needs a decision
       *[other] { $count } need a decision
    }
window-loading-label = Loading…
window-empty-label = Empty
window-count-item =
    { $count ->
        [one] { $count } item
       *[other] { $count } items
    }
window-quarantine-cannot-read = The quarantine can't be read
window-nothing-trusted = Nothing trusted
window-count-item-trusted =
    { $count ->
        [one] { $count } item trusted
       *[other] { $count } items trusted
    }
window-signature-folder-unreadable = Signature folder unreadable
window-another-allowlist-silences-clamav = Another allowlist silences ClamAV
window-lazaret-allowlist-unreadable = { -brand-name }'s allowlist is unreadable
window-sent-samples-cannot-read = Sent samples can't be read
window-nothing-sent = Nothing sent
window-count-refused-service =
    { $count ->
        [one] { $count } refused by its service
       *[other] { $count } refused by their services
    }
window-count-awaiting-review =
    { $count ->
        [one] { $count } awaiting review
       *[other] { $count } awaiting review
    }
window-count-sent =
    { $count ->
        [one] { $count } sent
       *[other] { $count } sent
    }
window-signatures-label = Signatures
window-scans-label = Scans
window-findings-label = Findings
window-detections-come = Where detections come from
window-files-get-checked = How files get checked
window-found-label = What was found
window-needs-review = Needs review
window-not-working = Not working
window-waiting-another-part = Waiting for another part
window-checking-subtitle = Checking…
window-menu-tooltip = Menu
window-show-lazaret = Show { -brand-name }
window-show-detections = Show Detections
window-quick-scan = Quick Scan
window-update-definitions = Update Definitions
window-quit-tray-item = Quit
window-quit-completely = Quit completely
window-minimize-tray = Minimize to tray
window-always-ask = Always ask
window-close-lazaret = Close { -brand-name }?
window-detection-alerts-stop-lazaret = Detection alerts stop while { -brand-name } isn't running. Keep it running in the background, or quit completely?
window-cancel-close-button = Cancel
window-quit-close-button = Quit
window-keep-running = Keep Running

scans-node-js-dependencies = Node.js dependencies
scans-git-repository-data = Git repository data
scans-python-virtual-environment = Python virtual environment
scans-build-output-directory = Build output directory
scans-distribution-output-directory = Distribution output directory
scans-python-bytecode-cache = Python bytecode cache
scans-remove-exclusion-tooltip = Remove
scans-preset-exclusions-title = Preset Exclusions
scans-common-directories-exclude-description = Common directories to exclude from scanning. Changes save immediately.
scans-custom-exclusions-title = Custom Exclusions
scans-enter-absolute-path-description = Enter an absolute path or a pattern such as *.tmp. Changes save immediately.
scans-path-exclude-tmp-title = /path/to/exclude or *.tmp
scans-automatic-label = Automatic
scans-clamd-daemon = clamd (daemon)
scans-clamscan-standalone = clamscan (standalone)
scans-options-title = Options
scans-backend-title = Scan Backend
scans-automatic-uses-daemon-subtitle = Automatic uses the daemon when it answers and clamscan otherwise.
scans-show-live-scan-title = Show Live Scan Progress
scans-display-files-scanned-subtitle = Display files being scanned in real time.
scans-not-added-history-error = The scan could not be added to the history: { $error }
scans-no-profile-manual = No Profile (Manual)
scans-removable-drive-scanned-title = A removable drive is being scanned. Start this scan when it has finished.
scans-open-removable-drives-button = Open Removable Drives
scans-checking-manual = Checking…
scans-backend-unknown-scanner-programs = Backend: unknown, the scanner programs could not be looked up: { $error }
scans-backend-clamd-daemon = Backend: clamd (daemon)
scans-backend-clamscan-standalone = Backend: clamscan (standalone)
scans-scan-title = Scan
scans-preparing-label = Preparing…
scans-cancelling-label = Cancelling…
scans-scanning-name = Scanning { $name }
scans-scanning-count-location =
    { $count ->
        [one] Scanning { $count } location
       *[other] Scanning { $count } locations
    }
scans-cancelled-user = Scan cancelled by user
scans-threats-found-scan = Threats found
scans-not-finish-cleanly = The scan did not finish cleanly
scans-no-threats-found = No threats found
scans-not-create-eicar-test = Could not create the EICAR test file: { $error }
scans-add-files-tooltip = Add Files
scans-add-folder-manual-tooltip = Add Folder
scans-more-tooltip = More
scans-run-eicar-test-label = Run EICAR Test
scans-copy-results-label = Copy Results
scans-export-results-label = Export Results…
scans-start-scan-label = Start Scan
scans-cancel-manual-label = Cancel
scans-lazaret-refuses-clamav-title = { -brand-name } refuses a ClamAV program that your account could replace
scans-details-button = Details
scans-profile-title = Scan Profile
scans-profile-short-title = Profile
scans-manage-profiles-tooltip = Manage Profiles
scans-targets-manual-title = Scan Targets
scans-drop-files-folders-label = Drop files or folders to scan
scans-add-files-label = Add Files…
scans-add-folder-label = Add Folder…
scans-found-far-scan-title = Found so far
scans-new-scan-button = New Scan
scans-recent-scans-title = Recent scans
scans-started-lazaret-newest-description = Scans started from { -brand-name }, newest first. Select one to read its transcript.
scans-choose-files-scan = Choose Files to Scan
scans-choose-folder-scan = Choose a Folder to Scan
scans-quick-scan = Quick Scan
scans-full-scan = Full Scan
scans-home-folder = Home Folder
scans-fast-scan-downloads-folder = Fast scan of the Downloads folder for quick threat detection
scans-comprehensive-system-wide-scan = Comprehensive system-wide scan of all accessible directories
scans-user-home-directory-personal = Scan of the user's home directory and personal files
scans-profile-file-not-accessed = The profile file could not be accessed: { $error }
scans-profile-data-not-valid = The profile data is not valid: { $error }
scans-profile-file-not-replaced = The profile file could not be replaced: { $error }
scans-profile-task-interrupted-error = The profile task was interrupted: { $error }
scans-profile-name-required = Profile name is required
scans-name-50-characters-less = Name must be 50 characters or less
scans-another-profile-already-uses = Another profile already uses this name
scans-add-least-one-target = Add at least one target
scans-edit-profile-title = Edit Profile
scans-save-label = Save
scans-name-title = Name
scans-description-title = Description
scans-targets-profile-title = Targets
scans-add-folder-profile-tooltip = Add Folder
scans-add-path-title = Add Path
scans-exclusions-title = Exclusions
scans-added-exclusions-preferences-description = Added to the exclusions from Preferences while this profile runs.
scans-add-pattern-title = Add Pattern
scans-cancel-profile = Cancel
scans-default-profiles-restored = Default profiles restored
scans-profile-deleted = Profile deleted
scans-profile-imported = Profile imported
scans-manage-profiles-title = Manage Profiles
scans-restore-defaults-tooltip = Restore Defaults
scans-new-profile-tooltip = New Profile
scans-import-tooltip = Import
scans-profiles-title = Scan Profiles
scans-select-profile-edit-description = Select a profile to edit or use for scanning
scans-restore-default-profiles = Restore Default Profiles
scans-restore-built-default-profiles = Restore the built-in default profiles? Any customizations to default profiles will be reset; custom profiles are preserved.
scans-delete-profile = Delete Profile
scans-delete-profile-name = Delete the profile '{ $name }'?
scans-delete-label = Delete
scans-profile-exported = Profile exported
scans-remove-profile-tooltip = Remove
scans-count-target =
    { $count ->
        [one] { $count } target
       *[other] { $count } targets
    }
scans-default-profile = Default profile
scans-run-tooltip = Run
scans-edit-tooltip = Edit
scans-delete-tooltip = Delete
scans-export-tooltip = Export
scans-remove-target-tooltip = Remove
scans-check-again-access-tooltip = Check again
scans-checking-access-tooltip = Checking
scans-status-access-title = Status
scans-access-settings-changed-description = On-access settings changed through { -brand-name }, as recorded by the system. Software running as you can't remove these records.
scans-no-access-settings-title = No on-access settings were changed through { -brand-name }
scans-clamonacc-log-title = clamonacc log
scans-clamonacc-not-written-title = clamonacc has not written to the journal
scans-checking-access-scanning = Checking on-access scanning…
scans-command-copied-clipboard-access = Command copied to clipboard
scans-no-readable-clamd-conf = No readable clamd.conf was found
scans-reading-clamd-conf-stopped = Reading clamd.conf stopped before it finished
scans-path-whole-mount = { $path } (whole mount)
scans-none-configured = None configured
scans-clamav-default = The ClamAV default
scans-configuration-label = Configuration
scans-watched-folders = Watched folders
scans-largest-file-scanned = Largest file scanned
scans-neither-proc-config-gz = neither /proc/config.gz nor the kernel's config file in /boot exists
scans-kernel-configuration-not-read = the kernel configuration could not be read
scans-proc-config-not-decompressed = /proc/config.gz could not be decompressed
scans-running-kernel-not-read = the running kernel release could not be read
scans-reading-kernel-configuration-stopped = reading the kernel configuration stopped before it finished
scans-detected-files-blocked-status = Detected files are blocked
scans-files-checked-detected-status = Files are checked, but detected files are not blocked
scans-files-onaccessprevention-status = Files are checked, but detected files are not blocked, because OnAccessPrevention doesn't work with OnAccessMountPath
scans-files-checked-kernel-status = Files are checked, but this kernel can't block detected files, because it was built without CONFIG_FANOTIFY_ACCESS_PERMISSIONS
scans-files-checked-whether-status = Files are checked, but whether the kernel can block detected files is unknown, because { $reason }
scans-running-clamd-conf-status = Running, but clamd.conf could not be read, so what is checked and whether detected files are blocked is unknown
scans-running-no-folders-status = Running, but no folders or mounts are watched
scans-running-detected-status = Running, but no folders or mounts are watched, and detected files would not be blocked
scans-unknown-whether-access-status = Unknown whether on-access scanning runs, because systemd could not report it
scans-access-not-set-status = On-access scanning is not set up
scans-unit-not-running-status = { $unit } is not running
scans-access-blocking-title = On-access blocking
scans-clamonacc-checks-files-watched = clamonacc checks files in the watched folders as they are opened. With prevention on, the kernel refuses access to a detected file, which is why such a file can't be opened even though it is still there.
scans-mount-watching = Mount watching
scans-onaccessmountpath-watches-whole-mount = OnAccessMountPath watches a whole mount through fanotify. ClamAV documents it as incompatible with OnAccessPrevention, OnAccessIncludePath, OnAccessExcludePath and OnAccessExtraScanning, so detected files on a watched mount are never blocked. Watch folders with OnAccessIncludePath to block.
scans-onaccessmountpath-cannot-combined-options = OnAccessMountPath can't be combined with { $options }
scans-loop-protection = Scan loop protection
scans-clamd-opens-every-file = clamd opens every file it is asked to scan. Unless OnAccessExcludeUname or OnAccessExcludeUID names the user clamd runs as, clamonacc sees those reads and asks clamd to scan the same files again.
scans-clamd-not-excluded-clamonacc = clamd isn't excluded, so clamonacc can scan its own reads endlessly
scans-watching-whole-file-system = Watching the whole file system
scans-clamonacc-refuses-onaccessincludepath-include = clamonacc refuses OnAccessIncludePath /. Include the folders that receive files, such as home directories, or watch a mount with OnAccessMountPath.
scans-onaccessincludepath-rejected-clamonacc = OnAccessIncludePath / is rejected by clamonacc
scans-folder-watch-limit = Folder watch limit
scans-path-watched-fs-inotify = { $path } is watched, but fs.inotify.max_user_watches is only { $limit }, so folders past the limit go unchecked
scans-unknown-whether-path-fits = Unknown whether { $path } fits the folder watch limit, because fs.inotify.max_user_watches could not be read
scans-clamonacc-watches-every-folder = clamonacc watches every folder below an included path with inotify, one watch per folder. A whole home directory or larger tree can need more watches than the old default limit of 8192 allows. Raise fs.inotify.max_user_watches with sysctl.
scans-device-auto-scan-title = Device Auto-Scan
scans-automatically-scan-removable-description = Automatically scan removable and external drives when they are connected.
scans-enable-device-auto-title = Enable Device Auto-Scan
scans-max-device-size-title = Max Device Size (GB)
scans-no-size-limit-subtitle = 0 means no size limit.
scans-skip-battery-device-title = Skip on Battery
scans-drive-scan-not-added = The drive scan could not be added to the history: { $error }
scans-device-auto-scan = Device Auto-Scan
scans-device-connected-during-another = A device was connected during another scan, so it was not scanned.
scans-removable-drives-title = Removable Drives
scans-cancel-removable-label = Cancel
scans-found-far-removable-title = Found so far
scans-drives-session-title = Drives this session
scans-drives-connected-since-description = Drives connected since { -brand-name } started, newest first, and what happened to each. This list is kept only until { -brand-name } quits.
scans-no-drive-connected-title = No drive was connected yet
scans-close-results-button = Close Results
scans-drive-scans-title = Drive scans
scans-drives-lazaret-scanned-description = Drives { -brand-name } scanned when they were connected, newest first. Select one to read its transcript.
scans-off-label = Off
scans-every-drive = Scans every drive
scans-drives-up-size = Scans drives up to { $size }
scans-not-scanned-size-larger = Not scanned: { $size } is larger than the { $limit } limit
scans-not-scanned-computer-running = Not scanned: the computer was running on battery
scans-not-scanned-device-auto = Not scanned: device auto-scan is off
scans-not-scanned-another-scan = Not scanned: another scan was running
scans-not-scanned-size-drive = Not scanned: the size of the drive could not be read
scans-checking-drive = Checking the drive…
scans-scanning-label = Scanning…
scans-scanned-threats-found = Scanned, threats found
scans-scanned-no-threats-found = Scanned, no threats found
scans-scanned-scan-not-finish = Scanned, but the scan did not finish cleanly
scans-cancelled-label = Scan cancelled
scans-scanning-drive = Scanning { $drive }
scans-drive-scan = Drive scan
scans-check-again-scan-tooltip = Check again
scans-checking-scan-tooltip = Checking
scans-status-scan-title = Status
scans-engine-detection-settings-description = Engine and detection settings changed through { -brand-name }, as recorded by the system. Software running as you can't remove these records.
scans-no-engine-detection-title = No engine or detection settings were changed through { -brand-name }
scans-clamd-log-title = clamd log
scans-no-clamd-log-title = No clamd log was found
scans-checking-scan-engine = Checking the scan engine…
scans-command-copied-clipboard-scan = Command copied to clipboard
scans-clamscan-not-started-error = clamscan could not be started: { $error }
scans-clamscan-not-answer-time = clamscan did not answer in time
scans-refused-path-account-replace = refused { $path }, because your account can replace it
scans-clamav-not-found-computer = ClamAV was not found on this computer
scans-clamav-installed = ClamAV installed
scans-clamav-engine-recognizes-malware = ClamAV is the engine that recognizes malware. { -brand-name } uses the copy installed on this computer and never bundles its own.
scans-clamd-still-uses-signatures = clamd still uses signatures { $loaded }, but { $disk } is on disk
scans-clamd-answering = clamd is answering
scans-unit-running-not-answer = { $unit } is running, but clamd does not answer. It may still be loading signatures.
scans-clamd-not-answer-systemd = clamd does not answer, and systemd could not report its state
scans-unit-not-running = { $unit } is not running
scans-clamd-not-set-up = clamd is not set up, so scans fall back to the slower clamscan
scans-same-version-disk = The same version as on disk
scans-loaded-disk = { $loaded }, while { $disk } is on disk
scans-unknown-loaded-stored-versions = Unknown, the loaded and stored versions could not be compared
scans-loaded-signatures = Loaded signatures
scans-daemon-running = Daemon running
scans-clamd-keeps-signatures-loaded = clamd keeps the signatures loaded in memory. On-access blocking and scheduled scans both send their files to it.
scans-system-replace-path = Only the system can replace { $path }
scans-account-replace-replaceable-program = Your account can replace { $replaceable }, so a program running as you could put a fake clamd at { $path } that reports every file as clean
scans-no-clamd-socket-path = No clamd socket at { $path }, so clamd isn't running or listens elsewhere
scans-flatpak-sandbox-cannot-see = The Flatpak sandbox can't see who owns the clamd socket
scans-clamd-socket-protected = clamd socket protected
scans-reach-clamd-through-socket = Scans reach clamd through its socket. Let clamd create it in a directory that only root or the clamav user can write, such as /run/clamav, so no program running as you can put a fake clamd in its place.
scans-checking-replace-clamd-socket = Checking who can replace the clamd socket…
scans-not-scheduled-scan = Not scheduled
scans-next-run-time = Next run: { $time }
scans-scheduled-time-next-run = Scheduled. The time of the next run is not known yet.
scans-scheduled-scans-need-systemd = Scheduled scans need a systemd user session, which is not available.
scans-system-already-runs-scheduled = Your system already runs scheduled scans, so these settings are locked to avoid a second schedule. Next scan: { $time }. Change it in your system configuration.
scans-system-scan-timer-settings = Your system has a scan timer, so these settings are locked to avoid a second schedule. Change it in your system configuration.
scans-checking-label = Checking…
scans-checking-whether-system-already = Checking whether your system already runs scheduled scans. These settings stay locked until then.
scans-lazaret-not-check-whether = { -brand-name } could not check whether your system already runs scheduled scans, so these settings are locked to avoid a second schedule: { $error }
scans-enter-scan-time-hh = Enter the scan time as HH:MM, for example 02:00.
scans-error-creating-systemd-files = Error creating systemd files: { $error }
scans-timer-not-applied-error = The timer could not be applied: { $error }
scans-hourly-label = Hourly
scans-daily-label = Daily
scans-weekly-label = Weekly
scans-monthly-label = Monthly
scans-monday-label = Monday
scans-tuesday-label = Tuesday
scans-wednesday-label = Wednesday
scans-thursday-label = Thursday
scans-friday-label = Friday
scans-saturday-label = Saturday
scans-sunday-label = Sunday
scans-schedule-title = Schedule
scans-apply-label = Apply
scans-enable-scheduled-scans-title = Enable Scheduled Scans
scans-frequency-title = Scan Frequency
scans-time-24-hour-title = Scan Time (24-hour)
scans-day-week-title = Day of Week
scans-used-weekly-scans-subtitle = Used for weekly scans.
scans-day-month-title = Day of Month
scans-used-monthly-scans-subtitle = Used for monthly scans.
scans-targets-separated-commas-title = Scan Targets (separated by commas; empty scans your home folder)
scans-skip-battery-schedule-title = Skip on Battery
scans-not-start-scheduled-subtitle = Do not start scheduled scans while on battery power.
scans-auto-quarantine-title = Auto-Quarantine
scans-automatically-quarantine-threats-subtitle = Automatically quarantine threats found by scheduled scans. While { -brand-name } is open, threats are only reported.
scans-review-button = Review
scans-check-again-scheduled-tooltip = Check again
scans-checking-scheduled-tooltip = Checking
scans-system-scan-title = System scan
scans-folders-scanned-title = Folders scanned
scans-run-now-label = Run Now
scans-starting-scan-tooltip = Starting the scan…
scans-status-scheduled-title = Status
scans-scheduled-runs-title = Scheduled runs
scans-lazaret-schedule-ran-description = Scans { -brand-name }'s schedule ran, newest first. Select one to read its transcript.
scans-checking-schedule = Checking the schedule…
scans-command-copied-clipboard-scheduled = Command copied to clipboard
scans-not-scheduled = Not scheduled
scans-next-scan = Next scan
scans-threats-found-scheduled = threats found
scans-nothing-found = nothing found
scans-not-run-yet = Not run yet
scans-last-scan = Last scan
scans-unknown-label = Unknown
scans-system-timer-not-active = The system timer is not active
scans-last-scan-ended-error = The last scan ended with an error
scans-next-scan-time = Next scan: { $time }
scans-timer-starts-scan-service = The timer starts a scan service that is not installed
scans-service-not-read-systemd = The scan service could not be read from systemd
scans-scheduled-lazaret = Scheduled by { -brand-name }
scans-no-scan-scheduled = No scan is scheduled
scans-not-scheduled-timer-active = Not scheduled, because the timer is not active
scans-folders-scanned = Folders scanned
scans-scheduled-scan = Scheduled scan
scans-scheduled-scan-catches-files = A scheduled scan catches files that were not opened, or that only became detectable after a signature update.
scans-unit-names-no-folders = The scan unit names no folders
scans-system-scan-service-unit = Your system's scan service { $unit } scans these folders with the system's own settings. Running it needs administrator authorization.
scans-needs-daemon-see-scan = Needs the daemon, see Scan Engine
scans-systemd-not-answer-error = systemd did not answer: { $error }
scans-systemd-reported-unit-state = systemd reported a unit state { -brand-name } does not recognize
scans-found-malware = The scan found malware
scans-finished-label = The scan finished
scans-not-finish-cleanly-result = The scan did not finish cleanly: { $result }
scans-starting-scan-needs-administrator = Starting the scan needs administrator authorization
scans-not-started-error = The scan could not be started: { $error }

findings-open-preferences = Open Preferences
findings-system-journal-cannot-read = The system journal can't be read. Add your user to the systemd-journal group.
findings-notifications-turned-off-preferences = Notifications are turned off in the preferences
findings-alerts-stop-quit-lazaret = Alerts stop when you quit { -brand-name }, because it isn't set up to run in the background
findings-running-background-following-journal = Running in the background and following the journal
findings-alerts-label = Alerts
findings-lazaret-follows-system-journal = { -brand-name } follows the system journal and notifies you when something is detected. It can only do that while it is running, so it should run in the background.
findings-on-label = On
findings-off-label = Off
findings-running-label = Running
findings-not-running = Not running
findings-not-set-up = Not set up
findings-unknown-systemd-cannot-reached = Unknown, systemd can't be reached
findings-followed-label = Followed
findings-cannot-read = Can't be read
findings-following-stopped = Following stopped
findings-notifications-label = Notifications
findings-background-service = Background service
findings-system-journal = System journal
findings-attach-unknown-setting = Whether other programs can attach is unknown, because the kernel setting can't be read: { $reason }
findings-attach-unknown-reports = Whether other programs can attach is unknown, because the kernel reports a value { -brand-name } doesn't recognize
findings-attach-unknown-check = Whether other programs can attach is unknown, because the check stopped before it finished
findings-programs-cannot-attach-other = Programs can't attach to other programs you run, such as { -brand-name }
findings-program-run-attach-lazaret = Any program you run can attach to { -brand-name } and change what it shows, including detections and the decisions you make here
findings-kernel-not-restrict-attaching = This kernel doesn't restrict attaching, so any program you run can attach to { -brand-name } and change what it shows
findings-other-programs-cannot-attach = Other programs can't attach to { -brand-name }
findings-program-attaches-lazaret-hide = A program that attaches to { -brand-name } can hide alerts or change your decisions. Setting kernel.yama.ptrace_scope to 1 lets only a program's parent attach to it.
findings-file-other-owner = This file belongs to another user, so it can't be moved or deleted from here
findings-file-not-checked-cannot = This file couldn't be checked, so it can't be moved or deleted from here
findings-file-not-deleted-error = The file could not be deleted: { $error }
findings-checking-file-stopped-unexpectedly = Checking the file stopped unexpectedly
findings-decision-not-saved-error = Your decision could not be saved: { $error }
findings-lazaret-cannot-read-system = { -brand-name } can't read the system journal, so new background detections won't appear here. This does not mean nothing was found. Add your user to the systemd-journal group.
findings-loading-detections = Loading detections…
findings-nothing-detected-background = Nothing has been detected in the background.
findings-access-scanning-scheduled-scans = What on-access scanning and scheduled scans have found.
findings-detections-title = Detections
findings-back-detections-tooltip = Back to detections
findings-alerts-title = Alerts
findings-detections-reach-keeps-description = How detections reach you, and what keeps other programs from hiding them.
findings-check-again-background-tooltip = Check again
findings-checking-background-tooltip = Checking
findings-detection-title = Detection
findings-copied-clipboard-toast = Copied to clipboard
findings-command-copied-clipboard-toast = Command copied to clipboard
findings-detections-lazaret-stopped = Detections while { -brand-name } wasn't running can't be counted, because where it stopped reading the system journal could not be read: { $error }
findings-detections-not-read-error = Detections could not be read: { $error }
findings-malware-blocked-signature = Malware blocked: { $signature }
findings-malware-found-signature = Malware found: { $signature }
findings-count-detection-not-running =
    { $count ->
        [one] { $count } detection while { -brand-name } wasn't running
       *[other] { $count } detections while { -brand-name } wasn't running
    }
findings-detections-not-running-detection = Detections while { -brand-name } wasn't running can't be counted
findings-lazaret-not-read-stopped = { -brand-name } couldn't read where it stopped following the system journal, so it only watches for new detections. Check the Detections page and the system journal for anything found while it was closed.
findings-quarantine-label = Quarantine
findings-delete-incident = Delete…
findings-delete-permanently = Delete Permanently
findings-cancel-incident = Cancel
findings-keep-label = Keep
findings-look-up = Look Up
findings-add-key = Add Key…
findings-submit-incident = Submit…
findings-restored-quarantine = Restored from quarantine
findings-moved-quarantine = Moved to quarantine
findings-deleted-label = Deleted
findings-kept-label = Kept by you
findings-belongs-another-user = Belongs to another user
findings-blocked-needs-decision = Blocked, needs a decision
findings-not-blocked-needs-decision = Not blocked, needs a decision
findings-needs-decision = Needs a decision
findings-file-gone-label = No longer there
findings-file-not-checked = File could not be checked
findings-signature-test-file-state = { $signature } (test file) · { $state }
findings-signature-state = { $signature } · { $state }
findings-access-scanning-scheduled-scan = On-access scanning and the scheduled scan
findings-access-scanning = On-access scanning
findings-scheduled-scan = The scheduled scan
findings-file-label = File
findings-signature-source = Signature source
findings-found-label = Found by
findings-last-seen = Last seen
findings-times-seen = Times seen
findings-detected-label = Detected
findings-signature-matches-harmless-eicar = This signature matches the harmless EICAR test pattern, which exists to check that detection works.
findings-clamav-matched-file-against = ClamAV matched this file against a signature. A match is strong evidence, but false positives happen, especially with packed or administrative tools.
findings-signature-first-seen-time = { $signature }, first seen { $time }
findings-opening-file-refused-why = Opening this file is refused, which is why it won't open
findings-not-opened-again-after = Not opened again after your decision
findings-file-no-longer = The file is no longer there
findings-not-blocked-scheduled-scan = Not blocked: the scheduled scan found it, and it can still be opened
findings-not-blocked-right-now = Not blocked right now: it can still be opened
findings-not-check-whether-file = Could not check whether the file is blocked
findings-blocked-label = Blocked
findings-access-prevention-kernel-refuses = With on-access prevention, the kernel refuses to open a detected file in a watched folder. The file stays where it is until you decide what to do with it.
findings-malwarebazaar-not-reached-error = MalwareBazaar could not be reached: { $error }
findings-malwarebazaar-sent-not-read = MalwareBazaar sent an answer that could not be read
findings-malwarebazaar-refused-api-key = MalwareBazaar refused the API key
findings-malwarebazaar-answered-status = MalwareBazaar answered: { $status }
findings-looking-up = Looking up…
findings-add-abuse-ch-key = Add your abuse.ch key in Preferences to look up detections.
findings-no-known-samples-match = No known samples match this signature.
findings-unknown-label = unknown
findings-count-known-samples-families = { $count } known samples. Families: { $families }. Earliest seen: { $date }.
findings-ask-malwarebazaar-signature-description = Ask MalwareBazaar about this signature. Only its name is sent.
findings-malwarebazaar-needs-free-description = MalwareBazaar needs a free abuse.ch key
findings-stop-asking-file-description = Stop asking about this file
findings-delete-file-permanently-description = Delete this file permanently? This can't be undone.
findings-go-back-without-description = Go back without deleting
findings-move-somewhere-safe-description = Move it somewhere safe. You can restore it later.
findings-remove-good-description = Remove it for good
findings-know-file-fine-description = You know this file is fine. ClamAV still reports it at the next scan; trust it from Quarantine or a scan result to stop that.
findings-decision-title = Your decision
findings-unfamiliar-quarantine-keeping-file = If it's unfamiliar, quarantine it. Keeping a file only stops { -brand-name } asking: on-access scanning keeps blocking it until the path is excluded in the ClamAV configuration, as shown here.
findings-quarantined-files-restored-quarantine = Quarantined files can be restored from the Quarantine page. A file you kept is still blocked by on-access scanning until its path is excluded in the ClamAV configuration.
findings-file-other-owner-decision = This file belongs to another user, so { -brand-name } can't move or delete it
findings-lazaret-not-check-file = { -brand-name } couldn't check this file, so it can't move or delete it
findings-choose-happens-file = Choose what happens to this file
findings-malwarebazaar-label = MalwareBazaar
findings-resolved-title = Resolved
findings-resolved-detection-opens-again = A resolved detection opens again if ClamAV finds the same file again later, for example after you download it a second time.
findings-waiting-decision = Waiting for your decision
findings-state-time = { $state }, { $time }
findings-send-quarantined-copy-description = Send the quarantined copy to ClamAV, MalwareBazaar or VirusTotal. A false positive reported to ClamAV is fixed for everyone by a later signature update.
findings-cancel-quarantine = Cancel
findings-delete-entries = Delete
findings-scanner-program-not-installed = the scanner program is not installed
findings-scanner-program-replaced-account = the scanner program at { $program } can be replaced by your account through { $path }
findings-file-not-passed-scanner = the file could not be passed to the scanner: { $error }
findings-scanner-not-started-error = the scanner could not be started: { $error }
findings-scan-took-too-long = the scan took too long
findings-clamd-detection-not-read = the clamd detection settings could not be read: { $error }
findings-scanner-programs-not-looked = the scanner programs could not be looked up: { $error }
findings-quarantined-copy-not-read = the quarantined copy could not be read: { $error }
findings-restore-file-heading = Restore This File?
findings-clamav-no-longer-verdict = ClamAV no longer detects this file with today's signatures.
findings-restore-button = Restore
findings-restore-file-clamav-heading = Restore a File ClamAV Still Detects?
findings-clamav-still-detects-verdict = ClamAV still detects this file as { $detection } with today's signatures. Only restore it if you are sure it is a false positive, and consider trusting it first.
findings-restore-anyway-button = Restore Anyway
findings-restore-unchecked-file-heading = Restore an Unchecked File?
findings-file-not-checked-verdict = The file could not be checked again before restoring it: { $error }
findings-location-outside-home-folder = This location is outside your home folder.
findings-file-goes-back = The file goes back to:
findings-count-item =
    { $count ->
        [one] { $count } item
       *[other] { $count } items
    }
findings-clear-old-items = Clear Old Items?
findings-count-quarantined-file-older =
    { $count ->
        [one] { $count } quarantined file older than 30 days will be permanently deleted.
       *[other] { $count } quarantined files older than 30 days will be permanently deleted.
    }
findings-delete-quarantined-file = Delete Quarantined File?
findings-file-permanently-deleted = The file will be permanently deleted.
findings-quarantine-title = Quarantine
findings-more-tooltip = More
findings-clear-old-items-label = Clear Old Items…
findings-check-again-label = Check Again
findings-scan-every-quarantined-tooltip = Scan every quarantined file with the current signatures
findings-refresh-tooltip = Refresh
findings-search-tooltip = Search
findings-search-threat-name-placeholder = Search by threat name or path
findings-quarantine-storage-title = Quarantine Storage
findings-detected-threats-isolated-description = Detected threats isolated from your system. Expand an entry to check its reputation.
findings-total-size-title = Total Size
findings-dismiss-label = Dismiss
findings-quarantine-cannot-read-title = Quarantine can't be read
findings-loading-quarantine = Loading quarantine…
findings-quarantine-empty = Quarantine is empty
findings-hash-copied-clipboard = Hash copied to clipboard
findings-every-quarantined-file-still = Every quarantined file is still detected or couldn't be checked
findings-count-quarantined-file-no =
    { $count ->
        [one] { $count } quarantined file is no longer detected. Check it before restoring it.
       *[other] { $count } quarantined files are no longer detected. Check them before restoring them.
    }
findings-count-report-clamav-settled =
    { $count ->
        [one] { $count } report to ClamAV is settled by the new signatures
       *[other] { $count } reports to ClamAV are settled by the new signatures
    }
findings-checking-label = Checking…
findings-no-longer-detected = No longer detected
findings-still-detected = Still detected
findings-unknown-quarantine-record-no = Unknown. This quarantine record has no usable hash, so the file cannot be verified, checked online, trusted or restored. You can still delete it.
findings-not-checked = Not checked yet
findings-integrity-unknown-label = Integrity Unknown
findings-restore-label = Restore
findings-delete-entry-label = Delete
findings-reputation-title = Reputation
findings-file-sha-256-subtitle = Only the file's SHA-256 hash is sent. The file itself never leaves this computer.
findings-check-label = Check
findings-add-key-label = Add Key
findings-view-report-label = View Report
findings-false-positive-title = False Positive
findings-trust-stops-clamav-subtitle = Trust… stops ClamAV reporting this exact file on this computer and lists it under Trusted Detections; restore it afterwards. Report… sends it to Cisco Talos, so a later signature update fixes the detection for everyone.
findings-report-label = Report…
findings-submit-quarantine-label = Submit…
findings-send-clamav-malwarebazaar-tooltip = Send it to ClamAV, MalwareBazaar or VirusTotal
findings-trust-label = Trust…
findings-copy-hash-tooltip = Copy Hash
findings-submit-quarantined-label = Submit…
findings-service-label = Service
findings-status-label = Status
findings-quarantine-record-no-usable = This quarantine record has no usable hash, so { -brand-name } doesn't send it anywhere
findings-file-no-longer-quarantine = This file is no longer in quarantine
findings-service-not-take-detection = This service doesn't take this detection
findings-file-no-usable-hash = This file is no longer in quarantine with a usable hash
findings-check-malwarebazaar-virustotal-tooltip = Check MalwareBazaar and VirusTotal for news
findings-checking-sent-tooltip = Checking
findings-send-file-title = Send a File
findings-nothing-leaves-computer-description = Nothing leaves this computer until you confirm what is sent.
findings-report-missed-malware-label = Report Missed Malware…
findings-send-clamav-malicious-tooltip = Send ClamAV a malicious file it doesn't detect
findings-sent-title = Sent
findings-lazaret-sent-became-description = What { -brand-name } sent and what became of it.
findings-filter-sent-samples-tooltip = Filter Sent Samples
findings-online-services-title = Online Services
findings-each-service-receives-subtitle = What each service receives, and the MalwareBazaar and VirusTotal keys
findings-choose-file-clamav-misses = Choose the File ClamAV Misses
findings-false-positive = False positive
findings-missed-malware = Missed malware
findings-malware-sample = Malware sample
findings-report-false-positive-clamav = Report False Positive to ClamAV
findings-report-missed-malware-clamav = Report Missed Malware to ClamAV
findings-upload-malwarebazaar = Upload to MalwareBazaar
findings-upload-virustotal = Upload to VirusTotal
findings-awaiting-clamav-review-reviews = Awaiting ClamAV review. Reviews take at least 48 hours; a later signature update shows the outcome.
findings-fixed-database-version = Fixed in database version { $version }
findings-fixed-current-signature-database = Fixed in the current signature database
findings-detected-since-database-version = Detected since database version { $version }
findings-detected-current-signature-database = Detected by the current signature database
findings-uploaded-waiting-malwarebazaar-list = Uploaded; waiting for MalwareBazaar to list it
findings-listed-malwarebazaar = Listed on MalwareBazaar
findings-malwarebazaar-refused-reason = MalwareBazaar refused it: { $reason }
findings-queued-analysis-virustotal = Queued for analysis on VirusTotal
findings-count-total-engine-flags =
    { $total ->
        [one] { $count } of { $total } engine flags it as malicious
       *[other] { $count } of { $total } engines flag it as malicious
    }
findings-virustotal-refused-reason = VirusTotal refused it: { $reason }
findings-service-kind-hash-date = { $service } · { $kind } · { $hash } · { $date }
findings-detection-no-usable-hash = { $detection } · No usable hash, so { -brand-name } can't prove what it would send
findings-all-services-option = All services
findings-any-status-option = Any status
findings-waiting-label = Waiting
findings-settled-label = Settled
findings-refused-label = Refused
findings-all-submissions-option = All submissions
findings-loading-submissions = Loading submissions…
findings-nothing-submitted = Nothing submitted
findings-no-submission-matches-filters = No submission matches the filters
findings-loading-quarantine-sent = Loading quarantine…
findings-quarantine-empty-sent = Quarantine is empty
findings-service-needs-key-check = { $service } needs a key to check uploads
findings-no-upload-changed = No upload changed
findings-count-upload-new-status =
    { $count ->
        [one] { $count } upload has a new status
       *[other] { $count } uploads have a new status
    }
findings-choose-service-heading = Where Should This File Go?
findings-each-service-gets-file = Each service gets the file itself. The next step shows exactly what is sent and who can see it.
findings-cancel-submission = Cancel
findings-open-remote-entry-tooltip = Open the Remote Entry
findings-file-already-sent-service = This file is already being sent to this service. Wait for that to finish before sending it again.
findings-check-again-trusted-tooltip = Check again
findings-checking-trusted-tooltip = Checking
findings-status-title = Status
findings-files-trusted-detections-description = Files trusted and detections ignored or restored through { -brand-name }, as recorded by the system. Software running as you can't remove these records.
findings-no-trusted-detections-title = No trusted detections were changed through { -brand-name }
findings-checking-trusted-detections = Checking trusted detections…
findings-command-copied-clipboard = Command copied to clipboard
findings-other-allowlist = Other allowlist
findings-file-reason = { $file }: { $reason }
findings-trusted-files = Trusted files
findings-ignored-detections = Ignored detections
findings-allowlist-not-write-status = An allowlist that { -brand-name } didn't write tells ClamAV to stay silent
findings-count-file-detection-status =
    { $count ->
        [one] { $count } file or detection is never reported
       *[other] { $count } files or detections are never reported
    }
findings-own-allowlist-unreadable-status = { -brand-name } can't read its own allowlist, so it can't tell what ClamAV stays silent about
findings-signature-folder-unreadable-status = { -brand-name } can't read the signature folder, so it can't tell what ClamAV stays silent about
findings-signature-folder = Signature folder
findings-trusted-detections = Trusted detections
findings-files-detection-names-clamav = Files and detection names on ClamAV's allowlists are never reported, by any scan. Malware that gets administrator rights could hide itself this way, so check that every entry was added on purpose.
findings-clamav-reports-everything-detects = ClamAV reports everything it detects
findings-remove-tooltip = Remove
findings-permission-denied = Permission denied
findings-reading-failed-reason = Reading failed: { $reason }
findings-reading-stopped-before-finished = Reading stopped before it finished
findings-unknown-added-reason = Unknown who added this. { $reason }
findings-added-time-user-uid = Added { $time } by user { $uid }
findings-added-time-unknown-user = Added { $time } by an unknown user
findings-no-record-added-system = No record of who added this in the system journal
findings-count-entry-not-added =
    { $count ->
        [one] { $count } entry, not added through { -brand-name }
       *[other] { $count } entries, not added through { -brand-name }
    }
findings-entries-cannot-counted-reason = Entries can't be counted ({ $reason }), not added through { -brand-name }
findings-lazaret-cannot-read-list = { -brand-name } can't read this list
findings-no-trusted-files = No trusted files
findings-no-ignored-detections = No ignored detections
findings-lazaret-cannot-read-signature = { -brand-name } can't read the signature folder
findings-trusted-detections-title = Trusted Detections
findings-clamav-never-reports-description = ClamAV never reports what is listed here, in any scan on this computer. Every change needs your administrator password and is recorded in the system journal, where software running as you can't remove the record.
findings-trusted-files-title = Trusted Files
findings-files-trusted-sha-description = Files trusted by their SHA-256 hash. Any change to such a file makes ClamAV check it again.
findings-ignored-detections-title = Ignored Detections
findings-detection-names-clamav-description = Detection names that ClamAV ignores in every file.
findings-other-allowlists-title = Other Allowlists
findings-allowlist-files-signature-description = Allowlist files in the signature folder that { -brand-name } didn't write. Check that you or your administrator added them on purpose.

signatures-failed-load-historical-logs = Failed to load historical logs: { $error }
signatures-database-update-finished = Database update finished
signatures-check-again-database-tooltip = Check again
signatures-checking-database-tooltip = Checking
signatures-update-history-title = Update history
signatures-updates-started-lazaret-description = Updates started from { -brand-name }, newest first. Select one to read its transcript.
signatures-historical-logs-cannot-title = Historical logs can't be read
signatures-status-title = Status
signatures-loading-history = Loading history…
signatures-no-updates-started-lazaret = No updates were started from { -brand-name } yet
signatures-checking-signatures = Checking the signatures…
signatures-command-copied-clipboard-database = Command copied to clipboard
signatures-update-not-added-history = The update could not be added to the history: { $error }
signatures-update-now = Update Now
signatures-cancel-label = Cancel
signatures-definitions-built-date = Definitions built { $when } ({ $date })
signatures-definitions-exist-not-determined = Definitions exist, but their age could not be determined
signatures-no-virus-definitions-found = No virus definitions were found
signatures-age-definitions-unknown-checking = The age of the definitions is unknown, because checking them stopped before it finished
signatures-last-update-failed = Its last update failed
signatures-no-completed-update-recorded = No completed update recorded yet
signatures-updater-updated = { $updater } updated
signatures-provider-fetched = { $provider } fetched
signatures-count-signature-size-file =
    { $count ->
        [one] { $count } signature, { $size } ({ $file })
       *[other] { $count } signatures, { $size } ({ $file })
    }
signatures-unknown-number-signatures-file = Unknown number of signatures, the file could not be read: { $error }, { $size } ({ $file })
signatures-other-signature-sources = Other signature sources
signatures-unknown-reason-database = Unknown. { $reason }
signatures-fetch-newest-signatures = Fetch the newest signatures from every updater now
signatures-manual-update = Manual update
signatures-update-service = Update service
signatures-database-folder = Database folder
signatures-updater-output = Updater output
signatures-current-label = Signatures current
signatures-describe-known-malware-new = Signatures describe known malware, and new ones are published constantly. Old signatures miss recent threats even when everything else is running.
signatures-updating-signatures = Updating signatures…
signatures-cancelling-update = Cancelling the update…
signatures-starting-update = Starting the update…
signatures-active-label = Active
signatures-scheduled-systemd-timer = Scheduled by a systemd timer
signatures-installed-stopped = Installed but stopped
signatures-no-freshclam-service-detected = No freshclam service detected; updates run manually
signatures-systemd-not-asked-freshclam = systemd could not be asked about a freshclam service; updates run manually
signatures-count-day-old =
    { $count ->
        [one] { $count } day old
       *[other] { $count } days old
    }
signatures-no-readable-virus-database = No readable virus database was found
signatures-unknown-reason-update = Unknown. { $reason }
signatures-freshclam-failed-exit-code = freshclam failed with exit code { $code }.
signatures-freshclam-stopped-signal = freshclam was stopped by signal { $signal }.
signatures-freshclam-ended-way-lazaret = freshclam ended in a way { -brand-name } does not recognize.
signatures-lazaret-lost-track-freshclam = { -brand-name } lost track of freshclam before it finished.
signatures-authorization-failed = Authorization failed. Please try again and enter your password.
signatures-systemd-not-reached = Systemd could not be reached to run the update.
signatures-update-service-not-finish = The update service did not finish; systemd reported: { $result }
signatures-update-rate-limited-mirror = Update rate limited by mirror. Please wait a few minutes and try again.
signatures-update-blocked-cdn = Update blocked by CDN. This may be due to rate limiting. Please wait and try again later.
signatures-clamav-mirror-currently-unavailable = ClamAV mirror is currently unavailable. Please try again later.
signatures-ssl-tls-certificate-error = SSL/TLS certificate error. The mirror may have configuration issues.
signatures-connection-timed-out = Connection timed out. Please check your network connection.
signatures-database-locked-another-freshclam = Database is locked. Another freshclam instance may be running.
signatures-permission-denied-need-elevated = Permission denied. You may need elevated privileges to update the database.
signatures-connection-error = Connection error. Please check your network connection.
signatures-dns-resolution-failed = DNS resolution failed. Please check your network settings.
signatures-update-failed-unknown-error = Update failed with an unknown error. Check the logs for details.
signatures-updated-databases = Updated: { $databases }
signatures-already-current-databases = Already current: { $databases }
signatures-rate-limited-database = Rate limited: { $database }
signatures-rate-limited-database-until = Rate limited: { $database } until { $cooldown }
signatures-database-update-completed = Database update completed
signatures-database-already-up-date = Database is already up to date
signatures-update-service-ran-log = The update service ran, but its log does not say whether definitions changed.
signatures-service-accepted-signal-performs = The service accepted the signal; it performs the update asynchronously. Check its logs for completion.
signatures-database-update-cancelled = Database update cancelled
signatures-install-freshclam-before-requesting = Install freshclam before requesting database updates.
signatures-provider-needs-credential-fangfrisch = { $provider } needs a { $credential }, and fangfrisch only reads it from its settings. On NixOS, services.clamav.fangfrisch.settings is copied into the Nix store, which every user of this computer can read, so the { $credential } would be exposed. Leave { $provider } off, or run fangfrisch with a settings file kept outside the Nix store, such as an agenix or sops-nix secret.
signatures-provider-needs-credential-website = { $provider } needs a { $credential } from its website. fangfrisch reads it from its settings file, usually { $path }. Add these lines there, replace the placeholder with your { $credential }, and make the file readable by root only, so other users can't see it.
signatures-not-use = Not in use
signatures-add-lines-fangfrisch-settings = Add these lines to fangfrisch's settings file, usually { $path }.
signatures-provider-needs-free-abuse = { $provider } needs a free abuse.ch Auth-Key. Add these lines to fangfrisch's settings file, usually { $path }, replace the placeholder with your key, and make the file readable by root only, so other users can't see it.
signatures-use-label = In use
signatures-unknown-reason-source = Unknown. { $reason }
signatures-check-again-signature-tooltip = Check again
signatures-checking-signature-tooltip = Checking
signatures-clamav-malware-signatures-description = Where ClamAV's malware signatures come from, and which other sources you could add.
signatures-sources-title = Signature Sources
signatures-abuse-ch-key-title = abuse.ch Key
signatures-abuse-ch-auth-title = abuse.ch Auth-Key
signatures-command-copied-clipboard-signature = Command copied to clipboard
signatures-not-open-documentation-title = Could not open documentation: { $error }
signatures-urlhaus-signatures-through-fangfrisch = URLhaus signatures through fangfrisch, MalwareBazaar lookups and MalwareBazaar uploads all use this key. It is kept and edited in Preferences, under Online Services.
signatures-checking-api-key = Checking the API key…
signatures-pass-label = Pass
signatures-warning-label = Warning
signatures-fail-label = Fail
signatures-unknown-label = Unknown
signatures-skipped-label = Skipped
signatures-checking-label = Checking
signatures-copy-tooltip = Copy
signatures-learn-more-title = Learn more
signatures-open-browser-tooltip = Open in browser
signatures-checking-title = Checking…

preferences-lazaret-not-find-own = { -brand-name } could not find its own launcher: { $error }
preferences-file-manager-not-added = The file manager action could not be added: { $error }
preferences-file-manager-not-removed = The file manager action could not be removed: { $error }
preferences-dialog-title = Preferences
preferences-general-title = General
preferences-window-behavior-title = Window Behavior
preferences-detection-alerts-work-description = Detection alerts only work while { -brand-name } is running.
preferences-closing-window-title = When closing window
preferences-start-background-title = Start in the Background
preferences-launch-background-without-subtitle = Launch in the background without opening a window.
preferences-desktop-notifications-title = Desktop Notifications
preferences-notify-detections-finished-subtitle = Notify about detections, and about finished scans and updates while the window is not in focus.
preferences-file-manager-integration-title = File Manager Integration
preferences-add-context-menu-description = Add context menu actions for supported file managers.
preferences-online-services-title = Online Services
preferences-services-tell-more-description = Services that tell you more about a detected or quarantined file. Lookups run only when you press Look Up on a detection or Check in a quarantine entry, and send only a hash or a signature name. A file leaves this computer only when you submit it and confirm what is sent.
preferences-lookups-send-file-description = Lookups send only a file's SHA-256 hash and tell whether it belongs to a known, harmless file. No key needed.
preferences-no-lookups-report-description = No lookups. A report you confirm sends the file, the detection name for a false positive, and your name and email address. Talos processes samples by automation, never makes them public and doesn't share them with other companies. A review takes at least 48 hours. No key needed.
preferences-lookups-send-virustotal-file = Lookups send VirusTotal only a file's SHA-256 hash. An upload you confirm sends the quarantined file itself, which VirusTotal shares with its paying customers. Uploads are skipped when VirusTotal already has a report, and refused above 32 MB.
preferences-malwarebazaar-lookups-send-detection = MalwareBazaar lookups send a detection's signature name or a file's SHA-256 hash. An upload you confirm sends the quarantined file with its ClamAV signature as a tag, and the sample becomes public. The same abuse.ch key also unlocks URLhaus signature downloads through fangfrisch; on NixOS, { -brand-name } reuses fangfrisch's key file when your user can read it. abuse.ch's fair-use terms say for-profit users may need its commercial API.
preferences-free-tier-4-requests = Free tier: 4 requests per minute, 500 per day.
preferences-free-abuse-ch-authentication = Free from the abuse.ch Authentication Portal.
preferences-not-configured = Not configured
preferences-checking-label = Checking…
preferences-set-system-configuration-title = Set by your system configuration
preferences-key-keyring-title = Key in your keyring
preferences-delete-key-label = Delete Key
preferences-paste-api-key-title = Paste API Key
preferences-get-free-api-title = Get a free API key

widgets-clean-label = Clean
widgets-infected-label = Infected
widgets-error-status = Error
widgets-cancelled-label = Cancelled
widgets-skipped-label = Skipped
widgets-success-label = Success
widgets-up-date = Up to date
widgets-requested-label = Requested
widgets-ran-label = Ran
widgets-found-count-threat =
    { $count ->
        [one] Found { $count } threat
       *[other] Found { $count } threats
    }
widgets-scanner-reported-threat-activity = The scanner reported a threat without naming the file.
widgets-no-threats-found-activity = No threats found
widgets-scan-not-finish-activity = The scan did not finish cleanly
widgets-scan-cancelled-user-activity = Scan cancelled by user
widgets-scheduled-scan-skipped-battery = Scheduled scan skipped (on battery power)
widgets-virus-definitions-updated = Virus definitions updated
widgets-virus-definitions-up-date = Virus definitions are up to date
widgets-update-requested-service = Update requested from the service
widgets-update-service-no-result = The update service ran without reporting a result
widgets-database-update-failed = Database update failed
widgets-database-update-cancelled = Database update cancelled
widgets-scan-log = SCAN LOG
widgets-update-log = UPDATE LOG
widgets-timestamp-label = Timestamp
widgets-status-label = Status
widgets-duration-label = Duration
widgets-summary-label = Summary
widgets-path-label = Path
widgets-full-output = Full Output
widgets-scan-history-cannot-title = Scan history can't be read
widgets-retry-label = Retry
widgets-copy-activity-tooltip = Copy
widgets-log-details-copied-toast = Log details copied to clipboard
widgets-not-save-file-error = Could not save the file: { $error }
widgets-export-title = Export
widgets-database-made-newer-lazaret = This database was made by a newer { -brand-name }: it has layout version { $found }, and this version knows up to { $known }. Update { -brand-name } to open it.
widgets-updating-database-layout-failed = Updating the database layout failed at step { $step }: { $error }
widgets-database-layout-not-checked = The database layout version could not be checked: { $error }
widgets-copy-configuration-line-title = Copy Configuration Line
widgets-nothing-applied-choose-description = Nothing is applied. Choose a value and copy the line, then add it to your configuration yourself.
widgets-unit-title = Unit
widgets-line-copy-title = Line to copy
widgets-copy-line-label = Copy Line
widgets-unlock-change-setting-locked = Unlock to change this setting. While locked, you can copy its line for your configuration.
widgets-administrator-not-copy = Your administrator doesn't allow changing this setting from { -brand-name }. You can copy its line for your configuration.
widgets-bytes-label = bytes
widgets-kib-label = KiB
widgets-mib-label = MiB
widgets-lazaret-not-know-option = { -brand-name } doesn't know this option, so it only shows it.
widgets-needs-review-value-yes = Needs review: this value should be yes or no.
widgets-needs-review-value-whole = Needs review: this value should be a whole number.
widgets-needs-review-value-between = Needs review: this value should be between { $minimum } and { $maximum }.
widgets-needs-review-value-size = Needs review: this value should be a size in bytes, or a number followed by K or M.
widgets-needs-review-value-full = Needs review: this value should be a full path that starts with /.
widgets-needs-review-value-empty = Needs review: this value is empty.
widgets-apply-tooltip = Apply
widgets-unit-tooltip = Unit
widgets-setting-runs-commands-changes = This setting runs commands or changes who ClamAV runs as, so { -brand-name } never offers to change it
widgets-check-value-configuration-file = Check this value in the configuration file.
widgets-copy-configuration-line-tooltip = Copy configuration line…
widgets-applying-restarting-clamav-tooltip = Applying, and restarting ClamAV
widgets-detect-potentially-unwanted-programs = Detect Potentially Unwanted Programs
widgets-heuristic-detections = Heuristic Detections
widgets-flag-office-files-macros = Flag Office Files With Macros
widgets-flag-encrypted-archives-documents = Flag Encrypted Archives and Documents
widgets-look-inside-archives = Look Inside Archives
widgets-flag-files-too-large = Flag Files Too Large to Check Fully
widgets-adware-hacking-tools-remote = Adware, hacking tools and remote-access tools. Expect more false positives.
widgets-suspicious-files-no-signature = Suspicious files that no signature names, such as broken executables and phishing tricks.
widgets-macros-common-way-deliver = Macros are a common way to deliver malware, but many legitimate documents use them.
widgets-clamav-cannot-look-inside = ClamAV can't look inside them, so they could hide malware.
widgets-checks-files-inside-zip = Checks the files inside zip, rar and other archives.
widgets-otherwise-files-above-size = Otherwise files above the size and recursion limits pass unchecked without notice.
widgets-unlock-change-setting = Unlock to change this setting.
widgets-administrator-not-allow-detection = Your administrator doesn't allow changing this setting from { -brand-name }.
widgets-clamd-settings = clamd settings
widgets-access-scanning-settings = On-access scanning settings
widgets-update-settings = Update settings
widgets-copied-add-configuration-apply = Copied. Add it to your configuration to apply it.
widgets-change-not-made-settings = The change was not made: the settings locked again or another change started first.
widgets-applied-not-system-configuration = Applied, but not yet in your system configuration.
widgets-clamav-not-come-back = ClamAV didn't come back healthy with that value, so the previous settings were restored.
widgets-change-not-made-error = The change was not made: { $error }
widgets-nothing-changed-without-password = Nothing changed without your password
widgets-installation-no-clamav-admin = This installation has no ClamAV admin service, so ClamAV's settings can't be changed from { -brand-name }
widgets-path-combines-system-configuration = { $path } combines your system configuration with the changes made here. While locked, settings are read-only: use Copy configuration line to copy one for your configuration. After unlocking, the allowed settings change directly.
widgets-change-setting = Change this setting?
widgets-cancel-engine = Cancel
widgets-reduce-protection = Reduce Protection
widgets-apply-label = Apply
widgets-key-value-until-time = { $key } { $value }, until { $time }
widgets-live-tuning-not-installed = Live tuning is not installed. On NixOS, enable services.lazaret.liveTuning; on Debian and Ubuntu, install the { -brand-name } package, which includes the ClamAV admin service.
widgets-settings-cannot-changed-here = Settings can't be changed here right now: the { -brand-name } ClamAV admin service could not be reached.
widgets-reading-configuration = Reading the configuration…
widgets-configuration-file-not-found = Configuration file not found
widgets-lock-label = Lock
widgets-unlock-label = Unlock
widgets-change-settings-title = Change Settings
widgets-unlock-change-settings-description = Unlock to change the settings your system allows at runtime. Unlocking grants nothing by itself: each change shows what will happen and asks for administrator authorization. It locks again after two minutes.
widgets-changes-system-configuration-title = Changes your system configuration doesn't know about
widgets-copy-engine-tooltip = Copy
widgets-reduced-protection-ends-title = Reduced protection that ends on its own
widgets-reset-system-configuration-title = Reset to System Configuration
widgets-detection-title = Detection
widgets-clamav-reports-every-description = What ClamAV reports in every scan on this computer. { -brand-name } passes the same choices to clamscan, so both scanners agree.
widgets-no-settings-section-title = No settings in this section
widgets-system-journal-not-read = The system journal could not be read: { $error }
widgets-journal-reading-incomplete = Reading the system journal stopped before it finished
widgets-signature-folder-not-read = The signature folder could not be read: { $error }
widgets-reading-signature-folder-stopped = Reading the signature folder stopped before it finished
widgets-journal-entry-timestamp-outside = a journal entry has a timestamp outside the supported range
widgets-journal-reader-not-start = the journal reader could not start: { $error }
widgets-journal-reading-stopped = Reading the system journal stopped: { $error }
widgets-file-hash-mismatch-corrupted = File hash mismatch — file may be corrupted
widgets-quarantine-record-verified = This quarantine record has no usable hash, so the file cannot be verified and is not restored. You can still delete it.
widgets-quarantine-record-lazaret = This quarantine record has no usable hash, so { -brand-name } can't prove the file is the one it quarantined and doesn't send it anywhere.
widgets-file-larger-size-quarantine = The file is larger than { $size }, the most { -brand-name } sends.
widgets-access-scanning-blocks-file = On-access scanning blocks this file and it is on another filesystem, so it cannot be moved to quarantine. You can delete it instead.
widgets-quarantine-entry-not-found = Quarantine entry not found: { $id }
widgets-failed-record-restored = Failed to record quarantine entry in database. File has been restored to original location.
widgets-failed-record-rollback = Failed to record quarantine entry in database. Rollback also failed: { $error }. File may be orphaned at: { $path }
widgets-file-moved-quarantine-record = The file was moved, but its quarantine record could not be removed. It may still appear in the list.
widgets-system-call-failed-error = A system call failed: { $error }
widgets-file-operation-failed-error = A file operation failed: { $error }
widgets-quarantine-database-operation-failed = The quarantine database operation failed: { $error }
widgets-background-quarantine-step-stopped = A background quarantine step stopped unexpectedly: { $error }
widgets-path-contains-nul-byte = The path contains a NUL byte
widgets-path-no-parent-folder = The path has no parent folder
widgets-path-no-file-name = The path has no file name
widgets-quarantine-database-not-valid = The quarantine database path is not valid UTF-8
widgets-quarantine-database-not-regular = The quarantine database is not a regular file
widgets-access-scanning-blocks-reading = On-access scanning blocks reading the file
widgets-blocked-file-still-unreadable = The blocked file is still unreadable after moving it to quarantine
widgets-file-not-regular = The file is not a regular file
widgets-quarantined-file-not-regular = The quarantined file is not a regular file
widgets-not-enough-free-space = There is not enough free space to quarantine the file
widgets-file-replaced-not-removed = The file was replaced while it was being moved, so it was not removed
widgets-path-not-inside-quarantine = The path is not inside the quarantine folder
widgets-restore-path-not-valid = The restore path is not valid
widgets-restoring-protected-system-folder = Restoring into a protected system folder is not allowed
widgets-path-stored-quarantine-database = A path stored in the quarantine database could not be read
widgets-scan-target-not-exist = Scan target does not exist or contains an invalid NUL byte
widgets-scan-target-not-regular = Scan target is not a regular file or directory
widgets-unable-enumerate-scan-target = Unable to enumerate scan target { $path }: { $error }
widgets-unable-write-private-clamd = Unable to write private clamd file list: { $error }
widgets-listing-scan-target-stopped = Listing the scan target stopped unexpectedly.
widgets-no-files-scanned = No files could be scanned
widgets-count-file-not-scanned =
    { $count ->
        [one] { $count } file was not scanned
       *[other] { $count } files were not scanned
    }
widgets-connection-clamd-lost-during = The connection to clamd was lost during the scan.
widgets-scanner-not-report-result = The scanner did not report a result for these files.
widgets-count-not-accessed-title =
    { $count ->
        [one] { $count } file could not be accessed
       *[other] { $count } files could not be accessed
    }
widgets-count-file-not-read =
    { $count ->
        [one] { $count } file could not be read
       *[other] { $count } files could not be read
    }
widgets-count-non-fatal-warning =
    { $count ->
        [one] { $count } non-fatal warning during scan; some files may have been only partially scanned
       *[other] { $count } non-fatal warnings during scan; some files may have been only partially scanned
    }
widgets-count-threat-found-outcome =
    { $count ->
        [one] { $count } threat found
       *[other] { $count } threats found
    }
widgets-count-file-scanned-outcome =
    { $count ->
        [one] { $count } file scanned
       *[other] { $count } files scanned
    }
widgets-clamav-not-open-files = ClamAV could not open these files.
widgets-scanner-reported-threat-scan = The scanner reported a threat without naming the file.
widgets-preparing-label = Preparing…
widgets-finding-files-count-found =
    { $count ->
        [one] Finding files… { $count } found
       *[other] Finding files… { $count } found
    }
widgets-trust-file = Trust This File…
widgets-ignore-detection-everywhere = Ignore This Detection Everywhere…
widgets-report-false-positive = Report False Positive…
widgets-skip-path-lazaret-scans = Skip This Path in { -brand-name } Scans
widgets-copy-path = Copy Path
widgets-critical-severity = Critical
widgets-high-label = High
widgets-medium-label = Medium
widgets-low-label = Low
widgets-moved-quarantine-path-check = Moved to quarantine: { $path }. Check its reputation on the Quarantine page.
widgets-quarantine-detected-label = Quarantine
widgets-more-actions-tooltip = More Actions
widgets-count-more-file-title =
    { $count ->
        [one] { $count } more file, listed in the exported report
       *[other] { $count } more files, listed in the exported report
    }
widgets-scan-details-title = Scan details
widgets-quarantine-count-threat =
    { $count ->
        [one] Quarantine { $count } Threat
       *[other] Quarantine { $count } Threats
    }
widgets-move-quarantine = Move to Quarantine?
widgets-removes-each-original-file = This removes each original file from its current location.
widgets-cancel-scan = Cancel
widgets-quarantine-label = Quarantine
widgets-detected-threats-title = Detected Threats
widgets-problems-title = Problems
widgets-move-path-quarantine = Move '{ $path }' to quarantine?
widgets-move-count-detected-file =
    { $count ->
        [one] Move { $count } detected file to quarantine?
       *[other] Move { $count } detected files to quarantine?
    }
widgets-path-copied-clipboard = Path copied to clipboard
widgets-file-skipped-future-scans = This file will be skipped in future scans
widgets-exclusion-already-exists = This exclusion already exists
widgets-results-copied-clipboard = Results copied to clipboard
widgets-results-exported = Results exported
widgets-done-total-file-scanned =
    { $total ->
        [one] { $done } of { $total } file scanned
       *[other] { $done } of { $total } files scanned
    }
widgets-count-file-scanned-results =
    { $count ->
        [one] { $count } file scanned
       *[other] { $count } files scanned
    }
widgets-count-entry-skipped-exclusions =
    { $count ->
        [one] { $count } entry skipped by exclusions
       *[other] { $count } entries skipped by exclusions
    }
widgets-threats-detected-results = Threats Detected
widgets-no-threats-found-results = No threats found
widgets-scan-not-finish-results = The scan did not finish cleanly
widgets-scan-cancelled-user = Scan cancelled by user
widgets-count-threat-found-results =
    { $count ->
        [one] { $count } threat found
       *[other] { $count } threats found
    }
widgets-files-scanned = Files scanned
widgets-files-planned = Files planned
widgets-directories-label = Directories
widgets-threats-found = Threats found
widgets-file-no-longer-quarantine = The file is no longer there. If it is in quarantine, trust it from the Quarantine page.
widgets-file-cannot-read-possibly = The file can't be read, possibly because on-access scanning blocks it. Move it to quarantine, then trust it from the Quarantine page.
widgets-file-not-read-scan = The file could not be read: { $error }
widgets-threats-detected-run = Threats Detected
widgets-no-threats-found-run = No threats found
widgets-scan-not-finish-run = The scan did not finish cleanly
widgets-scanner-stopped-error-code = The scanner stopped with error code { $code }.
widgets-scanner-stopped-signal = The scanner was stopped by signal { $signal }.
widgets-scanner-ended-way-lazaret = The scanner ended in a way { -brand-name } does not recognize (wait status { $status }).
widgets-lazaret-lost-track-scanner = { -brand-name } lost track of the scanner process: { $error }.
widgets-clamav-scanner-not-installed = The ClamAV scanner is not installed or not on the search path.
widgets-lazaret-refused-run-program = { -brand-name } refused to run { $program }, because your account can replace { $replaceable }. Install ClamAV system-wide so a program planted by malware can't fake clean results.
widgets-scanner-not-started-error = The scanner could not be started: { $error }
widgets-unable-create-private-scanner = Unable to create the private scanner work directory: { $error }
widgets-scanner-output-not-captured = The scanner output could not be captured.
widgets-scan-not-start-clamd = The scan did not start, because the clamd detection settings could not be read: { $error }
widgets-scan-not-start-scanner = The scan did not start, because the scanner programs could not be looked up: { $error }
widgets-unable-safely-open-scan = Unable to safely open scan input { $path }: { $error }
widgets-detections-warnings-errors-tooltip = Only detections, warnings and errors
widgets-copy-service-tooltip = Copy
widgets-export-text-tooltip = Export Text
widgets-search-log-placeholder = Search log
widgets-reading-log = Reading the log…
widgets-no-log-lines-match = No log lines match
widgets-log-copied-clipboard = Log copied to clipboard
widgets-exported-label = Exported.
widgets-detection-label = Detection
widgets-emergency-label = Emergency
widgets-alert-label = Alert
widgets-critical-level = Critical
widgets-error-level = Error
widgets-warning-label = Warning
widgets-notice-label = Notice
widgets-info-label = Info
widgets-debug-label = Debug
widgets-checking-label = Checking…
widgets-system-journal = System journal
widgets-neither-log-file-nor = Neither a log file nor journal lines were found.
widgets-no-log-file-found = No log file was found. { $reason }
widgets-program-use = { $program } in use
widgets-ignored-program = Ignored { $program }
widgets-refused-program = Refused { $program }
widgets-scanner-programs-trusted = Scanner programs can be trusted
widgets-freshclam-trusted = freshclam can be trusted
widgets-malware-running-clamdscan = Malware running as you could put a fake clamdscan earlier on your search path that reports every file as clean. { -brand-name } only runs ClamAV programs that your account can't replace, such as the ones installed for the whole system.
widgets-malware-running-freshclam = Malware running as you could put a fake freshclam earlier on your search path that pretends to fetch new signatures. { -brand-name } only runs ClamAV programs that your account can't replace, such as the ones installed for the whole system.
widgets-unknown-checking-clamav-programs = Unknown, because checking the ClamAV programs stopped before it finished
widgets-clamav-program-replaced-account = A ClamAV program can be replaced by your account, so { -brand-name } refuses to run it
widgets-clamav-program-account-shadows = A ClamAV program in your account shadows the system copy. { -brand-name } ignores it, but check where it came from
widgets-count-settings-reduced-since =
    { $count ->
        [one] { $count } setting checks less than configured since { $time }
       *[other] { $count } settings check less than configured since { $time }
    }
widgets-count-settings-reduced-unknown-time =
    { $count ->
        [one] { $count } setting checks less than configured, since a time that could not be read
       *[other] { $count } settings check less than configured, since a time that could not be read
    }
widgets-changed-label = Changed
widgets-unknown-label = Unknown
widgets-reduction-unknown-unread = Unknown whether settings were reduced, because the runtime changes could not be read: { $error }
widgets-reduction-unknown-file = Unknown whether settings were reduced, because the runtime changes file is not in the expected format. ClamAV runs with the system configuration until it is fixed.
widgets-reduction-unknown-reading = Unknown whether settings were reduced, because reading the runtime changes stopped before it finished
widgets-settings-changed-runtime = Settings changed at runtime
widgets-administrator-can-change = Someone with the administrator password can change ClamAV settings at runtime. { -brand-name } could not read those changes, so it can't tell whether they check less than your configuration does.
widgets-protection-reduced-purpose = Protection reduced on purpose
widgets-administrator-changed-runtime = Someone with the administrator password changed ClamAV settings at runtime in a way that checks less than your configuration does. The system changes of the page that owns the setting show who and when.
widgets-base-configuration-changed-now = { $base } in your configuration, { $changed } now
widgets-administrator-changed-clamd = Someone with the administrator password changed ClamAV settings at runtime. Your clamd.conf could not be read, so { -brand-name } can't tell whether these changes check less than your configuration does. The system changes of the page that owns the setting show who and when.
widgets-settings-changed-runtime-clamd = Settings were changed at runtime, but clamd.conf could not be read to compare them
widgets-changed-now = { $changed } now
widgets-needs-daemon-see-scan = Needs the daemon, see Scan Engine
widgets-unknown-reason = Unknown. { $reason }
widgets-copy-timeline-tooltip = Copy
widgets-moment-details = { $moment } · { $details }
widgets-why-matters-tooltip = Why this matters
widgets-what-is-sent-label = What is sent:
widgets-cisco-talos-team-behind = Cisco Talos, the team behind ClamAV, reviews it. Samples are processed by automation, only Talos can access them, and they are never made public or shared with other companies. A review takes at least 48 hours, and its outcome shows up in a later signature database version.
widgets-report-false-positive-clamav = Report a False Positive to ClamAV?
widgets-file-content = The file's content
widgets-detection-name = The detection name { $detection }
widgets-report-missed-malware-clamav = Report Missed Malware to ClamAV?
widgets-name-email-address-talos = Your name and email address, so Talos can contact you
widgets-lazaret-sends-file-clamav = { -brand-name } sends this file to ClamAV with clamsubmit. Only send files that contain nothing private.
widgets-send-report-button = Send Report
widgets-file-content-named-sha = The file's content, named by its SHA-256 hash, not by its name
widgets-upload-malwarebazaar-heading = Upload to MalwareBazaar?
widgets-lazaret-uploads-file-malwarebazaar = { -brand-name } uploads this file to MalwareBazaar, run by abuse.ch.
widgets-tag-clamav-detection = The tag { $tag }, from ClamAV's detection
widgets-abuse-ch-key-sample = Your abuse.ch key; the sample is listed without your account name
widgets-malwarebazaar-public-anyone-download = MalwareBazaar is public: anyone can download the sample once it is uploaded, and an upload can't be taken back. Only upload malware, never a document that could hold personal data.
widgets-upload-publicly-button = Upload Publicly
widgets-upload-virustotal-heading = Upload to VirusTotal?
widgets-lazaret-uploads-file-virustotal = { -brand-name } uploads this file to VirusTotal for analysis.
widgets-virustotal-key-links-upload = Your VirusTotal key, which links the upload to your account
widgets-virustotal-shares-uploaded-files = VirusTotal shares uploaded files with its paying customers, who can download them, and anyone can look up the result by the file's hash. Never upload a document that could hold personal data.
widgets-upload-button = Upload
widgets-cancel-submission = Cancel
widgets-name-title = Your Name
widgets-email-address-title = Your Email Address
widgets-file-not-read-submission = The file could not be read: { $error }
widgets-regular-files-sent = Only regular files can be sent
widgets-file-larger-size-submission = The file is larger than { $size }, the most { -brand-name } sends.
widgets-reading-file-stopped-unexpectedly = Reading the file stopped unexpectedly
widgets-submission-history-not-read = The submission history could not be read: { $error }
widgets-file-already-sent-time = This file was already sent on { $time }. Nothing was sent again.
widgets-report-sent-clamav-review = Report sent. ClamAV's review takes at least 48 hours; Sent Samples shows when a signature update changes the verdict.
widgets-report-not-sent-error = The report was not sent: { $error }
widgets-report-sent-not-added = Report sent, but it could not be added to Sent Samples: { $error }
widgets-nothing-uploaded-error = Nothing was uploaded: { $error }
widgets-malwarebazaar-refused-sample-reason = MalwareBazaar refused the sample: { $reason }
widgets-malwarebazaar-sample-sent-samples = MalwareBazaar has the sample. Sent Samples links to its public entry.
widgets-add-abuse-ch-key = add an abuse.ch key in Preferences first
widgets-malwarebazaar-takes-files-clamav = MalwareBazaar only takes files a ClamAV signature detected
widgets-malwarebazaar-not-take-potentially = MalwareBazaar doesn't take potentially unwanted programs, only malware
widgets-uploaded-not-added-sent = Uploaded, but it could not be added to Sent Samples: { $error }
widgets-virustotal-refused-file-reason = VirusTotal refused the file: { $reason }
widgets-virustotal-analysing-file-refresh = VirusTotal is analysing the file. Refresh on the Sent Samples page shows its verdict.
widgets-virustotal-already-knows-file =
    { $total ->
        [one] VirusTotal already knows this file: { $count } of { $total } engine detects it. Nothing was uploaded.
       *[other] VirusTotal already knows this file: { $count } of { $total } engines detect it. Nothing was uploaded.
    }
widgets-add-virustotal-key-preferences = add a VirusTotal key in Preferences first
widgets-time-outcome-requested-user = { $time } · { $outcome } · requested by user { $uid }
widgets-time-outcome-requested-unknown = { $time } · { $outcome } · requested by an unknown user
widgets-change-lowered-protection-tooltip = This change lowered protection
widgets-system-journal-cannot-read = The system journal can't be read: { $error }
widgets-system-changes-title = System changes
widgets-refresh-tooltip = Refresh
widgets-system-changes-cannot-title = System changes can't be read
widgets-loading-system-changes = Loading system changes…
widgets-trust-file-heading = Trust This File?
widgets-clamav-never-report-body =
    ClamAV will never report a file with exactly this content again, wherever it is on this computer and whichever scan finds it. Your administrator password is needed, and the password prompt shows the same SHA-256 hash as below.
    
    { $path }
widgets-sha-256-hash-size =
    SHA-256 { $hash }
    { $size }
widgets-trust-button = Trust
widgets-ignore-detection-heading = Ignore This Detection?
widgets-clamav-stop-reporting-body = ClamAV will stop reporting every file with this detection, not just this one. Trusting the file by its hash is safer when only one file is a false positive.
widgets-ignore-button = Ignore
widgets-stop-trusting-file-heading = Stop Trusting This File?
widgets-clamav-report-file-body = ClamAV will report this file again when it matches a signature.
widgets-stop-trusting-button = Stop Trusting
widgets-report-detection-again-heading = Report This Detection Again?
widgets-clamav-report-files-body = ClamAV will report files with this detection again.
widgets-report-again-button = Report Again
widgets-cancel-trust = Cancel

host-applied-label = Applied
host-rolled-back = Rolled back
host-refused-without-password = Refused without a password
host-failed-label = Failed
host-outcome-not-recorded = Outcome not recorded
host-unrecognized-outcome = Unrecognized outcome { $outcome }
host-detected-files-no-longer = Detected files will no longer be blocked when opened. They are only reported.
host-clamav-stop-checking-kind = ClamAV will stop checking for this kind of file or threat on this computer.
host-files-removed-folder-no = Files in the removed folder will no longer be checked when they are opened.
host-clamav-skip-new-entry = ClamAV will skip the new entry entirely, so malware there goes unnoticed.
host-files-archives-above-new = Files or archives above the new limit will be skipped without being checked.
host-current-value-not-read = The current value could not be read, so this change may reduce protection.
host-clamav-no-longer-report = ClamAV will no longer report what you trust here.
host-key-now-current-new =
    { $key }
    Now: { $current }
    New: { $new }
host-system-records-change-reduced = The system records that this change reduced protection, and the sidebar keeps flagging the page of the changed setting until you undo it.
host-clamav-loaded-change = ClamAV loaded the change
host-saved-clamd-not-answer = Saved. clamd didn't answer, so it picks up the change when it next reloads its signatures
host-nothing-changed-without-password = Nothing changed without your password
host-installation-no-clamav-admin = This installation has no ClamAV admin service, so ClamAV's trust list can't be changed from { -brand-name }
host-nothing-changed-error = Nothing changed: { $error }
host-path-generated-nixos-configuration = { $path } is generated by your NixOS configuration, so { -brand-name } can't change it. Use Copy configuration line on a setting to copy its line for your configuration, then rebuild.
host-path-belongs-system-lazaret = { $path } belongs to the system, so { -brand-name } can't change it. Use Copy configuration line on a setting to copy the line to put in that file.
host-enter-name-valid-email = Enter your name and a valid email address to send a report
host-reporting-needs-clamsubmit-comes = Reporting needs clamsubmit, which comes with ClamAV but isn't installed here
host-lazaret-refused-run-clamsubmit = { -brand-name } refused to run clamsubmit, because your account can replace it
host-report-not-sent-error = The report was not sent: { $error }
host-report-not-sent-upload = The report was not sent: the upload took too long
host-purchase-receipt-number = purchase receipt number
host-customer-id = customer ID
host-other-source = Other source
host-unnamed-detection = an unnamed detection
host-done-total-file =
    { $total ->
        [one] { $done } of { $total } file
       *[other] { $done } of { $total } files
    }
host-count-second-left =
    { $count ->
        [one] About { $count } second left
       *[other] About { $count } seconds left
    }
host-count-minute-left =
    { $count ->
        [one] About { $count } minute left
       *[other] About { $count } minutes left
    }
host-count-hour-left =
    { $count ->
        [one] About { $count } hour left
       *[other] About { $count } hours left
    }
host-seconds-s = { $seconds } s
host-minutes-min-seconds-s = { $minutes } min { $seconds } s
host-hours-h-minutes-min = { $hours } h { $minutes } min
host-blocked-access-scanning = Blocked by on-access scanning
host-size-count-byte =
    { $count ->
        [one] { $size } ({ $count } byte)
       *[other] { $size } ({ $count } bytes)
    }
host-needs-key = Needs a key
host-unnamed-source = an unnamed source
host-not-known-files-database = Not in the known-files database
host-listed-malicious-source-name = Listed as malicious by { $source } (as { $name })
host-listed-malicious-source = Listed as malicious by { $source }
host-listed-known-file-source = Listed as a known file by { $source }
host-known-file-name-listed = Known file { $name }, listed by { $source }
host-unnamed-family = unnamed family
host-unknown-date = an unknown date
host-not-known-malware-sample = Not a known malware sample
host-known-malware-sample = Known malware sample
host-known-malware-family-first = Known malware: { $family }, first seen { $date }
host-unknown-virustotal-verdict = Unknown to VirusTotal
host-count-total-engine-verdict =
    { $total ->
        [one] { $count } of { $total } engine detects it
       *[other] { $count } of { $total } engines detect it
    }
host-virustotal-rejected-api-key = VirusTotal rejected the API key.
host-virustotal-request-limit-reached = The VirusTotal request limit was reached. Try again later.
host-virustotal-answered-status = VirusTotal answered with status { $status }
host-virustotal-not-reached-error = VirusTotal could not be reached: { $error }
host-virustotal-sent-answer-lazaret = VirusTotal sent an answer { -brand-name } cannot read
host-circl-hashlookup-not-reached = CIRCL hashlookup could not be reached: { $error }
host-circl-hashlookup-sent-answer = CIRCL hashlookup sent an answer that could not be read
host-circl-hashlookup-answered-status = CIRCL hashlookup answered with status { $status }
host-malwarebazaar-not-reached-error = MalwareBazaar could not be reached: { $error }
host-malwarebazaar-sent-not-read = MalwareBazaar sent an answer that could not be read
host-malwarebazaar-refused-api-key = MalwareBazaar refused the API key
host-malwarebazaar-answered-status = MalwareBazaar answered: { $status }
host-api-key-configured = API key configured
host-no-api-key-service = No API key, so this service is off
host-api-key-cannot-read = The API key can't be read: { $reason }
host-api-key-invalid = This doesn't look like a valid API key
host-unable-read-stored-api = Unable to read the stored API key
host-keyring-unavailable = Keyring unavailable
host-key-file-unreadable-error = { -brand-name } can't read this file: { $error }. If it is an agenix or sops-nix secret, set its owner to your user.
host-key-file-unreadable-lookups = { -brand-name } can't read this file, so lookups with this service are off. Give your user read access, for example with the owner or group setting of the agenix or sops-nix secret.
host-file-not-hold-valid = The file doesn't hold a valid key for this service

headless-count-threat-found =
    { $count ->
        [one] { $count } threat found
       *[other] { $count } threats found
    }
headless-open-lazaret-review = Open { -brand-name } to review them.
headless-count-moved-quarantine =
    { $count ->
        [one] { $count } moved to quarantine.
       *[other] { $count } moved to quarantine.
    }
headless-threats-detected = Threats Detected
headless-scheduled-scan-failed = Scheduled scan failed
headless-open-lazaret-read-log = Open { -brand-name } to read the log.
