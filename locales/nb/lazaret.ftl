-brand-name = Lazaret
startup-settings-schema-missing = Innstillingsskjemaet for { -brand-name } er ikke installert. Kjør make install, eller sett GSETTINGS_SCHEMA_DIR.
startup-unknown-page-name = { -brand-name } har ingen side som heter { $page }, så det åpner det første området som trenger oppmerksomhet. Sider: { $pages }

window-signature-sources = Signaturkilder
window-database-updates = Databaseoppdateringer
window-scan-engine = Skannemotor
window-access-scanning = Skanning ved tilgang
window-scheduled-scans-page = Planlagte skanninger
window-manual-scan = Manuell skanning
window-removable-drives = Flyttbare stasjoner
window-detections-label = Oppdagelser
window-quarantine-label = Sett i karantene
window-trusted-detections = Klarerte oppdagelser
window-sent-samples = Innsendte prøver
window-quarantine-database-not-locked = Karantenedatabasen kunne ikke låses
window-quarantine-database-not-opened = Karantenedatabasen kunne ikke åpnes
window-file-operation-failed-error = En filoperasjon mislyktes: { $error }
window-lock-task-interrupted = Et karantenesteg i bakgrunnen stoppet uventet: { $error }
window-translator-credits = John Erling Blad https://github.com/jeblad
window-export-history-menu-item = Eksporter historikk …
window-clear-history-menu-item = Tøm historikk …
window-preferences-label = Brukervalg
window-about-menu-item = Om { -brand-name }
window-quit-menu-item = Avslutt
window-export-history-heading = Vil du eksportere historikken?
window-export-history-body = Lagrer alle registrerte skanninger og signaturoppdateringer, med detaljer.
window-cancel-history-button = Avbryt
window-json-label = JSON
window-csv-label = CSV
window-clear-history-heading = Vil du tømme historikken?
window-clear-history-body = Dette sletter alle registrerte skanninger og signaturoppdateringer for godt. Dette kan ikke angres.
window-clear-label = Tøm alle
window-scanning-drive-title = Skanner { $drive }
window-open-removable-drives-button = Åpne flyttbare stasjoner
window-not-export-history-toast = Kunne ikke eksportere historikken: { $error }
window-history-export-finished = Historikken er eksportert
window-history-cleared = Historikken er tømt
window-not-clear-history-toast = Kunne ikke tømme historikken: { $error }
window-other-sources-unknown = Andre kilder ukjent
window-source-out-date = En kilde er utdatert
window-official-signatures = Bare offisielle signaturer
window-official-count-other-source =
    { $count ->
        [one] Offisielle og { $count } annen kilde
       *[other] Offisielle og { $count } andre kilder
    }
window-database-missing = Ingen virusdefinisjoner
window-age-definitions-unknown = Definisjonenes alder er ukjent
window-last-update-failed = Siste oppdatering mislyktes
window-definitions-dated-future = Definisjonene er datert fram i tid
window-updated-label = Oppdatert { $relative-time }
window-definitions-label = Definisjoner fra { $relative-time }
window-freshclam-refused = freshclam blir avvist
window-unknown-whether-freshclam-trusted = Ukjent om freshclam er klarert
window-freshclam-copy-account-ignored = En freshclam-kopi i kontoen din ignoreres
window-clamd-uses-older-signatures = clamd bruker eldre signaturer
window-clamd-answering = clamd svarer
window-unit-active-not-answering = clamd svarer ikke
window-unit-state-unknown-daemon = clamd-tilstand ukjent
window-unit-stopped-daemon = clamd kjører ikke
window-never-set-up-daemon = clamd er ikke satt opp
window-engine-missing = ClamAV er ikke installert
window-scanner-refused = clamscan blir avvist
window-version-unverified = ClamAV-versjon ukjent
window-scanner-program-refused = Et skanneprogram blir avvist
window-unknown-whether-scanners-trusted = Ukjent om skannerne er klarert
window-scanner-copy-account-ignored = En skannerkopi i kontoen din ignoreres
window-runtime-changes-unknown = Kjøretidsendringer ukjent
window-settings-reduced-runtime = Innstillinger redusert under kjøring
window-waiting-daemon = Venter på clamd
window-unit-state-unknown-access = Tilstand ukjent
window-never-set-up-access = Ikke satt opp
window-unit-stopped-access = clamonacc kjører ikke
window-config-unreadable = clamd.conf kan ikke leses
window-nothing-watched = Ingenting overvåkes
window-checks-files-not-block = Sjekker filer, blokkerer ikke
window-checks-files-kernel-cannot = Sjekker filer, kjernen kan ikke blokkere
window-checks-files-blocking-unknown = Sjekker filer, blokkering ukjent
window-blocking-paths = Blokkerer { $paths }
window-folder-watch-limit-unknown = Grense for mappeovervåking ukjent
window-mount-watch-conflicts = Konflikt i overvåking av monteringspunkt
window-clamd-user-not-excluded = clamd er ikke unntatt
window-whole-root-included = Overvåker / direkte
window-watch-limit-low = Lav grense for mappeovervåking
window-timer-not-active = Tidsuret er ikke aktivt
window-last-scan-failed = Siste skanning mislyktes
window-next-scan = Neste skanning { $relative-time }
window-no-next-scan-planned = Ingen neste skanning planlagt
window-service-missing = Skannetjeneste mangler
window-service-unreadable = Skannetjeneste ukjent
window-lazaret-schedule = Planlagt av { -brand-name }
window-no-schedule = Ingen skanning planlagt
window-schedule-changed-outside-lazaret = Planen er endret utenfor { -brand-name }
window-ready-scan = Klar til skanning
window-uses-slower-clamscan = Bruker den tregere clamscan
window-exclusion-skip-malware = Et unntak kan hoppe over skadevare
window-device-scan-running = Skanner { $drive }
window-device-scan-ended = Trusler funnet på { $drive }
window-journal-unreadable = Journalen kan ikke leses
window-following-failed = Oppfølging av journalen stoppet
window-notifications-off = Varsler av
window-alerts-stop-lazaret-quits = Varsler stopper når { -brand-name } avsluttes
window-other-programs-attach-lazaret = Andre programmer kan koble seg til { -brand-name }
window-unknown-whether-programs-attach = Ukjent om programmer kan koble seg til
window-nothing-decide = Ingenting å avgjøre
window-count-needs-decision =
    { $count ->
        [one] { $count } trenger en avgjørelse
       *[other] { $count } trenger en avgjørelse
    }
window-loading-label = Laster inn …
window-quarantine-empty = Tom
window-count-item =
    { $count ->
        [one] { $count } element
       *[other] { $count } elementer
    }
window-quarantine-cannot-read = Karantenen kan ikke leses
window-nothing-trusted = Ingenting klarert
window-count-item-trusted =
    { $count ->
        [one] { $count } element klarert
       *[other] { $count } elementer klarert
    }
window-signature-folder-unreadable = Signaturmappen kan ikke leses
window-another-allowlist-silences-clamav = En annen tillatelsesliste demper ClamAV
window-lazaret-allowlist-unreadable = Tillatelseslisten til { -brand-name } kan ikke leses
window-sent-samples-cannot-read = Innsendte prøver kan ikke leses
window-nothing-sent = Ingenting sendt
window-count-samples-refused =
    { $count ->
        [one] { $count } avvist av tjenesten sin
       *[other] { $count } avvist av tjenestene sine
    }
window-count-samples-awaiting-review =
    { $count ->
        [one] { $count } venter på vurdering
       *[other] { $count } venter på vurdering
    }
window-count-samples-sent =
    { $count ->
        [one] { $count } sendt
       *[other] { $count } sendt
    }
window-signatures-label = Signaturer
window-scans-label = Skanninger
window-findings-label = Funn
window-detections-come = Hvor oppdagelsene kommer fra
window-files-get-checked = Hvordan filer sjekkes
window-found-label = Hva som ble funnet
window-needs-review = Trenger vurdering
window-not-working = Fungerer ikke
window-waiting-another-part = Venter på en annen del
window-summary-pending = Sjekker …
window-menu-tooltip = Meny
window-show-lazaret = Vis { -brand-name }
window-show-detections = Vis oppdagelser
window-quick-scan = Rask skanning
window-update-definitions = Oppdater definisjoner
window-quit-tray-item = Avslutt
window-quit-completely = Avslutt helt
window-hide-tray = Minimer til varslingsområdet
window-ask-on-next-close = Spør ved neste lukking
window-close-lazaret = Vil du lukke { -brand-name }?
window-detection-alerts-stop-lazaret = Varsler om oppdagelser stopper mens { -brand-name } ikke kjører. Vil du la det kjøre i bakgrunnen, eller avslutte helt? Du kan endre dette senere i Brukervalg.
window-cancel-close-button = Avbryt
window-quit-close-button = Avslutt
window-keep-running = Fortsett å kjøre

scans-node-js-dependencies = Node.js-avhengigheter
scans-git-repository-data = Data fra Git-depot
scans-python-virtual-environment = Virtuelt Python-miljø
scans-build-output-directory = Mappe for byggeresultater
scans-distribution-output-directory = Mappe for distribusjonsfiler
scans-python-bytecode-cache = Hurtigbuffer for Python-bytekode
scans-remove-exclusion-tooltip = Fjern
scans-preset-exclusions-title = Forhåndsinnstilte ekskluderinger
scans-common-directories-exclude-description = Vanlige mapper å ekskludere fra skanning. Endringer lagres umiddelbart.
scans-custom-exclusions-title = Egendefinerte ekskluderinger
scans-enter-absolute-path-description = Skriv inn en absolutt sti eller et mønster som *.tmp. Endringer lagres umiddelbart.
scans-path-exclude-tmp-title = /path/to/exclude eller *.tmp
scans-prefer-clamd-when-available = Automatisk
scans-always-clamd = clamd (tjeneste)
scans-always-clamscan = clamscan (frittstående)
scans-options-title = Alternativer
scans-backend-title = Skannemotor
scans-automatic-uses-daemon-subtitle = Automatisk bruker tjenesten når den svarer, og ellers clamscan.
scans-show-live-scan-title = Vis skannefremdrift fortløpende
scans-display-files-scanned-subtitle = Vis filer som skannes i sanntid.
scans-not-added-history-error = Skanningen kunne ikke legges til i historikken: { $error }
scans-no-profile-manual = Ingen profil (manuelt)
scans-removable-drive-scanned-title = En flyttbar stasjon skannes. Start denne skanningen når den er ferdig.
scans-open-removable-drives-button = Åpne flyttbare stasjoner
scans-detecting-backend = Sjekker …
scans-backend-undetermined = Motor: ukjent, skanneprogrammene kunne ikke slås opp: { $error }
scans-backend-clamd-daemon = Motor: clamd (tjeneste)
scans-backend-clamscan-standalone = Motor: clamscan (frittstående)
scans-scan-title = Skanning
scans-preparing-label = Forbereder …
scans-cancelling-label = Avbryter …
scans-scanning-name = Skanner { $name }
scans-scanning-count-location =
    { $count ->
        [one] Skanner { $count } plassering
       *[other] Skanner { $count } plasseringer
    }
scans-cancelled-user = Skanning avbrutt av brukeren
scans-threats-found-scan = Trusler funnet
scans-not-finish-cleanly = Skanningen ble ikke fullført feilfritt
scans-no-threats-found = Ingen trusler funnet
scans-not-create-eicar-test = Kunne ikke opprette EICAR-testfilen: { $error }
scans-add-files-tooltip = Legg til filer
scans-add-folder-manual-tooltip = Legg til mappe
scans-more-tooltip = Mer
scans-run-eicar-test-label = Kjør EICAR-test
scans-copy-results-label = Kopier resultater
scans-export-results-label = Eksporter resultater …
scans-start-scan-label = Start skanning
scans-cancel-manual-label = Avbryt
scans-lazaret-refuses-clamav-title = { -brand-name } avviser et ClamAV-program som kontoen din kan erstatte
scans-details-button = Detaljer
scans-profile-title = Skanneprofil
scans-profile-short-title = Profil
scans-manage-profiles-tooltip = Administrer profiler
scans-targets-manual-title = Skannemål
scans-drop-files-folders-label = Slipp filer eller mapper for å skanne dem
scans-add-files-label = Legg til filer …
scans-add-folder-label = Legg til mappe …
scans-found-far-scan-title = Funnet så langt
scans-new-scan-button = Ny skanning
scans-recent-scans-title = Nylige skanninger
scans-started-lazaret-newest-description = Skanninger startet fra { -brand-name }, nyeste først. Velg en for å lese utskriften.
scans-choose-files-scan = Velg filer å skanne
scans-choose-folder-scan = Velg en mappe å skanne
scans-quick-scan = Rask skanning
scans-full-scan = Full skanning
scans-home-folder = Hjemmappe
scans-fast-scan-downloads-folder = Rask skanning av nedlastingsmappen for rask trusseldeteksjon
scans-comprehensive-system-wide-scan = Omfattende systemomfattende skanning av alle tilgjengelige mapper
scans-user-home-directory-personal = Skanning av brukerens hjemmemappe og personlige filer
scans-profile-file-unavailable = Profilfilen var ikke tilgjengelig: { $error }
scans-profile-data-invalid = Profildataene er ugyldige: { $error }
scans-profile-file-not-replaced = Profilfilen kunne ikke erstattes: { $error }
scans-profile-task-interrupted = Profiloppgaven ble avbrutt: { $error }
scans-profile-name-missing = Profilen må ha et navn
scans-profile-name-too-long = Navnet må være på 50 tegn eller mindre
scans-profile-name-taken = En annen profil bruker allerede dette navnet
scans-profile-targets-missing = Legg til minst ett mål
scans-edit-profile-title = Rediger profil
scans-save-label = Lagre
scans-name-title = Navn
scans-description-title = Beskrivelse
scans-targets-profile-title = Mål
scans-add-folder-profile-tooltip = Legg til mappe
scans-add-path-title = Legg til bane
scans-exclusions-title = Unntak
scans-added-exclusions-preferences-description = Legges til unntakene fra Brukervalg mens denne profilen kjører.
scans-add-pattern-title = Legg til mønster
scans-cancel-profile = Avbryt
scans-defaults-restored = Standardprofiler gjenopprettet
scans-profile-deleted = Profilen er slettet
scans-profile-imported = Profilen er importert
scans-manage-profiles-title = Administrer profiler
scans-restore-defaults-tooltip = Gjenopprett standarder
scans-new-profile-tooltip = Ny profil
scans-import-tooltip = Import
scans-profiles-title = Skanneprofiler
scans-select-profile-edit-description = Velg en profil å redigere eller bruke til skanning
scans-restore-default-profiles = Gjenopprett standardprofiler
scans-restore-built-default-profiles = Vil du gjenopprette de innebygde standardprofilene? Eventuelle tilpasninger til standardprofiler vil bli tilbakestilt; tilpassede profiler er bevart.
scans-delete-profile = Slett profil
scans-delete-profile-name = Vil du slette profilen «{ $name }»?
scans-delete-label = Slett
scans-profile-exported = Profilen er eksportert
scans-remove-profile-tooltip = Fjern
scans-count-target =
    { $count ->
        [one] { $count } mål
       *[other] { $count } mål
    }
