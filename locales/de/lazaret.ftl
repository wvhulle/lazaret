-brand-name = Lazaret
startup-settings-schema-missing = Das Einstellungsschema von { -brand-name } ist nicht installiert. Führen Sie make install aus oder setzen Sie GSETTINGS_SCHEMA_DIR.
startup-unknown-page-name = { -brand-name } hat keine Seite namens { $page } und öffnet daher den ersten Bereich, der Aufmerksamkeit erfordert. Seiten: { $pages }

window-signature-sources = Signaturquellen
window-database-updates = Datenbankaktualisierungen
window-scan-engine = Scan-Engine
window-access-scanning = Zugriffsscan
window-scheduled-scans-page = Geplante Scans
window-manual-scan = Manueller Scan
window-removable-drives = Wechseldatenträger
window-detections-label = Erkennungen
window-quarantine-label = Quarantäne
window-trusted-detections = Vertrauenswürdige Erkennungen
window-sent-samples = Gesendete Proben
window-quarantine-database-not-locked = Die Quarantänedatenbank konnte nicht gesperrt werden
window-quarantine-database-not-opened = Die Quarantänedatenbank konnte nicht geöffnet werden
window-file-operation-failed-error = Ein Dateivorgang ist fehlgeschlagen: { $error }
window-lock-task-interrupted = Ein Quarantäneschritt im Hintergrund wurde unerwartet beendet: { $error }
window-translator-credits = translator-credits
window-export-history-menu-item = Verlauf exportieren …
window-clear-history-menu-item = Verlauf löschen …
window-preferences-label = Einstellungen
window-about-menu-item = Über { -brand-name }
window-quit-menu-item = Beenden
window-export-history-heading = Verlauf exportieren?
window-export-history-body = Speichert jeden aufgezeichneten Scan und jede Signaturaktualisierung mit ihren Details.
window-cancel-history-button = Abbrechen
window-json-label = JSON
window-csv-label = CSV
window-clear-history-heading = Verlauf löschen?
window-clear-history-body = Dadurch werden alle aufgezeichneten Scans und Signaturaktualisierungen dauerhaft gelöscht. Dies kann nicht rückgängig gemacht werden.
window-clear-label = Alle löschen
window-scanning-drive-title = { $drive } wird gescannt
window-open-removable-drives-button = Wechseldatenträger öffnen
window-not-export-history-toast = Der Verlauf konnte nicht exportiert werden: { $error }
window-history-export-finished = Verlauf exportiert
window-history-cleared = Verlauf gelöscht
window-not-clear-history-toast = Der Verlauf konnte nicht gelöscht werden: { $error }
window-other-sources-unknown = Andere Quellen unbekannt
window-source-out-date = Eine Quelle ist veraltet
window-official-signatures = Nur offizielle Signaturen
window-official-count-other-source =
    { $count ->
        [one] Offizielle und { $count } weitere Quelle
       *[other] Offizielle und { $count } weitere Quellen
    }
window-database-missing = Keine Virendefinitionen
window-age-definitions-unknown = Alter der Definitionen unbekannt
window-last-update-failed = Letzte Aktualisierung fehlgeschlagen
window-definitions-dated-future = Definitionen in der Zukunft datiert
window-updated-label = Aktualisiert { $relative-time }
window-definitions-label = Definitionen vom { $relative-time }
window-freshclam-refused = freshclam wird abgelehnt
window-unknown-whether-freshclam-trusted = Unbekannt, ob freshclam vertrauenswürdig ist
window-freshclam-copy-account-ignored = Eine freshclam-Kopie in Ihrem Konto wird ignoriert
window-clamd-uses-older-signatures = clamd verwendet ältere Signaturen
window-clamd-answering = clamd antwortet
window-unit-active-not-answering = clamd antwortet nicht
window-unit-state-unknown-daemon = clamd-Status unbekannt
window-unit-stopped-daemon = clamd läuft nicht
window-never-set-up-daemon = clamd nicht eingerichtet
window-engine-missing = ClamAV nicht installiert
window-scanner-refused = clamscan wird abgelehnt
window-version-unverified = ClamAV-Version unbekannt
window-scanner-program-refused = Ein Scanprogramm wird abgelehnt
window-unknown-whether-scanners-trusted = Unbekannt, ob die Scanner vertrauenswürdig sind
window-scanner-copy-account-ignored = Eine Scanner-Kopie in Ihrem Konto wird ignoriert
window-runtime-changes-unknown = Laufzeitänderungen unbekannt
window-settings-reduced-runtime = Einstellungen zur Laufzeit abgeschwächt
window-waiting-daemon = Warten auf clamd
window-unit-state-unknown-access = Status unbekannt
window-never-set-up-access = Nicht eingerichtet
window-unit-stopped-access = clamonacc läuft nicht
window-config-unreadable = clamd.conf nicht lesbar
window-nothing-watched = Nichts überwacht
window-checks-files-not-block = Prüft Dateien, blockiert nicht
window-checks-files-kernel-cannot = Prüft Dateien, Kernel kann nicht blockieren
window-checks-files-blocking-unknown = Prüft Dateien, Blockieren unbekannt
window-blocking-paths = Blockiert { $paths }
window-folder-watch-limit-unknown = Ordnerüberwachungslimit unbekannt
window-mount-watch-conflicts = Konflikte bei der Einhängepunktüberwachung
window-clamd-user-not-excluded = clamd nicht ausgeschlossen
window-whole-root-included = Überwacht / direkt
window-watch-limit-low = Ordnerüberwachungslimit niedrig
window-timer-not-active = Timer nicht aktiv
window-last-scan-failed = Letzter Scan fehlgeschlagen
window-next-scan = Nächster Scan { $relative-time }
window-no-next-scan-planned = Kein nächster Scan geplant
window-service-missing = Scandienst fehlt
window-service-unreadable = Scandienst unbekannt
window-lazaret-schedule = Geplant von { -brand-name }
window-no-schedule = Kein Scan geplant
window-schedule-changed-outside-lazaret = Zeitplan außerhalb von { -brand-name } geändert
window-ready-scan = Bereit zum Scannen
window-uses-slower-clamscan = Verwendet das langsamere clamscan
window-exclusion-skip-malware = Ein Ausschluss könnte Schadsoftware übergehen
window-device-scan-running = { $drive } wird gescannt
window-device-scan-ended = Bedrohungen auf { $drive } gefunden
window-journal-unreadable = Journal nicht lesbar
window-following-failed = Verfolgen des Journals beendet
window-notifications-off = Benachrichtigungen aus
window-alerts-stop-lazaret-quits = Warnungen enden, wenn { -brand-name } beendet wird
window-other-programs-attach-lazaret = Andere Programme können sich an { -brand-name } anhängen
window-unknown-whether-programs-attach = Unbekannt, ob sich Programme anhängen können
window-nothing-decide = Nichts zu entscheiden
window-count-needs-decision =
    { $count ->
        [one] { $count } erfordert eine Entscheidung
       *[other] { $count } erfordern eine Entscheidung
    }
window-loading-label = Wird geladen …
window-quarantine-empty = Leer
window-count-item =
    { $count ->
        [one] { $count } Eintrag
       *[other] { $count } Einträge
    }
window-quarantine-cannot-read = Die Quarantäne kann nicht gelesen werden
window-nothing-trusted = Nichts als vertrauenswürdig markiert
window-count-item-trusted =
    { $count ->
        [one] { $count } Eintrag vertrauenswürdig
       *[other] { $count } Einträge vertrauenswürdig
    }
window-signature-folder-unreadable = Signaturordner nicht lesbar
window-another-allowlist-silences-clamav = Eine andere Zulassungsliste bringt ClamAV zum Schweigen
window-lazaret-allowlist-unreadable = Die Zulassungsliste von { -brand-name } ist nicht lesbar
window-sent-samples-cannot-read = Gesendete Proben können nicht gelesen werden
window-nothing-sent = Nichts gesendet
window-count-samples-refused =
    { $count ->
        [one] { $count } vom Dienst abgelehnt
       *[other] { $count } von ihren Diensten abgelehnt
    }
window-count-samples-awaiting-review =
    { $count ->
        [one] { $count } wartet auf Prüfung
       *[other] { $count } warten auf Prüfung
    }
window-count-samples-sent =
    { $count ->
        [one] { $count } gesendet
       *[other] { $count } gesendet
    }
window-signatures-label = Signaturen
window-scans-label = Scans
window-findings-label = Funde
window-detections-come = Woher Erkennungen kommen
window-files-get-checked = Wie Dateien geprüft werden
window-found-label = Was gefunden wurde
window-needs-review = Prüfung erforderlich
window-not-working = Funktioniert nicht
window-waiting-another-part = Wartet auf eine andere Komponente
window-summary-pending = Wird geprüft …
window-menu-tooltip = Menü
window-show-lazaret = { -brand-name } anzeigen
window-show-detections = Erkennungen anzeigen
window-quick-scan = Schnellscan
window-update-definitions = Definitionen aktualisieren
window-quit-tray-item = Beenden
window-quit-completely = Vollständig beenden
window-hide-tray = In Systemleiste minimieren
window-ask-on-next-close = Beim nächsten Schließen fragen
window-close-lazaret = { -brand-name } schließen?
window-detection-alerts-stop-lazaret = Warnungen zu Erkennungen enden, während { -brand-name } nicht läuft. Im Hintergrund weiterlaufen lassen oder vollständig beenden? Das lässt sich später in den Einstellungen ändern.
window-cancel-close-button = Abbrechen
window-quit-close-button = Beenden
window-keep-running = Weiterlaufen lassen

scans-node-js-dependencies = Node.js-Abhängigkeiten
scans-git-repository-data = Git-Repository-Daten
scans-python-virtual-environment = Python-Umgebung
scans-build-output-directory = Build-Ausgabeverzeichnis
scans-distribution-output-directory = Distributionsausgabeverzeichnis
scans-python-bytecode-cache = Python-Bytecode-Cache
scans-remove-exclusion-tooltip = Entfernen
scans-preset-exclusions-title = Vordefinierte Ausschlüsse
scans-common-directories-exclude-description = Häufige Verzeichnisse vom Scan ausschließen. Änderungen werden sofort gespeichert.
scans-custom-exclusions-title = Benutzerdefinierte Ausschlüsse
scans-enter-absolute-path-description = Geben Sie einen absoluten Pfad oder ein Muster wie *.tmp ein. Änderungen werden sofort gespeichert.
scans-path-exclude-tmp-title = /Pfad/zum/Ausschluss oder *.tmp
scans-prefer-clamd-when-available = Automatisch
scans-always-clamd = clamd (Daemon)
scans-always-clamscan = clamscan (eigenständig)
scans-options-title = Optionen
scans-backend-title = Scan-Backend
scans-automatic-uses-daemon-subtitle = Automatisch verwendet den Daemon, wenn er antwortet, und andernfalls clamscan.
scans-show-live-scan-title = Live-Scan-Fortschritt anzeigen
scans-display-files-scanned-subtitle = Gerade gescannte Dateien in Echtzeit anzeigen.
scans-not-added-history-error = Der Scan konnte nicht zum Verlauf hinzugefügt werden: { $error }
scans-no-profile-manual = Kein Profil (manuell)
scans-removable-drive-scanned-title = Ein Wechseldatenträger wird gerade gescannt. Starten Sie diesen Scan, wenn er abgeschlossen ist.
scans-open-removable-drives-button = Wechseldatenträger öffnen
scans-detecting-backend = Wird geprüft …
scans-backend-undetermined = Backend: unbekannt, die Scanprogramme konnten nicht ermittelt werden: { $error }
scans-backend-clamd-daemon = Backend: clamd (Daemon)
scans-backend-clamscan-standalone = Backend: clamscan (eigenständig)
scans-scan-title = Scan
scans-preparing-label = Wird vorbereitet …
scans-cancelling-label = Wird abgebrochen …
scans-scanning-name = { $name } wird gescannt
scans-scanning-count-location =
    { $count ->
        [one] { $count } Ort wird gescannt
       *[other] { $count } Orte werden gescannt
    }
scans-cancelled-user = Scan vom Benutzer abgebrochen
scans-threats-found-scan = Gefundene Bedrohungen
scans-not-finish-cleanly = Der Scan wurde nicht sauber abgeschlossen
scans-no-threats-found = Keine Bedrohungen gefunden
scans-not-create-eicar-test = Die EICAR-Testdatei konnte nicht erstellt werden: { $error }
scans-add-files-tooltip = Dateien hinzufügen
scans-add-folder-manual-tooltip = Ordner hinzufügen
scans-more-tooltip = Mehr
scans-run-eicar-test-label = EICAR-Test ausführen
scans-copy-results-label = Ergebnisse kopieren
scans-export-results-label = Ergebnisse exportieren …
scans-start-scan-label = Scan starten
scans-cancel-manual-label = Abbrechen
scans-lazaret-refuses-clamav-title = { -brand-name } lehnt ein ClamAV-Programm ab, das Ihr Konto ersetzen könnte
scans-details-button = Details
scans-profile-title = Scan-Profil
scans-profile-short-title = Profil
scans-manage-profiles-tooltip = Profile verwalten
scans-targets-manual-title = Scan-Ziele
scans-drop-files-folders-label = Dateien oder Ordner zum Scannen hier ablegen
scans-add-files-label = Dateien hinzufügen …
scans-add-folder-label = Ordner hinzufügen …
scans-found-far-scan-title = Bisher gefunden
scans-new-scan-button = Neuer Scan
scans-recent-scans-title = Letzte Scans
scans-started-lazaret-newest-description = In { -brand-name } gestartete Scans, neueste zuerst. Wählen Sie einen aus, um sein Protokoll zu lesen.
scans-choose-files-scan = Dateien zum Scannen auswählen
scans-choose-folder-scan = Ordner zum Scannen auswählen
scans-quick-scan = Schnellscan
scans-full-scan = Vollständiger Scan
scans-home-folder = Home-Verzeichnis
scans-fast-scan-downloads-folder = Schnellscan des Downloads-Ordners zur schnellen Bedrohungserkennung
scans-comprehensive-system-wide-scan = Umfassender systemweiter Scan aller zugänglichen Verzeichnisse
scans-user-home-directory-personal = Scan des Home-Verzeichnisses und persönlicher Dateien
scans-profile-file-unavailable = Auf die Profildatei konnte nicht zugegriffen werden: { $error }
scans-profile-data-invalid = Die Profildaten sind ungültig: { $error }
scans-profile-file-not-replaced = Die Profildatei konnte nicht ersetzt werden: { $error }
scans-profile-task-interrupted = Der Profilvorgang wurde unterbrochen: { $error }
scans-profile-name-missing = Profilname ist erforderlich
scans-profile-name-too-long = Name darf höchstens 50 Zeichen enthalten
scans-profile-name-taken = Ein anderes Profil verwendet diesen Namen bereits
scans-profile-targets-missing = Fügen Sie mindestens ein Ziel hinzu
scans-edit-profile-title = Profil bearbeiten
scans-save-label = Speichern
scans-name-title = Name
scans-description-title = Beschreibung
scans-targets-profile-title = Ziele
scans-add-folder-profile-tooltip = Ordner hinzufügen
scans-add-path-title = Pfad hinzufügen
scans-exclusions-title = Ausschlüsse
scans-added-exclusions-preferences-description = Wird während der Ausführung dieses Profils zu den Ausschlüssen aus den Einstellungen hinzugefügt.
scans-add-pattern-title = Muster hinzufügen
scans-cancel-profile = Abbrechen
scans-defaults-restored = Standardprofile wiederhergestellt
scans-profile-deleted = Profil gelöscht
scans-profile-imported = Profil importiert
scans-manage-profiles-title = Profile verwalten
scans-restore-defaults-tooltip = Standardeinstellungen wiederherstellen
scans-new-profile-tooltip = Neues Profil
scans-import-tooltip = Importieren
scans-profiles-title = Scan-Profile
scans-select-profile-edit-description = Wählen Sie ein Profil zum Bearbeiten oder Scannen aus
scans-restore-default-profiles = Standardprofile wiederherstellen
scans-restore-built-default-profiles = Integrierte Standardprofile wiederherstellen? Anpassungen an Standardprofilen werden zurückgesetzt; benutzerdefinierte Profile bleiben erhalten.
scans-delete-profile = Profil löschen
scans-delete-profile-name = Das Profil „{ $name }“ löschen?
scans-delete-label = Löschen
scans-profile-exported = Profil exportiert
scans-remove-profile-tooltip = Entfernen
scans-count-target =
    { $count ->
        [one] { $count } Ziel
       *[other] { $count } Ziele
    }
