-brand-name = Lazaret
startup-settings-schema-missing = Le schéma de paramètres de { -brand-name } n’est pas installé. Exécutez make install ou définissez GSETTINGS_SCHEMA_DIR.
startup-unknown-page-name = { -brand-name } n’a pas de page nommée { $page } : il ouvre donc la première zone qui demande votre attention. Pages : { $pages }

window-signature-sources = Sources de signatures
window-database-updates = Mises à jour de la base de données
window-scan-engine = Moteur d’analyse
window-access-scanning = Analyse à l’accès
window-scheduled-scans-page = Analyses planifiées
window-manual-scan = Analyse manuelle
window-removable-drives = Lecteurs amovibles
window-detections-label = Détections
window-quarantine-label = Quarantaine
window-trusted-detections = Détections approuvées
window-sent-samples = Échantillons envoyés
window-quarantine-database-not-locked = La base de données de la quarantaine n’a pas pu être verrouillée
window-quarantine-database-not-opened = La base de données de la quarantaine n’a pas pu être ouverte
window-file-operation-failed-error = Une opération sur un fichier a échoué : { $error }
window-lock-task-interrupted = Une étape de quarantaine en arrière-plan s’est arrêtée de façon inattendue : { $error }
window-translator-credits = Robin GUY
window-export-history-menu-item = Exporter l’historique…
window-clear-history-menu-item = Effacer l’historique…
window-preferences-label = Préférences
window-about-menu-item = À propos de { -brand-name }
window-quit-menu-item = Quitter
window-export-history-heading = Exporter l’historique ?
window-export-history-body = Enregistre chaque analyse et chaque mise à jour des signatures enregistrées, avec leurs détails.
window-cancel-history-button = Annuler
window-json-label = JSON
window-csv-label = CSV
window-clear-history-heading = Effacer l’historique ?
window-clear-history-body = Cette action supprime définitivement chaque analyse et chaque mise à jour des signatures enregistrées. Elle est irréversible.
window-clear-label = Tout effacer
window-scanning-drive-title = Analyse de { $drive }
window-open-removable-drives-button = Ouvrir les lecteurs amovibles
window-not-export-history-toast = Impossible d’exporter l’historique : { $error }
window-history-export-finished = Historique exporté
window-history-cleared = Historique effacé
window-not-clear-history-toast = Impossible d’effacer l’historique : { $error }
window-other-sources-unknown = Autres sources inconnues
window-source-out-date = Une source n’est pas à jour
window-official-signatures = Signatures officielles uniquement
window-official-count-other-source =
    { $count ->
        [one] Officielles et { $count } autre source
       *[other] Officielles et { $count } autres sources
    }
window-database-missing = Aucune base de données virale
window-age-definitions-unknown = Âge de la base virale inconnu
window-last-update-failed = Échec de la dernière mise à jour
window-definitions-dated-future = Base virale datée dans le futur
window-updated-label = Mise à jour { $relative-time }
window-definitions-label = Base virale du { $relative-time }
window-freshclam-refused = freshclam est refusé
window-unknown-whether-freshclam-trusted = Fiabilité de freshclam inconnue
window-freshclam-copy-account-ignored = Une copie de freshclam dans votre compte est ignorée
window-clamd-uses-older-signatures = clamd utilise des signatures plus anciennes
window-clamd-answering = clamd répond
window-unit-active-not-answering = clamd ne répond pas
window-unit-state-unknown-daemon = État de clamd inconnu
window-unit-stopped-daemon = clamd n’est pas en cours d’exécution
window-never-set-up-daemon = clamd n’est pas configuré
window-engine-missing = ClamAV n’est pas installé
window-scanner-refused = clamscan est refusé
window-version-unverified = Version de ClamAV inconnue
window-scanner-program-refused = Un programme d’analyse est refusé
window-unknown-whether-scanners-trusted = Fiabilité des programmes d’analyse inconnue
window-scanner-copy-account-ignored = Une copie d’un programme d’analyse dans votre compte est ignorée
window-runtime-changes-unknown = Modifications à l’exécution inconnues
window-settings-reduced-runtime = Paramètres réduits à l’exécution
window-waiting-daemon = En attente de clamd
window-unit-state-unknown-access = État inconnu
window-never-set-up-access = Non configurée
window-unit-stopped-access = clamonacc n’est pas en cours d’exécution
window-config-unreadable = clamd.conf illisible
window-nothing-watched = Rien n’est surveillé
window-checks-files-not-block = Vérifie les fichiers, sans les bloquer
window-checks-files-kernel-cannot = Vérifie les fichiers, le noyau ne peut pas bloquer
window-checks-files-blocking-unknown = Vérifie les fichiers, blocage inconnu
window-blocking-paths = Blocage de { $paths }
window-folder-watch-limit-unknown = Limite de surveillance des dossiers inconnue
window-mount-watch-conflicts = Conflits de surveillance des points de montage
window-clamd-user-not-excluded = clamd n’est pas exclu
window-whole-root-included = Surveille / directement
window-watch-limit-low = Limite de surveillance des dossiers basse
window-timer-not-active = Minuterie inactive
window-last-scan-failed = Échec de la dernière analyse
window-next-scan = Prochaine analyse { $relative-time }
window-no-next-scan-planned = Aucune prochaine analyse prévue
window-service-missing = Service d’analyse manquant
window-service-unreadable = Service d’analyse inconnu
window-lazaret-schedule = Planifiée par { -brand-name }
window-no-schedule = Aucune analyse planifiée
window-schedule-changed-outside-lazaret = Planification modifiée en dehors de { -brand-name }
window-ready-scan = Prêt pour l’analyse
window-uses-slower-clamscan = Utilise clamscan, plus lent
window-exclusion-skip-malware = Une exclusion peut ignorer des logiciels malveillants
window-device-scan-running = Analyse de { $drive }
window-device-scan-ended = Menaces détectées sur { $drive }
window-journal-unreadable = Journal illisible
window-following-failed = Le suivi du journal s’est arrêté
window-notifications-off = Notifications désactivées
window-alerts-stop-lazaret-quits = Les alertes s’arrêtent quand { -brand-name } se ferme
window-other-programs-attach-lazaret = D’autres programmes peuvent s’attacher à { -brand-name }
window-unknown-whether-programs-attach = Impossible de savoir si des programmes peuvent s’attacher
window-nothing-decide = Aucune décision à prendre
window-count-needs-decision =
    { $count ->
        [one] { $count } attend une décision
       *[other] { $count } attendent une décision
    }
window-loading-label = Chargement…
window-quarantine-empty = Vide
window-count-item =
    { $count ->
        [one] { $count } élément
       *[other] { $count } éléments
    }
window-quarantine-cannot-read = La quarantaine est illisible
window-nothing-trusted = Rien n’est approuvé
window-count-item-trusted =
    { $count ->
        [one] { $count } élément approuvé
       *[other] { $count } éléments approuvés
    }
window-signature-folder-unreadable = Dossier des signatures illisible
window-another-allowlist-silences-clamav = Une autre liste d’autorisation fait taire ClamAV
window-lazaret-allowlist-unreadable = La liste d’autorisation de { -brand-name } est illisible
window-sent-samples-cannot-read = Les échantillons envoyés sont illisibles
window-nothing-sent = Rien n’a été envoyé
window-count-samples-refused =
    { $count ->
        [one] { $count } refusé par son service
       *[other] { $count } refusés par leurs services
    }
window-count-samples-awaiting-review =
    { $count ->
        [one] { $count } en attente d’examen
       *[other] { $count } en attente d’examen
    }
window-count-samples-sent =
    { $count ->
        [one] { $count } envoyé
       *[other] { $count } envoyés
    }
window-signatures-label = Signatures
window-scans-label = Analyses
window-findings-label = Résultats
window-detections-come = Origine des détections
window-files-get-checked = Comment les fichiers sont vérifiés
window-found-label = Ce qui a été trouvé
window-needs-review = À examiner
window-not-working = Ne fonctionne pas
window-waiting-another-part = En attente d’un autre composant
window-summary-pending = Vérification en cours…
window-menu-tooltip = Menu
window-show-lazaret = Afficher { -brand-name }
window-show-detections = Afficher les détections
window-quick-scan = Analyse rapide
window-update-definitions = Mettre à jour la base virale
window-quit-tray-item = Quitter
window-quit-completely = Quitter complètement
window-hide-tray = Réduire dans la zone de notification
window-ask-on-next-close = Demander à la prochaine fermeture
window-close-lazaret = Fermer { -brand-name } ?
window-detection-alerts-stop-lazaret = Les alertes de détection s’arrêtent tant que { -brand-name } n’est pas en cours d’exécution. Le laisser tourner en arrière-plan ou quitter complètement ? Vous pourrez modifier ce choix plus tard dans les Préférences.
window-cancel-close-button = Annuler
window-quit-close-button = Quitter
window-keep-running = Laisser tourner

scans-node-js-dependencies = Dépendances Node.js
scans-git-repository-data = Données du dépôt Git
scans-python-virtual-environment = Environnement virtuel Python
scans-build-output-directory = Répertoire de sortie de compilation
scans-distribution-output-directory = Répertoire de sortie de distribution
scans-python-bytecode-cache = Cache de code intermédiaire Python
scans-remove-exclusion-tooltip = Supprimer
scans-preset-exclusions-title = Exclusions prédéfinies
scans-common-directories-exclude-description = Répertoires courants à exclure de l’analyse. Les modifications sont enregistrées immédiatement.
scans-custom-exclusions-title = Exclusions personnalisées
scans-enter-absolute-path-description = Saisissez un chemin absolu ou un motif tel que *.tmp. Les modifications sont enregistrées immédiatement.
scans-path-exclude-tmp-title = /path/to/exclude ou *.tmp
scans-prefer-clamd-when-available = Automatique
scans-always-clamd = clamd (démon)
scans-always-clamscan = clamscan (autonome)
scans-options-title = Options
scans-backend-title = Moteur d’analyse
scans-automatic-uses-daemon-subtitle = Le mode automatique utilise le démon quand il répond, et clamscan sinon.
scans-show-live-scan-title = Afficher la progression de l’analyse en temps réel
scans-display-files-scanned-subtitle = Afficher les fichiers en cours d’analyse en temps réel.
scans-not-added-history-error = L’analyse n’a pas pu être ajoutée à l’historique : { $error }
scans-no-profile-manual = Aucun profil (manuel)
scans-removable-drive-scanned-title = Un lecteur amovible est en cours d’analyse. Lancez cette analyse lorsqu’elle sera terminée.
scans-open-removable-drives-button = Ouvrir les lecteurs amovibles
scans-detecting-backend = Vérification en cours…
scans-backend-undetermined = Moteur : inconnu, les programmes d’analyse n’ont pas pu être recherchés : { $error }
scans-backend-clamd-daemon = Moteur : clamd (démon)
scans-backend-clamscan-standalone = Moteur : clamscan (autonome)
scans-scan-title = Analyse
scans-preparing-label = Préparation…
scans-cancelling-label = Annulation…
scans-scanning-name = Analyse de { $name }
scans-scanning-count-location =
    { $count ->
        [one] Analyse de { $count } emplacement
       *[other] Analyse de { $count } emplacements
    }
scans-cancelled-user = Analyse interrompue par l'utilisateur
scans-threats-found-scan = Menaces détectées
scans-not-finish-cleanly = L’analyse ne s’est pas terminée correctement
scans-no-threats-found = Aucune menace détectée
scans-not-create-eicar-test = Impossible de créer le fichier de test EICAR : { $error }
scans-add-files-tooltip = Ajouter des fichiers
scans-add-folder-manual-tooltip = Ajouter un dossier
scans-more-tooltip = Plus
scans-run-eicar-test-label = Lancer le test EICAR
scans-copy-results-label = Copier les résultats
scans-export-results-label = Exporter les résultats…
scans-start-scan-label = Lancer l’analyse
scans-cancel-manual-label = Annuler
scans-lazaret-refuses-clamav-title = { -brand-name } refuse un programme ClamAV que votre compte pourrait remplacer
scans-details-button = Détails
scans-profile-title = Profil d’analyse
scans-profile-short-title = Profil
scans-manage-profiles-tooltip = Gérer les profils
scans-targets-manual-title = Cibles de l'analyse
scans-drop-files-folders-label = Déposez des fichiers ou des dossiers à analyser
scans-add-files-label = Ajouter des fichiers…
scans-add-folder-label = Ajouter un dossier…
scans-found-far-scan-title = Trouvé jusqu’à présent
scans-new-scan-button = Nouvelle analyse
scans-recent-scans-title = Analyses récentes
scans-started-lazaret-newest-description = Analyses lancées depuis { -brand-name }, les plus récentes en premier. Sélectionnez-en une pour lire sa transcription.
scans-choose-files-scan = Choisir les fichiers à analyser
scans-choose-folder-scan = Choisir un dossier à analyser
scans-quick-scan = Analyse rapide
scans-full-scan = Analyse complète
scans-home-folder = Dossier personnel
scans-fast-scan-downloads-folder = Analyse rapide du dossier Téléchargements pour détecter rapidement les menaces
scans-comprehensive-system-wide-scan = Analyse complète de tous les répertoires accessibles du système
scans-user-home-directory-personal = Analyse du répertoire personnel et des fichiers personnels de l’utilisateur
scans-profile-file-unavailable = Impossible d’accéder au fichier de profil : { $error }
scans-profile-data-invalid = Les données du profil ne sont pas valides : { $error }
scans-profile-file-not-replaced = Le fichier de profil n’a pas pu être remplacé : { $error }
scans-profile-task-interrupted = La tâche du profil a été interrompue : { $error }
scans-profile-name-missing = Nom du profil requis
scans-profile-name-too-long = Le nom doit comporter au plus 50 caractères
scans-profile-name-taken = Un autre profil utilise déjà ce nom
scans-profile-targets-missing = Ajoutez au moins une cible
scans-edit-profile-title = Modifier le profil
scans-save-label = Enregistrer
scans-name-title = Nom
scans-description-title = Description
scans-targets-profile-title = Cibles
scans-add-folder-profile-tooltip = Ajouter un dossier
scans-add-path-title = Ajouter un chemin
scans-exclusions-title = Exclusions
scans-added-exclusions-preferences-description = Ajoutées aux exclusions des préférences pendant l’exécution de ce profil.
scans-add-pattern-title = Ajouter un motif
scans-cancel-profile = Annuler
scans-defaults-restored = Profils par défaut restaurés
scans-profile-deleted = Profil supprimé
scans-profile-imported = Profil importé
scans-manage-profiles-title = Gérer les profils
scans-restore-defaults-tooltip = Restaurer les profils par défaut
scans-new-profile-tooltip = Nouveau profil
scans-import-tooltip = Importer
scans-profiles-title = Profils d’analyse
scans-select-profile-edit-description = Sélectionnez un profil à modifier ou à utiliser pour l’analyse
scans-restore-default-profiles = Restaurer les profils par défaut
scans-restore-built-default-profiles = Restaurer les profils par défaut intégrés ? Toute personnalisation des profils par défaut sera annulée ; les profils personnalisés seront conservés.
scans-delete-profile = Supprimer le profil
scans-delete-profile-name = Supprimer le profil « { $name } » ?
scans-delete-label = Supprimer
scans-profile-exported = Profil exporté
scans-remove-profile-tooltip = Supprimer
scans-count-target =
    { $count ->
        [one] { $count } cible
       *[other] { $count } cibles
    }