scans-default-profile = Standardprofil
scans-run-tooltip = Kjør
scans-edit-tooltip = Redigere
scans-delete-tooltip = Slett
scans-export-tooltip = Eksporter
scans-remove-target-tooltip = Fjern
scans-check-again-access-tooltip = Sjekk på nytt
scans-checking-access-tooltip = Kontrollerer
scans-status-access-title = Status
scans-access-settings-changed-description = Innstillinger for skanning ved tilgang endret gjennom { -brand-name }, slik systemet har registrert dem. Programvare som kjører som deg, kan ikke fjerne disse oppføringene.
scans-no-access-settings-title = Ingen innstillinger for skanning ved tilgang er endret gjennom { -brand-name }
scans-clamonacc-log-title = clamonacc-logg
scans-clamonacc-not-written-title = clamonacc har ikke skrevet til journalen
scans-checking-access-scanning = Sjekker skanning ved tilgang …
scans-command-copied-clipboard-access = Kommandoen er kopiert til utklippstavlen
scans-no-readable-config = Fant ingen lesbar clamd.conf
scans-reading-task-aborted-clamd = Lesingen av clamd.conf stoppet før den var ferdig
scans-watched-mount = { $path } (hele monteringspunktet)
scans-nothing-watched = Ingen satt opp
scans-default-limit = ClamAV-standarden
scans-configuration-label = Oppsett
scans-watched-folders = Overvåkede mapper
scans-largest-file-scanned = Største fil som skannes
scans-no-kernel-config-found = Verken /proc/config.gz eller kjernens oppsettsfil i /boot finnes.
scans-kernel-config-unreadable = Kjerneoppsettet kunne ikke leses.
scans-compressed-config-corrupt = /proc/config.gz kunne ikke pakkes ut.
scans-kernel-release-unreadable = Versjonen til kjernen som kjører, kunne ikke leses.
scans-reading-task-aborted-config = Lesingen av kjerneoppsettet stoppet før den var ferdig.
scans-blocks-detected-files = Oppdagede filer blokkeres
scans-prevention-off = Filer sjekkes, men oppdagede filer blokkeres ikke
scans-prevention-conflicts-mounts = Filer sjekkes, men oppdagede filer blokkeres ikke, fordi OnAccessPrevention ikke fungerer med OnAccessMountPath
scans-kernel-cannot-block = Filer sjekkes, men denne kjernen kan ikke blokkere oppdagede filer, fordi den ble bygget uten CONFIG_FANOTIFY_ACCESS_PERMISSIONS
scans-kernel-support-unknown = Filer sjekkes, men det er ukjent om kjernen kan blokkere oppdagede filer. { $reason }
scans-config-unreadable-monitor = Kjører, men clamd.conf kunne ikke leses, så det er ukjent hva som sjekkes og om oppdagede filer blokkeres
scans-running-no-folders-status = Kjører, men ingen mapper eller monteringspunkter overvåkes
scans-running-detected-status = Kjører, men ingen mapper eller monteringspunkter overvåkes, og oppdagede filer ville ikke blitt blokkert
scans-unit-state-unknown-access = Ukjent om skanning ved tilgang kjører, fordi systemd ikke kunne rapportere det
scans-never-set-up-access = Skanning ved tilgang er ikke satt opp
scans-unit-stopped-access = { $unit } kjører ikke
scans-access-blocking-title = Blokkering ved tilgang
scans-clamonacc-checks-files-watched = clamonacc sjekker filer i de overvåkede mappene idet de åpnes. Med forebygging på nekter kjernen tilgang til en oppdaget fil, og derfor kan en slik fil ikke åpnes selv om den fortsatt ligger der.
scans-mount-watching = Overvåking av monteringspunkt
scans-onaccessmountpath-watches-whole-mount = OnAccessMountPath overvåker et helt monteringspunkt gjennom fanotify. ClamAV dokumenterer at det ikke er forenlig med OnAccessPrevention, OnAccessIncludePath, OnAccessExcludePath og OnAccessExtraScanning, så oppdagede filer på et overvåket monteringspunkt blokkeres aldri. Overvåk mapper med OnAccessIncludePath for å blokkere.
scans-onaccessmountpath-cannot-combined-options = OnAccessMountPath kan ikke kombineres med { $options }
scans-loop-protection = Beskyttelse mot skanneløkker
scans-clamd-opens-every-file = clamd åpner hver fil den blir bedt om å skanne. Med mindre OnAccessExcludeUname eller OnAccessExcludeUID angir brukeren clamd kjører som, ser clamonacc disse lesingene og ber clamd skanne de samme filene på nytt.
scans-clamd-not-excluded-clamonacc = clamd er ikke unntatt, så clamonacc kan skanne sine egne lesinger i det uendelige
scans-watching-whole-file-system = Overvåker hele filsystemet
scans-clamonacc-refuses-onaccessincludepath-include = clamonacc avviser OnAccessIncludePath /. Ta med mappene som mottar filer, for eksempel hjemmemapper, eller overvåk et monteringspunkt med OnAccessMountPath.
scans-onaccessincludepath-rejected-clamonacc = OnAccessIncludePath / blir avvist av clamonacc
scans-folder-watch-limit = Grense for mappeovervåking
scans-path-watched-fs-inotify = { $path } overvåkes, men fs.inotify.max_user_watches er bare { $limit }, så mapper utover grensen blir ikke sjekket
scans-unknown-whether-path-fits = Ukjent om { $path } passer innenfor grensen for mappeovervåking, fordi fs.inotify.max_user_watches ikke kunne leses
scans-clamonacc-watches-every-folder = clamonacc overvåker hver mappe under en inkludert sti med inotify, én overvåking per mappe. En hel hjemmemappe eller et større tre kan trenge flere overvåkinger enn den gamle standardgrensen på 8192 tillater. Øk fs.inotify.max_user_watches med sysctl.
scans-device-auto-scan-title = Automatisk skanning av enheter
scans-automatically-scan-removable-description = Skann flyttbare og eksterne stasjoner automatisk når de kobles til.
scans-enable-device-auto-title = Slå på automatisk skanning av enheter
scans-max-device-size-title = Maks enhetsstørrelse (GB)
scans-no-size-limit-subtitle = 0 betyr ingen størrelsesgrense.
scans-skip-battery-device-title = Hopp over på batteri
scans-drive-scan-not-added = Stasjonsskanningen kunne ikke legges til i historikken: { $error }
scans-device-auto-scan = Automatisk skanning av enheter
scans-device-connected-during-another = En enhet ble koblet til under en annen skanning, så den ble ikke skannet.
scans-removable-drives-title = Flyttbare stasjoner
scans-cancel-removable-label = Avbryt
scans-found-far-removable-title = Funnet så langt
scans-drives-session-title = Stasjoner i denne økten
scans-drives-connected-since-description = Stasjoner koblet til siden { -brand-name } startet, nyeste først, og hva som skjedde med hver. Denne listen beholdes bare til { -brand-name } avsluttes.
scans-no-drive-connected-title = Ingen stasjon er koblet til ennå
scans-close-results-button = Lukk resultater
scans-drive-scans-title = Stasjonsskanninger
scans-drives-lazaret-scanned-description = Stasjoner { -brand-name } skannet da de ble koblet til, nyeste først. Velg en for å lese utskriften.
scans-auto-scan-off = Av
scans-every-drive = Skanner alle stasjoner
scans-drives-up-size = Skanner stasjoner opptil { $size }
scans-skipped-larger-than-limit = Ikke skannet: { $size } er større enn grensen på { $limit }
scans-skipped-battery = Ikke skannet: datamaskinen gikk på batteri
scans-skipped-auto-scan-off = Ikke skannet: automatisk skanning av enheter er av
scans-skipped-while-scanning = Ikke skannet: en annen skanning pågikk
scans-size-unreadable = Ikke skannet: størrelsen på stasjonen kunne ikke leses
scans-drive-settling = Sjekker stasjonen …
scans-drive-scanning = Skanner …
scans-threats-detected = Skannet, trusler funnet
scans-no-threats-detected = Skannet, ingen trusler funnet
scans-finished-problems = Skannet, men skanningen ble ikke fullført feilfritt
scans-scan-cancelled = Skanning avbrutt
scans-device-scan-running = Skanner { $drive }
scans-drive-scan = Stasjonsskanning
scans-check-again-scan-tooltip = Sjekk på nytt
scans-checking-scan-tooltip = Kontrollerer
scans-status-scan-title = Status
scans-engine-detection-settings-description = Innstillinger for motor og deteksjon endret gjennom { -brand-name }, slik systemet har registrert dem. Programvare som kjører som deg, kan ikke fjerne disse oppføringene.
scans-no-engine-detection-title = Ingen innstillinger for motor eller deteksjon er endret gjennom { -brand-name }
scans-clamd-log-title = clamd-logg
scans-no-clamd-log-title = Fant ingen clamd-logg
scans-checking-scan-engine = Sjekker skannemotoren …
scans-command-copied-clipboard-scan = Kommandoen er kopiert til utklippstavlen
scans-program-start-failed = clamscan kunne ikke startes: { $error }
scans-probe-timed-out = clamscan svarte ikke i tide
scans-scanner-refused = avviste { $path }, fordi kontoen din kan erstatte den
scans-engine-missing = ClamAV ble ikke funnet på denne datamaskinen
scans-clamav-installed = ClamAV er installert
scans-clamav-engine-recognizes-malware = ClamAV er motoren som gjenkjenner skadevare. { -brand-name } bruker kopien som er installert på denne datamaskinen, og har aldri med sin egen.
scans-clamd-still-uses-signatures = clamd bruker fortsatt signaturene { $loaded }, men { $disk } ligger på disken
scans-clamd-answering = clamd svarer
scans-unit-active-not-answering = { $unit } kjører, men clamd svarer ikke. Den laster kanskje fortsatt inn signaturer.
scans-unit-state-unknown-daemon = clamd svarer ikke, og systemd kunne ikke rapportere tilstanden
scans-unit-stopped-daemon = { $unit } kjører ikke
scans-never-set-up-daemon = clamd er ikke satt opp, så skanninger faller tilbake til den tregere clamscan
scans-same-version-disk = Samme versjon som på disken
scans-loaded-disk = { $loaded }, mens { $disk } ligger på disken
scans-unknown-loaded-stored-versions = Ukjent, de innlastede og lagrede versjonene kunne ikke sammenlignes
scans-loaded-signatures = Innlastede signaturer
scans-daemon-running = Tjenesten kjører
scans-clamd-keeps-signatures-loaded = clamd holder signaturene lastet inn i minnet. Blokkering ved tilgang og planlagte skanninger sender begge filene sine dit.
scans-socket-protected = Bare systemet kan erstatte { $path }
scans-socket-replaceable = Kontoen din kan erstatte { $replaceable }, så et program som kjører som deg, kan legge en falsk clamd på { $path } som melder hver fil som ren
scans-socket-missing = Ingen clamd-sokkel på { $path }, så clamd kjører ikke eller lytter et annet sted
scans-socket-not-checkable = Flatpak-sandkassen kan ikke se hvem som eier clamd-sokkelen
scans-clamd-socket-protected = clamd-sokkelen er beskyttet
scans-socket-mechanism = Skanninger når clamd gjennom sokkelen. La clamd opprette den i en mappe som bare root eller clamav-brukeren kan skrive til, for eksempel /run/clamav, slik at ingen programmer som kjører som deg, kan legge en falsk clamd i stedet.
scans-socket-inspecting = Sjekker hvem som kan erstatte clamd-sokkelen …
scans-timer-inactive = Ikke planlagt
scans-next-run-time = Neste kjøring: { $time }
scans-scheduled-time-next-run = Planlagt. Tidspunktet for neste kjøring er ikke kjent ennå.
scans-systemd-unavailable = Planlagte skanninger krever en systemd-brukerøkt, som ikke er tilgjengelig.
scans-system-already-runs-scheduled = Systemet ditt kjører allerede planlagte skanninger, så disse innstillingene er låst for å unngå en ekstra plan. Neste skanning: { $time }. Endre den i systemkonfigurasjonen.
scans-system-scan-timer-settings = Systemet ditt har et skannetidsur, så disse innstillingene er låst for å unngå en ekstra plan. Endre det i systemkonfigurasjonen.
scans-reading-user-timer = Sjekker …
scans-inspecting-system-timer = Sjekker om systemet ditt allerede kjører planlagte skanninger. Disse innstillingene er låst inntil da.
scans-system-timer-unknown = { -brand-name } kunne ikke sjekke om systemet ditt allerede kjører planlagte skanninger, så disse innstillingene er låst for å unngå en ekstra plan: { $error }
scans-scan-time-unreadable = Skriv inn skannetiden som TT:MM, for eksempel 02:00.
scans-unit-files-unwritable = Feil ved oppretting av systemd-filer: { $error }
scans-timer-not-applied-error = Tidsuret kunne ikke tas i bruk: { $error }
scans-hourly-label = Hver time
scans-daily-label = Daglig
scans-weekly-label = Ukentlig
scans-monthly-label = Månedlig
scans-monday-label = mandag
scans-tuesday-label = tirsdag
scans-wednesday-label = onsdag
scans-thursday-label = torsdag
scans-friday-label = fredag
scans-saturday-label = lørdag
scans-sunday-label = søndag
scans-schedule-title = Plan
scans-apply-label = Bruk
scans-enable-scheduled-scans-title = Slå på planlagte skanninger
scans-frequency-title = Skannehyppighet
scans-time-24-hour-title = Skannetid (24 timer)
scans-day-week-title = Ukedag
scans-used-weekly-scans-subtitle = Brukes til ukentlige skanninger.
scans-day-month-title = Dag i måneden
scans-used-monthly-scans-subtitle = Brukes til månedlige skanninger.
scans-targets-separated-commas-title = Skannemål (adskilt med komma; tomt skanner hjemmemappen din)
scans-skip-battery-schedule-title = Hopp over på batteri
scans-not-start-scheduled-subtitle = Ikke start planlagte skanninger mens du er på batteri.
scans-auto-quarantine-title = Automatisk karantene
scans-automatically-quarantine-threats-subtitle = Sett trusler funnet av planlagte skanninger i karantene automatisk. Mens { -brand-name } er åpent, blir trusler bare rapportert.
scans-review-button = Vurder
scans-check-again-scheduled-tooltip = Sjekk på nytt
scans-checking-scheduled-tooltip = Kontrollerer
scans-system-scan-title = Systemskanning
scans-folders-scanned-title = Skannede mapper
scans-run-now-label = Kjør nå
scans-starting-scan-tooltip = Starter skanningen …
scans-status-scheduled-title = Status
scans-scheduled-runs-title = Planlagte kjøringer
scans-lazaret-schedule-ran-description = Skanninger som planen til { -brand-name } har kjørt, nyeste først. Velg en for å lese utskriften.
scans-checking-schedule = Sjekker planen …
scans-command-copied-clipboard-scheduled = Kommandoen er kopiert til utklippstavlen
scans-not-scheduled = Ikke planlagt
scans-next-scan = Neste skanning
scans-threats-found-scheduled = trusler funnet
scans-nothing-found = ingenting funnet
scans-no-completed-scan = Ikke kjørt ennå
scans-last-scan = Siste skanning
scans-unknown-label = Ukjent
scans-system-timer = Systemtidsuret er ikke aktivt
scans-last-scan-ended-error = Siste skanning endte med en feil
scans-next-scan-time = Neste skanning: { $time }
scans-timer-starts-scan-service = Tidsuret starter en skannetjeneste som ikke er installert
scans-service-not-read-systemd = Skannetjenesten kunne ikke leses fra systemd
scans-lazaret-schedule = Planlagt av { -brand-name }
scans-no-schedule = Ingen skanning er planlagt
scans-not-scheduled-timer-active = Ikke planlagt, fordi tidsuret ikke er aktivt
scans-folders-scanned = Skannede mapper
scans-scheduled-scan = Planlagt skanning
scans-scheduled-scan-catches-files = En planlagt skanning fanger opp filer som ikke ble åpnet, eller som først ble gjenkjennelige etter en signaturoppdatering.
scans-unit-names-no-folders = Skanneenheten angir ingen mapper
scans-system-scan-service-unit = Skannetjenesten { $unit } på systemet ditt skanner disse mappene med systemets egne innstillinger. Å kjøre den krever administratorautorisasjon.
scans-needs-daemon-see-scan = Krever tjenesten, se Skannemotor
scans-bus-call-failed = systemd svarte ikke: { $error }
scans-unrecognized-load-state = systemd rapporterte en enhetstilstand som { -brand-name } ikke gjenkjenner
scans-found-malware = Skanningen fant skadevare
scans-unit-finished = Skanningen er fullført
scans-not-finish-cleanly-result = Skanningen ble ikke fullført feilfritt: { $result }
scans-authorization-refused = Å starte skanningen krever administratorautorisasjon
scans-not-started-error = Skanningen kunne ikke startes: { $error }

