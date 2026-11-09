-brand-name = Lazaret
startup-settings-schema-missing = Lo schema delle impostazioni di { -brand-name } non è installato. Esegui make install oppure imposta GSETTINGS_SCHEMA_DIR.
startup-unknown-page-name = { -brand-name } non ha nessuna pagina chiamata { $page }, quindi apre la prima area che richiede attenzione. Pagine: { $pages }

window-signature-sources = Fonti delle firme
window-database-updates = Aggiornamenti del database
window-scan-engine = Motore di scansione
window-access-scanning = Scansione all'accesso
window-scheduled-scans-page = Scansioni pianificate
window-manual-scan = Scansione manuale
window-removable-drives = Unità rimovibili
window-detections-label = Rilevamenti
window-quarantine-label = Quarantena
window-trusted-detections = Rilevamenti attendibili
window-sent-samples = Campioni inviati
window-quarantine-database-not-locked = Impossibile bloccare il database della quarantena
window-quarantine-database-not-opened = Impossibile aprire il database della quarantena
window-file-operation-failed-error = Un'operazione sui file non è riuscita: { $error }
window-lock-task-interrupted = Un passaggio della quarantena in background si è interrotto inaspettatamente: { $error }
window-translator-credits = Albano Battistella <albanobattistella@gmail.com>
window-export-history-menu-item = Esporta cronologia…
window-clear-history-menu-item = Cancella cronologia…
window-preferences-label = Preferenze
window-about-menu-item = Informazioni su { -brand-name }
window-quit-menu-item = Esci
window-export-history-heading = Esportare la cronologia?
window-export-history-body = Salva ogni scansione e ogni aggiornamento delle firme registrati, con i relativi dettagli.
window-cancel-history-button = Annulla
window-json-label = JSON
window-csv-label = CSV
window-clear-history-heading = Cancellare la cronologia?
window-clear-history-body = Questa operazione elimina definitivamente ogni scansione e ogni aggiornamento delle firme registrati. Non può essere annullata.
window-clear-label = Cancella tutto
window-scanning-drive-title = Scansione di { $drive }
window-open-removable-drives-button = Apri Unità rimovibili
window-not-export-history-toast = Impossibile esportare la cronologia: { $error }
window-history-export-finished = Cronologia esportata
window-history-cleared = Cronologia cancellata
window-not-clear-history-toast = Impossibile cancellare la cronologia: { $error }
window-other-sources-unknown = Altre fonti sconosciute
window-source-out-date = Una fonte non è aggiornata
window-official-signatures = Solo firme ufficiali
window-official-count-other-source =
    { $count ->
        [one] Ufficiali e { $count } altra fonte
       *[other] Ufficiali e { $count } altre fonti
    }
window-database-missing = Nessuna definizione antivirus
window-age-definitions-unknown = Età delle definizioni sconosciuta
window-last-update-failed = Ultimo aggiornamento non riuscito
window-definitions-dated-future = Definizioni con data futura
window-updated-label = Aggiornato { $relative-time }
window-definitions-label = Definizioni del { $relative-time }
window-freshclam-refused = freshclam rifiutato
window-unknown-whether-freshclam-trusted = Non è noto se freshclam sia attendibile
window-freshclam-copy-account-ignored = Una copia di freshclam nel tuo account viene ignorata
window-clamd-uses-older-signatures = clamd usa firme meno recenti
window-clamd-answering = clamd risponde
window-unit-active-not-answering = clamd non risponde
window-unit-state-unknown-daemon = Stato di clamd sconosciuto
window-unit-stopped-daemon = clamd non in esecuzione
window-never-set-up-daemon = clamd non configurato
window-engine-missing = ClamAV non installato
window-scanner-refused = clamscan rifiutato
window-version-unverified = Versione di ClamAV sconosciuta
window-scanner-program-refused = Un programma di scansione è rifiutato
window-unknown-whether-scanners-trusted = Non è noto se gli scanner siano attendibili
window-scanner-copy-account-ignored = Una copia dello scanner nel tuo account viene ignorata
window-runtime-changes-unknown = Modifiche in esecuzione sconosciute
window-settings-reduced-runtime = Impostazioni ridotte in esecuzione
window-waiting-daemon = In attesa di clamd
window-unit-state-unknown-access = Stato sconosciuto
window-never-set-up-access = Non configurato
window-unit-stopped-access = clamonacc non in esecuzione
window-config-unreadable = clamd.conf illeggibile
window-nothing-watched = Nulla sorvegliato
window-checks-files-not-block = Controlla i file, non blocca
window-checks-files-kernel-cannot = Controlla i file, il kernel non può bloccare
window-checks-files-blocking-unknown = Controlla i file, blocco sconosciuto
window-blocking-paths = Blocca { $paths }
window-folder-watch-limit-unknown = Limite di sorveglianza cartelle sconosciuto
window-mount-watch-conflicts = Conflitti nella sorveglianza dei mount
window-clamd-user-not-excluded = clamd non escluso
window-whole-root-included = Sorveglia / direttamente
window-watch-limit-low = Limite di sorveglianza cartelle basso
window-timer-not-active = Timer non attivo
window-last-scan-failed = Ultima scansione non riuscita
window-next-scan = Prossima scansione { $relative-time }
window-no-next-scan-planned = Nessuna scansione pianificata
window-service-missing = Servizio di scansione mancante
window-service-unreadable = Servizio di scansione sconosciuto
window-lazaret-schedule = Pianificata da { -brand-name }
window-no-schedule = Nessuna scansione pianificata
window-schedule-changed-outside-lazaret = Pianificazione modificata al di fuori di { -brand-name }
window-ready-scan = Pronto per la scansione
window-uses-slower-clamscan = Usa il più lento clamscan
window-exclusion-skip-malware = Un'esclusione potrebbe ignorare malware
window-device-scan-running = Scansione di { $drive }
window-device-scan-ended = Minacce trovate su { $drive }
window-journal-unreadable = Journal illeggibile
window-following-failed = Il monitoraggio del journal si è interrotto
window-notifications-off = Notifiche disattivate
window-alerts-stop-lazaret-quits = Gli avvisi si fermano quando { -brand-name } viene chiuso
window-other-programs-attach-lazaret = Altri programmi possono collegarsi a { -brand-name }
window-unknown-whether-programs-attach = Non è noto se altri programmi possano collegarsi
window-nothing-decide = Nulla da decidere
window-count-needs-decision =
    { $count ->
        [one] { $count } richiede una decisione
       *[other] { $count } richiedono una decisione
    }
window-loading-label = Caricamento…
window-quarantine-empty = Vuota
window-count-item =
    { $count ->
        [one] { $count } elemento
       *[other] { $count } elementi
    }
window-quarantine-cannot-read = Impossibile leggere la quarantena
window-nothing-trusted = Nulla di attendibile
window-count-item-trusted =
    { $count ->
        [one] { $count } elemento attendibile
       *[other] { $count } elementi attendibili
    }
window-signature-folder-unreadable = Cartella delle firme illeggibile
window-another-allowlist-silences-clamav = Un'altra lista di esclusione silenzia ClamAV
window-lazaret-allowlist-unreadable = La lista di esclusione di { -brand-name } è illeggibile
window-sent-samples-cannot-read = Impossibile leggere i campioni inviati
window-nothing-sent = Nulla inviato
window-count-samples-refused =
    { $count ->
        [one] { $count } rifiutato dal proprio servizio
       *[other] { $count } rifiutati dai propri servizi
    }
window-count-samples-awaiting-review =
    { $count ->
        [one] { $count } in attesa di revisione
       *[other] { $count } in attesa di revisione
    }
window-count-samples-sent =
    { $count ->
        [one] { $count } inviato
       *[other] { $count } inviati
    }
window-signatures-label = Firme
window-scans-label = Scansioni
window-findings-label = Risultati
window-detections-come = Da dove provengono i rilevamenti
window-files-get-checked = Come vengono controllati i file
window-found-label = Cosa è stato trovato
window-needs-review = Da verificare
window-not-working = Non funzionante
window-waiting-another-part = In attesa di un altro componente
window-summary-pending = Controllo in corso…
window-menu-tooltip = Menu
window-show-lazaret = Mostra { -brand-name }
window-show-detections = Mostra rilevamenti
window-quick-scan = Scansione rapida
window-update-definitions = Aggiorna definizioni
window-quit-tray-item = Esci
window-quit-completely = Esci completamente
window-hide-tray = Minimizza nell'area di notifica
window-ask-on-next-close = Chiedi alla prossima chiusura
window-close-lazaret = Chiudere { -brand-name }?
window-detection-alerts-stop-lazaret = Gli avvisi di rilevamento si fermano mentre { -brand-name } non è in esecuzione. Continuare a eseguirlo in background o uscire completamente? Puoi cambiarlo in seguito nelle Preferenze.
window-cancel-close-button = Annulla
window-quit-close-button = Esci
window-keep-running = Continua in background

scans-node-js-dependencies = Dipendenze Node.js
scans-git-repository-data = Dati del repository Git
scans-python-virtual-environment = Ambiente virtuale Python
scans-build-output-directory = Cartella di output della build
scans-distribution-output-directory = Cartella di output della distribuzione
scans-python-bytecode-cache = Cache del bytecode Python
scans-remove-exclusion-tooltip = Rimuovi
scans-preset-exclusions-title = Esclusioni predefinite
scans-common-directories-exclude-description = Directory comuni da escludere dalla scansione. Le modifiche vengono salvate immediatamente.
scans-custom-exclusions-title = Esclusioni personalizzate
scans-enter-absolute-path-description = Inserisci un percorso assoluto o un modello come *.tmp. Le modifiche vengono salvate immediatamente.
scans-path-exclude-tmp-title = /path/to/exclude oppure *.tmp
scans-prefer-clamd-when-available = Automatico
scans-always-clamd = clamd (demone)
scans-always-clamscan = clamscan (autonomo)
scans-options-title = Opzioni
scans-backend-title = Backend di scansione
scans-automatic-uses-daemon-subtitle = Automatico usa il demone quando risponde, altrimenti clamscan.
scans-show-live-scan-title = Mostra avanzamento della scansione in tempo reale
scans-display-files-scanned-subtitle = Visualizza in tempo reale i file sottoposti a scansione.
scans-not-added-history-error = Impossibile aggiungere la scansione alla cronologia: { $error }
scans-no-profile-manual = Nessun profilo (manuale)
scans-removable-drive-scanned-title = È in corso la scansione di un'unità rimovibile. Avvia questa scansione quando sarà terminata.
scans-open-removable-drives-button = Apri Unità rimovibili
scans-detecting-backend = Controllo in corso…
scans-backend-undetermined = Backend: sconosciuto, impossibile individuare i programmi di scansione: { $error }
scans-backend-clamd-daemon = Backend: clamd (demone)
scans-backend-clamscan-standalone = Backend: clamscan (autonomo)
scans-scan-title = Scansione
scans-preparing-label = Preparazione…
scans-cancelling-label = Annullamento…
scans-scanning-name = Scansione di { $name }
scans-scanning-count-location =
    { $count ->
        [one] Scansione di { $count } posizione
       *[other] Scansione di { $count } posizioni
    }
scans-cancelled-user = Scansione annullata dall'utente
scans-threats-found-scan = Minacce trovate
scans-not-finish-cleanly = La scansione non si è conclusa correttamente
scans-no-threats-found = Nessuna minaccia trovata
scans-not-create-eicar-test = Impossibile creare il file di prova EICAR: { $error }
scans-add-files-tooltip = Aggiungi file
scans-add-folder-manual-tooltip = Aggiungi cartella
scans-more-tooltip = Altro
scans-run-eicar-test-label = Esegui test EICAR
scans-copy-results-label = Copia risultati
scans-export-results-label = Esporta risultati…
scans-start-scan-label = Avvia scansione
scans-cancel-manual-label = Annulla
scans-lazaret-refuses-clamav-title = { -brand-name } rifiuta un programma ClamAV che il tuo account potrebbe sostituire
scans-details-button = Dettagli
scans-profile-title = Profilo di scansione
scans-profile-short-title = Profilo
scans-manage-profiles-tooltip = Gestisci profili
scans-targets-manual-title = Target di scansione
scans-drop-files-folders-label = Trascina qui file o cartelle da analizzare
scans-add-files-label = Aggiungi file…
scans-add-folder-label = Aggiungi cartella…
scans-found-far-scan-title = Trovato finora
scans-new-scan-button = Nuova scansione
scans-recent-scans-title = Scansioni recenti
scans-started-lazaret-newest-description = Scansioni avviate da { -brand-name }, dalla più recente. Selezionane una per leggerne la trascrizione.
scans-choose-files-scan = Scegli i file da analizzare
scans-choose-folder-scan = Scegli una cartella da analizzare
scans-quick-scan = Scansione rapida
scans-full-scan = Scansione completa
scans-home-folder = Cartella Home
scans-fast-scan-downloads-folder = Scansione rapida della cartella Scaricati per un rapido rilevamento delle minacce
scans-comprehensive-system-wide-scan = Scansione completa dell'intero sistema in tutte le directory accessibili
scans-user-home-directory-personal = Scansione della directory home dell'utente e dei file personali
scans-profile-file-unavailable = Impossibile accedere al file del profilo: { $error }
scans-profile-data-invalid = I dati del profilo non sono validi: { $error }
scans-profile-file-not-replaced = Impossibile sostituire il file del profilo: { $error }
scans-profile-task-interrupted = L'operazione sul profilo è stata interrotta: { $error }
scans-profile-name-missing = Il nome profilo è obbligatorio
scans-profile-name-too-long = Il nome deve contenere al massimo 50 caratteri
scans-profile-name-taken = Un altro profilo usa già questo nome
scans-profile-targets-missing = Aggiungi almeno una destinazione
scans-edit-profile-title = Modifica Profilo
scans-save-label = Salva
scans-name-title = Nome
scans-description-title = Descrizione
scans-targets-profile-title = Destinazioni
scans-add-folder-profile-tooltip = Aggiungi cartella
scans-add-path-title = Aggiungi percorso
scans-exclusions-title = Esclusioni
scans-added-exclusions-preferences-description = Aggiunte alle esclusioni delle Preferenze durante l'esecuzione di questo profilo.
scans-add-pattern-title = Aggiungi modello
scans-cancel-profile = Annulla
scans-defaults-restored = Profili predefiniti ripristinati
scans-profile-deleted = Profilo eliminato
scans-profile-imported = Profilo importato
scans-manage-profiles-title = Gestisci profili
scans-restore-defaults-tooltip = Ripristina predefiniti
scans-new-profile-tooltip = Nuovo Profilo
scans-import-tooltip = Importa
scans-profiles-title = Profili di scansione
scans-select-profile-edit-description = Seleziona un profilo da modificare o da usare per la scansione
scans-restore-default-profiles = Ripristina profili predefiniti
scans-restore-built-default-profiles = Ripristinare i profili predefiniti integrati? Eventuali personalizzazioni dei profili predefiniti verranno reimpostate; i profili personalizzati verranno mantenuti.
scans-delete-profile = Elimina profilo
scans-delete-profile-name = Eliminare il profilo '{ $name }'?
scans-delete-label = Elimina
scans-profile-exported = Profilo esportato
scans-remove-profile-tooltip = Rimuovi
scans-count-target =
    { $count ->
        [one] { $count } destinazione
       *[other] { $count } destinazioni
    }