scans-default-profile = Profil par défaut
scans-run-tooltip = Lancer
scans-edit-tooltip = Modifier
scans-delete-tooltip = Supprimer
scans-export-tooltip = Exporter
scans-remove-target-tooltip = Supprimer
scans-check-again-access-tooltip = Vérifier à nouveau
scans-checking-access-tooltip = Vérification
scans-status-access-title = État
scans-access-settings-changed-description = Paramètres d’analyse à l’accès modifiés via { -brand-name }, tels qu’enregistrés par le système. Les logiciels exécutés sous votre compte ne peuvent pas supprimer ces enregistrements.
scans-no-access-settings-title = Aucun paramètre d’analyse à l’accès n’a été modifié via { -brand-name }
scans-clamonacc-log-title = Journal de clamonacc
scans-clamonacc-not-written-title = clamonacc n’a rien écrit dans le journal
scans-checking-access-scanning = Vérification de l’analyse à l’accès…
scans-command-copied-clipboard-access = Commande copiée dans le presse-papiers
scans-no-readable-config = Aucun fichier clamd.conf lisible n’a été trouvé
scans-reading-task-aborted-clamd = La lecture de clamd.conf s’est arrêtée avant la fin
scans-watched-mount = { $path } (point de montage entier)
scans-nothing-watched = Aucun configuré
scans-default-limit = La valeur par défaut de ClamAV
scans-configuration-label = Configuration
scans-watched-folders = Dossiers surveillés
scans-largest-file-scanned = Plus gros fichier analysé
scans-no-kernel-config-found = Ni /proc/config.gz ni le fichier de configuration du noyau dans /boot n’existent.
scans-kernel-config-unreadable = La configuration du noyau n’a pas pu être lue.
scans-compressed-config-corrupt = /proc/config.gz n’a pas pu être décompressé.
scans-kernel-release-unreadable = La version du noyau en cours d’exécution n’a pas pu être lue.
scans-reading-task-aborted-config = La lecture de la configuration du noyau s’est arrêtée avant la fin.
scans-blocks-detected-files = Les fichiers détectés sont bloqués
scans-prevention-off = Les fichiers sont vérifiés, mais les fichiers détectés ne sont pas bloqués
scans-prevention-conflicts-mounts = Les fichiers sont vérifiés, mais les fichiers détectés ne sont pas bloqués, car OnAccessPrevention ne fonctionne pas avec OnAccessMountPath
scans-kernel-cannot-block = Les fichiers sont vérifiés, mais ce noyau ne peut pas bloquer les fichiers détectés, car il a été compilé sans CONFIG_FANOTIFY_ACCESS_PERMISSIONS
scans-kernel-support-unknown = Les fichiers sont vérifiés, mais on ignore si le noyau peut bloquer les fichiers détectés. { $reason }
scans-config-unreadable-monitor = En cours d’exécution, mais clamd.conf n’a pas pu être lu : on ignore donc ce qui est vérifié et si les fichiers détectés sont bloqués
scans-running-no-folders-status = En cours d’exécution, mais aucun dossier ni point de montage n’est surveillé
scans-running-detected-status = En cours d’exécution, mais aucun dossier ni point de montage n’est surveillé, et les fichiers détectés ne seraient pas bloqués
scans-unit-state-unknown-access = On ignore si l’analyse à l’accès est active, car systemd n’a pas pu l’indiquer
scans-never-set-up-access = L’analyse à l’accès n’est pas configurée
scans-unit-stopped-access = { $unit } n’est pas en cours d’exécution
scans-access-blocking-title = Blocage à l’accès
scans-clamonacc-checks-files-watched = clamonacc vérifie les fichiers des dossiers surveillés au moment de leur ouverture. Avec la prévention activée, le noyau refuse l’accès à un fichier détecté : c’est pourquoi un tel fichier ne peut pas être ouvert alors qu’il est toujours présent.
scans-mount-watching = Surveillance des points de montage
scans-onaccessmountpath-watches-whole-mount = OnAccessMountPath surveille un point de montage entier via fanotify. ClamAV le documente comme incompatible avec OnAccessPrevention, OnAccessIncludePath, OnAccessExcludePath et OnAccessExtraScanning : les fichiers détectés sur un point de montage surveillé ne sont donc jamais bloqués. Surveillez des dossiers avec OnAccessIncludePath pour bloquer.
scans-onaccessmountpath-cannot-combined-options = OnAccessMountPath ne peut pas être combiné avec { $options }
scans-loop-protection = Protection contre les boucles d’analyse
scans-clamd-opens-every-file = clamd ouvre chaque fichier qu’on lui demande d’analyser. Sauf si OnAccessExcludeUname ou OnAccessExcludeUID désigne l’utilisateur sous lequel clamd s’exécute, clamonacc voit ces lectures et demande à clamd d’analyser à nouveau les mêmes fichiers.
scans-clamd-not-excluded-clamonacc = clamd n’est pas exclu, clamonacc peut donc analyser ses propres lectures sans fin
scans-watching-whole-file-system = Surveillance de tout le système de fichiers
scans-clamonacc-refuses-onaccessincludepath-include = clamonacc refuse OnAccessIncludePath /. Incluez les dossiers qui reçoivent des fichiers, comme les répertoires personnels, ou surveillez un point de montage avec OnAccessMountPath.
scans-onaccessincludepath-rejected-clamonacc = OnAccessIncludePath / est rejeté par clamonacc
scans-folder-watch-limit = Limite de surveillance des dossiers
scans-path-watched-fs-inotify = { $path } est surveillé, mais fs.inotify.max_user_watches ne vaut que { $limit } : les dossiers au-delà de la limite ne sont pas vérifiés
scans-unknown-whether-path-fits = On ignore si { $path } respecte la limite de surveillance des dossiers, car fs.inotify.max_user_watches n’a pas pu être lu
scans-clamonacc-watches-every-folder = clamonacc surveille chaque dossier sous un chemin inclus avec inotify, à raison d’une surveillance par dossier. Un répertoire personnel entier ou une arborescence plus grande peut nécessiter plus de surveillances que l’ancienne limite par défaut de 8192. Augmentez fs.inotify.max_user_watches avec sysctl.
scans-device-auto-scan-title = Analyse automatique des périphériques
scans-automatically-scan-removable-description = Analyser automatiquement les lecteurs amovibles et externes lorsqu’ils sont connectés.
scans-enable-device-auto-title = Activer l’analyse automatique des périphériques
scans-max-device-size-title = Taille maximale des périphériques (Go)
scans-no-size-limit-subtitle = 0 signifie aucune limite de taille.
scans-skip-battery-device-title = Ne pas lancer sur batterie
scans-drive-scan-not-added = L’analyse du lecteur n’a pas pu être ajoutée à l’historique : { $error }
scans-device-auto-scan = Analyse automatique des périphériques
scans-device-connected-during-another = Un périphérique a été connecté pendant une autre analyse, il n’a donc pas été analysé.
scans-removable-drives-title = Lecteurs amovibles
scans-cancel-removable-label = Annuler
scans-found-far-removable-title = Trouvé jusqu’à présent
scans-drives-session-title = Lecteurs de cette session
scans-drives-connected-since-description = Lecteurs connectés depuis le démarrage de { -brand-name }, les plus récents en premier, et ce qu’il est advenu de chacun. Cette liste n’est conservée que jusqu’à la fermeture de { -brand-name }.
scans-no-drive-connected-title = Aucun lecteur n’a encore été connecté
scans-close-results-button = Fermer les résultats
scans-drive-scans-title = Analyses des lecteurs
scans-drives-lazaret-scanned-description = Lecteurs analysés par { -brand-name } lors de leur connexion, les plus récents en premier. Sélectionnez-en un pour lire sa transcription.
scans-auto-scan-off = Désactivée
scans-every-drive = Analyse tous les lecteurs
scans-drives-up-size = Analyse les lecteurs jusqu’à { $size }
scans-skipped-larger-than-limit = Non analysé : { $size } dépasse la limite de { $limit }
scans-skipped-battery = Non analysé : l’ordinateur fonctionnait sur batterie
scans-skipped-auto-scan-off = Non analysé : l’analyse automatique des périphériques est désactivée
scans-skipped-while-scanning = Non analysé : une autre analyse était en cours
scans-size-unreadable = Non analysé : la taille du lecteur n’a pas pu être lue
scans-drive-settling = Vérification du lecteur…
scans-drive-scanning = Analyse en cours…
scans-threats-detected = Analysé, menaces détectées
scans-no-threats-detected = Analysé, aucune menace détectée
scans-finished-problems = Analysé, mais l’analyse ne s’est pas terminée correctement
scans-scan-cancelled = Analyse annulée
scans-device-scan-running = Analyse de { $drive }
scans-drive-scan = Analyse du lecteur
scans-check-again-scan-tooltip = Vérifier à nouveau
scans-checking-scan-tooltip = Vérification
scans-status-scan-title = État
scans-engine-detection-settings-description = Paramètres du moteur et de détection modifiés via { -brand-name }, tels qu’enregistrés par le système. Les logiciels exécutés sous votre compte ne peuvent pas supprimer ces enregistrements.
scans-no-engine-detection-title = Aucun paramètre du moteur ou de détection n’a été modifié via { -brand-name }
scans-clamd-log-title = Journal de clamd
scans-no-clamd-log-title = Aucun journal de clamd n’a été trouvé
scans-checking-scan-engine = Vérification du moteur d’analyse…
scans-command-copied-clipboard-scan = Commande copiée dans le presse-papiers
scans-program-start-failed = clamscan n’a pas pu être démarré : { $error }
scans-probe-timed-out = clamscan n’a pas répondu à temps
scans-scanner-refused = { $path } refusé, car votre compte peut le remplacer
scans-engine-missing = ClamAV est introuvable sur cet ordinateur
scans-clamav-installed = ClamAV installé
scans-clamav-engine-recognizes-malware = ClamAV est le moteur qui reconnaît les logiciels malveillants. { -brand-name } utilise la copie installée sur cet ordinateur et n’intègre jamais la sienne.
scans-clamd-still-uses-signatures = clamd utilise encore les signatures { $loaded }, alors que { $disk } est sur le disque
scans-clamd-answering = clamd répond
scans-unit-active-not-answering = { $unit } est en cours d’exécution, mais clamd ne répond pas. Il est peut-être encore en train de charger les signatures.
scans-unit-state-unknown-daemon = clamd ne répond pas, et systemd n’a pas pu indiquer son état
scans-unit-stopped-daemon = { $unit } n’est pas en cours d’exécution
scans-never-set-up-daemon = clamd n’est pas configuré, les analyses se rabattent donc sur clamscan, plus lent
scans-same-version-disk = La même version que sur le disque
scans-loaded-disk = { $loaded }, alors que { $disk } est sur le disque
scans-unknown-loaded-stored-versions = Inconnues, les versions chargée et enregistrée n’ont pas pu être comparées
scans-loaded-signatures = Signatures chargées
scans-daemon-running = Démon en cours d’exécution
scans-clamd-keeps-signatures-loaded = clamd garde les signatures chargées en mémoire. Le blocage à l’accès et les analyses planifiées lui envoient tous deux leurs fichiers.
scans-socket-protected = Seul le système peut remplacer { $path }
scans-socket-replaceable = Votre compte peut remplacer { $replaceable } : un programme exécuté sous votre compte pourrait donc placer un faux clamd à { $path } qui déclarerait chaque fichier sain
scans-socket-missing = Aucun socket clamd à { $path } : clamd n’est pas en cours d’exécution ou écoute ailleurs
scans-socket-not-checkable = Le bac à sable Flatpak ne permet pas de voir à qui appartient le socket clamd
scans-clamd-socket-protected = Socket clamd protégé
scans-socket-mechanism = Les analyses atteignent clamd via son socket. Laissez clamd le créer dans un répertoire où seuls root ou l’utilisateur clamav peuvent écrire, comme /run/clamav, afin qu’aucun programme exécuté sous votre compte ne puisse mettre un faux clamd à sa place.
scans-socket-inspecting = Vérification de qui peut remplacer le socket clamd…
scans-timer-inactive = Non planifiée
scans-next-run-time = Prochaine exécution : { $time }
scans-scheduled-time-next-run = Planifiée. L’heure de la prochaine exécution n’est pas encore connue.
scans-systemd-unavailable = Les analyses planifiées nécessitent une session utilisateur systemd, qui n’est pas disponible.
scans-system-already-runs-scheduled = Votre système exécute déjà des analyses planifiées : ces paramètres sont donc verrouillés pour éviter une seconde planification. Prochaine analyse : { $time }. Modifiez-la dans la configuration de votre système.
scans-system-scan-timer-settings = Votre système dispose d’une minuterie d’analyse : ces paramètres sont donc verrouillés pour éviter une seconde planification. Modifiez-la dans la configuration de votre système.
scans-reading-user-timer = Vérification en cours…
scans-inspecting-system-timer = Vérification de l’existence d’analyses planifiées par votre système. Ces paramètres restent verrouillés d’ici là.
scans-system-timer-unknown = { -brand-name } n’a pas pu vérifier si votre système exécute déjà des analyses planifiées : ces paramètres sont donc verrouillés pour éviter une seconde planification : { $error }
scans-scan-time-unreadable = Saisissez l’heure de l’analyse au format HH:MM, par exemple 02:00.
scans-unit-files-unwritable = Erreur lors de la création des fichiers systemd : { $error }
scans-timer-not-applied-error = La minuterie n’a pas pu être appliquée : { $error }
scans-hourly-label = Toutes les heures
scans-daily-label = Tous les jours
scans-weekly-label = Toutes les semaines
scans-monthly-label = Tous les mois
scans-monday-label = Lundi
scans-tuesday-label = Mardi
scans-wednesday-label = Mercredi
scans-thursday-label = Jeudi
scans-friday-label = Vendredi
scans-saturday-label = Samedi
scans-sunday-label = Dimanche
scans-schedule-title = Planification
scans-apply-label = Appliquer
scans-enable-scheduled-scans-title = Activer les analyses planifiées
scans-frequency-title = Fréquence des analyses
scans-time-24-hour-title = Heure de l’analyse (format 24 heures)
scans-day-week-title = Jour de la semaine
scans-used-weekly-scans-subtitle = Utilisé pour les analyses hebdomadaires.
scans-day-month-title = Jour du mois
scans-used-monthly-scans-subtitle = Utilisé pour les analyses mensuelles.
scans-targets-separated-commas-title = Cibles de l’analyse (séparées par des virgules ; vide pour analyser votre dossier personnel)
scans-skip-battery-schedule-title = Ne pas lancer sur batterie
scans-not-start-scheduled-subtitle = Ne pas lancer d’analyses planifiées lorsque l’appareil fonctionne sur batterie.
scans-auto-quarantine-title = Mise en quarantaine automatique
scans-automatically-quarantine-threats-subtitle = Mettre automatiquement en quarantaine les menaces détectées par les analyses planifiées. Tant que { -brand-name } est ouvert, les menaces sont seulement signalées.
scans-review-button = Examiner
scans-check-again-scheduled-tooltip = Vérifier à nouveau
scans-checking-scheduled-tooltip = Vérification
scans-system-scan-title = Analyse du système
scans-folders-scanned-title = Dossiers analysés
scans-run-now-label = Lancer maintenant
scans-starting-scan-tooltip = Démarrage de l’analyse…
scans-status-scheduled-title = État
scans-scheduled-runs-title = Exécutions planifiées
scans-lazaret-schedule-ran-description = Analyses exécutées par la planification de { -brand-name }, les plus récentes en premier. Sélectionnez-en une pour lire sa transcription.
scans-checking-schedule = Vérification de la planification…
scans-command-copied-clipboard-scheduled = Commande copiée dans le presse-papiers
scans-not-scheduled = Non planifiée
scans-next-scan = Prochaine analyse
scans-threats-found-scheduled = menaces détectées
scans-nothing-found = rien de détecté
scans-no-completed-scan = Pas encore exécutée
scans-last-scan = Dernière analyse
scans-unknown-label = Inconnu
scans-system-timer = La minuterie du système n’est pas active
scans-last-scan-ended-error = La dernière analyse s’est terminée par une erreur
scans-next-scan-time = Prochaine analyse : { $time }
scans-timer-starts-scan-service = La minuterie démarre un service d’analyse qui n’est pas installé
scans-service-not-read-systemd = Le service d’analyse n’a pas pu être lu depuis systemd
scans-lazaret-schedule = Planifiée par { -brand-name }
scans-no-schedule = Aucune analyse n’est planifiée
scans-not-scheduled-timer-active = Non planifiée, car la minuterie n’est pas active
scans-folders-scanned = Dossiers analysés
scans-scheduled-scan = Analyse planifiée
scans-scheduled-scan-catches-files = Une analyse planifiée repère les fichiers qui n’ont pas été ouverts, ou qui ne sont devenus détectables qu’après une mise à jour des signatures.
scans-unit-names-no-folders = L’unité d’analyse ne désigne aucun dossier
scans-system-scan-service-unit = Le service d’analyse de votre système, { $unit }, analyse ces dossiers avec les paramètres propres au système. Son exécution nécessite une autorisation d’administrateur.
scans-needs-daemon-see-scan = Nécessite le démon, voir Moteur d’analyse
scans-bus-call-failed = systemd n’a pas répondu : { $error }
scans-unrecognized-load-state = systemd a signalé un état d’unité que { -brand-name } ne reconnaît pas
scans-found-malware = L’analyse a détecté des logiciels malveillants
scans-unit-finished = L’analyse est terminée
scans-not-finish-cleanly-result = L’analyse ne s’est pas terminée correctement : { $result }
scans-authorization-refused = Le démarrage de l’analyse nécessite une autorisation d’administrateur
scans-not-started-error = L’analyse n’a pas pu être démarrée : { $error }