scans-default-profile = Standardprofil
scans-run-tooltip = Ausführen
scans-edit-tooltip = Bearbeiten
scans-delete-tooltip = Löschen
scans-export-tooltip = Exportieren
scans-remove-target-tooltip = Entfernen
scans-check-again-access-tooltip = Erneut prüfen
scans-checking-access-tooltip = Überprüfe
scans-status-access-title = Status
scans-access-settings-changed-description = Über { -brand-name } geänderte Einstellungen des Zugriffsscans, wie vom System aufgezeichnet. Software, die unter Ihrem Konto läuft, kann diese Einträge nicht entfernen.
scans-no-access-settings-title = Über { -brand-name } wurden keine Einstellungen des Zugriffsscans geändert
scans-clamonacc-log-title = clamonacc-Protokoll
scans-clamonacc-not-written-title = clamonacc hat nichts ins Journal geschrieben
scans-checking-access-scanning = Zugriffsscan wird geprüft …
scans-command-copied-clipboard-access = Befehl in die Zwischenablage kopiert
scans-no-readable-config = Es wurde keine lesbare clamd.conf gefunden
scans-reading-task-aborted-clamd = Das Lesen von clamd.conf wurde vor dem Abschluss beendet
scans-watched-mount = { $path } (gesamter Einhängepunkt)
scans-nothing-watched = Keine konfiguriert
scans-default-limit = Der ClamAV-Standardwert
scans-configuration-label = Konfiguration
scans-watched-folders = Überwachte Ordner
scans-largest-file-scanned = Größte gescannte Datei
scans-no-kernel-config-found = Weder /proc/config.gz noch die Konfigurationsdatei des Kernels in /boot ist vorhanden.
scans-kernel-config-unreadable = Die Kernelkonfiguration konnte nicht gelesen werden.
scans-compressed-config-corrupt = /proc/config.gz konnte nicht entpackt werden.
scans-kernel-release-unreadable = Die Version des laufenden Kernels konnte nicht gelesen werden.
scans-reading-task-aborted-config = Das Lesen der Kernelkonfiguration wurde vor dem Abschluss beendet.
scans-blocks-detected-files = Erkannte Dateien werden blockiert
scans-prevention-off = Dateien werden geprüft, aber erkannte Dateien werden nicht blockiert
scans-prevention-conflicts-mounts = Dateien werden geprüft, aber erkannte Dateien werden nicht blockiert, weil OnAccessPrevention nicht mit OnAccessMountPath funktioniert
scans-kernel-cannot-block = Dateien werden geprüft, aber dieser Kernel kann erkannte Dateien nicht blockieren, weil er ohne CONFIG_FANOTIFY_ACCESS_PERMISSIONS gebaut wurde
scans-kernel-support-unknown = Dateien werden geprüft, aber es ist unbekannt, ob der Kernel erkannte Dateien blockieren kann. { $reason }
scans-config-unreadable-monitor = Läuft, aber clamd.conf konnte nicht gelesen werden, daher ist unbekannt, was geprüft wird und ob erkannte Dateien blockiert werden
scans-running-no-folders-status = Läuft, aber es werden keine Ordner oder Einhängepunkte überwacht
scans-running-detected-status = Läuft, aber es werden keine Ordner oder Einhängepunkte überwacht, und erkannte Dateien würden nicht blockiert
scans-unit-state-unknown-access = Unbekannt, ob der Zugriffsscan läuft, weil systemd es nicht melden konnte
scans-never-set-up-access = Der Zugriffsscan ist nicht eingerichtet
scans-unit-stopped-access = { $unit } läuft nicht
scans-access-blocking-title = Blockieren beim Zugriff
scans-clamonacc-checks-files-watched = clamonacc prüft Dateien in den überwachten Ordnern, wenn sie geöffnet werden. Mit aktivierter Prävention verweigert der Kernel den Zugriff auf eine erkannte Datei. Deshalb lässt sich eine solche Datei nicht öffnen, obwohl sie noch vorhanden ist.
scans-mount-watching = Einhängepunktüberwachung
scans-onaccessmountpath-watches-whole-mount = OnAccessMountPath überwacht einen ganzen Einhängepunkt über fanotify. Laut ClamAV-Dokumentation ist es nicht mit OnAccessPrevention, OnAccessIncludePath, OnAccessExcludePath und OnAccessExtraScanning kompatibel, daher werden erkannte Dateien auf einem überwachten Einhängepunkt nie blockiert. Überwachen Sie Ordner mit OnAccessIncludePath, um zu blockieren.
scans-onaccessmountpath-cannot-combined-options = OnAccessMountPath kann nicht mit { $options } kombiniert werden
scans-loop-protection = Schutz vor Scanschleifen
scans-clamd-opens-every-file = clamd öffnet jede Datei, die es scannen soll. Sofern OnAccessExcludeUname oder OnAccessExcludeUID nicht den Benutzer nennt, unter dem clamd läuft, sieht clamonacc diese Lesezugriffe und lässt clamd dieselben Dateien erneut scannen.
scans-clamd-not-excluded-clamonacc = clamd ist nicht ausgeschlossen, daher kann clamonacc seine eigenen Lesezugriffe endlos scannen
scans-watching-whole-file-system = Überwachung des gesamten Dateisystems
scans-clamonacc-refuses-onaccessincludepath-include = clamonacc lehnt OnAccessIncludePath / ab. Schließen Sie die Ordner ein, in denen Dateien ankommen, etwa Home-Verzeichnisse, oder überwachen Sie einen Einhängepunkt mit OnAccessMountPath.
scans-onaccessincludepath-rejected-clamonacc = OnAccessIncludePath / wird von clamonacc abgelehnt
scans-folder-watch-limit = Ordnerüberwachungslimit
scans-path-watched-fs-inotify = { $path } wird überwacht, aber fs.inotify.max_user_watches beträgt nur { $limit }, daher bleiben Ordner jenseits des Limits ungeprüft
scans-unknown-whether-path-fits = Unbekannt, ob { $path } in das Ordnerüberwachungslimit passt, weil fs.inotify.max_user_watches nicht gelesen werden konnte
scans-clamonacc-watches-every-folder = clamonacc überwacht jeden Ordner unterhalb eines eingeschlossenen Pfads mit inotify, eine Überwachung pro Ordner. Ein ganzes Home-Verzeichnis oder ein größerer Baum kann mehr Überwachungen benötigen, als das alte Standardlimit von 8192 erlaubt. Erhöhen Sie fs.inotify.max_user_watches mit sysctl.
scans-device-auto-scan-title = Automatischer Gerätescan
scans-automatically-scan-removable-description = Wechseldatenträger und externe Laufwerke beim Anschließen automatisch scannen.
scans-enable-device-auto-title = Automatischen Gerätescan aktivieren
scans-max-device-size-title = Maximale Gerätegröße (GB)
scans-no-size-limit-subtitle = 0 bedeutet keine Größenbeschränkung.
scans-skip-battery-device-title = Bei Akkubetrieb überspringen
scans-drive-scan-not-added = Der Laufwerksscan konnte nicht zum Verlauf hinzugefügt werden: { $error }
scans-device-auto-scan = Automatischer Gerätescan
scans-device-connected-during-another = Ein Gerät wurde während eines anderen Scans angeschlossen und daher nicht gescannt.
scans-removable-drives-title = Wechseldatenträger
scans-cancel-removable-label = Abbrechen
scans-found-far-removable-title = Bisher gefunden
scans-drives-session-title = Laufwerke in dieser Sitzung
scans-drives-connected-since-description = Seit dem Start von { -brand-name } angeschlossene Laufwerke, neueste zuerst, und was mit jedem geschehen ist. Diese Liste bleibt nur erhalten, bis { -brand-name } beendet wird.
scans-no-drive-connected-title = Es wurde noch kein Laufwerk angeschlossen
scans-close-results-button = Ergebnisse schließen
scans-drive-scans-title = Laufwerksscans
scans-drives-lazaret-scanned-description = Laufwerke, die { -brand-name } beim Anschließen gescannt hat, neueste zuerst. Wählen Sie eines aus, um sein Protokoll zu lesen.
scans-auto-scan-off = Aus
scans-every-drive = Scannt jedes Laufwerk
scans-drives-up-size = Scannt Laufwerke bis { $size }
scans-skipped-larger-than-limit = Nicht gescannt: { $size } ist größer als das Limit von { $limit }
scans-skipped-battery = Nicht gescannt: Der Computer lief im Akkubetrieb
scans-skipped-auto-scan-off = Nicht gescannt: Der automatische Gerätescan ist aus
scans-skipped-while-scanning = Nicht gescannt: Ein anderer Scan lief
scans-size-unreadable = Nicht gescannt: Die Größe des Laufwerks konnte nicht gelesen werden
scans-drive-settling = Laufwerk wird geprüft …
scans-drive-scanning = Scan läuft …
scans-threats-detected = Gescannt, Bedrohungen gefunden
scans-no-threats-detected = Gescannt, keine Bedrohungen gefunden
scans-finished-problems = Gescannt, aber der Scan wurde nicht sauber abgeschlossen
scans-scan-cancelled = Scan abgebrochen
scans-device-scan-running = { $drive } wird gescannt
scans-drive-scan = Laufwerksscan
scans-check-again-scan-tooltip = Erneut prüfen
scans-checking-scan-tooltip = Überprüfe
scans-status-scan-title = Status
scans-engine-detection-settings-description = Über { -brand-name } geänderte Engine- und Erkennungseinstellungen, wie vom System aufgezeichnet. Software, die unter Ihrem Konto läuft, kann diese Einträge nicht entfernen.
scans-no-engine-detection-title = Über { -brand-name } wurden keine Engine- oder Erkennungseinstellungen geändert
scans-clamd-log-title = clamd-Protokoll
scans-no-clamd-log-title = Es wurde kein clamd-Protokoll gefunden
scans-checking-scan-engine = Scan-Engine wird geprüft …
scans-command-copied-clipboard-scan = Befehl in die Zwischenablage kopiert
scans-program-start-failed = clamscan konnte nicht gestartet werden: { $error }
scans-probe-timed-out = clamscan hat nicht rechtzeitig geantwortet
scans-scanner-refused = { $path } abgelehnt, weil Ihr Konto es ersetzen kann
scans-engine-missing = ClamAV wurde auf diesem Computer nicht gefunden
scans-clamav-installed = ClamAV installiert
scans-clamav-engine-recognizes-malware = ClamAV ist die Engine, die Schadsoftware erkennt. { -brand-name } verwendet die auf diesem Computer installierte Kopie und bringt nie eine eigene mit.
scans-clamd-still-uses-signatures = clamd verwendet noch die Signaturen { $loaded }, aber auf der Festplatte liegt { $disk }
scans-clamd-answering = clamd antwortet
scans-unit-active-not-answering = { $unit } läuft, aber clamd antwortet nicht. Möglicherweise werden noch Signaturen geladen.
scans-unit-state-unknown-daemon = clamd antwortet nicht, und systemd konnte seinen Status nicht melden
scans-unit-stopped-daemon = { $unit } läuft nicht
scans-never-set-up-daemon = clamd ist nicht eingerichtet, daher greifen Scans auf das langsamere clamscan zurück
scans-same-version-disk = Dieselbe Version wie auf der Festplatte
scans-loaded-disk = { $loaded }, während auf der Festplatte { $disk } liegt
scans-unknown-loaded-stored-versions = Unbekannt, die geladene und die gespeicherte Version konnten nicht verglichen werden
scans-loaded-signatures = Geladene Signaturen
scans-daemon-running = Daemon läuft
scans-clamd-keeps-signatures-loaded = clamd hält die Signaturen im Speicher geladen. Das Blockieren beim Zugriff und geplante Scans senden ihre Dateien beide an clamd.
scans-socket-protected = Nur das System kann { $path } ersetzen
scans-socket-replaceable = Ihr Konto kann { $replaceable } ersetzen, daher könnte ein unter Ihrem Konto laufendes Programm unter { $path } ein gefälschtes clamd ablegen, das jede Datei als sauber meldet
scans-socket-missing = Kein clamd-Socket unter { $path }, daher läuft clamd nicht oder lauscht an anderer Stelle
scans-socket-not-checkable = Die Flatpak-Sandbox kann nicht sehen, wem der clamd-Socket gehört
scans-clamd-socket-protected = clamd-Socket geschützt
scans-socket-mechanism = Scans erreichen clamd über seinen Socket. Lassen Sie clamd ihn in einem Verzeichnis anlegen, in das nur root oder der Benutzer clamav schreiben kann, etwa /run/clamav, damit kein unter Ihrem Konto laufendes Programm ein gefälschtes clamd an seine Stelle setzen kann.
scans-socket-inspecting = Es wird geprüft, wer den clamd-Socket ersetzen kann …
scans-timer-inactive = Nicht geplant
scans-next-run-time = Nächste Ausführung: { $time }
scans-scheduled-time-next-run = Geplant. Der Zeitpunkt der nächsten Ausführung ist noch nicht bekannt.
scans-systemd-unavailable = Geplante Scans benötigen eine systemd-Benutzersitzung, die nicht verfügbar ist.
scans-system-already-runs-scheduled = Ihr System führt bereits geplante Scans aus, daher sind diese Einstellungen gesperrt, um einen zweiten Zeitplan zu vermeiden. Nächster Scan: { $time }. Ändern Sie ihn in Ihrer Systemkonfiguration.
scans-system-scan-timer-settings = Ihr System hat einen Scan-Timer, daher sind diese Einstellungen gesperrt, um einen zweiten Zeitplan zu vermeiden. Ändern Sie ihn in Ihrer Systemkonfiguration.
scans-reading-user-timer = Wird geprüft …
scans-inspecting-system-timer = Es wird geprüft, ob Ihr System bereits geplante Scans ausführt. Bis dahin bleiben diese Einstellungen gesperrt.
scans-system-timer-unknown = { -brand-name } konnte nicht prüfen, ob Ihr System bereits geplante Scans ausführt, daher sind diese Einstellungen gesperrt, um einen zweiten Zeitplan zu vermeiden: { $error }
scans-scan-time-unreadable = Geben Sie die Scan-Zeit als HH:MM ein, zum Beispiel 02:00.
scans-unit-files-unwritable = Fehler beim Erstellen der systemd-Dateien: { $error }
scans-timer-not-applied-error = Der Timer konnte nicht angewendet werden: { $error }
scans-hourly-label = Stündlich
scans-daily-label = Täglich
scans-weekly-label = Wöchentlich
scans-monthly-label = Monatlich
scans-monday-label = Montag
scans-tuesday-label = Dienstag
scans-wednesday-label = Mittwoch
scans-thursday-label = Donnerstag
scans-friday-label = Freitag
scans-saturday-label = Samstag
scans-sunday-label = Sonntag
scans-schedule-title = Zeitplan
scans-apply-label = Anwenden
scans-enable-scheduled-scans-title = Geplante Scans aktivieren
scans-frequency-title = Scan-Häufigkeit
scans-time-24-hour-title = Scan-Zeit (24-Stunden-Format)
scans-day-week-title = Wochentag
scans-used-weekly-scans-subtitle = Wird für wöchentliche Scans verwendet.
scans-day-month-title = Tag des Monats
scans-used-monthly-scans-subtitle = Wird für monatliche Scans verwendet.
scans-targets-separated-commas-title = Scan-Ziele (durch Kommas getrennt; leer scannt Ihr Home-Verzeichnis)
scans-skip-battery-schedule-title = Bei Akkubetrieb überspringen
scans-not-start-scheduled-subtitle = Geplante Scans bei Akkubetrieb nicht starten.
scans-auto-quarantine-title = Automatische Quarantäne
scans-automatically-quarantine-threats-subtitle = Von geplanten Scans gefundene Bedrohungen automatisch in Quarantäne verschieben. Solange { -brand-name } geöffnet ist, werden Bedrohungen nur gemeldet.
scans-review-button = Prüfen
scans-check-again-scheduled-tooltip = Erneut prüfen
scans-checking-scheduled-tooltip = Überprüfe
scans-system-scan-title = Systemscan
scans-folders-scanned-title = Gescannte Ordner
scans-run-now-label = Jetzt ausführen
scans-starting-scan-tooltip = Scan wird gestartet …
scans-status-scheduled-title = Status
scans-scheduled-runs-title = Geplante Ausführungen
scans-lazaret-schedule-ran-description = Scans, die nach dem Zeitplan von { -brand-name } ausgeführt wurden, neueste zuerst. Wählen Sie einen aus, um sein Protokoll zu lesen.
scans-checking-schedule = Zeitplan wird geprüft …
scans-command-copied-clipboard-scheduled = Befehl in die Zwischenablage kopiert
scans-not-scheduled = Nicht geplant
scans-next-scan = Nächster Scan
scans-threats-found-scheduled = Bedrohungen gefunden
scans-nothing-found = nichts gefunden
scans-no-completed-scan = Noch nicht ausgeführt
scans-last-scan = Letzter Scan
scans-unknown-label = Unbekannt
scans-system-timer = Der System-Timer ist nicht aktiv
scans-last-scan-ended-error = Der letzte Scan endete mit einem Fehler
scans-next-scan-time = Nächster Scan: { $time }
scans-timer-starts-scan-service = Der Timer startet einen Scandienst, der nicht installiert ist
scans-service-not-read-systemd = Der Scandienst konnte nicht aus systemd gelesen werden
scans-lazaret-schedule = Geplant von { -brand-name }
scans-no-schedule = Es ist kein Scan geplant
scans-not-scheduled-timer-active = Nicht geplant, weil der Timer nicht aktiv ist
scans-folders-scanned = Gescannte Ordner
scans-scheduled-scan = Geplanter Scan
scans-scheduled-scan-catches-files = Ein geplanter Scan findet Dateien, die nicht geöffnet wurden oder die erst nach einer Signaturaktualisierung erkennbar wurden.
scans-unit-names-no-folders = Die Scan-Unit nennt keine Ordner
scans-system-scan-service-unit = Der Scandienst { $unit } Ihres Systems scannt diese Ordner mit den eigenen Einstellungen des Systems. Zum Ausführen ist eine Administratorberechtigung erforderlich.
scans-needs-daemon-see-scan = Benötigt den Daemon, siehe Scan-Engine
scans-bus-call-failed = systemd hat nicht geantwortet: { $error }
scans-unrecognized-load-state = systemd hat einen Unit-Status gemeldet, den { -brand-name } nicht erkennt
scans-found-malware = Der Scan hat Schadsoftware gefunden
scans-unit-finished = Der Scan ist abgeschlossen
scans-not-finish-cleanly-result = Der Scan wurde nicht sauber abgeschlossen: { $result }
scans-authorization-refused = Das Starten des Scans erfordert eine Administratorberechtigung
scans-not-started-error = Der Scan konnte nicht gestartet werden: { $error }

