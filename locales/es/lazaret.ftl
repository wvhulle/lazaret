-brand-name = Lazaret
startup-settings-schema-missing = El esquema de configuración de { -brand-name } no está instalado. Ejecute make install o defina GSETTINGS_SCHEMA_DIR.
startup-unknown-page-name = { -brand-name } no tiene ninguna página llamada { $page }, así que abre Protección. Páginas: { $pages }

window-signature-sources = Fuentes de firmas
window-database-updates = Actualizaciones de la base de datos
window-scan-engine = Motor de análisis
window-access-scanning = Análisis en el acceso
window-scheduled-scans-page = Análisis programados
window-manual-scan = Análisis manual
window-removable-drives = Unidades extraíbles
window-detections-label = Detecciones
window-quarantine-label = Cuarentena
window-trusted-detections = Detecciones de confianza
window-sent-samples = Muestras enviadas
window-quarantine-database-not-locked = No se pudo bloquear la base de datos de cuarentena
window-quarantine-database-not-opened = No se pudo abrir la base de datos de cuarentena
window-file-operation-failed-error = Falló una operación de archivo: { $error }
window-lock-task-interrupted = Un paso de cuarentena en segundo plano se detuvo inesperadamente: { $error }
window-translator-credits = translator-credits
window-export-history-menu-item = Exportar historial…
window-clear-history-menu-item = Borrar historial…
window-preferences-label = Preferencias
window-about-menu-item = Acerca de { -brand-name }
window-quit-menu-item = Salir
window-export-history-heading = ¿Exportar el historial?
window-export-history-body = Guarda todos los análisis y actualizaciones de firmas registrados, con sus detalles.
window-cancel-history-button = Cancelar
window-json-label = JSON
window-csv-label = CSV
window-clear-history-heading = ¿Borrar el historial?
window-clear-history-body = Esto elimina de forma permanente todos los análisis y actualizaciones de firmas registrados. No se puede deshacer.
window-clear-label = Borrar todo
window-scanning-drive-title = Analizando { $drive }
window-open-removable-drives-button = Abrir unidades extraíbles
window-not-export-history-toast = No se pudo exportar el historial: { $error }
window-history-export-finished = Historial exportado
window-history-cleared = Historial borrado
window-not-clear-history-toast = No se pudo borrar el historial: { $error }
window-other-sources-unknown = Otras fuentes desconocidas
window-source-out-date = Una fuente está desactualizada
window-official-signatures = Solo firmas oficiales
window-official-count-other-source =
    { $count ->
        [one] Oficiales y { $count } fuente más
       *[other] Oficiales y { $count } fuentes más
    }
window-database-missing = Sin definiciones de virus
window-age-definitions-unknown = Antigüedad de las definiciones desconocida
window-last-update-failed = Falló la última actualización
window-definitions-dated-future = Definiciones con fecha futura
window-updated-label = Actualizado { $when }
window-definitions-label = Definiciones de { $when }
window-freshclam-refused = freshclam rechazado
window-unknown-whether-freshclam-trusted = Se desconoce si freshclam es de confianza
window-freshclam-copy-account-ignored = Se ignora una copia de freshclam en su cuenta
window-clamd-uses-older-signatures = clamd usa firmas más antiguas
window-clamd-answering = clamd responde
window-unit-active-not-answering = clamd no responde
window-unit-state-unknown-daemon = Estado de clamd desconocido
window-unit-stopped-daemon = clamd no está en ejecución
window-never-set-up-daemon = clamd no configurado
window-engine-missing = ClamAV no instalado
window-scanner-refused = clamscan rechazado
window-version-unverified = Versión de ClamAV desconocida
window-scanner-program-refused = Se rechaza un programa de análisis
window-unknown-whether-scanners-trusted = Se desconoce si los programas de análisis son de confianza
window-scanner-copy-account-ignored = Se ignora una copia del programa de análisis en su cuenta
window-runtime-changes-unknown = Cambios en tiempo de ejecución desconocidos
window-settings-reduced-runtime = Ajustes reducidos en tiempo de ejecución
window-waiting-daemon = Esperando a clamd
window-unit-state-unknown-access = Estado desconocido
window-never-set-up-access = No configurado
window-unit-stopped-access = clamonacc no está en ejecución
window-config-unreadable = clamd.conf ilegible
window-nothing-watched = Nada vigilado
window-checks-files-not-block = Comprueba archivos, no bloquea
window-checks-files-kernel-cannot = Comprueba archivos, el núcleo no puede bloquear
window-checks-files-blocking-unknown = Comprueba archivos, bloqueo desconocido
window-blocking-paths = Bloqueando { $paths }
window-folder-watch-limit-unknown = Límite de vigilancia de carpetas desconocido
window-mount-watch-conflicts = Conflictos en la vigilancia de montajes
window-clamd-user-not-excluded = clamd no excluido
window-whole-root-included = Vigila / directamente
window-watch-limit-low = Límite de vigilancia de carpetas bajo
window-timer-not-active = Temporizador inactivo
window-last-scan-failed = Falló el último análisis
window-next-scan = Próximo análisis { $when }
window-no-next-scan-planned = No hay ningún próximo análisis previsto
window-service-missing = Falta el servicio de análisis
window-service-unreadable = Servicio de análisis desconocido
window-lazaret-schedule = Programado por { -brand-name }
window-no-schedule = Ningún análisis programado
window-schedule-changed-outside-lazaret = Programación cambiada fuera de { -brand-name }
window-ready-scan = Listo para analizar
window-uses-slower-clamscan = Usa el clamscan más lento
window-exclusion-skip-malware = Una exclusión podría omitir malware
window-device-scan-running = Analizando { $drive }
window-device-scan-ended = Amenazas encontradas en { $drive }
window-journal-unreadable = Diario ilegible
window-following-failed = Se detuvo el seguimiento del diario
window-notifications-off = Notificaciones desactivadas
window-alerts-stop-lazaret-quits = Las alertas se detienen al salir de { -brand-name }
window-other-programs-attach-lazaret = Otros programas pueden conectarse a { -brand-name }
window-unknown-whether-programs-attach = Se desconoce si otros programas pueden conectarse
window-nothing-decide = Nada que decidir
window-count-needs-decision =
    { $count ->
        [one] { $count } necesita una decisión
       *[other] { $count } necesitan una decisión
    }
window-loading-label = Cargando…
window-empty-label = Vacía
window-count-item =
    { $count ->
        [one] { $count } elemento
       *[other] { $count } elementos
    }
window-quarantine-cannot-read = No se puede leer la cuarentena
window-nothing-trusted = Nada de confianza
window-count-item-trusted =
    { $count ->
        [one] { $count } elemento de confianza
       *[other] { $count } elementos de confianza
    }
window-signature-folder-unreadable = Carpeta de firmas ilegible
window-another-allowlist-silences-clamav = Otra lista de permitidos silencia a ClamAV
window-lazaret-allowlist-unreadable = La lista de permitidos de { -brand-name } es ilegible
window-sent-samples-cannot-read = No se pueden leer las muestras enviadas
window-nothing-sent = Nada enviado
window-count-refused-service =
    { $count ->
        [one] { $count } rechazada por su servicio
       *[other] { $count } rechazadas por sus servicios
    }
window-count-awaiting-review =
    { $count ->
        [one] { $count } pendiente de revisión
       *[other] { $count } pendientes de revisión
    }
window-count-sent =
    { $count ->
        [one] { $count } enviada
       *[other] { $count } enviadas
    }
window-signatures-label = Firmas
window-scans-label = Análisis
window-findings-label = Hallazgos
window-detections-come = De dónde vienen las detecciones
window-files-get-checked = Cómo se comprueban los archivos
window-found-label = Qué se encontró
window-needs-review = Requiere revisión
window-not-working = No funciona
window-waiting-another-part = Esperando a otro componente
window-summary-pending = Comprobando…
window-menu-tooltip = Menú
window-show-lazaret = Mostrar { -brand-name }
window-show-detections = Mostrar detecciones
window-quick-scan = Análisis rápido
window-update-definitions = Actualizar definiciones
window-quit-tray-item = Salir
window-quit-completely = Salir por completo
window-hide-tray = Minimizar a la bandeja del sistema
window-ask-each-time = Preguntar siempre
window-close-lazaret = ¿Cerrar { -brand-name }?
window-detection-alerts-stop-lazaret = Las alertas de detección se detienen mientras { -brand-name } no está en ejecución. ¿Mantenerlo en segundo plano o salir por completo?
window-cancel-close-button = Cancelar
window-quit-close-button = Salir
window-keep-running = Mantener en ejecución

scans-node-js-dependencies = Dependencias de Node.js
scans-git-repository-data = Datos del repositorio Git
scans-python-virtual-environment = Entorno virtual de Python
scans-build-output-directory = Directorio de salida de compilación
scans-distribution-output-directory = Directorio de salida de distribución
scans-python-bytecode-cache = Caché de código de bytes de Python
scans-remove-exclusion-tooltip = Eliminar
scans-preset-exclusions-title = Exclusiones predefinidas
scans-common-directories-exclude-description = Directorios habituales que se excluirán del análisis. Los cambios se guardan inmediatamente.
scans-custom-exclusions-title = Exclusiones personalizadas
scans-enter-absolute-path-description = Introduzca una ruta absoluta o un patrón como *.tmp. Los cambios se guardan inmediatamente.
scans-path-exclude-tmp-title = /path/to/exclude o *.tmp
scans-prefer-clamd-when-available = Automático
scans-always-clamd = clamd (demonio)
scans-always-clamscan = clamscan (independiente)
scans-options-title = Opciones
scans-backend-title = Motor de análisis
scans-automatic-uses-daemon-subtitle = Automático usa el demonio cuando responde y clamscan en caso contrario.
scans-show-live-scan-title = Mostrar el progreso del análisis en tiempo real
scans-display-files-scanned-subtitle = Mostrar los archivos que se analizan en tiempo real.
scans-not-added-history-error = No se pudo añadir el análisis al historial: { $error }
scans-no-profile-manual = Sin perfil (manual)
scans-removable-drive-scanned-title = Se está analizando una unidad extraíble. Inicie este análisis cuando haya terminado.
scans-open-removable-drives-button = Abrir unidades extraíbles
scans-detecting-backend = Comprobando…
scans-backend-undetermined = Motor: desconocido, no se pudieron localizar los programas de análisis: { $error }
scans-backend-clamd-daemon = Motor: clamd (demonio)
scans-backend-clamscan-standalone = Motor: clamscan (independiente)
scans-scan-title = Escanear
scans-preparing-label = Preparando…
scans-cancelling-label = Cancelando…
scans-scanning-name = Analizando { $name }
scans-scanning-count-location =
    { $count ->
        [one] Analizando { $count } ubicación
       *[other] Analizando { $count } ubicaciones
    }
scans-cancelled-user = Escaneo cancelado por el usuario
scans-threats-found-scan = Amenazas encontradas
scans-not-finish-cleanly = El análisis no terminó correctamente
scans-no-threats-found = No se encontraron amenazas
scans-not-create-eicar-test = No se pudo crear el archivo de prueba EICAR: { $error }
scans-add-files-tooltip = Añadir archivos
scans-add-folder-manual-tooltip = Añadir carpeta
scans-more-tooltip = Más
scans-run-eicar-test-label = Ejecutar prueba EICAR
scans-copy-results-label = Copiar resultados
scans-export-results-label = Exportar resultados…
scans-start-scan-label = Iniciar análisis
scans-cancel-manual-label = Cancelar
scans-lazaret-refuses-clamav-title = { -brand-name } rechaza un programa de ClamAV que su cuenta podría sustituir
scans-details-button = Detalles
scans-profile-title = Perfil de análisis
scans-profile-short-title = Perfil
scans-manage-profiles-tooltip = Gestionar perfiles
scans-targets-manual-title = Objetivos de escaneo
scans-drop-files-folders-label = Suelte archivos o carpetas para analizarlos
scans-add-files-label = Añadir archivos…
scans-add-folder-label = Añadir carpeta…
scans-found-far-scan-title = Encontrado hasta ahora
scans-new-scan-button = Nuevo análisis
scans-recent-scans-title = Análisis recientes
scans-started-lazaret-newest-description = Análisis iniciados desde { -brand-name }, los más recientes primero. Seleccione uno para leer su transcripción.
scans-choose-files-scan = Elegir archivos para analizar
scans-choose-folder-scan = Elegir una carpeta para analizar
scans-quick-scan = Análisis rápido
scans-full-scan = Análisis completo
scans-home-folder = Carpeta personal
scans-fast-scan-downloads-folder = Análisis rápido de la carpeta Descargas para detectar amenazas rápidamente
scans-comprehensive-system-wide-scan = Análisis exhaustivo de todo el sistema en todos los directorios accesibles
scans-user-home-directory-personal = Análisis del directorio personal del usuario y sus archivos personales
scans-profile-file-unavailable = No se pudo acceder al archivo de perfil: { $error }
scans-profile-data-invalid = Los datos del perfil no son válidos: { $error }
scans-profile-file-not-replaced = No se pudo sustituir el archivo de perfil: { $error }
scans-profile-task-interrupted = Se interrumpió la tarea del perfil: { $error }
scans-profile-name-missing = Es obligatorio introducir el nombre de perfil
scans-profile-name-too-long = El nombre debe tener 50 caracteres o menos
scans-profile-name-taken = Otro perfil ya usa este nombre
scans-profile-targets-missing = Añada al menos un destino
scans-edit-profile-title = Editar perfil
scans-save-label = Guardar
scans-name-title = Nombre
scans-description-title = Descripción
scans-targets-profile-title = Objetivos
scans-add-folder-profile-tooltip = Añadir carpeta
scans-add-path-title = Añadir ruta
scans-exclusions-title = Exclusiones
scans-added-exclusions-preferences-description = Se añaden a las exclusiones de Preferencias mientras se ejecuta este perfil.
scans-add-pattern-title = Añadir patrón
scans-cancel-profile = Cancelar
scans-defaults-restored = Perfiles predeterminados restaurados
scans-profile-deleted = Perfil eliminado
scans-profile-imported = Perfil importado
scans-manage-profiles-title = Gestionar perfiles
scans-restore-defaults-tooltip = Restaurar valores predeterminados
scans-new-profile-tooltip = Nuevo perfil
scans-import-tooltip = Importar
scans-profiles-title = Perfiles de análisis
scans-select-profile-edit-description = Seleccione un perfil para editarlo o usarlo para analizar
scans-restore-default-profiles = Restaurar perfiles predeterminados
scans-restore-built-default-profiles = ¿Restaurar los perfiles predeterminados integrados? Se restablecerán todas las personalizaciones de los perfiles predeterminados; los perfiles personalizados se conservarán.
scans-delete-profile = Eliminar perfil
scans-delete-profile-name = ¿Eliminar el perfil «{ $name }»?
scans-delete-label = Eliminar
scans-profile-exported = Perfil exportado
scans-remove-profile-tooltip = Eliminar
scans-count-target =
    { $count ->
        [one] { $count } destino
       *[other] { $count } destinos
    }