findings-open-preferences = Ouvrir les préférences
findings-journal-unreadable-alerts = Le journal du système est illisible. Ajoutez votre utilisateur au groupe systemd-journal.
findings-notifications-turned-off-preferences = Les notifications sont désactivées dans les préférences
findings-alerts-stop-quit-lazaret = Les alertes s’arrêtent quand vous quittez { -brand-name }, car il n’est pas configuré pour fonctionner en arrière-plan
findings-running-background-following-journal = Fonctionne en arrière-plan et suit le journal
findings-alerts-label = Alertes
findings-lazaret-follows-system-journal = { -brand-name } suit le journal du système et vous avertit lorsque quelque chose est détecté. Il ne peut le faire que lorsqu’il est en cours d’exécution : il doit donc fonctionner en arrière-plan.
findings-notifications-on = Activées
findings-notifications-off = Désactivées
findings-running-label = En cours d’exécution
findings-not-running = Arrêté
findings-unit-not-found = Non configuré
findings-systemd-unavailable = Inconnu, systemd est injoignable
findings-following-journal-access = Suivi
findings-journal-unreadable-access = Illisible
findings-following-failed = Suivi arrêté
findings-notifications-label = Notifications
findings-background-service = Service d’arrière-plan
findings-system-journal = Journal du système
findings-attach-unknown-setting = On ignore si d’autres programmes peuvent s’attacher, car le paramètre du noyau est illisible : { $reason }
findings-attach-unknown-reports = On ignore si d’autres programmes peuvent s’attacher, car le noyau indique une valeur que { -brand-name } ne reconnaît pas
findings-attach-unknown-check = On ignore si d’autres programmes peuvent s’attacher, car la vérification s’est arrêtée avant la fin
findings-attach-restricted = Les programmes ne peuvent pas s’attacher à d’autres programmes que vous exécutez, comme { -brand-name }
findings-any-program-may-attach = N’importe quel programme que vous exécutez peut s’attacher à { -brand-name } et modifier ce qu’il affiche, y compris les détections et les décisions que vous prenez ici
findings-yama-not-loaded = Ce noyau ne restreint pas l’attachement : n’importe quel programme que vous exécutez peut donc s’attacher à { -brand-name } et modifier ce qu’il affiche
findings-other-programs-cannot-attach = D’autres programmes ne peuvent pas s’attacher à { -brand-name }
findings-program-attaches-lazaret-hide = Un programme qui s’attache à { -brand-name } peut masquer des alertes ou modifier vos décisions. Régler kernel.yama.ptrace_scope sur 1 ne permet qu’au parent d’un programme de s’y attacher.
findings-file-owned-other-user = Ce fichier appartient à un autre utilisateur : il ne peut donc pas être déplacé ni supprimé d’ici
findings-file-not-checkable = Ce fichier n’a pas pu être vérifié : il ne peut donc pas être déplacé ni supprimé d’ici
findings-removal-failed = Le fichier n’a pas pu être supprimé : { $error }
findings-file-check-interrupted = La vérification du fichier s’est arrêtée de façon inattendue
findings-history-not-updated = Votre décision n’a pas pu être enregistrée : { $error }
findings-journal-unreadable-list = { -brand-name } ne peut pas lire le journal du système : les nouvelles détections en arrière-plan n’apparaîtront donc pas ici. Cela ne signifie pas que rien n’a été trouvé. Ajoutez votre utilisateur au groupe systemd-journal.
findings-loading-detections = Chargement des détections…
findings-nothing-detected-background = Rien n’a été détecté en arrière-plan.
findings-access-scanning-scheduled-scans = Ce que l’analyse à l’accès et les analyses planifiées ont trouvé.
findings-detections-title = Détections
findings-back-detections-tooltip = Retour aux détections
findings-alerts-title = Alertes
findings-detections-reach-keeps-description = Comment les détections vous parviennent, et ce qui empêche d’autres programmes de les masquer.
findings-check-again-background-tooltip = Vérifier à nouveau
findings-checking-background-tooltip = Vérification
findings-detection-title = Détection
findings-copied-clipboard-toast = Copié dans le presse-papiers
findings-command-copied-clipboard-toast = Commande copiée dans le presse-papiers
findings-detections-lazaret-stopped = Les détections survenues pendant que { -brand-name } n’était pas en cours d’exécution ne peuvent pas être comptées, car l’endroit où il a cessé de lire le journal du système n’a pas pu être lu : { $error }
findings-detections-not-read-error = Les détections n’ont pas pu être lues : { $error }
findings-malware-blocked-signature = Logiciel malveillant bloqué : { $signature }
findings-malware-found-signature = Logiciel malveillant détecté : { $signature }
findings-count-detection-not-running =
    { $count ->
        [one] { $count } détection pendant que { -brand-name } n’était pas en cours d’exécution
       *[other] { $count } détections pendant que { -brand-name } n’était pas en cours d’exécution
    }
findings-detections-not-running-detection = Les détections survenues pendant que { -brand-name } n’était pas en cours d’exécution ne peuvent pas être comptées
findings-lazaret-not-read-stopped = { -brand-name } n’a pas pu lire l’endroit où il a cessé de suivre le journal du système : il ne surveille donc que les nouvelles détections. Consultez la page Détections et le journal du système pour tout ce qui a été trouvé pendant qu’il était fermé.
findings-quarantine-incident = Quarantaine
findings-ask-incident-deletion = Supprimer…
findings-delete-incident-file = Supprimer définitivement
findings-cancel-incident-deletion = Annuler
findings-dismiss-incident = Conserver
findings-look-up-incident-signature = Rechercher
findings-submit-incident-sample = Envoyer…
findings-restored-quarantine = Restauré depuis la quarantaine
findings-moved-quarantine = Déplacé en quarantaine
findings-deleted-label = Supprimé
findings-kept-label = Conservé par vous
findings-belongs-another-user = Appartient à un autre utilisateur
findings-blocked-needs-decision = Bloqué, décision requise
findings-not-blocked-needs-decision = Non bloqué, décision requise
findings-needs-decision = Décision requise
findings-no-longer-there-detection = N’est plus présent
findings-state-unknown-detection = Le fichier n’a pas pu être vérifié
findings-signature-test-file-state = { $signature } (fichier de test) · { $state }
findings-signature-state = { $signature } · { $state }
findings-access-schedule = L’analyse à l’accès et l’analyse planifiée
findings-access-only = Analyse à l’accès
findings-schedule-only = L’analyse planifiée
findings-file-label = Fichier
findings-signature-source = Source de la signature
findings-found-label = Détecté par
findings-last-seen = Vu pour la dernière fois
findings-times-seen = Nombre de détections
findings-detected-label = Détecté
findings-signature-matches-harmless-eicar = Cette signature correspond au motif de test EICAR inoffensif, qui sert à vérifier que la détection fonctionne.
findings-clamav-matched-file-against = ClamAV a fait correspondre ce fichier à une signature. Une correspondance est une preuve solide, mais les faux positifs existent, en particulier avec les outils compressés ou d’administration.
findings-signature-first-seen-time = { $signature }, vu pour la première fois { $local-time }
findings-opening-file-refused-why = L’ouverture de ce fichier est refusée, c’est pourquoi il ne s’ouvre pas
findings-not-opened-again-after = Pas rouvert après votre décision
findings-no-longer-there-blocked = Le fichier n’est plus présent
findings-not-blocked-scheduled-scan = Non bloqué : l’analyse planifiée l’a détecté, et il peut encore être ouvert
findings-not-blocked-right-now = Non bloqué pour l’instant : il peut encore être ouvert
findings-state-unknown-blocked = Impossible de vérifier si le fichier est bloqué
findings-blocked-label = Bloqué
findings-access-prevention-kernel-refuses = Avec la prévention à l’accès, le noyau refuse d’ouvrir un fichier détecté dans un dossier surveillé. Le fichier reste où il est jusqu’à ce que vous décidiez quoi en faire.
findings-looking-up = Recherche…
findings-no-known-samples-match = Aucun échantillon connu ne correspond à cette signature.
findings-unknown-label = inconnu
findings-count-known-samples-families = { $count } échantillons connus. Familles : { $families }. Première observation : { $date }.
findings-ask-malwarebazaar-signature-description = Interroger MalwareBazaar sur cette signature. Seul son nom est envoyé.
findings-set-up-lookup-button = Configurer…
findings-bazaar-no-key = Aucune clé
findings-bazaar-key-unreadable = Clé illisible
findings-stop-asking-file-description = Ne plus demander pour ce fichier
findings-delete-file-permanently-description = Supprimer définitivement ce fichier ? Cette action est irréversible.
findings-go-back-without-description = Revenir sans supprimer
findings-move-somewhere-safe-description = Le déplacer en lieu sûr. Vous pourrez le restaurer plus tard.
findings-remove-good-description = Le supprimer définitivement
findings-know-file-fine-description = Vous savez que ce fichier est sain. ClamAV le signale encore à la prochaine analyse ; approuvez-le depuis la quarantaine ou un résultat d’analyse pour que cela cesse.
findings-decision-title = Votre décision
findings-unfamiliar-quarantine-keeping-file = S’il ne vous est pas familier, mettez-le en quarantaine. Conserver un fichier empêche seulement { -brand-name } de vous le demander : l’analyse à l’accès continue de le bloquer jusqu’à ce que son chemin soit exclu dans la configuration de ClamAV, comme indiqué ici.
findings-quarantined-files-restored-quarantine = Les fichiers en quarantaine peuvent être restaurés depuis la page Quarantaine. Un fichier que vous avez conservé reste bloqué par l’analyse à l’accès jusqu’à ce que son chemin soit exclu dans la configuration de ClamAV.
findings-file-other-owner-decision = Ce fichier appartient à un autre utilisateur : { -brand-name } ne peut donc pas le déplacer ni le supprimer
findings-state-unknown-review = { -brand-name } n’a pas pu vérifier ce fichier : il ne peut donc pas le déplacer ni le supprimer
findings-choose-happens-file = Choisissez ce qu’il advient de ce fichier
findings-malwarebazaar-label = MalwareBazaar
findings-resolved-title = Résolue
findings-resolved-detection-opens-again = Une détection résolue est rouverte si ClamAV retrouve plus tard le même fichier, par exemple si vous le téléchargez une seconde fois.
findings-waiting-decision = En attente de votre décision
findings-resolved-state-step = { $state }, { $time }
findings-send-quarantined-copy-description = Envoyer la copie en quarantaine à ClamAV, MalwareBazaar ou VirusTotal. Un faux positif signalé à ClamAV est corrigé pour tout le monde par une mise à jour ultérieure des signatures.
findings-cancel-quarantine = Annuler
findings-delete-entries = Supprimer
findings-scanner-program-not-installed = Le programme d’analyse n’est pas installé.
findings-scanner-program-replaced-account = Le programme d’analyse situé à { $program } peut être remplacé par votre compte via { $path }.
findings-content-pipe-failed = Le fichier n’a pas pu être transmis au programme d’analyse : { $error }
findings-scanner-spawn-failed = Le programme d’analyse n’a pas pu être démarré : { $error }
findings-verdict-timed-out = L’analyse a pris trop de temps.
findings-detection-settings-unreadable = Les paramètres de détection de clamd n’ont pas pu être lus : { $error }
findings-program-lookup-failed = Les programmes d’analyse n’ont pas pu être recherchés : { $error }
findings-payload-unreadable = La copie en quarantaine n’a pas pu être lue : { $error }
findings-restore-file-heading = Restaurer ce fichier ?
findings-clamav-no-longer-verdict = ClamAV ne détecte plus ce fichier avec les signatures actuelles.
findings-restore-button = Restaurer
findings-restore-file-clamav-heading = Restaurer un fichier que ClamAV détecte encore ?
findings-clamav-still-detects-verdict = ClamAV détecte toujours ce fichier comme { $detection } avec les signatures actuelles. Ne le restaurez que si vous êtes sûr qu’il s’agit d’un faux positif, et envisagez d’abord de l’approuver.
findings-clamav-still-detects-unnamed-verdict = ClamAV détecte toujours ce fichier avec les signatures actuelles, sans nommer la détection. Ne le restaurez que si vous êtes sûr qu’il s’agit d’un faux positif, et envisagez d’abord de l’approuver.
findings-restore-anyway-button = Restaurer quand même
findings-restore-unchecked-file-heading = Restaurer un fichier non vérifié ?
findings-file-not-checked-verdict = Le fichier n’a pas pu être revérifié avant sa restauration. { $reason }
findings-location-outside-home-folder = Cet emplacement se trouve en dehors de votre dossier personnel.
findings-file-goes-back = Le fichier retourne à :
findings-entry-count =
    { $count ->
        [one] { $count } élément
       *[other] { $count } éléments
    }