findings-open-preferences = Einstellungen öffnen
findings-journal-unreadable-alerts = Das Systemjournal kann nicht gelesen werden. Fügen Sie Ihren Benutzer zur Gruppe systemd-journal hinzu.
findings-notifications-turned-off-preferences = Benachrichtigungen sind in den Einstellungen ausgeschaltet
findings-alerts-stop-quit-lazaret = Warnungen enden, wenn Sie { -brand-name } beenden, weil es nicht für die Ausführung im Hintergrund eingerichtet ist
findings-running-background-following-journal = Läuft im Hintergrund und verfolgt das Journal
findings-alerts-label = Warnungen
findings-lazaret-follows-system-journal = { -brand-name } verfolgt das Systemjournal und benachrichtigt Sie, wenn etwas erkannt wird. Das funktioniert nur, solange es läuft, daher sollte es im Hintergrund laufen.
findings-notifications-on = An
findings-notifications-off = Aus
findings-running-label = Läuft
findings-not-running = Läuft nicht
findings-unit-not-found = Nicht eingerichtet
findings-systemd-unavailable = Unbekannt, systemd ist nicht erreichbar
findings-following-journal-access = Wird verfolgt
findings-journal-unreadable-access = Nicht lesbar
findings-following-failed = Verfolgen beendet
findings-notifications-label = Benachrichtigungen
findings-background-service = Hintergrunddienst
findings-system-journal = Systemjournal
findings-attach-unknown-setting = Ob sich andere Programme anhängen können, ist unbekannt, weil die Kernel-Einstellung nicht gelesen werden kann: { $reason }
findings-attach-unknown-reports = Ob sich andere Programme anhängen können, ist unbekannt, weil der Kernel einen Wert meldet, den { -brand-name } nicht erkennt
findings-attach-unknown-check = Ob sich andere Programme anhängen können, ist unbekannt, weil die Prüfung vor dem Abschluss beendet wurde
findings-attach-restricted = Programme können sich nicht an andere von Ihnen ausgeführte Programme wie { -brand-name } anhängen
findings-any-program-may-attach = Jedes von Ihnen ausgeführte Programm kann sich an { -brand-name } anhängen und ändern, was es anzeigt, einschließlich der Erkennungen und der Entscheidungen, die Sie hier treffen
findings-yama-not-loaded = Dieser Kernel schränkt das Anhängen nicht ein, daher kann sich jedes von Ihnen ausgeführte Programm an { -brand-name } anhängen und ändern, was es anzeigt
findings-other-programs-cannot-attach = Andere Programme können sich nicht an { -brand-name } anhängen
findings-program-attaches-lazaret-hide = Ein Programm, das sich an { -brand-name } anhängt, kann Warnungen verbergen oder Ihre Entscheidungen ändern. Wenn Sie kernel.yama.ptrace_scope auf 1 setzen, kann sich nur der Elternprozess an ein Programm anhängen.
findings-file-owned-other-user = Diese Datei gehört einem anderen Benutzer, daher kann sie hier nicht verschoben oder gelöscht werden
findings-file-not-checkable = Diese Datei konnte nicht geprüft werden, daher kann sie hier nicht verschoben oder gelöscht werden
findings-removal-failed = Die Datei konnte nicht gelöscht werden: { $error }
findings-file-check-interrupted = Die Prüfung der Datei wurde unerwartet beendet
findings-history-not-updated = Ihre Entscheidung konnte nicht gespeichert werden: { $error }
findings-journal-unreadable-list = { -brand-name } kann das Systemjournal nicht lesen, daher erscheinen neue Erkennungen im Hintergrund hier nicht. Das bedeutet nicht, dass nichts gefunden wurde. Fügen Sie Ihren Benutzer zur Gruppe systemd-journal hinzu.
findings-loading-detections = Erkennungen werden geladen …
findings-nothing-detected-background = Im Hintergrund wurde nichts erkannt.
findings-access-scanning-scheduled-scans = Was der Zugriffsscan und geplante Scans gefunden haben.
findings-detections-title = Erkennungen
findings-back-detections-tooltip = Zurück zu den Erkennungen
findings-back-quarantine-tooltip = Zurück zur Quarantäne
findings-alerts-title = Warnungen
findings-detections-reach-keeps-description = Wie Erkennungen Sie erreichen und was andere Programme daran hindert, sie zu verbergen.
findings-check-again-background-tooltip = Erneut prüfen
findings-checking-background-tooltip = Überprüfe
findings-detection-title = Erkennung
findings-copied-clipboard-toast = In die Zwischenablage kopiert
findings-command-copied-clipboard-toast = Befehl in die Zwischenablage kopiert
findings-detections-lazaret-stopped = Erkennungen, während { -brand-name } nicht lief, können nicht gezählt werden, weil nicht gelesen werden konnte, wo das Lesen des Systemjournals beendet wurde: { $error }
findings-detections-not-read-error = Erkennungen konnten nicht gelesen werden: { $error }
findings-malware-blocked-signature = Schadsoftware blockiert: { $signature }
findings-malware-found-signature = Schadsoftware gefunden: { $signature }
findings-count-detection-not-running =
    { $count ->
        [one] { $count } Erkennung, während { -brand-name } nicht lief
       *[other] { $count } Erkennungen, während { -brand-name } nicht lief
    }
findings-detections-not-running-detection = Erkennungen, während { -brand-name } nicht lief, können nicht gezählt werden
findings-lazaret-not-read-stopped = { -brand-name } konnte nicht lesen, wo es das Verfolgen des Systemjournals beendet hat, daher achtet es nur auf neue Erkennungen. Prüfen Sie die Seite Erkennungen und das Systemjournal auf Funde, während es geschlossen war.
findings-quarantine-incident = Quarantäne
findings-ask-incident-deletion = Löschen …
findings-delete-incident-file = Endgültig löschen
findings-cancel-incident-deletion = Abbrechen
findings-dismiss-incident = Behalten
findings-look-up-incident-signature = Nachschlagen
findings-submit-incident-sample = Einsenden …
findings-restored-quarantine = Aus der Quarantäne wiederhergestellt
findings-moved-quarantine = In Quarantäne verschoben
findings-deleted-label = Gelöscht
findings-kept-label = Von Ihnen behalten
findings-belongs-another-user = Gehört einem anderen Benutzer
findings-blocked-needs-decision = Blockiert, Entscheidung erforderlich
findings-not-blocked-needs-decision = Nicht blockiert, Entscheidung erforderlich
findings-needs-decision = Entscheidung erforderlich
findings-no-longer-there-detection = Nicht mehr vorhanden
findings-state-unknown-detection = Datei konnte nicht geprüft werden
findings-signature-test-file-state = { $signature } (Testdatei) · { $state }
findings-signature-state = { $signature } · { $state }
findings-access-schedule = Zugriffsscan und geplanter Scan
findings-access-only = Zugriffsscan
findings-schedule-only = Der geplante Scan
findings-file-label = Datei
findings-signature-source = Signaturquelle
findings-found-label = Gefunden von
findings-last-seen = Zuletzt gesehen
findings-times-seen = Anzahl Funde
findings-detected-label = Erkannt
findings-signature-matches-harmless-eicar = Diese Signatur passt auf das harmlose EICAR-Testmuster, das dazu dient, die Erkennung zu überprüfen.
findings-clamav-matched-file-against = ClamAV hat diese Datei einer Signatur zugeordnet. Ein Treffer ist ein starkes Indiz, aber Fehlalarme kommen vor, insbesondere bei gepackten Programmen oder Administrationswerkzeugen.
findings-signature-first-seen-time = { $signature }, zuerst gesehen { $local-time }
findings-opening-file-refused-why = Das Öffnen dieser Datei wird verweigert, deshalb lässt sie sich nicht öffnen
findings-not-opened-again-after = Nach Ihrer Entscheidung nicht mehr geöffnet
findings-no-longer-there-blocked = Die Datei ist nicht mehr vorhanden
findings-not-blocked-scheduled-scan = Nicht blockiert: Der geplante Scan hat sie gefunden, und sie kann weiterhin geöffnet werden
findings-not-blocked-right-now = Derzeit nicht blockiert: Sie kann weiterhin geöffnet werden
findings-state-unknown-blocked = Es konnte nicht geprüft werden, ob die Datei blockiert ist
findings-blocked-label = Blockiert
findings-access-prevention-kernel-refuses = Mit Prävention beim Zugriff verweigert der Kernel das Öffnen einer erkannten Datei in einem überwachten Ordner. Die Datei bleibt, wo sie ist, bis Sie entscheiden, was damit geschehen soll.
findings-looking-up = Wird nachgeschlagen …
findings-no-known-samples-match = Keine bekannten Proben passen zu dieser Signatur.
findings-unknown-label = unbekannt
findings-count-known-samples-families = { $count } bekannte Proben. Familien: { $families }. Zuerst gesehen: { $date }.
findings-ask-malwarebazaar-signature-description = MalwareBazaar zu dieser Signatur befragen. Nur ihr Name wird gesendet.
findings-set-up-lookup-button = Einrichten …
findings-bazaar-no-key = Kein Schlüssel
findings-bazaar-key-unreadable = Schlüssel nicht lesbar
findings-stop-asking-file-description = Nicht mehr zu dieser Datei fragen
findings-delete-file-permanently-description = Diese Datei endgültig löschen? Dies kann nicht rückgängig gemacht werden.
findings-go-back-without-description = Zurück, ohne zu löschen
findings-move-somewhere-safe-description = An einen sicheren Ort verschieben. Sie können sie später wiederherstellen.
findings-remove-good-description = Endgültig entfernen
findings-know-file-fine-description = Sie wissen, dass diese Datei in Ordnung ist. ClamAV meldet sie beim nächsten Scan trotzdem; vertrauen Sie ihr in der Quarantäne oder in einem Scanergebnis, um das zu verhindern.
findings-decision-title = Ihre Entscheidung
findings-unfamiliar-quarantine-keeping-file = Wenn sie Ihnen unbekannt ist, verschieben Sie sie in Quarantäne. Das Behalten einer Datei sorgt nur dafür, dass { -brand-name } nicht mehr fragt: Der Zugriffsscan blockiert sie weiterhin, bis der Pfad in der ClamAV-Konfiguration ausgeschlossen wird, wie hier gezeigt.
findings-quarantined-files-restored-quarantine = Dateien in Quarantäne können auf der Seite Quarantäne wiederhergestellt werden. Eine behaltene Datei wird vom Zugriffsscan weiterhin blockiert, bis ihr Pfad in der ClamAV-Konfiguration ausgeschlossen wird.
findings-file-other-owner-decision = Diese Datei gehört einem anderen Benutzer, daher kann { -brand-name } sie weder verschieben noch löschen
findings-state-unknown-review = { -brand-name } konnte diese Datei nicht prüfen, daher kann es sie weder verschieben noch löschen
findings-choose-happens-file = Wählen Sie, was mit dieser Datei geschehen soll
findings-malwarebazaar-label = MalwareBazaar
findings-resolved-title = Erledigt
findings-resolved-detection-opens-again = Eine erledigte Erkennung wird erneut geöffnet, wenn ClamAV dieselbe Datei später wieder findet, zum Beispiel nachdem Sie sie ein zweites Mal heruntergeladen haben.
findings-waiting-decision = Wartet auf Ihre Entscheidung
findings-resolved-state-step = { $state }, { $time }
findings-send-quarantined-copy-description = Die Kopie in Quarantäne an ClamAV, MalwareBazaar oder VirusTotal senden. Ein an ClamAV gemeldeter Fehlalarm wird durch eine spätere Signaturaktualisierung für alle behoben.
findings-cancel-quarantine = Abbrechen
findings-delete-entries = Löschen
findings-scanner-program-not-installed = Das Scannerprogramm ist nicht installiert.
findings-scanner-program-replaced-account = Das Scannerprogramm unter { $program } kann von Ihrem Konto über { $path } ersetzt werden.
findings-content-pipe-failed = Die Datei konnte nicht an den Scanner übergeben werden: { $error }
findings-scanner-spawn-failed = Der Scanner konnte nicht gestartet werden: { $error }
findings-verdict-timed-out = Der Scan hat zu lange gedauert.
findings-detection-settings-unreadable = Die Erkennungseinstellungen von clamd konnten nicht gelesen werden: { $error }
findings-program-lookup-failed = Die Scannerprogramme konnten nicht ermittelt werden: { $error }
findings-payload-unreadable = Die Kopie in Quarantäne konnte nicht gelesen werden: { $error }
findings-restore-file-heading = Diese Datei wiederherstellen?
findings-clamav-no-longer-verdict = ClamAV erkennt diese Datei mit den heutigen Signaturen nicht mehr.
findings-restore-button = Wiederherstellen
findings-restore-file-clamav-heading = Eine Datei wiederherstellen, die ClamAV noch erkennt?
findings-clamav-still-detects-verdict = ClamAV erkennt diese Datei mit den heutigen Signaturen weiterhin als { $detection }. Stellen Sie sie nur wieder her, wenn Sie sicher sind, dass es sich um einen Fehlalarm handelt, und erwägen Sie, ihr vorher zu vertrauen.
findings-clamav-still-detects-unnamed-verdict = ClamAV erkennt diese Datei mit den heutigen Signaturen weiterhin, ohne die Erkennung zu benennen. Stellen Sie sie nur wieder her, wenn Sie sicher sind, dass es sich um einen Fehlalarm handelt, und erwägen Sie, ihr vorher zu vertrauen.
findings-restore-anyway-button = Trotzdem wiederherstellen
findings-restore-unchecked-file-heading = Eine ungeprüfte Datei wiederherstellen?
findings-file-not-checked-verdict = Die Datei konnte vor dem Wiederherstellen nicht erneut geprüft werden. { $reason }
findings-location-outside-home-folder = Dieser Ort liegt außerhalb Ihres Home-Verzeichnisses.
findings-file-goes-back = Die Datei kommt zurück nach:
findings-entry-count =
    { $count ->
        [one] { $count } Eintrag
       *[other] { $count } Einträge
    }