scans-default-profile = Profilo predefinito
scans-run-tooltip = Esegui
scans-edit-tooltip = Modifica
scans-delete-tooltip = Elimina
scans-export-tooltip = Esporta
scans-remove-target-tooltip = Rimuovi
scans-check-again-access-tooltip = Controlla di nuovo
scans-checking-access-tooltip = Controllo in corso
scans-status-access-title = Stato
scans-access-settings-changed-description = Impostazioni della scansione all'accesso modificate tramite { -brand-name }, come registrato dal sistema. Il software eseguito con il tuo utente non può rimuovere questi record.
scans-no-access-settings-title = Nessuna impostazione della scansione all'accesso è stata modificata tramite { -brand-name }
scans-clamonacc-log-title = Registro di clamonacc
scans-clamonacc-not-written-title = clamonacc non ha scritto nel journal
scans-checking-access-scanning = Controllo della scansione all'accesso…
scans-command-copied-clipboard-access = Comando copiato negli appunti
scans-no-readable-config = Nessun clamd.conf leggibile trovato
scans-reading-task-aborted-clamd = La lettura di clamd.conf si è interrotta prima del termine
scans-watched-mount = { $path } (intero mount)
scans-nothing-watched = Nessuna configurata
scans-default-limit = Il valore predefinito di ClamAV
scans-configuration-label = Configurazione
scans-watched-folders = Cartelle sorvegliate
scans-largest-file-scanned = File più grande analizzato
scans-no-kernel-config-found = Non esiste né /proc/config.gz né il file di configurazione del kernel in /boot.
scans-kernel-config-unreadable = Impossibile leggere la configurazione del kernel.
scans-compressed-config-corrupt = Impossibile decomprimere /proc/config.gz.
scans-kernel-release-unreadable = Impossibile leggere la versione del kernel in esecuzione.
scans-reading-task-aborted-config = La lettura della configurazione del kernel si è interrotta prima di terminare.
scans-blocks-detected-files = I file rilevati vengono bloccati
scans-prevention-off = I file vengono controllati, ma i file rilevati non vengono bloccati
scans-prevention-conflicts-mounts = I file vengono controllati, ma i file rilevati non vengono bloccati, perché OnAccessPrevention non funziona con OnAccessMountPath
scans-kernel-cannot-block = I file vengono controllati, ma questo kernel non può bloccare i file rilevati, perché è stato compilato senza CONFIG_FANOTIFY_ACCESS_PERMISSIONS
scans-kernel-support-unknown = I file vengono controllati, ma non è noto se il kernel possa bloccare i file rilevati. { $reason }
scans-config-unreadable-monitor = In esecuzione, ma impossibile leggere clamd.conf, quindi non è noto cosa venga controllato e se i file rilevati vengano bloccati
scans-running-no-folders-status = In esecuzione, ma nessuna cartella o mount è sorvegliato
scans-running-detected-status = In esecuzione, ma nessuna cartella o mount è sorvegliato e i file rilevati non verrebbero bloccati
scans-unit-state-unknown-access = Non è noto se la scansione all'accesso sia in esecuzione, perché systemd non ha potuto riportarlo
scans-never-set-up-access = La scansione all'accesso non è configurata
scans-unit-stopped-access = { $unit } non è in esecuzione
scans-access-blocking-title = Blocco all'accesso
scans-clamonacc-checks-files-watched = clamonacc controlla i file nelle cartelle sorvegliate quando vengono aperti. Con la prevenzione attiva, il kernel nega l'accesso a un file rilevato: per questo un file del genere non si apre anche se è ancora presente.
scans-mount-watching = Sorveglianza dei mount
scans-onaccessmountpath-watches-whole-mount = OnAccessMountPath sorveglia un intero mount tramite fanotify. ClamAV lo documenta come incompatibile con OnAccessPrevention, OnAccessIncludePath, OnAccessExcludePath e OnAccessExtraScanning, quindi i file rilevati su un mount sorvegliato non vengono mai bloccati. Sorveglia le cartelle con OnAccessIncludePath per bloccarli.
scans-onaccessmountpath-cannot-combined-options = OnAccessMountPath non può essere combinato con { $options }
scans-loop-protection = Protezione dai cicli di scansione
scans-clamd-opens-every-file = clamd apre ogni file che deve analizzare. A meno che OnAccessExcludeUname o OnAccessExcludeUID non indichi l'utente con cui viene eseguito clamd, clamonacc vede queste letture e chiede a clamd di analizzare di nuovo gli stessi file.
scans-clamd-not-excluded-clamonacc = clamd non è escluso, quindi clamonacc può analizzare le sue stesse letture all'infinito
scans-watching-whole-file-system = Sorveglianza dell'intero file system
scans-clamonacc-refuses-onaccessincludepath-include = clamonacc rifiuta OnAccessIncludePath /. Includi le cartelle che ricevono file, come le cartelle home, oppure sorveglia un mount con OnAccessMountPath.
scans-onaccessincludepath-rejected-clamonacc = OnAccessIncludePath / viene rifiutato da clamonacc
scans-folder-watch-limit = Limite di sorveglianza cartelle
scans-path-watched-fs-inotify = { $path } è sorvegliato, ma fs.inotify.max_user_watches è solo { $limit }, quindi le cartelle oltre il limite non vengono controllate
scans-unknown-whether-path-fits = Non è noto se { $path } rientri nel limite di sorveglianza cartelle, perché impossibile leggere fs.inotify.max_user_watches
scans-clamonacc-watches-every-folder = clamonacc sorveglia con inotify ogni cartella sotto un percorso incluso, una sorveglianza per cartella. Un'intera cartella home o un albero più grande può richiedere più sorveglianze di quante ne consenta il vecchio limite predefinito di 8192. Aumenta fs.inotify.max_user_watches con sysctl.
scans-device-auto-scan-title = Scansione automatica dei dispositivi
scans-automatically-scan-removable-description = Analizza automaticamente le unità rimovibili ed esterne quando vengono collegate.
scans-enable-device-auto-title = Abilita scansione automatica dei dispositivi
scans-max-device-size-title = Dimensione massima del dispositivo (GB)
scans-no-size-limit-subtitle = 0 significa nessun limite di dimensione.
scans-skip-battery-device-title = Salta con alimentazione a batteria
scans-drive-scan-not-added = Impossibile aggiungere la scansione dell'unità alla cronologia: { $error }
scans-device-auto-scan = Scansione automatica dei dispositivi
scans-device-connected-during-another = Un dispositivo è stato collegato durante un'altra scansione, quindi non è stato analizzato.
scans-removable-drives-title = Unità rimovibili
scans-cancel-removable-label = Annulla
scans-found-far-removable-title = Trovato finora
scans-drives-session-title = Unità in questa sessione
scans-drives-connected-since-description = Unità collegate dall'avvio di { -brand-name }, dalla più recente, e cosa è successo a ciascuna. Questo elenco viene conservato solo fino alla chiusura di { -brand-name }.
scans-no-drive-connected-title = Nessuna unità è stata ancora collegata
scans-close-results-button = Chiudi risultati
scans-drive-scans-title = Scansioni delle unità
scans-drives-lazaret-scanned-description = Unità analizzate da { -brand-name } al collegamento, dalla più recente. Selezionane una per leggerne la trascrizione.
scans-auto-scan-off = Disattivata
scans-every-drive = Analizza ogni unità
scans-drives-up-size = Analizza le unità fino a { $size }
scans-skipped-larger-than-limit = Non analizzata: { $size } supera il limite di { $limit }
scans-skipped-battery = Non analizzata: il computer era alimentato a batteria
scans-skipped-auto-scan-off = Non analizzata: la scansione automatica dei dispositivi è disattivata
scans-skipped-while-scanning = Non analizzata: era in corso un'altra scansione
scans-size-unreadable = Non analizzata: impossibile leggere la dimensione dell'unità
scans-drive-settling = Controllo dell'unità…
scans-drive-scanning = Scansione in corso…
scans-threats-detected = Analizzata, minacce trovate
scans-no-threats-detected = Analizzata, nessuna minaccia trovata
scans-finished-problems = Analizzata, ma la scansione non si è conclusa correttamente
scans-scan-cancelled = Scansione annullata
scans-device-scan-running = Scansione di { $drive }
scans-drive-scan = Scansione dell'unità
scans-check-again-scan-tooltip = Controlla di nuovo
scans-checking-scan-tooltip = Controllo in corso
scans-status-scan-title = Stato
scans-engine-detection-settings-description = Impostazioni del motore e di rilevamento modificate tramite { -brand-name }, come registrato dal sistema. Il software eseguito con il tuo utente non può rimuovere questi record.
scans-no-engine-detection-title = Nessuna impostazione del motore o di rilevamento è stata modificata tramite { -brand-name }
scans-clamd-log-title = Registro di clamd
scans-no-clamd-log-title = Nessun registro di clamd trovato
scans-checking-scan-engine = Controllo del motore di scansione…
scans-command-copied-clipboard-scan = Comando copiato negli appunti
scans-program-start-failed = Impossibile avviare clamscan: { $error }
scans-probe-timed-out = clamscan non ha risposto in tempo
scans-scanner-refused = { $path } rifiutato, perché il tuo account può sostituirlo
scans-engine-missing = ClamAV non è stato trovato su questo computer
scans-clamav-installed = ClamAV installato
scans-clamav-engine-recognizes-malware = ClamAV è il motore che riconosce il malware. { -brand-name } usa la copia installata su questo computer e non ne include mai una propria.
scans-clamd-still-uses-signatures = clamd usa ancora le firme { $loaded }, ma su disco è presente { $disk }
scans-clamd-answering = clamd risponde
scans-unit-active-not-answering = { $unit } è in esecuzione, ma clamd non risponde. Potrebbe essere ancora in fase di caricamento delle firme.
scans-unit-state-unknown-daemon = clamd non risponde e systemd non ha potuto riportarne lo stato
scans-unit-stopped-daemon = { $unit } non è in esecuzione
scans-never-set-up-daemon = clamd non è configurato, quindi le scansioni ripiegano sul più lento clamscan
scans-same-version-disk = La stessa versione presente su disco
scans-loaded-disk = { $loaded }, mentre su disco è presente { $disk }
scans-unknown-loaded-stored-versions = Sconosciuta, impossibile confrontare la versione caricata con quella salvata
scans-loaded-signatures = Firme caricate
scans-daemon-running = Demone in esecuzione
scans-clamd-keeps-signatures-loaded = clamd mantiene le firme caricate in memoria. Il blocco all'accesso e le scansioni pianificate gli inviano entrambi i propri file.
scans-socket-protected = Solo il sistema può sostituire { $path }
scans-socket-replaceable = Il tuo account può sostituire { $replaceable }, quindi un programma eseguito con il tuo utente potrebbe mettere in { $path } un falso clamd che segnala ogni file come pulito
scans-socket-missing = Nessun socket di clamd in { $path }, quindi clamd non è in esecuzione o è in ascolto altrove
scans-socket-not-checkable = La sandbox Flatpak non può vedere chi possiede il socket di clamd
scans-clamd-socket-protected = Socket di clamd protetto
scans-socket-mechanism = Le scansioni raggiungono clamd tramite il suo socket. Lascia che clamd lo crei in una cartella scrivibile solo da root o dall'utente clamav, come /run/clamav, così nessun programma eseguito con il tuo utente può sostituirlo con un falso clamd.
scans-socket-inspecting = Controllo di chi può sostituire il socket di clamd…
scans-timer-inactive = Non pianificata
scans-next-run-time = Prossima esecuzione: { $time }
scans-scheduled-time-next-run = Pianificata. L'ora della prossima esecuzione non è ancora nota.
scans-systemd-unavailable = Le scansioni pianificate richiedono una sessione utente di systemd, che non è disponibile.
scans-system-already-runs-scheduled = Il tuo sistema esegue già scansioni pianificate, quindi queste impostazioni sono bloccate per evitare una seconda pianificazione. Prossima scansione: { $time }. Modificala nella configurazione di sistema.
scans-system-scan-timer-settings = Il tuo sistema ha un timer di scansione, quindi queste impostazioni sono bloccate per evitare una seconda pianificazione. Modificalo nella configurazione di sistema.
scans-reading-user-timer = Controllo in corso…
scans-inspecting-system-timer = Controllo se il sistema esegue già scansioni pianificate. Fino ad allora queste impostazioni restano bloccate.
scans-system-timer-unknown = { -brand-name } non ha potuto controllare se il sistema esegue già scansioni pianificate, quindi queste impostazioni sono bloccate per evitare una seconda pianificazione: { $error }
scans-scan-time-unreadable = Inserisci l'ora della scansione nel formato HH:MM, ad esempio 02:00.
scans-unit-files-unwritable = Errore durante la creazione dei file systemd: { $error }
scans-timer-not-applied-error = Impossibile applicare il timer: { $error }
scans-hourly-label = Ogni ora
scans-daily-label = Giornaliero
scans-weekly-label = Settimanale
scans-monthly-label = Mensile
scans-monday-label = Lunedì
scans-tuesday-label = Martedì
scans-wednesday-label = Mercoledì
scans-thursday-label = Giovedì
scans-friday-label = Venerdì
scans-saturday-label = Sabato
scans-sunday-label = Domenica
scans-schedule-title = Pianificazione
scans-apply-label = Applica
scans-enable-scheduled-scans-title = Abilita scansioni pianificate
scans-frequency-title = Frequenza di scansione
scans-time-24-hour-title = Ora della scansione (formato 24 ore)
scans-day-week-title = Giorno della settimana
scans-used-weekly-scans-subtitle = Utilizzato per le scansioni settimanali.
scans-day-month-title = Giorno del mese
scans-used-monthly-scans-subtitle = Utilizzato per le scansioni mensili.
scans-targets-separated-commas-title = Destinazioni di scansione (separate da virgole; se vuoto viene analizzata la cartella home)
scans-skip-battery-schedule-title = Salta con alimentazione a batteria
scans-not-start-scheduled-subtitle = Non avviare scansioni pianificate quando si utilizza l'alimentazione a batteria.
scans-auto-quarantine-title = Quarantena automatica
scans-automatically-quarantine-threats-subtitle = Metti automaticamente in quarantena le minacce trovate dalle scansioni pianificate. Mentre { -brand-name } è aperto, le minacce vengono solo segnalate.
scans-review-button = Verifica
scans-check-again-scheduled-tooltip = Controlla di nuovo
scans-checking-scheduled-tooltip = Controllo in corso
scans-system-scan-title = Scansione di sistema
scans-folders-scanned-title = Cartelle analizzate
scans-run-now-label = Esegui ora
scans-starting-scan-tooltip = Avvio della scansione…
scans-status-scheduled-title = Stato
scans-scheduled-runs-title = Esecuzioni pianificate
scans-lazaret-schedule-ran-description = Scansioni eseguite dalla pianificazione di { -brand-name }, dalla più recente. Selezionane una per leggerne la trascrizione.
scans-checking-schedule = Controllo della pianificazione…
scans-command-copied-clipboard-scheduled = Comando copiato negli appunti
scans-not-scheduled = Non pianificata
scans-next-scan = Prossima scansione
scans-threats-found-scheduled = minacce trovate
scans-nothing-found = nulla trovato
scans-no-completed-scan = Non ancora eseguita
scans-last-scan = Ultima scansione
scans-unknown-label = Sconosciuto
scans-system-timer = Il timer di sistema non è attivo
scans-last-scan-ended-error = L'ultima scansione è terminata con un errore
scans-next-scan-time = Prossima scansione: { $time }
scans-timer-starts-scan-service = Il timer avvia un servizio di scansione che non è installato
scans-service-not-read-systemd = Impossibile leggere il servizio di scansione da systemd
scans-lazaret-schedule = Pianificata da { -brand-name }
scans-no-schedule = Nessuna scansione è pianificata
scans-not-scheduled-timer-active = Non pianificata, perché il timer non è attivo
scans-folders-scanned = Cartelle analizzate
scans-scheduled-scan = Scansione pianificata
scans-scheduled-scan-catches-files = Una scansione pianificata individua i file che non sono stati aperti, o che sono diventati rilevabili solo dopo un aggiornamento delle firme.
scans-unit-names-no-folders = L'unità di scansione non indica alcuna cartella
scans-system-scan-service-unit = Il servizio di scansione di sistema { $unit } analizza queste cartelle con le impostazioni del sistema. Eseguirlo richiede l'autorizzazione dell'amministratore.
scans-needs-daemon-see-scan = Richiede il demone, vedi Motore di scansione
scans-bus-call-failed = systemd non ha risposto: { $error }
scans-unrecognized-load-state = systemd ha riportato uno stato dell'unità che { -brand-name } non riconosce
scans-found-malware = La scansione ha trovato malware
scans-unit-finished = La scansione è terminata
scans-not-finish-cleanly-result = La scansione non si è conclusa correttamente: { $result }
scans-authorization-refused = L'avvio della scansione richiede l'autorizzazione dell'amministratore
scans-not-started-error = Impossibile avviare la scansione: { $error }