findings-clear-old-items = Effacer les anciens éléments ?
findings-count-quarantined-file-older =
    { $count ->
        [one] { $count } fichier en quarantaine datant de plus de 30 jours sera définitivement supprimé.
       *[other] { $count } fichiers en quarantaine datant de plus de 30 jours seront définitivement supprimés.
    }
findings-delete-quarantined-file = Supprimer le fichier en quarantaine ?
findings-file-permanently-deleted = Le fichier sera définitivement supprimé.
findings-quarantine-title = Quarantaine
findings-more-tooltip = Plus
findings-clear-old-items-label = Effacer les anciens éléments…
findings-check-again-label = Vérifier à nouveau
findings-scan-every-quarantined-tooltip = Analyser chaque fichier en quarantaine avec les signatures actuelles
findings-refresh-tooltip = Actualiser
findings-search-tooltip = Rechercher
findings-search-threat-name-placeholder = Rechercher par nom de menace ou par chemin
findings-quarantine-storage-title = Stockage en quarantaine
findings-detected-threats-isolated-description = Menaces détectées isolées de votre système. Développez une entrée pour vérifier sa réputation.
findings-total-size-title = Taille totale
findings-dismiss-label = Ignorer
findings-quarantine-cannot-read-title = La quarantaine est illisible
findings-loading-quarantine = Chargement de la quarantaine…
findings-quarantine-empty = La quarantaine est vide
findings-hash-copied-clipboard = Empreinte copiée dans le presse-papiers
findings-every-quarantined-file-still = Chaque fichier en quarantaine est encore détecté ou n’a pas pu être vérifié
findings-count-quarantined-file-no =
    { $count ->
        [one] { $count } fichier en quarantaine n’est plus détecté. Vérifiez-le avant de le restaurer.
       *[other] { $count } fichiers en quarantaine ne sont plus détectés. Vérifiez-les avant de les restaurer.
    }
findings-count-report-clamav-settled =
    { $count ->
        [one] { $count } signalement à ClamAV est réglé par les nouvelles signatures
       *[other] { $count } signalements à ClamAV sont réglés par les nouvelles signatures
    }
findings-checking-label = Vérification en cours…
findings-no-longer-detected = N’est plus détecté
findings-still-detected = Toujours détecté
findings-integrity-unknown-hash = Inconnue. Cet enregistrement de quarantaine n’a pas d’empreinte utilisable : le fichier ne peut donc pas être vérifié, contrôlé en ligne, approuvé ni restauré. Vous pouvez toujours le supprimer.
findings-not-checked = Pas encore vérifié
findings-integrity-unknown-label = Intégrité inconnue
findings-restore-label = Restaurer
findings-delete-entry-label = Supprimer
findings-reputation-title = Réputation
findings-look-up-reputation-button = Rechercher
findings-set-up-source-button = Configurer…
findings-reputation-no-key = Aucune clé
findings-reputation-key-unreadable = Clé illisible
findings-file-sha-256-subtitle = Seule l’empreinte SHA-256 du fichier est envoyée. Le fichier lui-même ne quitte jamais cet ordinateur.
findings-view-report-label = Voir le rapport
findings-false-positive-title = Faux positif
findings-trust-stops-clamav-subtitle = Approuver… empêche ClamAV de signaler ce fichier précis sur cet ordinateur et l’ajoute aux détections approuvées ; restaurez-le ensuite. Signaler… l’envoie à Cisco Talos, afin qu’une mise à jour ultérieure des signatures corrige la détection pour tout le monde.
findings-report-label = Signaler…
findings-submit-quarantine-label = Envoyer…
findings-send-clamav-malwarebazaar-tooltip = L’envoyer à ClamAV, MalwareBazaar ou VirusTotal
findings-trust-label = Approuver…
findings-copy-hash-tooltip = Copier l’empreinte
findings-submit-quarantined-label = Envoyer…
findings-service-label = Service
findings-status-label = État
findings-quarantine-record-no-usable = Cet enregistrement de quarantaine n’a pas d’empreinte utilisable : { -brand-name } ne l’envoie donc nulle part
findings-file-no-longer-quarantine = Ce fichier n’est plus en quarantaine
findings-service-not-take-detection = Ce service n’accepte pas cette détection
findings-file-no-usable-hash = Ce fichier n’est plus en quarantaine avec une empreinte utilisable
findings-check-malwarebazaar-virustotal-tooltip = Rechercher des nouvelles sur MalwareBazaar et VirusTotal
findings-checking-sent-tooltip = Vérification
findings-send-file-title = Envoyer un fichier
findings-nothing-leaves-computer-description = Rien ne quitte cet ordinateur tant que vous n’avez pas confirmé ce qui est envoyé.
findings-report-missed-malware-label = Signaler un logiciel malveillant manqué…
findings-send-clamav-malicious-tooltip = Envoyer à ClamAV un fichier malveillant qu’il ne détecte pas
findings-sent-title = Envoyés
findings-lazaret-sent-became-description = Ce que { -brand-name } a envoyé et ce qu’il en est advenu.
findings-filter-sent-samples-tooltip = Filtrer les échantillons envoyés
findings-online-services-title = Services en ligne
findings-each-service-receives-subtitle = Ce que reçoit chaque service, et les clés MalwareBazaar et VirusTotal
findings-choose-file-clamav-misses = Choisir le fichier que ClamAV ne détecte pas
findings-false-positive = Faux positif
findings-missed-malware = Logiciel malveillant manqué
findings-malware-sample = Échantillon de logiciel malveillant
findings-clamav-false-positive = Signaler un faux positif à ClamAV
findings-clamav-missed-malware = Signaler un logiciel malveillant manqué à ClamAV
findings-malwarebazaar-sample-choice = Téléverser sur MalwareBazaar
findings-virustotal-sample-choice = Téléverser sur VirusTotal
findings-awaiting-clamav = En attente de l’examen de ClamAV. Les examens prennent au moins 48 heures ; une mise à jour ultérieure des signatures en montre le résultat.
findings-fixed-database-version = Corrigé dans la version { $version } de la base de données
findings-fixed-current-signature-database = Corrigé dans la base de signatures actuelle
findings-detected-since-database-version = Détecté depuis la version { $version } de la base de données
findings-detected-current-signature-database = Détecté par la base de signatures actuelle
findings-uploaded-waiting-malwarebazaar-list = Téléversé ; en attente de son référencement par MalwareBazaar
findings-listed-malwarebazaar = Référencé sur MalwareBazaar
findings-malwarebazaar-refused-reason = MalwareBazaar l’a refusé : { $reason }
findings-queued-analysis-virustotal = En file d’attente pour analyse sur VirusTotal
findings-count-total-engine-flags =
    { $count ->
        [one] { NUMBER($count) } moteur sur { $total } le signale comme malveillant
       *[other] { NUMBER($count) } moteurs sur { $total } le signalent comme malveillant
    }
findings-virustotal-refused-reason = VirusTotal l’a refusé : { $reason }
findings-service-kind-hash-date = { $service } · { $kind } · { $hash } · { $date }
findings-integrity-unknown-candidate = { $detection } · Aucune empreinte utilisable : { -brand-name } ne peut donc pas prouver ce qu’il enverrait
findings-all-services = Tous les services
findings-any-status-option = Tous les états
findings-waiting-label = En attente
findings-settled-label = Réglé
findings-refused-label = Refusé
findings-all-submissions-option = Tous les envois
findings-listing-loading = Chargement des envois…
findings-nothing-submitted = Rien n’a été envoyé
findings-no-submission-matches-filters = Aucun envoi ne correspond aux filtres
findings-candidates-loading = Chargement de la quarantaine…
findings-quarantine-empty-sent = La quarantaine est vide
findings-no-upload-changed = Aucun téléversement n’a changé
findings-count-upload-new-status =
    { $count ->
        [one] { $count } téléversement a un nouvel état
       *[other] { $count } téléversements ont un nouvel état
    }
findings-choose-service-heading = Où envoyer ce fichier ?
findings-set-up-keys-response = Configurer les clés…
findings-each-service-gets-file = Chaque service reçoit le fichier lui-même. L’étape suivante montre exactement ce qui est envoyé et qui peut le voir.
findings-cancel-submission = Annuler
findings-open-remote-entry-tooltip = Ouvrir l’entrée distante
findings-same-upload-running = Ce fichier est déjà en cours d’envoi à ce service. Attendez la fin avant de l’envoyer à nouveau.
findings-check-again-trusted-tooltip = Vérifier à nouveau
findings-checking-trusted-tooltip = Vérification
findings-status-title = État
findings-files-trusted-detections-description = Fichiers approuvés et détections ignorées ou restaurées via { -brand-name }, tels qu’enregistrés par le système. Les logiciels exécutés sous votre compte ne peuvent pas supprimer ces enregistrements.
findings-no-trusted-detections-title = Aucune détection approuvée n’a été modifiée via { -brand-name }
findings-checking-trusted-detections = Vérification des détections approuvées…
findings-command-copied-clipboard = Commande copiée dans le presse-papiers
findings-other-allowlist = Autre liste d’autorisation
findings-file-reason = { $file } : { $reason }
findings-trusted-files = Fichiers approuvés
findings-ignored-detections = Détections ignorées
findings-allowlist-not-write-status = Une liste d’autorisation que { -brand-name } n’a pas écrite demande à ClamAV de rester silencieux
findings-count-file-detection-status =
    { $count ->
        [one] { $count } fichier ou détection n’est jamais signalé
       *[other] { $count } fichiers ou détections ne sont jamais signalés
    }
findings-own-allowlist-unreadable-status = { -brand-name } ne peut pas lire sa propre liste d’autorisation : il ne peut donc pas savoir sur quoi ClamAV reste silencieux
findings-folder-unreadable = { -brand-name } ne peut pas lire le dossier des signatures : il ne peut donc pas savoir sur quoi ClamAV reste silencieux
findings-signature-folder = Dossier des signatures
findings-trusted-detections = Détections approuvées
findings-trust-list-mechanism = Les fichiers et noms de détection figurant sur les listes d’autorisation de ClamAV ne sont jamais signalés, quelle que soit l’analyse. Un logiciel malveillant qui obtient des droits d’administrateur pourrait se cacher ainsi : vérifiez donc que chaque entrée a été ajoutée volontairement.
findings-clamav-reports-everything-detects = ClamAV signale tout ce qu’il détecte
findings-remove-tooltip = Supprimer
findings-access-denied = Permission refusée
findings-other-read-failure = Échec de la lecture : { $reason }
findings-reading-task-aborted = La lecture s’est arrêtée avant la fin
findings-journal-unreadable-added = On ignore qui a ajouté ceci. { $reason }
findings-known-user = Ajouté { $local-time } par l’utilisateur { $uid }
findings-unknown-user = Ajouté { $time } par un utilisateur inconnu
findings-no-record-added-system = Aucune trace dans le journal du système de qui a ajouté ceci
findings-count-entry-not-added =
    { $count ->
        [one] { $count } entrée, non ajoutée via { -brand-name }
       *[other] { $count } entrées, non ajoutées via { -brand-name }
    }