findings-clear-old-items = Alte Einträge löschen?
findings-count-quarantined-file-older =
    { $count ->
        [one] { $count } Datei in Quarantäne, die älter als 30 Tage ist, wird endgültig gelöscht.
       *[other] { $count } Dateien in Quarantäne, die älter als 30 Tage sind, werden endgültig gelöscht.
    }
findings-delete-quarantined-file = Datei in Quarantäne löschen?
findings-file-permanently-deleted = Die Datei wird endgültig gelöscht.
findings-quarantine-title = Quarantäne
findings-more-tooltip = Mehr
findings-clear-old-items-label = Alte Einträge löschen …
findings-check-again-label = Erneut prüfen
findings-scan-every-quarantined-tooltip = Jede Datei in Quarantäne mit den aktuellen Signaturen scannen
findings-refresh-tooltip = Aktualisieren
findings-search-tooltip = Suchen
findings-search-threat-name-placeholder = Nach Bedrohungsname oder Pfad suchen
findings-quarantine-storage-title = Quarantänespeicher
findings-quarantine-storage-description = Erkannte Bedrohungen, von Ihrem System isoliert.
findings-look-up-hash-button = Prüfsumme nachschlagen
findings-reputation-section-title = Ruf
findings-reputation-section-description = Prüfsumme nachschlagen sendet nur die SHA-256-Prüfsumme der Datei, nie die Datei.
findings-submit-sample-row-title = Die Datei selbst senden
findings-report-false-positive-row-title = Cisco Talos informieren
findings-report-false-positive-row-subtitle = Ein späteres Signatur-Update behebt die Erkennung dann für alle.
findings-trust-file-row-title = Auf diesem Computer vertrauen
findings-trust-file-row-subtitle = ClamAV meldet genau diese Datei hier nicht mehr und führt sie unter Vertrauenswürdige Erkennungen. Stellen Sie sie danach wieder her.
findings-total-size-title = Gesamtgröße
findings-dismiss-label = Verwerfen
findings-quarantine-cannot-read-title = Quarantäne kann nicht gelesen werden
findings-loading-quarantine = Quarantäne wird geladen …
findings-quarantine-empty = Quarantäne ist leer
findings-hash-copied-clipboard = Prüfsumme in die Zwischenablage kopiert
findings-every-quarantined-file-still = Jede Datei in Quarantäne wird noch erkannt oder konnte nicht geprüft werden
findings-count-quarantined-file-no =
    { $count ->
        [one] { $count } Datei in Quarantäne wird nicht mehr erkannt. Prüfen Sie sie, bevor Sie sie wiederherstellen.
       *[other] { $count } Dateien in Quarantäne werden nicht mehr erkannt. Prüfen Sie sie, bevor Sie sie wiederherstellen.
    }
findings-count-report-clamav-settled =
    { $count ->
        [one] { $count } Meldung an ClamAV ist durch die neuen Signaturen erledigt
       *[other] { $count } Meldungen an ClamAV sind durch die neuen Signaturen erledigt
    }
findings-checking-label = Wird geprüft …
findings-no-longer-detected = Nicht mehr erkannt
findings-still-detected = Noch erkannt
findings-integrity-unknown-hash = Unbekannt. Dieser Quarantäne-Eintrag hat keine verwendbare Prüfsumme, daher kann die Datei nicht verifiziert, online geprüft, als vertrauenswürdig markiert oder wiederhergestellt werden. Sie können sie trotzdem löschen.
findings-not-checked = Noch nicht geprüft
findings-integrity-unknown-label = Integrität unbekannt
findings-restore-label = Wiederherstellen
findings-delete-entry-label = Löschen
findings-set-up-source-button = Einrichten …
findings-reputation-no-key = Kein Schlüssel
findings-reputation-key-unreadable = Schlüssel nicht lesbar
findings-view-report-label = Bericht anzeigen
findings-false-positive-title = Fehlalarm
findings-report-label = Melden …
findings-submit-quarantine-label = Einsenden …
findings-send-clamav-malwarebazaar-tooltip = An ClamAV, MalwareBazaar oder VirusTotal senden
findings-trust-label = Vertrauen …
findings-copy-hash-tooltip = Prüfsumme kopieren
findings-submit-quarantined-label = Einsenden …
findings-service-label = Dienst
findings-status-label = Status
findings-quarantine-record-no-usable = Dieser Quarantäne-Eintrag hat keine verwendbare Prüfsumme, daher sendet { -brand-name } ihn nirgendwohin
findings-file-no-longer-quarantine = Diese Datei befindet sich nicht mehr in Quarantäne
findings-service-not-take-detection = Dieser Dienst nimmt diese Erkennung nicht an
findings-file-no-usable-hash = Diese Datei befindet sich nicht mehr mit einer verwendbaren Prüfsumme in Quarantäne
findings-check-malwarebazaar-virustotal-tooltip = MalwareBazaar und VirusTotal auf Neuigkeiten prüfen
findings-checking-sent-tooltip = Überprüfe
findings-send-file-title = Eine Datei senden
findings-nothing-leaves-computer-description = Nichts verlässt diesen Computer, bis Sie bestätigen, was gesendet wird.
findings-report-missed-malware-label = Übersehene Schadsoftware melden …
findings-send-clamav-malicious-tooltip = ClamAV eine schädliche Datei senden, die es nicht erkennt
findings-sent-title = Gesendet
findings-lazaret-sent-became-description = Was { -brand-name } gesendet hat und was daraus geworden ist.
findings-filter-sent-samples-tooltip = Gesendete Proben filtern
findings-online-services-title = Onlinedienste
findings-each-service-receives-subtitle = Was jeder Dienst erhält, und die Schlüssel für MalwareBazaar und VirusTotal
findings-choose-file-clamav-misses = Datei auswählen, die ClamAV übersieht
findings-false-positive = Fehlalarm
findings-missed-malware = Übersehene Schadsoftware
findings-malware-sample = Schadsoftware-Probe
findings-clamav-false-positive = Fehlalarm an ClamAV melden
findings-clamav-missed-malware = Übersehene Schadsoftware an ClamAV melden
findings-malwarebazaar-sample-choice = Auf MalwareBazaar hochladen
findings-virustotal-sample-choice = Auf VirusTotal hochladen
findings-awaiting-clamav = Wartet auf Prüfung durch ClamAV. Prüfungen dauern mindestens 48 Stunden; eine spätere Signaturaktualisierung zeigt das Ergebnis.
findings-fixed-database-version = Behoben in Datenbankversion { $version }
findings-fixed-current-signature-database = Behoben in der aktuellen Signaturdatenbank
findings-detected-since-database-version = Erkannt seit Datenbankversion { $version }
findings-detected-current-signature-database = Von der aktuellen Signaturdatenbank erkannt
findings-uploaded-waiting-malwarebazaar-list = Hochgeladen; wartet auf Aufnahme bei MalwareBazaar
findings-listed-malwarebazaar = Bei MalwareBazaar gelistet
findings-malwarebazaar-refused-reason = MalwareBazaar hat sie abgelehnt: { $reason }
findings-queued-analysis-virustotal = Zur Analyse bei VirusTotal eingereiht
findings-count-total-engine-flags =
    { $total ->
        [one]
            { $count ->
                [one] { NUMBER($count) } von { $total } Engine stuft sie als schädlich ein
               *[other] { NUMBER($count) } von { $total } Engine stufen sie als schädlich ein
            }
       *[other]
            { $count ->
                [one] { NUMBER($count) } von { $total } Engines stuft sie als schädlich ein
               *[other] { NUMBER($count) } von { $total } Engines stufen sie als schädlich ein
            }
    }
findings-virustotal-refused-reason = VirusTotal hat sie abgelehnt: { $reason }
findings-service-kind-hash-date = { $service } · { $kind } · { $hash } · { $date }
findings-integrity-unknown-candidate = { $detection } · Keine verwendbare Prüfsumme, daher kann { -brand-name } nicht nachweisen, was es senden würde
findings-all-services = Alle Dienste
findings-any-status-option = Beliebiger Status
findings-waiting-label = Wartend
findings-settled-label = Erledigt
findings-refused-label = Abgelehnt
findings-all-submissions-option = Alle Einsendungen
findings-listing-loading = Einsendungen werden geladen …
findings-nothing-submitted = Nichts eingesendet
findings-no-submission-matches-filters = Keine Einsendung entspricht den Filtern
findings-candidates-loading = Quarantäne wird geladen …
findings-quarantine-empty-sent = Quarantäne ist leer
findings-no-upload-changed = Kein Upload hat sich geändert
findings-count-upload-new-status =
    { $count ->
        [one] { $count } Upload hat einen neuen Status
       *[other] { $count } Uploads haben einen neuen Status
    }
findings-choose-service-heading = Wohin soll diese Datei gehen?
findings-set-up-keys-response = Schlüssel einrichten …
findings-each-service-gets-file = Jeder Dienst erhält die Datei selbst. Der nächste Schritt zeigt genau, was gesendet wird und wer es sehen kann.
findings-cancel-submission = Abbrechen
findings-open-remote-entry-tooltip = Externen Eintrag öffnen
findings-same-upload-running = Diese Datei wird bereits an diesen Dienst gesendet. Warten Sie, bis das abgeschlossen ist, bevor Sie sie erneut senden.
findings-check-again-trusted-tooltip = Erneut prüfen
findings-checking-trusted-tooltip = Überprüfe
findings-status-title = Status
findings-files-trusted-detections-description = Über { -brand-name } als vertrauenswürdig markierte Dateien sowie ignorierte oder wiederhergestellte Erkennungen, wie vom System aufgezeichnet. Software, die unter Ihrem Konto läuft, kann diese Einträge nicht entfernen.
findings-no-trusted-detections-title = Über { -brand-name } wurden keine vertrauenswürdigen Erkennungen geändert
findings-checking-trusted-detections = Vertrauenswürdige Erkennungen werden geprüft …
findings-command-copied-clipboard = Befehl in die Zwischenablage kopiert
findings-other-allowlist = Andere Zulassungsliste
findings-file-reason = { $file }: { $reason }
findings-trusted-files = Vertrauenswürdige Dateien
findings-ignored-detections = Ignorierte Erkennungen
findings-allowlist-not-write-status = Eine Zulassungsliste, die nicht von { -brand-name } stammt, lässt ClamAV schweigen
findings-count-file-detection-status =
    { $count ->
        [one] { $count } Datei oder Erkennung wird nie gemeldet
       *[other] { $count } Dateien oder Erkennungen werden nie gemeldet
    }
findings-own-allowlist-unreadable-status = { -brand-name } kann seine eigene Zulassungsliste nicht lesen und daher nicht feststellen, worüber ClamAV schweigt
findings-folder-unreadable = { -brand-name } kann den Signaturordner nicht lesen und daher nicht feststellen, worüber ClamAV schweigt
findings-signature-folder = Signaturordner
findings-trusted-detections = Vertrauenswürdige Erkennungen
findings-trust-list-mechanism = Dateien und Erkennungsnamen auf den Zulassungslisten von ClamAV werden von keinem Scan gemeldet. Schadsoftware mit Administratorrechten könnte sich so verstecken, prüfen Sie daher, ob jeder Eintrag absichtlich hinzugefügt wurde.
findings-clamav-reports-everything-detects = ClamAV meldet alles, was es erkennt
findings-remove-tooltip = Entfernen
findings-access-denied = Zugriff verweigert
findings-other-read-failure = Lesen fehlgeschlagen: { $reason }
findings-reading-task-aborted = Das Lesen wurde vor dem Abschluss beendet
findings-journal-unreadable-added = Unbekannt, wer dies hinzugefügt hat. { $reason }
findings-known-user = Hinzugefügt { $local-time } von Benutzer { $uid }
findings-unknown-user = Hinzugefügt { $time } von einem unbekannten Benutzer
findings-no-record-added-system = Kein Eintrag im Systemjournal darüber, wer dies hinzugefügt hat
findings-count-entry-not-added =
    { $count ->
        [one] { $count } Eintrag, nicht über { -brand-name } hinzugefügt
       *[other] { $count } Einträge, nicht über { -brand-name } hinzugefügt
    }
findings-entries-cannot-counted-reason = Einträge können nicht gezählt werden ({ $reason }), nicht über { -brand-name } hinzugefügt
findings-list-unreadable = { -brand-name } kann diese Liste nicht lesen
findings-no-trusted-files = Keine vertrauenswürdigen Dateien
findings-no-ignored-detections = Keine ignorierten Erkennungen
findings-lazaret-cannot-read-signature = { -brand-name } kann den Signaturordner nicht lesen
findings-trusted-detections-title = Vertrauenswürdige Erkennungen
findings-trusted-by-you-title = Von Ihnen vertraut
findings-trusted-by-you-description = Ihre Scans zeigen Ihnen diese Dateien nicht an, und weder Ihre Scans noch Hintergrundwarnungen zeigen diese Erkennungen an. ClamAV selbst erkennt sie weiterhin, daher kann der Zugriffsscan die Dateien weiterhin blockieren, und andere Benutzer sehen sie weiterhin. Jede Änderung an einer vertrauenswürdigen Datei lässt sie wieder erscheinen.
findings-nothing-trusted-by-you = Sie haben noch nichts als vertrauenswürdig markiert
findings-trust-by-you-unreadable = { -brand-name } kann nicht lesen, was Sie als vertrauenswürdig markiert haben: { $error }
findings-file-trusted-by-you = Eine von Ihnen vertraute Datei
findings-trust-for-all-users-tooltip = Für alle Benutzer vertrauen …
findings-trusted-for-all-users-title = Für alle Benutzer vertraut
findings-clamav-never-reports-description = ClamAV meldet nie, was hier aufgeführt ist, für keinen Benutzer und in keinem Scan auf diesem Computer. Jede Änderung erfordert ein Administratorpasswort und wird im Systemjournal aufgezeichnet, wo Software, die unter Ihrem Konto läuft, den Eintrag nicht entfernen kann.
findings-ignored-detections-title = Ignorierte Erkennungen
findings-detection-names-clamav-description = Erkennungsnamen, die ClamAV in jeder Datei und für jeden Benutzer ignoriert. { -brand-name } fügt diese nicht mehr hinzu, weil sie jede Datei mit dieser Erkennung verbergen; entfernen Sie sie und vertrauen Sie Dateien stattdessen anhand ihrer Prüfsumme.
findings-other-allowlists-title = Andere Zulassungslisten
findings-allowlist-files-signature-description = Zulassungslisten im Signaturordner, die nicht von { -brand-name } stammen. Prüfen Sie, ob Sie oder Ihr Administrator sie absichtlich hinzugefügt haben.