findings-open-preferences = Apri Preferenze
findings-journal-unreadable-alerts = Impossibile leggere il journal di sistema. Aggiungi il tuo utente al gruppo systemd-journal.
findings-notifications-turned-off-preferences = Le notifiche sono disattivate nelle preferenze
findings-alerts-stop-quit-lazaret = Gli avvisi si fermano quando chiudi { -brand-name }, perché non è configurato per l'esecuzione in background
findings-running-background-following-journal = In esecuzione in background e in ascolto del journal
findings-alerts-label = Avvisi
findings-lazaret-follows-system-journal = { -brand-name } segue il journal di sistema e ti avvisa quando viene rilevato qualcosa. Può farlo solo mentre è in esecuzione, quindi dovrebbe restare attivo in background.
findings-notifications-on = Attive
findings-notifications-off = Disattivate
findings-running-label = In esecuzione
findings-not-running = Non in esecuzione
findings-unit-not-found = Non configurato
findings-systemd-unavailable = Sconosciuto, impossibile raggiungere systemd
findings-following-journal-access = Seguito
findings-journal-unreadable-access = Illeggibile
findings-following-failed = Monitoraggio interrotto
findings-notifications-label = Notifiche
findings-background-service = Servizio in background
findings-system-journal = Journal di sistema
findings-attach-unknown-setting = Non è noto se altri programmi possano collegarsi, perché impossibile leggere l'impostazione del kernel: { $reason }
findings-attach-unknown-reports = Non è noto se altri programmi possano collegarsi, perché il kernel riporta un valore che { -brand-name } non riconosce
findings-attach-unknown-check = Non è noto se altri programmi possano collegarsi, perché il controllo si è interrotto prima del termine
findings-attach-restricted = I programmi non possono collegarsi ad altri programmi che esegui, come { -brand-name }
findings-any-program-may-attach = Qualsiasi programma che esegui può collegarsi a { -brand-name } e cambiare ciò che mostra, inclusi i rilevamenti e le decisioni che prendi qui
findings-yama-not-loaded = Questo kernel non limita i collegamenti, quindi qualsiasi programma che esegui può collegarsi a { -brand-name } e cambiare ciò che mostra
findings-other-programs-cannot-attach = Altri programmi non possono collegarsi a { -brand-name }
findings-program-attaches-lazaret-hide = Un programma che si collega a { -brand-name } può nascondere gli avvisi o cambiare le tue decisioni. Impostare kernel.yama.ptrace_scope a 1 permette solo al processo padre di collegarsi a un programma.
findings-file-owned-other-user = Questo file appartiene a un altro utente, quindi non può essere spostato o eliminato da qui
findings-file-not-checkable = Impossibile controllare questo file, quindi non può essere spostato o eliminato da qui
findings-removal-failed = Impossibile eliminare il file: { $error }
findings-file-check-interrupted = Il controllo del file si è interrotto inaspettatamente
findings-history-not-updated = Impossibile salvare la tua decisione: { $error }
findings-journal-unreadable-list = { -brand-name } non può leggere il journal di sistema, quindi i nuovi rilevamenti in background non appariranno qui. Questo non significa che non sia stato trovato nulla. Aggiungi il tuo utente al gruppo systemd-journal.
findings-loading-detections = Caricamento dei rilevamenti…
findings-nothing-detected-background = Nulla è stato rilevato in background.
findings-access-scanning-scheduled-scans = Cosa hanno trovato la scansione all'accesso e le scansioni pianificate.
findings-detections-title = Rilevamenti
findings-back-detections-tooltip = Torna ai rilevamenti
findings-back-quarantine-tooltip = Torna alla quarantena
findings-alerts-title = Avvisi
findings-detections-reach-keeps-description = Come ti arrivano i rilevamenti e cosa impedisce ad altri programmi di nasconderli.
findings-check-again-background-tooltip = Controlla di nuovo
findings-checking-background-tooltip = Controllo in corso
findings-detection-title = Rilevamento
findings-copied-clipboard-toast = Copiato negli appunti
findings-command-copied-clipboard-toast = Comando copiato negli appunti
findings-detections-lazaret-stopped = I rilevamenti avvenuti mentre { -brand-name } non era in esecuzione non possono essere contati, perché impossibile leggere il punto in cui ha smesso di leggere il journal di sistema: { $error }
findings-detections-not-read-error = Impossibile leggere i rilevamenti: { $error }
findings-malware-blocked-signature = Malware bloccato: { $signature }
findings-malware-found-signature = Malware trovato: { $signature }
findings-count-detection-not-running =
    { $count ->
        [one] { $count } rilevamento mentre { -brand-name } non era in esecuzione
       *[other] { $count } rilevamenti mentre { -brand-name } non era in esecuzione
    }
findings-detections-not-running-detection = Impossibile contare i rilevamenti avvenuti mentre { -brand-name } non era in esecuzione
findings-lazaret-not-read-stopped = { -brand-name } non ha potuto leggere il punto in cui ha smesso di seguire il journal di sistema, quindi sorveglia solo i nuovi rilevamenti. Controlla la pagina Rilevamenti e il journal di sistema per eventuali elementi trovati mentre era chiuso.
findings-quarantine-incident = Quarantena
findings-ask-incident-deletion = Elimina…
findings-delete-incident-file = Elimina definitivamente
findings-cancel-incident-deletion = Annulla
findings-dismiss-incident = Mantieni
findings-look-up-incident-signature = Cerca
findings-submit-incident-sample = Invia…
findings-restored-quarantine = Ripristinato dalla quarantena
findings-moved-quarantine = Spostato in quarantena
findings-deleted-label = Eliminato
findings-kept-label = Mantenuto da te
findings-belongs-another-user = Appartiene a un altro utente
findings-blocked-needs-decision = Bloccato, richiede una decisione
findings-not-blocked-needs-decision = Non bloccato, richiede una decisione
findings-needs-decision = Richiede una decisione
findings-no-longer-there-detection = Non più presente
findings-state-unknown-detection = Impossibile controllare il file
findings-signature-test-file-state = { $signature } (file di prova) · { $state }
findings-signature-state = { $signature } · { $state }
findings-access-schedule = Scansione all'accesso e scansione pianificata
findings-access-only = Scansione all'accesso
findings-schedule-only = La scansione pianificata
findings-file-label = File
findings-signature-source = Fonte della firma
findings-found-label = Trovato da
findings-last-seen = Ultima rilevazione
findings-times-seen = Numero di rilevazioni
findings-detected-label = Rilevato
findings-signature-matches-harmless-eicar = Questa firma corrisponde all'innocuo schema di prova EICAR, che serve a verificare che il rilevamento funzioni.
findings-clamav-matched-file-against = ClamAV ha trovato una corrispondenza tra questo file e una firma. Una corrispondenza è un indizio forte, ma i falsi positivi capitano, soprattutto con strumenti compressi o di amministrazione.
findings-signature-first-seen-time = { $signature }, rilevato per la prima volta { $local-time }
findings-opening-file-refused-why = L'apertura di questo file viene negata, per questo non si apre
findings-not-opened-again-after = Non più aperto dopo la tua decisione
findings-no-longer-there-blocked = Il file non è più presente
findings-not-blocked-scheduled-scan = Non bloccato: l'ha trovato la scansione pianificata e può ancora essere aperto
findings-not-blocked-right-now = Non bloccato al momento: può ancora essere aperto
findings-state-unknown-blocked = Impossibile verificare se il file è bloccato
findings-blocked-label = Bloccato
findings-access-prevention-kernel-refuses = Con la prevenzione all'accesso, il kernel si rifiuta di aprire un file rilevato in una cartella sorvegliata. Il file resta dove si trova finché non decidi cosa farne.
findings-looking-up = Ricerca in corso…
findings-no-known-samples-match = Nessun campione noto corrisponde a questa firma.
findings-unknown-label = sconosciuto
findings-count-known-samples-families = { $count } campioni noti. Famiglie: { $families }. Prima rilevazione: { $date }.
findings-ask-malwarebazaar-signature-description = Chiedi a MalwareBazaar informazioni su questa firma. Viene inviato solo il nome.
findings-set-up-lookup-button = Configura…
findings-bazaar-no-key = Nessuna chiave
findings-bazaar-key-unreadable = Chiave illeggibile
findings-stop-asking-file-description = Non chiedere più di questo file
findings-delete-file-permanently-description = Eliminare definitivamente questo file? L'operazione non può essere annullata.
findings-go-back-without-description = Torna indietro senza eliminare
findings-move-somewhere-safe-description = Spostalo in un luogo sicuro. Potrai ripristinarlo in seguito.
findings-remove-good-description = Rimuovilo definitivamente
findings-know-file-fine-description = Sai che questo file è innocuo. ClamAV lo segnala comunque alla prossima scansione; consideralo attendibile dalla Quarantena o da un risultato di scansione per evitarlo.
findings-decision-title = La tua decisione
findings-unfamiliar-quarantine-keeping-file = Se non lo riconosci, mettilo in quarantena. Mantenere un file impedisce solo a { -brand-name } di chiedertelo: la scansione all'accesso continua a bloccarlo finché il percorso non viene escluso nella configurazione di ClamAV, come mostrato qui.
findings-quarantined-files-restored-quarantine = I file in quarantena possono essere ripristinati dalla pagina Quarantena. Un file mantenuto resta bloccato dalla scansione all'accesso finché il suo percorso non viene escluso nella configurazione di ClamAV.
findings-file-other-owner-decision = Questo file appartiene a un altro utente, quindi { -brand-name } non può spostarlo o eliminarlo
findings-state-unknown-review = { -brand-name } non ha potuto controllare questo file, quindi non può spostarlo o eliminarlo
findings-choose-happens-file = Scegli cosa fare di questo file
findings-malwarebazaar-label = MalwareBazaar
findings-resolved-title = Risolto
findings-resolved-detection-opens-again = Un rilevamento risolto si riapre se ClamAV trova di nuovo lo stesso file in seguito, ad esempio dopo che lo scarichi una seconda volta.
findings-waiting-decision = In attesa della tua decisione
findings-resolved-state-step = { $state }, { $time }
findings-send-quarantined-copy-description = Invia la copia in quarantena a ClamAV, MalwareBazaar o VirusTotal. Un falso positivo segnalato a ClamAV viene corretto per tutti da un successivo aggiornamento delle firme.
findings-cancel-quarantine = Annulla
findings-delete-entries = Elimina
findings-scanner-program-not-installed = Il programma di scansione non è installato.
findings-scanner-program-replaced-account = Il programma di scansione in { $program } può essere sostituito dal tuo account tramite { $path }.
findings-content-pipe-failed = Impossibile passare il file allo scanner: { $error }
findings-scanner-spawn-failed = Impossibile avviare lo scanner: { $error }
findings-verdict-timed-out = La scansione ha richiesto troppo tempo.
findings-detection-settings-unreadable = Impossibile leggere le impostazioni di rilevamento di clamd: { $error }
findings-program-lookup-failed = Impossibile individuare i programmi di scansione: { $error }
findings-payload-unreadable = Impossibile leggere la copia in quarantena: { $error }
findings-restore-file-heading = Ripristinare questo file?
findings-clamav-no-longer-verdict = ClamAV non rileva più questo file con le firme attuali.
findings-restore-button = Ripristina
findings-restore-file-clamav-heading = Ripristinare un file che ClamAV rileva ancora?
findings-clamav-still-detects-verdict = ClamAV rileva ancora questo file come { $detection } con le firme di oggi. Ripristinalo solo se sei sicuro che si tratti di un falso positivo, e valuta di renderlo prima attendibile.
findings-clamav-still-detects-unnamed-verdict = ClamAV rileva ancora questo file con le firme di oggi, senza indicare il nome del rilevamento. Ripristinalo solo se sei sicuro che si tratti di un falso positivo, e valuta di renderlo prima attendibile.
findings-restore-anyway-button = Ripristina comunque
findings-restore-unchecked-file-heading = Ripristinare un file non controllato?
findings-file-not-checked-verdict = Non è stato possibile ricontrollare il file prima di ripristinarlo. { $reason }
findings-location-outside-home-folder = Questa posizione è al di fuori della tua cartella home.
findings-file-goes-back = Il file torna in:
findings-entry-count =
    { $count ->
        [one] { $count } elemento
       *[other] { $count } elementi
    }