scans-default-profile = Perfil predeterminado
scans-run-tooltip = Ejecutar
scans-edit-tooltip = Editar
scans-delete-tooltip = Eliminar
scans-export-tooltip = Exportar
scans-remove-target-tooltip = Eliminar
scans-check-again-access-tooltip = Comprobar de nuevo
scans-checking-access-tooltip = Comprobando
scans-status-access-title = Estado
scans-access-settings-changed-description = Ajustes de análisis en el acceso cambiados mediante { -brand-name }, según los registros del sistema. El software que se ejecuta con su usuario no puede eliminar estos registros.
scans-no-access-settings-title = No se ha cambiado ningún ajuste de análisis en el acceso mediante { -brand-name }
scans-clamonacc-log-title = Registro de clamonacc
scans-clamonacc-not-written-title = clamonacc no ha escrito en el diario
scans-checking-access-scanning = Comprobando el análisis en el acceso…
scans-command-copied-clipboard-access = Comando copiado al portapapeles
scans-no-readable-config = No se encontró ningún clamd.conf legible
scans-reading-task-aborted-clamd = La lectura de clamd.conf se detuvo antes de terminar
scans-watched-mount = { $path } (montaje completo)
scans-nothing-watched = Ninguno configurado
scans-default-limit = El valor predeterminado de ClamAV
scans-configuration-label = Configuración
scans-watched-folders = Carpetas vigiladas
scans-largest-file-scanned = Archivo más grande analizado
scans-no-kernel-config-found = no existe ni /proc/config.gz ni el archivo de configuración del núcleo en /boot
scans-kernel-config-unreadable = no se pudo leer la configuración del núcleo
scans-compressed-config-corrupt = no se pudo descomprimir /proc/config.gz
scans-kernel-release-unreadable = no se pudo leer la versión del núcleo en ejecución
scans-reading-task-aborted-config = la lectura de la configuración del núcleo se detuvo antes de terminar
scans-blocks-detected-files = Los archivos detectados se bloquean
scans-prevention-off = Los archivos se comprueban, pero los archivos detectados no se bloquean
scans-prevention-conflicts-mounts = Los archivos se comprueban, pero los archivos detectados no se bloquean, porque OnAccessPrevention no funciona con OnAccessMountPath
scans-kernel-cannot-block = Los archivos se comprueban, pero este núcleo no puede bloquear los archivos detectados, porque se compiló sin CONFIG_FANOTIFY_ACCESS_PERMISSIONS
scans-kernel-support-unknown = Los archivos se comprueban, pero se desconoce si el núcleo puede bloquear los archivos detectados, porque { $reason }
scans-config-unreadable-monitor = En ejecución, pero no se pudo leer clamd.conf, así que se desconoce qué se comprueba y si los archivos detectados se bloquean
scans-running-no-folders-status = En ejecución, pero no se vigila ninguna carpeta ni montaje
scans-running-detected-status = En ejecución, pero no se vigila ninguna carpeta ni montaje, y los archivos detectados no se bloquearían
scans-unit-state-unknown-access = Se desconoce si el análisis en el acceso se ejecuta, porque systemd no pudo informar de ello
scans-never-set-up-access = El análisis en el acceso no está configurado
scans-unit-stopped-access = { $unit } no está en ejecución
scans-access-blocking-title = Bloqueo en el acceso
scans-clamonacc-checks-files-watched = clamonacc comprueba los archivos de las carpetas vigiladas a medida que se abren. Con la prevención activada, el núcleo deniega el acceso a un archivo detectado, por eso no se puede abrir aunque siga ahí.
scans-mount-watching = Vigilancia de montajes
scans-onaccessmountpath-watches-whole-mount = OnAccessMountPath vigila un montaje completo mediante fanotify. ClamAV lo documenta como incompatible con OnAccessPrevention, OnAccessIncludePath, OnAccessExcludePath y OnAccessExtraScanning, así que los archivos detectados en un montaje vigilado nunca se bloquean. Vigile carpetas con OnAccessIncludePath para bloquear.
scans-onaccessmountpath-cannot-combined-options = OnAccessMountPath no se puede combinar con { $options }
scans-loop-protection = Protección contra bucles de análisis
scans-clamd-opens-every-file = clamd abre cada archivo que se le pide analizar. Salvo que OnAccessExcludeUname u OnAccessExcludeUID indiquen el usuario con el que se ejecuta clamd, clamonacc ve esas lecturas y pide a clamd que vuelva a analizar los mismos archivos.
scans-clamd-not-excluded-clamonacc = clamd no está excluido, así que clamonacc puede analizar sus propias lecturas sin fin
scans-watching-whole-file-system = Vigilando todo el sistema de archivos
scans-clamonacc-refuses-onaccessincludepath-include = clamonacc rechaza OnAccessIncludePath /. Incluya las carpetas que reciben archivos, como los directorios personales, o vigile un montaje con OnAccessMountPath.
scans-onaccessincludepath-rejected-clamonacc = clamonacc rechaza OnAccessIncludePath /
scans-folder-watch-limit = Límite de vigilancia de carpetas
scans-path-watched-fs-inotify = { $path } está vigilado, pero fs.inotify.max_user_watches es solo { $limit }, así que las carpetas que superan el límite quedan sin comprobar
scans-unknown-whether-path-fits = Se desconoce si { $path } cabe en el límite de vigilancia de carpetas, porque no se pudo leer fs.inotify.max_user_watches
scans-clamonacc-watches-every-folder = clamonacc vigila con inotify cada carpeta bajo una ruta incluida, una vigilancia por carpeta. Un directorio personal completo o un árbol mayor puede necesitar más vigilancias de las que permite el antiguo límite predeterminado de 8192. Aumente fs.inotify.max_user_watches con sysctl.
scans-device-auto-scan-title = Análisis automático de dispositivos
scans-automatically-scan-removable-description = Analizar automáticamente las unidades extraíbles y externas cuando se conectan.
scans-enable-device-auto-title = Activar análisis automático de dispositivos
scans-max-device-size-title = Tamaño máximo del dispositivo (GB)
scans-no-size-limit-subtitle = 0 significa sin límite de tamaño.
scans-skip-battery-device-title = Omitir si se usa la batería
scans-drive-scan-not-added = No se pudo añadir el análisis de la unidad al historial: { $error }
scans-device-auto-scan = Análisis automático de dispositivos
scans-device-connected-during-another = Se conectó un dispositivo durante otro análisis, así que no se analizó.
scans-removable-drives-title = Unidades extraíbles
scans-cancel-removable-label = Cancelar
scans-found-far-removable-title = Encontrado hasta ahora
scans-drives-session-title = Unidades de esta sesión
scans-drives-connected-since-description = Unidades conectadas desde que se inició { -brand-name }, las más recientes primero, y qué pasó con cada una. Esta lista solo se conserva hasta que salga de { -brand-name }.
scans-no-drive-connected-title = Todavía no se ha conectado ninguna unidad
scans-close-results-button = Cerrar resultados
scans-drive-scans-title = Análisis de unidades
scans-drives-lazaret-scanned-description = Unidades que { -brand-name } analizó al conectarse, las más recientes primero. Seleccione una para leer su transcripción.
scans-off-label = Desactivado
scans-every-drive = Analiza todas las unidades
scans-drives-up-size = Analiza unidades de hasta { $size }
scans-skipped-larger-than-limit = No analizada: { $size } supera el límite de { $limit }
scans-skipped-battery = No analizada: el equipo funcionaba con batería
scans-skipped-auto-scan-off = No analizada: el análisis automático de dispositivos está desactivado
scans-skipped-while-scanning = No analizada: había otro análisis en curso
scans-size-unreadable = No analizada: no se pudo leer el tamaño de la unidad
scans-drive-settling = Comprobando la unidad…
scans-drive-scanning = Analizando…
scans-threats-detected = Analizada, amenazas encontradas
scans-no-threats-detected = Analizada, no se encontraron amenazas
scans-finished-problems = Analizada, pero el análisis no terminó correctamente
scans-scan-cancelled = Análisis cancelado
scans-device-scan-running = Analizando { $drive }
scans-drive-scan = Análisis de unidad
scans-check-again-scan-tooltip = Comprobar de nuevo
scans-checking-scan-tooltip = Comprobando
scans-status-scan-title = Estado
scans-engine-detection-settings-description = Ajustes del motor y de detección cambiados mediante { -brand-name }, según los registros del sistema. El software que se ejecuta con su usuario no puede eliminar estos registros.
scans-no-engine-detection-title = No se ha cambiado ningún ajuste del motor ni de detección mediante { -brand-name }
scans-clamd-log-title = Registro de clamd
scans-no-clamd-log-title = No se encontró ningún registro de clamd
scans-checking-scan-engine = Comprobando el motor de análisis…
scans-command-copied-clipboard-scan = Comando copiado al portapapeles
scans-program-start-failed = No se pudo iniciar clamscan: { $error }
scans-probe-timed-out = clamscan no respondió a tiempo
scans-scanner-refused = se rechazó { $path }, porque su cuenta puede sustituirlo
scans-engine-missing = No se encontró ClamAV en este equipo
scans-clamav-installed = ClamAV instalado
scans-clamav-engine-recognizes-malware = ClamAV es el motor que reconoce el malware. { -brand-name } usa la copia instalada en este equipo y nunca incluye la suya propia.
scans-clamd-still-uses-signatures = clamd todavía usa las firmas { $loaded }, pero en el disco está { $disk }
scans-clamd-answering = clamd responde
scans-unit-active-not-answering = { $unit } está en ejecución, pero clamd no responde. Puede que todavía esté cargando firmas.
scans-unit-state-unknown-daemon = clamd no responde y systemd no pudo informar de su estado
scans-unit-stopped-daemon = { $unit } no está en ejecución
scans-never-set-up-daemon = clamd no está configurado, así que los análisis recurren al clamscan más lento
scans-same-version-disk = La misma versión que en el disco
scans-loaded-disk = { $loaded }, mientras que en el disco está { $disk }
scans-unknown-loaded-stored-versions = Desconocido, no se pudieron comparar las versiones cargada y almacenada
scans-loaded-signatures = Firmas cargadas
scans-daemon-running = Demonio en ejecución
scans-clamd-keeps-signatures-loaded = clamd mantiene las firmas cargadas en memoria. El bloqueo en el acceso y los análisis programados le envían sus archivos.
scans-socket-protected = Solo el sistema puede sustituir { $path }
scans-socket-replaceable = Su cuenta puede sustituir { $replaceable }, así que un programa que se ejecute con su usuario podría poner un clamd falso en { $path } que informe de que todos los archivos están limpios
scans-socket-missing = No hay ningún socket de clamd en { $path }, así que clamd no está en ejecución o escucha en otro lugar
scans-socket-not-checkable = El entorno aislado de Flatpak no puede ver quién es el propietario del socket de clamd
scans-clamd-socket-protected = Socket de clamd protegido
scans-socket-mechanism = Los análisis llegan a clamd a través de su socket. Deje que clamd lo cree en un directorio en el que solo root o el usuario clamav puedan escribir, como /run/clamav, para que ningún programa que se ejecute con su usuario pueda poner un clamd falso en su lugar.
scans-socket-inspecting = Comprobando quién puede sustituir el socket de clamd…
scans-timer-inactive = No programado
scans-next-run-time = Próxima ejecución: { $time }
scans-scheduled-time-next-run = Programado. Todavía no se conoce la hora de la próxima ejecución.
scans-systemd-unavailable = Los análisis programados necesitan una sesión de usuario de systemd, que no está disponible.
scans-system-already-runs-scheduled = Su sistema ya ejecuta análisis programados, así que estos ajustes están bloqueados para evitar una segunda programación. Próximo análisis: { $time }. Cámbielo en la configuración del sistema.
scans-system-scan-timer-settings = Su sistema tiene un temporizador de análisis, así que estos ajustes están bloqueados para evitar una segunda programación. Cámbielo en la configuración del sistema.
scans-reading-user-timer = Comprobando…
scans-inspecting-system-timer = Comprobando si su sistema ya ejecuta análisis programados. Estos ajustes permanecen bloqueados hasta entonces.
scans-system-timer-unknown = { -brand-name } no pudo comprobar si su sistema ya ejecuta análisis programados, así que estos ajustes están bloqueados para evitar una segunda programación: { $error }
scans-scan-time-unreadable = Introduzca la hora del análisis como HH:MM, por ejemplo 02:00.
scans-unit-files-unwritable = Error al crear los archivos de systemd: { $error }
scans-timer-not-applied-error = No se pudo aplicar el temporizador: { $error }
scans-hourly-label = Cada hora
scans-daily-label = Diariamente
scans-weekly-label = Semanalmente
scans-monthly-label = Mensualmente
scans-monday-label = lunes
scans-tuesday-label = martes
scans-wednesday-label = miércoles
scans-thursday-label = jueves
scans-friday-label = viernes
scans-saturday-label = sábado
scans-sunday-label = domingo
scans-schedule-title = Programación
scans-apply-label = Aplicar
scans-enable-scheduled-scans-title = Activar análisis programados
scans-frequency-title = Frecuencia de análisis
scans-time-24-hour-title = Hora del análisis (formato de 24 horas)
scans-day-week-title = Día de la semana
scans-used-weekly-scans-subtitle = Se utiliza para análisis semanales.
scans-day-month-title = Día del mes
scans-used-monthly-scans-subtitle = Se utiliza para análisis mensuales.
scans-targets-separated-commas-title = Objetivos del análisis (separados por comas; si está vacío, se analiza su carpeta personal)
scans-skip-battery-schedule-title = Omitir si se usa la batería
scans-not-start-scheduled-subtitle = No iniciar análisis programados mientras se use alimentación por batería.
scans-auto-quarantine-title = Cuarentena automática
scans-automatically-quarantine-threats-subtitle = Poner automáticamente en cuarentena las amenazas encontradas por los análisis programados. Mientras { -brand-name } está abierto, las amenazas solo se notifican.
scans-review-button = Revisar
scans-check-again-scheduled-tooltip = Comprobar de nuevo
scans-checking-scheduled-tooltip = Comprobando
scans-system-scan-title = Análisis del sistema
scans-folders-scanned-title = Carpetas analizadas
scans-run-now-label = Ejecutar ahora
scans-starting-scan-tooltip = Iniciando el análisis…
scans-status-scheduled-title = Estado
scans-scheduled-runs-title = Ejecuciones programadas
scans-lazaret-schedule-ran-description = Análisis ejecutados por la programación de { -brand-name }, los más recientes primero. Seleccione uno para leer su transcripción.
scans-checking-schedule = Comprobando la programación…
scans-command-copied-clipboard-scheduled = Comando copiado al portapapeles
scans-not-scheduled = No programado
scans-next-scan = Próximo análisis
scans-threats-found-scheduled = amenazas encontradas
scans-nothing-found = no se encontró nada
scans-no-completed-scan = Aún no se ha ejecutado
scans-last-scan = Último análisis
scans-unknown-label = Desconocido
scans-system-timer = El temporizador del sistema no está activo
scans-last-scan-ended-error = El último análisis terminó con un error
scans-next-scan-time = Próximo análisis: { $time }
scans-timer-starts-scan-service = El temporizador inicia un servicio de análisis que no está instalado
scans-service-not-read-systemd = No se pudo leer el servicio de análisis desde systemd
scans-lazaret-schedule = Programado por { -brand-name }
scans-no-schedule = No hay ningún análisis programado
scans-not-scheduled-timer-active = No programado, porque el temporizador no está activo
scans-folders-scanned = Carpetas analizadas
scans-scheduled-scan = Análisis programado
scans-scheduled-scan-catches-files = Un análisis programado detecta archivos que no se abrieron o que solo se volvieron detectables tras una actualización de firmas.
scans-unit-names-no-folders = La unidad de análisis no indica ninguna carpeta
scans-system-scan-service-unit = El servicio de análisis de su sistema { $unit } analiza estas carpetas con los ajustes propios del sistema. Ejecutarlo requiere autorización de administrador.
scans-needs-daemon-see-scan = Requiere el demonio, consulte Motor de análisis
scans-bus-call-failed = systemd no respondió: { $error }
scans-unrecognized-load-state = systemd informó de un estado de unidad que { -brand-name } no reconoce
scans-found-malware = El análisis encontró malware
scans-unit-finished = El análisis terminó
scans-not-finish-cleanly-result = El análisis no terminó correctamente: { $result }
scans-authorization-refused = Iniciar el análisis requiere autorización de administrador
scans-not-started-error = No se pudo iniciar el análisis: { $error }