findings-open-preferences = Åpne brukervalg
findings-journal-unreadable-alerts = Systemjournalen kan ikke leses. Legg brukeren din til i gruppen systemd-journal.
findings-notifications-turned-off-preferences = Varsler er slått av i brukervalgene
findings-alerts-stop-quit-lazaret = Varsler stopper når du avslutter { -brand-name }, fordi det ikke er satt opp til å kjøre i bakgrunnen
findings-running-background-following-journal = Kjører i bakgrunnen og følger journalen
findings-alerts-label = Varsler
findings-lazaret-follows-system-journal = { -brand-name } følger systemjournalen og varsler deg når noe oppdages. Det kan bare gjøre det mens det kjører, så det bør kjøre i bakgrunnen.
findings-notifications-on = På
findings-notifications-off = Av
findings-running-label = Kjører
findings-not-running = Kjører ikke
findings-unit-not-found = Ikke satt opp
findings-systemd-unavailable = Ukjent, systemd kan ikke nås
findings-following-journal-access = Følges
findings-journal-unreadable-access = Kan ikke leses
findings-following-failed = Oppfølgingen stoppet
findings-notifications-label = Varsler
findings-background-service = Bakgrunnstjeneste
findings-system-journal = Systemjournal
findings-attach-unknown-setting = Det er ukjent om andre programmer kan koble seg til, fordi kjerneinnstillingen ikke kan leses: { $reason }
findings-attach-unknown-reports = Det er ukjent om andre programmer kan koble seg til, fordi kjernen rapporterer en verdi som { -brand-name } ikke gjenkjenner
findings-attach-unknown-check = Det er ukjent om andre programmer kan koble seg til, fordi kontrollen stoppet før den var ferdig
findings-attach-restricted = Programmer kan ikke koble seg til andre programmer du kjører, for eksempel { -brand-name }
findings-any-program-may-attach = Ethvert program du kjører, kan koble seg til { -brand-name } og endre det som vises, inkludert oppdagelser og avgjørelsene du tar her
findings-yama-not-loaded = Denne kjernen begrenser ikke tilkobling, så ethvert program du kjører, kan koble seg til { -brand-name } og endre det som vises
findings-other-programs-cannot-attach = Andre programmer kan ikke koble seg til { -brand-name }
findings-program-attaches-lazaret-hide = Et program som kobler seg til { -brand-name }, kan skjule varsler eller endre avgjørelsene dine. Når kernel.yama.ptrace_scope settes til 1, kan bare et programs overordnede prosess koble seg til det.
findings-file-owned-other-user = Denne filen tilhører en annen bruker, så den kan ikke flyttes eller slettes herfra
findings-file-not-checkable = Denne filen kunne ikke sjekkes, så den kan ikke flyttes eller slettes herfra
findings-removal-failed = Filen kunne ikke slettes: { $error }
findings-file-check-interrupted = Kontrollen av filen stoppet uventet
findings-history-not-updated = Avgjørelsen din kunne ikke lagres: { $error }
findings-journal-unreadable-list = { -brand-name } kan ikke lese systemjournalen, så nye oppdagelser i bakgrunnen vises ikke her. Det betyr ikke at ingenting ble funnet. Legg brukeren din til i gruppen systemd-journal.
findings-loading-detections = Laster inn oppdagelser …
findings-nothing-detected-background = Ingenting er oppdaget i bakgrunnen.
findings-access-scanning-scheduled-scans = Hva skanning ved tilgang og planlagte skanninger har funnet.
findings-detections-title = Oppdagelser
findings-back-detections-tooltip = Tilbake til oppdagelser
findings-alerts-title = Varsler
findings-detections-reach-keeps-description = Hvordan oppdagelser når fram til deg, og hva som hindrer andre programmer i å skjule dem.
findings-check-again-background-tooltip = Sjekk på nytt
findings-checking-background-tooltip = Kontrollerer
findings-detection-title = Oppdagelse
findings-copied-clipboard-toast = Kopiert til utklippstavlen
findings-command-copied-clipboard-toast = Kommandoen er kopiert til utklippstavlen
findings-detections-lazaret-stopped = Oppdagelser mens { -brand-name } ikke kjørte, kan ikke telles, fordi stedet der det sluttet å lese systemjournalen, ikke kunne leses: { $error }
findings-detections-not-read-error = Oppdagelser kunne ikke leses: { $error }
findings-malware-blocked-signature = Skadevare blokkert: { $signature }
findings-malware-found-signature = Skadevare funnet: { $signature }
findings-count-detection-not-running =
    { $count ->
        [one] { $count } oppdagelse mens { -brand-name } ikke kjørte
       *[other] { $count } oppdagelser mens { -brand-name } ikke kjørte
    }
findings-detections-not-running-detection = Oppdagelser mens { -brand-name } ikke kjørte, kan ikke telles
findings-lazaret-not-read-stopped = { -brand-name } kunne ikke lese hvor det sluttet å følge systemjournalen, så det ser bare etter nye oppdagelser. Sjekk siden Oppdagelser og systemjournalen for noe som ble funnet mens det var lukket.
findings-quarantine-incident = Sett i karantene
findings-ask-incident-deletion = Slett …
findings-delete-incident-file = Slett for godt
findings-cancel-incident-deletion = Avbryt
findings-dismiss-incident = Behold
findings-look-up-incident-signature = Slå opp
findings-submit-incident-sample = Send inn …
findings-restored-quarantine = Gjenopprettet fra karantene
findings-moved-quarantine = Flyttet til karantene
findings-deleted-label = Slettet
findings-kept-label = Beholdt av deg
findings-belongs-another-user = Tilhører en annen bruker
findings-blocked-needs-decision = Blokkert, trenger en avgjørelse
findings-not-blocked-needs-decision = Ikke blokkert, trenger en avgjørelse
findings-needs-decision = Trenger en avgjørelse
findings-no-longer-there-detection = Finnes ikke lenger
findings-state-unknown-detection = Filen kunne ikke sjekkes
findings-signature-test-file-state = { $signature } (testfil) · { $state }
findings-signature-state = { $signature } · { $state }
findings-access-schedule = Skanning ved tilgang og den planlagte skanningen
findings-access-only = Skanning ved tilgang
findings-schedule-only = Den planlagte skanningen
findings-file-label = Fil
findings-signature-source = Signaturkilde
findings-found-label = Funnet av
findings-last-seen = Sist sett
findings-times-seen = Antall ganger sett
findings-detected-label = Oppdaget
findings-signature-matches-harmless-eicar = Denne signaturen samsvarer med det ufarlige EICAR-testmønsteret, som finnes for å sjekke at deteksjonen fungerer.
findings-clamav-matched-file-against = ClamAV fant at denne filen samsvarer med en signatur. Et treff er et sterkt bevis, men falske positiver forekommer, særlig med pakkede verktøy eller administrasjonsverktøy.
findings-signature-first-seen-time = { $signature }, først sett { $local-time }
findings-opening-file-refused-why = Åpning av denne filen blir nektet, og derfor åpnes den ikke
findings-not-opened-again-after = Ikke åpnet igjen etter avgjørelsen din
findings-no-longer-there-blocked = Filen finnes ikke lenger
findings-not-blocked-scheduled-scan = Ikke blokkert: den planlagte skanningen fant den, og den kan fortsatt åpnes
findings-not-blocked-right-now = Ikke blokkert akkurat nå: den kan fortsatt åpnes
findings-state-unknown-blocked = Kunne ikke sjekke om filen er blokkert
findings-blocked-label = Blokkert
findings-access-prevention-kernel-refuses = Med forebygging ved tilgang nekter kjernen å åpne en oppdaget fil i en overvåket mappe. Filen blir liggende der den er til du bestemmer hva som skal skje med den.
findings-looking-up = Slår opp …
findings-no-known-samples-match = Ingen kjente prøver samsvarer med denne signaturen.
findings-unknown-label = ukjent
findings-count-known-samples-families = { $count } kjente prøver. Familier: { $families }. Først sett: { $date }.
findings-ask-malwarebazaar-signature-description = Spør MalwareBazaar om denne signaturen. Bare navnet sendes.
findings-set-up-lookup-button = Sett opp …
findings-bazaar-no-key = Ingen nøkkel
findings-bazaar-key-unreadable = Nøkkelen kan ikke leses
findings-stop-asking-file-description = Slutt å spørre om denne filen
findings-delete-file-permanently-description = Vil du slette denne filen for godt? Dette kan ikke angres.
findings-go-back-without-description = Gå tilbake uten å slette
findings-move-somewhere-safe-description = Flytt den til et trygt sted. Du kan gjenopprette den senere.
findings-remove-good-description = Fjern den for godt
findings-know-file-fine-description = Du vet at denne filen er i orden. ClamAV rapporterer den likevel ved neste skanning; klarer den fra Karantene eller et skanneresultat for å stoppe det.
findings-decision-title = Din avgjørelse
findings-unfamiliar-quarantine-keeping-file = Hvis den er ukjent, sett den i karantene. Å beholde en fil gjør bare at { -brand-name } slutter å spørre: skanning ved tilgang fortsetter å blokkere den til stien er unntatt i ClamAV-konfigurasjonen, som vist her.
findings-quarantined-files-restored-quarantine = Filer i karantene kan gjenopprettes fra siden Karantene. En fil du beholdt, blokkeres fortsatt av skanning ved tilgang til stien er unntatt i ClamAV-konfigurasjonen.
findings-file-other-owner-decision = Denne filen tilhører en annen bruker, så { -brand-name } kan ikke flytte eller slette den
findings-state-unknown-review = { -brand-name } kunne ikke sjekke denne filen, så det kan ikke flytte eller slette den
findings-choose-happens-file = Velg hva som skal skje med denne filen
findings-malwarebazaar-label = MalwareBazaar
findings-resolved-title = Løst
findings-resolved-detection-opens-again = En løst oppdagelse åpnes igjen hvis ClamAV finner den samme filen senere, for eksempel etter at du laster den ned en gang til.
findings-waiting-decision = Venter på avgjørelsen din
findings-resolved-state-step = { $state }, { $time }
findings-send-quarantined-copy-description = Send kopien i karantene til ClamAV, MalwareBazaar eller VirusTotal. En falsk positiv som rapporteres til ClamAV, rettes for alle ved en senere signaturoppdatering.
findings-cancel-quarantine = Avbryt
findings-delete-entries = Slett
findings-scanner-program-not-installed = Skanneprogrammet er ikke installert.
findings-scanner-program-replaced-account = Skanneprogrammet i { $program } kan erstattes av kontoen din via { $path }.
findings-content-pipe-failed = Filen kunne ikke sendes videre til skanneren: { $error }
findings-scanner-spawn-failed = Skanneren kunne ikke startes: { $error }
findings-verdict-timed-out = Skanningen tok for lang tid.
findings-detection-settings-unreadable = Deteksjonsinnstillingene til clamd kunne ikke leses: { $error }
findings-program-lookup-failed = Skanneprogrammene kunne ikke slås opp: { $error }
findings-payload-unreadable = Kopien i karantene kunne ikke leses: { $error }
findings-restore-file-heading = Vil du gjenopprette denne filen?
findings-clamav-no-longer-verdict = ClamAV oppdager ikke lenger denne filen med dagens signaturer.
findings-restore-button = Gjenopprett
findings-restore-file-clamav-heading = Vil du gjenopprette en fil som ClamAV fortsatt oppdager?
findings-clamav-still-detects-verdict = ClamAV oppdager fortsatt denne filen som { $detection } med dagens signaturer. Gjenopprett den bare hvis du er sikker på at det er en falsk positiv, og vurder å klarere den først.
findings-clamav-still-detects-unnamed-verdict = ClamAV oppdager fortsatt denne filen med dagens signaturer, uten å navngi deteksjonen. Gjenopprett den bare hvis du er sikker på at det er en falsk positiv, og vurder å klarere den først.
findings-restore-anyway-button = Gjenopprett likevel
findings-restore-unchecked-file-heading = Vil du gjenopprette en ukontrollert fil?
findings-file-not-checked-verdict = Filen kunne ikke sjekkes på nytt før den ble gjenopprettet. { $reason }
findings-location-outside-home-folder = Denne plasseringen er utenfor hjemmemappen din.
findings-file-goes-back = Filen flyttes tilbake til:
findings-entry-count =
    { $count ->
        [one] { $count } element
       *[other] { $count } elementer
    }