findings-clear-old-items = Cancellare gli elementi vecchi?
findings-count-quarantined-file-older =
    { $count ->
        [one] { $count } file in quarantena più vecchio di 30 giorni verrà eliminato definitivamente.
       *[other] { $count } file in quarantena più vecchi di 30 giorni verranno eliminati definitivamente.
    }
findings-delete-quarantined-file = Eliminare il file in quarantena?
findings-file-permanently-deleted = Il file verrà eliminato definitivamente.
findings-quarantine-title = Quarantena
findings-more-tooltip = Altro
findings-check-all-again-label = Ricontrolla tutto
findings-count-still-detected-after-recheck =
    { $count ->
        [one] { $count } file è ancora rilevato.
       *[other] { $count } file sono ancora rilevati.
    }
findings-count-not-rechecked =
    { $count ->
        [one] Non è stato possibile controllare { $count } file.
       *[other] Non è stato possibile controllare { $count } file.
    }
findings-clear-old-items-label = Cancella elementi vecchi…
findings-search-tooltip = Cerca
findings-search-threat-name-placeholder = Cerca per nome della minaccia o percorso
findings-quarantine-storage-title = Archivio della quarantena
findings-quarantine-storage-description = Minacce rilevate isolate dal sistema.
findings-look-up-hash-button = Cerca hash
findings-reputation-section-title = Reputazione
findings-reputation-section-description = Cerca hash invia solo l’hash SHA-256 del file, mai il file.
findings-submit-sample-row-title = Invia il file stesso
findings-report-false-positive-row-title = Avvisa Cisco Talos
findings-report-false-positive-row-subtitle = Un successivo aggiornamento delle firme corregge poi il rilevamento per tutti.
findings-trust-file-row-title = Considera attendibile su questo computer
findings-trust-file-row-subtitle = ClamAV smette di segnalare qui questo file esatto e lo elenca nei Rilevamenti attendibili. Poi ripristinalo.
findings-total-size-title = Dimensione totale
findings-dismiss-label = Ignora
findings-quarantine-cannot-read-title = Impossibile leggere la quarantena
findings-loading-quarantine = Caricamento della quarantena…
findings-quarantine-empty = La quarantena è vuota
findings-hash-copied-clipboard = Hash copiato negli appunti
findings-count-quarantined-file-no =
    { $count ->
        [one] { $count } file in quarantena non viene più rilevato. Controllalo prima di ripristinarlo.
       *[other] { $count } file in quarantena non vengono più rilevati. Controllali prima di ripristinarli.
    }
findings-count-report-clamav-settled =
    { $count ->
        [one] { $count } segnalazione a ClamAV è risolta dalle nuove firme
       *[other] { $count } segnalazioni a ClamAV sono risolte dalle nuove firme
    }
findings-checking-label = Controllo in corso…
findings-no-longer-detected = Non più rilevato
findings-still-detected = Ancora rilevato
findings-integrity-unknown-hash = Sconosciuta. Questo record di quarantena non ha un hash utilizzabile, quindi il file non può essere verificato, controllato online, considerato attendibile o ripristinato. Puoi comunque eliminarlo.
findings-not-checked = Non ancora controllato
findings-integrity-unknown-label = Integrità sconosciuta
findings-restore-label = Ripristina
findings-delete-entry-label = Elimina
findings-set-up-source-button = Configura…
findings-reputation-no-key = Nessuna chiave
findings-reputation-key-unreadable = Chiave illeggibile
findings-view-report-label = Visualizza rapporto
findings-false-positive-title = Falso positivo
findings-report-label = Segnala…
findings-submit-quarantine-label = Invia…
findings-send-clamav-malwarebazaar-tooltip = Invialo a ClamAV, MalwareBazaar o VirusTotal
findings-trust-label = Considera attendibile…
findings-copy-hash-tooltip = Copia hash
findings-submit-quarantined-label = Invia…
findings-service-label = Servizio
findings-status-label = Stato
findings-quarantine-record-no-usable = Questo record di quarantena non ha un hash utilizzabile, quindi { -brand-name } non lo invia da nessuna parte
findings-file-no-longer-quarantine = Questo file non è più in quarantena
findings-service-not-take-detection = Questo servizio non accetta questo rilevamento
findings-file-no-usable-hash = Questo file non è più in quarantena con un hash utilizzabile
findings-check-malwarebazaar-virustotal-tooltip = Controlla novità su MalwareBazaar e VirusTotal
findings-checking-sent-tooltip = Controllo in corso
findings-send-file-title = Invia un file
findings-nothing-leaves-computer-description = Nulla lascia questo computer finché non confermi cosa viene inviato.
findings-report-missed-malware-label = Segnala malware non rilevato…
findings-send-clamav-malicious-tooltip = Invia a ClamAV un file dannoso che non rileva
findings-sent-title = Inviati
findings-lazaret-sent-became-description = Cosa ha inviato { -brand-name } e che esito ha avuto.
findings-filter-sent-samples-tooltip = Filtra campioni inviati
findings-online-services-title = Servizi online
findings-each-service-receives-subtitle = Cosa riceve ogni servizio, e le chiavi di MalwareBazaar e VirusTotal
findings-choose-file-clamav-misses = Scegli il file che ClamAV non rileva
findings-false-positive = Falso positivo
findings-missed-malware = Malware non rilevato
findings-malware-sample = Campione di malware
findings-clamav-false-positive = Segnala falso positivo a ClamAV
findings-clamav-missed-malware = Segnala malware non rilevato a ClamAV
findings-malwarebazaar-sample-choice = Carica su MalwareBazaar
findings-virustotal-sample-choice = Carica su VirusTotal
findings-awaiting-clamav = In attesa della revisione di ClamAV. Le revisioni richiedono almeno 48 ore; un successivo aggiornamento delle firme ne mostra l'esito.
findings-fixed-database-version = Corretto nella versione del database { $version }
findings-fixed-current-signature-database = Corretto nel database delle firme attuale
findings-detected-since-database-version = Rilevato dalla versione del database { $version }
findings-detected-current-signature-database = Rilevato dal database delle firme attuale
findings-uploaded-waiting-malwarebazaar-list = Caricato; in attesa che MalwareBazaar lo elenchi
findings-listed-malwarebazaar = Elencato su MalwareBazaar
findings-malwarebazaar-refused-reason = MalwareBazaar lo ha rifiutato: { $reason }
findings-queued-analysis-virustotal = In coda per l'analisi su VirusTotal
findings-count-total-engine-flags =
    { $count ->
        [one] { NUMBER($count) } motore su { $total } lo segnala come dannoso
       *[other] { NUMBER($count) } motori su { $total } lo segnalano come dannoso
    }
findings-virustotal-refused-reason = VirusTotal lo ha rifiutato: { $reason }
findings-service-kind-hash-date = { $service } · { $kind } · { $hash } · { $date }
findings-integrity-unknown-candidate = { $detection } · Nessun hash utilizzabile, quindi { -brand-name } non può dimostrare cosa invierebbe
findings-all-services = Tutti i servizi
findings-any-status-option = Qualsiasi stato
findings-waiting-label = In attesa
findings-settled-label = Risolto
findings-refused-label = Rifiutato
findings-all-submissions-option = Tutti gli invii
findings-listing-loading = Caricamento degli invii…
findings-nothing-submitted = Nulla inviato
findings-no-submission-matches-filters = Nessun invio corrisponde ai filtri
findings-candidates-loading = Caricamento della quarantena…
findings-quarantine-empty-sent = La quarantena è vuota
findings-no-upload-changed = Nessun caricamento è cambiato
findings-count-upload-new-status =
    { $count ->
        [one] { $count } caricamento ha un nuovo stato
       *[other] { $count } caricamenti hanno un nuovo stato
    }
findings-choose-service-heading = Dove inviare questo file?
findings-set-up-keys-response = Configura chiavi…
findings-each-service-gets-file = Ogni servizio riceve il file stesso. Il passaggio successivo mostra esattamente cosa viene inviato e chi può vederlo.
findings-cancel-submission = Annulla
findings-open-remote-entry-tooltip = Apri la voce remota
findings-same-upload-running = Questo file è già in fase di invio a questo servizio. Attendi che finisca prima di inviarlo di nuovo.
findings-check-again-trusted-tooltip = Controlla di nuovo
findings-checking-trusted-tooltip = Controllo in corso
findings-status-title = Stato
findings-files-trusted-detections-description = File considerati attendibili e rilevamenti ignorati o ripristinati tramite { -brand-name }, come registrato dal sistema. Il software eseguito con il tuo utente non può rimuovere questi record.
findings-no-trusted-detections-title = Nessun rilevamento attendibile è stato modificato tramite { -brand-name }
findings-checking-trusted-detections = Controllo dei rilevamenti attendibili…
findings-command-copied-clipboard = Comando copiato negli appunti
findings-other-allowlist = Altra lista di esclusione
findings-file-reason = { $file }: { $reason }
findings-trusted-files = File attendibili
findings-ignored-detections = Rilevamenti ignorati
findings-allowlist-not-write-status = Una lista di esclusione non scritta da { -brand-name } dice a ClamAV di tacere
findings-count-file-detection-status =
    { $count ->
        [one] { $count } file o rilevamento non viene mai segnalato
       *[other] { $count } file o rilevamenti non vengono mai segnalati
    }
findings-own-allowlist-unreadable-status = { -brand-name } non può leggere la propria lista di esclusione, quindi non può sapere su cosa ClamAV tace
findings-folder-unreadable = { -brand-name } non può leggere la cartella delle firme, quindi non può sapere su cosa ClamAV tace
findings-signature-folder = Cartella delle firme
findings-trusted-detections = Rilevamenti attendibili
findings-trust-list-mechanism = I file e i nomi dei rilevamenti presenti nelle liste di esclusione di ClamAV non vengono mai segnalati, da nessuna scansione. Un malware che ottiene i privilegi di amministratore potrebbe nascondersi in questo modo, quindi verifica che ogni voce sia stata aggiunta intenzionalmente.
findings-clamav-reports-everything-detects = ClamAV segnala tutto ciò che rileva
findings-remove-tooltip = Rimuovi
findings-access-denied = Permesso negato
findings-other-read-failure = Lettura non riuscita: { $reason }
findings-reading-task-aborted = La lettura si è interrotta prima del termine
findings-journal-unreadable-added = Non è noto chi l'abbia aggiunto. { $reason }
findings-known-user = Aggiunto { $local-time } dall'utente { $uid }
findings-unknown-user = Aggiunto { $time } da un utente sconosciuto
findings-no-record-added-system = Nel journal di sistema non risulta chi l'abbia aggiunto
findings-count-entry-not-added =
    { $count ->
        [one] { $count } voce, non aggiunta tramite { -brand-name }
       *[other] { $count } voci, non aggiunte tramite { -brand-name }
    }