signatures-history-load-failed = Verlauf konnte nicht geladen werden: { $error }
signatures-database-update-finished = Datenbankaktualisierung abgeschlossen
signatures-check-again-database-tooltip = Erneut prüfen
signatures-checking-database-tooltip = Überprüfe
signatures-update-history-title = Aktualisierungsverlauf
signatures-updates-started-lazaret-description = In { -brand-name } gestartete Aktualisierungen, neueste zuerst. Wählen Sie eine aus, um ihr Protokoll zu lesen.
signatures-historical-logs-cannot-title = Frühere Protokolle können nicht gelesen werden
signatures-status-title = Status
signatures-loading-history = Verlauf wird geladen …
signatures-no-updates-started-lazaret = Aus { -brand-name } wurden noch keine Aktualisierungen gestartet
signatures-checking-signatures = Signaturen werden geprüft …
signatures-command-copied-clipboard-database = Befehl in die Zwischenablage kopiert
signatures-update-not-added-history = Die Aktualisierung konnte nicht zum Verlauf hinzugefügt werden: { $error }
signatures-update-signatures = Jetzt aktualisieren
signatures-cancel-signature-update = Abbrechen
signatures-definitions-built-date = Definitionen erstellt { $relative-time } ({ $date })
signatures-definitions-exist-not-determined = Definitionen sind vorhanden, aber ihr Alter konnte nicht ermittelt werden
signatures-no-virus-definitions-found = Es wurden keine Virendefinitionen gefunden
signatures-age-definitions-unknown-checking = Das Alter der Definitionen ist unbekannt, weil ihre Prüfung vor dem Abschluss beendet wurde
signatures-last-update-failed = Die letzte Aktualisierung ist fehlgeschlagen
signatures-no-completed-update = Noch keine abgeschlossene Aktualisierung aufgezeichnet
signatures-updater-updated = Aktualisiert von { $updater }
signatures-provider-fetched = Abgerufen von { $provider }
signatures-count-signature-size-file =
    { $count ->
        [one] { $count } Signatur, { $size } ({ $file })
       *[other] { $count } Signaturen, { $size } ({ $file })
    }
signatures-file-unreadable = Unbekannte Anzahl von Signaturen, die Datei konnte nicht gelesen werden: { $error }, { $size } ({ $file })
signatures-other-signature-sources = Andere Signaturquellen
signatures-unknown-reason-database = Unbekannt. { $reason }
signatures-fetch-newest-signatures = Die neuesten Signaturen jetzt von jedem Aktualisierer abrufen
signatures-manual-update = Manuelle Aktualisierung
signatures-update-service = Aktualisierungsdienst
signatures-database-folder = Datenbankordner
signatures-updater-output = Ausgabe des Aktualisierers
signatures-current-label = Signaturen aktuell
signatures-signature-mechanism = Signaturen beschreiben bekannte Schadsoftware, und ständig werden neue veröffentlicht. Alte Signaturen übersehen neue Bedrohungen, selbst wenn alles andere läuft.
signatures-updating-signatures = Signaturen werden aktualisiert …
signatures-update-cancelling = Aktualisierung wird abgebrochen …
signatures-starting-update = Aktualisierung wird gestartet …
signatures-daemon-running = Aktiv
signatures-timer-scheduled = Durch einen systemd-Timer geplant
signatures-service-stopped = Installiert, aber angehalten
signatures-service-not-found = Kein freshclam-Dienst erkannt; Aktualisierungen werden manuell ausgeführt
signatures-systemd-unreachable-service = systemd konnte nicht nach einem freshclam-Dienst gefragt werden; Aktualisierungen werden manuell ausgeführt
signatures-count-day-old =
    { $count ->
        [one] { $count } Tag alt
       *[other] { $count } Tage alt
    }
signatures-definitions-missing = Keine lesbare Virendatenbank gefunden
signatures-directory-unreadable = Unbekannt. { $reason }
signatures-scanner-error = freshclam ist mit Exit-Code { $code } fehlgeschlagen.
signatures-killed-signal = freshclam wurde durch Signal { $signal } beendet.
signatures-unrecognized-wait-status = freshclam wurde auf eine Weise beendet, die { -brand-name } nicht erkennt.
signatures-wait-failed = { -brand-name } hat freshclam vor dem Abschluss aus den Augen verloren.
signatures-authorization-refused = Legitimierung fehlgeschlagen. Bitte versuchen Sie es erneut und geben Sie Ihr Passwort ein.
signatures-systemd-not-reached = systemd konnte nicht erreicht werden, um die Aktualisierung auszuführen.
signatures-service-job-ended = Der Aktualisierungsdienst wurde nicht abgeschlossen; systemd meldete: { $result }
signatures-update-rate-limited-mirror = Aktualisierung durch den Spiegelserver begrenzt. Bitte warten Sie einige Minuten und versuchen Sie es erneut.
signatures-update-blocked-cdn = Aktualisierung durch das CDN blockiert. Dies kann an einer Ratenbegrenzung liegen. Bitte warten Sie und versuchen Sie es später erneut.
signatures-mirror-unavailable = Der ClamAV-Spiegelserver ist derzeit nicht verfügbar. Bitte versuchen Sie es später erneut.
signatures-certificate-rejected = SSL/TLS-Zertifikatfehler. Der Spiegelserver hat möglicherweise Konfigurationsprobleme.
signatures-connection-timed-out = Zeitüberschreitung der Verbindung. Bitte prüfen Sie Ihre Netzwerkverbindung.
signatures-database-locked = Die Datenbank ist gesperrt. Möglicherweise läuft eine andere freshclam-Instanz.
signatures-permission-denied = Zugriff verweigert. Möglicherweise benötigen Sie erweiterte Rechte, um die Datenbank zu aktualisieren.
signatures-connection-failed = Verbindungsfehler. Bitte prüfen Sie Ihre Netzwerkverbindung.
signatures-name-resolution-failed = DNS-Auflösung fehlgeschlagen. Bitte prüfen Sie Ihre Netzwerkeinstellungen.
signatures-update-failed-unknown-error = Aktualisierung mit einem unbekannten Fehler fehlgeschlagen. Details finden Sie in den Protokollen.
signatures-updated-databases = Aktualisiert: { $databases }
signatures-already-current-databases = Bereits aktuell: { $databases }
signatures-cooldown-unknown = Ratenbegrenzt: { $database }
signatures-cooldown-until = Ratenbegrenzt: { $database } bis { $cooldown }
signatures-definitions-updated = Datenbankaktualisierung abgeschlossen
signatures-already-current = Datenbank ist bereits auf dem neuesten Stand
signatures-service-ran-without-verdict = Der Aktualisierungsdienst wurde ausgeführt, aber sein Protokoll sagt nicht, ob sich Definitionen geändert haben.
signatures-update-requested-daemon = Der Dienst hat das Signal angenommen und führt die Aktualisierung asynchron aus. Prüfen Sie seine Protokolle auf den Abschluss.
signatures-update-cancelled = Datenbankaktualisierung abgebrochen
signatures-scanner-program-missing = Installieren Sie freshclam, bevor Sie Datenbankaktualisierungen anfordern.
signatures-provider-needs-credential-fangfrisch =
    { $credential ->
        [purchase-receipt] { $provider } benötigt eine Kaufbelegnummer, und fangfrisch liest sie nur aus seinen Einstellungen. Unter NixOS wird services.clamav.fangfrisch.settings in den Nix-Store kopiert, den jeder Benutzer dieses Computers lesen kann, sodass die Kaufbelegnummer offengelegt würde. Lassen Sie { $provider } deaktiviert oder führen Sie fangfrisch mit einer Einstellungsdatei außerhalb des Nix-Stores aus, etwa einem agenix- oder sops-nix-Geheimnis.
       *[customer-identifier] { $provider } benötigt eine Kundennummer, und fangfrisch liest sie nur aus seinen Einstellungen. Unter NixOS wird services.clamav.fangfrisch.settings in den Nix-Store kopiert, den jeder Benutzer dieses Computers lesen kann, sodass die Kundennummer offengelegt würde. Lassen Sie { $provider } deaktiviert oder führen Sie fangfrisch mit einer Einstellungsdatei außerhalb des Nix-Stores aus, etwa einem agenix- oder sops-nix-Geheimnis.
    }
signatures-provider-needs-credential-website =
    { $credential ->
        [purchase-receipt] { $provider } benötigt eine Kaufbelegnummer von seiner Website. fangfrisch liest sie aus seiner Einstellungsdatei, meist { $path }. Fügen Sie dort diese Zeilen hinzu, ersetzen Sie den Platzhalter durch Ihre Kaufbelegnummer und machen Sie die Datei nur für root lesbar, damit andere Benutzer sie nicht sehen können.
       *[customer-identifier] { $provider } benötigt eine Kundennummer von seiner Website. fangfrisch liest sie aus seiner Einstellungsdatei, meist { $path }. Fügen Sie dort diese Zeilen hinzu, ersetzen Sie den Platzhalter durch Ihre Kundennummer und machen Sie die Datei nur für root lesbar, damit andere Benutzer sie nicht sehen können.
    }
signatures-not-use = Nicht in Verwendung
signatures-add-lines-fangfrisch-settings = Fügen Sie diese Zeilen zur Einstellungsdatei von fangfrisch hinzu, normalerweise { $path }.
signatures-provider-needs-free-abuse = { $provider } benötigt einen kostenlosen abuse.ch-Auth-Key. Fügen Sie diese Zeilen zur Einstellungsdatei von fangfrisch hinzu, normalerweise { $path }, ersetzen Sie den Platzhalter durch Ihren Schlüssel und machen Sie die Datei nur für root lesbar, damit andere Benutzer sie nicht sehen können.
signatures-use-label = In Verwendung
signatures-unknown-reason-source = Unbekannt. { $reason }
signatures-check-again-signature-tooltip = Erneut prüfen
signatures-checking-signature-tooltip = Überprüfe
signatures-clamav-malware-signatures-description = Woher die Schadsoftware-Signaturen von ClamAV stammen und welche anderen Quellen Sie hinzufügen könnten.
signatures-sources-title = Signaturquellen
signatures-abuse-ch-key-title = abuse.ch-Schlüssel
signatures-abuse-ch-auth-title = abuse.ch-Auth-Key
signatures-abuse-ch-key-uses = Für URLhaus-Signaturen sowie für Abfragen und Uploads bei MalwareBazaar.
signatures-abuse-ch-no-key = Kein Schlüssel
signatures-abuse-ch-key-unreadable = Schlüssel nicht lesbar
signatures-command-copied-clipboard-signature = Befehl in die Zwischenablage kopiert
signatures-not-open-documentation-title = Dokumentation konnte nicht geöffnet werden: { $error }
signatures-checking-api-key = API-Schlüssel wird geprüft …
signatures-check-passed = Bestanden
signatures-needs-review = Warnung
signatures-check-failed = Nicht bestanden
signatures-status-unknown = Unbekannt
signatures-check-skipped = Übersprungen
signatures-check-progress = Überprüfe
signatures-copy-tooltip = Kopieren
signatures-learn-more-title = Mehr erfahren
signatures-open-browser-tooltip = Im Browser öffnen
signatures-checking-title = Wird geprüft …

preferences-launcher-not-found = { -brand-name } konnte seinen eigenen Starter nicht finden: { $error }
preferences-action-unwritable = Die Dateimanager-Aktion konnte nicht hinzugefügt werden: { $error }
preferences-action-unremovable = Die Dateimanager-Aktion konnte nicht entfernt werden: { $error }
preferences-dialog-title = Einstellungen
preferences-general-title = Allgemein
preferences-window-behavior-title = Fensterverhalten
preferences-detection-alerts-work-description = Warnungen zu Erkennungen funktionieren nur, während { -brand-name } läuft.
preferences-closing-window-title = Beim Schließen des Fensters
preferences-start-background-title = Im Hintergrund starten
preferences-launch-background-without-subtitle = Im Hintergrund starten, ohne ein Fenster zu öffnen.
preferences-desktop-notifications-title = Desktop-Benachrichtigungen
preferences-notify-detections-finished-subtitle = Über Erkennungen benachrichtigen sowie über abgeschlossene Scans und Aktualisierungen, während das Fenster nicht im Fokus ist.
preferences-file-manager-integration-title = Dateimanager-Integration
preferences-add-context-menu-description = Kontextmenüaktionen für unterstützte Dateimanager hinzufügen.
preferences-online-services-title = Onlinedienste
preferences-online-services-summary = Nachschlagen sendet nur eine Prüfsumme oder einen Signaturnamen. Eine Datei verlässt diesen Computer nur, wenn Sie sie einsenden.
preferences-circl-receives-summary = Erhält die SHA-256-Prüfsumme einer Datei und sagt, ob es eine bekannte, harmlose Datei ist. Kein Schlüssel nötig.
preferences-talos-receives-summary = Erhält die Dateien, die Sie melden, mit Ihrem Namen und Ihrer E-Mail-Adresse. Talos veröffentlicht sie nie. Kein Schlüssel nötig.
preferences-virustotal-receives-summary = Erhält die SHA-256-Prüfsumme einer Datei. Hochgeladene Dateien werden mit zahlenden Kunden von VirusTotal geteilt.
preferences-malwarebazaar-receives-summary = Erhält einen Signaturnamen oder die SHA-256-Prüfsumme einer Datei. Hochgeladene Dateien werden öffentlich.
preferences-key-file-not-permitted-hint = Machen Sie sie für Ihren Benutzer lesbar, zum Beispiel über den Besitzer des agenix- oder sops-nix-Geheimnisses.
preferences-key-file-missing-hint = Prüfen Sie den Pfad der Schlüsseldatei in Ihrer Systemkonfiguration.
preferences-key-file-unreadable-hint = Prüfen Sie, ob der Pfad auf eine normale Datei zeigt.
preferences-key-not-valid-hint = Kopieren Sie den ganzen Schlüssel von der Website des Dienstes.
preferences-keyring-unavailable-hint = Entsperren Sie Ihren Schlüsselbund und öffnen Sie dann die Einstellungen erneut.
preferences-stored-key-unreadable-hint = Löschen Sie den Schlüssel und fügen Sie ihn erneut ein.
preferences-virustotal-service-hint = Kostenlose Stufe: 4 Anfragen pro Minute, 500 pro Tag.
preferences-abuse-ch-service-hint = Kostenlos im abuse.ch Authentication Portal.
preferences-not-configured = Nicht konfiguriert
preferences-checking-label = Wird geprüft …
preferences-set-system-configuration-title = Durch Ihre Systemkonfiguration festgelegt
preferences-key-keyring-title = Schlüssel in Ihrem Schlüsselbund
preferences-key-file-title = Schlüsseldatei
preferences-key-unreadable-status = Nicht lesbar
preferences-key-file-not-permitted = Schlüsseldatei nicht lesbar: keine Berechtigung
preferences-key-file-missing = Die Schlüsseldatei existiert nicht
preferences-key-file-unreadable = Schlüsseldatei nicht lesbar
preferences-key-not-valid = Kein gültiger Schlüssel für diesen Dienst
preferences-keyring-unavailable = Der Schlüsselbund ist nicht verfügbar
preferences-stored-key-unreadable = Der Schlüssel in Ihrem Schlüsselbund ist nicht lesbar
preferences-delete-key-label = Schlüssel löschen
preferences-paste-api-key-title = API-Schlüssel einfügen
preferences-get-free-api-title = Kostenlosen API-Schlüssel erhalten