findings-clear-old-items = Vil du tømme gamle elementer?
findings-count-quarantined-file-older =
    { $count ->
        [one] { $count } fil i karantene som er eldre enn 30 dager, blir slettet for godt.
       *[other] { $count } filer i karantene som er eldre enn 30 dager, blir slettet for godt.
    }
findings-delete-quarantined-file = Vil du slette filen i karantene?
findings-file-permanently-deleted = Filen blir slettet for godt.
findings-quarantine-title = Sett i karantene
findings-more-tooltip = Mer
findings-clear-old-items-label = Tøm gamle elementer …
findings-check-again-label = Sjekk på nytt
findings-scan-every-quarantined-tooltip = Skann hver fil i karantene med de gjeldende signaturene
findings-refresh-tooltip = Oppdater
findings-search-tooltip = Søk
findings-search-threat-name-placeholder = Søk etter trusselnavn eller bane
findings-quarantine-storage-title = Karantenelager
findings-detected-threats-isolated-description = Oppdagede trusler isolert fra systemet ditt. Utvid en oppføring for å sjekke omdømmet.
findings-total-size-title = Total størrelse
findings-dismiss-label = Avvis
findings-quarantine-cannot-read-title = Karantenen kan ikke leses
findings-loading-quarantine = Laster inn karantene …
findings-quarantine-empty = Karantenen er tom
findings-hash-copied-clipboard = Hash kopiert til utklippstavlen
findings-every-quarantined-file-still = Hver fil i karantene oppdages fortsatt eller kunne ikke sjekkes
findings-count-quarantined-file-no =
    { $count ->
        [one] { $count } fil i karantene oppdages ikke lenger. Sjekk den før du gjenoppretter den.
       *[other] { $count } filer i karantene oppdages ikke lenger. Sjekk dem før du gjenoppretter dem.
    }
findings-count-report-clamav-settled =
    { $count ->
        [one] { $count } rapport til ClamAV er avgjort av de nye signaturene
       *[other] { $count } rapporter til ClamAV er avgjort av de nye signaturene
    }
findings-checking-label = Sjekker …
findings-no-longer-detected = Oppdages ikke lenger
findings-still-detected = Oppdages fortsatt
findings-integrity-unknown-hash = Ukjent. Denne karanteneoppføringen har ingen brukbar hash, så filen kan ikke verifiseres, sjekkes på nett, klareres eller gjenopprettes. Du kan fortsatt slette den.
findings-not-checked = Ikke sjekket ennå
findings-integrity-unknown-label = Integritet ukjent
findings-restore-label = Gjenopprett
findings-delete-entry-label = Slett
findings-reputation-title = Omdømme
findings-look-up-reputation-button = Slå opp
findings-set-up-source-button = Sett opp …
findings-reputation-no-key = Ingen nøkkel
findings-reputation-key-unreadable = Nøkkelen kan ikke leses
findings-file-sha-256-subtitle = Bare filens SHA-256-hash sendes. Selve filen forlater aldri denne datamaskinen.
findings-view-report-label = Vis rapport
findings-false-positive-title = Falsk positiv
findings-trust-stops-clamav-subtitle = Klarer … hindrer ClamAV i å rapportere akkurat denne filen på denne datamaskinen og viser den under Klarerte oppdagelser; gjenopprett den etterpå. Rapporter … sender den til Cisco Talos, slik at en senere signaturoppdatering retter deteksjonen for alle.
findings-report-label = Rapporter …
findings-submit-quarantine-label = Send inn …
findings-send-clamav-malwarebazaar-tooltip = Send den til ClamAV, MalwareBazaar eller VirusTotal
findings-trust-label = Klarer …
findings-copy-hash-tooltip = Kopier hash
findings-submit-quarantined-label = Send inn …
findings-service-label = Tjeneste
findings-status-label = Status
findings-quarantine-record-no-usable = Denne karanteneoppføringen har ingen brukbar hash, så { -brand-name } sender den ikke noe sted
findings-file-no-longer-quarantine = Denne filen er ikke lenger i karantene
findings-service-not-take-detection = Denne tjenesten tar ikke imot denne oppdagelsen
findings-file-no-usable-hash = Denne filen er ikke lenger i karantene med en brukbar hash
findings-check-malwarebazaar-virustotal-tooltip = Sjekk MalwareBazaar og VirusTotal for nytt
findings-checking-sent-tooltip = Kontrollerer
findings-send-file-title = Send en fil
findings-nothing-leaves-computer-description = Ingenting forlater denne datamaskinen før du bekrefter hva som sendes.
findings-report-missed-malware-label = Rapporter oversett skadevare …
findings-send-clamav-malicious-tooltip = Send ClamAV en skadelig fil den ikke oppdager
findings-sent-title = Sendt
findings-lazaret-sent-became-description = Hva { -brand-name } har sendt, og hva som ble resultatet.
findings-filter-sent-samples-tooltip = Filtrer innsendte prøver
findings-online-services-title = Nettjenester
findings-each-service-receives-subtitle = Hva hver tjeneste mottar, og nøklene til MalwareBazaar og VirusTotal
findings-choose-file-clamav-misses = Velg filen ClamAV ikke oppdager
findings-false-positive = Falsk positiv
findings-missed-malware = Oversett skadevare
findings-malware-sample = Skadevareprøve
findings-clamav-false-positive = Rapporter falsk positiv til ClamAV
findings-clamav-missed-malware = Rapporter oversett skadevare til ClamAV
findings-malwarebazaar-sample-choice = Last opp til MalwareBazaar
findings-virustotal-sample-choice = Last opp til VirusTotal
findings-awaiting-clamav = Venter på vurdering fra ClamAV. Vurderinger tar minst 48 timer; en senere signaturoppdatering viser utfallet.
findings-fixed-database-version = Rettet i databaseversjon { $version }
findings-fixed-current-signature-database = Rettet i den gjeldende signaturdatabasen
findings-detected-since-database-version = Oppdaget siden databaseversjon { $version }
findings-detected-current-signature-database = Oppdaget av den gjeldende signaturdatabasen
findings-uploaded-waiting-malwarebazaar-list = Lastet opp; venter på at MalwareBazaar skal oppføre den
findings-listed-malwarebazaar = Oppført på MalwareBazaar
findings-malwarebazaar-refused-reason = MalwareBazaar avviste den: { $reason }
findings-queued-analysis-virustotal = I kø for analyse på VirusTotal
findings-count-total-engine-flags =
    { $total ->
        [one] { NUMBER($count) } av { $total } motor markerer den som skadelig
       *[other] { NUMBER($count) } av { $total } motorer markerer den som skadelig
    }
findings-virustotal-refused-reason = VirusTotal avviste den: { $reason }
findings-service-kind-hash-date = { $service } · { $kind } · { $hash } · { $date }
findings-integrity-unknown-candidate = { $detection } · Ingen brukbar hash, så { -brand-name } kan ikke bevise hva det ville sende
findings-all-services = Alle tjenester
findings-any-status-option = Alle statuser
findings-waiting-label = Venter
findings-settled-label = Avgjort
findings-refused-label = Avvist
findings-all-submissions-option = Alle innsendinger
findings-listing-loading = Laster inn innsendinger …
findings-nothing-submitted = Ingenting sendt inn
findings-no-submission-matches-filters = Ingen innsending samsvarer med filtrene
findings-candidates-loading = Laster inn karantene …
findings-quarantine-empty-sent = Karantenen er tom
findings-no-upload-changed = Ingen opplasting er endret
findings-count-upload-new-status =
    { $count ->
        [one] { $count } opplasting har ny status
       *[other] { $count } opplastinger har ny status
    }
findings-choose-service-heading = Hvor skal denne filen sendes?
findings-set-up-keys-response = Sett opp nøkler …
findings-each-service-gets-file = Hver tjeneste får selve filen. Neste steg viser nøyaktig hva som sendes, og hvem som kan se det.
findings-cancel-submission = Avbryt
findings-open-remote-entry-tooltip = Åpne den eksterne oppføringen
findings-same-upload-running = Denne filen sendes allerede til denne tjenesten. Vent til det er ferdig før du sender den på nytt.
findings-check-again-trusted-tooltip = Sjekk på nytt
findings-checking-trusted-tooltip = Kontrollerer
findings-status-title = Status
findings-files-trusted-detections-description = Filer som er klarert, og oppdagelser som er ignorert eller gjenopprettet gjennom { -brand-name }, slik systemet har registrert dem. Programvare som kjører som deg, kan ikke fjerne disse oppføringene.
findings-no-trusted-detections-title = Ingen klarerte oppdagelser er endret gjennom { -brand-name }
findings-checking-trusted-detections = Sjekker klarerte oppdagelser …
findings-command-copied-clipboard = Kommandoen er kopiert til utklippstavlen
findings-other-allowlist = Annen tillatelsesliste
findings-file-reason = { $file }: { $reason }
findings-trusted-files = Klarerte filer
findings-ignored-detections = Ignorerte oppdagelser
findings-allowlist-not-write-status = En tillatelsesliste som { -brand-name } ikke har skrevet, ber ClamAV om å tie
findings-count-file-detection-status =
    { $count ->
        [one] { $count } fil eller oppdagelse rapporteres aldri
       *[other] { $count } filer eller oppdagelser rapporteres aldri
    }
findings-own-allowlist-unreadable-status = { -brand-name } kan ikke lese sin egen tillatelsesliste, så det kan ikke si hva ClamAV tier om
findings-folder-unreadable = { -brand-name } kan ikke lese signaturmappen, så det kan ikke si hva ClamAV tier om
findings-signature-folder = Signaturmappe
findings-trusted-detections = Klarerte oppdagelser
findings-trust-list-mechanism = Filer og deteksjonsnavn på ClamAVs tillatelseslister rapporteres aldri, av noen skanning. Skadevare som får administratorrettigheter, kan skjule seg på denne måten, så sjekk at hver oppføring ble lagt til med vilje.
findings-clamav-reports-everything-detects = ClamAV rapporterer alt det oppdager
findings-remove-tooltip = Fjern
findings-access-denied = Tilgang nektet
findings-other-read-failure = Lesingen mislyktes: { $reason }
findings-reading-task-aborted = Lesingen stoppet før den var ferdig
findings-journal-unreadable-added = Ukjent hvem som la til dette. { $reason }
findings-known-user = Lagt til { $local-time } av bruker { $uid }
findings-unknown-user = Lagt til { $time } av en ukjent bruker
findings-no-record-added-system = Ingen oppføring i systemjournalen om hvem som la til dette
findings-count-entry-not-added =
    { $count ->
        [one] { $count } oppføring, ikke lagt til gjennom { -brand-name }
       *[other] { $count } oppføringer, ikke lagt til gjennom { -brand-name }
    }