findings-entries-cannot-counted-reason = Impossibile contare le voci ({ $reason }), non aggiunte tramite { -brand-name }
findings-list-unreadable = { -brand-name } non può leggere questo elenco
findings-no-trusted-files = Nessun file attendibile
findings-no-ignored-detections = Nessun rilevamento ignorato
findings-lazaret-cannot-read-signature = { -brand-name } non può leggere la cartella delle firme
findings-trusted-detections-title = Rilevamenti attendibili
findings-trusted-by-you-title = Attendibili per te
findings-trusted-by-you-description = Le tue scansioni non ti mostrano questi file, e né le tue scansioni né gli avvisi in background ti mostrano questi rilevamenti. ClamAV stesso continua a rilevarli, quindi la scansione all'accesso può ancora bloccare i file e gli altri utenti continuano a vederli. Qualsiasi modifica a un file attendibile lo fa riapparire.
findings-nothing-trusted-by-you = Non hai considerato attendibile nulla
findings-trust-by-you-unreadable = { -brand-name } non può leggere ciò che hai considerato attendibile: { $error }
findings-file-trusted-by-you = Un file che hai considerato attendibile
findings-trust-for-all-users-tooltip = Considera attendibile per tutti gli utenti…
findings-trusted-for-all-users-title = Attendibili per tutti gli utenti
findings-clamav-never-reports-description = ClamAV non segnala mai ciò che è elencato qui, per nessun utente e in nessuna scansione su questo computer. Ogni modifica richiede una password di amministratore e viene registrata nel journal di sistema, dove il software eseguito con il tuo utente non può rimuovere il record.
findings-ignored-detections-title = Rilevamenti ignorati
findings-detection-names-clamav-description = Nomi di rilevamento che ClamAV ignora in ogni file, per ogni utente. { -brand-name } non li aggiunge più, perché nascondono ogni file con quel rilevamento; rimuovili e considera invece attendibili i file in base al loro hash.
findings-other-allowlists-title = Altre liste di esclusione
findings-allowlist-files-signature-description = File di liste di esclusione nella cartella delle firme non scritti da { -brand-name }. Verifica che siano stati aggiunti intenzionalmente da te o dal tuo amministratore.

signatures-history-load-failed = Impossibile caricare i registri storici: { $error }
signatures-database-update-finished = Aggiornamento del database terminato
signatures-check-again-database-tooltip = Controlla di nuovo
signatures-checking-database-tooltip = Controllo in corso
signatures-update-history-title = Cronologia degli aggiornamenti
signatures-updates-started-lazaret-description = Aggiornamenti avviati da { -brand-name }, dal più recente. Selezionane uno per leggerne la trascrizione.
signatures-historical-logs-cannot-title = Impossibile leggere i registri storici
signatures-status-title = Stato
signatures-loading-history = Caricamento della cronologia…
signatures-no-updates-started-lazaret = Nessun aggiornamento è stato ancora avviato da { -brand-name }
signatures-checking-signatures = Controllo delle firme…
signatures-command-copied-clipboard-database = Comando copiato negli appunti
signatures-update-not-added-history = Impossibile aggiungere l'aggiornamento alla cronologia: { $error }
signatures-update-signatures = Aggiorna ora
signatures-cancel-signature-update = Annulla
signatures-definitions-built-date = Definizioni create { $relative-time } ({ $date })
signatures-definitions-exist-not-determined = Le definizioni esistono, ma non è stato possibile determinarne l'età
signatures-no-virus-definitions-found = Nessuna definizione antivirus trovata
signatures-age-definitions-unknown-checking = L'età delle definizioni è sconosciuta, perché il controllo si è interrotto prima del termine
signatures-last-update-failed = Il suo ultimo aggiornamento non è riuscito
signatures-no-completed-update = Nessun aggiornamento completato registrato finora
signatures-updater-updated = Aggiornato da { $updater }
signatures-provider-fetched = Scaricato da { $provider }
signatures-count-signature-size-file =
    { $count ->
        [one] { $count } firma, { $size } ({ $file })
       *[other] { $count } firme, { $size } ({ $file })
    }
signatures-file-unreadable = Numero di firme sconosciuto, impossibile leggere il file: { $error }, { $size } ({ $file })
signatures-other-signature-sources = Altre fonti di firme
signatures-unknown-reason-database = Sconosciuto. { $reason }
signatures-fetch-newest-signatures = Scarica ora le firme più recenti da ogni programma di aggiornamento
signatures-manual-update = Aggiornamento manuale
signatures-update-service = Servizio di aggiornamento
signatures-database-folder = Cartella del database
signatures-updater-output = Output del programma di aggiornamento
signatures-current-label = Firme aggiornate
signatures-signature-mechanism = Le firme descrivono malware noti e ne vengono pubblicate di nuove continuamente. Le firme vecchie non rilevano le minacce recenti anche quando tutto il resto funziona.
signatures-updating-signatures = Aggiornamento delle firme…
signatures-update-cancelling = Annullamento dell'aggiornamento…
signatures-starting-update = Avvio dell'aggiornamento…
signatures-daemon-running = Attivo
signatures-timer-scheduled = Pianificato da un timer systemd
signatures-service-stopped = Installato ma arrestato
signatures-service-not-found = Nessun servizio freshclam rilevato; gli aggiornamenti vengono eseguiti manualmente
signatures-systemd-unreachable-service = Impossibile interrogare systemd su un servizio freshclam; gli aggiornamenti vengono eseguiti manualmente
signatures-count-day-old =
    { $count ->
        [one] di { $count } giorno
       *[other] di { $count } giorni
    }
signatures-definitions-missing = Nessun database antivirus leggibile trovato
signatures-directory-unreadable = Sconosciuto. { $reason }
signatures-scanner-error = freshclam non è riuscito con codice di uscita { $code }.
signatures-killed-signal = freshclam è stato fermato dal segnale { $signal }.
signatures-unrecognized-wait-status = freshclam è terminato in un modo che { -brand-name } non riconosce.
signatures-wait-failed = { -brand-name } ha perso traccia di freshclam prima che terminasse.
signatures-authorization-refused = Autorizzazione non riuscita. Riprova e inserisci la password.
signatures-systemd-not-reached = Impossibile raggiungere systemd per eseguire l'aggiornamento.
signatures-service-job-ended = Il servizio di aggiornamento non è terminato; systemd ha riportato: { $result }
signatures-update-rate-limited-mirror = Aggiornamento limitato dal mirror. Attendi qualche minuto e riprova.
signatures-update-blocked-cdn = Aggiornamento bloccato dalla CDN, probabilmente a causa di un limite di frequenza. Attendi e riprova più tardi.
signatures-mirror-unavailable = Il mirror di ClamAV non è al momento disponibile. Riprova più tardi.
signatures-certificate-rejected = Errore del certificato SSL/TLS. Il mirror potrebbe avere problemi di configurazione.
signatures-connection-timed-out = Connessione scaduta. Verifica la connessione di rete.
signatures-database-locked = Il database è bloccato. Potrebbe essere in esecuzione un'altra istanza di freshclam.
signatures-permission-denied = Permesso negato. Potrebbero essere necessari privilegi elevati per aggiornare il database.
signatures-connection-failed = Errore di connessione. Verifica la connessione di rete.
signatures-name-resolution-failed = Risoluzione DNS non riuscita. Verifica le impostazioni di rete.
signatures-update-failed-unknown-error = Aggiornamento non riuscito con un errore sconosciuto. Verifica i registri per i dettagli.
signatures-updated-databases = Aggiornato: { $databases }
signatures-already-current-databases = Già aggiornato: { $databases }
signatures-cooldown-unknown = Frequenza limitata: { $database }
signatures-cooldown-until = Frequenza limitata: { $database } fino a { $cooldown }
signatures-definitions-updated = Aggiornamento del database completato
signatures-already-current = Database già aggiornato
signatures-service-ran-without-verdict = Il servizio di aggiornamento è stato eseguito, ma il suo registro non indica se le definizioni sono cambiate.
signatures-update-requested-daemon = Il servizio ha accettato il segnale; esegue l'aggiornamento in modo asincrono. Verifica i registri per il completamento.
signatures-update-cancelled = Aggiornamento database annullato
signatures-scanner-program-missing = Installa freshclam prima di richiedere aggiornamenti del database.
signatures-provider-needs-credential-fangfrisch =
    { $credential ->
        [purchase-receipt] { $provider } richiede un numero di ricevuta d'acquisto, e fangfrisch lo legge solo dalle sue impostazioni. Su NixOS, services.clamav.fangfrisch.settings viene copiato nello store di Nix, leggibile da ogni utente di questo computer, quindi il numero di ricevuta d'acquisto sarebbe esposto. Lascia { $provider } disattivato, oppure esegui fangfrisch con un file di impostazioni conservato fuori dallo store di Nix, come un segreto agenix o sops-nix.
       *[customer-identifier] { $provider } richiede un ID cliente, e fangfrisch lo legge solo dalle sue impostazioni. Su NixOS, services.clamav.fangfrisch.settings viene copiato nello store di Nix, leggibile da ogni utente di questo computer, quindi l'ID cliente sarebbe esposto. Lascia { $provider } disattivato, oppure esegui fangfrisch con un file di impostazioni conservato fuori dallo store di Nix, come un segreto agenix o sops-nix.
    }
signatures-provider-needs-credential-website =
    { $credential ->
        [purchase-receipt] { $provider } richiede un numero di ricevuta d'acquisto dal suo sito web. fangfrisch lo legge dal suo file di impostazioni, di solito { $path }. Aggiungi lì queste righe, sostituisci il segnaposto con il tuo numero di ricevuta d'acquisto e rendi il file leggibile solo da root, così gli altri utenti non possono vederlo.
       *[customer-identifier] { $provider } richiede un ID cliente dal suo sito web. fangfrisch lo legge dal suo file di impostazioni, di solito { $path }. Aggiungi lì queste righe, sostituisci il segnaposto con il tuo ID cliente e rendi il file leggibile solo da root, così gli altri utenti non possono vederlo.
    }
signatures-not-use = Non in uso
signatures-add-lines-fangfrisch-settings = Aggiungi queste righe al file di impostazioni di fangfrisch, di solito { $path }.
signatures-provider-needs-free-abuse = { $provider } richiede un Auth-Key gratuito di abuse.ch. Aggiungi queste righe al file di impostazioni di fangfrisch, di solito { $path }, sostituisci il segnaposto con la tua chiave e rendi il file leggibile solo da root, così gli altri utenti non possono vederlo.
signatures-use-label = In uso
signatures-unknown-reason-source = Sconosciuto. { $reason }
signatures-check-again-signature-tooltip = Controlla di nuovo
signatures-checking-signature-tooltip = Controllo in corso
signatures-clamav-malware-signatures-description = Da dove provengono le firme malware di ClamAV e quali altre fonti potresti aggiungere.
signatures-sources-title = Fonti delle firme
signatures-abuse-ch-key-title = Chiave abuse.ch
signatures-abuse-ch-auth-title = Auth-Key abuse.ch
signatures-abuse-ch-key-uses = Usata per le firme URLhaus e per le ricerche e i caricamenti su MalwareBazaar.
signatures-abuse-ch-no-key = Nessuna chiave
signatures-abuse-ch-key-unreadable = Chiave illeggibile
signatures-command-copied-clipboard-signature = Comando copiato negli appunti
signatures-not-open-documentation-title = Impossibile aprire la documentazione: { $error }
signatures-checking-api-key = Controllo della chiave API…
signatures-check-passed = Superato
signatures-needs-review = Avviso
signatures-check-failed = Non riuscito
signatures-status-unknown = Sconosciuto
signatures-check-skipped = Ignorato
signatures-check-progress = Controllo in corso
signatures-copy-tooltip = Copia
signatures-learn-more-title = Scopri di più
signatures-open-browser-tooltip = Apri nel browser
signatures-checking-title = Controllo in corso…