widgets-clean-label = Sauber
widgets-scan-infected = Infiziert
widgets-error-status = Fehler
widgets-cancelled-label = Abgebrochen
widgets-skipped-label = Übersprungen
widgets-success-label = Erfolgreich
widgets-up-date = Aktuell
widgets-requested-label = Angefordert
widgets-ran-label = Ausgeführt
widgets-counted-threats =
    { $count ->
        [one] { $count } Bedrohung gefunden
       *[other] { $count } Bedrohungen gefunden
    }
widgets-uncounted-threats = Der Scanner hat eine Bedrohung gemeldet, ohne die Datei zu nennen.
widgets-no-threats-found-activity = Keine Bedrohungen gefunden
widgets-scan-failed = Der Scan wurde nicht sauber abgeschlossen
widgets-scan-cancelled-activity = Scan vom Benutzer abgebrochen
widgets-scheduled-scan-skipped-battery = Geplanter Scan übersprungen (Akkubetrieb)
widgets-virus-definitions-updated = Virendefinitionen aktualisiert
widgets-virus-definitions-up-date = Virendefinitionen sind auf dem neuesten Stand
widgets-update-requested-service = Aktualisierung beim Dienst angefordert
widgets-update-service-no-result = Der Aktualisierungsdienst wurde ohne Ergebnismeldung ausgeführt
widgets-update-failed = Datenbankaktualisierung fehlgeschlagen
widgets-update-cancelled = Datenbankaktualisierung abgebrochen
widgets-scan-log = SCAN-LOG
widgets-update-log = AKTUALISIERUNGS-LOG
widgets-timestamp-label = Zeitstempel
widgets-status-label = Status
widgets-duration-label = Dauer
widgets-summary-label = Zusammenfassung
widgets-path-label = Pfad
widgets-full-output = Vollständige Ausgabe
widgets-scan-history-cannot-title = Scanverlauf kann nicht gelesen werden
widgets-retry-label = Erneut versuchen
widgets-copy-activity-tooltip = Kopieren
widgets-log-details-copied-toast = Protokolldetails in die Zwischenablage kopiert
widgets-not-save-file-error = Die Datei konnte nicht gespeichert werden: { $error }
widgets-not-choose-file-error = Die Datei konnte nicht ausgewählt werden: { $error }
widgets-chosen-file-not-local = { $location } ist keine Datei auf diesem Computer
widgets-export-title = Exportieren
widgets-database-newer-release = Diese Datenbank wurde von einer neueren Version von { -brand-name } erstellt: Sie hat die Layoutversion { $found }, und diese Version kennt nur bis { $known }. Aktualisieren Sie { -brand-name }, um sie zu öffnen.
widgets-migration-step-failed = Die Aktualisierung des Datenbanklayouts ist bei Schritt { $step } fehlgeschlagen: { $error }
widgets-version-check-failed = Die Layoutversion der Datenbank konnte nicht geprüft werden: { $error }
widgets-copy-configuration-line-title = Konfigurationszeile kopieren
widgets-nothing-applied-choose-description = Es wird nichts angewendet. Wählen Sie einen Wert, kopieren Sie die Zeile und fügen Sie sie dann selbst zu Ihrer Konfiguration hinzu.
widgets-unit-title = Einheit
widgets-line-copy-title = Zu kopierende Zeile
widgets-copy-line-label = Zeile kopieren
widgets-unlock-required = Entsperren Sie, um diese Einstellung zu ändern. Im gesperrten Zustand können Sie ihre Zeile für Ihre Konfiguration kopieren.
widgets-administrator-not-copy = Ihr Administrator erlaubt nicht, diese Einstellung in { -brand-name } zu ändern. Sie können ihre Zeile für Ihre Konfiguration kopieren.
widgets-plain-bytes = Bytes
widgets-kibibytes-unit = KiB
widgets-mebibytes-unit = MiB
widgets-unknown-option = { -brand-name } kennt diese Option nicht und zeigt sie daher nur an.
widgets-not-yes-no = Prüfung erforderlich: Dieser Wert sollte yes oder no sein.
widgets-not-whole-number = Prüfung erforderlich: Dieser Wert sollte eine ganze Zahl sein.
widgets-count-out-range = Prüfung erforderlich: Dieser Wert sollte zwischen { $minimum } und { $maximum } liegen.
widgets-not-size = Prüfung erforderlich: Dieser Wert sollte eine Größe in Bytes sein oder eine Zahl, gefolgt von K oder M.
widgets-path-not-absolute = Prüfung erforderlich: Dieser Wert sollte ein vollständiger Pfad sein, der mit / beginnt.
widgets-empty-text = Prüfung erforderlich: Dieser Wert ist leer.
widgets-apply-tooltip = Anwenden
widgets-unit-tooltip = Einheit
widgets-setting-runs-commands-changes = Diese Einstellung führt Befehle aus oder ändert, unter welchem Benutzer ClamAV läuft, daher bietet { -brand-name } nie an, sie zu ändern
widgets-check-value-configuration-file = Prüfen Sie diesen Wert in der Konfigurationsdatei.
widgets-copy-configuration-line-tooltip = Konfigurationszeile kopieren …
widgets-applying-restarting-clamav-tooltip = Wird angewendet, ClamAV wird neu gestartet
widgets-detect-potentially-unwanted-programs = Potenziell unerwünschte Programme erkennen
widgets-heuristic-detections = Heuristische Erkennung
widgets-flag-office-files-macros = Office-Dateien mit Makros markieren
widgets-flag-encrypted-archives-documents = Verschlüsselte Archive und Dokumente markieren
widgets-look-inside-archives = In Archive hineinsehen
widgets-flag-files-too-large = Dateien markieren, die zu groß für eine vollständige Prüfung sind
widgets-adware-hacking-tools-remote = Adware, Hacking-Werkzeuge und Fernzugriffswerkzeuge. Rechnen Sie mit mehr Fehlalarmen.
widgets-suspicious-files-no-signature = Verdächtige Dateien, die keine Signatur benennt, etwa beschädigte ausführbare Dateien und Phishing-Tricks.
widgets-macros-common-way-deliver = Makros sind ein häufiger Weg, Schadsoftware zu verbreiten, aber viele legitime Dokumente verwenden sie.
widgets-clamav-cannot-look-inside = ClamAV kann nicht in sie hineinsehen, daher könnten sie Schadsoftware verbergen.
widgets-checks-files-inside-zip = Prüft die Dateien in zip-, rar- und anderen Archiven.
widgets-otherwise-files-above-size = Andernfalls bleiben Dateien oberhalb der Größen- und Rekursionsgrenzen ohne Hinweis ungeprüft.
widgets-locked-until-unlocked = Entsperren Sie, um diese Einstellung zu ändern.
widgets-administrator-not-allow-detection = Ihr Administrator erlaubt nicht, diese Einstellung in { -brand-name } zu ändern.
widgets-clamd-settings = clamd-Einstellungen
widgets-access-scanning-settings = Einstellungen des Zugriffsscans
widgets-update-settings = Aktualisierungseinstellungen
widgets-change-snippet-copied = Kopiert. Fügen Sie es zu Ihrer Konfiguration hinzu, um es anzuwenden.
widgets-stale-confirmation-dropped = Die Änderung wurde nicht vorgenommen: Die Einstellungen wurden erneut gesperrt oder eine andere Änderung hat zuerst begonnen.
widgets-settings-applied = Angewendet, aber noch nicht in Ihrer Systemkonfiguration.
widgets-previous-restored = ClamAV lief mit diesem Wert nicht fehlerfrei wieder an, daher wurden die vorherigen Einstellungen wiederhergestellt.
widgets-change-not-made-error = Die Änderung wurde nicht vorgenommen: { $error }
widgets-authorization-refused = Ohne Ihr Passwort wurde nichts geändert
widgets-admin-service-missing = Diese Installation hat keinen ClamAV-Verwaltungsdienst, daher können die Einstellungen von ClamAV nicht in { -brand-name } geändert werden
widgets-live-tunable-config = { $path } kombiniert Ihre Systemkonfiguration mit den hier vorgenommenen Änderungen. Im gesperrten Zustand sind die Einstellungen schreibgeschützt: Verwenden Sie Konfigurationszeile kopieren, um eine für Ihre Konfiguration zu kopieren. Nach dem Entsperren werden die erlaubten Einstellungen direkt geändert.
widgets-change-setting = Diese Einstellung ändern?
widgets-cancel-engine = Abbrechen
widgets-reduce-protection = Schutz verringern
widgets-apply-label = Anwenden
widgets-key-value-until-time = { $key } { $value }, bis { $time }
widgets-live-tuning-not-installed = Die Live-Anpassung ist nicht installiert. Aktivieren Sie unter NixOS services.lazaret.liveTuning; installieren Sie unter Debian und Ubuntu das Paket { -brand-name }, das den ClamAV-Verwaltungsdienst enthält.
widgets-admin-service-unreachable = Einstellungen können hier derzeit nicht geändert werden: Der ClamAV-Verwaltungsdienst von { -brand-name } konnte nicht erreicht werden.
widgets-loading-config = Konfiguration wird gelesen …
widgets-config-missing = Konfigurationsdatei nicht gefunden
widgets-lock-label = Sperren
widgets-unlock-label = Entsperren
widgets-change-settings-title = Einstellungen ändern
widgets-unlock-change-settings-description = Entsperren Sie, um die Einstellungen zu ändern, die Ihr System zur Laufzeit erlaubt. Das Entsperren allein gewährt nichts: Jede Änderung zeigt, was geschehen wird, und fragt nach einer Administratorberechtigung. Nach zwei Minuten wird wieder gesperrt.
widgets-changes-system-configuration-title = Änderungen, die Ihre Systemkonfiguration nicht kennt
widgets-copy-engine-tooltip = Kopieren
widgets-reduced-protection-ends-title = Verringerter Schutz, der von selbst endet
widgets-reset-system-configuration-title = Auf Systemkonfiguration zurücksetzen
widgets-detection-title = Erkennung
widgets-clamav-reports-every-description = Was ClamAV in jedem Scan auf diesem Computer meldet. { -brand-name } übergibt dieselben Optionen an clamscan, sodass beide Scanner übereinstimmen.
widgets-no-settings-section-title = Keine Einstellungen in diesem Abschnitt
widgets-journal-read-failed = Das Systemjournal konnte nicht gelesen werden: { $error }
widgets-journal-reading-incomplete = Das Lesen des Systemjournals wurde vor dem Abschluss beendet
widgets-folder-unreadable = Der Signaturordner konnte nicht gelesen werden: { $error }
widgets-reading-signature-folder-stopped = Das Lesen des Signaturordners wurde vor dem Abschluss beendet
widgets-entry-timestamp-invalid = Ein Journaleintrag hat einen Zeitstempel außerhalb des unterstützten Bereichs.
widgets-reader-thread-failed = Der Journal-Leser konnte nicht gestartet werden: { $error }
widgets-journal-reading-stopped = Das Lesen des Systemjournals wurde beendet. { $reason }
widgets-file-hash-mismatch = Datei-Prüfsumme stimmt nicht überein – die Datei ist möglicherweise beschädigt
widgets-restore-refused-integrity-unknown = Dieser Quarantäne-Eintrag hat keine verwendbare Prüfsumme, daher kann die Datei nicht verifiziert werden und wird nicht wiederhergestellt. Sie können sie trotzdem löschen.
widgets-upload-refused-integrity-unknown = Dieser Quarantäne-Eintrag hat keine verwendbare Prüfsumme, daher kann { -brand-name } nicht nachweisen, dass es die Datei ist, die es in Quarantäne verschoben hat, und sendet sie nirgendwohin.
widgets-payload-too-large-send = Die Datei ist größer als { $size }, das Maximum, das { -brand-name } sendet.
widgets-blocked-file-other-filesystem = Der Zugriffsscan blockiert diese Datei, und sie liegt auf einem anderen Dateisystem, daher kann sie nicht in Quarantäne verschoben werden. Sie können sie stattdessen löschen.
widgets-entry-not-found = Quarantäne-Eintrag nicht gefunden: { $id }
widgets-recording-failed-file-restored = Quarantäne-Eintrag konnte nicht in der Datenbank gespeichert werden. Datei wurde an ihren ursprünglichen Ort zurückgesetzt.
widgets-recording-failed-rollback = Quarantäne-Eintrag konnte nicht in der Datenbank gespeichert werden. Rollback ist ebenfalls fehlgeschlagen: { $error }. Datei könnte verwaist sein unter: { $path }
widgets-entry-record-not-removed = Die Datei wurde verschoben, aber ihr Quarantäne-Eintrag konnte nicht entfernt werden. Sie erscheint möglicherweise noch in der Liste.
widgets-system-call-failed = Ein Systemaufruf ist fehlgeschlagen: { $error }
widgets-file-operation-failed = Ein Dateivorgang ist fehlgeschlagen: { $error }
widgets-database-operation-failed = Der Vorgang in der Quarantänedatenbank ist fehlgeschlagen: { $error }
widgets-blocking-step-interrupted = Ein Quarantäneschritt im Hintergrund wurde unerwartet beendet: { $error }
widgets-path-contains-nul = Der Pfad enthält ein NUL-Byte
widgets-path-has-no-parent = Der Pfad hat keinen übergeordneten Ordner
widgets-path-has-no-file = Der Pfad hat keinen Dateinamen
widgets-database-path-not-utf8 = Der Pfad der Quarantänedatenbank ist kein gültiges UTF-8
widgets-database-not-regular-file = Die Quarantänedatenbank ist keine reguläre Datei
widgets-source-blocked-access = Der Zugriffsscan blockiert das Lesen der Datei
widgets-moved-payload-still-blocked = Die blockierte Datei ist nach dem Verschieben in die Quarantäne immer noch nicht lesbar
widgets-source-not-regular-file = Die Datei ist keine reguläre Datei
widgets-payload-not-regular-file = Die Datei in Quarantäne ist keine reguläre Datei
widgets-not-enough-free-space = Es ist nicht genügend freier Speicherplatz vorhanden, um die Datei in Quarantäne zu verschieben
widgets-file-replaced-during-operation = Die Datei wurde während des Verschiebens ersetzt und daher nicht entfernt
widgets-path-outside-quarantine = Der Pfad liegt nicht im Quarantäneordner
widgets-invalid-restore-path = Der Wiederherstellungspfad ist ungültig
widgets-protected-restore-destination = Das Wiederherstellen in einen geschützten Systemordner ist nicht erlaubt
widgets-stored-path-unreadable = Ein in der Quarantänedatenbank gespeicherter Pfad konnte nicht gelesen werden
widgets-target-missing = Scan-Ziel existiert nicht oder enthält ein ungültiges NUL-Byte
widgets-target-not-scannable = Scan-Ziel ist keine reguläre Datei oder kein Verzeichnis
widgets-target-directory-unreadable = Scan-Ziel { $path } kann nicht aufgelistet werden: { $error }
widgets-file-list-unwritable = Private clamd-Dateiliste kann nicht geschrieben werden: { $error }
widgets-enumeration-interrupted = Das Auflisten des Scan-Ziels wurde unerwartet beendet.
widgets-no-files-scanned = Keine Dateien konnten gescannt werden
widgets-unscanned-files =
    { $count ->
        [one] { $count } Datei wurde nicht gescannt
       *[other] { $count } Dateien wurden nicht gescannt
    }