findings-entries-cannot-counted-reason = Oppføringene kan ikke telles ({ $reason }), ikke lagt til gjennom { -brand-name }
findings-list-unreadable = { -brand-name } kan ikke lese denne listen
findings-no-trusted-files = Ingen klarerte filer
findings-no-ignored-detections = Ingen ignorerte oppdagelser
findings-lazaret-cannot-read-signature = { -brand-name } kan ikke lese signaturmappen
findings-trusted-detections-title = Klarerte oppdagelser
findings-trusted-by-you-title = Klarert av deg
findings-trusted-by-you-description = Skanningene dine viser deg ikke disse filene, og verken skanningene dine eller varsler i bakgrunnen viser disse oppdagelsene. ClamAV oppdager dem likevel, så skanning ved tilgang kan fortsatt blokkere filene, og andre brukere ser dem fortsatt. Enhver endring i en klarert fil gjør at den vises igjen.
findings-nothing-trusted-by-you = Du har ikke klarert noe
findings-trust-by-you-unreadable = { -brand-name } kan ikke lese hva du har klarert: { $error }
findings-file-trusted-by-you = En fil du har klarert
findings-trust-for-all-users-tooltip = Klarer for alle brukere …
findings-trusted-for-all-users-title = Klarert for alle brukere
findings-clamav-never-reports-description = ClamAV rapporterer aldri det som står her, for noen bruker og i noen skanning på denne datamaskinen. Hver endring krever et administratorpassord og registreres i systemjournalen, der programvare som kjører som deg, ikke kan fjerne oppføringen.
findings-ignored-detections-title = Ignorerte oppdagelser
findings-detection-names-clamav-description = Deteksjonsnavn som ClamAV ignorerer i alle filer, for alle brukere. { -brand-name } legger ikke lenger til disse, fordi de skjuler alle filer med den deteksjonen; fjern dem og klarer filer etter hashen deres i stedet.
findings-other-allowlists-title = Andre tillatelseslister
findings-allowlist-files-signature-description = Tillatelseslistefiler i signaturmappen som { -brand-name } ikke har skrevet. Sjekk at du eller administratoren din la dem til med vilje.

signatures-history-load-failed = Kunne ikke laste inn historiske logger: { $error }
signatures-database-update-finished = Databaseoppdateringen er fullført
signatures-check-again-database-tooltip = Sjekk på nytt
signatures-checking-database-tooltip = Kontrollerer
signatures-update-history-title = Oppdateringshistorikk
signatures-updates-started-lazaret-description = Oppdateringer startet fra { -brand-name }, nyeste først. Velg en for å lese utskriften.
signatures-historical-logs-cannot-title = Historiske logger kan ikke leses
signatures-status-title = Status
signatures-loading-history = Laster inn historikk …
signatures-no-updates-started-lazaret = Ingen oppdateringer er startet fra { -brand-name } ennå
signatures-checking-signatures = Sjekker signaturene …
signatures-command-copied-clipboard-database = Kommandoen er kopiert til utklippstavlen
signatures-update-not-added-history = Oppdateringen kunne ikke legges til i historikken: { $error }
signatures-update-signatures = Oppdater nå
signatures-cancel-signature-update = Avbryt
signatures-definitions-built-date = Definisjoner bygget { $relative-time } ({ $date })
signatures-definitions-exist-not-determined = Definisjoner finnes, men alderen deres kunne ikke fastslås
signatures-no-virus-definitions-found = Fant ingen virusdefinisjoner
signatures-age-definitions-unknown-checking = Definisjonenes alder er ukjent, fordi kontrollen stoppet før den var ferdig
signatures-last-update-failed = Den siste oppdateringen mislyktes
signatures-no-completed-update = Ingen fullført oppdatering er registrert ennå
signatures-updater-updated = Oppdatert av { $updater }
signatures-provider-fetched = Hentet fra { $provider }
signatures-count-signature-size-file =
    { $count ->
        [one] { $count } signatur, { $size } ({ $file })
       *[other] { $count } signaturer, { $size } ({ $file })
    }
signatures-file-unreadable = Ukjent antall signaturer, filen kunne ikke leses: { $error }, { $size } ({ $file })
signatures-other-signature-sources = Andre signaturkilder
signatures-unknown-reason-database = Ukjent. { $reason }
signatures-fetch-newest-signatures = Hent de nyeste signaturene fra alle oppdaterere nå
signatures-manual-update = Manuell oppdatering
signatures-update-service = Oppdateringstjeneste
signatures-database-folder = Databasemappe
signatures-updater-output = Utdata fra oppdatereren
signatures-current-label = Signaturene er oppdaterte
signatures-signature-mechanism = Signaturer beskriver kjent skadevare, og nye publiseres hele tiden. Gamle signaturer går glipp av nye trusler selv når alt annet kjører.
signatures-updating-signatures = Oppdaterer signaturer …
signatures-update-cancelling = Avbryter oppdateringen …
signatures-starting-update = Starter oppdateringen …
signatures-daemon-running = Aktiv
signatures-timer-scheduled = Planlagt av et systemd-tidsur
signatures-service-stopped = Installert men stoppet
signatures-service-not-found = Ingen freshclam-tjeneste oppdaget; oppdateringer kjøres manuelt
signatures-systemd-unreachable-service = systemd kunne ikke spørres om en freshclam-tjeneste; oppdateringer kjøres manuelt
signatures-count-day-old =
    { $count ->
        [one] { $count } dag gammel
       *[other] { $count } dager gammel
    }
signatures-definitions-missing = Ingen lesbar virusdatabase ble funnet
signatures-directory-unreadable = Ukjent. { $reason }
signatures-scanner-error = freshclam mislyktes med avslutningskode { $code }.
signatures-killed-signal = freshclam ble stoppet av signal { $signal }.
signatures-unrecognized-wait-status = freshclam avsluttet på en måte som { -brand-name } ikke gjenkjenner.
signatures-wait-failed = { -brand-name } mistet oversikten over freshclam før det var ferdig.
signatures-authorization-refused = Autorisasjonen mislyktes. Prøv igjen og skriv inn passordet ditt.
signatures-systemd-not-reached = Systemd kunne ikke nås for å kjøre oppdateringen.
signatures-service-job-ended = Oppdateringstjenesten ble ikke fullført; systemd rapporterte: { $result }
signatures-update-rate-limited-mirror = Oppdateringen ble begrenset av speilet. Vent noen minutter og prøv igjen.
signatures-update-blocked-cdn = Oppdateringen ble blokkert av CDN. Dette kan skyldes hastighetsbegrensning. Vent og prøv igjen senere.
signatures-mirror-unavailable = ClamAV-speilet er utilgjengelig for øyeblikket. Prøv igjen senere.
signatures-certificate-rejected = SSL/TLS-sertifikatfeil. Speilet kan ha problemer med oppsettet.
signatures-connection-timed-out = Tidsavbrudd for tilkoblingen. Sjekk nettverkstilkoblingen.
signatures-database-locked = Databasen er låst. En annen freshclam-instans kjører kanskje.
signatures-permission-denied = Tilgang nektet. Du trenger kanskje utvidede rettigheter for å oppdatere databasen.
signatures-connection-failed = Tilkoblingsfeil. Sjekk nettverkstilkoblingen.
signatures-name-resolution-failed = DNS-oppslag mislyktes. Sjekk nettverksinnstillingene.
signatures-update-failed-unknown-error = Oppdateringen mislyktes med en ukjent feil. Se loggene for detaljer.
signatures-updated-databases = Oppdatert: { $databases }
signatures-already-current-databases = Allerede oppdatert: { $databases }
signatures-cooldown-unknown = Begrenset: { $database }
signatures-cooldown-until = Begrenset: { $database } til { $cooldown }
signatures-definitions-updated = Databaseoppdatering fullført
signatures-already-current = Databasen er allerede oppdatert
signatures-service-ran-without-verdict = Oppdateringstjenesten kjørte, men loggen sier ikke om definisjonene ble endret.
signatures-update-requested-daemon = Tjenesten aksepterte signalet; den utfører oppdateringen asynkront. Sjekk loggene for fullføring.
signatures-update-cancelled = Oppdatering av databasen avbrutt
signatures-scanner-program-missing = Installer freshclam før du ber om databaseoppdateringer.
signatures-provider-needs-credential-fangfrisch =
    { $credential ->
        [purchase-receipt] { $provider } krever et kjøpskvitteringsnummer, og fangfrisch leser det bare fra innstillingene sine. På NixOS kopieres services.clamav.fangfrisch.settings til Nix-lageret, som alle brukere av denne datamaskinen kan lese, så kjøpskvitteringsnummeret ville blitt eksponert. La { $provider } være av, eller kjør fangfrisch med en innstillingsfil som holdes utenfor Nix-lageret, for eksempel en agenix- eller sops-nix-hemmelighet.
       *[customer-identifier] { $provider } krever en kunde-ID, og fangfrisch leser den bare fra innstillingene sine. På NixOS kopieres services.clamav.fangfrisch.settings til Nix-lageret, som alle brukere av denne datamaskinen kan lese, så kunde-ID-en ville blitt eksponert. La { $provider } være av, eller kjør fangfrisch med en innstillingsfil som holdes utenfor Nix-lageret, for eksempel en agenix- eller sops-nix-hemmelighet.
    }
signatures-provider-needs-credential-website =
    { $credential ->
        [purchase-receipt] { $provider } krever et kjøpskvitteringsnummer fra nettstedet sitt. fangfrisch leser det fra innstillingsfilen sin, vanligvis { $path }. Legg til disse linjene der, erstatt plassholderen med kjøpskvitteringsnummeret ditt, og gjør filen lesbar bare for root, slik at andre brukere ikke kan se det.
       *[customer-identifier] { $provider } krever en kunde-ID fra nettstedet sitt. fangfrisch leser den fra innstillingsfilen sin, vanligvis { $path }. Legg til disse linjene der, erstatt plassholderen med kunde-ID-en din, og gjør filen lesbar bare for root, slik at andre brukere ikke kan se den.
    }
signatures-not-use = Ikke i bruk
signatures-add-lines-fangfrisch-settings = Legg til disse linjene i innstillingsfilen til fangfrisch, vanligvis { $path }.
signatures-provider-needs-free-abuse = { $provider } krever en gratis abuse.ch Auth-Key. Legg til disse linjene i innstillingsfilen til fangfrisch, vanligvis { $path }, erstatt plassholderen med nøkkelen din, og gjør filen lesbar bare for root, slik at andre brukere ikke kan se den.
signatures-use-label = I bruk
signatures-unknown-reason-source = Ukjent. { $reason }
signatures-check-again-signature-tooltip = Sjekk på nytt
signatures-checking-signature-tooltip = Kontrollerer
signatures-clamav-malware-signatures-description = Hvor ClamAVs skadevaresignaturer kommer fra, og hvilke andre kilder du kan legge til.
signatures-sources-title = Signaturkilder
signatures-abuse-ch-key-title = abuse.ch-nøkkel
signatures-abuse-ch-auth-title = abuse.ch Auth-Key
signatures-abuse-ch-key-uses = Brukes til URLhaus-signaturer og til oppslag og opplastinger hos MalwareBazaar.
signatures-abuse-ch-no-key = Ingen nøkkel
signatures-abuse-ch-key-unreadable = Nøkkelen kan ikke leses
signatures-command-copied-clipboard-signature = Kommandoen er kopiert til utklippstavlen
signatures-not-open-documentation-title = Kunne ikke åpne dokumentasjonen: { $error }
signatures-checking-api-key = Sjekker API-nøkkelen …
signatures-check-passed = Bestått
signatures-needs-review = Advarsel
signatures-check-failed = Mislykket
signatures-status-unknown = Ukjent
signatures-check-skipped = Hoppet over
signatures-check-progress = Kontrollerer
signatures-copy-tooltip = Kopier
signatures-learn-more-title = Lær mer
signatures-open-browser-tooltip = Åpne i nettleseren
signatures-checking-title = Sjekker …

preferences-launcher-not-found = { -brand-name } fant ikke sin egen programstarter: { $error }
preferences-action-unwritable = Handlingen for filbehandleren kunne ikke legges til: { $error }
preferences-action-unremovable = Handlingen for filbehandleren kunne ikke fjernes: { $error }
preferences-dialog-title = Brukervalg
preferences-general-title = Generelt
preferences-window-behavior-title = Vindusoppførsel
preferences-detection-alerts-work-description = Varsler om oppdagelser fungerer bare mens { -brand-name } kjører.
preferences-closing-window-title = Når vinduet lukkes
preferences-start-background-title = Start i bakgrunnen
preferences-launch-background-without-subtitle = Start i bakgrunnen uten å åpne et vindu.
preferences-desktop-notifications-title = Skrivebordsvarsler
preferences-notify-detections-finished-subtitle = Varsle om oppdagelser, og om fullførte skanninger og oppdateringer mens vinduet ikke er i fokus.
preferences-file-manager-integration-title = Integrasjon med filbehandleren
preferences-add-context-menu-description = Legg til kontekstmenyhandlinger for støttede filbehandlere.
preferences-online-services-title = Nettjenester
preferences-services-tell-more-description = Tjenester som forteller deg mer om en oppdaget fil eller en fil i karantene. Oppslag kjøres bare når du trykker Slå opp på en oppdagelse eller en karanteneoppføring, og sender bare en hash eller et signaturnavn. En fil forlater denne datamaskinen bare når du sender den inn og bekrefter hva som sendes.
preferences-lookups-send-file-description = Oppslag sender bare filens SHA-256-hash og forteller om den tilhører en kjent, ufarlig fil. Ingen nøkkel trengs.
preferences-no-lookups-report-description = Ingen oppslag. En rapport du bekrefter, sender filen, deteksjonsnavnet ved en falsk positiv, og navnet og e-postadressen din. Talos behandler prøver automatisk, gjør dem aldri offentlige og deler dem ikke med andre selskaper. En vurdering tar minst 48 timer. Ingen nøkkel trengs.
preferences-lookups-send-virustotal-file = Oppslag sender VirusTotal bare filens SHA-256-hash. En opplasting du bekrefter, sender selve filen i karantene, som VirusTotal deler med sine betalende kunder. Opplastinger hoppes over når VirusTotal allerede har en rapport, og avvises over 32 MB.
preferences-malwarebazaar-lookups-send-detection = Oppslag i MalwareBazaar sender signaturnavnet til en oppdagelse eller filens SHA-256-hash. En opplasting du bekrefter, sender filen i karantene med ClamAV-signaturen som merkelapp, og prøven blir offentlig. Den samme abuse.ch-nøkkelen låser også opp nedlasting av URLhaus-signaturer gjennom fangfrisch; på NixOS gjenbruker { -brand-name } nøkkelfilen til fangfrisch når brukeren din kan lese den. Ifølge vilkårene for rimelig bruk hos abuse.ch kan kommersielle brukere trenge det kommersielle API-et.
preferences-virustotal-service-hint = Gratisnivå: 4 forespørsler per minutt, 500 per dag.
preferences-abuse-ch-service-hint = Gratis fra abuse.ch Authentication Portal.
preferences-not-configured = Ikke satt opp
preferences-checking-label = Sjekker …
preferences-set-system-configuration-title = Satt av systemkonfigurasjonen din
preferences-key-keyring-title = Nøkkel i nøkkelringen din
preferences-key-file-title = Nøkkelfil
preferences-key-unreadable-status = Kan ikke leses
preferences-key-file-not-permitted = Kan ikke lese nøkkelfilen: ingen tilgang
preferences-key-file-missing = Nøkkelfilen finnes ikke
preferences-key-file-unreadable = Kan ikke lese nøkkelfilen
preferences-key-not-valid = Ikke en gyldig nøkkel for denne tjenesten
preferences-keyring-unavailable = Nøkkelringen er utilgjengelig
preferences-stored-key-unreadable = Kan ikke lese nøkkelen i nøkkelringen din
preferences-delete-key-label = Slett nøkkel
preferences-paste-api-key-title = Lim inn API-nøkkel
preferences-get-free-api-title = Skaff en gratis API-nøkkel