preferences-launcher-not-found = { -brand-name } non ha trovato il proprio lanciatore: { $error }
preferences-action-unwritable = Impossibile aggiungere l'azione del gestore file: { $error }
preferences-action-unremovable = Impossibile rimuovere l'azione del gestore file: { $error }
preferences-dialog-title = Preferenze
preferences-general-title = Generale
preferences-window-behavior-title = Comportamento della finestra
preferences-detection-alerts-work-description = Gli avvisi di rilevamento funzionano solo mentre { -brand-name } è in esecuzione.
preferences-closing-window-title = Alla chiusura della finestra
preferences-start-background-title = Avvia in background
preferences-launch-background-without-subtitle = Avvia in background senza aprire una finestra.
preferences-desktop-notifications-title = Notifiche desktop
preferences-notify-detections-finished-subtitle = Notifica i rilevamenti, e le scansioni e gli aggiornamenti terminati mentre la finestra non è in primo piano.
preferences-file-manager-integration-title = Integrazione Gestore File
preferences-add-context-menu-description = Aggiungi azioni al menu contestuale per i gestori di file supportati.
preferences-online-services-title = Servizi online
preferences-online-services-summary = Cerca invia solo un hash o il nome di una firma. Un file lascia questo computer solo quando lo invii.
preferences-circl-receives-summary = Riceve l’hash SHA-256 di un file e indica se è un file noto e innocuo. Nessuna chiave necessaria.
preferences-talos-receives-summary = Riceve i file che segnali, con il tuo nome e indirizzo email. Talos non li rende mai pubblici. Nessuna chiave necessaria.
preferences-virustotal-receives-summary = Riceve l’hash SHA-256 di un file. I file caricati vengono condivisi con i clienti paganti di VirusTotal.
preferences-malwarebazaar-receives-summary = Riceve il nome di una firma o l’hash SHA-256 di un file. I file caricati diventano pubblici.
preferences-key-file-not-permitted-hint = Rendilo leggibile dal tuo utente, per esempio impostando il proprietario del segreto agenix o sops-nix.
preferences-key-file-missing-hint = Controlla il percorso del file della chiave nella configurazione di sistema.
preferences-key-file-unreadable-hint = Controlla che il percorso indichi un file normale.
preferences-key-not-valid-hint = Copia la chiave intera dal sito del servizio.
preferences-keyring-unavailable-hint = Sblocca il portachiavi, poi riapri le Preferenze.
preferences-stored-key-unreadable-hint = Elimina la chiave e incollala di nuovo.
preferences-virustotal-service-hint = Piano gratuito: 4 richieste al minuto, 500 al giorno.
preferences-abuse-ch-service-hint = Gratuita dal portale di autenticazione di abuse.ch.
preferences-not-configured = Non configurata
preferences-checking-label = Controllo in corso…
preferences-set-system-configuration-title = Impostata dalla configurazione di sistema
preferences-key-keyring-title = Chiave nel portachiavi
preferences-key-file-title = File della chiave
preferences-key-unreadable-status = Illeggibile
preferences-key-file-not-permitted = Impossibile leggere il file della chiave: permesso negato
preferences-key-file-missing = Il file della chiave non esiste
preferences-key-file-unreadable = Impossibile leggere il file della chiave
preferences-key-not-valid = Chiave non valida per questo servizio
preferences-keyring-unavailable = Il portachiavi non è disponibile
preferences-stored-key-unreadable = Impossibile leggere la chiave nel portachiavi
preferences-delete-key-label = Elimina chiave
preferences-paste-api-key-title = Incolla chiave API
preferences-get-free-api-title = Ottieni una chiave API gratuita

widgets-clean-label = Pulito
widgets-scan-infected = Infetto
widgets-error-status = Errore
widgets-cancelled-label = Annullato
widgets-skipped-label = Ignorato
widgets-success-label = Operazione riuscita
widgets-up-date = Aggiornato
widgets-requested-label = Richiesto
widgets-ran-label = Eseguito
widgets-counted-threats =
    { $count ->
        [one] Trovata { $count } minaccia
       *[other] Trovate { $count } minacce
    }
widgets-uncounted-threats = Lo scanner ha segnalato una minaccia senza indicare il file.
widgets-no-threats-found-activity = Nessuna minaccia trovata
widgets-scan-failed = La scansione non si è conclusa correttamente
widgets-scan-cancelled-activity = Scansione annullata dall'utente
widgets-scheduled-scan-skipped-battery = Scansione programmata saltata (su batteria)
widgets-virus-definitions-updated = Definizioni antivirus aggiornate
widgets-virus-definitions-up-date = Le definizioni antivirus sono aggiornate
widgets-update-requested-service = Aggiornamento richiesto al servizio
widgets-update-service-no-result = Il servizio di aggiornamento è stato eseguito senza riportare un risultato
widgets-update-failed = Aggiornamento del database non riuscito
widgets-update-cancelled = Aggiornamento database annullato
widgets-scan-log = LOG SCANSIONE
widgets-update-log = LOG AGGIORNAMENTO
widgets-timestamp-label = Marca Temporale
widgets-status-label = Stato
widgets-duration-label = Durata
widgets-summary-label = Riepilogo
widgets-path-label = Percorso
widgets-full-output = Output completo
widgets-scan-history-cannot-title = Impossibile leggere la cronologia delle scansioni
widgets-retry-label = Riprova
widgets-copy-activity-tooltip = Copia
widgets-log-details-copied-toast = Dettagli del registro copiati negli appunti
widgets-not-save-file-error = Impossibile salvare il file: { $error }
widgets-not-choose-file-error = Impossibile scegliere il file: { $error }
widgets-chosen-file-not-local = { $location } non è un file di questo computer
widgets-export-title = Esporta
widgets-database-newer-release = Questo database è stato creato da una versione più recente di { -brand-name }: ha la versione di struttura { $found } e questa versione conosce fino alla { $known }. Aggiorna { -brand-name } per aprirlo.
widgets-migration-step-failed = L'aggiornamento della struttura del database non è riuscito al passaggio { $step }: { $error }
widgets-version-check-failed = Impossibile verificare la versione della struttura del database: { $error }
widgets-copy-configuration-line-title = Copia riga di configurazione
widgets-nothing-applied-choose-description = Nulla viene applicato. Scegli un valore e copia la riga, poi aggiungila tu stesso alla configurazione.
widgets-unit-title = Unità
widgets-line-copy-title = Riga da copiare
widgets-copy-line-label = Copia riga
widgets-unlock-required = Sblocca per modificare questa impostazione. Mentre è bloccata, puoi copiarne la riga per la tua configurazione.
widgets-administrator-not-copy = Il tuo amministratore non consente di modificare questa impostazione da { -brand-name }. Puoi copiarne la riga per la tua configurazione.
widgets-plain-bytes = byte
widgets-kibibytes-unit = KiB
widgets-mebibytes-unit = MiB
widgets-unknown-option = { -brand-name } non conosce questa opzione, quindi la mostra soltanto.
widgets-not-yes-no = Da verificare: questo valore dovrebbe essere yes o no.
widgets-not-whole-number = Da verificare: questo valore dovrebbe essere un numero intero.
widgets-count-out-range = Da verificare: questo valore dovrebbe essere compreso tra { $minimum } e { $maximum }.
widgets-not-size = Da verificare: questo valore dovrebbe essere una dimensione in byte, o un numero seguito da K o M.
widgets-path-not-absolute = Da verificare: questo valore dovrebbe essere un percorso completo che inizia con /.
widgets-empty-text = Da verificare: questo valore è vuoto.
widgets-apply-tooltip = Applica
widgets-unit-tooltip = Unità
widgets-setting-runs-commands-changes = Questa impostazione esegue comandi o cambia l'utente con cui viene eseguito ClamAV, quindi { -brand-name } non offre mai di modificarla
widgets-check-value-configuration-file = Verifica questo valore nel file di configurazione.
widgets-copy-configuration-line-tooltip = Copia riga di configurazione…
widgets-applying-restarting-clamav-tooltip = Applicazione e riavvio di ClamAV
widgets-detect-potentially-unwanted-programs = Rileva programmi potenzialmente indesiderati
widgets-heuristic-detections = Rilevamenti euristici
widgets-flag-office-files-macros = Segnala file Office con macro
widgets-flag-encrypted-archives-documents = Segnala archivi e documenti cifrati
widgets-look-inside-archives = Analizza il contenuto degli archivi
widgets-flag-files-too-large = Segnala file troppo grandi per un controllo completo
widgets-adware-hacking-tools-remote = Adware, strumenti di hacking e strumenti di accesso remoto. Aspettati più falsi positivi.
widgets-suspicious-files-no-signature = File sospetti non indicati da alcuna firma, come eseguibili danneggiati e trucchi di phishing.
widgets-macros-common-way-deliver = Le macro sono un modo comune per diffondere malware, ma molti documenti legittimi le usano.
widgets-clamav-cannot-look-inside = ClamAV non può analizzarne il contenuto, quindi potrebbero nascondere malware.
widgets-checks-files-inside-zip = Controlla i file all'interno di archivi zip, rar e altri.
widgets-otherwise-files-above-size = Altrimenti i file oltre i limiti di dimensione e ricorsione passano senza controllo e senza avviso.
widgets-locked-until-unlocked = Sblocca per modificare questa impostazione.
widgets-administrator-not-allow-detection = Il tuo amministratore non consente di modificare questa impostazione da { -brand-name }.
widgets-clamd-settings = Impostazioni di clamd
widgets-access-scanning-settings = Impostazioni della scansione all'accesso
widgets-update-settings = Impostazioni di aggiornamento
widgets-change-snippet-copied = Copiato. Aggiungilo alla configurazione per applicarlo.
widgets-stale-confirmation-dropped = La modifica non è stata apportata: le impostazioni sono state bloccate di nuovo o è iniziata prima un'altra modifica.
widgets-settings-applied = Applicata, ma non ancora nella configurazione di sistema.
widgets-previous-restored = ClamAV non è tornato in uno stato corretto con quel valore, quindi sono state ripristinate le impostazioni precedenti.
widgets-change-not-made-error = La modifica non è stata apportata: { $error }
widgets-authorization-refused = Nulla è cambiato senza la tua password
widgets-admin-service-missing = Questa installazione non ha un servizio di amministrazione di ClamAV, quindi le impostazioni di ClamAV non possono essere modificate da { -brand-name }
widgets-live-tunable-config = { $path } combina la configurazione di sistema con le modifiche fatte qui. Mentre sono bloccate, le impostazioni sono di sola lettura: usa Copia riga di configurazione per copiarne una per la tua configurazione. Dopo lo sblocco, le impostazioni consentite si modificano direttamente.
widgets-change-setting = Modificare questa impostazione?
widgets-cancel-engine = Annulla
widgets-reduce-protection = Riduci protezione
widgets-apply-label = Applica
widgets-key-value-until-time = { $key } { $value }, fino a { $time }
widgets-live-tuning-not-installed = La regolazione in esecuzione non è installata. Su NixOS, abilita services.lazaret.liveTuning; su Debian e Ubuntu, installa il pacchetto { -brand-name }, che include il servizio di amministrazione di ClamAV.
widgets-admin-service-unreachable = Al momento le impostazioni non possono essere modificate qui: impossibile raggiungere il servizio di amministrazione di ClamAV di { -brand-name }.
widgets-loading-config = Lettura della configurazione…
widgets-config-missing = File di configurazione non trovato
widgets-lock-label = Blocca
widgets-unlock-label = Sblocca
widgets-change-settings-title = Modifica impostazioni
widgets-unlock-change-settings-description = Sblocca per modificare le impostazioni che il sistema consente in esecuzione. Lo sblocco di per sé non concede nulla: ogni modifica mostra cosa succederà e chiede l'autorizzazione dell'amministratore. Si blocca di nuovo dopo due minuti.
widgets-changes-system-configuration-title = Modifiche che la configurazione di sistema non conosce
widgets-copy-engine-tooltip = Copia
widgets-reduced-protection-ends-title = Protezione ridotta che termina da sola
widgets-reset-system-configuration-title = Ripristina configurazione di sistema
widgets-detection-title = Rilevamento
widgets-clamav-reports-every-description = Cosa segnala ClamAV in ogni scansione su questo computer. { -brand-name } passa le stesse scelte a clamscan, così entrambi gli scanner concordano.
widgets-no-settings-section-title = Nessuna impostazione in questa sezione
widgets-journal-read-failed = Impossibile leggere il journal di sistema: { $error }
widgets-journal-reading-incomplete = La lettura del journal di sistema si è interrotta prima del termine
widgets-folder-unreadable = Impossibile leggere la cartella delle firme: { $error }
widgets-reading-signature-folder-stopped = La lettura della cartella delle firme si è interrotta prima del termine
widgets-entry-timestamp-invalid = Una voce del journal ha un timestamp fuori dall'intervallo supportato.
widgets-reader-thread-failed = Impossibile avviare il lettore del journal: { $error }
widgets-journal-reading-stopped = La lettura del journal di sistema si è interrotta. { $reason }
widgets-file-hash-mismatch = Hash del file non corrispondente — il file potrebbe essere danneggiato
widgets-restore-refused-integrity-unknown = Questo record di quarantena non ha un hash utilizzabile, quindi il file non può essere verificato e non viene ripristinato. Puoi comunque eliminarlo.
widgets-upload-refused-integrity-unknown = Questo record di quarantena non ha un hash utilizzabile, quindi { -brand-name } non può dimostrare che il file sia quello messo in quarantena e non lo invia da nessuna parte.
widgets-payload-too-large-send = Il file supera { $size }, il massimo che { -brand-name } invia.
widgets-blocked-file-other-filesystem = La scansione all'accesso blocca questo file e si trova su un altro file system, quindi non può essere spostato in quarantena. Puoi invece eliminarlo.
widgets-entry-not-found = Voce quarantena non trovata: { $id }
widgets-recording-failed-file-restored = Impossibile registrare la voce di quarantena nel database. Il file è stato ripristinato nella posizione originale.
widgets-recording-failed-rollback = Impossibile registrare la voce di quarantena nel database. Anche il rollback non è riuscito: { $error }. Il file potrebbe essere orfano in: { $path }
widgets-entry-record-not-removed = Il file è stato spostato, ma impossibile rimuovere il suo record di quarantena. Potrebbe ancora apparire nell'elenco.
widgets-system-call-failed = Una chiamata di sistema non è riuscita: { $error }
widgets-file-operation-failed = Un'operazione sui file non è riuscita: { $error }
widgets-database-operation-failed = L'operazione sul database della quarantena non è riuscita: { $error }
widgets-blocking-step-interrupted = Un passaggio della quarantena in background si è interrotto inaspettatamente: { $error }
widgets-path-contains-nul = Il percorso contiene un byte NUL
widgets-path-has-no-parent = Il percorso non ha una cartella superiore
widgets-path-has-no-file = Il percorso non ha un nome di file
widgets-database-path-not-utf8 = Il percorso del database della quarantena non è UTF-8 valido
widgets-database-not-regular-file = Il database della quarantena non è un file regolare
widgets-source-blocked-access = La scansione all'accesso blocca la lettura del file
widgets-moved-payload-still-blocked = Il file bloccato è ancora illeggibile dopo essere stato spostato in quarantena
widgets-source-not-regular-file = Il file non è un file regolare
widgets-payload-not-regular-file = Il file in quarantena non è un file regolare
widgets-not-enough-free-space = Spazio libero insufficiente per mettere in quarantena il file
widgets-file-replaced-during-operation = Il file è stato sostituito durante lo spostamento, quindi non è stato rimosso
widgets-path-outside-quarantine = Il percorso non si trova nella cartella della quarantena
widgets-invalid-restore-path = Il percorso di ripristino non è valido
widgets-protected-restore-destination = Il ripristino in una cartella di sistema protetta non è consentito
widgets-stored-path-unreadable = Impossibile leggere un percorso salvato nel database della quarantena
widgets-target-missing = La destinazione della scansione non esiste o contiene un byte NUL non valido
widgets-target-not-scannable = La destinazione della scansione non è un file o una directory regolare
widgets-target-directory-unreadable = Impossibile elencare la destinazione di scansione { $path }: { $error }
widgets-file-list-unwritable = Impossibile scrivere l'elenco privato dei file per clamd: { $error }
widgets-enumeration-interrupted = L'elenco della destinazione di scansione si è interrotto inaspettatamente.
widgets-no-files-scanned = Non è stato possibile analizzare alcun file
widgets-unscanned-files =
    { $count ->
        [one] { $count } file non è stato analizzato
       *[other] { $count } file non sono stati analizzati
    }