findings-open-preferences = Abrir preferencias
findings-journal-unreadable-alerts = No se puede leer el diario del sistema. Añada su usuario al grupo systemd-journal.
findings-notifications-turned-off-preferences = Las notificaciones están desactivadas en las preferencias
findings-alerts-stop-quit-lazaret = Las alertas se detienen al salir de { -brand-name }, porque no está configurado para ejecutarse en segundo plano
findings-running-background-following-journal = En ejecución en segundo plano y siguiendo el diario
findings-alerts-label = Alertas
findings-lazaret-follows-system-journal = { -brand-name } sigue el diario del sistema y le avisa cuando se detecta algo. Solo puede hacerlo mientras está en ejecución, así que debería ejecutarse en segundo plano.
findings-on-label = Activado
findings-off-label = Desactivado
findings-running-label = En ejecución
findings-not-running = Detenido
findings-unit-not-found = No configurado
findings-systemd-unavailable = Desconocido, no se puede contactar con systemd
findings-following-journal-access = Seguido
findings-journal-unreadable-access = No se puede leer
findings-following-failed = Seguimiento detenido
findings-notifications-label = Notificaciones
findings-background-service = Servicio en segundo plano
findings-system-journal = Diario del sistema
findings-attach-unknown-setting = Se desconoce si otros programas pueden conectarse, porque no se puede leer el ajuste del núcleo: { $reason }
findings-attach-unknown-reports = Se desconoce si otros programas pueden conectarse, porque el núcleo informa de un valor que { -brand-name } no reconoce
findings-attach-unknown-check = Se desconoce si otros programas pueden conectarse, porque la comprobación se detuvo antes de terminar
findings-attach-restricted = Los programas no pueden conectarse a otros programas que ejecute, como { -brand-name }
findings-any-program-may-attach = Cualquier programa que ejecute puede conectarse a { -brand-name } y cambiar lo que muestra, incluidas las detecciones y las decisiones que tome aquí
findings-yama-not-loaded = Este núcleo no restringe las conexiones, así que cualquier programa que ejecute puede conectarse a { -brand-name } y cambiar lo que muestra
findings-other-programs-cannot-attach = Otros programas no pueden conectarse a { -brand-name }
findings-program-attaches-lazaret-hide = Un programa que se conecte a { -brand-name } puede ocultar alertas o cambiar sus decisiones. Establecer kernel.yama.ptrace_scope a 1 permite que solo el proceso padre de un programa se conecte a él.
findings-file-owned-other-user = Este archivo pertenece a otro usuario, así que no se puede mover ni eliminar desde aquí
findings-file-not-checkable = No se pudo comprobar este archivo, así que no se puede mover ni eliminar desde aquí
findings-removal-failed = No se pudo eliminar el archivo: { $error }
findings-file-check-interrupted = La comprobación del archivo se detuvo inesperadamente
findings-history-not-updated = No se pudo guardar su decisión: { $error }
findings-journal-unreadable-list = { -brand-name } no puede leer el diario del sistema, así que las nuevas detecciones en segundo plano no aparecerán aquí. Esto no significa que no se haya encontrado nada. Añada su usuario al grupo systemd-journal.
findings-loading-detections = Cargando detecciones…
findings-nothing-detected-background = No se ha detectado nada en segundo plano.
findings-access-scanning-scheduled-scans = Lo que han encontrado el análisis en el acceso y los análisis programados.
findings-detections-title = Detecciones
findings-back-detections-tooltip = Volver a las detecciones
findings-alerts-title = Alertas
findings-detections-reach-keeps-description = Cómo le llegan las detecciones y qué impide que otros programas las oculten.
findings-check-again-background-tooltip = Comprobar de nuevo
findings-checking-background-tooltip = Comprobando
findings-detection-title = Detección
findings-copied-clipboard-toast = Copiado al portapapeles
findings-command-copied-clipboard-toast = Comando copiado al portapapeles
findings-detections-lazaret-stopped = No se pueden contar las detecciones ocurridas mientras { -brand-name } no estaba en ejecución, porque no se pudo leer dónde dejó de leer el diario del sistema: { $error }
findings-detections-not-read-error = No se pudieron leer las detecciones: { $error }
findings-malware-blocked-signature = Malware bloqueado: { $signature }
findings-malware-found-signature = Malware encontrado: { $signature }
findings-count-detection-not-running =
    { $count ->
        [one] { $count } detección mientras { -brand-name } no estaba en ejecución
       *[other] { $count } detecciones mientras { -brand-name } no estaba en ejecución
    }
findings-detections-not-running-detection = No se pueden contar las detecciones ocurridas mientras { -brand-name } no estaba en ejecución
findings-lazaret-not-read-stopped = { -brand-name } no pudo leer dónde dejó de seguir el diario del sistema, así que solo vigila las nuevas detecciones. Consulte la página Detecciones y el diario del sistema por si se encontró algo mientras estaba cerrado.
findings-quarantine-incident = Cuarentena
findings-ask-incident-deletion = Eliminar…
findings-delete-incident-file = Eliminar permanentemente
findings-cancel-incident-deletion = Cancelar
findings-dismiss-incident = Conservar
findings-look-up-incident-signature = Consultar
findings-open-online-services = Añadir clave…
findings-submit-incident-sample = Enviar…
findings-restored-quarantine = Restaurado de la cuarentena
findings-moved-quarantine = Movido a cuarentena
findings-deleted-label = Eliminado
findings-kept-label = Conservado por usted
findings-belongs-another-user = Pertenece a otro usuario
findings-blocked-needs-decision = Bloqueado, requiere una decisión
findings-not-blocked-needs-decision = No bloqueado, requiere una decisión
findings-needs-decision = Requiere una decisión
findings-no-longer-there-detection = Ya no está
findings-state-unknown-detection = No se pudo comprobar el archivo
findings-signature-test-file-state = { $signature } (archivo de prueba) · { $state }
findings-signature-state = { $signature } · { $state }
findings-access-schedule = El análisis en el acceso y el análisis programado
findings-access-only = Análisis en el acceso
findings-schedule-only = El análisis programado
findings-file-label = Archivo
findings-signature-source = Fuente de la firma
findings-found-label = Encontrado por
findings-last-seen = Visto por última vez
findings-times-seen = Veces visto
findings-detected-label = Detectado
findings-signature-matches-harmless-eicar = Esta firma coincide con el patrón de prueba inofensivo EICAR, que existe para comprobar que la detección funciona.
findings-clamav-matched-file-against = ClamAV ha encontrado que este archivo coincide con una firma. Una coincidencia es una prueba sólida, pero hay falsos positivos, sobre todo con herramientas empaquetadas o de administración.
findings-signature-first-seen-time = { $signature }, visto por primera vez { $time }
findings-opening-file-refused-why = Se deniega la apertura de este archivo, por eso no se abre
findings-not-opened-again-after = No se ha vuelto a abrir desde su decisión
findings-no-longer-there-blocked = El archivo ya no está
findings-not-blocked-scheduled-scan = No bloqueado: lo encontró el análisis programado y todavía se puede abrir
findings-not-blocked-right-now = No bloqueado ahora mismo: todavía se puede abrir
findings-state-unknown-blocked = No se pudo comprobar si el archivo está bloqueado
findings-blocked-label = Bloqueado
findings-access-prevention-kernel-refuses = Con la prevención en el acceso, el núcleo se niega a abrir un archivo detectado en una carpeta vigilada. El archivo permanece donde está hasta que decida qué hacer con él.
findings-request-failed = No se pudo contactar con MalwareBazaar: { $error }
findings-answer-unreadable = MalwareBazaar envió una respuesta que no se pudo leer
findings-key-refused = MalwareBazaar rechazó la clave de API
findings-query-refused = MalwareBazaar respondió: { $status }
findings-looking-up = Consultando…
findings-key-missing-lookup = Añada su clave de abuse.ch en Preferencias para consultar las detecciones.
findings-no-known-samples-match = Ninguna muestra conocida coincide con esta firma.
findings-unknown-label = desconocido
findings-count-known-samples-families = { $count } muestras conocidas. Familias: { $families }. Vista por primera vez: { $date }.
findings-ask-malwarebazaar-signature-description = Consultar a MalwareBazaar sobre esta firma. Solo se envía su nombre.
findings-key-missing-choices = MalwareBazaar necesita una clave gratuita de abuse.ch
findings-stop-asking-file-description = Dejar de preguntar por este archivo
findings-delete-file-permanently-description = ¿Eliminar este archivo permanentemente? No se puede deshacer.
findings-go-back-without-description = Volver sin eliminar
findings-move-somewhere-safe-description = Moverlo a un lugar seguro. Puede restaurarlo más tarde.
findings-remove-good-description = Eliminarlo definitivamente
findings-know-file-fine-description = Sabe que este archivo es inofensivo. ClamAV lo seguirá notificando en el próximo análisis; márquelo como de confianza desde Cuarentena o desde un resultado de análisis para evitarlo.
findings-decision-title = Su decisión
findings-unfamiliar-quarantine-keeping-file = Si no lo reconoce, póngalo en cuarentena. Conservar un archivo solo hace que { -brand-name } deje de preguntar: el análisis en el acceso lo sigue bloqueando hasta que se excluya la ruta en la configuración de ClamAV, como se muestra aquí.
findings-quarantined-files-restored-quarantine = Los archivos en cuarentena se pueden restaurar desde la página Cuarentena. Un archivo que haya conservado sigue bloqueado por el análisis en el acceso hasta que se excluya su ruta en la configuración de ClamAV.
findings-file-other-owner-decision = Este archivo pertenece a otro usuario, así que { -brand-name } no puede moverlo ni eliminarlo
findings-state-unknown-review = { -brand-name } no pudo comprobar este archivo, así que no puede moverlo ni eliminarlo
findings-choose-happens-file = Elija qué hacer con este archivo
findings-malwarebazaar-label = MalwareBazaar
findings-resolved-title = Resuelto
findings-resolved-detection-opens-again = Una detección resuelta se vuelve a abrir si ClamAV encuentra de nuevo el mismo archivo más adelante, por ejemplo, si lo descarga una segunda vez.
findings-waiting-decision = Esperando su decisión
findings-resolved-state-step = { $state }, { $time }
findings-send-quarantined-copy-description = Enviar la copia en cuarentena a ClamAV, MalwareBazaar o VirusTotal. Un falso positivo notificado a ClamAV se corrige para todos en una actualización de firmas posterior.
findings-cancel-quarantine = Cancelar
findings-delete-entries = Eliminar
findings-scanner-program-not-installed = el programa de análisis no está instalado
findings-scanner-program-replaced-account = su cuenta puede sustituir el programa de análisis en { $program } a través de { $path }
findings-content-pipe-failed = no se pudo pasar el archivo al programa de análisis: { $error }
findings-scanner-spawn-failed = no se pudo iniciar el programa de análisis: { $error }
findings-verdict-timed-out = el análisis tardó demasiado
findings-detection-settings-unreadable = no se pudieron leer los ajustes de detección de clamd: { $error }
findings-program-lookup-failed = no se pudieron localizar los programas de análisis: { $error }
findings-payload-unreadable = no se pudo leer la copia en cuarentena: { $error }
findings-restore-file-heading = ¿Restaurar este archivo?
findings-clamav-no-longer-verdict = ClamAV ya no detecta este archivo con las firmas actuales.
findings-restore-button = Restaurar
findings-restore-file-clamav-heading = ¿Restaurar un archivo que ClamAV aún detecta?
findings-clamav-still-detects-verdict = ClamAV aún detecta este archivo como { $detection } con las firmas actuales. Restáurelo solo si está seguro de que es un falso positivo y considere marcarlo antes como de confianza.
findings-restore-anyway-button = Restaurar de todos modos
findings-restore-unchecked-file-heading = ¿Restaurar un archivo sin comprobar?
findings-file-not-checked-verdict = No se pudo volver a comprobar el archivo antes de restaurarlo: { $error }
findings-location-outside-home-folder = Esta ubicación está fuera de su carpeta personal.
findings-file-goes-back = El archivo vuelve a:
findings-entry-count =
    { $count ->
        [one] { $count } elemento
       *[other] { $count } elementos
    }