widgets-clamd-connection-lost = Die Verbindung zu clamd ist während des Scans abgebrochen.
widgets-scanner-stopped-reporting = Der Scanner hat für diese Dateien kein Ergebnis gemeldet.
widgets-count-not-accessed-title =
    { $count ->
        [one] Auf { $count } Datei konnte nicht zugegriffen werden
       *[other] Auf { $count } Dateien konnte nicht zugegriffen werden
    }
widgets-files-unreadable =
    { $count ->
        [one] { $count } Datei konnte nicht gelesen werden
       *[other] { $count } Dateien konnten nicht gelesen werden
    }
widgets-nonfatal-warnings =
    { $count ->
        [one] { $count } nicht schwerwiegende Warnung während des Scans; einige Dateien wurden möglicherweise nur teilweise gescannt
       *[other] { $count } nicht schwerwiegende Warnungen während des Scans; einige Dateien wurden möglicherweise nur teilweise gescannt
    }
widgets-count-threat-found-outcome =
    { $count ->
        [one] { $count } Bedrohung gefunden
       *[other] { $count } Bedrohungen gefunden
    }
widgets-count-file-scanned-outcome =
    { $count ->
        [one] { $count } Datei gescannt
       *[other] { $count } Dateien gescannt
    }
widgets-clamav-not-open-files = ClamAV konnte diese Dateien nicht öffnen.
widgets-unattributed-threats-only = Der Scanner hat eine Bedrohung gemeldet, ohne die Datei zu nennen.
widgets-preparing-scan = Wird vorbereitet …
widgets-finding-files =
    { $count ->
        [one] Dateien werden gesucht … { $count } gefunden
       *[other] Dateien werden gesucht … { $count } gefunden
    }
widgets-trust-file = Dieser Datei vertrauen …
widgets-ignore-detection-for-me = Diese Erkennung für mich ignorieren …
widgets-report-false-positive = Fehlalarm melden …
widgets-skip-path-lazaret-scans = Diesen Pfad in Scans von { -brand-name } überspringen
widgets-copy-path = Pfad kopieren
widgets-critical-severity = Kritisch
widgets-high-severity = Hoch
widgets-medium-severity = Mittel
widgets-low-severity = Niedrig
widgets-moved-quarantine-path-check = In Quarantäne verschoben: { $path }. Prüfen Sie ihren Ruf auf der Seite Quarantäne.
widgets-quarantine-detected-label = Quarantäne
widgets-more-actions-tooltip = Weitere Aktionen
widgets-count-more-file-title =
    { $count ->
        [one] { $count } weitere Datei, im exportierten Bericht aufgeführt
       *[other] { $count } weitere Dateien, im exportierten Bericht aufgeführt
    }
widgets-scan-details-title = Scan-Details
widgets-quarantine-all =
    { $count ->
        [one] { $count } Bedrohung in Quarantäne verschieben
       *[other] { $count } Bedrohungen in Quarantäne verschieben
    }
widgets-move-quarantine = In Quarantäne verschieben?
widgets-removes-each-original-file = Dadurch wird jede Originaldatei von ihrem aktuellen Speicherort entfernt.
widgets-cancel-scan = Abbrechen
widgets-quarantine-label = Quarantäne
widgets-detected-threats-title = Erkannte Bedrohungen
widgets-problems-title = Probleme
widgets-move-path-quarantine = „{ $path }“ in Quarantäne verschieben?
widgets-move-count-detected-file =
    { $count ->
        [one] { $count } erkannte Datei in Quarantäne verschieben?
       *[other] { $count } erkannte Dateien in Quarantäne verschieben?
    }
widgets-path-copied-clipboard = Pfad in die Zwischenablage kopiert
widgets-file-skipped-future-scans = Diese Datei wird bei zukünftigen Scans übersprungen
widgets-exclusion-already-exists = Dieser Ausschluss ist bereits vorhanden
widgets-results-copied-clipboard = Ergebnisse in die Zwischenablage kopiert
widgets-results-exported = Ergebnisse exportiert
widgets-done-total-file-scanned =
    { $total ->
        [one] { NUMBER($done) } von { $total } Datei gescannt
       *[other] { NUMBER($done) } von { $total } Dateien gescannt
    }
widgets-count-file-scanned-results =
    { $count ->
        [one] { $count } Datei gescannt
       *[other] { $count } Dateien gescannt
    }
widgets-count-entry-skipped-exclusions =
    { $count ->
        [one] { $count } Eintrag durch Ausschlüsse übersprungen
       *[other] { $count } Einträge durch Ausschlüsse übersprungen
    }
widgets-threats-detected-status = Bedrohungen erkannt
widgets-no-threats-detected-status = Keine Bedrohungen gefunden
widgets-finished-problems-status = Der Scan wurde nicht sauber abgeschlossen
widgets-scan-cancelled-status = Scan vom Benutzer abgebrochen
widgets-count-threat-found-results =
    { $count ->
        [one] { $count } Bedrohung gefunden
       *[other] { $count } Bedrohungen gefunden
    }
widgets-files-scanned = Gescannte Dateien
widgets-files-planned = Geplante Dateien
widgets-directories-label = Verzeichnisse
widgets-threats-found = Gefundene Bedrohungen
widgets-file-no-longer-quarantine = Die Datei ist nicht mehr vorhanden. Wenn sie sich in Quarantäne befindet, vertrauen Sie ihr auf der Seite Quarantäne.
widgets-permission-denied = Die Datei kann nicht gelesen werden, möglicherweise weil der Zugriffsscan sie blockiert. Verschieben Sie sie in Quarantäne und vertrauen Sie ihr dann auf der Seite Quarantäne.
widgets-file-not-read-scan = Die Datei konnte nicht gelesen werden: { $error }
widgets-threats-detected-finished = Bedrohungen erkannt
widgets-no-threats-detected-finished = Keine Bedrohungen gefunden
widgets-finished-problems-notify = Der Scan wurde nicht sauber abgeschlossen
widgets-scanner-error = Der Scanner wurde mit Fehlercode { $code } beendet.
widgets-killed-signal = Der Scanner wurde durch Signal { $signal } beendet.
widgets-unrecognized-wait-status = Der Scanner wurde auf eine Weise beendet, die { -brand-name } nicht erkennt (Wartestatus { $status }).
widgets-wait-failed = { -brand-name } hat den Scannerprozess aus den Augen verloren: { $error }.
widgets-scanner-program-missing = Der ClamAV-Scanner ist nicht installiert oder nicht im Suchpfad.
widgets-scanner-program-replaceable = { -brand-name } hat die Ausführung von { $program } verweigert, weil Ihr Konto { $replaceable } ersetzen kann. Installieren Sie ClamAV systemweit, damit ein von Schadsoftware platziertes Programm keine sauberen Ergebnisse vortäuschen kann.
widgets-scanner-spawn-failed = Der Scanner konnte nicht gestartet werden: { $error }
widgets-scanner-work-directory-unavailable = Das private Arbeitsverzeichnis des Scanners kann nicht erstellt werden: { $error }
widgets-scanner-output-uncaptured = Die Ausgabe des Scanners konnte nicht erfasst werden.
widgets-detection-settings-unreadable = Der Scan wurde nicht gestartet, weil die clamd-Erkennungseinstellungen nicht gelesen werden konnten: { $error }
widgets-program-lookup-failed = Der Scan wurde nicht gestartet, weil die Scanprogramme nicht ermittelt werden konnten: { $error }
widgets-scan-input-unavailable = Scan-Eingabe { $path } kann nicht sicher geöffnet werden: { $error }
widgets-detections-warnings-errors-tooltip = Nur Erkennungen, Warnungen und Fehler
widgets-copy-service-tooltip = Kopieren
widgets-export-text-tooltip = Text exportieren
widgets-search-log-placeholder = Protokoll durchsuchen
widgets-reading-log = Protokoll wird gelesen …
widgets-no-log-lines-match = Keine Protokollzeilen gefunden
widgets-log-copied-clipboard = Protokoll in die Zwischenablage kopiert
widgets-text-export-finished = Exportiert.
widgets-threat-detected = Erkennung
widgets-emergency-label = Notfall
widgets-alert-label = Alarm
widgets-critical-level = Kritisch
widgets-error-level = Fehler
widgets-warning-label = Warnung
widgets-notice-label = Hinweis
widgets-info-label = Info
widgets-debug-label = Debug
widgets-log-reading = Wird geprüft …
widgets-system-journal = Systemjournal
widgets-neither-log-file-nor = Weder eine Protokolldatei noch Journalzeilen wurden gefunden.
widgets-no-log-file-found = Es wurde keine Protokolldatei gefunden. { $reason }
widgets-program-use = { $program } in Verwendung
widgets-ignored-program = { $program } ignoriert
widgets-refused-program = { $program } abgelehnt
widgets-scanner-programs-trusted = Scanprogramme sind vertrauenswürdig
widgets-freshclam-trusted = freshclam ist vertrauenswürdig
widgets-malware-running-clamdscan = Schadsoftware, die unter Ihrem Konto läuft, könnte ein gefälschtes clamdscan weiter vorne in Ihrem Suchpfad ablegen, das jede Datei als sauber meldet. { -brand-name } führt nur ClamAV-Programme aus, die Ihr Konto nicht ersetzen kann, etwa die systemweit installierten.
widgets-malware-running-freshclam = Schadsoftware, die unter Ihrem Konto läuft, könnte ein gefälschtes freshclam weiter vorne in Ihrem Suchpfad ablegen, das nur vorgibt, neue Signaturen abzurufen. { -brand-name } führt nur ClamAV-Programme aus, die Ihr Konto nicht ersetzen kann, etwa die systemweit installierten.
widgets-check-interrupted = Unbekannt, weil die Prüfung der ClamAV-Programme vor dem Abschluss beendet wurde
widgets-clamav-program-replaced-account = Ein ClamAV-Programm kann von Ihrem Konto ersetzt werden, daher verweigert { -brand-name } dessen Ausführung
widgets-clamav-program-account-shadows = Ein ClamAV-Programm in Ihrem Konto verdeckt die Systemkopie. { -brand-name } ignoriert es, aber prüfen Sie, woher es stammt
widgets-count-settings-reduced-since =
    { $count ->
        [one] { $count } Einstellung prüft seit { $time } weniger als konfiguriert
       *[other] { $count } Einstellungen prüfen seit { $time } weniger als konfiguriert
    }
widgets-count-settings-reduced-unknown-time =
    { $count ->
        [one] { $count } Einstellung prüft weniger als konfiguriert, seit einem Zeitpunkt, der nicht gelesen werden konnte
       *[other] { $count } Einstellungen prüfen weniger als konfiguriert, seit einem Zeitpunkt, der nicht gelesen werden konnte
    }
widgets-changed-label = Geändert
widgets-change-time-unknown-override = Unbekannt
widgets-reduction-unknown-unread = Unbekannt, ob Einstellungen abgeschwächt wurden, weil die Laufzeitänderungen nicht gelesen werden konnten: { $error }
widgets-reduction-unknown-file = Unbekannt, ob Einstellungen abgeschwächt wurden, weil die Datei mit den Laufzeitänderungen nicht das erwartete Format hat. ClamAV läuft mit der Systemkonfiguration, bis dies behoben ist.
widgets-reduction-unknown-reading = Unbekannt, ob Einstellungen abgeschwächt wurden, weil das Lesen der Laufzeitänderungen vor dem Abschluss beendet wurde
widgets-settings-changed-runtime = Einstellungen zur Laufzeit geändert
widgets-administrator-can-change = Jemand mit dem Administratorpasswort kann ClamAV-Einstellungen zur Laufzeit ändern. { -brand-name } konnte diese Änderungen nicht lesen und kann daher nicht feststellen, ob sie weniger prüfen als Ihre Konfiguration.
widgets-protection-reduced-purpose = Schutz absichtlich verringert
widgets-administrator-changed-runtime = Jemand mit dem Administratorpasswort hat ClamAV-Einstellungen zur Laufzeit so geändert, dass weniger geprüft wird als in Ihrer Konfiguration. Die Systemänderungen auf der Seite, zu der die Einstellung gehört, zeigen, wer und wann.
widgets-base-configuration-changed-now = { $base } in Ihrer Konfiguration, jetzt { $changed }
widgets-administrator-changed-clamd = Jemand mit dem Administratorpasswort hat ClamAV-Einstellungen zur Laufzeit geändert. Ihre clamd.conf konnte nicht gelesen werden, daher kann { -brand-name } nicht feststellen, ob diese Änderungen weniger prüfen als Ihre Konfiguration. Die Systemänderungen auf der Seite, zu der die Einstellung gehört, zeigen, wer und wann.
widgets-settings-changed-runtime-clamd = Einstellungen wurden zur Laufzeit geändert, aber clamd.conf konnte zum Vergleich nicht gelesen werden
widgets-changed-now = jetzt { $changed }
widgets-daemon-awaited = Benötigt den Daemon, siehe Scan-Engine
widgets-unknown-run = Unbekannt. { $reason }
widgets-copy-timeline-tooltip = Kopieren
widgets-moment-details = { $moment } · { $details }
widgets-why-matters-tooltip = Warum das wichtig ist
widgets-what-is-sent-label = Was gesendet wird:
widgets-clamav-audience = Cisco Talos, das Team hinter ClamAV, prüft sie. Proben werden automatisiert verarbeitet, nur Talos hat Zugriff darauf, und sie werden nie veröffentlicht oder an andere Unternehmen weitergegeben. Eine Prüfung dauert mindestens 48 Stunden, und ihr Ergebnis zeigt sich in einer späteren Version der Signaturdatenbank.
widgets-report-false-positive-clamav = Einen Fehlalarm an ClamAV melden?
widgets-file-content = Der Inhalt der Datei
widgets-detection-name = Der Erkennungsname { $detection }
widgets-report-missed-malware-clamav = Übersehene Schadsoftware an ClamAV melden?
widgets-name-email-address-talos = Ihr Name und Ihre E-Mail-Adresse, damit Talos Sie kontaktieren kann
widgets-lazaret-sends-file-clamav = { -brand-name } sendet diese Datei mit clamsubmit an ClamAV. Senden Sie nur Dateien, die nichts Privates enthalten.
widgets-send-report-button = Meldung senden
widgets-file-content-named-sha = Der Inhalt der Datei, benannt nach seiner SHA-256-Prüfsumme, nicht nach seinem Namen
widgets-upload-malwarebazaar-heading = Auf MalwareBazaar hochladen?
widgets-lazaret-uploads-file-malwarebazaar = { -brand-name } lädt diese Datei auf MalwareBazaar hoch, betrieben von abuse.ch.
widgets-tag-clamav-detection = Das Schlagwort { $tag } aus der Erkennung von ClamAV
widgets-abuse-ch-key-sample = Ihr abuse.ch-Schlüssel; die Probe wird ohne Ihren Kontonamen gelistet
widgets-malwarebazaar-public-anyone-download = MalwareBazaar ist öffentlich: Jeder kann die Probe herunterladen, sobald sie hochgeladen ist, und ein Upload kann nicht zurückgenommen werden. Laden Sie nur Schadsoftware hoch, niemals ein Dokument, das persönliche Daten enthalten könnte.
widgets-upload-publicly-button = Öffentlich hochladen
widgets-upload-virustotal-heading = Auf VirusTotal hochladen?
widgets-lazaret-uploads-file-virustotal = { -brand-name } lädt diese Datei zur Analyse auf VirusTotal hoch.
widgets-virustotal-key-links-upload = Ihr VirusTotal-Schlüssel, der den Upload mit Ihrem Konto verknüpft
widgets-virustotal-shares-uploaded-files = VirusTotal teilt hochgeladene Dateien mit seinen zahlenden Kunden, die sie herunterladen können, und jeder kann das Ergebnis anhand der Prüfsumme der Datei nachschlagen. Laden Sie niemals ein Dokument hoch, das persönliche Daten enthalten könnte.
widgets-upload-button = Hochladen
widgets-cancel-submission = Abbrechen
widgets-name-title = Ihr Name
widgets-email-address-title = Ihre E-Mail-Adresse
widgets-file-unreadable = Die Datei konnte nicht gelesen werden: { $error }
widgets-file-not-regular = Nur reguläre Dateien können gesendet werden
widgets-file-too-large = Die Datei ist größer als { $size }, das Maximum, das { -brand-name } sendet.
widgets-reading-interrupted = Das Lesen der Datei wurde unerwartet beendet
widgets-history-failure = Der Einsendeverlauf konnte nicht gelesen werden: { $error }
widgets-earlier-submission = Diese Datei wurde bereits am { $local-time } gesendet. Es wurde nichts erneut gesendet.
widgets-report-recorded = Meldung gesendet. Die Prüfung durch ClamAV dauert mindestens 48 Stunden; Gesendete Proben zeigt an, wann eine Signaturaktualisierung das Urteil ändert.
widgets-report-not-sent-error = Die Meldung wurde nicht gesendet: { $error }
widgets-report-sent-without-record = Meldung gesendet, aber sie konnte nicht zu Gesendete Proben hinzugefügt werden: { $error }
widgets-nothing-uploaded-error = Es wurde nichts hochgeladen. { $reason }
widgets-bazaar-key-not-usable = MalwareBazaar hat keinen nutzbaren Schlüssel.
widgets-virustotal-key-not-usable = VirusTotal hat keinen nutzbaren Schlüssel.
widgets-malwarebazaar-refused-sample-reason = MalwareBazaar hat die Probe abgelehnt: { $reason }
widgets-malwarebazaar-sample-sent-samples = MalwareBazaar hat die Probe. Gesendete Proben verlinkt auf ihren öffentlichen Eintrag.
widgets-not-detected-clamav = MalwareBazaar nimmt nur Dateien an, die eine ClamAV-Signatur erkannt hat
widgets-unwanted-program-only = MalwareBazaar nimmt keine potenziell unerwünschten Programme an, nur Schadsoftware
widgets-sent-without-record = Hochgeladen, aber es konnte nicht zu Gesendete Proben hinzugefügt werden: { $error }
widgets-virustotal-refused-file-reason = VirusTotal hat die Datei abgelehnt: { $reason }
widgets-virustotal-analysing-file-refresh = VirusTotal analysiert die Datei. Aktualisieren auf der Seite Gesendete Proben zeigt das Urteil an.
widgets-report-already-available =
    { $total ->
        [one]
            { $count ->
                [one] VirusTotal kennt diese Datei bereits: { NUMBER($count) } von { $total } Engine erkennt sie. Es wurde nichts hochgeladen.
               *[other] VirusTotal kennt diese Datei bereits: { NUMBER($count) } von { $total } Engine erkennen sie. Es wurde nichts hochgeladen.
            }
       *[other]
            { $count ->
                [one] VirusTotal kennt diese Datei bereits: { NUMBER($count) } von { $total } Engines erkennt sie. Es wurde nichts hochgeladen.
               *[other] VirusTotal kennt diese Datei bereits: { NUMBER($count) } von { $total } Engines erkennen sie. Es wurde nichts hochgeladen.
            }
    }