widgets-clamd-connection-lost = La connessione a clamd è stata persa durante la scansione.
widgets-scanner-stopped-reporting = Lo scanner non ha riportato un risultato per questi file.
widgets-count-not-accessed-title =
    { $count ->
        [one] Impossibile accedere a { $count } file
       *[other] Impossibile accedere a { $count } file
    }
widgets-files-unreadable =
    { $count ->
        [one] Impossibile leggere { $count } file
       *[other] Impossibile leggere { $count } file
    }
widgets-nonfatal-warnings =
    { $count ->
        [one] { $count } avviso non fatale durante la scansione; alcuni file potrebbero essere stati sottoposti solo a una scansione parziale
       *[other] { $count } avvisi non fatali durante la scansione; alcuni file potrebbero essere stati sottoposti solo a una scansione parziale
    }
widgets-count-threat-found-outcome =
    { $count ->
        [one] { $count } minaccia trovata
       *[other] { $count } minacce trovate
    }
widgets-count-file-scanned-outcome =
    { $count ->
        [one] { $count } file analizzato
       *[other] { $count } file analizzati
    }
widgets-clamav-not-open-files = ClamAV non ha potuto aprire questi file.
widgets-unattributed-threats-only = Lo scanner ha segnalato una minaccia senza indicare il file.
widgets-preparing-scan = Preparazione…
widgets-finding-files =
    { $count ->
        [one] Ricerca dei file… { $count } trovato
       *[other] Ricerca dei file… { $count } trovati
    }
widgets-trust-file = Considera attendibile questo file…
widgets-ignore-detection-for-me = Ignora questo rilevamento per me…
widgets-report-false-positive = Segnala falso positivo…
widgets-skip-path-lazaret-scans = Salta questo percorso nelle scansioni di { -brand-name }
widgets-copy-path = Copia percorso
widgets-critical-severity = Critica
widgets-high-severity = Alta
widgets-medium-severity = Media
widgets-low-severity = Bassa
widgets-moved-quarantine-path-check = Spostato in quarantena: { $path }. Verificane la reputazione nella pagina Quarantena.
widgets-quarantine-detected-label = Quarantena
widgets-more-actions-tooltip = Altre azioni
widgets-count-more-file-title =
    { $count ->
        [one] { $count } altro file, elencato nel rapporto esportato
       *[other] Altri { $count } file, elencati nel rapporto esportato
    }
widgets-scan-details-title = Dettagli della scansione
widgets-quarantine-all =
    { $count ->
        [one] Metti in quarantena { $count } minaccia
       *[other] Metti in quarantena { $count } minacce
    }
widgets-move-quarantine = Spostare in quarantena?
widgets-removes-each-original-file = Questa operazione rimuove ogni file originale dalla posizione corrente.
widgets-cancel-scan = Annulla
widgets-quarantine-label = Quarantena
widgets-detected-threats-title = Minacce rilevate
widgets-problems-title = Problemi
widgets-move-path-quarantine = Spostare '{ $path }' in quarantena?
widgets-move-count-detected-file =
    { $count ->
        [one] Spostare { $count } file rilevato in quarantena?
       *[other] Spostare { $count } file rilevati in quarantena?
    }
widgets-path-copied-clipboard = Percorso copiato negli appunti
widgets-file-skipped-future-scans = Questo file verrà saltato nelle scansioni future
widgets-exclusion-already-exists = Questa esclusione esiste già
widgets-results-copied-clipboard = Risultati copiati negli appunti
widgets-results-exported = Risultati esportati
widgets-done-total-file-scanned =
    { $total ->
        [one] { NUMBER($done) } file su { $total } scansionato
       *[other]
            { $done ->
                [one] { NUMBER($done) } file su { $total } scansionato
               *[other] { NUMBER($done) } file su { $total } scansionati
            }
    }
widgets-count-file-scanned-results =
    { $count ->
        [one] { $count } file analizzato
       *[other] { $count } file analizzati
    }
widgets-count-entry-skipped-exclusions =
    { $count ->
        [one] { $count } voce saltata dalle esclusioni
       *[other] { $count } voci saltate dalle esclusioni
    }
widgets-threats-detected-status = Minacce rilevate
widgets-no-threats-detected-status = Nessuna minaccia trovata
widgets-finished-problems-status = La scansione non si è conclusa correttamente
widgets-scan-cancelled-status = Scansione annullata dall'utente
widgets-count-threat-found-results =
    { $count ->
        [one] { $count } minaccia trovata
       *[other] { $count } minacce trovate
    }
widgets-files-scanned = File analizzati
widgets-files-planned = File previsti
widgets-directories-label = Cartelle
widgets-threats-found = Minacce trovate
widgets-file-no-longer-quarantine = Il file non è più presente. Se è in quarantena, consideralo attendibile dalla pagina Quarantena.
widgets-permission-denied = Impossibile leggere il file, forse perché la scansione all'accesso lo blocca. Spostalo in quarantena, poi consideralo attendibile dalla pagina Quarantena.
widgets-file-not-read-scan = Impossibile leggere il file: { $error }
widgets-threats-detected-finished = Minacce rilevate
widgets-no-threats-detected-finished = Nessuna minaccia trovata
widgets-finished-problems-notify = La scansione non si è conclusa correttamente
widgets-scanner-error = Lo scanner si è fermato con codice di errore { $code }.
widgets-killed-signal = Lo scanner è stato fermato dal segnale { $signal }.
widgets-unrecognized-wait-status = Lo scanner è terminato in un modo che { -brand-name } non riconosce (stato di attesa { $status }).
widgets-wait-failed = { -brand-name } ha perso traccia del processo dello scanner: { $error }.
widgets-scanner-program-missing = Lo scanner di ClamAV non è installato o non si trova nel percorso di ricerca.
widgets-scanner-program-replaceable = { -brand-name } si è rifiutato di eseguire { $program }, perché il tuo account può sostituire { $replaceable }. Installa ClamAV a livello di sistema, così un programma introdotto da un malware non può falsificare risultati puliti.
widgets-scanner-spawn-failed = Impossibile avviare lo scanner: { $error }
widgets-scanner-work-directory-unavailable = Impossibile creare la cartella di lavoro privata dello scanner: { $error }
widgets-scanner-output-uncaptured = Impossibile acquisire l'output dello scanner.
widgets-detection-settings-unreadable = La scansione non è partita, perché impossibile leggere le impostazioni di rilevamento di clamd: { $error }
widgets-program-lookup-failed = La scansione non è partita, perché impossibile individuare i programmi di scansione: { $error }
widgets-scan-input-unavailable = Impossibile aprire in modo sicuro l'input di scansione { $path }: { $error }
widgets-detections-warnings-errors-tooltip = Solo rilevamenti, avvisi ed errori
widgets-copy-service-tooltip = Copia
widgets-export-text-tooltip = Esporta testo
widgets-search-log-placeholder = Cerca nel registro
widgets-reading-log = Lettura del registro…
widgets-no-log-lines-match = Nessuna riga del registro corrisponde
widgets-log-copied-clipboard = Registro copiato negli appunti
widgets-text-export-finished = Esportato.
widgets-threat-detected = Rilevamento
widgets-emergency-label = Emergenza
widgets-alert-label = Allarme
widgets-critical-level = Critica
widgets-error-level = Errore
widgets-warning-label = Avviso
widgets-notice-label = Nota
widgets-info-label = Info
widgets-debug-label = Debug
widgets-log-reading = Controllo in corso…
widgets-system-journal = Journal di sistema
widgets-neither-log-file-nor = Non sono stati trovati né un file di registro né righe del journal.
widgets-no-log-file-found = Nessun file di registro trovato. { $reason }
widgets-program-use = { $program } in uso
widgets-ignored-program = { $program } ignorato
widgets-refused-program = { $program } rifiutato
widgets-scanner-programs-trusted = I programmi di scansione sono attendibili
widgets-freshclam-trusted = freshclam è attendibile
widgets-malware-running-clamdscan = Un malware eseguito con il tuo utente potrebbe inserire un falso clamdscan più avanti nel percorso di ricerca che segnala ogni file come pulito. { -brand-name } esegue solo programmi ClamAV che il tuo account non può sostituire, come quelli installati per l'intero sistema.
widgets-malware-running-freshclam = Un malware eseguito con il tuo utente potrebbe inserire un falso freshclam più avanti nel percorso di ricerca che finge di scaricare nuove firme. { -brand-name } esegue solo programmi ClamAV che il tuo account non può sostituire, come quelli installati per l'intero sistema.
widgets-check-interrupted = Sconosciuto, perché il controllo dei programmi ClamAV si è interrotto prima del termine
widgets-clamav-program-replaced-account = Un programma ClamAV può essere sostituito dal tuo account, quindi { -brand-name } si rifiuta di eseguirlo
widgets-clamav-program-account-shadows = Un programma ClamAV nel tuo account nasconde la copia di sistema. { -brand-name } lo ignora, ma verifica da dove proviene
widgets-count-settings-reduced-since =
    { $count ->
        [one] { $count } impostazione controlla meno di quanto configurato dal { $time }
       *[other] { $count } impostazioni controllano meno di quanto configurato dal { $time }
    }
widgets-count-settings-reduced-unknown-time =
    { $count ->
        [one] { $count } impostazione controlla meno di quanto configurato, da un momento che non è stato possibile leggere
       *[other] { $count } impostazioni controllano meno di quanto configurato, da un momento che non è stato possibile leggere
    }