findings-clear-old-items = ¿Borrar los elementos antiguos?
findings-count-quarantined-file-older =
    { $count ->
        [one] Se eliminará permanentemente { $count } archivo en cuarentena con más de 30 días.
       *[other] Se eliminarán permanentemente { $count } archivos en cuarentena con más de 30 días.
    }
findings-delete-quarantined-file = ¿Eliminar el archivo en cuarentena?
findings-file-permanently-deleted = El archivo se eliminará permanentemente.
findings-quarantine-title = Cuarentena
findings-more-tooltip = Más
findings-clear-old-items-label = Borrar elementos antiguos…
findings-check-again-label = Comprobar de nuevo
findings-scan-every-quarantined-tooltip = Analizar todos los archivos en cuarentena con las firmas actuales
findings-refresh-tooltip = Actualizar
findings-search-tooltip = Buscar
findings-search-threat-name-placeholder = Buscar por nombre de amenaza o ruta
findings-quarantine-storage-title = Almacenamiento de cuarentena
findings-detected-threats-isolated-description = Amenazas detectadas aisladas de su sistema. Despliegue una entrada para comprobar su reputación.
findings-total-size-title = Tamaño total
findings-dismiss-label = Descartar
findings-quarantine-cannot-read-title = No se puede leer la cuarentena
findings-loading-quarantine = Cargando la cuarentena…
findings-quarantine-empty = La cuarentena está vacía
findings-hash-copied-clipboard = Hash copiado al portapapeles
findings-every-quarantined-file-still = Todos los archivos en cuarentena se siguen detectando o no se pudieron comprobar
findings-count-quarantined-file-no =
    { $count ->
        [one] { $count } archivo en cuarentena ya no se detecta. Compruébelo antes de restaurarlo.
       *[other] { $count } archivos en cuarentena ya no se detectan. Compruébelos antes de restaurarlos.
    }
findings-count-report-clamav-settled =
    { $count ->
        [one] Las nuevas firmas resuelven { $count } informe a ClamAV
       *[other] Las nuevas firmas resuelven { $count } informes a ClamAV
    }
findings-checking-label = Comprobando…
findings-no-longer-detected = Ya no se detecta
findings-still-detected = Aún se detecta
findings-integrity-unknown-hash = Desconocida. Este registro de cuarentena no tiene un hash utilizable, así que el archivo no se puede verificar, comprobar en línea, marcar como de confianza ni restaurar. Todavía puede eliminarlo.
findings-not-checked = Aún sin comprobar
findings-integrity-unknown-label = Integridad desconocida
findings-restore-label = Restaurar
findings-delete-entry-label = Eliminar
findings-reputation-title = Reputación
findings-file-sha-256-subtitle = Solo se envía el hash SHA-256 del archivo. El archivo nunca sale de este equipo.
findings-check-label = Comprobar
findings-add-key-label = Añadir clave
findings-view-report-label = Ver informe
findings-false-positive-title = Falso positivo
findings-trust-stops-clamav-subtitle = Confiar… hace que ClamAV deje de notificar este archivo exacto en este equipo y lo incluye en Detecciones de confianza; restáurelo después. Notificar… lo envía a Cisco Talos, para que una actualización de firmas posterior corrija la detección para todos.
findings-report-label = Notificar…
findings-submit-quarantine-label = Enviar…
findings-send-clamav-malwarebazaar-tooltip = Enviarlo a ClamAV, MalwareBazaar o VirusTotal
findings-trust-label = Confiar…
findings-copy-hash-tooltip = Copiar hash
findings-submit-quarantined-label = Enviar…
findings-service-label = Servicio
findings-status-label = Estado
findings-quarantine-record-no-usable = Este registro de cuarentena no tiene un hash utilizable, así que { -brand-name } no lo envía a ninguna parte
findings-file-no-longer-quarantine = Este archivo ya no está en cuarentena
findings-service-not-take-detection = Este servicio no acepta esta detección
findings-file-no-usable-hash = Este archivo ya no está en cuarentena con un hash utilizable
findings-check-malwarebazaar-virustotal-tooltip = Buscar novedades en MalwareBazaar y VirusTotal
findings-checking-sent-tooltip = Comprobando
findings-send-file-title = Enviar un archivo
findings-nothing-leaves-computer-description = Nada sale de este equipo hasta que confirme lo que se envía.
findings-report-missed-malware-label = Notificar malware no detectado…
findings-send-clamav-malicious-tooltip = Enviar a ClamAV un archivo malicioso que no detecta
findings-sent-title = Enviado
findings-lazaret-sent-became-description = Lo que { -brand-name } envió y qué fue de ello.
findings-filter-sent-samples-tooltip = Filtrar muestras enviadas
findings-online-services-title = Servicios en línea
findings-each-service-receives-subtitle = Lo que recibe cada servicio, y las claves de MalwareBazaar y VirusTotal
findings-choose-file-clamav-misses = Elegir el archivo que ClamAV no detecta
findings-false-positive = Falso positivo
findings-missed-malware = Malware no detectado
findings-malware-sample = Muestra de malware
findings-clamav-false-positive = Notificar falso positivo a ClamAV
findings-clamav-missed-malware = Notificar malware no detectado a ClamAV
findings-malwarebazaar-sample-choice = Subir a MalwareBazaar
findings-virustotal-sample-choice = Subir a VirusTotal
findings-awaiting-clamav = Pendiente de revisión por ClamAV. Las revisiones tardan al menos 48 horas; una actualización de firmas posterior muestra el resultado.
findings-fixed-database-version = Corregido en la versión { $version } de la base de datos
findings-fixed-current-signature-database = Corregido en la base de datos de firmas actual
findings-detected-since-database-version = Detectado desde la versión { $version } de la base de datos
findings-detected-current-signature-database = Detectado por la base de datos de firmas actual
findings-uploaded-waiting-malwarebazaar-list = Subido; esperando a que MalwareBazaar lo publique
findings-listed-malwarebazaar = Publicado en MalwareBazaar
findings-malwarebazaar-refused-reason = MalwareBazaar lo rechazó: { $reason }
findings-queued-analysis-virustotal = En cola para análisis en VirusTotal
findings-count-total-engine-flags =
    { $total ->
        [one] { $count } de { $total } motor lo marca como malicioso
       *[other] { $count } de { $total } motores lo marcan como malicioso
    }
findings-virustotal-refused-reason = VirusTotal lo rechazó: { $reason }
findings-service-kind-hash-date = { $service } · { $kind } · { $hash } · { $date }
findings-integrity-unknown-candidate = { $detection } · Sin hash utilizable, así que { -brand-name } no puede demostrar qué enviaría
findings-all-services = Todos los servicios
findings-any-status-option = Cualquier estado
findings-waiting-label = En espera
findings-settled-label = Resuelto
findings-refused-label = Rechazado
findings-all-submissions-option = Todos los envíos
findings-listing-loading = Cargando envíos…
findings-nothing-submitted = Nada enviado
findings-no-submission-matches-filters = Ningún envío coincide con los filtros
findings-candidates-loading = Cargando la cuarentena…
findings-quarantine-empty-sent = La cuarentena está vacía
findings-key-missing-failure = { $service } necesita una clave para comprobar las subidas
findings-no-upload-changed = Ninguna subida ha cambiado
findings-count-upload-new-status =
    { $count ->
        [one] { $count } subida tiene un estado nuevo
       *[other] { $count } subidas tienen un estado nuevo
    }
findings-choose-service-heading = ¿Adónde debe ir este archivo?
findings-each-service-gets-file = Cada servicio recibe el propio archivo. El siguiente paso muestra exactamente qué se envía y quién puede verlo.
findings-cancel-submission = Cancelar
findings-open-remote-entry-tooltip = Abrir la entrada remota
findings-same-upload-running = Este archivo ya se está enviando a este servicio. Espere a que termine antes de volver a enviarlo.
findings-check-again-trusted-tooltip = Comprobar de nuevo
findings-checking-trusted-tooltip = Comprobando
findings-status-title = Estado
findings-files-trusted-detections-description = Archivos marcados como de confianza y detecciones ignoradas o restauradas mediante { -brand-name }, según los registros del sistema. El software que se ejecuta con su usuario no puede eliminar estos registros.
findings-no-trusted-detections-title = No se ha cambiado ninguna detección de confianza mediante { -brand-name }
findings-checking-trusted-detections = Comprobando las detecciones de confianza…
findings-command-copied-clipboard = Comando copiado al portapapeles
findings-other-allowlist = Otra lista de permitidos
findings-file-reason = { $file }: { $reason }
findings-trusted-files = Archivos de confianza
findings-ignored-detections = Detecciones ignoradas
findings-allowlist-not-write-status = Una lista de permitidos que { -brand-name } no escribió indica a ClamAV que guarde silencio
findings-count-file-detection-status =
    { $count ->
        [one] { $count } archivo o detección nunca se notifica
       *[other] { $count } archivos o detecciones nunca se notifican
    }
findings-own-allowlist-unreadable-status = { -brand-name } no puede leer su propia lista de permitidos, así que no puede saber sobre qué guarda silencio ClamAV
findings-folder-unreadable = { -brand-name } no puede leer la carpeta de firmas, así que no puede saber sobre qué guarda silencio ClamAV
findings-signature-folder = Carpeta de firmas
findings-trusted-detections = Detecciones de confianza
findings-trust-list-mechanism = Los archivos y nombres de detección incluidos en las listas de permitidos de ClamAV nunca se notifican, en ningún análisis. Un malware que obtenga permisos de administrador podría ocultarse así, por lo que debe comprobar que cada entrada se añadió a propósito.
findings-clamav-reports-everything-detects = ClamAV notifica todo lo que detecta
findings-remove-tooltip = Eliminar
findings-access-denied = Permiso denegado
findings-other-read-failure = Falló la lectura: { $reason }
findings-reading-task-aborted = La lectura se detuvo antes de terminar
findings-journal-unreadable-added = Se desconoce quién lo añadió. { $reason }
findings-known-user = Añadido { $time } por el usuario { $uid }
findings-unknown-user = Añadido { $time } por un usuario desconocido
findings-no-record-added-system = No hay registro en el diario del sistema de quién lo añadió
findings-count-entry-not-added =
    { $count ->
        [one] { $count } entrada, no añadida mediante { -brand-name }
       *[other] { $count } entradas, no añadidas mediante { -brand-name }
    }