widgets-clean-label = Ren
widgets-scan-infected = Infisert
widgets-error-status = Feil
widgets-cancelled-label = Kansellert
widgets-skipped-label = Hoppet over
widgets-success-label = Suksess
widgets-up-date = Oppdatert
widgets-requested-label = Forespurt
widgets-ran-label = Kjørt
widgets-counted-threats =
    { $count ->
        [one] Fant { $count } trussel
       *[other] Fant { $count } trusler
    }
widgets-uncounted-threats = Skanneren rapporterte en trussel uten å oppgi filen.
widgets-no-threats-found-activity = Ingen trusler funnet
widgets-scan-failed = Skanningen ble ikke fullført feilfritt
widgets-scan-cancelled-activity = Skanning avbrutt av brukeren
widgets-scheduled-scan-skipped-battery = Planlagt skanning ble hoppet over (går på batteri)
widgets-virus-definitions-updated = Virusdefinisjonene er oppdatert
widgets-virus-definitions-up-date = Virusdefinisjonene er allerede oppdatert
widgets-update-requested-service = Oppdatering forespurt fra tjenesten
widgets-update-service-no-result = Oppdateringstjenesten kjørte uten å rapportere et resultat
widgets-update-failed = Databaseoppdatering mislyktes
widgets-update-cancelled = Oppdatering av databasen avbrutt
widgets-scan-log = SKANNELOGG
widgets-update-log = OPPDATERINGSLOGG
widgets-timestamp-label = Tidsstempel
widgets-status-label = Status
widgets-duration-label = Varighet
widgets-summary-label = Sammendrag
widgets-path-label = Sti
widgets-full-output = Fullstendig utdata
widgets-scan-history-cannot-title = Skannehistorikken kan ikke leses
widgets-retry-label = Prøv igjen
widgets-copy-activity-tooltip = Kopier
widgets-log-details-copied-toast = Loggdetaljene er kopiert til utklippstavlen
widgets-not-save-file-error = Kunne ikke lagre filen: { $error }
widgets-not-choose-file-error = Kunne ikke velge filen: { $error }
widgets-chosen-file-not-local = { $location } er ikke en fil på denne datamaskinen
widgets-export-title = Eksporter
widgets-database-newer-release = Denne databasen ble laget av en nyere { -brand-name }: den har oppsettsversjon { $found }, og denne versjonen kjenner til og med { $known }. Oppdater { -brand-name } for å åpne den.
widgets-migration-step-failed = Oppdateringen av databaseoppsettet mislyktes ved steg { $step }: { $error }
widgets-version-check-failed = Versjonen av databaseoppsettet kunne ikke sjekkes: { $error }
widgets-copy-configuration-line-title = Kopier konfigurasjonslinje
widgets-nothing-applied-choose-description = Ingenting tas i bruk. Velg en verdi og kopier linjen, og legg den så til i konfigurasjonen din selv.
widgets-unit-title = Enhet
widgets-line-copy-title = Linje som skal kopieres
widgets-copy-line-label = Kopier linje
widgets-unlock-required = Lås opp for å endre denne innstillingen. Mens den er låst, kan du kopiere linjen for konfigurasjonen din.
widgets-administrator-not-copy = Administratoren din tillater ikke at denne innstillingen endres fra { -brand-name }. Du kan kopiere linjen for konfigurasjonen din.
widgets-plain-bytes = byte
widgets-kibibytes-unit = KiB
widgets-mebibytes-unit = MiB
widgets-unknown-option = { -brand-name } kjenner ikke dette alternativet, så det bare viser det.
widgets-not-yes-no = Trenger vurdering: denne verdien skal være yes eller no.
widgets-not-whole-number = Trenger vurdering: denne verdien skal være et heltall.
widgets-count-out-range = Trenger vurdering: denne verdien skal være mellom { $minimum } og { $maximum }.
widgets-not-size = Trenger vurdering: denne verdien skal være en størrelse i byte, eller et tall etterfulgt av K eller M.
widgets-path-not-absolute = Trenger vurdering: denne verdien skal være en full sti som starter med /.
widgets-empty-text = Trenger vurdering: denne verdien er tom.
widgets-apply-tooltip = Bruk
widgets-unit-tooltip = Enhet
widgets-setting-runs-commands-changes = Denne innstillingen kjører kommandoer eller endrer hvilken bruker ClamAV kjører som, så { -brand-name } tilbyr aldri å endre den
widgets-check-value-configuration-file = Sjekk denne verdien i konfigurasjonsfilen.
widgets-copy-configuration-line-tooltip = Kopier konfigurasjonslinje …
widgets-applying-restarting-clamav-tooltip = Tar i bruk og starter ClamAV på nytt
widgets-detect-potentially-unwanted-programs = Oppdag potensielt uønskede programmer
widgets-heuristic-detections = Heuristiske oppdagelser
widgets-flag-office-files-macros = Merk Office-filer med makroer
widgets-flag-encrypted-archives-documents = Merk krypterte arkiver og dokumenter
widgets-look-inside-archives = Se inni arkiver
widgets-flag-files-too-large = Merk filer som er for store til å sjekkes fullt ut
widgets-adware-hacking-tools-remote = Reklamevare, hackerverktøy og verktøy for fjerntilgang. Regn med flere falske positiver.
widgets-suspicious-files-no-signature = Mistenkelige filer som ingen signatur angir, for eksempel ødelagte kjørbare filer og phishingtriks.
widgets-macros-common-way-deliver = Makroer er en vanlig måte å levere skadevare på, men mange legitime dokumenter bruker dem.
widgets-clamav-cannot-look-inside = ClamAV kan ikke se inni dem, så de kan skjule skadevare.
widgets-checks-files-inside-zip = Sjekker filene inni zip-, rar- og andre arkiver.
widgets-otherwise-files-above-size = Ellers slipper filer over størrelses- og rekursjonsgrensene gjennom usjekket uten varsel.
widgets-locked-until-unlocked = Lås opp for å endre denne innstillingen.
widgets-administrator-not-allow-detection = Administratoren din tillater ikke at denne innstillingen endres fra { -brand-name }.
widgets-clamd-settings = clamd-innstillinger
widgets-access-scanning-settings = Innstillinger for skanning ved tilgang
widgets-update-settings = Oppdateringsinnstillinger
widgets-change-snippet-copied = Kopiert. Legg den til i konfigurasjonen din for å ta den i bruk.
widgets-stale-confirmation-dropped = Endringen ble ikke gjort: innstillingene ble låst igjen, eller en annen endring startet først.
widgets-settings-applied = Tatt i bruk, men ikke ennå i systemkonfigurasjonen din.
widgets-previous-restored = ClamAV kom ikke tilbake i god stand med den verdien, så de forrige innstillingene ble gjenopprettet.
widgets-change-not-made-error = Endringen ble ikke gjort: { $error }
widgets-authorization-refused = Ingenting ble endret uten passordet ditt
widgets-admin-service-missing = Denne installasjonen har ingen administrasjonstjeneste for ClamAV, så innstillingene til ClamAV kan ikke endres fra { -brand-name }
widgets-live-tunable-config = { $path } kombinerer systemkonfigurasjonen din med endringene som gjøres her. Mens innstillingene er låst, er de skrivebeskyttet: bruk Kopier konfigurasjonslinje for å kopiere en til konfigurasjonen din. Etter opplåsing endres de tillatte innstillingene direkte.
widgets-change-setting = Vil du endre denne innstillingen?
widgets-cancel-engine = Avbryt
widgets-reduce-protection = Reduser beskyttelsen
widgets-apply-label = Bruk
widgets-key-value-until-time = { $key } { $value }, til { $time }
widgets-live-tuning-not-installed = Direktejustering er ikke installert. På NixOS slår du på services.lazaret.liveTuning; på Debian og Ubuntu installerer du { -brand-name }-pakken, som inneholder administrasjonstjenesten for ClamAV.
widgets-admin-service-unreachable = Innstillinger kan ikke endres her akkurat nå: administrasjonstjenesten for ClamAV i { -brand-name } kunne ikke nås.
widgets-loading-config = Leser konfigurasjonen …
widgets-config-missing = Finner ikke konfigurasjonsfilen
widgets-lock-label = Lås
widgets-unlock-label = Lås opp
widgets-change-settings-title = Endre innstillinger
widgets-unlock-change-settings-description = Lås opp for å endre innstillingene systemet ditt tillater under kjøring. Opplåsing gir ingen rettigheter i seg selv: hver endring viser hva som vil skje, og ber om administratorautorisasjon. Den låses igjen etter to minutter.
widgets-changes-system-configuration-title = Endringer som systemkonfigurasjonen din ikke kjenner til
widgets-copy-engine-tooltip = Kopier
widgets-reduced-protection-ends-title = Redusert beskyttelse som opphører av seg selv
widgets-reset-system-configuration-title = Tilbakestill til systemkonfigurasjonen
widgets-detection-title = Deteksjon
widgets-clamav-reports-every-description = Hva ClamAV rapporterer i hver skanning på denne datamaskinen. { -brand-name } sender de samme valgene til clamscan, slik at begge skannerne er enige.
widgets-no-settings-section-title = Ingen innstillinger i denne delen
widgets-journal-read-failed = Systemjournalen kunne ikke leses: { $error }
widgets-journal-reading-incomplete = Lesingen av systemjournalen stoppet før den var ferdig
widgets-folder-unreadable = Signaturmappen kunne ikke leses: { $error }
widgets-reading-signature-folder-stopped = Lesingen av signaturmappen stoppet før den var ferdig
widgets-entry-timestamp-invalid = En journaloppføring har et tidsstempel utenfor det støttede området.
widgets-reader-thread-failed = Journalleseren kunne ikke starte: { $error }
widgets-journal-reading-stopped = Lesingen av systemjournalen stoppet. { $reason }
widgets-file-hash-mismatch = Filhash samsvarer ikke — filen kan være ødelagt
widgets-restore-refused-integrity-unknown = Denne karanteneoppføringen har ingen brukbar hash, så filen kan ikke verifiseres og blir ikke gjenopprettet. Du kan fortsatt slette den.
widgets-upload-refused-integrity-unknown = Denne karanteneoppføringen har ingen brukbar hash, så { -brand-name } kan ikke bevise at filen er den som ble satt i karantene, og sender den ikke noe sted.
widgets-payload-too-large-send = Filen er større enn { $size }, som er det meste { -brand-name } sender.
widgets-blocked-file-other-filesystem = Skanning ved tilgang blokkerer denne filen, og den ligger på et annet filsystem, så den kan ikke flyttes til karantene. Du kan slette den i stedet.
widgets-entry-not-found = Fant ikke karanteneoppføringen: { $id }
widgets-recording-failed-file-restored = Kunne ikke registrere karanteneoppføringen i databasen. Filen er gjenopprettet til den opprinnelige plasseringen.
widgets-recording-failed-rollback = Kunne ikke registrere karanteneoppføringen i databasen. Tilbakestillingen mislyktes også: { $error }. Filen kan ligge igjen uten oppføring på: { $path }
widgets-entry-record-not-removed = Filen ble flyttet, men karanteneoppføringen kunne ikke fjernes. Den kan fortsatt vises i listen.
widgets-system-call-failed = Et systemkall mislyktes: { $error }
widgets-file-operation-failed = En filoperasjon mislyktes: { $error }
widgets-database-operation-failed = Operasjonen i karantenedatabasen mislyktes: { $error }
widgets-blocking-step-interrupted = Et karantenesteg i bakgrunnen stoppet uventet: { $error }
widgets-path-contains-nul = Stien inneholder en NUL-byte
widgets-path-has-no-parent = Stien har ingen overordnet mappe
widgets-path-has-no-file = Stien har ikke noe filnavn
widgets-database-path-not-utf8 = Stien til karantenedatabasen er ikke gyldig UTF-8
widgets-database-not-regular-file = Karantenedatabasen er ikke en vanlig fil
widgets-source-blocked-access = Skanning ved tilgang blokkerer lesing av filen
widgets-moved-payload-still-blocked = Den blokkerte filen kan fortsatt ikke leses etter at den ble flyttet til karantene
widgets-source-not-regular-file = Filen er ikke en vanlig fil
widgets-payload-not-regular-file = Filen i karantene er ikke en vanlig fil
widgets-not-enough-free-space = Det er ikke nok ledig plass til å sette filen i karantene
widgets-file-replaced-during-operation = Filen ble erstattet mens den ble flyttet, så den ble ikke fjernet
widgets-path-outside-quarantine = Stien er ikke inne i karantenemappen
widgets-invalid-restore-path = Gjenopprettingsstien er ugyldig
widgets-protected-restore-destination = Gjenoppretting til en beskyttet systemmappe er ikke tillatt
widgets-stored-path-unreadable = En sti lagret i karantenedatabasen kunne ikke leses
widgets-target-missing = Skannemål eksisterer ikke eller inneholder en ugyldig NUL-byte
widgets-target-not-scannable = Skannemål er ikke en vanlig fil eller mappe
widgets-target-directory-unreadable = Kan ikke liste opp skannemålet { $path }: { $error }
widgets-file-list-unwritable = Kan ikke skrive privat filliste for clamd: { $error }
widgets-enumeration-interrupted = Oppføringen av skannemålet stoppet uventet.
widgets-no-files-scanned = Ingen filer kunne skannes
widgets-unscanned-files =
    { $count ->
        [one] { $count } fil ble ikke skannet
       *[other] { $count } filer ble ikke skannet
    }