widgets-changed-label = Modificato
widgets-change-time-unknown-override = Sconosciuto
widgets-reduction-unknown-unread = Non è noto se le impostazioni siano state ridotte, perché impossibile leggere le modifiche in esecuzione: { $error }
widgets-reduction-unknown-file = Non è noto se le impostazioni siano state ridotte, perché il file delle modifiche in esecuzione non è nel formato previsto. ClamAV funziona con la configurazione di sistema finché non viene corretto.
widgets-reduction-unknown-reading = Non è noto se le impostazioni siano state ridotte, perché la lettura delle modifiche in esecuzione si è interrotta prima del termine
widgets-settings-changed-runtime = Impostazioni modificate in esecuzione
widgets-administrator-can-change = Chiunque abbia la password di amministratore può modificare le impostazioni di ClamAV in esecuzione. { -brand-name } non ha potuto leggere tali modifiche, quindi non può sapere se controllano meno della tua configurazione.
widgets-protection-reduced-purpose = Protezione ridotta intenzionalmente
widgets-administrator-changed-runtime = Qualcuno con la password di amministratore ha modificato le impostazioni di ClamAV in esecuzione in modo da controllare meno della tua configurazione. Le modifiche di sistema della pagina a cui appartiene l'impostazione mostrano chi e quando.
widgets-base-configuration-changed-now = { $base } nella configurazione, { $changed } ora
widgets-administrator-changed-clamd = Qualcuno con la password di amministratore ha modificato le impostazioni di ClamAV in esecuzione. Impossibile leggere il tuo clamd.conf, quindi { -brand-name } non può sapere se queste modifiche controllano meno della tua configurazione. Le modifiche di sistema della pagina a cui appartiene l'impostazione mostrano chi e quando.
widgets-settings-changed-runtime-clamd = Le impostazioni sono state modificate in esecuzione, ma impossibile leggere clamd.conf per confrontarle
widgets-changed-now = { $changed } ora
widgets-daemon-awaited = Richiede il demone, vedi Motore di scansione
widgets-unknown-run = Sconosciuto. { $reason }
widgets-copy-timeline-tooltip = Copia
widgets-moment-details = { $moment } · { $details }
widgets-why-matters-tooltip = Perché è importante
widgets-what-is-sent-label = Cosa viene inviato:
widgets-clamav-audience = Lo esamina Cisco Talos, il team dietro ClamAV. I campioni vengono elaborati in modo automatico, solo Talos può accedervi e non vengono mai resi pubblici né condivisi con altre aziende. Una revisione richiede almeno 48 ore e il suo esito appare in una successiva versione del database delle firme.
widgets-report-false-positive-clamav = Segnalare un falso positivo a ClamAV?
widgets-file-content = Il contenuto del file
widgets-detection-name = Il nome del rilevamento { $detection }
widgets-report-missed-malware-clamav = Segnalare malware non rilevato a ClamAV?
widgets-name-email-address-talos = Il tuo nome e indirizzo email, così Talos può contattarti
widgets-lazaret-sends-file-clamav = { -brand-name } invia questo file a ClamAV con clamsubmit. Invia solo file che non contengono nulla di privato.
widgets-send-report-button = Invia segnalazione
widgets-file-content-named-sha = Il contenuto del file, identificato dal suo hash SHA-256 e non dal nome
widgets-upload-malwarebazaar-heading = Caricare su MalwareBazaar?
widgets-lazaret-uploads-file-malwarebazaar = { -brand-name } carica questo file su MalwareBazaar, gestito da abuse.ch.
widgets-tag-clamav-detection = Il tag { $tag }, dal rilevamento di ClamAV
widgets-abuse-ch-key-sample = La tua chiave abuse.ch; il campione viene elencato senza il nome del tuo account
widgets-malwarebazaar-public-anyone-download = MalwareBazaar è pubblico: chiunque può scaricare il campione una volta caricato, e un caricamento non può essere ritirato. Carica solo malware, mai un documento che potrebbe contenere dati personali.
widgets-upload-publicly-button = Carica pubblicamente
widgets-upload-virustotal-heading = Caricare su VirusTotal?
widgets-lazaret-uploads-file-virustotal = { -brand-name } carica questo file su VirusTotal per l'analisi.
widgets-virustotal-key-links-upload = La tua chiave VirusTotal, che collega il caricamento al tuo account
widgets-virustotal-shares-uploaded-files = VirusTotal condivide i file caricati con i suoi clienti paganti, che possono scaricarli, e chiunque può cercare il risultato tramite l'hash del file. Non caricare mai un documento che potrebbe contenere dati personali.
widgets-upload-button = Carica
widgets-cancel-submission = Annulla
widgets-name-title = Il tuo nome
widgets-email-address-title = Il tuo indirizzo email
widgets-file-unreadable = Impossibile leggere il file: { $error }
widgets-file-not-regular = Si possono inviare solo file regolari
widgets-file-too-large = Il file supera { $size }, il massimo che { -brand-name } invia.
widgets-reading-interrupted = La lettura del file si è interrotta inaspettatamente
widgets-history-failure = Impossibile leggere la cronologia degli invii: { $error }
widgets-earlier-submission = Questo file è già stato inviato il { $local-time }. Non è stato inviato nulla di nuovo.
widgets-report-recorded = Segnalazione inviata. La revisione di ClamAV richiede almeno 48 ore; Campioni inviati mostra quando un aggiornamento delle firme cambia il verdetto.
widgets-report-not-sent-error = La segnalazione non è stata inviata: { $error }
widgets-report-sent-without-record = Segnalazione inviata, ma impossibile aggiungerla a Campioni inviati: { $error }
widgets-nothing-uploaded-error = Non è stato caricato nulla. { $reason }
widgets-bazaar-key-not-usable = MalwareBazaar non ha una chiave utilizzabile.
widgets-virustotal-key-not-usable = VirusTotal non ha una chiave utilizzabile.
widgets-malwarebazaar-refused-sample-reason = MalwareBazaar ha rifiutato il campione: { $reason }
widgets-malwarebazaar-sample-sent-samples = MalwareBazaar ha il campione. Campioni inviati rimanda alla sua voce pubblica.
widgets-not-detected-clamav = MalwareBazaar accetta solo file rilevati da una firma di ClamAV
widgets-unwanted-program-only = MalwareBazaar non accetta programmi potenzialmente indesiderati, solo malware
widgets-sent-without-record = Caricato, ma impossibile aggiungerlo a Campioni inviati: { $error }
widgets-virustotal-refused-file-reason = VirusTotal ha rifiutato il file: { $reason }
widgets-virustotal-analysing-file-refresh = VirusTotal sta analizzando il file. Aggiorna nella pagina Campioni inviati per vederne il verdetto.
widgets-report-already-available =
    { $count ->
        [one] VirusTotal conosce già questo file: { NUMBER($count) } motore su { $total } lo rileva. Non è stato caricato nulla.
       *[other] VirusTotal conosce già questo file: { NUMBER($count) } motori su { $total } lo rilevano. Non è stato caricato nulla.
    }
widgets-known-user = { $local-time } · { $outcome } · richiesto dall'utente { $uid }
widgets-unknown-user = { $time } · { $outcome } · richiesto da un utente sconosciuto
widgets-change-lowered-protection-tooltip = Questa modifica ha ridotto la protezione
widgets-system-changes-failed = Impossibile leggere il journal di sistema: { $error }
widgets-system-changes-title = Modifiche di sistema
widgets-refresh-tooltip = Aggiorna
widgets-system-changes-cannot-title = Impossibile leggere le modifiche di sistema
widgets-loading-system-changes = Caricamento delle modifiche di sistema…
widgets-trust-file-heading = Considerare attendibile questo file?
widgets-clamav-never-report-body =
    ClamAV non segnalerà mai più un file con esattamente questo contenuto, per nessun utente, ovunque si trovi su questo computer e qualunque scansione lo trovi, e la scansione all'accesso smetterà di bloccarlo. È necessaria la password di amministratore, e la richiesta di password mostra lo stesso hash SHA-256 riportato sotto.

    { $path }
widgets-stop-showing-file-body =
    { -brand-name } smetterà di mostrarti un file con esattamente questo contenuto nelle scansioni che esegui, ovunque si trovi. ClamAV stesso continua a rilevarlo, quindi la scansione all'accesso può ancora bloccarlo e avvisarti, e gli altri utenti continuano a vederlo. Considera attendibile per tutti gli utenti impedisce a ClamAV di rilevarlo per tutti e richiede la password di amministratore.

    { $path }
widgets-trust-for-me-button = Attendibile per me
widgets-user-trust-unreadable = Impossibile leggere i file e i rilevamenti che hai considerato attendibili, quindi questa scansione mostra ogni rilevamento: { $error }
widgets-trust-for-all-users-button = Attendibile per tutti gli utenti…
widgets-sha-256-hash-size =
    SHA-256 { $hash }
    { $size }
widgets-ignore-detection-heading = Ignorare questo rilevamento?
widgets-stop-showing-detection-body = { -brand-name } smetterà di mostrarti ogni file con questo rilevamento, non solo questo. ClamAV stesso e gli altri utenti continuano a vederli. Considerare attendibile il file in base al suo hash è più sicuro quando solo un file è un falso positivo.
widgets-ignore-button = Ignora
widgets-stop-trusting-file-heading = Non considerare più attendibile questo file?
widgets-clamav-report-file-body = ClamAV segnalerà di nuovo questo file a ogni utente quando corrisponde a una firma.
widgets-show-file-again-body = { -brand-name } ti mostrerà di nuovo questo file quando ClamAV lo rileva.
widgets-stop-trusting-button = Revoca attendibilità
widgets-report-detection-again-heading = Segnalare di nuovo questo rilevamento?
widgets-clamav-report-files-body = ClamAV segnalerà di nuovo a ogni utente i file con questo rilevamento.
widgets-show-detection-again-body = { -brand-name } ti mostrerà di nuovo i file con questo rilevamento.
widgets-report-again-button = Segnala di nuovo
widgets-cancel-trust = Annulla

host-action-applied = Applicata
host-action-rolled-back = Annullata
host-authorization-refused-described = Rifiutata senza password
host-action-failed = Non riuscita
host-outcome-not-recorded = Esito non registrato
host-unrecognized-outcome = Esito non riconosciuto { $outcome }
host-blocking-turned-off = I file rilevati non verranno più bloccati all'apertura. Verranno solo segnalati.
host-check-turned-off = ClamAV smetterà di controllare questo tipo di file o minaccia su questo computer.
host-watched-folder-removed = I file nella cartella rimossa non verranno più controllati all'apertura.
host-skipped-location-added = ClamAV salterà completamente la nuova voce, quindi il malware lì presente passerà inosservato.
host-scan-limit-lowered = I file o gli archivi oltre il nuovo limite verranno saltati senza essere controllati.
host-current-values-unknown = Impossibile leggere il valore attuale, quindi questa modifica potrebbe ridurre la protezione.
host-detection-trusted = ClamAV non segnalerà più ciò che consideri attendibile qui.
host-key-now-current-new =
    { $key }
    Ora: { $current }
    Nuovo: { $new }
host-system-records-change-reduced = Il sistema registra che questa modifica ha ridotto la protezione, e la barra laterale continua a segnalare la pagina dell'impostazione modificata finché non la annulli.
host-trust-applied = ClamAV ha caricato la modifica
host-trust-recorded-for-you = { -brand-name } non te lo mostra più. ClamAV stesso, il blocco all'accesso e gli altri utenti continuano a vederlo.
host-applied-reload-pending = Salvato. clamd non ha risposto, quindi applicherà la modifica al prossimo ricaricamento delle firme
host-authorization-refused-outcome = Nulla è cambiato senza la tua password
host-admin-service-missing = Questa installazione non ha un servizio di amministrazione di ClamAV, quindi la lista di attendibilità di ClamAV non può essere modificata da { -brand-name }
host-trust-failed = Nulla è cambiato: { $error }
host-nixos-configuration = { $path } è generato dalla tua configurazione NixOS, quindi { -brand-name } non può modificarlo. Usa Copia riga di configurazione su un'impostazione per copiarne la riga per la tua configurazione, poi ricompila.
host-system-package = { $path } appartiene al sistema, quindi { -brand-name } non può modificarlo. Usa Copia riga di configurazione su un'impostazione per copiare la riga da inserire in quel file.
host-contact-incomplete = Inserisci il tuo nome e un indirizzo email valido per inviare una segnalazione
host-submitter-missing = La segnalazione richiede clamsubmit, che è incluso in ClamAV ma non è installato qui
host-submitter-replaceable = { -brand-name } si è rifiutato di eseguire clamsubmit, perché il tuo account può sostituirlo
host-report-not-sent-error = La segnalazione non è stata inviata: { $error }
host-upload-timed-out = La segnalazione non è stata inviata: il caricamento ha richiesto troppo tempo
host-unrecognized-source = Altra fonte
host-file-progress =
    { $total ->
        [one] { NUMBER($done) } file su { $total }
       *[other] { NUMBER($done) } file su { $total }
    }
host-count-second-left =
    { $count ->
        [one] Circa { $count } secondo rimanente
       *[other] Circa { $count } secondi rimanenti
    }
host-count-minute-left =
    { $count ->
        [one] Circa { $count } minuto rimanente
       *[other] Circa { $count } minuti rimanenti
    }
host-count-hour-left =
    { $count ->
        [one] Circa { $count } ora rimanente
       *[other] Circa { $count } ore rimanenti
    }
host-seconds-s = { $seconds } s
host-minutes-min-seconds-s = { $minutes } min { $seconds } s
host-hours-h-minutes-min = { $hours } h { $minutes } min
host-blocked-access = Bloccato dalla scansione all'accesso
host-exact-byte-size =
    { $count ->
        [one] { $size } ({ $count } byte)
       *[other] { $size } ({ $count } byte)
    }
host-hash-unknown = Non presente nel database dei file noti
host-listed-malicious-source-name = Elencato come dannoso da { $source } (come { $name })
host-listed-malicious-unnamed-source-name = Elencato come dannoso da una fonte senza nome (come { $name })
host-listed-malicious-source = Elencato come dannoso da { $source }
host-listed-malicious-unnamed-source = Elencato come dannoso da una fonte senza nome
host-listed-known-file-source = Elencato come file noto da { $source }
host-listed-known-file-unnamed-source = Elencato come file noto da una fonte senza nome
host-known-file-name-listed = File noto { $name }, elencato da { $source }
host-known-file-name-unnamed-source = File noto { $name }, elencato da una fonte senza nome
host-unnamed-family = famiglia senza nome
host-no-known-samples = Non è un campione di malware noto
host-known-malware-sample = Campione di malware noto
host-samples-found = Malware noto: { $family }, rilevato per la prima volta { $date }
host-samples-found-date-unknown = Malware noto: { $family }, rilevato per la prima volta in una data sconosciuta
host-unnamed-family-samples-found = Malware noto di una famiglia senza nome, rilevato per la prima volta { $date }
host-unnamed-family-date-unknown = Malware noto di una famiglia senza nome, rilevato per la prima volta in una data sconosciuta
host-file-unknown-virustotal = Sconosciuto a VirusTotal
host-count-total-engine-verdict =
    { $count ->
        [one] { NUMBER($count) } motore su { $total } lo rileva
       *[other] { NUMBER($count) } motori su { $total } lo rilevano
    }
host-api-key-rejected = VirusTotal ha rifiutato la chiave API.
host-request-limit-reached = È stato raggiunto il limite di richieste di VirusTotal. Riprova più tardi.
host-virustotal-answered-status = VirusTotal ha risposto con lo stato { $status }
host-virustotal-not-reached-error = Impossibile raggiungere VirusTotal: { $error }
host-virustotal-sent-answer-lazaret = VirusTotal ha inviato una risposta che { -brand-name } non riesce a leggere
host-request-failed-circl = Impossibile raggiungere CIRCL hashlookup: { $error }
host-answer-unreadable-circl = CIRCL hashlookup ha inviato una risposta illeggibile
host-unexpected-status-circl = CIRCL hashlookup ha risposto con lo stato { $status }
host-request-failed-bazaar = Impossibile raggiungere MalwareBazaar: { $error }
host-answer-unreadable-bazaar = MalwareBazaar ha inviato una risposta illeggibile
host-request-limit-reached-bazaar = È stato raggiunto il limite di richieste di MalwareBazaar. Riprova più tardi.
host-unexpected-status-bazaar = MalwareBazaar ha risposto con lo stato { $status }
host-key-refused = MalwareBazaar ha rifiutato la chiave API
host-query-refused = Risposta di MalwareBazaar: { $status }
headless-count-threat-found =
    { $count ->
        [one] { $count } minaccia trovata
       *[other] { $count } minacce trovate
    }
headless-open-lazaret-review = Apri { -brand-name } per esaminarle.
headless-count-threats-quarantined =
    { $count ->
        [one] { $count } spostata in quarantena.
       *[other] { $count } spostate in quarantena.
    }
headless-scan-infected = Minacce rilevate
headless-scheduled-scan-failed = Scansione pianificata non riuscita
headless-open-lazaret-read-log = Apri { -brand-name } per leggere il registro.