findings-entries-cannot-counted-reason = No se pueden contar las entradas ({ $reason }), no añadidas mediante { -brand-name }
findings-list-unreadable = { -brand-name } no puede leer esta lista
findings-no-trusted-files = No hay archivos de confianza
findings-no-ignored-detections = No hay detecciones ignoradas
findings-lazaret-cannot-read-signature = { -brand-name } no puede leer la carpeta de firmas
findings-trusted-detections-title = Detecciones de confianza
findings-trusted-by-you-title = De su confianza
findings-trusted-by-you-description = Sus análisis no le muestran estos archivos, y ni sus análisis ni las alertas en segundo plano le muestran estas detecciones. ClamAV en sí las sigue detectando, así que el análisis en el acceso todavía puede bloquear los archivos y otros usuarios todavía los ven. Cualquier cambio en un archivo de confianza hace que vuelva a aparecer.
findings-nothing-trusted-by-you = No ha marcado nada como de confianza
findings-trust-by-you-unreadable = { -brand-name } no puede leer lo que ha marcado como de confianza: { $error }
findings-file-trusted-by-you = Un archivo que ha marcado como de confianza
findings-trust-for-all-users-tooltip = Confiar para todos los usuarios…
findings-trusted-for-all-users-title = De confianza para todos los usuarios
findings-clamav-never-reports-description = ClamAV nunca notifica lo que figura aquí, para ningún usuario y en ningún análisis de este equipo. Cada cambio requiere una contraseña de administrador y queda registrado en el diario del sistema, donde el software que se ejecuta con su usuario no puede eliminar el registro.
findings-ignored-detections-title = Detecciones ignoradas
findings-detection-names-clamav-description = Nombres de detección que ClamAV ignora en todos los archivos, para todos los usuarios. { -brand-name } ya no los añade, porque ocultan todos los archivos con esa detección; elimínelos y confíe en los archivos por su hash en su lugar.
findings-other-allowlists-title = Otras listas de permitidos
findings-allowlist-files-signature-description = Archivos de listas de permitidos en la carpeta de firmas que { -brand-name } no escribió. Compruebe que usted o su administrador los añadieron a propósito.

signatures-history-load-failed = No se pudieron cargar los registros históricos: { $error }
signatures-database-update-finished = Actualización de la base de datos completada
signatures-check-again-database-tooltip = Comprobar de nuevo
signatures-checking-database-tooltip = Comprobando
signatures-update-history-title = Historial de actualizaciones
signatures-updates-started-lazaret-description = Actualizaciones iniciadas desde { -brand-name }, las más recientes primero. Seleccione una para leer su transcripción.
signatures-historical-logs-cannot-title = No se pueden leer los registros históricos
signatures-status-title = Estado
signatures-loading-history = Cargando el historial…
signatures-no-updates-started-lazaret = Todavía no se ha iniciado ninguna actualización desde { -brand-name }
signatures-checking-signatures = Comprobando las firmas…
signatures-command-copied-clipboard-database = Comando copiado al portapapeles
signatures-update-not-added-history = No se pudo añadir la actualización al historial: { $error }
signatures-update-signatures = Actualizar ahora
signatures-cancel-signature-update = Cancelar
signatures-definitions-built-date = Definiciones generadas { $when } ({ $date })
signatures-definitions-exist-not-determined = Hay definiciones, pero no se pudo determinar su antigüedad
signatures-no-virus-definitions-found = No se encontró ninguna definición de virus
signatures-age-definitions-unknown-checking = Se desconoce la antigüedad de las definiciones, porque su comprobación se detuvo antes de terminar
signatures-last-update-failed = Falló su última actualización
signatures-no-completed-update = Todavía no hay ninguna actualización completada registrada
signatures-updater-updated = { $updater } actualizado
signatures-provider-fetched = { $provider } descargado
signatures-count-signature-size-file =
    { $count ->
        [one] { $count } firma, { $size } ({ $file })
       *[other] { $count } firmas, { $size } ({ $file })
    }
signatures-file-unreadable = Número de firmas desconocido, no se pudo leer el archivo: { $error }, { $size } ({ $file })
signatures-other-signature-sources = Otras fuentes de firmas
signatures-unknown-reason-database = Desconocido. { $reason }
signatures-fetch-newest-signatures = Descargar ahora las firmas más recientes de todos los actualizadores
signatures-manual-update = Actualización manual
signatures-update-service = Servicio de actualización
signatures-database-folder = Carpeta de la base de datos
signatures-updater-output = Salida del actualizador
signatures-current-label = Firmas al día
signatures-signature-mechanism = Las firmas describen malware conocido, y se publican nuevas constantemente. Las firmas antiguas no detectan las amenazas recientes aunque todo lo demás esté en ejecución.
signatures-updating-signatures = Actualizando las firmas…
signatures-update-cancelling = Cancelando la actualización…
signatures-starting-update = Iniciando la actualización…
signatures-daemon-running = Activo
signatures-timer-scheduled = Programado por un temporizador de systemd
signatures-service-stopped = Instalado pero detenido
signatures-service-not-found = No se detectó ningún servicio freshclam; las actualizaciones se ejecutan manualmente
signatures-systemd-unreachable-service = No se pudo consultar a systemd sobre un servicio freshclam; las actualizaciones se ejecutan manualmente
signatures-count-day-old =
    { $count ->
        [one] { $count } día de antigüedad
       *[other] { $count } días de antigüedad
    }
signatures-definitions-missing = No se encontró ninguna base de datos de virus legible
signatures-directory-unreadable = Desconocido. { $reason }
signatures-scanner-error = freshclam falló con el código de salida { $code }.
signatures-killed-signal = freshclam se detuvo por la señal { $signal }.
signatures-unrecognized-wait-status = freshclam terminó de una forma que { -brand-name } no reconoce.
signatures-wait-failed = { -brand-name } perdió el rastro de freshclam antes de que terminara.
signatures-authorization-refused = Falló la autorización. Inténtelo de nuevo e introduzca su contraseña.
signatures-systemd-not-reached = No se pudo contactar con systemd para ejecutar la actualización.
signatures-service-job-ended = El servicio de actualización no terminó; systemd informó: { $result }
signatures-update-rate-limited-mirror = El servidor espejo ha limitado la frecuencia de actualización. Espere unos minutos e inténtelo de nuevo.
signatures-update-blocked-cdn = La CDN ha bloqueado la actualización. Puede deberse a una limitación de frecuencia. Espere e inténtelo de nuevo más tarde.
signatures-mirror-unavailable = El servidor espejo de ClamAV no está disponible en este momento. Inténtelo de nuevo más tarde.
signatures-certificate-rejected = Error de certificado SSL/TLS. Puede que el servidor espejo tenga problemas de configuración.
signatures-connection-timed-out = Se agotó el tiempo de conexión. Compruebe su conexión de red.
signatures-database-locked = La base de datos está bloqueada. Puede que haya otra instancia de freshclam en ejecución.
signatures-permission-denied = Permiso denegado. Puede que necesite privilegios elevados para actualizar la base de datos.
signatures-connection-failed = Error de conexión. Compruebe su conexión de red.
signatures-name-resolution-failed = Falló la resolución DNS. Compruebe los ajustes de red.
signatures-update-failed-unknown-error = La actualización falló con un error desconocido. Consulte los registros para obtener detalles.
signatures-updated-databases = Actualizado: { $databases }
signatures-already-current-databases = Ya está actualizado: { $databases }
signatures-cooldown-unknown = Frecuencia limitada: { $database }
signatures-cooldown-until = Frecuencia limitada: { $database } hasta { $cooldown }
signatures-definitions-updated = Actualización de la base de datos completada
signatures-already-current = La base de datos ya está actualizada
signatures-service-ran-without-verdict = El servicio de actualización se ejecutó, pero su registro no indica si las definiciones cambiaron.
signatures-update-requested-daemon = El servicio aceptó la señal; realiza la actualización de forma asíncrona. Consulte sus registros para confirmar que haya terminado.
signatures-update-cancelled = Actualización de la base de datos cancelada
signatures-scanner-program-missing = Instale freshclam antes de solicitar actualizaciones de la base de datos.
signatures-provider-needs-credential-fangfrisch = { $provider } necesita { $credential }, y fangfrisch solo la lee de sus ajustes. En NixOS, services.clamav.fangfrisch.settings se copia en la Nix store, que cualquier usuario de este equipo puede leer, así que { $credential } quedaría expuesta. Deje { $provider } desactivado o ejecute fangfrisch con un archivo de ajustes guardado fuera de la Nix store, como un secreto de agenix o sops-nix.
signatures-provider-needs-credential-website = { $provider } necesita { $credential } de su sitio web. fangfrisch la lee de su archivo de ajustes, normalmente { $path }. Añada allí estas líneas, sustituya el marcador de posición por su { $credential } y haga que solo root pueda leer el archivo, para que otros usuarios no puedan verlo.
signatures-not-use = Sin usar
signatures-add-lines-fangfrisch-settings = Añada estas líneas al archivo de ajustes de fangfrisch, normalmente { $path }.
signatures-provider-needs-free-abuse = { $provider } necesita una Auth-Key gratuita de abuse.ch. Añada estas líneas al archivo de ajustes de fangfrisch, normalmente { $path }, sustituya el marcador de posición por su clave y haga que solo root pueda leer el archivo, para que otros usuarios no puedan verlo.
signatures-use-label = En uso
signatures-unknown-reason-source = Desconocido. { $reason }
signatures-check-again-signature-tooltip = Comprobar de nuevo
signatures-checking-signature-tooltip = Comprobando
signatures-clamav-malware-signatures-description = De dónde vienen las firmas de malware de ClamAV y qué otras fuentes podría añadir.
signatures-sources-title = Fuentes de firmas
signatures-abuse-ch-key-title = Clave de abuse.ch
signatures-abuse-ch-auth-title = Auth-Key de abuse.ch
signatures-command-copied-clipboard-signature = Comando copiado al portapapeles
signatures-not-open-documentation-title = No se pudo abrir la documentación: { $error }
signatures-abuse-ch-key-purpose = Las firmas de URLhaus mediante fangfrisch, las consultas a MalwareBazaar y las subidas a MalwareBazaar usan esta clave. Se guarda y se edita en Preferencias, en Servicios en línea.
signatures-checking-api-key = Comprobando la clave de API…
signatures-check-passed = Correcto
signatures-needs-review = Advertencia
signatures-check-failed = Fallo
signatures-status-unknown = Desconocido
signatures-check-skipped = Omitido
signatures-check-progress = Comprobando
signatures-copy-tooltip = Copiar
signatures-learn-more-title = Más información
signatures-open-browser-tooltip = Abrir en el navegador
signatures-checking-title = Comprobando…

preferences-launcher-not-found = { -brand-name } no pudo encontrar su propio lanzador: { $error }
preferences-action-unwritable = No se pudo añadir la acción del gestor de archivos: { $error }
preferences-action-unremovable = No se pudo eliminar la acción del gestor de archivos: { $error }
preferences-dialog-title = Preferencias
preferences-general-title = General
preferences-window-behavior-title = Comportamiento de la ventana
preferences-detection-alerts-work-description = Las alertas de detección solo funcionan mientras { -brand-name } está en ejecución.
preferences-closing-window-title = Al cerrar la ventana
preferences-start-background-title = Iniciar en segundo plano
preferences-launch-background-without-subtitle = Iniciar en segundo plano sin abrir ninguna ventana.
preferences-desktop-notifications-title = Notificaciones de escritorio
preferences-notify-detections-finished-subtitle = Avisar de las detecciones, y de los análisis y actualizaciones terminados mientras la ventana no tiene el foco.
preferences-file-manager-integration-title = Integración con el gestor de archivos
preferences-add-context-menu-description = Añadir acciones al menú contextual para los gestores de archivos compatibles.
preferences-online-services-title = Servicios en línea
preferences-services-tell-more-description = Servicios que le informan más sobre un archivo detectado o en cuarentena. Las consultas solo se ejecutan cuando pulsa Consultar en una detección o Comprobar en una entrada de cuarentena, y solo envían un hash o un nombre de firma. Un archivo solo sale de este equipo cuando lo envía y confirma lo que se envía.
preferences-lookups-send-file-description = Las consultas solo envían el hash SHA-256 de un archivo e indican si pertenece a un archivo conocido e inofensivo. No se necesita clave.
preferences-no-lookups-report-description = Sin consultas. Un informe que confirme envía el archivo, el nombre de la detección en caso de falso positivo, y su nombre y dirección de correo electrónico. Talos procesa las muestras de forma automatizada, nunca las hace públicas y no las comparte con otras empresas. Una revisión tarda al menos 48 horas. No se necesita clave.
preferences-lookups-send-virustotal-file = Las consultas solo envían a VirusTotal el hash SHA-256 de un archivo. Una subida que confirme envía el propio archivo en cuarentena, que VirusTotal comparte con sus clientes de pago. Las subidas se omiten cuando VirusTotal ya tiene un informe, y se rechazan por encima de 32 MB.
preferences-malwarebazaar-lookups-send-detection = Las consultas a MalwareBazaar envían el nombre de firma de una detección o el hash SHA-256 de un archivo. Una subida que confirme envía el archivo en cuarentena con su firma de ClamAV como etiqueta, y la muestra se hace pública. La misma clave de abuse.ch también permite descargar firmas de URLhaus mediante fangfrisch; en NixOS, { -brand-name } reutiliza el archivo de clave de fangfrisch cuando su usuario puede leerlo. Las condiciones de uso razonable de abuse.ch indican que los usuarios con fines comerciales pueden necesitar su API comercial.
preferences-virustotal-service-hint = Nivel gratuito: 4 solicitudes por minuto, 500 al día.
preferences-abuse-ch-service-hint = Gratuita en el portal de autenticación de abuse.ch.
preferences-not-configured = Sin configurar
preferences-checking-label = Comprobando…
preferences-set-system-configuration-title = Definida por la configuración del sistema
preferences-key-keyring-title = Clave en su llavero
preferences-delete-key-label = Eliminar clave
preferences-paste-api-key-title = Pegar clave de API
preferences-get-free-api-title = Obtener una clave de API gratuita