findings-entries-cannot-counted-reason = Les entrées ne peuvent pas être comptées ({ $reason }), non ajoutées via { -brand-name }
findings-list-unreadable = { -brand-name } ne peut pas lire cette liste
findings-no-trusted-files = Aucun fichier approuvé
findings-no-ignored-detections = Aucune détection ignorée
findings-lazaret-cannot-read-signature = { -brand-name } ne peut pas lire le dossier des signatures
findings-trusted-detections-title = Détections approuvées
findings-trusted-by-you-title = Approuvées par vous
findings-trusted-by-you-description = Vos analyses ne vous montrent pas ces fichiers, et ni vos analyses ni les alertes en arrière-plan ne montrent ces détections. ClamAV lui-même les détecte toujours : l’analyse à l’accès peut donc encore bloquer ces fichiers et les autres utilisateurs les voient toujours. Toute modification d’un fichier approuvé le fait réapparaître.
findings-nothing-trusted-by-you = Vous n’avez rien approuvé
findings-trust-by-you-unreadable = { -brand-name } ne peut pas lire ce que vous avez approuvé : { $error }
findings-file-trusted-by-you = Un fichier que vous avez approuvé
findings-trust-for-all-users-tooltip = Approuver pour tous les utilisateurs…
findings-trusted-for-all-users-title = Approuvées pour tous les utilisateurs
findings-clamav-never-reports-description = ClamAV ne signale jamais ce qui est listé ici, pour aucun utilisateur et dans aucune analyse sur cet ordinateur. Chaque modification nécessite un mot de passe d’administrateur et est enregistrée dans le journal du système, où les logiciels exécutés sous votre compte ne peuvent pas supprimer l’enregistrement.
findings-ignored-detections-title = Détections ignorées
findings-detection-names-clamav-description = Noms de détection que ClamAV ignore dans chaque fichier, pour chaque utilisateur. { -brand-name } ne les ajoute plus, car ils masquent chaque fichier portant cette détection ; supprimez-les et approuvez plutôt les fichiers par leur empreinte.
findings-other-allowlists-title = Autres listes d’autorisation
findings-allowlist-files-signature-description = Fichiers de liste d’autorisation dans le dossier des signatures que { -brand-name } n’a pas écrits. Vérifiez que vous ou votre administrateur les avez ajoutés volontairement.

signatures-history-load-failed = Échec du chargement de l’historique des journaux : { $error }
signatures-database-update-finished = Mise à jour de la base de données terminée
signatures-check-again-database-tooltip = Vérifier à nouveau
signatures-checking-database-tooltip = Vérification
signatures-update-history-title = Historique des mises à jour
signatures-updates-started-lazaret-description = Mises à jour lancées depuis { -brand-name }, les plus récentes en premier. Sélectionnez-en une pour lire sa transcription.
signatures-historical-logs-cannot-title = L’historique des journaux est illisible
signatures-status-title = État
signatures-loading-history = Chargement de l’historique…
signatures-no-updates-started-lazaret = Aucune mise à jour n’a encore été lancée depuis { -brand-name }
signatures-checking-signatures = Vérification des signatures…
signatures-command-copied-clipboard-database = Commande copiée dans le presse-papiers
signatures-update-not-added-history = La mise à jour n’a pas pu être ajoutée à l’historique : { $error }
signatures-update-signatures = Mettre à jour
signatures-cancel-signature-update = Annuler
signatures-definitions-built-date = Base virale générée { $relative-time } ({ $date })
signatures-definitions-exist-not-determined = La base virale existe, mais son âge n’a pas pu être déterminé
signatures-no-virus-definitions-found = Aucune base virale n’a été trouvée
signatures-age-definitions-unknown-checking = L’âge de la base virale est inconnu, car sa vérification s’est arrêtée avant la fin
signatures-last-update-failed = Sa dernière mise à jour a échoué
signatures-no-completed-update = Aucune mise à jour terminée enregistrée pour l’instant
signatures-updater-updated = Mis à jour par { $updater }
signatures-provider-fetched = Récupéré depuis { $provider }
signatures-count-signature-size-file =
    { $count ->
        [one] { $count } signature, { $size } ({ $file })
       *[other] { $count } signatures, { $size } ({ $file })
    }
signatures-file-unreadable = Nombre de signatures inconnu, le fichier n’a pas pu être lu : { $error }, { $size } ({ $file })
signatures-other-signature-sources = Autres sources de signatures
signatures-unknown-reason-database = Inconnu. { $reason }
signatures-fetch-newest-signatures = Récupérer maintenant les signatures les plus récentes depuis chaque outil de mise à jour
signatures-manual-update = Mise à jour manuelle
signatures-update-service = Service de mise à jour
signatures-database-folder = Dossier de la base de données
signatures-updater-output = Sortie de l’outil de mise à jour
signatures-current-label = Signatures à jour
signatures-signature-mechanism = Les signatures décrivent les logiciels malveillants connus, et de nouvelles sont publiées en permanence. D’anciennes signatures manquent les menaces récentes même quand tout le reste fonctionne.
signatures-updating-signatures = Mise à jour des signatures…
signatures-update-cancelling = Annulation de la mise à jour…
signatures-starting-update = Démarrage de la mise à jour…
signatures-daemon-running = Actif
signatures-timer-scheduled = Planifiée par une minuterie systemd
signatures-service-stopped = Installé mais arrêté
signatures-service-not-found = Aucun service freshclam détecté ; les mises à jour sont exécutées manuellement
signatures-systemd-unreachable-service = systemd n’a pas pu être interrogé sur un service freshclam ; les mises à jour sont exécutées manuellement
signatures-count-day-old =
    { $count ->
        [one] il y a { $count } jour
       *[other] il y a { $count } jours
    }
signatures-definitions-missing = Aucune base de données virale lisible n’a été trouvée
signatures-directory-unreadable = Inconnu. { $reason }
signatures-scanner-error = freshclam a échoué avec le code de sortie { $code }.
signatures-killed-signal = freshclam a été arrêté par le signal { $signal }.
signatures-unrecognized-wait-status = freshclam s’est terminé d’une manière que { -brand-name } ne reconnaît pas.
signatures-wait-failed = { -brand-name } a perdu la trace de freshclam avant la fin.
signatures-authorization-refused = Échec de l’autorisation. Veuillez réessayer et saisir votre mot de passe.
signatures-systemd-not-reached = systemd est injoignable pour exécuter la mise à jour.
signatures-service-job-ended = Le service de mise à jour ne s’est pas terminé ; systemd a signalé : { $result }
signatures-update-rate-limited-mirror = Mise à jour limitée par le miroir. Veuillez patienter quelques minutes et réessayer.
signatures-update-blocked-cdn = Mise à jour bloquée par le CDN, peut-être en raison d’une limitation de débit. Veuillez patienter et réessayer plus tard.
signatures-mirror-unavailable = Le miroir ClamAV est actuellement indisponible. Veuillez réessayer plus tard.
signatures-certificate-rejected = Erreur de certificat SSL/TLS. Le miroir présente peut-être des problèmes de configuration.
signatures-connection-timed-out = Délai de connexion dépassé. Veuillez vérifier votre connexion réseau.
signatures-database-locked = La base de données est verrouillée. Une autre instance de freshclam est peut-être en cours d’exécution.
signatures-permission-denied = Permission refusée. Des privilèges élevés sont peut-être nécessaires pour mettre à jour la base de données.
signatures-connection-failed = Erreur de connexion. Veuillez vérifier votre connexion réseau.
signatures-name-resolution-failed = Échec de la résolution DNS. Veuillez vérifier vos paramètres réseau.
signatures-update-failed-unknown-error = La mise à jour a échoué avec une erreur inconnue. Consultez les journaux pour plus de détails.
signatures-updated-databases = Mis à jour : { $databases }
signatures-already-current-databases = Déjà à jour : { $databases }
signatures-cooldown-unknown = Débit limité : { $database }
signatures-cooldown-until = Débit limité : { $database } jusqu’à { $cooldown }
signatures-definitions-updated = Mise à jour de la base de données terminée
signatures-already-current = La base virale est déjà à jour
signatures-service-ran-without-verdict = Le service de mise à jour s’est exécuté, mais son journal n’indique pas si la base virale a changé.
signatures-update-requested-daemon = Le service a accepté le signal ; il effectue la mise à jour de manière asynchrone. Consultez ses journaux pour savoir si elle est terminée.
signatures-update-cancelled = Mise à jour de la base virale annulée
signatures-scanner-program-missing = Installez freshclam avant de demander des mises à jour de la base de données.
signatures-provider-needs-credential-fangfrisch =
    { $credential ->
        [purchase-receipt] { $provider } nécessite un numéro de reçu d’achat, et fangfrisch ne le lit que dans ses paramètres. Sous NixOS, services.clamav.fangfrisch.settings est copié dans le Nix store, que tout utilisateur de cet ordinateur peut lire : le numéro de reçu d’achat serait donc exposé. Laissez { $provider } désactivé, ou exécutez fangfrisch avec un fichier de paramètres conservé hors du Nix store, comme un secret agenix ou sops-nix.
       *[customer-identifier] { $provider } nécessite un identifiant client, et fangfrisch ne le lit que dans ses paramètres. Sous NixOS, services.clamav.fangfrisch.settings est copié dans le Nix store, que tout utilisateur de cet ordinateur peut lire : l’identifiant client serait donc exposé. Laissez { $provider } désactivé, ou exécutez fangfrisch avec un fichier de paramètres conservé hors du Nix store, comme un secret agenix ou sops-nix.
    }
signatures-provider-needs-credential-website =
    { $credential ->
        [purchase-receipt] { $provider } nécessite un numéro de reçu d’achat obtenu sur son site web. fangfrisch le lit dans son fichier de paramètres, généralement { $path }. Ajoutez-y ces lignes, remplacez l’espace réservé par votre numéro de reçu d’achat et rendez le fichier lisible par root uniquement, afin que les autres utilisateurs ne puissent pas le voir.
       *[customer-identifier] { $provider } nécessite un identifiant client obtenu sur son site web. fangfrisch le lit dans son fichier de paramètres, généralement { $path }. Ajoutez-y ces lignes, remplacez l’espace réservé par votre identifiant client et rendez le fichier lisible par root uniquement, afin que les autres utilisateurs ne puissent pas le voir.
    }
signatures-not-use = Non utilisée
signatures-add-lines-fangfrisch-settings = Ajoutez ces lignes au fichier de paramètres de fangfrisch, généralement { $path }.
signatures-provider-needs-free-abuse = { $provider } nécessite une Auth-Key abuse.ch gratuite. Ajoutez ces lignes au fichier de paramètres de fangfrisch, généralement { $path }, remplacez l’espace réservé par votre clé et rendez le fichier lisible uniquement par root, afin que les autres utilisateurs ne puissent pas le voir.
signatures-use-label = Utilisée
signatures-unknown-reason-source = Inconnu. { $reason }
signatures-check-again-signature-tooltip = Vérifier à nouveau
signatures-checking-signature-tooltip = Vérification
signatures-clamav-malware-signatures-description = D’où viennent les signatures de logiciels malveillants de ClamAV, et quelles autres sources vous pourriez ajouter.
signatures-sources-title = Sources de signatures
signatures-abuse-ch-key-title = Clé abuse.ch
signatures-abuse-ch-auth-title = Auth-Key abuse.ch
signatures-abuse-ch-key-uses = Utilisée pour les signatures URLhaus et pour les recherches et envois MalwareBazaar.
signatures-abuse-ch-no-key = Aucune clé
signatures-abuse-ch-key-unreadable = Clé illisible
signatures-command-copied-clipboard-signature = Commande copiée dans le presse-papiers
signatures-not-open-documentation-title = Impossible d’ouvrir la documentation : { $error }
signatures-checking-api-key = Vérification de la clé API…
signatures-check-passed = Réussi
signatures-needs-review = Avertissement
signatures-check-failed = Échec
signatures-status-unknown = Inconnu
signatures-check-skipped = Ignoré
signatures-check-progress = Vérification
signatures-copy-tooltip = Copier
signatures-learn-more-title = En savoir plus
signatures-open-browser-tooltip = Ouvrir dans le navigateur
signatures-checking-title = Vérification en cours…

preferences-launcher-not-found = { -brand-name } n’a pas pu trouver son propre lanceur : { $error }
preferences-action-unwritable = L’action du gestionnaire de fichiers n’a pas pu être ajoutée : { $error }
preferences-action-unremovable = L’action du gestionnaire de fichiers n’a pas pu être supprimée : { $error }
preferences-dialog-title = Préférences
preferences-general-title = Général
preferences-window-behavior-title = Comportement de la fenêtre
preferences-detection-alerts-work-description = Les alertes de détection ne fonctionnent que lorsque { -brand-name } est en cours d’exécution.
preferences-closing-window-title = À la fermeture de la fenêtre
preferences-start-background-title = Démarrer en arrière-plan
preferences-launch-background-without-subtitle = Démarrer en arrière-plan sans ouvrir de fenêtre.
preferences-desktop-notifications-title = Notifications de bureau
preferences-notify-detections-finished-subtitle = Avertir des détections, ainsi que des analyses et mises à jour terminées lorsque la fenêtre n’a pas le focus.
preferences-file-manager-integration-title = Intégration du gestionnaire de fichiers
preferences-add-context-menu-description = Ajouter des actions au menu contextuel des gestionnaires de fichiers pris en charge.
preferences-online-services-title = Services en ligne
preferences-services-tell-more-description = Services qui vous en disent plus sur un fichier détecté ou mis en quarantaine. Les recherches ne sont lancées que lorsque vous appuyez sur Rechercher pour une détection ou une entrée de la quarantaine, et n’envoient qu’une empreinte ou un nom de signature. Un fichier ne quitte cet ordinateur que lorsque vous l’envoyez et confirmez ce qui est envoyé.
preferences-lookups-send-file-description = Les recherches n’envoient que l’empreinte SHA-256 d’un fichier et indiquent s’il correspond à un fichier connu et inoffensif. Aucune clé nécessaire.
preferences-no-lookups-report-description = Aucune recherche. Un signalement que vous confirmez envoie le fichier, le nom de la détection pour un faux positif, ainsi que votre nom et votre adresse électronique. Talos traite les échantillons de façon automatisée, ne les rend jamais publics et ne les partage pas avec d’autres entreprises. Un examen prend au moins 48 heures. Aucune clé nécessaire.
preferences-lookups-send-virustotal-file = Les recherches n’envoient à VirusTotal que l’empreinte SHA-256 d’un fichier. Un téléversement que vous confirmez envoie le fichier en quarantaine lui-même, que VirusTotal partage avec ses clients payants. Les téléversements sont ignorés lorsque VirusTotal dispose déjà d’un rapport, et refusés au-delà de 32 Mo.
preferences-malwarebazaar-lookups-send-detection = Les recherches MalwareBazaar envoient le nom de signature d’une détection ou l’empreinte SHA-256 d’un fichier. Un téléversement que vous confirmez envoie le fichier en quarantaine avec sa signature ClamAV comme étiquette, et l’échantillon devient public. La même clé abuse.ch déverrouille aussi le téléchargement des signatures URLhaus via fangfrisch ; sous NixOS, { -brand-name } réutilise le fichier de clé de fangfrisch lorsque votre utilisateur peut le lire. Selon les conditions d’utilisation équitable d’abuse.ch, les utilisateurs à but lucratif peuvent avoir besoin de son API commerciale.
preferences-virustotal-service-hint = Offre gratuite : 4 requêtes par minute, 500 par jour.
preferences-abuse-ch-service-hint = Gratuite sur l’Authentication Portal d’abuse.ch.
preferences-not-configured = Non configurée
preferences-checking-label = Vérification en cours…
preferences-set-system-configuration-title = Définie par la configuration de votre système
preferences-key-keyring-title = Clé dans votre trousseau
preferences-key-file-title = Fichier de clé
preferences-key-unreadable-status = Illisible
preferences-key-file-not-permitted = Impossible de lire le fichier de clé : permission refusée
preferences-key-file-missing = Le fichier de clé n’existe pas
preferences-key-file-unreadable = Impossible de lire le fichier de clé
preferences-key-not-valid = Clé non valide pour ce service
preferences-keyring-unavailable = Le trousseau de clés est indisponible
preferences-stored-key-unreadable = Impossible de lire la clé de votre trousseau
preferences-delete-key-label = Supprimer la clé
preferences-paste-api-key-title = Coller la clé API
preferences-get-free-api-title = Obtenir une clé API gratuite