widgets-clamd-connection-lost = Tilkoblingen til clamd ble brutt under skanningen.
widgets-scanner-stopped-reporting = Skanneren rapporterte ikke noe resultat for disse filene.
widgets-count-not-accessed-title =
    { $count ->
        [one] { $count } fil var utilgjengelig
       *[other] { $count } filer var utilgjengelige
    }
widgets-files-unreadable =
    { $count ->
        [one] { $count } fil kunne ikke leses
       *[other] { $count } filer kunne ikke leses
    }
widgets-nonfatal-warnings =
    { $count ->
        [one] { $count } ikke-dødelig advarsel under skanning; noen filer kan bare ha blitt skannet delvis
       *[other] { $count } ikke-dødelige advarsler under skanning; noen filer kan bare ha blitt skannet delvis
    }
widgets-count-threat-found-outcome =
    { $count ->
        [one] { $count } trussel funnet
       *[other] { $count } trusler funnet
    }
widgets-count-file-scanned-outcome =
    { $count ->
        [one] { $count } fil skannet
       *[other] { $count } filer skannet
    }
widgets-clamav-not-open-files = ClamAV kunne ikke åpne disse filene.
widgets-unattributed-threats-only = Skanneren rapporterte en trussel uten å oppgi filen.
widgets-preparing-scan = Forbereder …
widgets-finding-files =
    { $count ->
        [one] Finner filer … { $count } funnet
       *[other] Finner filer … { $count } funnet
    }
widgets-trust-file = Klarer denne filen …
widgets-ignore-detection-for-me = Ignorer denne oppdagelsen for meg …
widgets-report-false-positive = Rapporter falsk positiv …
widgets-skip-path-lazaret-scans = Hopp over denne stien i { -brand-name }-skanninger
widgets-copy-path = Kopier sti
widgets-critical-severity = Kritisk
widgets-high-severity = Høy
widgets-medium-severity = Medium
widgets-low-severity = Lav
widgets-moved-quarantine-path-check = Flyttet til karantene: { $path }. Sjekk omdømmet på siden Karantene.
widgets-quarantine-detected-label = Sett i karantene
widgets-more-actions-tooltip = Flere handlinger
widgets-count-more-file-title =
    { $count ->
        [one] { $count } fil til, oppført i den eksporterte rapporten
       *[other] { $count } filer til, oppført i den eksporterte rapporten
    }
widgets-scan-details-title = Skannedetaljer
widgets-quarantine-all =
    { $count ->
        [one] Sett { $count } trussel i karantene
       *[other] Sett { $count } trusler i karantene
    }
widgets-move-quarantine = Vil du flytte til karantene?
widgets-removes-each-original-file = Dette fjerner hver originalfil fra sin nåværende plassering.
widgets-cancel-scan = Avbryt
widgets-quarantine-label = Sett i karantene
widgets-detected-threats-title = Oppdagede trusler
widgets-problems-title = Problemer
widgets-move-path-quarantine = Flytte '{ $path }' til karantene?
widgets-move-count-detected-file =
    { $count ->
        [one] Flytte { $count } oppdaget fil til karantene?
       *[other] Flytte { $count } oppdagede filer til karantene?
    }
widgets-path-copied-clipboard = Stien er kopiert til utklippstavlen
widgets-file-skipped-future-scans = Denne filen hoppes over i framtidige skanninger
widgets-exclusion-already-exists = Denne ekskluderingen eksisterer allerede
widgets-results-copied-clipboard = Resultatene er kopiert til utklippstavlen
widgets-results-exported = Resultatene er eksportert
widgets-done-total-file-scanned =
    { $total ->
        [one] { NUMBER($done) } av { $total } fil skannet
       *[other] { NUMBER($done) } av { $total } filer skannet
    }
widgets-count-file-scanned-results =
    { $count ->
        [one] { $count } fil skannet
       *[other] { $count } filer skannet
    }
widgets-count-entry-skipped-exclusions =
    { $count ->
        [one] { $count } oppføring hoppet over av unntak
       *[other] { $count } oppføringer hoppet over av unntak
    }
widgets-threats-detected-status = Trusler oppdaget
widgets-no-threats-detected-status = Ingen trusler funnet
widgets-finished-problems-status = Skanningen ble ikke fullført feilfritt
widgets-scan-cancelled-status = Skanning avbrutt av brukeren
widgets-count-threat-found-results =
    { $count ->
        [one] { $count } trussel funnet
       *[other] { $count } trusler funnet
    }
widgets-files-scanned = Filer skannet
widgets-files-planned = Planlagte filer
widgets-directories-label = Mapper
widgets-threats-found = Trusler funnet
widgets-file-no-longer-quarantine = Filen finnes ikke lenger. Hvis den er i karantene, klarer du den fra siden Karantene.
widgets-permission-denied = Filen kan ikke leses, muligens fordi skanning ved tilgang blokkerer den. Flytt den til karantene, og klarer den deretter fra siden Karantene.
widgets-file-not-read-scan = Filen kunne ikke leses: { $error }
widgets-threats-detected-finished = Trusler oppdaget
widgets-no-threats-detected-finished = Ingen trusler funnet
widgets-finished-problems-notify = Skanningen ble ikke fullført feilfritt
widgets-scanner-error = Skanneren stoppet med feilkode { $code }.
widgets-killed-signal = Skanneren ble stoppet av signal { $signal }.
widgets-unrecognized-wait-status = Skanneren avsluttet på en måte som { -brand-name } ikke gjenkjenner (ventestatus { $status }).
widgets-wait-failed = { -brand-name } mistet oversikten over skanneprosessen: { $error }.
widgets-scanner-program-missing = ClamAV-skanneren er ikke installert eller ligger ikke i søkestien.
widgets-scanner-program-replaceable = { -brand-name } nektet å kjøre { $program }, fordi kontoen din kan erstatte { $replaceable }. Installer ClamAV for hele systemet, slik at et program plantet av skadevare ikke kan forfalske rene resultater.
widgets-scanner-spawn-failed = Skanneren kunne ikke startes: { $error }
widgets-scanner-work-directory-unavailable = Kan ikke opprette den private arbeidsmappen for skanneren: { $error }
widgets-scanner-output-uncaptured = Utdataene fra skanneren kunne ikke fanges opp.
widgets-detection-settings-unreadable = Skanningen startet ikke, fordi deteksjonsinnstillingene til clamd ikke kunne leses: { $error }
widgets-program-lookup-failed = Skanningen startet ikke, fordi skanneprogrammene ikke kunne slås opp: { $error }
widgets-scan-input-unavailable = Kan ikke åpne skanneinndata { $path } på en trygg måte: { $error }
widgets-detections-warnings-errors-tooltip = Bare oppdagelser, advarsler og feil
widgets-copy-service-tooltip = Kopier
widgets-export-text-tooltip = Eksporter tekst
widgets-search-log-placeholder = Søk i loggen
widgets-reading-log = Leser loggen …
widgets-no-log-lines-match = Ingen logglinjer samsvarer
widgets-log-copied-clipboard = Loggen er kopiert til utklippstavlen
widgets-text-export-finished = Eksportert.
widgets-threat-detected = Oppdagelse
widgets-emergency-label = Nødsituasjon
widgets-alert-label = Alarm
widgets-critical-level = Kritisk
widgets-error-level = Feil
widgets-warning-label = Advarsel
widgets-notice-label = Merknad
widgets-info-label = Info
widgets-debug-label = Feilsøking
widgets-log-reading = Sjekker …
widgets-system-journal = Systemjournal
widgets-neither-log-file-nor = Fant verken en loggfil eller journallinjer.
widgets-no-log-file-found = Fant ingen loggfil. { $reason }
widgets-program-use = { $program } i bruk
widgets-ignored-program = Ignorerte { $program }
widgets-refused-program = Avviste { $program }
widgets-scanner-programs-trusted = Skanneprogrammene kan klareres
widgets-freshclam-trusted = freshclam kan klareres
widgets-malware-running-clamdscan = Skadevare som kjører som deg, kan legge en falsk clamdscan tidligere i søkestien som melder hver fil som ren. { -brand-name } kjører bare ClamAV-programmer som kontoen din ikke kan erstatte, for eksempel de som er installert for hele systemet.
widgets-malware-running-freshclam = Skadevare som kjører som deg, kan legge en falsk freshclam tidligere i søkestien som later som den henter nye signaturer. { -brand-name } kjører bare ClamAV-programmer som kontoen din ikke kan erstatte, for eksempel de som er installert for hele systemet.
widgets-check-interrupted = Ukjent, fordi kontrollen av ClamAV-programmene stoppet før den var ferdig
widgets-clamav-program-replaced-account = Et ClamAV-program kan erstattes av kontoen din, så { -brand-name } nekter å kjøre det
widgets-clamav-program-account-shadows = Et ClamAV-program i kontoen din skygger for systemkopien. { -brand-name } ignorerer det, men sjekk hvor det kom fra
widgets-count-settings-reduced-since =
    { $count ->
        [one] { $count } innstilling sjekker mindre enn konfigurert siden { $time }
       *[other] { $count } innstillinger sjekker mindre enn konfigurert siden { $time }
    }
widgets-count-settings-reduced-unknown-time =
    { $count ->
        [one] { $count } innstilling sjekker mindre enn konfigurert, siden et tidspunkt som ikke kunne leses
       *[other] { $count } innstillinger sjekker mindre enn konfigurert, siden et tidspunkt som ikke kunne leses
    }