widgets-clean-label = Limpio
widgets-scan-infected = Infectado
widgets-error-status = Error
widgets-cancelled-label = Cancelado
widgets-skipped-label = Omitido
widgets-success-label = Correcto
widgets-up-date = Actualizado
widgets-requested-label = Solicitado
widgets-ran-label = Ejecutado
widgets-counted-threats =
    { $count ->
        [one] Se encontró { $count } amenaza
       *[other] Se encontraron { $count } amenazas
    }
widgets-uncounted-threats = El programa de análisis notificó una amenaza sin indicar el archivo.
widgets-no-threats-found-activity = No se encontraron amenazas
widgets-scan-failed = El análisis no terminó correctamente
widgets-scan-cancelled-activity = Escaneo cancelado por el usuario
widgets-scheduled-scan-skipped-battery = Escaneo programado omitido (con alimentación por batería)
widgets-virus-definitions-updated = Definiciones de virus actualizadas
widgets-virus-definitions-up-date = Las definiciones de virus están al día
widgets-update-requested-service = Actualización solicitada al servicio
widgets-update-service-no-result = El servicio de actualización se ejecutó sin informar de ningún resultado
widgets-update-failed = La actualización de la base de datos falló
widgets-update-cancelled = Actualización de la base de datos cancelada
widgets-scan-log = REGISTRO DE ESCANEOS
widgets-update-log = REGISTRO DE ACTUALIZACIONES
widgets-timestamp-label = Marca de tiempo
widgets-status-label = Estado
widgets-duration-label = Duración
widgets-summary-label = Resumen
widgets-path-label = Ruta
widgets-full-output = Salida completa
widgets-scan-history-cannot-title = No se puede leer el historial de análisis
widgets-retry-label = Reintentar
widgets-copy-activity-tooltip = Copiar
widgets-log-details-copied-toast = Detalles del registro copiados al portapapeles
widgets-not-save-file-error = No se pudo guardar el archivo: { $error }
widgets-export-title = Exportar
widgets-database-newer-release = Esta base de datos la creó una versión más reciente de { -brand-name }: tiene la versión de estructura { $found }, y esta versión conoce hasta la { $known }. Actualice { -brand-name } para abrirla.
widgets-migration-step-failed = La actualización de la estructura de la base de datos falló en el paso { $step }: { $error }
widgets-version-check-failed = No se pudo comprobar la versión de estructura de la base de datos: { $error }
widgets-copy-configuration-line-title = Copiar línea de configuración
widgets-nothing-applied-choose-description = No se aplica nada. Elija un valor y copie la línea; después, añádala usted mismo a su configuración.
widgets-unit-title = Unidad
widgets-line-copy-title = Línea que copiar
widgets-copy-line-label = Copiar línea
widgets-unlock-required = Desbloquee para cambiar este ajuste. Mientras está bloqueado, puede copiar su línea para su configuración.
widgets-administrator-not-copy = Su administrador no permite cambiar este ajuste desde { -brand-name }. Puede copiar su línea para su configuración.
widgets-plain-bytes = bytes
widgets-kibibytes-unit = KiB
widgets-mebibytes-unit = MiB
widgets-unknown-option = { -brand-name } no conoce esta opción, así que solo la muestra.
widgets-not-yes-no = Requiere revisión: este valor debería ser yes o no.
widgets-not-whole-number = Requiere revisión: este valor debería ser un número entero.
widgets-count-out-range = Requiere revisión: este valor debería estar entre { $minimum } y { $maximum }.
widgets-not-size = Requiere revisión: este valor debería ser un tamaño en bytes, o un número seguido de K o M.
widgets-path-not-absolute = Requiere revisión: este valor debería ser una ruta completa que empiece por /.
widgets-empty-text = Requiere revisión: este valor está vacío.
widgets-apply-tooltip = Aplicar
widgets-unit-tooltip = Unidad
widgets-setting-runs-commands-changes = Este ajuste ejecuta comandos o cambia el usuario con el que se ejecuta ClamAV, así que { -brand-name } nunca ofrece cambiarlo
widgets-check-value-configuration-file = Compruebe este valor en el archivo de configuración.
widgets-copy-configuration-line-tooltip = Copiar línea de configuración…
widgets-applying-restarting-clamav-tooltip = Aplicando y reiniciando ClamAV
widgets-detect-potentially-unwanted-programs = Detectar programas potencialmente no deseados
widgets-heuristic-detections = Detecciones heurísticas
widgets-flag-office-files-macros = Marcar archivos de Office con macros
widgets-flag-encrypted-archives-documents = Marcar archivos comprimidos y documentos cifrados
widgets-look-inside-archives = Examinar el interior de archivos comprimidos
widgets-flag-files-too-large = Marcar archivos demasiado grandes para comprobarlos por completo
widgets-adware-hacking-tools-remote = Adware, herramientas de hacking y herramientas de acceso remoto. Habrá más falsos positivos.
widgets-suspicious-files-no-signature = Archivos sospechosos que ninguna firma identifica, como ejecutables dañados y trucos de phishing.
widgets-macros-common-way-deliver = Las macros son una vía habitual de distribución de malware, pero muchos documentos legítimos las usan.
widgets-clamav-cannot-look-inside = ClamAV no puede examinar su interior, así que podrían ocultar malware.
widgets-checks-files-inside-zip = Comprueba los archivos dentro de zip, rar y otros archivos comprimidos.
widgets-otherwise-files-above-size = De lo contrario, los archivos que superan los límites de tamaño y recursión pasan sin comprobar y sin aviso.
widgets-locked-until-unlocked = Desbloquee para cambiar este ajuste.
widgets-administrator-not-allow-detection = Su administrador no permite cambiar este ajuste desde { -brand-name }.
widgets-clamd-settings = Ajustes de clamd
widgets-access-scanning-settings = Ajustes del análisis en el acceso
widgets-update-settings = Ajustes de actualización
widgets-change-snippet-copied = Copiado. Añádalo a su configuración para aplicarlo.
widgets-stale-confirmation-dropped = No se realizó el cambio: los ajustes se volvieron a bloquear o se inició otro cambio antes.
widgets-settings-applied = Aplicado, pero todavía no está en la configuración del sistema.
widgets-previous-restored = ClamAV no volvió a funcionar correctamente con ese valor, así que se restauraron los ajustes anteriores.
widgets-change-not-made-error = No se realizó el cambio: { $error }
widgets-authorization-refused = No se cambió nada sin su contraseña
widgets-admin-service-missing = Esta instalación no tiene el servicio de administración de ClamAV, así que los ajustes de ClamAV no se pueden cambiar desde { -brand-name }
widgets-live-tunable-config = { $path } combina la configuración del sistema con los cambios realizados aquí. Mientras están bloqueados, los ajustes son de solo lectura: use Copiar línea de configuración para copiar uno para su configuración. Tras desbloquear, los ajustes permitidos cambian directamente.
widgets-change-setting = ¿Cambiar este ajuste?
widgets-cancel-engine = Cancelar
widgets-reduce-protection = Reducir protección
widgets-apply-label = Aplicar
widgets-key-value-until-time = { $key } { $value }, hasta { $time }
widgets-live-tuning-not-installed = El ajuste en vivo no está instalado. En NixOS, active services.lazaret.liveTuning; en Debian y Ubuntu, instale el paquete de { -brand-name }, que incluye el servicio de administración de ClamAV.
widgets-admin-service-unreachable = Ahora mismo no se pueden cambiar los ajustes aquí: no se pudo contactar con el servicio de administración de ClamAV de { -brand-name }.
widgets-loading-config = Leyendo la configuración…
widgets-config-missing = No se encontró el archivo de configuración
widgets-lock-label = Bloquear
widgets-unlock-label = Desbloquear
widgets-change-settings-title = Cambiar ajustes
widgets-unlock-change-settings-description = Desbloquee para cambiar los ajustes que el sistema permite en tiempo de ejecución. Desbloquear no concede nada por sí solo: cada cambio muestra lo que ocurrirá y pide autorización de administrador. Se vuelve a bloquear al cabo de dos minutos.
widgets-changes-system-configuration-title = Cambios que la configuración del sistema desconoce
widgets-copy-engine-tooltip = Copiar
widgets-reduced-protection-ends-title = Protección reducida que termina por sí sola
widgets-reset-system-configuration-title = Restablecer a la configuración del sistema
widgets-detection-title = Detección
widgets-clamav-reports-every-description = Lo que ClamAV notifica en cada análisis de este equipo. { -brand-name } pasa las mismas opciones a clamscan, para que ambos programas de análisis coincidan.
widgets-no-settings-section-title = No hay ajustes en esta sección
widgets-journal-read-failed = No se pudo leer el diario del sistema: { $error }
widgets-journal-reading-incomplete = La lectura del diario del sistema se detuvo antes de terminar
widgets-folder-unreadable = No se pudo leer la carpeta de firmas: { $error }
widgets-reading-signature-folder-stopped = La lectura de la carpeta de firmas se detuvo antes de terminar
widgets-entry-timestamp-invalid = una entrada del diario tiene una marca de tiempo fuera del intervalo admitido
widgets-reader-thread-failed = no se pudo iniciar el lector del diario: { $error }
widgets-journal-reading-stopped = Se detuvo la lectura del diario del sistema: { $error }
widgets-file-hash-mismatch = La suma hash del archivo no coincide; el archivo puede estar dañado
widgets-restore-refused-integrity-unknown = Este registro de cuarentena no tiene un hash utilizable, así que el archivo no se puede verificar y no se restaura. Todavía puede eliminarlo.
widgets-upload-refused-integrity-unknown = Este registro de cuarentena no tiene un hash utilizable, así que { -brand-name } no puede demostrar que el archivo es el que puso en cuarentena y no lo envía a ninguna parte.
widgets-payload-too-large-send = El archivo supera { $size }, el máximo que envía { -brand-name }.
widgets-blocked-file-other-filesystem = El análisis en el acceso bloquea este archivo y está en otro sistema de archivos, así que no se puede mover a cuarentena. En su lugar, puede eliminarlo.
widgets-entry-not-found = No se ha encontrado la entrada de cuarentena: { $id }
widgets-recording-failed-file-restored = No se ha podido registrar la entrada de cuarentena en la base de datos. El archivo se ha restaurado en su ubicación original.
widgets-recording-failed-rollback = No se ha podido registrar la entrada de cuarentena en la base de datos. La reversión tampoco se ha podido realizar: { $error }. Es posible que haya un archivo huérfano en: { $path }
widgets-entry-record-not-removed = El archivo se movió, pero no se pudo eliminar su registro de cuarentena. Puede que siga apareciendo en la lista.
widgets-system-call-failed = Falló una llamada al sistema: { $error }
widgets-file-operation-failed = Falló una operación de archivo: { $error }
widgets-database-operation-failed = Falló la operación en la base de datos de cuarentena: { $error }
widgets-blocking-step-interrupted = Un paso de cuarentena en segundo plano se detuvo inesperadamente: { $error }
widgets-path-contains-nul = La ruta contiene un byte NUL
widgets-path-has-no-parent = La ruta no tiene carpeta superior
widgets-path-has-no-file = La ruta no tiene nombre de archivo
widgets-database-path-not-utf8 = La ruta de la base de datos de cuarentena no es UTF-8 válido
widgets-database-not-regular-file = La base de datos de cuarentena no es un archivo normal
widgets-source-blocked-access = El análisis en el acceso bloquea la lectura del archivo
widgets-moved-payload-still-blocked = El archivo bloqueado sigue sin poder leerse tras moverlo a cuarentena
widgets-source-not-regular-file = El archivo no es un archivo normal
widgets-payload-not-regular-file = El archivo en cuarentena no es un archivo normal
widgets-not-enough-free-space = No hay suficiente espacio libre para poner el archivo en cuarentena
widgets-file-replaced-during-operation = El archivo se sustituyó mientras se movía, así que no se eliminó
widgets-path-outside-quarantine = La ruta no está dentro de la carpeta de cuarentena
widgets-invalid-restore-path = La ruta de restauración no es válida
widgets-protected-restore-destination = No se permite restaurar en una carpeta protegida del sistema
widgets-stored-path-unreadable = No se pudo leer una ruta guardada en la base de datos de cuarentena
widgets-target-missing = El destino del análisis no existe o contiene un byte NUL no válido
widgets-target-not-scannable = El destino del análisis no es un archivo normal ni un directorio
widgets-target-directory-unreadable = No se puede enumerar el objetivo del análisis { $path }: { $error }
widgets-file-list-unwritable = No se puede escribir la lista privada de archivos de clamd: { $error }
widgets-enumeration-interrupted = El listado del objetivo del análisis se detuvo inesperadamente.
widgets-no-files-scanned = No se pudo analizar ningún archivo
widgets-unscanned-files =
    { $count ->
        [one] { $count } archivo no se analizó
       *[other] { $count } archivos no se analizaron
    }