widgets-clean-label = Sain
widgets-scan-infected = Infecté
widgets-error-status = Erreur
widgets-cancelled-label = Annulé
widgets-skipped-label = Ignoré
widgets-success-label = Succès
widgets-up-date = À jour
widgets-requested-label = Demandée
widgets-ran-label = Exécutée
widgets-counted-threats =
    { $count ->
        [one] { $count } menace détectée
       *[other] { $count } menaces détectées
    }
widgets-uncounted-threats = Le programme d’analyse a signalé une menace sans nommer le fichier.
widgets-no-threats-found-activity = Aucune menace détectée
widgets-scan-failed = L’analyse ne s’est pas terminée correctement
widgets-scan-cancelled-activity = Analyse interrompue par l'utilisateur
widgets-scheduled-scan-skipped-battery = Analyse planifiée ignorée (fonctionne sur batterie)
widgets-virus-definitions-updated = Base virale mise à jour
widgets-virus-definitions-up-date = La base virale est à jour
widgets-update-requested-service = Mise à jour demandée au service
widgets-update-service-no-result = Le service de mise à jour s’est exécuté sans signaler de résultat
widgets-update-failed = Échec de la mise à jour de la base de données
widgets-update-cancelled = Mise à jour de la base virale annulée
widgets-scan-log = JOURNAL D'ANALYSE
widgets-update-log = JOURNAL DE MISE À JOUR
widgets-timestamp-label = Horodatage
widgets-status-label = État
widgets-duration-label = Durée
widgets-summary-label = Résumé
widgets-path-label = Chemin
widgets-full-output = Sortie complète
widgets-scan-history-cannot-title = L’historique des analyses est illisible
widgets-retry-label = Réessayer
widgets-copy-activity-tooltip = Copier
widgets-log-details-copied-toast = Détails du journal copiés dans le presse-papiers
widgets-not-save-file-error = Impossible d’enregistrer le fichier : { $error }
widgets-not-choose-file-error = Impossible de choisir le fichier : { $error }
widgets-chosen-file-not-local = { $location } n’est pas un fichier de cet ordinateur
widgets-export-title = Exporter
widgets-database-newer-release = Cette base de données a été créée par une version plus récente de { -brand-name } : sa version de structure est { $found }, et cette version ne connaît que jusqu’à { $known }. Mettez à jour { -brand-name } pour l’ouvrir.
widgets-migration-step-failed = La mise à jour de la structure de la base de données a échoué à l’étape { $step } : { $error }
widgets-version-check-failed = La version de structure de la base de données n’a pas pu être vérifiée : { $error }
widgets-copy-configuration-line-title = Copier la ligne de configuration
widgets-nothing-applied-choose-description = Rien n’est appliqué. Choisissez une valeur et copiez la ligne, puis ajoutez-la vous-même à votre configuration.
widgets-unit-title = Unité
widgets-line-copy-title = Ligne à copier
widgets-copy-line-label = Copier la ligne
widgets-unlock-required = Déverrouillez pour modifier ce paramètre. Tant qu’il est verrouillé, vous pouvez copier sa ligne pour votre configuration.
widgets-administrator-not-copy = Votre administrateur n’autorise pas la modification de ce paramètre depuis { -brand-name }. Vous pouvez copier sa ligne pour votre configuration.
widgets-plain-bytes = octets
widgets-kibibytes-unit = KiB
widgets-mebibytes-unit = MiB
widgets-unknown-option = { -brand-name } ne connaît pas cette option : il se contente donc de l’afficher.
widgets-not-yes-no = À examiner : cette valeur doit être yes ou no.
widgets-not-whole-number = À examiner : cette valeur doit être un nombre entier.
widgets-count-out-range = À examiner : cette valeur doit être comprise entre { $minimum } et { $maximum }.
widgets-not-size = À examiner : cette valeur doit être une taille en octets, ou un nombre suivi de K ou M.
widgets-path-not-absolute = À examiner : cette valeur doit être un chemin complet commençant par /.
widgets-empty-text = À examiner : cette valeur est vide.
widgets-apply-tooltip = Appliquer
widgets-unit-tooltip = Unité
widgets-setting-runs-commands-changes = Ce paramètre exécute des commandes ou change l’utilisateur sous lequel ClamAV s’exécute : { -brand-name } ne propose donc jamais de le modifier
widgets-check-value-configuration-file = Vérifiez cette valeur dans le fichier de configuration.
widgets-copy-configuration-line-tooltip = Copier la ligne de configuration…
widgets-applying-restarting-clamav-tooltip = Application et redémarrage de ClamAV
widgets-detect-potentially-unwanted-programs = Détecter les programmes potentiellement indésirables
widgets-heuristic-detections = Détections heuristiques
widgets-flag-office-files-macros = Signaler les fichiers Office contenant des macros
widgets-flag-encrypted-archives-documents = Signaler les archives et documents chiffrés
widgets-look-inside-archives = Examiner le contenu des archives
widgets-flag-files-too-large = Signaler les fichiers trop gros pour être entièrement vérifiés
widgets-adware-hacking-tools-remote = Logiciels publicitaires, outils de piratage et outils d’accès à distance. Attendez-vous à davantage de faux positifs.
widgets-suspicious-files-no-signature = Fichiers suspects qu’aucune signature ne désigne, comme les exécutables endommagés et les techniques d’hameçonnage.
widgets-macros-common-way-deliver = Les macros sont un moyen courant de diffuser des logiciels malveillants, mais de nombreux documents légitimes en utilisent.
widgets-clamav-cannot-look-inside = ClamAV ne peut pas examiner leur contenu : ils pourraient donc cacher des logiciels malveillants.
widgets-checks-files-inside-zip = Vérifie les fichiers contenus dans les archives zip, rar et autres.
widgets-otherwise-files-above-size = Sinon, les fichiers dépassant les limites de taille et de récursion passent sans vérification ni avertissement.
widgets-locked-until-unlocked = Déverrouillez pour modifier ce paramètre.
widgets-administrator-not-allow-detection = Votre administrateur n’autorise pas la modification de ce paramètre depuis { -brand-name }.
widgets-clamd-settings = Paramètres de clamd
widgets-access-scanning-settings = Paramètres de l’analyse à l’accès
widgets-update-settings = Paramètres de mise à jour
widgets-change-snippet-copied = Copié. Ajoutez-le à votre configuration pour l’appliquer.
widgets-stale-confirmation-dropped = La modification n’a pas été effectuée : les paramètres ont été reverrouillés ou une autre modification a commencé avant.
widgets-settings-applied = Appliqué, mais pas encore dans la configuration de votre système.
widgets-previous-restored = ClamAV n’a pas redémarré correctement avec cette valeur : les paramètres précédents ont donc été restaurés.
widgets-change-not-made-error = La modification n’a pas été effectuée : { $error }
widgets-authorization-refused = Rien n’a été modifié sans votre mot de passe
widgets-admin-service-missing = Cette installation ne dispose pas du service d’administration de ClamAV : les paramètres de ClamAV ne peuvent donc pas être modifiés depuis { -brand-name }
widgets-live-tunable-config = { $path } combine la configuration de votre système avec les modifications effectuées ici. Tant qu’ils sont verrouillés, les paramètres sont en lecture seule : utilisez Copier la ligne de configuration pour en copier une pour votre configuration. Après le déverrouillage, les paramètres autorisés sont modifiés directement.
widgets-change-setting = Modifier ce paramètre ?
widgets-cancel-engine = Annuler
widgets-reduce-protection = Réduire la protection
widgets-apply-label = Appliquer
widgets-key-value-until-time = { $key } { $value }, jusqu’à { $time }
widgets-live-tuning-not-installed = Le réglage à chaud n’est pas installé. Sous NixOS, activez services.lazaret.liveTuning ; sous Debian et Ubuntu, installez le paquet { -brand-name }, qui inclut le service d’administration de ClamAV.
widgets-admin-service-unreachable = Les paramètres ne peuvent pas être modifiés ici pour le moment : le service d’administration ClamAV de { -brand-name } est injoignable.
widgets-loading-config = Lecture de la configuration…
widgets-config-missing = Fichier de configuration introuvable
widgets-lock-label = Verrouiller
widgets-unlock-label = Déverrouiller
widgets-change-settings-title = Modifier les paramètres
widgets-unlock-change-settings-description = Déverrouillez pour modifier les paramètres que votre système autorise à l’exécution. Le déverrouillage n’accorde rien par lui-même : chaque modification indique ce qui va se passer et demande une autorisation d’administrateur. Le verrouillage revient au bout de deux minutes.
widgets-changes-system-configuration-title = Modifications inconnues de la configuration de votre système
widgets-copy-engine-tooltip = Copier
widgets-reduced-protection-ends-title = Protection réduite qui prend fin d’elle-même
widgets-reset-system-configuration-title = Rétablir la configuration du système
widgets-detection-title = Détection
widgets-clamav-reports-every-description = Ce que ClamAV signale dans chaque analyse sur cet ordinateur. { -brand-name } transmet les mêmes choix à clamscan, afin que les deux programmes d’analyse soient d’accord.
widgets-no-settings-section-title = Aucun paramètre dans cette section
widgets-journal-read-failed = Le journal du système n’a pas pu être lu : { $error }
widgets-journal-reading-incomplete = La lecture du journal du système s’est arrêtée avant la fin
widgets-folder-unreadable = Le dossier des signatures n’a pas pu être lu : { $error }
widgets-reading-signature-folder-stopped = La lecture du dossier des signatures s’est arrêtée avant la fin
widgets-entry-timestamp-invalid = Une entrée du journal a un horodatage en dehors de la plage prise en charge.
widgets-reader-thread-failed = Le lecteur du journal n’a pas pu démarrer : { $error }
widgets-journal-reading-stopped = La lecture du journal système s’est arrêtée. { $reason }
widgets-file-hash-mismatch = L’empreinte du fichier ne correspond pas — le fichier est peut-être corrompu
widgets-restore-refused-integrity-unknown = Cet enregistrement de quarantaine n’a pas d’empreinte utilisable : le fichier ne peut donc pas être vérifié et n’est pas restauré. Vous pouvez toujours le supprimer.
widgets-upload-refused-integrity-unknown = Cet enregistrement de quarantaine n’a pas d’empreinte utilisable : { -brand-name } ne peut donc pas prouver que le fichier est celui qu’il a mis en quarantaine et ne l’envoie nulle part.
widgets-payload-too-large-send = Le fichier dépasse { $size }, la taille maximale que { -brand-name } envoie.
widgets-blocked-file-other-filesystem = L’analyse à l’accès bloque ce fichier et il se trouve sur un autre système de fichiers : il ne peut donc pas être déplacé en quarantaine. Vous pouvez le supprimer à la place.
widgets-entry-not-found = Entrée dans la quarantaine introuvable : { $id }
widgets-recording-failed-file-restored = Impossible d'enregistrer l'entrée de quarantaine dans la base de données. Le fichier a été restauré à son emplacement d'origine.
widgets-recording-failed-rollback = Impossible d'enregistrer l'entrée de quarantaine dans la base de données. La restauration a également échoué : { $error }. Le fichier peut être orphelin à : { $path }
widgets-entry-record-not-removed = Le fichier a été déplacé, mais son enregistrement de quarantaine n’a pas pu être supprimé. Il peut encore apparaître dans la liste.
widgets-system-call-failed = Un appel système a échoué : { $error }
widgets-file-operation-failed = Une opération sur un fichier a échoué : { $error }
widgets-database-operation-failed = L’opération sur la base de données de la quarantaine a échoué : { $error }
widgets-blocking-step-interrupted = Une étape de quarantaine en arrière-plan s’est arrêtée de façon inattendue : { $error }
widgets-path-contains-nul = Le chemin contient un octet NUL
widgets-path-has-no-parent = Le chemin n’a pas de dossier parent
widgets-path-has-no-file = Le chemin n’a pas de nom de fichier
widgets-database-path-not-utf8 = Le chemin de la base de données de la quarantaine n’est pas un UTF-8 valide
widgets-database-not-regular-file = La base de données de la quarantaine n’est pas un fichier ordinaire
widgets-source-blocked-access = L’analyse à l’accès bloque la lecture du fichier
widgets-moved-payload-still-blocked = Le fichier bloqué est toujours illisible après son déplacement en quarantaine
widgets-source-not-regular-file = Le fichier n’est pas un fichier ordinaire
widgets-payload-not-regular-file = Le fichier en quarantaine n’est pas un fichier ordinaire
widgets-not-enough-free-space = L’espace libre est insuffisant pour mettre le fichier en quarantaine
widgets-file-replaced-during-operation = Le fichier a été remplacé pendant son déplacement : il n’a donc pas été supprimé
widgets-path-outside-quarantine = Le chemin ne se trouve pas dans le dossier de quarantaine
widgets-invalid-restore-path = Le chemin de restauration n’est pas valide
widgets-protected-restore-destination = La restauration dans un dossier système protégé n’est pas autorisée
widgets-stored-path-unreadable = Un chemin enregistré dans la base de données de la quarantaine n’a pas pu être lu
widgets-target-missing = La cible d’analyse n’existe pas ou contient un octet NUL non valide
widgets-target-not-scannable = La cible d’analyse n’est ni un fichier ordinaire ni un dossier
widgets-target-directory-unreadable = Impossible d’énumérer la cible d’analyse { $path } : { $error }
widgets-file-list-unwritable = Impossible d’écrire la liste privée de fichiers pour clamd : { $error }
widgets-enumeration-interrupted = L’énumération de la cible d’analyse s’est arrêtée de façon inattendue.
widgets-no-files-scanned = Aucun fichier n’a pu être analysé
widgets-unscanned-files =
    { $count ->
        [one] { $count } fichier n’a pas été analysé
       *[other] { $count } fichiers n’ont pas été analysés
    }