widgets-known-user = { $local-time } · { $outcome } · angefordert von Benutzer { $uid }
widgets-unknown-user = { $time } · { $outcome } · angefordert von einem unbekannten Benutzer
widgets-change-lowered-protection-tooltip = Diese Änderung hat den Schutz verringert
widgets-system-changes-failed = Das Systemjournal kann nicht gelesen werden: { $error }
widgets-system-changes-title = Systemänderungen
widgets-refresh-tooltip = Aktualisieren
widgets-system-changes-cannot-title = Systemänderungen können nicht gelesen werden
widgets-loading-system-changes = Systemänderungen werden geladen …
widgets-trust-file-heading = Dieser Datei vertrauen?
widgets-clamav-never-report-body =
    ClamAV wird eine Datei mit genau diesem Inhalt nie wieder melden, für keinen Benutzer, egal wo sie sich auf diesem Computer befindet und welcher Scan sie findet, und der Zugriffsscan blockiert sie nicht mehr. Ihr Administratorpasswort ist erforderlich, und die Passwortabfrage zeigt dieselbe SHA-256-Prüfsumme wie unten.
    
    { $path }
widgets-stop-showing-file-body =
    { -brand-name } zeigt Ihnen eine Datei mit genau diesem Inhalt in Ihren Scans nicht mehr an, egal wo sie sich befindet. ClamAV selbst erkennt sie weiterhin, daher kann der Zugriffsscan sie weiterhin blockieren und Sie warnen, und andere Benutzer sehen sie weiterhin. Für alle Benutzer vertrauen sorgt dafür, dass ClamAV sie für niemanden mehr erkennt, und erfordert Ihr Administratorpasswort.
    
    { $path }
widgets-trust-for-me-button = Für mich vertrauen
widgets-user-trust-unreadable = Die Dateien und Erkennungen, denen Sie vertrauen, konnten nicht gelesen werden, daher zeigt dieser Scan jede Erkennung an: { $error }
widgets-trust-for-all-users-button = Für alle Benutzer vertrauen …
widgets-sha-256-hash-size =
    SHA-256 { $hash }
    { $size }
widgets-ignore-detection-heading = Diese Erkennung ignorieren?
widgets-stop-showing-detection-body = { -brand-name } zeigt Ihnen keine Datei mit dieser Erkennung mehr an, nicht nur diese eine. ClamAV selbst und andere Benutzer sehen sie weiterhin. Der Datei anhand ihrer Prüfsumme zu vertrauen ist sicherer, wenn nur eine Datei ein Fehlalarm ist.
widgets-ignore-button = Ignorieren
widgets-stop-trusting-file-heading = Dieser Datei nicht mehr vertrauen?
widgets-clamav-report-file-body = ClamAV meldet diese Datei wieder jedem Benutzer, wenn sie auf eine Signatur passt.
widgets-show-file-again-body = { -brand-name } zeigt Ihnen diese Datei wieder an, wenn ClamAV sie erkennt.
widgets-stop-trusting-button = Nicht mehr vertrauen
widgets-report-detection-again-heading = Diese Erkennung wieder melden?
widgets-clamav-report-files-body = ClamAV meldet Dateien mit dieser Erkennung wieder jedem Benutzer.
widgets-show-detection-again-body = { -brand-name } zeigt Ihnen Dateien mit dieser Erkennung wieder an.
widgets-report-again-button = Wieder melden
widgets-cancel-trust = Abbrechen

host-action-applied = Angewendet
host-action-rolled-back = Zurückgenommen
host-authorization-refused-described = Ohne Passwort abgelehnt
host-action-failed = Fehlgeschlagen
host-outcome-not-recorded = Ergebnis nicht aufgezeichnet
host-unrecognized-outcome = Unbekanntes Ergebnis { $outcome }
host-blocking-turned-off = Erkannte Dateien werden beim Öffnen nicht mehr blockiert. Sie werden nur gemeldet.
host-check-turned-off = ClamAV prüft auf diesem Computer nicht mehr auf diese Art von Datei oder Bedrohung.
host-watched-folder-removed = Dateien im entfernten Ordner werden beim Öffnen nicht mehr geprüft.
host-skipped-location-added = ClamAV überspringt den neuen Eintrag vollständig, daher bleibt Schadsoftware dort unbemerkt.
host-scan-limit-lowered = Dateien oder Archive oberhalb des neuen Limits werden ungeprüft übersprungen.
host-current-values-unknown = Der aktuelle Wert konnte nicht gelesen werden, daher könnte diese Änderung den Schutz verringern.
host-detection-trusted = ClamAV meldet nicht mehr, was Sie hier als vertrauenswürdig markieren.
host-key-now-current-new =
    { $key }
    Jetzt: { $current }
    Neu: { $new }
host-system-records-change-reduced = Das System zeichnet auf, dass diese Änderung den Schutz verringert hat, und die Seitenleiste markiert die Seite der geänderten Einstellung, bis Sie sie rückgängig machen.
host-trust-applied = ClamAV hat die Änderung geladen
host-trust-recorded-for-you = { -brand-name } zeigt Ihnen dies nicht mehr an. ClamAV selbst, das Blockieren beim Zugriff und andere Benutzer sehen es weiterhin.
host-applied-reload-pending = Gespeichert. clamd hat nicht geantwortet und übernimmt die Änderung daher beim nächsten Neuladen seiner Signaturen
host-authorization-refused-outcome = Ohne Ihr Passwort wurde nichts geändert
host-admin-service-missing = Diese Installation hat keinen ClamAV-Verwaltungsdienst, daher kann die Vertrauensliste von ClamAV nicht in { -brand-name } geändert werden
host-trust-failed = Nichts geändert: { $error }
host-nixos-configuration = { $path } wird von Ihrer NixOS-Konfiguration erzeugt, daher kann { -brand-name } die Datei nicht ändern. Verwenden Sie Konfigurationszeile kopieren bei einer Einstellung, um ihre Zeile für Ihre Konfiguration zu kopieren, und bauen Sie das System dann neu.
host-system-package = { $path } gehört zum System, daher kann { -brand-name } die Datei nicht ändern. Verwenden Sie Konfigurationszeile kopieren bei einer Einstellung, um die Zeile zu kopieren, die in diese Datei gehört.
host-contact-incomplete = Geben Sie Ihren Namen und eine gültige E-Mail-Adresse ein, um eine Meldung zu senden
host-submitter-missing = Zum Melden wird clamsubmit benötigt, das mit ClamAV geliefert wird, hier aber nicht installiert ist
host-submitter-replaceable = { -brand-name } hat die Ausführung von clamsubmit verweigert, weil Ihr Konto es ersetzen kann
host-report-not-sent-error = Die Meldung wurde nicht gesendet: { $error }
host-upload-timed-out = Die Meldung wurde nicht gesendet: Der Upload hat zu lange gedauert
host-unrecognized-source = Andere Quelle
host-file-progress =
    { $total ->
        [one] { NUMBER($done) } von { $total } Datei
       *[other] { NUMBER($done) } von { $total } Dateien
    }
host-count-second-left =
    { $count ->
        [one] Noch etwa { $count } Sekunde
       *[other] Noch etwa { $count } Sekunden
    }
host-count-minute-left =
    { $count ->
        [one] Noch etwa { $count } Minute
       *[other] Noch etwa { $count } Minuten
    }
host-count-hour-left =
    { $count ->
        [one] Noch etwa { $count } Stunde
       *[other] Noch etwa { $count } Stunden
    }
host-seconds-s = { $seconds } s
host-minutes-min-seconds-s = { $minutes } min { $seconds } s
host-hours-h-minutes-min = { $hours } h { $minutes } min
host-blocked-access = Vom Zugriffsscan blockiert
host-exact-byte-size =
    { $count ->
        [one] { $size } ({ $count } Byte)
       *[other] { $size } ({ $count } Bytes)
    }
host-hash-unknown = Nicht in der Datenbank bekannter Dateien
host-listed-malicious-source-name = Von { $source } als schädlich gelistet (als { $name })
host-listed-malicious-unnamed-source-name = Von einer unbenannten Quelle als schädlich gelistet (als { $name })
host-listed-malicious-source = Von { $source } als schädlich gelistet
host-listed-malicious-unnamed-source = Von einer unbenannten Quelle als schädlich gelistet
host-listed-known-file-source = Von { $source } als bekannte Datei gelistet
host-listed-known-file-unnamed-source = Von einer unbenannten Quelle als bekannte Datei gelistet
host-known-file-name-listed = Bekannte Datei { $name }, gelistet von { $source }
host-known-file-name-unnamed-source = Bekannte Datei { $name }, gelistet von einer unbenannten Quelle
host-unnamed-family = unbenannte Familie
host-no-known-samples = Keine bekannte Schadsoftware-Probe
host-known-malware-sample = Bekannte Schadsoftware-Probe
host-samples-found = Bekannte Schadsoftware: { $family }, zuerst gesehen { $date }
host-samples-found-date-unknown = Bekannte Schadsoftware: { $family }, zuerst gesehen an einem unbekannten Datum
host-unnamed-family-samples-found = Bekannte Schadsoftware einer unbenannten Familie, zuerst gesehen { $date }
host-unnamed-family-date-unknown = Bekannte Schadsoftware einer unbenannten Familie, zuerst gesehen an einem unbekannten Datum
host-file-unknown-virustotal = VirusTotal unbekannt
host-count-total-engine-verdict =
    { $total ->
        [one]
            { $count ->
                [one] { NUMBER($count) } von { $total } Engine erkennt sie
               *[other] { NUMBER($count) } von { $total } Engine erkennen sie
            }
       *[other]
            { $count ->
                [one] { NUMBER($count) } von { $total } Engines erkennt sie
               *[other] { NUMBER($count) } von { $total } Engines erkennen sie
            }
    }
host-api-key-rejected = VirusTotal hat den API-Schlüssel abgelehnt.
host-request-limit-reached = Das Anfragelimit von VirusTotal wurde erreicht. Versuchen Sie es später erneut.
host-virustotal-answered-status = VirusTotal antwortete mit Status { $status }
host-virustotal-not-reached-error = VirusTotal konnte nicht erreicht werden: { $error }
host-virustotal-sent-answer-lazaret = VirusTotal hat eine Antwort gesendet, die { -brand-name } nicht lesen kann
host-request-failed-circl = CIRCL hashlookup konnte nicht erreicht werden: { $error }
host-answer-unreadable-circl = CIRCL hashlookup hat eine Antwort gesendet, die nicht gelesen werden konnte
host-unexpected-status-circl = CIRCL hashlookup antwortete mit Status { $status }
host-request-failed-bazaar = MalwareBazaar konnte nicht erreicht werden: { $error }
host-answer-unreadable-bazaar = MalwareBazaar hat eine Antwort gesendet, die nicht gelesen werden konnte
host-request-limit-reached-bazaar = Das Anfragelimit von MalwareBazaar wurde erreicht. Versuchen Sie es später erneut.
host-unexpected-status-bazaar = MalwareBazaar antwortete mit Status { $status }
host-key-refused = MalwareBazaar hat den API-Schlüssel abgelehnt
host-query-refused = MalwareBazaar antwortete: { $status }
headless-count-threat-found =
    { $count ->
        [one] { $count } Bedrohung gefunden
       *[other] { $count } Bedrohungen gefunden
    }
headless-open-lazaret-review = Öffnen Sie { -brand-name }, um sie zu prüfen.
headless-count-threats-quarantined =
    { $count ->
        [one] { $count } in Quarantäne verschoben.
       *[other] { $count } in Quarantäne verschoben.
    }
headless-scan-infected = Bedrohungen erkannt
headless-scheduled-scan-failed = Geplanter Scan fehlgeschlagen
headless-open-lazaret-read-log = Öffnen Sie { -brand-name }, um das Protokoll zu lesen.