widgets-clamd-connection-lost = Se perdió la conexión con clamd durante el análisis.
widgets-scanner-stopped-reporting = El programa de análisis no informó de ningún resultado para estos archivos.
widgets-count-not-accessed-title =
    { $count ->
        [one] No se pudo acceder a { $count } archivo
       *[other] No se pudo acceder a { $count } archivos
    }
widgets-files-unreadable =
    { $count ->
        [one] No se pudo leer { $count } archivo
       *[other] No se pudo leer { $count } archivos
    }
widgets-nonfatal-warnings =
    { $count ->
        [one] { $count } advertencia no crítica durante el análisis; es posible que algunos archivos se hayan analizado solo parcialmente
       *[other] { $count } advertencias no críticas durante el análisis; es posible que algunos archivos se hayan analizado solo parcialmente
    }
widgets-count-threat-found-outcome =
    { $count ->
        [one] { $count } amenaza encontrada
       *[other] { $count } amenazas encontradas
    }
widgets-count-file-scanned-outcome =
    { $count ->
        [one] { $count } archivo analizado
       *[other] { $count } archivos analizados
    }
widgets-clamav-not-open-files = ClamAV no pudo abrir estos archivos.
widgets-unattributed-threats-only = El programa de análisis notificó una amenaza sin indicar el archivo.
widgets-preparing-scan = Preparando…
widgets-finding-files =
    { $count ->
        [one] Buscando archivos… { $count } encontrado
       *[other] Buscando archivos… { $count } encontrados
    }
widgets-trust-file = Confiar en este archivo…
widgets-ignore-detection-for-me = Ignorar esta detección para mí…
widgets-report-false-positive = Notificar falso positivo…
widgets-skip-path-lazaret-scans = Omitir esta ruta en los análisis de { -brand-name }
widgets-copy-path = Copiar ruta
widgets-critical-severity = Crítico
widgets-high-severity = Alto
widgets-medium-severity = Medio
widgets-low-severity = Bajo
widgets-moved-quarantine-path-check = Movido a cuarentena: { $path }. Compruebe su reputación en la página Cuarentena.
widgets-quarantine-detected-label = Cuarentena
widgets-more-actions-tooltip = Más acciones
widgets-count-more-file-title =
    { $count ->
        [one] { $count } archivo más, incluido en el informe exportado
       *[other] { $count } archivos más, incluidos en el informe exportado
    }
widgets-scan-details-title = Detalles del análisis
widgets-quarantine-all =
    { $count ->
        [one] Poner { $count } amenaza en cuarentena
       *[other] Poner { $count } amenazas en cuarentena
    }
widgets-move-quarantine = ¿Mover a cuarentena?
widgets-removes-each-original-file = Esto elimina cada archivo original de su ubicación actual.
widgets-cancel-scan = Cancelar
widgets-quarantine-label = Cuarentena
widgets-detected-threats-title = Amenazas detectadas
widgets-problems-title = Problemas
widgets-move-path-quarantine = ¿Mover '{ $path }' a cuarentena?
widgets-move-count-detected-file =
    { $count ->
        [one] ¿Mover { $count } archivo detectado a cuarentena?
       *[other] ¿Mover { $count } archivos detectados a cuarentena?
    }
widgets-path-copied-clipboard = Ruta copiada al portapapeles
widgets-file-skipped-future-scans = Este archivo se omitirá en los análisis futuros
widgets-exclusion-already-exists = Esta exclusión ya existe
widgets-results-copied-clipboard = Resultados copiados al portapapeles
widgets-results-exported = Resultados exportados
widgets-done-total-file-scanned =
    { $total ->
        [one] { $done } de { $total } archivo analizado
       *[other] { $done } de { $total } archivos analizados
    }
widgets-count-file-scanned-results =
    { $count ->
        [one] { $count } archivo analizado
       *[other] { $count } archivos analizados
    }
widgets-count-entry-skipped-exclusions =
    { $count ->
        [one] { $count } entrada omitida por las exclusiones
       *[other] { $count } entradas omitidas por las exclusiones
    }
widgets-threats-detected-status = Amenazas detectadas
widgets-no-threats-detected-status = No se encontraron amenazas
widgets-finished-problems-status = El análisis no terminó correctamente
widgets-scan-cancelled-status = Escaneo cancelado por el usuario
widgets-count-threat-found-results =
    { $count ->
        [one] { $count } amenaza encontrada
       *[other] { $count } amenazas encontradas
    }
widgets-files-scanned = Archivos analizados
widgets-files-planned = Archivos previstos
widgets-directories-label = Directorios
widgets-threats-found = Amenazas encontradas
widgets-file-no-longer-quarantine = El archivo ya no está. Si está en cuarentena, márquelo como de confianza desde la página Cuarentena.
widgets-permission-denied = No se puede leer el archivo, posiblemente porque el análisis en el acceso lo bloquea. Muévalo a cuarentena y márquelo como de confianza desde la página Cuarentena.
widgets-file-not-read-scan = No se pudo leer el archivo: { $error }
widgets-threats-detected-finished = Amenazas detectadas
widgets-no-threats-detected-finished = No se encontraron amenazas
widgets-finished-problems-notify = El análisis no terminó correctamente
widgets-scanner-error = El programa de análisis se detuvo con el código de error { $code }.
widgets-killed-signal = El programa de análisis se detuvo por la señal { $signal }.
widgets-unrecognized-wait-status = El programa de análisis terminó de una forma que { -brand-name } no reconoce (estado de espera { $status }).
widgets-wait-failed = { -brand-name } perdió el rastro del proceso de análisis: { $error }.
widgets-scanner-program-missing = El programa de análisis de ClamAV no está instalado o no está en la ruta de búsqueda.
widgets-scanner-program-replaceable = { -brand-name } se negó a ejecutar { $program }, porque su cuenta puede sustituir { $replaceable }. Instale ClamAV para todo el sistema, para que un programa colocado por malware no pueda falsear resultados limpios.
widgets-scanner-spawn-failed = No se pudo iniciar el programa de análisis: { $error }
widgets-scanner-work-directory-unavailable = No se puede crear el directorio de trabajo privado del programa de análisis: { $error }
widgets-scanner-output-uncaptured = No se pudo capturar la salida del programa de análisis.
widgets-detection-settings-unreadable = El análisis no se inició, porque no se pudieron leer los ajustes de detección de clamd: { $error }
widgets-program-lookup-failed = El análisis no se inició, porque no se pudieron localizar los programas de análisis: { $error }
widgets-scan-input-unavailable = No se puede abrir de forma segura la entrada del análisis { $path }: { $error }
widgets-detections-warnings-errors-tooltip = Solo detecciones, advertencias y errores
widgets-copy-service-tooltip = Copiar
widgets-export-text-tooltip = Exportar texto
widgets-search-log-placeholder = Buscar en el registro
widgets-reading-log = Leyendo el registro…
widgets-no-log-lines-match = Ninguna línea del registro coincide
widgets-log-copied-clipboard = Registro copiado al portapapeles
widgets-text-export-finished = Exportado.
widgets-threat-detected = Detección
widgets-emergency-label = Emergencia
widgets-alert-label = Alerta
widgets-critical-level = Crítico
widgets-error-level = Error
widgets-warning-label = Advertencia
widgets-notice-label = Aviso
widgets-info-label = Información
widgets-debug-label = Depuración
widgets-log-reading = Comprobando…
widgets-system-journal = Diario del sistema
widgets-neither-log-file-nor = No se encontró ningún archivo de registro ni líneas del diario.
widgets-no-log-file-found = No se encontró ningún archivo de registro. { $reason }
widgets-program-use = { $program } en uso
widgets-ignored-program = { $program } ignorado
widgets-refused-program = { $program } rechazado
widgets-scanner-programs-trusted = Los programas de análisis son de confianza
widgets-freshclam-trusted = freshclam es de confianza
widgets-malware-running-clamdscan = Un malware que se ejecute con su usuario podría poner un clamdscan falso antes en su ruta de búsqueda que informe de que todos los archivos están limpios. { -brand-name } solo ejecuta programas de ClamAV que su cuenta no puede sustituir, como los instalados para todo el sistema.
widgets-malware-running-freshclam = Un malware que se ejecute con su usuario podría poner un freshclam falso antes en su ruta de búsqueda que finja descargar firmas nuevas. { -brand-name } solo ejecuta programas de ClamAV que su cuenta no puede sustituir, como los instalados para todo el sistema.
widgets-check-interrupted = Desconocido, porque la comprobación de los programas de ClamAV se detuvo antes de terminar
widgets-clamav-program-replaced-account = Su cuenta puede sustituir un programa de ClamAV, así que { -brand-name } se niega a ejecutarlo
widgets-clamav-program-account-shadows = Un programa de ClamAV en su cuenta oculta la copia del sistema. { -brand-name } lo ignora, pero compruebe de dónde procede
widgets-count-settings-reduced-since =
    { $count ->
        [one] { $count } ajuste comprueba menos de lo configurado desde { $time }
       *[other] { $count } ajustes comprueban menos de lo configurado desde { $time }
    }
widgets-count-settings-reduced-unknown-time =
    { $count ->
        [one] { $count } ajuste comprueba menos de lo configurado, desde un momento que no se pudo leer
       *[other] { $count } ajustes comprueban menos de lo configurado, desde un momento que no se pudo leer
    }
widgets-changed-label = Cambiado
widgets-change-time-unknown-override = Desconocido
widgets-reduction-unknown-unread = Se desconoce si se redujeron ajustes, porque no se pudieron leer los cambios en tiempo de ejecución: { $error }
widgets-reduction-unknown-file = Se desconoce si se redujeron ajustes, porque el archivo de cambios en tiempo de ejecución no tiene el formato esperado. ClamAV se ejecuta con la configuración del sistema hasta que se corrija.
widgets-reduction-unknown-reading = Se desconoce si se redujeron ajustes, porque la lectura de los cambios en tiempo de ejecución se detuvo antes de terminar
widgets-settings-changed-runtime = Ajustes cambiados en tiempo de ejecución
widgets-administrator-can-change = Alguien con la contraseña de administrador puede cambiar los ajustes de ClamAV en tiempo de ejecución. { -brand-name } no pudo leer esos cambios, así que no puede saber si comprueban menos que su configuración.
widgets-protection-reduced-purpose = Protección reducida a propósito
widgets-administrator-changed-runtime = Alguien con la contraseña de administrador cambió los ajustes de ClamAV en tiempo de ejecución de forma que comprueban menos que su configuración. Los cambios del sistema de la página a la que pertenece el ajuste muestran quién y cuándo.
widgets-base-configuration-changed-now = { $base } en su configuración, { $changed } ahora
widgets-administrator-changed-clamd = Alguien con la contraseña de administrador cambió los ajustes de ClamAV en tiempo de ejecución. No se pudo leer su clamd.conf, así que { -brand-name } no puede saber si estos cambios comprueban menos que su configuración. Los cambios del sistema de la página a la que pertenece el ajuste muestran quién y cuándo.
widgets-settings-changed-runtime-clamd = Se cambiaron ajustes en tiempo de ejecución, pero no se pudo leer clamd.conf para compararlos
widgets-changed-now = { $changed } ahora
widgets-daemon-awaited = Requiere el demonio, consulte Motor de análisis
widgets-unknown-run = Desconocido. { $reason }
widgets-copy-timeline-tooltip = Copiar
widgets-moment-details = { $moment } · { $details }
widgets-why-matters-tooltip = Por qué es importante
widgets-what-is-sent-label = Qué se envía:
widgets-clamav-audience = Cisco Talos, el equipo detrás de ClamAV, lo revisa. Las muestras se procesan de forma automatizada, solo Talos puede acceder a ellas y nunca se hacen públicas ni se comparten con otras empresas. Una revisión tarda al menos 48 horas, y su resultado aparece en una versión posterior de la base de datos de firmas.
widgets-report-false-positive-clamav = ¿Notificar un falso positivo a ClamAV?
widgets-file-content = El contenido del archivo
widgets-detection-name = El nombre de la detección { $detection }
widgets-report-missed-malware-clamav = ¿Notificar malware no detectado a ClamAV?
widgets-name-email-address-talos = Su nombre y dirección de correo electrónico, para que Talos pueda contactar con usted
widgets-lazaret-sends-file-clamav = { -brand-name } envía este archivo a ClamAV con clamsubmit. Envíe solo archivos que no contengan nada privado.
widgets-send-report-button = Enviar informe
widgets-file-content-named-sha = El contenido del archivo, identificado por su hash SHA-256, no por su nombre
widgets-upload-malwarebazaar-heading = ¿Subir a MalwareBazaar?
widgets-lazaret-uploads-file-malwarebazaar = { -brand-name } sube este archivo a MalwareBazaar, gestionado por abuse.ch.
widgets-tag-clamav-detection = La etiqueta { $tag }, de la detección de ClamAV
widgets-abuse-ch-key-sample = Su clave de abuse.ch; la muestra se publica sin el nombre de su cuenta
widgets-malwarebazaar-public-anyone-download = MalwareBazaar es público: cualquiera puede descargar la muestra una vez subida, y una subida no se puede retirar. Suba solo malware, nunca un documento que pueda contener datos personales.
widgets-upload-publicly-button = Subir públicamente
widgets-upload-virustotal-heading = ¿Subir a VirusTotal?
widgets-lazaret-uploads-file-virustotal = { -brand-name } sube este archivo a VirusTotal para su análisis.
widgets-virustotal-key-links-upload = Su clave de VirusTotal, que vincula la subida a su cuenta
widgets-virustotal-shares-uploaded-files = VirusTotal comparte los archivos subidos con sus clientes de pago, que pueden descargarlos, y cualquiera puede consultar el resultado mediante el hash del archivo. Nunca suba un documento que pueda contener datos personales.
widgets-upload-button = Subir
widgets-cancel-submission = Cancelar
widgets-name-title = Su nombre
widgets-email-address-title = Su dirección de correo electrónico
widgets-file-unreadable = No se pudo leer el archivo: { $error }
widgets-file-not-regular = Solo se pueden enviar archivos normales
widgets-file-too-large = El archivo supera { $size }, el máximo que envía { -brand-name }.
widgets-reading-interrupted = La lectura del archivo se detuvo inesperadamente
widgets-history-failure = No se pudo leer el historial de envíos: { $error }
widgets-earlier-submission = Este archivo ya se envió el { $time }. No se ha vuelto a enviar nada.
widgets-report-recorded = Informe enviado. La revisión de ClamAV tarda al menos 48 horas; Muestras enviadas muestra cuándo una actualización de firmas cambia el veredicto.
widgets-report-not-sent-error = No se envió el informe: { $error }
widgets-report-sent-without-record = Informe enviado, pero no se pudo añadir a Muestras enviadas: { $error }
widgets-nothing-uploaded-error = No se subió nada: { $error }
widgets-malwarebazaar-refused-sample-reason = MalwareBazaar rechazó la muestra: { $reason }
widgets-malwarebazaar-sample-sent-samples = MalwareBazaar tiene la muestra. Muestras enviadas enlaza a su entrada pública.
widgets-add-abuse-ch-key = añada primero una clave de abuse.ch en Preferencias
widgets-not-detected-clamav = MalwareBazaar solo acepta archivos que haya detectado una firma de ClamAV
widgets-unwanted-program-only = MalwareBazaar no acepta programas potencialmente no deseados, solo malware
widgets-sent-without-record = Subido, pero no se pudo añadir a Muestras enviadas: { $error }
widgets-virustotal-refused-file-reason = VirusTotal rechazó el archivo: { $reason }
widgets-virustotal-analysing-file-refresh = VirusTotal está analizando el archivo. Actualizar en la página Muestras enviadas muestra su veredicto.
widgets-report-already-available =
    { $total ->
        [one] VirusTotal ya conoce este archivo: { $count } de { $total } motor lo detecta. No se subió nada.
       *[other] VirusTotal ya conoce este archivo: { $count } de { $total } motores lo detectan. No se subió nada.
    }