widgets-clamd-connection-lost = La connexion à clamd a été perdue pendant l’analyse.
widgets-scanner-stopped-reporting = Le programme d’analyse n’a pas signalé de résultat pour ces fichiers.
widgets-count-not-accessed-title =
    { $count ->
        [one] Impossible d’accéder à { $count } fichier
       *[other] Impossible d’accéder à { $count } fichiers
    }
widgets-files-unreadable =
    { $count ->
        [one] Impossible de lire { $count } fichier
       *[other] Impossible de lire { $count } fichiers
    }
widgets-nonfatal-warnings =
    { $count ->
        [one] { $count } avertissement non fatal pendant l’analyse ; certains fichiers n’ont peut-être été analysés que partiellement
       *[other] { $count } avertissements non fatals pendant l’analyse ; certains fichiers n’ont peut-être été analysés que partiellement
    }
widgets-count-threat-found-outcome =
    { $count ->
        [one] { $count } menace détectée
       *[other] { $count } menaces détectées
    }
widgets-count-file-scanned-outcome =
    { $count ->
        [one] { $count } fichier analysé
       *[other] { $count } fichiers analysés
    }
widgets-clamav-not-open-files = ClamAV n’a pas pu ouvrir ces fichiers.
widgets-unattributed-threats-only = Le programme d’analyse a signalé une menace sans nommer le fichier.
widgets-preparing-scan = Préparation…
widgets-finding-files =
    { $count ->
        [one] Recherche des fichiers… { $count } trouvé
       *[other] Recherche des fichiers… { $count } trouvés
    }
widgets-trust-file = Approuver ce fichier…
widgets-ignore-detection-for-me = Ignorer cette détection pour moi…
widgets-report-false-positive = Signaler un faux positif…
widgets-skip-path-lazaret-scans = Ignorer ce chemin dans les analyses de { -brand-name }
widgets-copy-path = Copier le chemin
widgets-critical-severity = Critique
widgets-high-severity = Élevé
widgets-medium-severity = Moyen
widgets-low-severity = Faible
widgets-moved-quarantine-path-check = Déplacé en quarantaine : { $path }. Vérifiez sa réputation sur la page Quarantaine.
widgets-quarantine-detected-label = Quarantaine
widgets-more-actions-tooltip = Plus d’actions
widgets-count-more-file-title =
    { $count ->
        [one] { $count } fichier de plus, listé dans le rapport exporté
       *[other] { $count } fichiers de plus, listés dans le rapport exporté
    }
widgets-scan-details-title = Détails de l’analyse
widgets-quarantine-all =
    { $count ->
        [one] Mettre { $count } menace en quarantaine
       *[other] Mettre { $count } menaces en quarantaine
    }
widgets-move-quarantine = Déplacer en quarantaine ?
widgets-removes-each-original-file = Cette opération retire chaque fichier d’origine de son emplacement actuel.
widgets-cancel-scan = Annuler
widgets-quarantine-label = Quarantaine
widgets-detected-threats-title = Menaces détectées
widgets-problems-title = Problèmes
widgets-move-path-quarantine = Déplacer « { $path } » en quarantaine ?
widgets-move-count-detected-file =
    { $count ->
        [one] Déplacer { $count } fichier détecté en quarantaine ?
       *[other] Déplacer { $count } fichiers détectés en quarantaine ?
    }
widgets-path-copied-clipboard = Chemin copié dans le presse-papiers
widgets-file-skipped-future-scans = Ce fichier sera ignoré lors des prochaines analyses
widgets-exclusion-already-exists = Cette exclusion existe déjà
widgets-results-copied-clipboard = Résultats copiés dans le presse-papiers
widgets-results-exported = Résultats exportés
widgets-done-total-file-scanned =
    { $done ->
        [one] { NUMBER($done) } fichier sur { $total } analysé
       *[other] { NUMBER($done) } fichiers sur { $total } analysés
    }
widgets-count-file-scanned-results =
    { $count ->
        [one] { $count } fichier analysé
       *[other] { $count } fichiers analysés
    }
widgets-count-entry-skipped-exclusions =
    { $count ->
        [one] { $count } entrée ignorée par les exclusions
       *[other] { $count } entrées ignorées par les exclusions
    }
widgets-threats-detected-status = Menaces détectées
widgets-no-threats-detected-status = Aucune menace détectée
widgets-finished-problems-status = L’analyse ne s’est pas terminée correctement
widgets-scan-cancelled-status = Analyse interrompue par l'utilisateur
widgets-count-threat-found-results =
    { $count ->
        [one] { $count } menace détectée
       *[other] { $count } menaces détectées
    }
widgets-files-scanned = Fichiers analysés
widgets-files-planned = Fichiers prévus
widgets-directories-label = Répertoires
widgets-threats-found = Menaces détectées
widgets-file-no-longer-quarantine = Le fichier n’est plus présent. S’il est en quarantaine, approuvez-le depuis la page Quarantaine.
widgets-permission-denied = Le fichier est illisible, peut-être parce que l’analyse à l’accès le bloque. Déplacez-le en quarantaine, puis approuvez-le depuis la page Quarantaine.
widgets-file-not-read-scan = Le fichier n’a pas pu être lu : { $error }
widgets-threats-detected-finished = Menaces détectées
widgets-no-threats-detected-finished = Aucune menace détectée
widgets-finished-problems-notify = L’analyse ne s’est pas terminée correctement
widgets-scanner-error = Le programme d’analyse s’est arrêté avec le code d’erreur { $code }.
widgets-killed-signal = Le programme d’analyse a été arrêté par le signal { $signal }.
widgets-unrecognized-wait-status = Le programme d’analyse s’est terminé d’une manière que { -brand-name } ne reconnaît pas (état d’attente { $status }).
widgets-wait-failed = { -brand-name } a perdu la trace du processus d’analyse : { $error }.
widgets-scanner-program-missing = Le programme d’analyse de ClamAV n’est pas installé ou ne se trouve pas dans le chemin de recherche.
widgets-scanner-program-replaceable = { -brand-name } a refusé d’exécuter { $program }, car votre compte peut remplacer { $replaceable }. Installez ClamAV pour tout le système afin qu’un programme placé par un logiciel malveillant ne puisse pas falsifier des résultats sains.
widgets-scanner-spawn-failed = Le programme d’analyse n’a pas pu être démarré : { $error }
widgets-scanner-work-directory-unavailable = Impossible de créer le répertoire de travail privé du programme d’analyse : { $error }
widgets-scanner-output-uncaptured = La sortie du programme d’analyse n’a pas pu être capturée.
widgets-detection-settings-unreadable = L’analyse n’a pas démarré, car les paramètres de détection de clamd n’ont pas pu être lus : { $error }
widgets-program-lookup-failed = L’analyse n’a pas démarré, car les programmes d’analyse n’ont pas pu être recherchés : { $error }
widgets-scan-input-unavailable = Impossible d’ouvrir en toute sécurité l’entrée d’analyse { $path } : { $error }
widgets-detections-warnings-errors-tooltip = Uniquement les détections, avertissements et erreurs
widgets-copy-service-tooltip = Copier
widgets-export-text-tooltip = Exporter au format texte
widgets-search-log-placeholder = Rechercher dans le journal
widgets-reading-log = Lecture du journal…
widgets-no-log-lines-match = Aucune ligne du journal ne correspond
widgets-log-copied-clipboard = Journal copié dans le presse-papiers
widgets-text-export-finished = Exporté.
widgets-threat-detected = Détection
widgets-emergency-label = Urgence
widgets-alert-label = Alerte
widgets-critical-level = Critique
widgets-error-level = Erreur
widgets-warning-label = Avertissement
widgets-notice-label = Notice
widgets-info-label = Info
widgets-debug-label = Débogage
widgets-log-reading = Vérification en cours…
widgets-system-journal = Journal du système
widgets-neither-log-file-nor = Aucun fichier journal ni aucune ligne de journal n’a été trouvé.
widgets-no-log-file-found = Aucun fichier journal n’a été trouvé. { $reason }
widgets-program-use = { $program } utilisé
widgets-ignored-program = { $program } ignoré
widgets-refused-program = { $program } refusé
widgets-scanner-programs-trusted = Les programmes d’analyse sont fiables
widgets-freshclam-trusted = freshclam est fiable
widgets-malware-running-clamdscan = Un logiciel malveillant exécuté sous votre compte pourrait placer un faux clamdscan plus tôt dans votre chemin de recherche, qui déclarerait chaque fichier sain. { -brand-name } n’exécute que les programmes ClamAV que votre compte ne peut pas remplacer, comme ceux installés pour tout le système.
widgets-malware-running-freshclam = Un logiciel malveillant exécuté sous votre compte pourrait placer un faux freshclam plus tôt dans votre chemin de recherche, qui ferait semblant de récupérer de nouvelles signatures. { -brand-name } n’exécute que les programmes ClamAV que votre compte ne peut pas remplacer, comme ceux installés pour tout le système.
widgets-check-interrupted = Inconnu, car la vérification des programmes ClamAV s’est arrêtée avant la fin
widgets-clamav-program-replaced-account = Un programme ClamAV peut être remplacé par votre compte : { -brand-name } refuse donc de l’exécuter
widgets-clamav-program-account-shadows = Un programme ClamAV dans votre compte masque la copie du système. { -brand-name } l’ignore, mais vérifiez sa provenance
widgets-count-settings-reduced-since =
    { $count ->
        [one] { $count } paramètre vérifie moins que configuré depuis { $time }
       *[other] { $count } paramètres vérifient moins que configuré depuis { $time }
    }
widgets-count-settings-reduced-unknown-time =
    { $count ->
        [one] { $count } paramètre vérifie moins que configuré, depuis un moment qui n’a pas pu être lu
       *[other] { $count } paramètres vérifient moins que configuré, depuis un moment qui n’a pas pu être lu
    }