widgets-changed-label = Endret
widgets-change-time-unknown-override = Ukjent
widgets-reduction-unknown-unread = Ukjent om innstillinger ble redusert, fordi kjøretidsendringene ikke kunne leses: { $error }
widgets-reduction-unknown-file = Ukjent om innstillinger ble redusert, fordi filen med kjøretidsendringer ikke har det forventede formatet. ClamAV kjører med systemkonfigurasjonen til den er rettet.
widgets-reduction-unknown-reading = Ukjent om innstillinger ble redusert, fordi lesingen av kjøretidsendringene stoppet før den var ferdig
widgets-settings-changed-runtime = Innstillinger endret under kjøring
widgets-administrator-can-change = Noen med administratorpassordet kan endre ClamAV-innstillinger under kjøring. { -brand-name } kunne ikke lese disse endringene, så det kan ikke si om de sjekker mindre enn konfigurasjonen din.
widgets-protection-reduced-purpose = Beskyttelse redusert med vilje
widgets-administrator-changed-runtime = Noen med administratorpassordet har endret ClamAV-innstillinger under kjøring slik at de sjekker mindre enn konfigurasjonen din. Systemendringene på siden som eier innstillingen, viser hvem og når.
widgets-base-configuration-changed-now = { $base } i konfigurasjonen din, { $changed } nå
widgets-administrator-changed-clamd = Noen med administratorpassordet har endret ClamAV-innstillinger under kjøring. clamd.conf kunne ikke leses, så { -brand-name } kan ikke si om disse endringene sjekker mindre enn konfigurasjonen din. Systemendringene på siden som eier innstillingen, viser hvem og når.
widgets-settings-changed-runtime-clamd = Innstillinger ble endret under kjøring, men clamd.conf kunne ikke leses for å sammenligne dem
widgets-changed-now = { $changed } nå
widgets-daemon-awaited = Krever tjenesten, se Skannemotor
widgets-unknown-run = Ukjent. { $reason }
widgets-copy-timeline-tooltip = Kopier
widgets-moment-details = { $moment } · { $details }
widgets-why-matters-tooltip = Hvorfor dette er viktig
widgets-what-is-sent-label = Dette sendes:
widgets-clamav-audience = Cisco Talos, teamet bak ClamAV, vurderer den. Prøver behandles automatisk, bare Talos har tilgang til dem, og de gjøres aldri offentlige eller deles med andre selskaper. En vurdering tar minst 48 timer, og utfallet vises i en senere versjon av signaturdatabasen.
widgets-report-false-positive-clamav = Vil du rapportere en falsk positiv til ClamAV?
widgets-file-content = Innholdet i filen
widgets-detection-name = Deteksjonsnavnet { $detection }
widgets-report-missed-malware-clamav = Vil du rapportere oversett skadevare til ClamAV?
widgets-name-email-address-talos = Navnet og e-postadressen din, slik at Talos kan kontakte deg
widgets-lazaret-sends-file-clamav = { -brand-name } sender denne filen til ClamAV med clamsubmit. Send bare filer som ikke inneholder noe privat.
widgets-send-report-button = Send rapport
widgets-file-content-named-sha = Innholdet i filen, navngitt etter SHA-256-hashen, ikke etter navnet
widgets-upload-malwarebazaar-heading = Vil du laste opp til MalwareBazaar?
widgets-lazaret-uploads-file-malwarebazaar = { -brand-name } laster opp denne filen til MalwareBazaar, som drives av abuse.ch.
widgets-tag-clamav-detection = Merkelappen { $tag }, fra ClamAVs deteksjon
widgets-abuse-ch-key-sample = abuse.ch-nøkkelen din; prøven oppføres uten kontonavnet ditt
widgets-malwarebazaar-public-anyone-download = MalwareBazaar er offentlig: alle kan laste ned prøven når den er lastet opp, og en opplasting kan ikke trekkes tilbake. Last bare opp skadevare, aldri et dokument som kan inneholde personopplysninger.
widgets-upload-publicly-button = Last opp offentlig
widgets-upload-virustotal-heading = Vil du laste opp til VirusTotal?
widgets-lazaret-uploads-file-virustotal = { -brand-name } laster opp denne filen til VirusTotal for analyse.
widgets-virustotal-key-links-upload = VirusTotal-nøkkelen din, som knytter opplastingen til kontoen din
widgets-virustotal-shares-uploaded-files = VirusTotal deler opplastede filer med sine betalende kunder, som kan laste dem ned, og hvem som helst kan slå opp resultatet etter filens hash. Last aldri opp et dokument som kan inneholde personopplysninger.
widgets-upload-button = Last opp
widgets-cancel-submission = Avbryt
widgets-name-title = Navnet ditt
widgets-email-address-title = E-postadressen din
widgets-file-unreadable = Filen kunne ikke leses: { $error }
widgets-file-not-regular = Bare vanlige filer kan sendes
widgets-file-too-large = Filen er større enn { $size }, som er det meste { -brand-name } sender.
widgets-reading-interrupted = Lesingen av filen stoppet uventet
widgets-history-failure = Innsendingshistorikken kunne ikke leses: { $error }
widgets-earlier-submission = Denne filen ble allerede sendt { $local-time }. Ingenting ble sendt på nytt.
widgets-report-recorded = Rapporten er sendt. ClamAVs vurdering tar minst 48 timer; Innsendte prøver viser når en signaturoppdatering endrer vurderingen.
widgets-report-not-sent-error = Rapporten ble ikke sendt: { $error }
widgets-report-sent-without-record = Rapporten er sendt, men den kunne ikke legges til i Innsendte prøver: { $error }
widgets-nothing-uploaded-error = Ingenting ble lastet opp. { $reason }
widgets-bazaar-key-not-usable = MalwareBazaar har ingen brukbar nøkkel.
widgets-virustotal-key-not-usable = VirusTotal har ingen brukbar nøkkel.
widgets-malwarebazaar-refused-sample-reason = MalwareBazaar avviste prøven: { $reason }
widgets-malwarebazaar-sample-sent-samples = MalwareBazaar har prøven. Innsendte prøver lenker til den offentlige oppføringen.
widgets-not-detected-clamav = MalwareBazaar tar bare imot filer som en ClamAV-signatur har oppdaget
widgets-unwanted-program-only = MalwareBazaar tar ikke imot potensielt uønskede programmer, bare skadevare
widgets-sent-without-record = Lastet opp, men den kunne ikke legges til i Innsendte prøver: { $error }
widgets-virustotal-refused-file-reason = VirusTotal avviste filen: { $reason }
widgets-virustotal-analysing-file-refresh = VirusTotal analyserer filen. Oppdater på siden Innsendte prøver viser vurderingen.
widgets-report-already-available =
    { $total ->
        [one] VirusTotal kjenner allerede denne filen: { NUMBER($count) } av { $total } motor oppdager den. Ingenting ble lastet opp.
       *[other] VirusTotal kjenner allerede denne filen: { NUMBER($count) } av { $total } motorer oppdager den. Ingenting ble lastet opp.
    }
widgets-known-user = { $local-time } · { $outcome } · forespurt av bruker { $uid }
widgets-unknown-user = { $time } · { $outcome } · forespurt av en ukjent bruker
widgets-change-lowered-protection-tooltip = Denne endringen senket beskyttelsen
widgets-system-changes-failed = Systemjournalen kan ikke leses: { $error }
widgets-system-changes-title = Systemendringer
widgets-refresh-tooltip = Oppdater
widgets-system-changes-cannot-title = Systemendringer kan ikke leses
widgets-loading-system-changes = Laster inn systemendringer …
widgets-trust-file-heading = Vil du klarere denne filen?
widgets-clamav-never-report-body =
    ClamAV vil aldri igjen rapportere en fil med nøyaktig dette innholdet, for noen bruker, uansett hvor den ligger på denne datamaskinen og hvilken skanning som finner den, og skanning ved tilgang slutter å blokkere den. Administratorpassordet ditt trengs, og passordspørsmålet viser den samme SHA-256-hashen som nedenfor.

    { $path }
widgets-stop-showing-file-body =
    { -brand-name } slutter å vise deg en fil med nøyaktig dette innholdet i skanningene du kjører, uansett hvor den ligger. ClamAV oppdager den likevel, så skanning ved tilgang kan fortsatt blokkere den og varsle deg, og andre brukere ser den fortsatt. Klarer for alle brukere hindrer ClamAV i å oppdage den for alle og krever administratorpassordet ditt.

    { $path }
widgets-trust-for-me-button = Klarer for meg
widgets-user-trust-unreadable = Filene og oppdagelsene du har klarert, kunne ikke leses, så denne skanningen viser alle oppdagelser: { $error }
widgets-trust-for-all-users-button = Klarer for alle brukere …
widgets-sha-256-hash-size =
    SHA-256 { $hash }
    { $size }
widgets-ignore-detection-heading = Vil du ignorere denne oppdagelsen?
widgets-stop-showing-detection-body = { -brand-name } slutter å vise deg alle filer med denne oppdagelsen, ikke bare denne. ClamAV selv og andre brukere ser dem fortsatt. Å klarere filen etter hashen er tryggere når bare én fil er en falsk positiv.
widgets-ignore-button = Ignorer
widgets-stop-trusting-file-heading = Vil du slutte å klarere denne filen?
widgets-clamav-report-file-body = ClamAV vil rapportere denne filen igjen til alle brukere når den samsvarer med en signatur.
widgets-show-file-again-body = { -brand-name } viser deg denne filen igjen når ClamAV oppdager den.
widgets-stop-trusting-button = Slutt å klarere
widgets-report-detection-again-heading = Vil du rapportere denne oppdagelsen igjen?
widgets-clamav-report-files-body = ClamAV vil rapportere filer med denne oppdagelsen igjen til alle brukere.
widgets-show-detection-again-body = { -brand-name } viser deg filer med denne oppdagelsen igjen.
widgets-report-again-button = Rapporter igjen
widgets-cancel-trust = Avbryt

host-action-applied = Tatt i bruk
host-action-rolled-back = Tilbakestilt
host-authorization-refused-described = Avvist uten passord
host-action-failed = Mislykket
host-outcome-not-recorded = Utfallet er ikke registrert
host-unrecognized-outcome = Ukjent utfall { $outcome }
host-blocking-turned-off = Oppdagede filer blir ikke lenger blokkert når de åpnes. De blir bare rapportert.
host-check-turned-off = ClamAV slutter å se etter denne typen fil eller trussel på denne datamaskinen.
host-watched-folder-removed = Filer i den fjernede mappen blir ikke lenger sjekket når de åpnes.
host-skipped-location-added = ClamAV hopper helt over den nye oppføringen, så skadevare der går ubemerket hen.
host-scan-limit-lowered = Filer eller arkiver over den nye grensen hoppes over uten å bli sjekket.
host-current-values-unknown = Den gjeldende verdien kunne ikke leses, så denne endringen kan redusere beskyttelsen.
host-detection-trusted = ClamAV vil ikke lenger rapportere det du klarerer her.
host-key-now-current-new =
    { $key }
    Nå: { $current }
    Ny: { $new }
host-system-records-change-reduced = Systemet registrerer at denne endringen reduserte beskyttelsen, og sidepanelet fortsetter å markere siden for den endrede innstillingen til du angrer den.
host-trust-applied = ClamAV lastet inn endringen
host-trust-recorded-for-you = { -brand-name } viser ikke lenger dette til deg. ClamAV selv, blokkering ved tilgang og andre brukere ser det fortsatt.
host-applied-reload-pending = Lagret. clamd svarte ikke, så den tar i bruk endringen neste gang den laster inn signaturene på nytt
host-authorization-refused-outcome = Ingenting ble endret uten passordet ditt
host-admin-service-missing = Denne installasjonen har ingen administrasjonstjeneste for ClamAV, så tillatelseslisten til ClamAV kan ikke endres fra { -brand-name }
host-trust-failed = Ingenting ble endret: { $error }
host-nixos-configuration = { $path } genereres av NixOS-konfigurasjonen din, så { -brand-name } kan ikke endre den. Bruk Kopier konfigurasjonslinje på en innstilling for å kopiere linjen til konfigurasjonen din, og bygg deretter på nytt.
host-system-package = { $path } tilhører systemet, så { -brand-name } kan ikke endre den. Bruk Kopier konfigurasjonslinje på en innstilling for å kopiere linjen som skal stå i den filen.
host-contact-incomplete = Skriv inn navnet ditt og en gyldig e-postadresse for å sende en rapport
host-submitter-missing = Rapportering krever clamsubmit, som følger med ClamAV, men ikke er installert her
host-submitter-replaceable = { -brand-name } nektet å kjøre clamsubmit, fordi kontoen din kan erstatte den
host-report-not-sent-error = Rapporten ble ikke sendt: { $error }
host-upload-timed-out = Rapporten ble ikke sendt: opplastingen tok for lang tid
host-unrecognized-source = Annen kilde
host-file-progress =
    { $total ->
        [one] { NUMBER($done) } av { $total } fil
       *[other] { NUMBER($done) } av { $total } filer
    }
host-count-second-left =
    { $count ->
        [one] Omtrent { $count } sekund igjen
       *[other] Omtrent { $count } sekunder igjen
    }
host-count-minute-left =
    { $count ->
        [one] Omtrent { $count } minutt igjen
       *[other] Omtrent { $count } minutter igjen
    }
host-count-hour-left =
    { $count ->
        [one] Omtrent { $count } time igjen
       *[other] Omtrent { $count } timer igjen
    }
host-seconds-s = { $seconds } s
host-minutes-min-seconds-s = { $minutes } min { $seconds } s
host-hours-h-minutes-min = { $hours } t { $minutes } min
host-blocked-access = Blokkert av skanning ved tilgang
host-exact-byte-size =
    { $count ->
        [one] { $size } ({ $count } byte)
       *[other] { $size } ({ $count } byte)
    }
host-hash-unknown = Ikke i databasen over kjente filer
host-listed-malicious-source-name = Oppført som skadelig av { $source } (som { $name })
host-listed-malicious-unnamed-source-name = Oppført som skadelig av en navnløs kilde (som { $name })
host-listed-malicious-source = Oppført som skadelig av { $source }
host-listed-malicious-unnamed-source = Oppført som skadelig av en navnløs kilde
host-listed-known-file-source = Oppført som kjent fil av { $source }
host-listed-known-file-unnamed-source = Oppført som kjent fil av en navnløs kilde
host-known-file-name-listed = Kjent fil { $name }, oppført av { $source }
host-known-file-name-unnamed-source = Kjent fil { $name }, oppført av en navnløs kilde
host-unnamed-family = navnløs familie
host-no-known-samples = Ikke en kjent skadevareprøve
host-known-malware-sample = Kjent skadevareprøve
host-samples-found = Kjent skadevare: { $family }, først sett { $date }
host-samples-found-date-unknown = Kjent skadevare: { $family }, først sett på en ukjent dato
host-unnamed-family-samples-found = Kjent skadevare fra en navnløs familie, først sett { $date }
host-unnamed-family-date-unknown = Kjent skadevare fra en navnløs familie, først sett på en ukjent dato
host-file-unknown-virustotal = Ukjent for VirusTotal
host-count-total-engine-verdict =
    { $total ->
        [one] { NUMBER($count) } av { $total } motor oppdager den
       *[other] { NUMBER($count) } av { $total } motorer oppdager den
    }
host-api-key-rejected = VirusTotal avviste API-nøkkelen.
host-request-limit-reached = Grensen for forespørsler til VirusTotal er nådd. Prøv igjen senere.
host-virustotal-answered-status = VirusTotal svarte med status { $status }
host-virustotal-not-reached-error = VirusTotal kunne ikke nås: { $error }
host-virustotal-sent-answer-lazaret = VirusTotal sendte et svar som { -brand-name } ikke kan lese
host-request-failed-circl = CIRCL hashlookup kunne ikke nås: { $error }
host-answer-unreadable-circl = CIRCL hashlookup sendte et svar som ikke kunne leses
host-unexpected-status-circl = CIRCL hashlookup svarte med status { $status }
host-request-failed-bazaar = MalwareBazaar kunne ikke nås: { $error }
host-answer-unreadable-bazaar = MalwareBazaar sendte et svar som ikke kunne leses
host-request-limit-reached-bazaar = Grensen for forespørsler til MalwareBazaar er nådd. Prøv igjen senere.
host-unexpected-status-bazaar = MalwareBazaar svarte med status { $status }
host-key-refused = MalwareBazaar avviste API-nøkkelen
host-query-refused = MalwareBazaar svarte: { $status }
headless-count-threat-found =
    { $count ->
        [one] { $count } trussel funnet
       *[other] { $count } trusler funnet
    }
headless-open-lazaret-review = Åpne { -brand-name } for å se gjennom dem.
headless-count-threats-quarantined =
    { $count ->
        [one] { $count } flyttet til karantene.
       *[other] { $count } flyttet til karantene.
    }
headless-scan-infected = Trusler oppdaget
headless-scheduled-scan-failed = Planlagt skanning mislyktes
headless-open-lazaret-read-log = Åpne { -brand-name } for å lese loggen.