widgets-add-virustotal-key-preferences = añada primero una clave de VirusTotal en Preferencias
widgets-known-user = { $time } · { $outcome } · solicitado por el usuario { $uid }
widgets-unknown-user = { $time } · { $outcome } · solicitado por un usuario desconocido
widgets-change-lowered-protection-tooltip = Este cambio redujo la protección
widgets-system-changes-failed = No se puede leer el diario del sistema: { $error }
widgets-system-changes-title = Cambios del sistema
widgets-refresh-tooltip = Actualizar
widgets-system-changes-cannot-title = No se pueden leer los cambios del sistema
widgets-loading-system-changes = Cargando los cambios del sistema…
widgets-trust-file-heading = ¿Confiar en este archivo?
widgets-clamav-never-report-body =
    ClamAV no volverá a notificar ningún archivo con exactamente este contenido, para ningún usuario, esté donde esté en este equipo y lo encuentre el análisis que lo encuentre, y el análisis en el acceso dejará de bloquearlo. Se necesita su contraseña de administrador, y la solicitud de contraseña muestra el mismo hash SHA-256 que aparece abajo.
    
    { $path }
widgets-stop-showing-file-body =
    { -brand-name } dejará de mostrarle cualquier archivo con exactamente este contenido en los análisis que ejecute, esté donde esté. ClamAV en sí lo sigue detectando, así que el análisis en el acceso todavía puede bloquearlo y avisarle, y otros usuarios todavía lo ven. Confiar para todos los usuarios hace que ClamAV deje de detectarlo para todos y requiere su contraseña de administrador.
    
    { $path }
widgets-trust-for-me-button = Confiar solo para mí
widgets-user-trust-unreadable = No se pudieron leer los archivos y detecciones que ha marcado como de confianza, así que este análisis muestra todas las detecciones: { $error }
widgets-trust-for-all-users-button = Confiar para todos los usuarios…
widgets-sha-256-hash-size =
    SHA-256 { $hash }
    { $size }
widgets-ignore-detection-heading = ¿Ignorar esta detección?
widgets-stop-showing-detection-body = { -brand-name } dejará de mostrarle todos los archivos con esta detección, no solo este. ClamAV en sí y otros usuarios los siguen viendo. Confiar en el archivo por su hash es más seguro cuando solo un archivo es un falso positivo.
widgets-ignore-button = Ignorar
widgets-stop-trusting-file-heading = ¿Dejar de confiar en este archivo?
widgets-clamav-report-file-body = ClamAV volverá a notificar este archivo a todos los usuarios cuando coincida con una firma.
widgets-show-file-again-body = { -brand-name } volverá a mostrarle este archivo cuando ClamAV lo detecte.
widgets-stop-trusting-button = Dejar de confiar
widgets-report-detection-again-heading = ¿Volver a notificar esta detección?
widgets-clamav-report-files-body = ClamAV volverá a notificar a todos los usuarios los archivos con esta detección.
widgets-show-detection-again-body = { -brand-name } volverá a mostrarle los archivos con esta detección.
widgets-report-again-button = Volver a notificar
widgets-cancel-trust = Cancelar

host-action-applied = Aplicado
host-action-rolled-back = Revertido
host-authorization-refused-described = Rechazado sin contraseña
host-action-failed = Fallido
host-outcome-not-recorded = Resultado no registrado
host-unrecognized-outcome = Resultado no reconocido { $outcome }
host-blocking-turned-off = Los archivos detectados ya no se bloquearán al abrirse. Solo se notificarán.
host-check-turned-off = ClamAV dejará de buscar este tipo de archivo o amenaza en este equipo.
host-watched-folder-removed = Los archivos de la carpeta eliminada ya no se comprobarán al abrirse.
host-skipped-location-added = ClamAV omitirá por completo la nueva entrada, así que el malware que haya ahí pasará desapercibido.
host-scan-limit-lowered = Los archivos o archivos comprimidos que superen el nuevo límite se omitirán sin comprobarse.
host-current-values-unknown = No se pudo leer el valor actual, así que este cambio puede reducir la protección.
host-detection-trusted = ClamAV dejará de notificar lo que marque aquí como de confianza.
host-key-now-current-new =
    { $key }
    Ahora: { $current }
    Nuevo: { $new }
host-system-records-change-reduced = El sistema registra que este cambio redujo la protección, y la barra lateral sigue señalando la página del ajuste cambiado hasta que lo deshaga.
host-trust-applied = ClamAV cargó el cambio
host-trust-recorded-for-you = { -brand-name } ya no le muestra esto. ClamAV en sí, el bloqueo en el acceso y otros usuarios lo siguen viendo.
host-applied-reload-pending = Guardado. clamd no respondió, así que aplicará el cambio la próxima vez que recargue sus firmas
host-authorization-refused-outcome = No se cambió nada sin su contraseña
host-admin-service-missing = Esta instalación no tiene el servicio de administración de ClamAV, así que la lista de confianza de ClamAV no se puede cambiar desde { -brand-name }
host-trust-failed = No se cambió nada: { $error }
host-nixos-configuration = { $path } lo genera su configuración de NixOS, así que { -brand-name } no puede cambiarlo. Use Copiar línea de configuración en un ajuste para copiar su línea para su configuración y, después, recompile.
host-system-package = { $path } pertenece al sistema, así que { -brand-name } no puede cambiarlo. Use Copiar línea de configuración en un ajuste para copiar la línea que debe poner en ese archivo.
host-contact-incomplete = Introduzca su nombre y una dirección de correo electrónico válida para enviar un informe
host-submitter-missing = Para notificar se necesita clamsubmit, que viene con ClamAV pero no está instalado aquí
host-submitter-replaceable = { -brand-name } se negó a ejecutar clamsubmit, porque su cuenta puede sustituirlo
host-report-not-sent-error = No se envió el informe: { $error }
host-upload-timed-out = No se envió el informe: la subida tardó demasiado
host-purchase-receipt = número de recibo de compra
host-customer-identifier = ID de cliente
host-unrecognized-source = Otra fuente
host-unnamed-detection = una detección sin nombre
host-file-progress =
    { $total ->
        [one] { $done } de { $total } archivo
       *[other] { $done } de { $total } archivos
    }
host-count-second-left =
    { $count ->
        [one] Queda { $count } segundo aproximadamente
       *[other] Quedan { $count } segundos aproximadamente
    }
host-count-minute-left =
    { $count ->
        [one] Queda { $count } minuto aproximadamente
       *[other] Quedan { $count } minutos aproximadamente
    }
host-count-hour-left =
    { $count ->
        [one] Queda { $count } hora aproximadamente
       *[other] Quedan { $count } horas aproximadamente
    }
host-seconds-s = { $seconds } s
host-minutes-min-seconds-s = { $minutes } min { $seconds } s
host-hours-h-minutes-min = { $hours } h { $minutes } min
host-blocked-access = Bloqueado por el análisis en el acceso
host-exact-byte-size =
    { $count ->
        [one] { $size } ({ $count } byte)
       *[other] { $size } ({ $count } bytes)
    }
host-needs-key = Requiere una clave
host-unnamed-source = una fuente sin nombre
host-hash-unknown = No figura en la base de datos de archivos conocidos
host-listed-malicious-source-name = { $source } lo cataloga como malicioso (como { $name })
host-listed-malicious-source = { $source } lo cataloga como malicioso
host-listed-known-file-source = { $source } lo cataloga como archivo conocido
host-known-file-name-listed = Archivo conocido { $name }, catalogado por { $source }
host-unnamed-family = familia sin nombre
host-first-seen-unknown = una fecha desconocida
host-no-known-samples = No es una muestra de malware conocida
host-known-malware-sample = Muestra de malware conocida
host-samples-found = Malware conocido: { $family }, visto por primera vez { $date }
host-file-unknown-virustotal = Desconocido para VirusTotal
host-count-total-engine-verdict =
    { $total ->
        [one] { $count } de { $total } motor lo detecta
       *[other] { $count } de { $total } motores lo detectan
    }
host-api-key-rejected = VirusTotal rechazó la clave de API.
host-request-limit-reached = Se alcanzó el límite de solicitudes de VirusTotal. Inténtelo de nuevo más tarde.
host-virustotal-answered-status = VirusTotal respondió con el estado { $status }
host-virustotal-not-reached-error = No se pudo contactar con VirusTotal: { $error }
host-virustotal-sent-answer-lazaret = VirusTotal envió una respuesta que { -brand-name } no puede leer
host-request-failed-circl = No se pudo contactar con CIRCL hashlookup: { $error }
host-answer-unreadable-circl = CIRCL hashlookup envió una respuesta que no se pudo leer
host-unexpected-status-circl = CIRCL hashlookup respondió con el estado { $status }
host-request-failed-bazaar = No se pudo contactar con MalwareBazaar: { $error }
host-answer-unreadable-bazaar = MalwareBazaar envió una respuesta que no se pudo leer
host-key-refused = MalwareBazaar rechazó la clave de API
host-query-refused = MalwareBazaar respondió: { $status }
host-key-configured = Clave de API configurada
host-key-missing = No hay clave de API, así que este servicio está desactivado
host-key-unreadable = No se puede leer la clave de API: { $reason }
host-key-format-invalid = No parece una clave de API válida
host-stored-key-unreadable = No se puede leer la clave de API guardada
host-keyring-unavailable = El llavero no está disponible
host-key-file-unreadable = { -brand-name } no puede leer este archivo: { $error }. Si es un secreto de agenix o sops-nix, establezca su usuario como propietario.
host-key-file-permission-denied = { -brand-name } no puede leer este archivo, así que las consultas con este servicio están desactivadas. Conceda a su usuario acceso de lectura, por ejemplo con el ajuste de propietario o grupo del secreto de agenix o sops-nix.
host-key-file-content-invalid = El archivo no contiene una clave válida para este servicio

headless-count-threat-found =
    { $count ->
        [one] { $count } amenaza encontrada
       *[other] { $count } amenazas encontradas
    }
headless-open-lazaret-review = Abra { -brand-name } para revisarlas.
headless-count-moved-quarantine =
    { $count ->
        [one] { $count } movida a cuarentena.
       *[other] { $count } movidas a cuarentena.
    }
headless-scan-infected = Amenazas detectadas
headless-scheduled-scan-failed = Falló el análisis programado
headless-open-lazaret-read-log = Abra { -brand-name } para leer el registro.