widgets-changed-label = Modifié
widgets-change-time-unknown-override = Inconnu
widgets-reduction-unknown-unread = On ignore si des paramètres ont été réduits, car les modifications à l’exécution n’ont pas pu être lues : { $error }
widgets-reduction-unknown-file = On ignore si des paramètres ont été réduits, car le fichier des modifications à l’exécution n’est pas au format attendu. ClamAV fonctionne avec la configuration du système jusqu’à ce qu’il soit corrigé.
widgets-reduction-unknown-reading = On ignore si des paramètres ont été réduits, car la lecture des modifications à l’exécution s’est arrêtée avant la fin
widgets-settings-changed-runtime = Paramètres modifiés à l’exécution
widgets-administrator-can-change = Toute personne disposant du mot de passe d’administrateur peut modifier les paramètres de ClamAV à l’exécution. { -brand-name } n’a pas pu lire ces modifications : il ne peut donc pas savoir si elles vérifient moins que votre configuration.
widgets-protection-reduced-purpose = Protection réduite volontairement
widgets-administrator-changed-runtime = Une personne disposant du mot de passe d’administrateur a modifié les paramètres de ClamAV à l’exécution de façon à vérifier moins que votre configuration. Les modifications du système de la page dont relève le paramètre indiquent qui et quand.
widgets-base-configuration-changed-now = { $base } dans votre configuration, { $changed } maintenant
widgets-administrator-changed-clamd = Une personne disposant du mot de passe d’administrateur a modifié les paramètres de ClamAV à l’exécution. Votre clamd.conf n’a pas pu être lu : { -brand-name } ne peut donc pas savoir si ces modifications vérifient moins que votre configuration. Les modifications du système de la page dont relève le paramètre indiquent qui et quand.
widgets-settings-changed-runtime-clamd = Des paramètres ont été modifiés à l’exécution, mais clamd.conf n’a pas pu être lu pour les comparer
widgets-changed-now = { $changed } maintenant
widgets-daemon-awaited = Nécessite le démon, voir Moteur d’analyse
widgets-unknown-run = Inconnu. { $reason }
widgets-copy-timeline-tooltip = Copier
widgets-moment-details = { $moment } · { $details }
widgets-why-matters-tooltip = Pourquoi c’est important
widgets-what-is-sent-label = Ce qui est envoyé :
widgets-clamav-audience = Cisco Talos, l’équipe derrière ClamAV, l’examine. Les échantillons sont traités de façon automatisée, seul Talos peut y accéder, et ils ne sont jamais rendus publics ni partagés avec d’autres entreprises. Un examen prend au moins 48 heures, et son résultat apparaît dans une version ultérieure de la base de signatures.
widgets-report-false-positive-clamav = Signaler un faux positif à ClamAV ?
widgets-file-content = Le contenu du fichier
widgets-detection-name = Le nom de détection { $detection }
widgets-report-missed-malware-clamav = Signaler un logiciel malveillant manqué à ClamAV ?
widgets-name-email-address-talos = Votre nom et votre adresse électronique, afin que Talos puisse vous contacter
widgets-lazaret-sends-file-clamav = { -brand-name } envoie ce fichier à ClamAV avec clamsubmit. N’envoyez que des fichiers qui ne contiennent rien de privé.
widgets-send-report-button = Envoyer le signalement
widgets-file-content-named-sha = Le contenu du fichier, désigné par son empreinte SHA-256 et non par son nom
widgets-upload-malwarebazaar-heading = Téléverser sur MalwareBazaar ?
widgets-lazaret-uploads-file-malwarebazaar = { -brand-name } téléverse ce fichier sur MalwareBazaar, géré par abuse.ch.
widgets-tag-clamav-detection = L’étiquette { $tag }, issue de la détection de ClamAV
widgets-abuse-ch-key-sample = Votre clé abuse.ch ; l’échantillon est listé sans le nom de votre compte
widgets-malwarebazaar-public-anyone-download = MalwareBazaar est public : n’importe qui peut télécharger l’échantillon une fois téléversé, et un téléversement ne peut pas être annulé. Ne téléversez que des logiciels malveillants, jamais un document pouvant contenir des données personnelles.
widgets-upload-publicly-button = Téléverser publiquement
widgets-upload-virustotal-heading = Téléverser sur VirusTotal ?
widgets-lazaret-uploads-file-virustotal = { -brand-name } téléverse ce fichier sur VirusTotal pour analyse.
widgets-virustotal-key-links-upload = Votre clé VirusTotal, qui associe le téléversement à votre compte
widgets-virustotal-shares-uploaded-files = VirusTotal partage les fichiers téléversés avec ses clients payants, qui peuvent les télécharger, et n’importe qui peut consulter le résultat grâce à l’empreinte du fichier. Ne téléversez jamais un document pouvant contenir des données personnelles.
widgets-upload-button = Téléverser
widgets-cancel-submission = Annuler
widgets-name-title = Votre nom
widgets-email-address-title = Votre adresse électronique
widgets-file-unreadable = Le fichier n’a pas pu être lu : { $error }
widgets-file-not-regular = Seuls les fichiers ordinaires peuvent être envoyés
widgets-file-too-large = Le fichier dépasse { $size }, la taille maximale que { -brand-name } envoie.
widgets-reading-interrupted = La lecture du fichier s’est arrêtée de façon inattendue
widgets-history-failure = L’historique des envois n’a pas pu être lu : { $error }
widgets-earlier-submission = Ce fichier a déjà été envoyé le { $local-time }. Rien n’a été renvoyé.
widgets-report-recorded = Signalement envoyé. L’examen de ClamAV prend au moins 48 heures ; Échantillons envoyés indique quand une mise à jour des signatures change le verdict.
widgets-report-not-sent-error = Le signalement n’a pas été envoyé : { $error }
widgets-report-sent-without-record = Signalement envoyé, mais il n’a pas pu être ajouté aux échantillons envoyés : { $error }
widgets-nothing-uploaded-error = Rien n’a été envoyé. { $reason }
widgets-bazaar-key-not-usable = MalwareBazaar n’a aucune clé utilisable.
widgets-virustotal-key-not-usable = VirusTotal n’a aucune clé utilisable.
widgets-malwarebazaar-refused-sample-reason = MalwareBazaar a refusé l’échantillon : { $reason }
widgets-malwarebazaar-sample-sent-samples = MalwareBazaar dispose de l’échantillon. Échantillons envoyés renvoie vers son entrée publique.
widgets-not-detected-clamav = MalwareBazaar n’accepte que les fichiers détectés par une signature ClamAV
widgets-unwanted-program-only = MalwareBazaar n’accepte pas les programmes potentiellement indésirables, uniquement les logiciels malveillants
widgets-sent-without-record = Téléversé, mais il n’a pas pu être ajouté aux échantillons envoyés : { $error }
widgets-virustotal-refused-file-reason = VirusTotal a refusé le fichier : { $reason }
widgets-virustotal-analysing-file-refresh = VirusTotal analyse le fichier. Actualiser sur la page Échantillons envoyés affiche son verdict.
widgets-report-already-available =
    { $count ->
        [one] VirusTotal connaît déjà ce fichier : { NUMBER($count) } moteur sur { $total } le détecte. Rien n’a été envoyé.
       *[other] VirusTotal connaît déjà ce fichier : { NUMBER($count) } moteurs sur { $total } le détectent. Rien n’a été envoyé.
    }
widgets-known-user = { $local-time } · { $outcome } · demandé par l’utilisateur { $uid }
widgets-unknown-user = { $time } · { $outcome } · demandé par un utilisateur inconnu
widgets-change-lowered-protection-tooltip = Cette modification a réduit la protection
widgets-system-changes-failed = Le journal du système est illisible : { $error }
widgets-system-changes-title = Modifications du système
widgets-refresh-tooltip = Actualiser
widgets-system-changes-cannot-title = Les modifications du système sont illisibles
widgets-loading-system-changes = Chargement des modifications du système…
widgets-trust-file-heading = Approuver ce fichier ?
widgets-clamav-never-report-body =
    ClamAV ne signalera plus jamais un fichier ayant exactement ce contenu, pour aucun utilisateur, où qu’il se trouve sur cet ordinateur et quelle que soit l’analyse qui le trouve, et l’analyse à l’accès cessera de le bloquer. Votre mot de passe d’administrateur est nécessaire, et la demande de mot de passe affiche la même empreinte SHA-256 que ci-dessous.
    
    { $path }
widgets-stop-showing-file-body =
    { -brand-name } cessera de vous montrer un fichier ayant exactement ce contenu dans les analyses que vous lancez, où qu’il se trouve. ClamAV lui-même le détecte toujours : l’analyse à l’accès peut donc encore le bloquer et vous alerter, et les autres utilisateurs le voient toujours. Approuver pour tous les utilisateurs empêche ClamAV de le détecter pour tout le monde et nécessite votre mot de passe d’administrateur.
    
    { $path }
widgets-trust-for-me-button = Approuver pour moi
widgets-user-trust-unreadable = Les fichiers et détections que vous avez approuvés n’ont pas pu être lus : cette analyse affiche donc toutes les détections : { $error }
widgets-trust-for-all-users-button = Approuver pour tous les utilisateurs…
widgets-sha-256-hash-size =
    SHA-256 { $hash }
    { $size }
widgets-ignore-detection-heading = Ignorer cette détection ?
widgets-stop-showing-detection-body = { -brand-name } cessera de vous montrer chaque fichier portant cette détection, pas seulement celui-ci. ClamAV lui-même et les autres utilisateurs les voient toujours. Approuver le fichier par son empreinte est plus sûr lorsqu’un seul fichier est un faux positif.
widgets-ignore-button = Ignorer
widgets-stop-trusting-file-heading = Ne plus approuver ce fichier ?
widgets-clamav-report-file-body = ClamAV signalera à nouveau ce fichier à chaque utilisateur lorsqu’il correspondra à une signature.
widgets-show-file-again-body = { -brand-name } vous montrera à nouveau ce fichier lorsque ClamAV le détectera.
widgets-stop-trusting-button = Ne plus approuver
widgets-report-detection-again-heading = Signaler à nouveau cette détection ?
widgets-clamav-report-files-body = ClamAV signalera à nouveau les fichiers portant cette détection à chaque utilisateur.
widgets-show-detection-again-body = { -brand-name } vous montrera à nouveau les fichiers portant cette détection.
widgets-report-again-button = Signaler à nouveau
widgets-cancel-trust = Annuler

host-action-applied = Appliqué
host-action-rolled-back = Annulé
host-authorization-refused-described = Refusé sans mot de passe
host-action-failed = Échec
host-outcome-not-recorded = Résultat non enregistré
host-unrecognized-outcome = Résultat non reconnu { $outcome }
host-blocking-turned-off = Les fichiers détectés ne seront plus bloqués à l’ouverture. Ils seront seulement signalés.
host-check-turned-off = ClamAV cessera de rechercher ce type de fichier ou de menace sur cet ordinateur.
host-watched-folder-removed = Les fichiers du dossier retiré ne seront plus vérifiés à leur ouverture.
host-skipped-location-added = ClamAV ignorera entièrement la nouvelle entrée : les logiciels malveillants qui s’y trouvent passeront inaperçus.
host-scan-limit-lowered = Les fichiers ou archives au-delà de la nouvelle limite seront ignorés sans être vérifiés.
host-current-values-unknown = La valeur actuelle n’a pas pu être lue : cette modification risque donc de réduire la protection.
host-detection-trusted = ClamAV ne signalera plus ce que vous approuvez ici.
host-key-now-current-new =
    { $key }
    Actuel : { $current }
    Nouveau : { $new }
host-system-records-change-reduced = Le système enregistre que cette modification a réduit la protection, et la barre latérale continue de signaler la page du paramètre modifié jusqu’à ce que vous l’annuliez.
host-trust-applied = ClamAV a chargé la modification
host-trust-recorded-for-you = { -brand-name } ne vous montre plus ceci. ClamAV lui-même, le blocage à l’accès et les autres utilisateurs le voient toujours.
host-applied-reload-pending = Enregistré. clamd n’a pas répondu : il prendra en compte la modification au prochain rechargement de ses signatures
host-authorization-refused-outcome = Rien n’a été modifié sans votre mot de passe
host-admin-service-missing = Cette installation ne dispose pas du service d’administration de ClamAV : la liste d’approbation de ClamAV ne peut donc pas être modifiée depuis { -brand-name }
host-trust-failed = Rien n’a été modifié : { $error }
host-nixos-configuration = { $path } est généré par votre configuration NixOS : { -brand-name } ne peut donc pas le modifier. Utilisez Copier la ligne de configuration sur un paramètre pour copier sa ligne pour votre configuration, puis reconstruisez.
host-system-package = { $path } appartient au système : { -brand-name } ne peut donc pas le modifier. Utilisez Copier la ligne de configuration sur un paramètre pour copier la ligne à placer dans ce fichier.
host-contact-incomplete = Saisissez votre nom et une adresse électronique valide pour envoyer un signalement
host-submitter-missing = Le signalement nécessite clamsubmit, fourni avec ClamAV mais non installé ici
host-submitter-replaceable = { -brand-name } a refusé d’exécuter clamsubmit, car votre compte peut le remplacer
host-report-not-sent-error = Le signalement n’a pas été envoyé : { $error }
host-upload-timed-out = Le signalement n’a pas été envoyé : le téléversement a pris trop de temps
host-unrecognized-source = Autre source
host-file-progress =
    { $done ->
        [one] { NUMBER($done) } fichier sur { $total }
       *[other] { NUMBER($done) } fichiers sur { $total }
    }
host-count-second-left =
    { $count ->
        [one] Environ { $count } seconde restante
       *[other] Environ { $count } secondes restantes
    }
host-count-minute-left =
    { $count ->
        [one] Environ { $count } minute restante
       *[other] Environ { $count } minutes restantes
    }
host-count-hour-left =
    { $count ->
        [one] Environ { $count } heure restante
       *[other] Environ { $count } heures restantes
    }
host-seconds-s = { $seconds } s
host-minutes-min-seconds-s = { $minutes } min { $seconds } s
host-hours-h-minutes-min = { $hours } h { $minutes } min
host-blocked-access = Bloqué par l’analyse à l’accès
host-exact-byte-size =
    { $count ->
        [one] { $size } ({ $count } octet)
       *[other] { $size } ({ $count } octets)
    }
host-hash-unknown = Absent de la base des fichiers connus
host-listed-malicious-source-name = Répertorié comme malveillant par { $source } (sous le nom { $name })
host-listed-malicious-unnamed-source-name = Répertorié comme malveillant par une source sans nom (sous le nom { $name })
host-listed-malicious-source = Répertorié comme malveillant par { $source }
host-listed-malicious-unnamed-source = Répertorié comme malveillant par une source sans nom
host-listed-known-file-source = Répertorié comme fichier connu par { $source }
host-listed-known-file-unnamed-source = Répertorié comme fichier connu par une source sans nom
host-known-file-name-listed = Fichier connu { $name }, répertorié par { $source }
host-known-file-name-unnamed-source = Fichier connu { $name }, répertorié par une source sans nom
host-unnamed-family = famille sans nom
host-no-known-samples = Pas un échantillon de logiciel malveillant connu
host-known-malware-sample = Échantillon de logiciel malveillant connu
host-samples-found = Logiciel malveillant connu : { $family }, vu pour la première fois { $date }
host-samples-found-date-unknown = Logiciel malveillant connu : { $family }, vu pour la première fois à une date inconnue
host-unnamed-family-samples-found = Logiciel malveillant connu d’une famille sans nom, vu pour la première fois { $date }
host-unnamed-family-date-unknown = Logiciel malveillant connu d’une famille sans nom, vu pour la première fois à une date inconnue
host-file-unknown-virustotal = Inconnu de VirusTotal
host-count-total-engine-verdict =
    { $count ->
        [one] { NUMBER($count) } moteur sur { $total } le détecte
       *[other] { NUMBER($count) } moteurs sur { $total } le détectent
    }
host-api-key-rejected = VirusTotal a rejeté la clé API.
host-request-limit-reached = La limite de requêtes VirusTotal a été atteinte. Réessayez plus tard.
host-virustotal-answered-status = VirusTotal a répondu avec l’état { $status }
host-virustotal-not-reached-error = VirusTotal est injoignable : { $error }
host-virustotal-sent-answer-lazaret = VirusTotal a envoyé une réponse que { -brand-name } ne peut pas lire
host-request-failed-circl = CIRCL hashlookup est injoignable : { $error }
host-answer-unreadable-circl = CIRCL hashlookup a envoyé une réponse illisible
host-unexpected-status-circl = CIRCL hashlookup a répondu avec l’état { $status }
host-request-failed-bazaar = MalwareBazaar est injoignable : { $error }
host-answer-unreadable-bazaar = MalwareBazaar a envoyé une réponse illisible
host-request-limit-reached-bazaar = La limite de requêtes MalwareBazaar a été atteinte. Réessayez plus tard.
host-unexpected-status-bazaar = MalwareBazaar a répondu avec l’état { $status }
host-key-refused = MalwareBazaar a refusé la clé API
host-query-refused = MalwareBazaar a répondu : { $status }
headless-count-threat-found =
    { $count ->
        [one] { $count } menace détectée
       *[other] { $count } menaces détectées
    }
headless-open-lazaret-review = Ouvrez { -brand-name } pour les examiner.
headless-count-threats-quarantined =
    { $count ->
        [one] { $count } déplacée en quarantaine.
       *[other] { $count } déplacées en quarantaine.
    }
headless-scan-infected = Menaces détectées
headless-scheduled-scan-failed = Échec de l’analyse planifiée
headless-open-lazaret-read-log = Ouvrez { -brand-name } pour lire le journal.
