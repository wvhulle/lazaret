-brand-name = Lazaret
startup-settings-schema-missing = A { -brand-name } beállításséma nincs telepítve. Futtassa a make install parancsot, vagy állítsa be a GSETTINGS_SCHEMA_DIR változót.
startup-unknown-page-name = A { -brand-name } alkalmazásban nincs { $page } nevű oldal, ezért az első figyelmet igénylő terület nyílik meg. Oldalak: { $pages }

window-signature-sources = Aláírásforrások
window-database-updates = Adatbázis-frissítések
window-scan-engine = Vizsgálómotor
window-access-scanning = Hozzáféréskori vizsgálat
window-scheduled-scans-page = Ütemezett vizsgálatok
window-manual-scan = Kézi vizsgálat
window-removable-drives = Cserélhető meghajtók
window-detections-label = Észlelések
window-quarantine-label = Karanténba helyezés
window-trusted-detections = Megbízható észlelések
window-sent-samples = Elküldött minták
window-quarantine-database-not-locked = A karanténadatbázist nem sikerült zárolni
window-quarantine-database-not-opened = A karanténadatbázist nem sikerült megnyitni
window-file-operation-failed-error = Egy fájlművelet sikertelen: { $error }
window-lock-task-interrupted = Egy háttérben futó karanténlépés váratlanul leállt: { $error }
window-translator-credits = Kálmán „KAMI” Szalai
window-export-history-menu-item = Előzmények exportálása…
window-clear-history-menu-item = Előzmények törlése…
window-preferences-label = Beállítások
window-about-menu-item = A { -brand-name } névjegye
window-quit-menu-item = Kilépés
window-export-history-heading = Exportálja az előzményeket?
window-export-history-body = Minden rögzített vizsgálatot és aláírásfrissítést ment a részleteikkel együtt.
window-cancel-history-button = Mégse
window-json-label = JSON
window-csv-label = CSV
window-clear-history-heading = Törli az előzményeket?
window-clear-history-body = Ez véglegesen törli az összes rögzített vizsgálatot és aláírásfrissítést. Ez nem vonható vissza.
window-clear-label = Összes törlése
window-scanning-drive-title = Vizsgálat: { $drive }
window-open-removable-drives-button = Cserélhető meghajtók megnyitása
window-not-export-history-toast = Az előzmények nem exportálhatók: { $error }
window-history-export-finished = Előzmények exportálva
window-history-cleared = Előzmények törölve
window-not-clear-history-toast = Az előzmények nem törölhetők: { $error }
window-other-sources-unknown = Az egyéb források ismeretlenek
window-source-out-date = Egy forrás elavult
window-official-signatures = Csak hivatalos aláírások
window-official-count-other-source =
    { $count ->
        [one] Hivatalos és { $count } egyéb forrás
       *[other] Hivatalos és { $count } egyéb forrás
    }
window-database-missing = Nincsenek vírusdefiníciók
window-age-definitions-unknown = A definíciók kora ismeretlen
window-last-update-failed = A legutóbbi frissítés sikertelen
window-definitions-dated-future = Jövőbeli dátumú definíciók
window-updated-label = Frissítve: { $relative-time }
window-definitions-label = Definíciók dátuma: { $relative-time }
window-freshclam-refused = A freshclam elutasítva
window-unknown-whether-freshclam-trusted = Nem tudható, megbízható-e a freshclam
window-freshclam-copy-account-ignored = A fiókjában lévő freshclam-példány figyelmen kívül marad
window-clamd-uses-older-signatures = A clamd régebbi aláírásokat használ
window-clamd-answering = A clamd válaszol
window-unit-active-not-answering = A clamd nem válaszol
window-unit-state-unknown-daemon = A clamd állapota ismeretlen
window-unit-stopped-daemon = A clamd nem fut
window-never-set-up-daemon = A clamd nincs beállítva
window-engine-missing = A ClamAV nincs telepítve
window-scanner-refused = A clamscan elutasítva
window-version-unverified = A ClamAV verziója ismeretlen
window-scanner-program-refused = Egy vizsgálóprogram elutasítva
window-unknown-whether-scanners-trusted = Nem tudható, megbízhatók-e a vizsgálóprogramok
window-scanner-copy-account-ignored = A fiókjában lévő vizsgálóprogram-példány figyelmen kívül marad
window-runtime-changes-unknown = A futásidejű módosítások ismeretlenek
window-settings-reduced-runtime = Futásidőben csökkentett beállítások
window-waiting-daemon = Várakozás a clamd szolgáltatásra
window-unit-state-unknown-access = Ismeretlen állapot
window-never-set-up-access = Nincs beállítva
window-unit-stopped-access = A clamonacc nem fut
window-config-unreadable = A clamd.conf nem olvasható
window-nothing-watched = Semmi sincs megfigyelve
window-checks-files-not-block = Ellenőriz, de nem blokkol
window-checks-files-kernel-cannot = Ellenőriz, a kernel nem tud blokkolni
window-checks-files-blocking-unknown = Ellenőriz, a blokkolás ismeretlen
window-blocking-paths = Blokkolva: { $paths }
window-folder-watch-limit-unknown = A mappafigyelési korlát ismeretlen
window-mount-watch-conflicts = Ütköző csatolásfigyelés
window-clamd-user-not-excluded = A clamd nincs kizárva
window-whole-root-included = Közvetlenül a / figyelése
window-watch-limit-low = Alacsony mappafigyelési korlát
window-timer-not-active = Az időzítő nem aktív
window-last-scan-failed = A legutóbbi vizsgálat sikertelen
window-next-scan = Következő vizsgálat: { $relative-time }
window-no-next-scan-planned = Nincs tervezett következő vizsgálat
window-service-missing = A vizsgálati szolgáltatás hiányzik
window-service-unreadable = A vizsgálati szolgáltatás ismeretlen
window-lazaret-schedule = Ütemezte: { -brand-name }
window-no-schedule = Nincs ütemezett vizsgálat
window-schedule-changed-outside-lazaret = Az ütemezés a { -brand-name } alkalmazáson kívül módosult
window-ready-scan = Kész a vizsgálatra
window-uses-slower-clamscan = A lassabb clamscan használatban
window-exclusion-skip-malware = Egy kizárás miatt kártevő maradhat ki
window-device-scan-running = Vizsgálat: { $drive }
window-device-scan-ended = Fenyegetés található itt: { $drive }
window-journal-unreadable = A napló nem olvasható
window-following-failed = A napló követése leállt
window-notifications-off = Értesítések kikapcsolva
window-alerts-stop-lazaret-quits = A riasztások leállnak, amikor a { -brand-name } kilép
window-other-programs-attach-lazaret = Más programok csatlakozhatnak a { -brand-name } alkalmazáshoz
window-unknown-whether-programs-attach = Nem tudható, csatlakozhatnak-e programok
window-nothing-decide = Nincs teendő
window-count-needs-decision =
    { $count ->
        [one] { $count } döntésre vár
       *[other] { $count } döntésre vár
    }
window-loading-label = Betöltés…
window-quarantine-empty = Üres
window-count-item =
    { $count ->
        [one] { $count } elem
       *[other] { $count } elem
    }
window-quarantine-cannot-read = A karantén nem olvasható
window-nothing-trusted = Nincs megbízhatónak jelölt elem
window-count-item-trusted =
    { $count ->
        [one] { $count } megbízható elem
       *[other] { $count } megbízható elem
    }
window-signature-folder-unreadable = Az aláírásmappa nem olvasható
window-another-allowlist-silences-clamav = Egy másik engedélyezési lista elnémítja a ClamAV-t
window-lazaret-allowlist-unreadable = A { -brand-name } engedélyezési listája nem olvasható
window-sent-samples-cannot-read = Az elküldött minták nem olvashatók
window-nothing-sent = Nincs elküldött elem
window-count-samples-refused =
    { $count ->
        [one] { $count } elemet elutasított a szolgáltatása
       *[other] { $count } elemet elutasítottak a szolgáltatásaik
    }
window-count-samples-awaiting-review =
    { $count ->
        [one] { $count } felülvizsgálatra vár
       *[other] { $count } felülvizsgálatra vár
    }
window-count-samples-sent =
    { $count ->
        [one] { $count } elküldve
       *[other] { $count } elküldve
    }
window-signatures-label = Aláírások
window-scans-label = Vizsgálatok
window-findings-label = Találatok
window-detections-come = Honnan származnak az észlelések
window-files-get-checked = Hogyan ellenőrződnek a fájlok
window-found-label = Mi található
window-needs-review = Felülvizsgálatot igényel
window-not-working = Nem működik
window-waiting-another-part = Várakozás egy másik összetevőre
window-summary-pending = Ellenőrzés…
window-menu-tooltip = Menü
window-show-lazaret = A { -brand-name } megjelenítése
window-show-detections = Észlelések megjelenítése
window-quick-scan = Gyors vizsgálat
window-update-definitions = Definíciók frissítése
window-quit-tray-item = Kilépés
window-quit-completely = Teljes kilépés
window-hide-tray = Kicsinyítés a tálcára
window-ask-on-next-close = Kérdezzen rá a következő bezáráskor
window-close-lazaret = Bezárja a { -brand-name }-t?
window-detection-alerts-stop-lazaret = Az észlelési riasztások leállnak, amíg a { -brand-name } nem fut. Futva hagyja a háttérben, vagy teljesen kilép? Ezt később a Beállításokban módosíthatja.
window-cancel-close-button = Mégse
window-quit-close-button = Kilépés
window-keep-running = Futva hagyás

scans-node-js-dependencies = Node.js-függőségek
scans-git-repository-data = Git-tároló adatai
scans-python-virtual-environment = Python virtuális környezet
scans-build-output-directory = Build kimeneti könyvtár
scans-distribution-output-directory = Terjesztési kimeneti könyvtár
scans-python-bytecode-cache = Python bájtkódgyorsítótár
scans-remove-exclusion-tooltip = Eltávolítás
scans-preset-exclusions-title = Előre beállított kizárások
scans-common-directories-exclude-description = A vizsgálatból kizárandó gyakori könyvtárak. A módosítások azonnal mentődnek.
scans-custom-exclusions-title = Egyéni kizárások
scans-enter-absolute-path-description = Adjon meg egy abszolút elérési utat vagy egy mintát, például *.tmp. A módosítások azonnal mentődnek.
scans-path-exclude-tmp-title = /path/to/exclude vagy *.tmp
scans-prefer-clamd-when-available = Automatikus
scans-always-clamd = clamd (démon)
scans-always-clamscan = clamscan (önálló)
scans-options-title = Beállítások
scans-backend-title = Vizsgálati háttérprogram
scans-automatic-uses-daemon-subtitle = Az automatikus mód a démont használja, ha az válaszol, egyébként a clamscant.
scans-show-live-scan-title = Élő vizsgálati előrehaladás megjelenítése
scans-display-files-scanned-subtitle = A vizsgált fájlok valós idejű megjelenítése.
scans-not-added-history-error = A vizsgálatot nem sikerült felvenni az előzmények közé: { $error }
scans-no-profile-manual = Nincs profil (kézi)
scans-removable-drive-scanned-title = Egy cserélhető meghajtó vizsgálata folyamatban van. Ezt a vizsgálatot annak befejezése után indítsa el.
scans-open-removable-drives-button = Cserélhető meghajtók megnyitása
scans-detecting-backend = Ellenőrzés…
scans-backend-undetermined = Háttérprogram: ismeretlen, a vizsgálóprogramokat nem sikerült megkeresni: { $error }
scans-backend-clamd-daemon = Háttérprogram: clamd (démon)
scans-backend-clamscan-standalone = Háttérprogram: clamscan (önálló)
scans-scan-title = Vizsgálat
scans-preparing-label = Előkészítés…
scans-cancelling-label = Megszakítás…
scans-scanning-name = Vizsgálat: { $name }
scans-scanning-count-location =
    { $count ->
        [one] { $count } hely vizsgálata
       *[other] { $count } hely vizsgálata
    }
scans-cancelled-user = A felhasználó megszakította a vizsgálatot
scans-threats-found-scan = Talált fenyegetések
scans-not-finish-cleanly = A vizsgálat nem fejeződött be hibátlanul
scans-no-threats-found = Nem található fenyegetés
scans-not-create-eicar-test = Nem sikerült létrehozni az EICAR tesztfájlt: { $error }
scans-add-files-tooltip = Fájlok hozzáadása
scans-add-folder-manual-tooltip = Mappa hozzáadása
scans-more-tooltip = Továbbiak
scans-run-eicar-test-label = EICAR teszt futtatása
scans-copy-results-label = Eredmények másolása
scans-export-results-label = Eredmények exportálása…
scans-start-scan-label = Vizsgálat indítása
scans-cancel-manual-label = Mégse
scans-lazaret-refuses-clamav-title = A { -brand-name } elutasít egy ClamAV-programot, amelyet a fiókja lecserélhet
scans-details-button = Részletek
scans-profile-title = Vizsgálati profil
scans-profile-short-title = Profil
scans-manage-profiles-tooltip = Profilok kezelése
scans-targets-manual-title = Vizsgálati célok
scans-drop-files-folders-label = Húzzon ide fájlokat vagy mappákat a vizsgálathoz
scans-add-files-label = Fájlok hozzáadása…
scans-add-folder-label = Mappa hozzáadása…
scans-found-far-scan-title = Eddigi találatok
scans-new-scan-button = Új vizsgálat
scans-recent-scans-title = Legutóbbi vizsgálatok
scans-started-lazaret-newest-description = A { -brand-name } alkalmazásból indított vizsgálatok, a legújabb elöl. Válasszon ki egyet az átirata megtekintéséhez.
scans-choose-files-scan = Vizsgálandó fájlok kiválasztása
scans-choose-folder-scan = Vizsgálandó mappa kiválasztása
scans-quick-scan = Gyors vizsgálat
scans-full-scan = Teljes vizsgálat
scans-home-folder = Saját mappa
scans-fast-scan-downloads-folder = A Letöltések mappa gyors vizsgálata a fenyegetések gyors felismeréséhez
scans-comprehensive-system-wide-scan = Az összes elérhető könyvtár átfogó, rendszerszintű vizsgálata
scans-user-home-directory-personal = A felhasználó saját könyvtárának és személyes fájljainak vizsgálata
scans-profile-file-unavailable = A profilfájl nem érhető el: { $error }
scans-profile-data-invalid = A profiladatok érvénytelenek: { $error }
scans-profile-file-not-replaced = A profilfájlt nem sikerült lecserélni: { $error }
scans-profile-task-interrupted = A profilművelet megszakadt: { $error }
scans-profile-name-missing = A profilnév megadása kötelező
scans-profile-name-too-long = A név legfeljebb 50 karakterből állhat
scans-profile-name-taken = Már létezik ilyen nevű profil
scans-profile-targets-missing = Adjon meg legalább egy célt
scans-edit-profile-title = Profil szerkesztése
scans-save-label = Mentés
scans-name-title = Név
scans-description-title = Leírás
scans-targets-profile-title = Célok
scans-add-folder-profile-tooltip = Mappa hozzáadása
scans-add-path-title = Elérési út hozzáadása
scans-exclusions-title = Kizárások
scans-added-exclusions-preferences-description = A profil futása alatt a Beállításokban megadott kizárásokhoz adódik.
scans-add-pattern-title = Minta hozzáadása
scans-cancel-profile = Mégse
scans-defaults-restored = Alapértelmezett profilok visszaállítva
scans-profile-deleted = Profil törölve
scans-profile-imported = Profil importálva
scans-manage-profiles-title = Profilok kezelése
scans-restore-defaults-tooltip = Alapértékek visszaállítása
scans-new-profile-tooltip = Új profil
scans-import-tooltip = Importálás
scans-profiles-title = Vizsgálati profilok
scans-select-profile-edit-description = Válasszon egy profilt a szerkesztéshez vagy a vizsgálathoz
scans-restore-default-profiles = Alapértelmezett profilok visszaállítása
scans-restore-built-default-profiles = Visszaállítja a beépített alapértelmezett profilokat? Az alapértelmezett profilok minden módosítása vissza lesz állítva; az egyéni profilok megmaradnak.
scans-delete-profile = Profil törlése
scans-delete-profile-name = Törli a(z) „{ $name }” profilt?
scans-delete-label = Törlés
scans-profile-exported = Profil exportálva
scans-remove-profile-tooltip = Eltávolítás
scans-count-target =
    { $count ->
        [one] { $count } cél
       *[other] { $count } cél
    }
scans-default-profile = Alapértelmezett profil
scans-run-tooltip = Futtatás
scans-edit-tooltip = Szerkesztés
scans-delete-tooltip = Törlés
scans-export-tooltip = Exportálás
scans-remove-target-tooltip = Eltávolítás
scans-check-again-access-tooltip = Ellenőrzés újra
scans-checking-access-tooltip = Ellenőrzés
scans-status-access-title = Állapot
scans-access-settings-changed-description = A { -brand-name } alkalmazással módosított hozzáféréskori beállítások, a rendszer nyilvántartása szerint. Az Ön nevében futó szoftverek nem tudják eltávolítani ezeket a bejegyzéseket.
scans-no-access-settings-title = Nem módosultak hozzáféréskori beállítások a { -brand-name } alkalmazással
scans-clamonacc-log-title = clamonacc-napló
scans-clamonacc-not-written-title = A clamonacc még nem írt a naplóba
scans-checking-access-scanning = A hozzáféréskori vizsgálat ellenőrzése…
scans-command-copied-clipboard-access = A parancs a vágólapra másolva
scans-no-readable-config = Nem található olvasható clamd.conf
scans-reading-task-aborted-clamd = A clamd.conf olvasása befejezés előtt leállt
scans-watched-mount = { $path } (teljes csatolási pont)
scans-nothing-watched = Nincs beállítva
scans-default-limit = A ClamAV alapértéke
scans-configuration-label = Konfiguráció
scans-watched-folders = Figyelt mappák
scans-largest-file-scanned = Legnagyobb vizsgált fájl
scans-no-kernel-config-found = Sem a /proc/config.gz, sem a kernel /boot könyvtárban lévő konfigurációs fájlja nem létezik.
scans-kernel-config-unreadable = A kernelkonfiguráció nem olvasható.
scans-compressed-config-corrupt = A /proc/config.gz nem tömöríthető ki.
scans-kernel-release-unreadable = A futó kernel kiadása nem olvasható.
scans-reading-task-aborted-config = A kernelkonfiguráció olvasása a befejezés előtt leállt.
scans-blocks-detected-files = Az észlelt fájlok blokkolva vannak
scans-prevention-off = A fájlok ellenőrzése megtörténik, de az észlelt fájlok nincsenek blokkolva
scans-prevention-conflicts-mounts = A fájlok ellenőrzése megtörténik, de az észlelt fájlok nincsenek blokkolva, mert az OnAccessPrevention nem működik együtt az OnAccessMountPath beállítással
scans-kernel-cannot-block = A fájlok ellenőrzése megtörténik, de ez a kernel nem tudja blokkolni az észlelt fájlokat, mert CONFIG_FANOTIFY_ACCESS_PERMISSIONS nélkül készült
scans-kernel-support-unknown = A fájlok ellenőrzése megtörténik, de nem tudható, hogy a kernel képes-e blokkolni az észlelt fájlokat. { $reason }
scans-config-unreadable-monitor = Fut, de a clamd.conf nem olvasható, így nem tudható, mi kerül ellenőrzésre, és blokkolódnak-e az észlelt fájlok
scans-running-no-folders-status = Fut, de egyetlen mappa vagy csatolási pont sincs figyelve
scans-running-detected-status = Fut, de egyetlen mappa vagy csatolási pont sincs figyelve, és az észlelt fájlok nem lennének blokkolva
scans-unit-state-unknown-access = Nem tudható, fut-e a hozzáféréskori vizsgálat, mert a systemd nem tudta jelenteni
scans-never-set-up-access = A hozzáféréskori vizsgálat nincs beállítva
scans-unit-stopped-access = A(z) { $unit } nem fut
scans-access-blocking-title = Hozzáféréskori blokkolás
scans-clamonacc-checks-files-watched = A clamonacc a figyelt mappákban lévő fájlokat megnyitáskor ellenőrzi. Bekapcsolt megelőzés esetén a kernel megtagadja a hozzáférést az észlelt fájlhoz, ezért az ilyen fájl nem nyitható meg, pedig még ott van.
scans-mount-watching = Csatolásfigyelés
scans-onaccessmountpath-watches-whole-mount = Az OnAccessMountPath egy teljes csatolási pontot figyel a fanotify segítségével. A ClamAV dokumentációja szerint nem kompatibilis az OnAccessPrevention, OnAccessIncludePath, OnAccessExcludePath és OnAccessExtraScanning beállításokkal, így a figyelt csatolási ponton lévő észlelt fájlok sosem blokkolódnak. A blokkoláshoz mappákat figyeljen az OnAccessIncludePath beállítással.
scans-onaccessmountpath-cannot-combined-options = Az OnAccessMountPath nem kombinálható ezekkel: { $options }
scans-loop-protection = Vizsgálati hurok elleni védelem
scans-clamd-opens-every-file = A clamd minden fájlt megnyit, amelynek vizsgálatát kérik tőle. Ha az OnAccessExcludeUname vagy az OnAccessExcludeUID nem nevezi meg azt a felhasználót, akinek nevében a clamd fut, a clamonacc észleli ezeket az olvasásokat, és újra ugyanazoknak a fájloknak a vizsgálatát kéri a clamd szolgáltatástól.
scans-clamd-not-excluded-clamonacc = A clamd nincs kizárva, ezért a clamonacc a végtelenségig vizsgálhatja a saját olvasásait
scans-watching-whole-file-system = A teljes fájlrendszer figyelése
scans-clamonacc-refuses-onaccessincludepath-include = A clamonacc elutasítja az OnAccessIncludePath / beállítást. Azokat a mappákat vegye fel, amelyekbe fájlok érkeznek, például a saját könyvtárakat, vagy figyeljen egy csatolási pontot az OnAccessMountPath beállítással.
scans-onaccessincludepath-rejected-clamonacc = A clamonacc elutasítja az OnAccessIncludePath / beállítást
scans-folder-watch-limit = Mappafigyelési korlát
scans-path-watched-fs-inotify = A(z) { $path } figyelve van, de az fs.inotify.max_user_watches értéke csak { $limit }, így a korlát feletti mappák ellenőrizetlenek maradnak
scans-unknown-whether-path-fits = Nem tudható, belefér-e a(z) { $path } a mappafigyelési korlátba, mert az fs.inotify.max_user_watches nem olvasható
scans-clamonacc-watches-every-folder = A clamonacc egy belefoglalt útvonal alatti minden mappát az inotify segítségével figyel, mappánként egy figyelővel. Egy teljes saját könyvtár vagy nagyobb fa több figyelőt igényelhet, mint amennyit a régi, 8192-es alapértelmezett korlát enged. Növelje az fs.inotify.max_user_watches értékét a sysctl segítségével.
scans-device-auto-scan-title = Eszközök automatikus vizsgálata
scans-automatically-scan-removable-description = A cserélhető és külső meghajtók automatikus vizsgálata csatlakoztatáskor.
scans-enable-device-auto-title = Eszközök automatikus vizsgálatának engedélyezése
scans-max-device-size-title = Eszköz maximális mérete (GB)
scans-no-size-limit-subtitle = A 0 azt jelenti, hogy nincs méretkorlát.
scans-skip-battery-device-title = Kihagyás akkumulátoros használatkor
scans-drive-scan-not-added = A meghajtóvizsgálatot nem sikerült felvenni az előzmények közé: { $error }
scans-device-auto-scan = Eszközök automatikus vizsgálata
scans-device-connected-during-another = Egy eszközt egy másik vizsgálat közben csatlakoztattak, ezért nem lett megvizsgálva.
scans-removable-drives-title = Cserélhető meghajtók
scans-cancel-removable-label = Mégse
scans-found-far-removable-title = Eddigi találatok
scans-drives-session-title = Meghajtók ebben a munkamenetben
scans-drives-connected-since-description = A { -brand-name } indítása óta csatlakoztatott meghajtók, a legújabb elöl, és hogy mi történt velük. Ez a lista csak a { -brand-name } bezárásáig marad meg.
scans-no-drive-connected-title = Még nem csatlakoztattak meghajtót
scans-close-results-button = Eredmények bezárása
scans-drive-scans-title = Meghajtóvizsgálatok
scans-drives-lazaret-scanned-description = A { -brand-name } által csatlakoztatáskor megvizsgált meghajtók, a legújabb elöl. Válasszon ki egyet az átirata megtekintéséhez.
scans-auto-scan-off = Ki
scans-every-drive = Minden meghajtót megvizsgál
scans-drives-up-size = Legfeljebb { $size } méretű meghajtókat vizsgál
scans-skipped-larger-than-limit = Nem lett megvizsgálva: a(z) { $size } nagyobb a(z) { $limit } korlátnál
scans-skipped-battery = Nem lett megvizsgálva: a számítógép akkumulátorról működött
scans-skipped-auto-scan-off = Nem lett megvizsgálva: az eszközök automatikus vizsgálata ki van kapcsolva
scans-skipped-while-scanning = Nem lett megvizsgálva: egy másik vizsgálat futott
scans-size-unreadable = Nem lett megvizsgálva: a meghajtó mérete nem olvasható
scans-drive-settling = A meghajtó ellenőrzése…
scans-drive-scanning = Vizsgálat folyamatban…
scans-threats-detected = Megvizsgálva, fenyegetés található
scans-no-threats-detected = Megvizsgálva, nem található fenyegetés
scans-finished-problems = Megvizsgálva, de a vizsgálat nem fejeződött be hibátlanul
scans-scan-cancelled = Vizsgálat megszakítva
scans-device-scan-running = Vizsgálat: { $drive }
scans-drive-scan = Meghajtóvizsgálat
scans-check-again-scan-tooltip = Ellenőrzés újra
scans-checking-scan-tooltip = Ellenőrzés
scans-status-scan-title = Állapot
scans-engine-detection-settings-description = A { -brand-name } alkalmazással módosított motor- és észlelési beállítások, a rendszer nyilvántartása szerint. Az Ön nevében futó szoftverek nem tudják eltávolítani ezeket a bejegyzéseket.
scans-no-engine-detection-title = Nem módosultak motor- vagy észlelési beállítások a { -brand-name } alkalmazással
scans-clamd-log-title = clamd-napló
scans-no-clamd-log-title = Nem található clamd-napló
scans-checking-scan-engine = A vizsgálómotor ellenőrzése…
scans-command-copied-clipboard-scan = A parancs a vágólapra másolva
scans-program-start-failed = A clamscan nem indítható el: { $error }
scans-probe-timed-out = A clamscan nem válaszolt időben
scans-scanner-refused = { $path } elutasítva, mert a fiókja lecserélheti
scans-engine-missing = A ClamAV nem található ezen a számítógépen
scans-clamav-installed = ClamAV telepítve
scans-clamav-engine-recognizes-malware = A ClamAV az a motor, amely felismeri a kártevőket. A { -brand-name } az ezen a számítógépen telepített példányt használja, és sosem csomagol sajátot.
scans-clamd-still-uses-signatures = A clamd még a(z) { $loaded } aláírásokat használja, de a lemezen a(z) { $disk } van
scans-clamd-answering = A clamd válaszol
scans-unit-active-not-answering = A(z) { $unit } fut, de a clamd nem válaszol. Lehet, hogy még tölti az aláírásokat.
scans-unit-state-unknown-daemon = A clamd nem válaszol, és a systemd nem tudta jelenteni az állapotát
scans-unit-stopped-daemon = A(z) { $unit } nem fut
scans-never-set-up-daemon = A clamd nincs beállítva, ezért a vizsgálatok a lassabb clamscant használják
scans-same-version-disk = Ugyanaz a verzió, mint a lemezen
scans-loaded-disk = { $loaded }, a lemezen viszont { $disk }
scans-unknown-loaded-stored-versions = Ismeretlen, a betöltött és a tárolt verziót nem sikerült összehasonlítani
scans-loaded-signatures = Betöltött aláírások
scans-daemon-running = A démon fut
scans-clamd-keeps-signatures-loaded = A clamd a memóriában tartja a betöltött aláírásokat. A hozzáféréskori blokkolás és az ütemezett vizsgálatok is neki küldik a fájljaikat.
scans-socket-protected = Csak a rendszer cserélheti le ezt: { $path }
scans-socket-replaceable = A fiókja lecserélheti ezt: { $replaceable }, így egy az Ön nevében futó program hamis clamd szolgáltatást helyezhet ide: { $path }, amely minden fájlt tisztának jelent
scans-socket-missing = Nincs clamd-foglalat itt: { $path }, tehát a clamd nem fut, vagy máshol figyel
scans-socket-not-checkable = A Flatpak homokozó nem látja, ki a clamd-foglalat tulajdonosa
scans-clamd-socket-protected = A clamd-foglalat védett
scans-socket-mechanism = A vizsgálatok a foglalatán keresztül érik el a clamd szolgáltatást. Hagyja, hogy a clamd olyan könyvtárban hozza létre, amelyet csak a root vagy a clamav felhasználó írhat, például a /run/clamav könyvtárban, így egyetlen az Ön nevében futó program sem tehet hamis clamd szolgáltatást a helyére.
scans-socket-inspecting = Annak ellenőrzése, ki cserélheti le a clamd-foglalatot…
scans-timer-inactive = Nincs ütemezve
scans-next-run-time = Következő futás: { $time }
scans-scheduled-time-next-run = Ütemezve. A következő futás ideje még nem ismert.
scans-systemd-unavailable = Az ütemezett vizsgálatokhoz systemd felhasználói munkamenet szükséges, amely nem érhető el.
scans-system-already-runs-scheduled = A rendszere már futtat ütemezett vizsgálatokat, ezért ezek a beállítások zárolva vannak egy második ütemezés elkerülése érdekében. Következő vizsgálat: { $time }. A rendszerkonfigurációban módosíthatja.
scans-system-scan-timer-settings = A rendszerének van vizsgálati időzítője, ezért ezek a beállítások zárolva vannak egy második ütemezés elkerülése érdekében. A rendszerkonfigurációban módosíthatja.
scans-reading-user-timer = Ellenőrzés…
scans-inspecting-system-timer = Annak ellenőrzése, hogy a rendszere futtat-e már ütemezett vizsgálatokat. Addig ezek a beállítások zárolva maradnak.
scans-system-timer-unknown = A { -brand-name } nem tudta ellenőrizni, hogy a rendszere futtat-e már ütemezett vizsgálatokat, ezért ezek a beállítások zárolva vannak egy második ütemezés elkerülése érdekében: { $error }
scans-scan-time-unreadable = A vizsgálat idejét ÓÓ:PP formában adja meg, például 02:00.
scans-unit-files-unwritable = Hiba a systemd-fájlok létrehozásakor: { $error }
scans-timer-not-applied-error = Az időzítőt nem sikerült alkalmazni: { $error }
scans-hourly-label = Óránként
scans-daily-label = Naponta
scans-weekly-label = Hetente
scans-monthly-label = Havonta
scans-monday-label = Hétfő
scans-tuesday-label = Kedd
scans-wednesday-label = Szerda
scans-thursday-label = Csütörtök
scans-friday-label = Péntek
scans-saturday-label = Szombat
scans-sunday-label = Vasárnap
scans-schedule-title = Ütemezés
scans-apply-label = Alkalmaz
scans-enable-scheduled-scans-title = Ütemezett vizsgálatok engedélyezése
scans-frequency-title = Vizsgálat gyakorisága
scans-time-24-hour-title = Vizsgálat ideje (24 órás)
scans-day-week-title = A hét napja
scans-used-weekly-scans-subtitle = Heti vizsgálatokhoz használatos.
scans-day-month-title = A hónap napja
scans-used-monthly-scans-subtitle = Havi vizsgálatokhoz használatos.
scans-targets-separated-commas-title = Vizsgálati célok (vesszővel elválasztva; üresen a saját mappa kerül vizsgálatra)
scans-skip-battery-schedule-title = Kihagyás akkumulátoros használatkor
scans-not-start-scheduled-subtitle = Ne indítsa el az ütemezett vizsgálatokat akkumulátoros üzemmódban.
scans-auto-quarantine-title = Automatikus karantén
scans-automatically-quarantine-threats-subtitle = Az ütemezett vizsgálatok által talált fenyegetések automatikus karanténba helyezése. Amíg a { -brand-name } nyitva van, a fenyegetések csak jelentésre kerülnek.
scans-review-button = Áttekintés
scans-check-again-scheduled-tooltip = Ellenőrzés újra
scans-checking-scheduled-tooltip = Ellenőrzés
scans-system-scan-title = Rendszervizsgálat
scans-folders-scanned-title = Vizsgált mappák
scans-run-now-label = Futtatás most
scans-starting-scan-tooltip = A vizsgálat indítása…
scans-status-scheduled-title = Állapot
scans-scheduled-runs-title = Ütemezett futások
scans-lazaret-schedule-ran-description = A { -brand-name } ütemezése által futtatott vizsgálatok, a legújabb elöl. Válasszon ki egyet az átirata megtekintéséhez.
scans-checking-schedule = Az ütemezés ellenőrzése…
scans-command-copied-clipboard-scheduled = A parancs a vágólapra másolva
scans-not-scheduled = Nincs ütemezve
scans-next-scan = Következő vizsgálat
scans-threats-found-scheduled = fenyegetés található
scans-nothing-found = nincs találat
scans-no-completed-scan = Még nem futott
scans-last-scan = Legutóbbi vizsgálat
scans-unknown-label = Ismeretlen
scans-system-timer = A rendszeridőzítő nem aktív
scans-last-scan-ended-error = A legutóbbi vizsgálat hibával zárult
scans-next-scan-time = Következő vizsgálat: { $time }
scans-timer-starts-scan-service = Az időzítő egy nem telepített vizsgálati szolgáltatást indít
scans-service-not-read-systemd = A vizsgálati szolgáltatás nem olvasható ki a systemd rendszerből
scans-lazaret-schedule = Ütemezte: { -brand-name }
scans-no-schedule = Nincs ütemezett vizsgálat
scans-not-scheduled-timer-active = Nincs ütemezve, mert az időzítő nem aktív
scans-folders-scanned = Vizsgált mappák
scans-scheduled-scan = Ütemezett vizsgálat
scans-scheduled-scan-catches-files = Az ütemezett vizsgálat azokat a fájlokat is megtalálja, amelyeket nem nyitottak meg, vagy amelyek csak egy aláírásfrissítés után váltak észlelhetővé.
scans-unit-names-no-folders = A vizsgálati egység nem nevez meg mappákat
scans-system-scan-service-unit = A rendszer { $unit } vizsgálati szolgáltatása ezeket a mappákat a rendszer saját beállításaival vizsgálja. A futtatásához rendszergazdai jogosultság szükséges.
scans-needs-daemon-see-scan = A démon szükséges hozzá, lásd: Vizsgálómotor
scans-bus-call-failed = A systemd nem válaszolt: { $error }
scans-unrecognized-load-state = A systemd olyan egységállapotot jelentett, amelyet a { -brand-name } nem ismer fel
scans-found-malware = A vizsgálat kártevőt talált
scans-unit-finished = A vizsgálat befejeződött
scans-not-finish-cleanly-result = A vizsgálat nem fejeződött be hibátlanul: { $result }
scans-authorization-refused = A vizsgálat indításához rendszergazdai jogosultság szükséges
scans-not-started-error = A vizsgálatot nem sikerült elindítani: { $error }

findings-open-preferences = Beállítások megnyitása
findings-journal-unreadable-alerts = A rendszernapló nem olvasható. Vegye fel a felhasználóját a systemd-journal csoportba.
findings-notifications-turned-off-preferences = Az értesítések ki vannak kapcsolva a beállításokban
findings-alerts-stop-quit-lazaret = A riasztások leállnak, amikor kilép a { -brand-name } alkalmazásból, mert nincs beállítva a háttérben futásra
findings-running-background-following-journal = Fut a háttérben, és követi a naplót
findings-alerts-label = Riasztások
findings-lazaret-follows-system-journal = A { -brand-name } követi a rendszernaplót, és értesíti, ha valamit észlel. Ezt csak futás közben tudja megtenni, ezért a háttérben kell futnia.
findings-notifications-on = Be
findings-notifications-off = Ki
findings-running-label = Fut
findings-not-running = Nem fut
findings-unit-not-found = Nincs beállítva
findings-systemd-unavailable = Ismeretlen, a systemd nem érhető el
findings-following-journal-access = Követve
findings-journal-unreadable-access = Nem olvasható
findings-following-failed = A követés leállt
findings-notifications-label = Értesítések
findings-background-service = Háttérszolgáltatás
findings-system-journal = Rendszernapló
findings-attach-unknown-setting = Nem tudható, csatlakozhatnak-e más programok, mert a kernelbeállítás nem olvasható: { $reason }
findings-attach-unknown-reports = Nem tudható, csatlakozhatnak-e más programok, mert a kernel olyan értéket jelent, amelyet a { -brand-name } nem ismer fel
findings-attach-unknown-check = Nem tudható, csatlakozhatnak-e más programok, mert az ellenőrzés befejezés előtt leállt
findings-attach-restricted = A programok nem csatlakozhatnak az Ön által futtatott más programokhoz, például a { -brand-name } alkalmazáshoz
findings-any-program-may-attach = Bármely Ön által futtatott program csatlakozhat a { -brand-name } alkalmazáshoz, és megváltoztathatja, amit megjelenít, beleértve az észleléseket és az itt hozott döntéseit
findings-yama-not-loaded = Ez a kernel nem korlátozza a csatlakozást, így bármely Ön által futtatott program csatlakozhat a { -brand-name } alkalmazáshoz, és megváltoztathatja, amit megjelenít
findings-other-programs-cannot-attach = Más programok nem csatlakozhatnak a { -brand-name } alkalmazáshoz
findings-program-attaches-lazaret-hide = Egy a { -brand-name } alkalmazáshoz csatlakozó program elrejtheti a riasztásokat, vagy megváltoztathatja a döntéseit. A kernel.yama.ptrace_scope 1-re állítása esetén egy programhoz csak a szülője csatlakozhat.
findings-file-owned-other-user = Ez a fájl egy másik felhasználóé, ezért innen nem helyezhető át és nem törölhető
findings-file-not-checkable = Ezt a fájlt nem sikerült ellenőrizni, ezért innen nem helyezhető át és nem törölhető
findings-removal-failed = A fájlt nem sikerült törölni: { $error }
findings-file-check-interrupted = A fájl ellenőrzése váratlanul leállt
findings-history-not-updated = A döntését nem sikerült menteni: { $error }
findings-journal-unreadable-list = A { -brand-name } nem tudja olvasni a rendszernaplót, ezért az új háttérbeli észlelések nem jelennek meg itt. Ez nem jelenti azt, hogy semmi sem található. Vegye fel a felhasználóját a systemd-journal csoportba.
findings-loading-detections = Észlelések betöltése…
findings-nothing-detected-background = A háttérben semmi sem lett észlelve.
findings-access-scanning-scheduled-scans = Amit a hozzáféréskori vizsgálat és az ütemezett vizsgálatok találtak.
findings-detections-title = Észlelések
findings-back-detections-tooltip = Vissza az észlelésekhez
findings-alerts-title = Riasztások
findings-detections-reach-keeps-description = Hogyan jutnak el Önhöz az észlelések, és mi akadályozza meg, hogy más programok elrejtsék őket.
findings-check-again-background-tooltip = Ellenőrzés újra
findings-checking-background-tooltip = Ellenőrzés
findings-detection-title = Észlelés
findings-copied-clipboard-toast = A vágólapra másolva
findings-command-copied-clipboard-toast = A parancs a vágólapra másolva
findings-detections-lazaret-stopped = A { -brand-name } leállása alatti észlelések nem számolhatók meg, mert nem olvasható, hol hagyta abba a rendszernapló olvasását: { $error }
findings-detections-not-read-error = Az észlelések nem olvashatók: { $error }
findings-malware-blocked-signature = Kártevő blokkolva: { $signature }
findings-malware-found-signature = Kártevő található: { $signature }
findings-count-detection-not-running =
    { $count ->
        [one] { $count } észlelés, amíg a { -brand-name } nem futott
       *[other] { $count } észlelés, amíg a { -brand-name } nem futott
    }
findings-detections-not-running-detection = A { -brand-name } leállása alatti észlelések nem számolhatók meg
findings-lazaret-not-read-stopped = A { -brand-name } nem tudta kiolvasni, hol hagyta abba a rendszernapló követését, ezért csak az új észleléseket figyeli. Ellenőrizze az Észlelések oldalt és a rendszernaplót, hogy talált-e valamit, amíg zárva volt.
findings-quarantine-incident = Karanténba helyezés
findings-ask-incident-deletion = Törlés…
findings-delete-incident-file = Végleges törlés
findings-cancel-incident-deletion = Mégse
findings-dismiss-incident = Megtartás
findings-look-up-incident-signature = Keresés
findings-submit-incident-sample = Beküldés…
findings-restored-quarantine = Visszaállítva a karanténból
findings-moved-quarantine = Karanténba helyezve
findings-deleted-label = Törölve
findings-kept-label = Ön megtartotta
findings-belongs-another-user = Egy másik felhasználóé
findings-blocked-needs-decision = Blokkolva, döntésre vár
findings-not-blocked-needs-decision = Nincs blokkolva, döntésre vár
findings-needs-decision = Döntésre vár
findings-no-longer-there-detection = Már nincs ott
findings-state-unknown-detection = A fájlt nem sikerült ellenőrizni
findings-signature-test-file-state = { $signature } (tesztfájl) · { $state }
findings-signature-state = { $signature } · { $state }
findings-access-schedule = A hozzáféréskori vizsgálat és az ütemezett vizsgálat
findings-access-only = Hozzáféréskori vizsgálat
findings-schedule-only = Az ütemezett vizsgálat
findings-file-label = Fájl
findings-signature-source = Aláírásforrás
findings-found-label = Észlelte
findings-last-seen = Utoljára látva
findings-times-seen = Előfordulások száma
findings-detected-label = Észlelve
findings-signature-matches-harmless-eicar = Ez az aláírás az ártalmatlan EICAR tesztmintára illeszkedik, amely az észlelés működésének ellenőrzésére szolgál.
findings-clamav-matched-file-against = A ClamAV ezt a fájlt egy aláírásra illesztette. Az egyezés erős bizonyíték, de előfordulnak téves riasztások, különösen tömörített vagy adminisztrációs eszközök esetén.
findings-signature-first-seen-time = { $signature }, először látva: { $local-time }
findings-opening-file-refused-why = A fájl megnyitása meg van tagadva, ezért nem nyílik meg
findings-not-opened-again-after = A döntése óta nem nyitották meg újra
findings-no-longer-there-blocked = A fájl már nincs ott
findings-not-blocked-scheduled-scan = Nincs blokkolva: az ütemezett vizsgálat találta meg, és továbbra is megnyitható
findings-not-blocked-right-now = Jelenleg nincs blokkolva: továbbra is megnyitható
findings-state-unknown-blocked = Nem sikerült ellenőrizni, hogy a fájl blokkolva van-e
findings-blocked-label = Blokkolva
findings-access-prevention-kernel-refuses = Hozzáféréskori megelőzés esetén a kernel megtagadja a figyelt mappában lévő észlelt fájl megnyitását. A fájl a helyén marad, amíg el nem dönti, mi legyen vele.
findings-looking-up = Keresés…
findings-no-known-samples-match = Egyetlen ismert minta sem illeszkedik erre az aláírásra.
findings-unknown-label = ismeretlen
findings-count-known-samples-families = { $count } ismert minta. Családok: { $families }. Legkorábban látva: { $date }.
findings-ask-malwarebazaar-signature-description = A MalwareBazaar megkérdezése erről az aláírásról. Csak a neve kerül elküldésre.
findings-set-up-lookup-button = Beállítás…
findings-bazaar-no-key = Nincs kulcs
findings-bazaar-key-unreadable = A kulcs nem olvasható
findings-stop-asking-file-description = Ne kérdezzen többet erről a fájlról
findings-delete-file-permanently-description = Véglegesen törli ezt a fájlt? Ez nem vonható vissza.
findings-go-back-without-description = Vissza törlés nélkül
findings-move-somewhere-safe-description = Áthelyezés biztonságos helyre. Később visszaállíthatja.
findings-remove-good-description = Végleges eltávolítás
findings-know-file-fine-description = Tudja, hogy ez a fájl rendben van. A ClamAV a következő vizsgálatnál továbbra is jelenti; ennek megszüntetéséhez jelölje megbízhatónak a Karantén oldalon vagy egy vizsgálati eredményben.
findings-decision-title = Az Ön döntése
findings-unfamiliar-quarantine-keeping-file = Ha ismeretlen Önnek, helyezze karanténba. Egy fájl megtartása csak a { -brand-name } kérdezését állítja le: a hozzáféréskori vizsgálat addig blokkolja, amíg az elérési út nincs kizárva a ClamAV konfigurációjában, ahogy itt látható.
findings-quarantined-files-restored-quarantine = A karanténba helyezett fájlok a Karantén oldalon állíthatók vissza. Egy megtartott fájlt a hozzáféréskori vizsgálat addig blokkol, amíg az elérési útja nincs kizárva a ClamAV konfigurációjában.
findings-file-other-owner-decision = Ez a fájl egy másik felhasználóé, ezért a { -brand-name } nem tudja áthelyezni vagy törölni
findings-state-unknown-review = A { -brand-name } nem tudta ellenőrizni ezt a fájlt, ezért nem tudja áthelyezni vagy törölni
findings-choose-happens-file = Válassza ki, mi történjen ezzel a fájllal
findings-malwarebazaar-label = MalwareBazaar
findings-resolved-title = Megoldva
findings-resolved-detection-opens-again = Egy megoldott észlelés újra megnyílik, ha a ClamAV később ismét megtalálja ugyanazt a fájlt, például ha másodszor is letölti.
findings-waiting-decision = Az Ön döntésére vár
findings-resolved-state-step = { $state }, { $time }
findings-send-quarantined-copy-description = A karanténba helyezett példány elküldése a ClamAV, a MalwareBazaar vagy a VirusTotal számára. A ClamAV-nak jelentett téves riasztást egy későbbi aláírásfrissítés mindenki számára kijavítja.
findings-cancel-quarantine = Mégse
findings-delete-entries = Törlés
findings-scanner-program-not-installed = A vizsgálóprogram nincs telepítve.
findings-scanner-program-replaced-account = A(z) { $program } helyen lévő vizsgálóprogramot a fiókja lecserélheti a következőn keresztül: { $path }.
findings-content-pipe-failed = A fájlt nem sikerült átadni a vizsgálónak: { $error }
findings-scanner-spawn-failed = A vizsgálót nem sikerült elindítani: { $error }
findings-verdict-timed-out = A vizsgálat túl sokáig tartott.
findings-detection-settings-unreadable = A clamd észlelési beállításai nem olvashatók: { $error }
findings-program-lookup-failed = A vizsgálóprogramokat nem sikerült megkeresni: { $error }
findings-payload-unreadable = A karanténba helyezett példány nem olvasható: { $error }
findings-restore-file-heading = Visszaállítja ezt a fájlt?
findings-clamav-no-longer-verdict = A ClamAV a mai aláírásokkal már nem észleli ezt a fájlt.
findings-restore-button = Visszaállítás
findings-restore-file-clamav-heading = Visszaállít egy fájlt, amelyet a ClamAV még észlel?
findings-clamav-still-detects-verdict = A ClamAV a mai aláírásokkal továbbra is { $detection } néven észleli ezt a fájlt. Csak akkor állítsa vissza, ha biztos benne, hogy téves riasztásról van szó, és előtte fontolja meg, hogy megbízhatónak jelöli.
findings-clamav-still-detects-unnamed-verdict = A ClamAV a mai aláírásokkal továbbra is észleli ezt a fájlt, de nem nevezi meg az észlelést. Csak akkor állítsa vissza, ha biztos benne, hogy téves riasztásról van szó, és előtte fontolja meg, hogy megbízhatónak jelöli.
findings-restore-anyway-button = Visszaállítás mégis
findings-restore-unchecked-file-heading = Visszaállít egy ellenőrizetlen fájlt?
findings-file-not-checked-verdict = A fájlt nem sikerült visszaállítás előtt újra ellenőrizni. { $reason }
findings-location-outside-home-folder = Ez a hely a saját mappáján kívül található.
findings-file-goes-back = A fájl ide kerül vissza:
findings-entry-count =
    { $count ->
        [one] { $count } elem
       *[other] { $count } elem
    }
findings-clear-old-items = Törli a régi elemeket?
findings-count-quarantined-file-older =
    { $count ->
        [one] { $count } 30 napnál régebbi karanténba helyezett fájl véglegesen törlődik.
       *[other] { $count } 30 napnál régebbi karanténba helyezett fájl véglegesen törlődik.
    }
findings-delete-quarantined-file = Törli a karanténba helyezett fájlt?
findings-file-permanently-deleted = A fájl véglegesen törlődik.
findings-quarantine-title = Karanténba helyezés
findings-more-tooltip = Továbbiak
findings-clear-old-items-label = Régi elemek törlése…
findings-check-again-label = Ellenőrzés újra
findings-scan-every-quarantined-tooltip = Minden karanténba helyezett fájl vizsgálata a jelenlegi aláírásokkal
findings-refresh-tooltip = Frissítés
findings-search-tooltip = Keresés
findings-search-threat-name-placeholder = Keresés fenyegetésnév vagy elérési út alapján
findings-quarantine-storage-title = Karanténtároló
findings-detected-threats-isolated-description = A rendszeréből elkülönített észlelt fenyegetések. Bontson ki egy bejegyzést a hírnevének ellenőrzéséhez.
findings-total-size-title = Teljes méret
findings-dismiss-label = Elvetés
findings-quarantine-cannot-read-title = A karantén nem olvasható
findings-loading-quarantine = Karantén betöltése…
findings-quarantine-empty = A karantén üres
findings-hash-copied-clipboard = A kivonat a vágólapra másolva
findings-every-quarantined-file-still = Minden karanténba helyezett fájl továbbra is észlelhető, vagy nem sikerült ellenőrizni
findings-count-quarantined-file-no =
    { $count ->
        [one] { $count } karanténba helyezett fájl már nem észlelhető. Visszaállítás előtt ellenőrizze.
       *[other] { $count } karanténba helyezett fájl már nem észlelhető. Visszaállítás előtt ellenőrizze őket.
    }
findings-count-report-clamav-settled =
    { $count ->
        [one] { $count } ClamAV-jelentést rendeztek az új aláírások
       *[other] { $count } ClamAV-jelentést rendeztek az új aláírások
    }
findings-checking-label = Ellenőrzés…
findings-no-longer-detected = Már nem észlelhető
findings-still-detected = Továbbra is észlelhető
findings-integrity-unknown-hash = Ismeretlen. Ennek a karantén-bejegyzésnek nincs használható kivonata, így a fájl nem ellenőrizhető, nem nézhető meg online, nem jelölhető megbízhatónak és nem állítható vissza. Törölni viszont lehet.
findings-not-checked = Még nincs ellenőrizve
findings-integrity-unknown-label = Ismeretlen sértetlenség
findings-restore-label = Visszaállítás
findings-delete-entry-label = Törlés
findings-reputation-title = Hírnév
findings-look-up-reputation-button = Keresés
findings-set-up-source-button = Beállítás…
findings-reputation-no-key = Nincs kulcs
findings-reputation-key-unreadable = A kulcs nem olvasható
findings-file-sha-256-subtitle = Csak a fájl SHA-256 kivonata kerül elküldésre. Maga a fájl sosem hagyja el ezt a számítógépet.
findings-view-report-label = Jelentés megtekintése
findings-false-positive-title = Téves riasztás
findings-trust-stops-clamav-subtitle = A Megbízhatónak jelölés… leállítja, hogy a ClamAV ezen a számítógépen jelentse ezt a pontos fájlt, és felveszi a Megbízható észlelések közé; utána állítsa vissza. A Jelentés… elküldi a Cisco Talos számára, így egy későbbi aláírásfrissítés mindenki számára kijavítja az észlelést.
findings-report-label = Jelentés…
findings-submit-quarantine-label = Beküldés…
findings-send-clamav-malwarebazaar-tooltip = Elküldés a ClamAV, a MalwareBazaar vagy a VirusTotal számára
findings-trust-label = Megbízhatónak jelölés…
findings-copy-hash-tooltip = Kivonat másolása
findings-submit-quarantined-label = Beküldés…
findings-service-label = Szolgáltatás
findings-status-label = Állapot
findings-quarantine-record-no-usable = Ennek a karantén-bejegyzésnek nincs használható kivonata, ezért a { -brand-name } sehová sem küldi el
findings-file-no-longer-quarantine = Ez a fájl már nincs karanténban
findings-service-not-take-detection = Ez a szolgáltatás nem fogadja ezt az észlelést
findings-file-no-usable-hash = Ez a fájl már nincs használható kivonattal karanténban
findings-check-malwarebazaar-virustotal-tooltip = Újdonságok keresése a MalwareBazaar és a VirusTotal szolgáltatásban
findings-checking-sent-tooltip = Ellenőrzés
findings-send-file-title = Fájl küldése
findings-nothing-leaves-computer-description = Semmi sem hagyja el ezt a számítógépet, amíg meg nem erősíti, mi kerül elküldésre.
findings-report-missed-malware-label = Kimaradt kártevő jelentése…
findings-send-clamav-malicious-tooltip = Olyan kártékony fájl küldése a ClamAV számára, amelyet nem észlel
findings-sent-title = Elküldve
findings-lazaret-sent-became-description = Mit küldött el a { -brand-name }, és mi lett vele.
findings-filter-sent-samples-tooltip = Elküldött minták szűrése
findings-online-services-title = Online szolgáltatások
findings-each-service-receives-subtitle = Mit kapnak az egyes szolgáltatások, valamint a MalwareBazaar- és VirusTotal-kulcsok
findings-choose-file-clamav-misses = Válassza ki a fájlt, amelyet a ClamAV nem észlel
findings-false-positive = Téves riasztás
findings-missed-malware = Kimaradt kártevő
findings-malware-sample = Kártevőminta
findings-clamav-false-positive = Téves riasztás jelentése a ClamAV számára
findings-clamav-missed-malware = Kimaradt kártevő jelentése a ClamAV számára
findings-malwarebazaar-sample-choice = Feltöltés a MalwareBazaar szolgáltatásba
findings-virustotal-sample-choice = Feltöltés a VirusTotal szolgáltatásba
findings-awaiting-clamav = ClamAV-felülvizsgálatra vár. A felülvizsgálat legalább 48 órát vesz igénybe; az eredményt egy későbbi aláírásfrissítés mutatja.
findings-fixed-database-version = Javítva a(z) { $version } adatbázisverzióban
findings-fixed-current-signature-database = Javítva a jelenlegi aláírás-adatbázisban
findings-detected-since-database-version = Észlelve a(z) { $version } adatbázisverzió óta
findings-detected-current-signature-database = A jelenlegi aláírás-adatbázis észleli
findings-uploaded-waiting-malwarebazaar-list = Feltöltve; várakozás, hogy a MalwareBazaar listázza
findings-listed-malwarebazaar = Listázva a MalwareBazaar szolgáltatásban
findings-malwarebazaar-refused-reason = A MalwareBazaar elutasította: { $reason }
findings-queued-analysis-virustotal = Elemzésre vár a VirusTotal szolgáltatásban
findings-count-total-engine-flags =
    { $total ->
        [one] { $total } motorból { NUMBER($count) } kártékonynak jelöli
       *[other] { $total } motorból { NUMBER($count) } kártékonynak jelöli
    }
findings-virustotal-refused-reason = A VirusTotal elutasította: { $reason }
findings-service-kind-hash-date = { $service } · { $kind } · { $hash } · { $date }
findings-integrity-unknown-candidate = { $detection } · Nincs használható kivonat, így a { -brand-name } nem tudja igazolni, mit küldene el
findings-all-services = Minden szolgáltatás
findings-any-status-option = Bármely állapot
findings-waiting-label = Várakozik
findings-settled-label = Rendezve
findings-refused-label = Elutasítva
findings-all-submissions-option = Minden beküldés
findings-listing-loading = Beküldések betöltése…
findings-nothing-submitted = Nincs beküldött elem
findings-no-submission-matches-filters = Egyetlen beküldés sem felel meg a szűrőknek
findings-candidates-loading = Karantén betöltése…
findings-quarantine-empty-sent = A karantén üres
findings-no-upload-changed = Egyetlen feltöltés sem változott
findings-count-upload-new-status =
    { $count ->
        [one] { $count } feltöltés állapota megváltozott
       *[other] { $count } feltöltés állapota megváltozott
    }
findings-choose-service-heading = Hová kerüljön ez a fájl?
findings-set-up-keys-response = Kulcsok beállítása…
findings-each-service-gets-file = Minden szolgáltatás magát a fájlt kapja meg. A következő lépés pontosan megmutatja, mi kerül elküldésre, és ki láthatja.
findings-cancel-submission = Mégse
findings-open-remote-entry-tooltip = Távoli bejegyzés megnyitása
findings-same-upload-running = Ez a fájl már küldés alatt áll ehhez a szolgáltatáshoz. Várja meg, amíg befejeződik, mielőtt újra elküldené.
findings-check-again-trusted-tooltip = Ellenőrzés újra
findings-checking-trusted-tooltip = Ellenőrzés
findings-status-title = Állapot
findings-files-trusted-detections-description = A { -brand-name } alkalmazással megbízhatónak jelölt fájlok és figyelmen kívül hagyott vagy visszaállított észlelések, a rendszer nyilvántartása szerint. Az Ön nevében futó szoftverek nem tudják eltávolítani ezeket a bejegyzéseket.
findings-no-trusted-detections-title = Nem módosultak megbízható észlelések a { -brand-name } alkalmazással
findings-checking-trusted-detections = Megbízható észlelések ellenőrzése…
findings-command-copied-clipboard = A parancs a vágólapra másolva
findings-other-allowlist = Egyéb engedélyezési lista
findings-file-reason = { $file }: { $reason }
findings-trusted-files = Megbízható fájlok
findings-ignored-detections = Figyelmen kívül hagyott észlelések
findings-allowlist-not-write-status = Egy nem a { -brand-name } által írt engedélyezési lista hallgatásra utasítja a ClamAV-t
findings-count-file-detection-status =
    { $count ->
        [one] { $count } fájl vagy észlelés sosem kerül jelentésre
       *[other] { $count } fájl vagy észlelés sosem kerül jelentésre
    }
findings-own-allowlist-unreadable-status = A { -brand-name } nem tudja olvasni a saját engedélyezési listáját, így nem tudja megállapítani, miről hallgat a ClamAV
findings-folder-unreadable = A { -brand-name } nem tudja olvasni az aláírásmappát, így nem tudja megállapítani, miről hallgat a ClamAV
findings-signature-folder = Aláírásmappa
findings-trusted-detections = Megbízható észlelések
findings-trust-list-mechanism = A ClamAV engedélyezési listáin szereplő fájlokat és észlelési neveket egyetlen vizsgálat sem jelenti. Egy rendszergazdai jogokat szerzett kártevő így elrejtheti magát, ezért ellenőrizze, hogy minden bejegyzést szándékosan vettek-e fel.
findings-clamav-reports-everything-detects = A ClamAV mindent jelent, amit észlel
findings-remove-tooltip = Eltávolítás
findings-access-denied = Hozzáférés megtagadva
findings-other-read-failure = Az olvasás sikertelen: { $reason }
findings-reading-task-aborted = Az olvasás befejezés előtt leállt
findings-journal-unreadable-added = Nem tudható, ki vette fel. { $reason }
findings-known-user = Felvette: { $uid } azonosítójú felhasználó, ekkor: { $local-time }
findings-unknown-user = Felvette: ismeretlen felhasználó, ekkor: { $time }
findings-no-record-added-system = A rendszernaplóban nincs nyilvántartás arról, ki vette fel
findings-count-entry-not-added =
    { $count ->
        [one] { $count } bejegyzés, nem a { -brand-name } alkalmazással felvéve
       *[other] { $count } bejegyzés, nem a { -brand-name } alkalmazással felvéve
    }
findings-entries-cannot-counted-reason = A bejegyzések nem számolhatók meg ({ $reason }), nem a { -brand-name } alkalmazással felvéve
findings-list-unreadable = A { -brand-name } nem tudja olvasni ezt a listát
findings-no-trusted-files = Nincsenek megbízható fájlok
findings-no-ignored-detections = Nincsenek figyelmen kívül hagyott észlelések
findings-lazaret-cannot-read-signature = A { -brand-name } nem tudja olvasni az aláírásmappát
findings-trusted-detections-title = Megbízható észlelések
findings-trusted-by-you-title = Ön által megbízhatónak jelölt
findings-trusted-by-you-description = A vizsgálatai nem mutatják Önnek ezeket a fájlokat, és sem a vizsgálatai, sem a háttérbeli riasztások nem mutatják ezeket az észleléseket. Maga a ClamAV továbbra is észleli őket, így a hozzáféréskori vizsgálat továbbra is blokkolhatja a fájlokat, és más felhasználók továbbra is látják őket. Egy megbízható fájl bármilyen módosítása után újra megjelenik.
findings-nothing-trusted-by-you = Még semmit sem jelölt megbízhatónak
findings-trust-by-you-unreadable = A { -brand-name } nem tudja olvasni, mit jelölt megbízhatónak: { $error }
findings-file-trusted-by-you = Egy Ön által megbízhatónak jelölt fájl
findings-trust-for-all-users-tooltip = Megbízható minden felhasználónak…
findings-trusted-for-all-users-title = Minden felhasználó számára megbízható
findings-clamav-never-reports-description = A ClamAV sosem jelenti az itt felsoroltakat, egyetlen felhasználónál és egyetlen vizsgálatban sem ezen a számítógépen. Minden módosításhoz rendszergazdai jelszó szükséges, és a rendszernaplóba kerül, ahonnan az Ön nevében futó szoftverek nem tudják eltávolítani a bejegyzést.
findings-ignored-detections-title = Figyelmen kívül hagyott észlelések
findings-detection-names-clamav-description = Észlelési nevek, amelyeket a ClamAV minden fájlban, minden felhasználónál figyelmen kívül hagy. A { -brand-name } már nem vesz fel ilyeneket, mert elrejtenek minden ilyen észlelésű fájlt; távolítsa el őket, és inkább a kivonatuk alapján jelöljön meg megbízható fájlokat.
findings-other-allowlists-title = Egyéb engedélyezési listák
findings-allowlist-files-signature-description = Az aláírásmappában lévő engedélyezésilista-fájlok, amelyeket nem a { -brand-name } írt. Ellenőrizze, hogy Ön vagy a rendszergazdája szándékosan vette-e fel őket.

signatures-history-load-failed = Nem sikerült betölteni az előzmény-naplókat: { $error }
signatures-database-update-finished = Az adatbázis frissítése befejeződött
signatures-check-again-database-tooltip = Ellenőrzés újra
signatures-checking-database-tooltip = Ellenőrzés
signatures-update-history-title = Frissítési előzmények
signatures-updates-started-lazaret-description = A { -brand-name } alkalmazásból indított frissítések, a legújabb elöl. Válasszon ki egyet az átirata megtekintéséhez.
signatures-historical-logs-cannot-title = Az előzménynaplók nem olvashatók
signatures-status-title = Állapot
signatures-loading-history = Előzmények betöltése…
signatures-no-updates-started-lazaret = Még nem indítottak frissítést a { -brand-name } alkalmazásból
signatures-checking-signatures = Az aláírások ellenőrzése…
signatures-command-copied-clipboard-database = A parancs a vágólapra másolva
signatures-update-not-added-history = A frissítést nem sikerült felvenni az előzmények közé: { $error }
signatures-update-signatures = Frissítés most
signatures-cancel-signature-update = Mégse
signatures-definitions-built-date = Definíciók készítése: { $relative-time } ({ $date })
signatures-definitions-exist-not-determined = Vannak definíciók, de a koruk nem állapítható meg
signatures-no-virus-definitions-found = Nem találhatók vírusdefiníciók
signatures-age-definitions-unknown-checking = A definíciók kora ismeretlen, mert az ellenőrzésük befejezés előtt leállt
signatures-last-update-failed = A legutóbbi frissítése sikertelen volt
signatures-no-completed-update = Még nincs rögzített befejezett frissítés
signatures-updater-updated = Frissítette: { $updater }
signatures-provider-fetched = Letöltve innen: { $provider }
signatures-count-signature-size-file =
    { $count ->
        [one] { $count } aláírás, { $size } ({ $file })
       *[other] { $count } aláírás, { $size } ({ $file })
    }
signatures-file-unreadable = Ismeretlen számú aláírás, a fájl nem olvasható: { $error }, { $size } ({ $file })
signatures-other-signature-sources = Egyéb aláírásforrások
signatures-unknown-reason-database = Ismeretlen. { $reason }
signatures-fetch-newest-signatures = A legújabb aláírások letöltése most minden frissítőből
signatures-manual-update = Kézi frissítés
signatures-update-service = Frissítési szolgáltatás
signatures-database-folder = Adatbázismappa
signatures-updater-output = A frissítő kimenete
signatures-current-label = Aktuális aláírások
signatures-signature-mechanism = Az aláírások ismert kártevőket írnak le, és folyamatosan jelennek meg újak. A régi aláírások nem ismerik fel a friss fenyegetéseket, még ha minden más működik is.
signatures-updating-signatures = Aláírások frissítése…
signatures-update-cancelling = A frissítés megszakítása…
signatures-starting-update = A frissítés indítása…
signatures-daemon-running = Aktív
signatures-timer-scheduled = Egy systemd időzítő ütemezi
signatures-service-stopped = Telepítve, de leállítva
signatures-service-not-found = Nem található freshclam szolgáltatás; a frissítések kézzel futnak
signatures-systemd-unreachable-service = A systemd nem volt lekérdezhető freshclam szolgáltatásról; a frissítések kézzel futnak
signatures-count-day-old =
    { $count ->
        [one] { $count } napos
       *[other] { $count } napos
    }
signatures-definitions-missing = Nem található olvasható vírusadatbázis
signatures-directory-unreadable = Ismeretlen. { $reason }
signatures-scanner-error = A freshclam { $code } kilépési kóddal hibázott.
signatures-killed-signal = A freshclam folyamatot a(z) { $signal } jelzés állította le.
signatures-unrecognized-wait-status = A freshclam olyan módon fejeződött be, amelyet a { -brand-name } nem ismer fel.
signatures-wait-failed = A { -brand-name } elvesztette a freshclam nyomát, mielőtt az befejeződött volna.
signatures-authorization-refused = A hitelesítés sikertelen. Próbálja újra, és adja meg a jelszavát.
signatures-systemd-not-reached = A systemd nem érhető el a frissítés futtatásához.
signatures-service-job-ended = A frissítési szolgáltatás nem fejeződött be; a systemd jelentése: { $result }
signatures-update-rate-limited-mirror = A tükör korlátozta a frissítések gyakoriságát. Várjon néhány percet, és próbálja újra.
signatures-update-blocked-cdn = A CDN blokkolta a frissítést. Ennek oka gyakorisági korlátozás lehet. Várjon, és próbálja újra később.
signatures-mirror-unavailable = A ClamAV tükör jelenleg nem érhető el. Próbálja újra később.
signatures-certificate-rejected = SSL/TLS tanúsítványhiba. A tükör konfigurációja hibás lehet.
signatures-connection-timed-out = A kapcsolat túllépte az időkorlátot. Ellenőrizze a hálózati kapcsolatát.
signatures-database-locked = Az adatbázis zárolva van. Lehet, hogy egy másik freshclam példány fut.
signatures-permission-denied = Hozzáférés megtagadva. Az adatbázis frissítéséhez emelt szintű jogosultságok kellhetnek.
signatures-connection-failed = Kapcsolódási hiba. Ellenőrizze a hálózati kapcsolatát.
signatures-name-resolution-failed = A DNS-feloldás sikertelen. Ellenőrizze a hálózati beállításait.
signatures-update-failed-unknown-error = A frissítés ismeretlen hibával meghiúsult. A részleteket a naplókban találja.
signatures-updated-databases = Frissítve: { $databases }
signatures-already-current-databases = Már naprakész: { $databases }
signatures-cooldown-unknown = Gyakoriság korlátozva: { $database }
signatures-cooldown-until = Gyakoriság korlátozva: { $database }, eddig: { $cooldown }
signatures-definitions-updated = Az adatbázis frissítése befejeződött
signatures-already-current = Az adatbázis már naprakész
signatures-service-ran-without-verdict = A frissítési szolgáltatás lefutott, de a naplójából nem derül ki, változtak-e a definíciók.
signatures-update-requested-daemon = A szolgáltatás elfogadta a jelzést; a frissítést aszinkron módon végzi. A befejezést a naplókban ellenőrizheti.
signatures-update-cancelled = Az adatbázis-frissítés megszakítva
signatures-scanner-program-missing = Az adatbázis-frissítés kérése előtt telepítse a freshclam programot.
signatures-provider-needs-credential-fangfrisch =
    { $credential ->
        [purchase-receipt] A(z) { $provider } szolgáltatónak vásárlási nyugtaszám szükséges, és a fangfrisch csak a beállításaiból olvassa be. NixOS-en a services.clamav.fangfrisch.settings a Nix store-ba másolódik, amelyet a számítógép minden felhasználója olvashat, így a vásárlási nyugtaszám nyilvánossá válna. Hagyja kikapcsolva a(z) { $provider } szolgáltatót, vagy futtassa a fangfrischt a Nix store-on kívül tárolt beállításfájllal, például agenix- vagy sops-nix-titokkal.
       *[customer-identifier] A(z) { $provider } szolgáltatónak ügyfél-azonosító szükséges, és a fangfrisch csak a beállításaiból olvassa be. NixOS-en a services.clamav.fangfrisch.settings a Nix store-ba másolódik, amelyet a számítógép minden felhasználója olvashat, így az ügyfél-azonosító nyilvánossá válna. Hagyja kikapcsolva a(z) { $provider } szolgáltatót, vagy futtassa a fangfrischt a Nix store-on kívül tárolt beállításfájllal, például agenix- vagy sops-nix-titokkal.
    }
signatures-provider-needs-credential-website =
    { $credential ->
        [purchase-receipt] A(z) { $provider } szolgáltatónak a weboldaláról származó vásárlási nyugtaszám szükséges. A fangfrisch a beállításfájljából olvassa be, amely általában ez: { $path }. Adja hozzá ott ezeket a sorokat, cserélje le a helykitöltőt a vásárlási nyugtaszámára, és tegye a fájlt csak a root számára olvashatóvá, hogy más felhasználók ne láthassák.
       *[customer-identifier] A(z) { $provider } szolgáltatónak a weboldaláról származó ügyfél-azonosító szükséges. A fangfrisch a beállításfájljából olvassa be, amely általában ez: { $path }. Adja hozzá ott ezeket a sorokat, cserélje le a helykitöltőt az ügyfél-azonosítójára, és tegye a fájlt csak a root számára olvashatóvá, hogy más felhasználók ne láthassák.
    }
signatures-not-use = Nincs használatban
signatures-add-lines-fangfrisch-settings = Adja hozzá ezeket a sorokat a fangfrisch beállításfájljához, amely általában ez: { $path }.
signatures-provider-needs-free-abuse = A(z) { $provider } szolgáltatónak ingyenes abuse.ch Auth-Key szükséges. Adja hozzá ezeket a sorokat a fangfrisch beállításfájljához, amely általában ez: { $path }, cserélje le a helykitöltőt a kulcsára, és tegye a fájlt csak a root számára olvashatóvá, hogy más felhasználók ne láthassák.
signatures-use-label = Használatban
signatures-unknown-reason-source = Ismeretlen. { $reason }
signatures-check-again-signature-tooltip = Ellenőrzés újra
signatures-checking-signature-tooltip = Ellenőrzés
signatures-clamav-malware-signatures-description = Honnan származnak a ClamAV kártevő-aláírásai, és milyen egyéb forrásokat adhat hozzá.
signatures-sources-title = Aláírásforrások
signatures-abuse-ch-key-title = abuse.ch-kulcs
signatures-abuse-ch-auth-title = abuse.ch Auth-Key
signatures-abuse-ch-key-uses = Az URLhaus-aláírásokhoz, valamint a MalwareBazaar-keresésekhez és -feltöltésekhez.
signatures-abuse-ch-no-key = Nincs kulcs
signatures-abuse-ch-key-unreadable = A kulcs nem olvasható
signatures-command-copied-clipboard-signature = A parancs a vágólapra másolva
signatures-not-open-documentation-title = A dokumentáció nem nyitható meg: { $error }
signatures-checking-api-key = Az API-kulcs ellenőrzése…
signatures-check-passed = Sikeres
signatures-needs-review = Figyelmeztetés
signatures-check-failed = Sikertelen
signatures-status-unknown = Ismeretlen
signatures-check-skipped = Kihagyva
signatures-check-progress = Ellenőrzés
signatures-copy-tooltip = Másolás
signatures-learn-more-title = További információk
signatures-open-browser-tooltip = Megnyitás böngészőben
signatures-checking-title = Ellenőrzés…

preferences-launcher-not-found = A { -brand-name } nem találja a saját indítóját: { $error }
preferences-action-unwritable = A fájlkezelő-műveletet nem sikerült hozzáadni: { $error }
preferences-action-unremovable = A fájlkezelő-műveletet nem sikerült eltávolítani: { $error }
preferences-dialog-title = Beállítások
preferences-general-title = Általános
preferences-window-behavior-title = Ablak viselkedése
preferences-detection-alerts-work-description = Az észlelési riasztások csak a { -brand-name } futása közben működnek.
preferences-closing-window-title = Ablak bezárásakor
preferences-start-background-title = Indítás a háttérben
preferences-launch-background-without-subtitle = Indítás a háttérben, ablak megnyitása nélkül.
preferences-desktop-notifications-title = Asztali értesítések
preferences-notify-detections-finished-subtitle = Értesítés az észlelésekről, valamint a befejezett vizsgálatokról és frissítésekről, amikor az ablak nincs fókuszban.
preferences-file-manager-integration-title = Fájlkezelő-integráció
preferences-add-context-menu-description = Helyi menü műveleteinek hozzáadása a támogatott fájlkezelőkhöz.
preferences-online-services-title = Online szolgáltatások
preferences-online-services-summary = A Keresés csak egy kivonatot vagy egy aláírásnevet küld el. Egy fájl csak akkor hagyja el ezt a számítógépet, ha beküldi.
preferences-circl-receives-summary = Megkapja egy fájl SHA-256-kivonatát, és megmondja, hogy ismert, ártalmatlan fájl-e. Nem kell kulcs.
preferences-talos-receives-summary = Megkapja a bejelentett fájlokat a nevével és e-mail-címével. A Talos soha nem teszi őket nyilvánossá. Nem kell kulcs.
preferences-virustotal-receives-summary = Megkapja egy fájl SHA-256-kivonatát. A feltöltött fájlokat a VirusTotal megosztja fizető ügyfeleivel.
preferences-malwarebazaar-receives-summary = Megkap egy aláírásnevet vagy egy fájl SHA-256-kivonatát. A feltöltött fájlok nyilvánossá válnak.
preferences-key-file-not-permitted-hint = Tegye olvashatóvá a felhasználója számára, például az agenix- vagy sops-nix-titok tulajdonosának beállításával.
preferences-key-file-missing-hint = Ellenőrizze a kulcsfájl útvonalát a rendszerkonfigurációban.
preferences-key-file-unreadable-hint = Ellenőrizze, hogy az útvonal egy szabályos fájlra mutat-e.
preferences-key-not-valid-hint = Másolja ki a teljes kulcsot a szolgáltatás weboldaláról.
preferences-keyring-unavailable-hint = Oldja fel a kulcstartót, majd nyissa meg újra a Beállításokat.
preferences-stored-key-unreadable-hint = Törölje a kulcsot, és illessze be újra.
preferences-virustotal-service-hint = Ingyenes csomag: percenként 4, naponta 500 kérés.
preferences-abuse-ch-service-hint = Ingyenesen beszerezhető az abuse.ch Authentication Portal oldalon.
preferences-not-configured = Nincs beállítva
preferences-checking-label = Ellenőrzés…
preferences-set-system-configuration-title = A rendszerkonfiguráció állítja be
preferences-key-keyring-title = Kulcs a kulcstartóban
preferences-key-file-title = Kulcsfájl
preferences-key-unreadable-status = Nem olvasható
preferences-key-file-not-permitted = A kulcsfájl nem olvasható: nincs jogosultság
preferences-key-file-missing = A kulcsfájl nem létezik
preferences-key-file-unreadable = A kulcsfájl nem olvasható
preferences-key-not-valid = Nem érvényes kulcs ehhez a szolgáltatáshoz
preferences-keyring-unavailable = A kulcstartó nem érhető el
preferences-stored-key-unreadable = A kulcstartóban lévő kulcs nem olvasható
preferences-delete-key-label = Kulcs törlése
preferences-paste-api-key-title = API-kulcs beillesztése
preferences-get-free-api-title = Ingyenes API-kulcs beszerzése

widgets-clean-label = Tiszta
widgets-scan-infected = Fertőzött
widgets-error-status = Hiba
widgets-cancelled-label = Megszakítva
widgets-skipped-label = Kihagyva
widgets-success-label = Sikeres
widgets-up-date = Naprakész
widgets-requested-label = Kérve
widgets-ran-label = Lefutott
widgets-counted-threats =
    { $count ->
        [one] { $count } fenyegetés található
       *[other] { $count } fenyegetés található
    }
widgets-uncounted-threats = A vizsgálóprogram fenyegetést jelentett a fájl megnevezése nélkül.
widgets-no-threats-found-activity = Nem található fenyegetés
widgets-scan-failed = A vizsgálat nem fejeződött be hibátlanul
widgets-scan-cancelled-activity = A felhasználó megszakította a vizsgálatot
widgets-scheduled-scan-skipped-battery = Az ütemezett vizsgálat kihagyva (akkumulátoros üzemmódban)
widgets-virus-definitions-updated = A vírusdefiníciók frissítve
widgets-virus-definitions-up-date = A vírusdefiníciók naprakészek
widgets-update-requested-service = Frissítés kérve a szolgáltatástól
widgets-update-service-no-result = A frissítési szolgáltatás eredmény jelentése nélkül futott le
widgets-update-failed = Az adatbázis frissítése sikertelen
widgets-update-cancelled = Az adatbázis-frissítés megszakítva
widgets-scan-log = VIZSGÁLATI NAPLÓ
widgets-update-log = FRISSÍTÉSI NAPLÓ
widgets-timestamp-label = Időbélyeg
widgets-status-label = Állapot
widgets-duration-label = Időtartam
widgets-summary-label = Összegzés
widgets-path-label = Útvonal
widgets-full-output = Teljes kimenet
widgets-scan-history-cannot-title = A vizsgálati előzmények nem olvashatók
widgets-retry-label = Újra
widgets-copy-activity-tooltip = Másolás
widgets-log-details-copied-toast = A napló részletei a vágólapra másolva
widgets-not-save-file-error = A fájl nem menthető: { $error }
widgets-not-choose-file-error = A fájl nem választható ki: { $error }
widgets-chosen-file-not-local = { $location } nem ezen a számítógépen lévő fájl
widgets-export-title = Exportálás
widgets-database-newer-release = Ezt az adatbázist a { -brand-name } egy újabb verziója hozta létre: a szerkezeti verziója { $found }, ez a verzió pedig legfeljebb a(z) { $known } verziót ismeri. A megnyitásához frissítse a { -brand-name } alkalmazást.
widgets-migration-step-failed = Az adatbázis szerkezetének frissítése meghiúsult a(z) { $step }. lépésnél: { $error }
widgets-version-check-failed = Az adatbázis szerkezeti verziója nem ellenőrizhető: { $error }
widgets-copy-configuration-line-title = Konfigurációs sor másolása
widgets-nothing-applied-choose-description = Semmi sem kerül alkalmazásra. Válasszon egy értéket, másolja a sort, majd adja hozzá saját kezűleg a konfigurációjához.
widgets-unit-title = Mértékegység
widgets-line-copy-title = Másolandó sor
widgets-copy-line-label = Sor másolása
widgets-unlock-required = A beállítás módosításához oldja fel a zárolást. Zárolt állapotban a konfigurációjához másolhatja a sorát.
widgets-administrator-not-copy = A rendszergazdája nem engedélyezi ennek a beállításnak a módosítását a { -brand-name } alkalmazásból. A konfigurációjához másolhatja a sorát.
widgets-plain-bytes = bájt
widgets-kibibytes-unit = KiB
widgets-mebibytes-unit = MiB
widgets-unknown-option = A { -brand-name } nem ismeri ezt a beállítást, ezért csak megjeleníti.
widgets-not-yes-no = Felülvizsgálatot igényel: ennek az értéknek yes vagy no értékűnek kell lennie.
widgets-not-whole-number = Felülvizsgálatot igényel: ennek az értéknek egész számnak kell lennie.
widgets-count-out-range = Felülvizsgálatot igényel: ennek az értéknek { $minimum } és { $maximum } között kell lennie.
widgets-not-size = Felülvizsgálatot igényel: ennek az értéknek bájtban megadott méretnek, vagy K vagy M betűvel követett számnak kell lennie.
widgets-path-not-absolute = Felülvizsgálatot igényel: ennek az értéknek /-rel kezdődő teljes elérési útnak kell lennie.
widgets-empty-text = Felülvizsgálatot igényel: ez az érték üres.
widgets-apply-tooltip = Alkalmaz
widgets-unit-tooltip = Mértékegység
widgets-setting-runs-commands-changes = Ez a beállítás parancsokat futtat, vagy megváltoztatja, kinek a nevében fut a ClamAV, ezért a { -brand-name } sosem kínálja fel a módosítását
widgets-check-value-configuration-file = Ellenőrizze ezt az értéket a konfigurációs fájlban.
widgets-copy-configuration-line-tooltip = Konfigurációs sor másolása…
widgets-applying-restarting-clamav-tooltip = Alkalmazás és a ClamAV újraindítása
widgets-detect-potentially-unwanted-programs = Esetlegesen nemkívánatos programok észlelése
widgets-heuristic-detections = Heurisztikus észlelések
widgets-flag-office-files-macros = Makrókat tartalmazó Office-fájlok megjelölése
widgets-flag-encrypted-archives-documents = Titkosított archívumok és dokumentumok megjelölése
widgets-look-inside-archives = Archívumok tartalmának vizsgálata
widgets-flag-files-too-large = A teljes ellenőrzéshez túl nagy fájlok megjelölése
widgets-adware-hacking-tools-remote = Reklámprogramok, hackereszközök és távelérési eszközök. Több téves riasztásra számíthat.
widgets-suspicious-files-no-signature = Gyanús fájlok, amelyeket egyetlen aláírás sem nevez meg, például hibás futtatható állományok és adathalász trükkök.
widgets-macros-common-way-deliver = A makrók gyakori módjai a kártevők terjesztésének, de sok legitim dokumentum is használja őket.
widgets-clamav-cannot-look-inside = A ClamAV nem lát beléjük, így kártevőt rejthetnek.
widgets-checks-files-inside-zip = Ellenőrzi a zip, rar és egyéb archívumokban lévő fájlokat.
widgets-otherwise-files-above-size = Ellenkező esetben a méret- és rekurziós korlátok feletti fájlok szó nélkül, ellenőrizetlenül átjutnak.
widgets-locked-until-unlocked = A beállítás módosításához oldja fel a zárolást.
widgets-administrator-not-allow-detection = A rendszergazdája nem engedélyezi ennek a beállításnak a módosítását a { -brand-name } alkalmazásból.
widgets-clamd-settings = clamd-beállítások
widgets-access-scanning-settings = Hozzáféréskori vizsgálat beállításai
widgets-update-settings = Frissítési beállítások
widgets-change-snippet-copied = Másolva. Az alkalmazáshoz adja hozzá a konfigurációjához.
widgets-stale-confirmation-dropped = A módosítás nem történt meg: a beállítások újra zárolódtak, vagy egy másik módosítás indult el előbb.
widgets-settings-applied = Alkalmazva, de még nem szerepel a rendszerkonfigurációjában.
widgets-previous-restored = A ClamAV ezzel az értékkel nem állt helyre megfelelően, ezért az előző beállítások visszaállítva.
widgets-change-not-made-error = A módosítás nem történt meg: { $error }
widgets-authorization-refused = A jelszava nélkül semmi sem változott
widgets-admin-service-missing = Ebben a telepítésben nincs ClamAV adminisztrációs szolgáltatás, ezért a ClamAV beállításai nem módosíthatók a { -brand-name } alkalmazásból
widgets-live-tunable-config = A(z) { $path } a rendszerkonfigurációját egyesíti az itt végzett módosításokkal. Zárolt állapotban a beállítások csak olvashatók: a Konfigurációs sor másolása funkcióval másolhat egyet a konfigurációjához. A feloldás után az engedélyezett beállítások közvetlenül módosulnak.
widgets-change-setting = Módosítja ezt a beállítást?
widgets-cancel-engine = Mégse
widgets-reduce-protection = Védelem csökkentése
widgets-apply-label = Alkalmaz
widgets-key-value-until-time = { $key } { $value }, eddig: { $time }
widgets-live-tuning-not-installed = Az élő hangolás nincs telepítve. NixOS rendszeren engedélyezze a services.lazaret.liveTuning beállítást; Debian és Ubuntu rendszeren telepítse a { -brand-name } csomagot, amely tartalmazza a ClamAV adminisztrációs szolgáltatást.
widgets-admin-service-unreachable = A beállítások jelenleg nem módosíthatók itt: a { -brand-name } ClamAV adminisztrációs szolgáltatása nem érhető el.
widgets-loading-config = A konfiguráció olvasása…
widgets-config-missing = A konfigurációs fájl nem található
widgets-lock-label = Zárolás
widgets-unlock-label = Feloldás
widgets-change-settings-title = Beállítások módosítása
widgets-unlock-change-settings-description = Oldja fel a zárolást a rendszere által futásidőben engedélyezett beállítások módosításához. A feloldás önmagában semmit sem engedélyez: minden módosítás megmutatja, mi fog történni, és rendszergazdai jogosultságot kér. Két perc után újra zárolódik.
widgets-changes-system-configuration-title = Módosítások, amelyekről a rendszerkonfigurációja nem tud
widgets-copy-engine-tooltip = Másolás
widgets-reduced-protection-ends-title = Magától megszűnő csökkentett védelem
widgets-reset-system-configuration-title = Visszaállítás a rendszerkonfigurációra
widgets-detection-title = Észlelés
widgets-clamav-reports-every-description = Amit a ClamAV ezen a számítógépen minden vizsgálatban jelent. A { -brand-name } ugyanezeket a választásokat adja át a clamscan programnak, így mindkét vizsgálóprogram egyezik.
widgets-no-settings-section-title = Ebben a szakaszban nincsenek beállítások
widgets-journal-read-failed = A rendszernapló nem olvasható: { $error }
widgets-journal-reading-incomplete = A rendszernapló olvasása befejezés előtt leállt
widgets-folder-unreadable = Az aláírásmappa nem olvasható: { $error }
widgets-reading-signature-folder-stopped = Az aláírásmappa olvasása befejezés előtt leállt
widgets-entry-timestamp-invalid = Egy naplóbejegyzés időbélyege kívül esik a támogatott tartományon.
widgets-reader-thread-failed = A naplóolvasót nem sikerült elindítani: { $error }
widgets-journal-reading-stopped = A rendszernapló olvasása leállt. { $reason }
widgets-file-hash-mismatch = A fájl kivonata nem egyezik — a fájl sérült lehet
widgets-restore-refused-integrity-unknown = Ennek a karantén-bejegyzésnek nincs használható kivonata, így a fájl nem ellenőrizhető, és nem lesz visszaállítva. Törölni viszont lehet.
widgets-upload-refused-integrity-unknown = Ennek a karantén-bejegyzésnek nincs használható kivonata, így a { -brand-name } nem tudja igazolni, hogy a fájl azonos a karanténba helyezettel, és sehová sem küldi el.
widgets-payload-too-large-send = A fájl nagyobb, mint { $size }, ami a { -brand-name } által elküldhető legnagyobb méret.
widgets-blocked-file-other-filesystem = A hozzáféréskori vizsgálat blokkolja ezt a fájlt, és az egy másik fájlrendszeren van, így nem helyezhető karanténba. Helyette törölheti.
widgets-entry-not-found = A karantén-bejegyzés nem található: { $id }
widgets-recording-failed-file-restored = Nem sikerült rögzíteni a karantén-bejegyzést az adatbázisban. A fájl visszaállítva az eredeti helyére.
widgets-recording-failed-rollback = Nem sikerült rögzíteni a karantén-bejegyzést az adatbázisban. A visszaállítás is sikertelen volt: { $error }. A fájl elárvult állapotban lehet itt: { $path }
widgets-entry-record-not-removed = A fájl áthelyezése megtörtént, de a karantén-bejegyzését nem sikerült eltávolítani. Még megjelenhet a listában.
widgets-system-call-failed = Egy rendszerhívás sikertelen: { $error }
widgets-file-operation-failed = Egy fájlművelet sikertelen: { $error }
widgets-database-operation-failed = A karanténadatbázis-művelet sikertelen: { $error }
widgets-blocking-step-interrupted = Egy háttérben futó karanténlépés váratlanul leállt: { $error }
widgets-path-contains-nul = Az elérési út NUL bájtot tartalmaz
widgets-path-has-no-parent = Az elérési útnak nincs szülőmappája
widgets-path-has-no-file = Az elérési útnak nincs fájlneve
widgets-database-path-not-utf8 = A karanténadatbázis elérési útja nem érvényes UTF-8
widgets-database-not-regular-file = A karanténadatbázis nem szabályos fájl
widgets-source-blocked-access = A hozzáféréskori vizsgálat blokkolja a fájl olvasását
widgets-moved-payload-still-blocked = A blokkolt fájl a karanténba helyezés után is olvashatatlan
widgets-source-not-regular-file = A fájl nem szabályos fájl
widgets-payload-not-regular-file = A karanténba helyezett fájl nem szabályos fájl
widgets-not-enough-free-space = Nincs elég szabad hely a fájl karanténba helyezéséhez
widgets-file-replaced-during-operation = A fájlt áthelyezés közben lecserélték, ezért nem lett eltávolítva
widgets-path-outside-quarantine = Az elérési út nem a karanténmappán belül van
widgets-invalid-restore-path = A visszaállítási útvonal érvénytelen
widgets-protected-restore-destination = Védett rendszermappába történő visszaállítás nem engedélyezett
widgets-stored-path-unreadable = A karanténadatbázisban tárolt egyik elérési út nem olvasható
widgets-target-missing = A vizsgálati cél nem létezik, vagy érvénytelen NUL bájtot tartalmaz
widgets-target-not-scannable = A vizsgálati cél nem szabályos fájl vagy könyvtár
widgets-target-directory-unreadable = A(z) { $path } vizsgálati cél tartalma nem listázható: { $error }
widgets-file-list-unwritable = A privát clamd-fájllista nem írható: { $error }
widgets-enumeration-interrupted = A vizsgálati cél listázása váratlanul leállt.
widgets-no-files-scanned = Egyetlen fájl sem volt megvizsgálható
widgets-unscanned-files =
    { $count ->
        [one] { $count } fájl nem lett megvizsgálva
       *[other] { $count } fájl nem lett megvizsgálva
    }
widgets-clamd-connection-lost = A kapcsolat a clamd szolgáltatással megszakadt a vizsgálat közben.
widgets-scanner-stopped-reporting = A vizsgálóprogram nem jelentett eredményt ezekről a fájlokról.
widgets-count-not-accessed-title =
    { $count ->
        [one] { $count } fájl nem volt elérhető
       *[other] { $count } fájl nem volt elérhető
    }
widgets-files-unreadable =
    { $count ->
        [one] { $count } fájl nem volt olvasható
       *[other] { $count } fájl nem volt olvasható
    }
widgets-nonfatal-warnings =
    { $count ->
        [one] { $count } nem végzetes figyelmeztetés történt a vizsgálat során; egyes fájlok csak részben lettek megvizsgálva
       *[other] { $count } nem végzetes figyelmeztetés történt a vizsgálat során; egyes fájlok csak részben lettek megvizsgálva
    }
widgets-count-threat-found-outcome =
    { $count ->
        [one] { $count } fenyegetés található
       *[other] { $count } fenyegetés található
    }
widgets-count-file-scanned-outcome =
    { $count ->
        [one] { $count } fájl ellenőrizve
       *[other] { $count } fájl ellenőrizve
    }
widgets-clamav-not-open-files = A ClamAV nem tudta megnyitni ezeket a fájlokat.
widgets-unattributed-threats-only = A vizsgálóprogram fenyegetést jelentett a fájl megnevezése nélkül.
widgets-preparing-scan = Előkészítés…
widgets-finding-files =
    { $count ->
        [one] Fájlok keresése… { $count } találat
       *[other] Fájlok keresése… { $count } találat
    }
widgets-trust-file = Fájl megjelölése megbízhatóként…
widgets-ignore-detection-for-me = Észlelés figyelmen kívül hagyása számomra…
widgets-report-false-positive = Téves riasztás jelentése…
widgets-skip-path-lazaret-scans = Útvonal kihagyása a { -brand-name } vizsgálataiból
widgets-copy-path = Elérési út másolása
widgets-critical-severity = Kritikus
widgets-high-severity = Magas
widgets-medium-severity = Közepes
widgets-low-severity = Alacsony
widgets-moved-quarantine-path-check = Karanténba helyezve: { $path }. A hírnevét a Karantén oldalon ellenőrizheti.
widgets-quarantine-detected-label = Karanténba helyezés
widgets-more-actions-tooltip = További műveletek
widgets-count-more-file-title =
    { $count ->
        [one] További { $count } fájl, az exportált jelentésben felsorolva
       *[other] További { $count } fájl, az exportált jelentésben felsorolva
    }
widgets-scan-details-title = Vizsgálat részletei
widgets-quarantine-all =
    { $count ->
        [one] { $count } fenyegetés karanténba helyezése
       *[other] { $count } fenyegetés karanténba helyezése
    }
widgets-move-quarantine = Karanténba helyezi?
widgets-removes-each-original-file = Ezzel minden eredeti fájl eltávolításra kerül a jelenlegi helyéről.
widgets-cancel-scan = Mégse
widgets-quarantine-label = Karanténba helyezés
widgets-detected-threats-title = Észlelt fenyegetések
widgets-problems-title = Problémák
widgets-move-path-quarantine = Áthelyezi a(z) „{ $path }” fájlt karanténba?
widgets-move-count-detected-file =
    { $count ->
        [one] Áthelyezi { $count } észlelt fájlt karanténba?
       *[other] Áthelyezi { $count } észlelt fájlt karanténba?
    }
widgets-path-copied-clipboard = Az elérési út a vágólapra másolva
widgets-file-skipped-future-scans = Ez a fájl a jövőbeli vizsgálatokból kimarad
widgets-exclusion-already-exists = Ez a kizárás már létezik
widgets-results-copied-clipboard = Az eredmények a vágólapra másolva
widgets-results-exported = Eredmények exportálva
widgets-done-total-file-scanned =
    { $total ->
        [one] { NUMBER($done) } / { $total } fájl megvizsgálva
       *[other] { NUMBER($done) } / { $total } fájl megvizsgálva
    }
widgets-count-file-scanned-results =
    { $count ->
        [one] { $count } fájl ellenőrizve
       *[other] { $count } fájl ellenőrizve
    }
widgets-count-entry-skipped-exclusions =
    { $count ->
        [one] { $count } bejegyzés kihagyva a kizárások miatt
       *[other] { $count } bejegyzés kihagyva a kizárások miatt
    }
widgets-threats-detected-status = Fenyegetés észlelve
widgets-no-threats-detected-status = Nem található fenyegetés
widgets-finished-problems-status = A vizsgálat nem fejeződött be hibátlanul
widgets-scan-cancelled-status = A felhasználó megszakította a vizsgálatot
widgets-count-threat-found-results =
    { $count ->
        [one] { $count } fenyegetés található
       *[other] { $count } fenyegetés található
    }
widgets-files-scanned = Vizsgált fájlok
widgets-files-planned = Tervezett fájlok
widgets-directories-label = Mappák
widgets-threats-found = Talált fenyegetések
widgets-file-no-longer-quarantine = A fájl már nincs ott. Ha karanténban van, a Karantén oldalon jelölje megbízhatónak.
widgets-permission-denied = A fájl nem olvasható, valószínűleg azért, mert a hozzáféréskori vizsgálat blokkolja. Helyezze karanténba, majd a Karantén oldalon jelölje megbízhatónak.
widgets-file-not-read-scan = A fájl nem olvasható: { $error }
widgets-threats-detected-finished = Fenyegetés észlelve
widgets-no-threats-detected-finished = Nem található fenyegetés
widgets-finished-problems-notify = A vizsgálat nem fejeződött be hibátlanul
widgets-scanner-error = A vizsgálóprogram { $code } hibakóddal leállt.
widgets-killed-signal = A vizsgálóprogramot a(z) { $signal } jelzés állította le.
widgets-unrecognized-wait-status = A vizsgálóprogram olyan módon fejeződött be, amelyet a { -brand-name } nem ismer fel (várakozási állapot: { $status }).
widgets-wait-failed = A { -brand-name } elvesztette a vizsgálófolyamat nyomát: { $error }.
widgets-scanner-program-missing = A ClamAV vizsgálóprogram nincs telepítve, vagy nincs a keresési útvonalon.
widgets-scanner-program-replaceable = A { -brand-name } megtagadta a(z) { $program } futtatását, mert a fiókja lecserélheti ezt: { $replaceable }. Telepítse a ClamAV-t rendszerszinten, hogy egy kártevő által elhelyezett program ne hamisíthasson tiszta eredményeket.
widgets-scanner-spawn-failed = A vizsgálóprogramot nem sikerült elindítani: { $error }
widgets-scanner-work-directory-unavailable = Nem sikerült létrehozni a vizsgálóprogram privát munkakönyvtárát: { $error }
widgets-scanner-output-uncaptured = A vizsgálóprogram kimenetét nem sikerült rögzíteni.
widgets-detection-settings-unreadable = A vizsgálat nem indult el, mert a clamd észlelési beállításai nem olvashatók: { $error }
widgets-program-lookup-failed = A vizsgálat nem indult el, mert a vizsgálóprogramokat nem sikerült megkeresni: { $error }
widgets-scan-input-unavailable = A(z) { $path } vizsgálati bemenet nem nyitható meg biztonságosan: { $error }
widgets-detections-warnings-errors-tooltip = Csak észlelések, figyelmeztetések és hibák
widgets-copy-service-tooltip = Másolás
widgets-export-text-tooltip = Szöveg exportálása
widgets-search-log-placeholder = Keresés a naplóban
widgets-reading-log = A napló olvasása…
widgets-no-log-lines-match = Egyetlen naplósor sem egyezik
widgets-log-copied-clipboard = A napló a vágólapra másolva
widgets-text-export-finished = Exportálva.
widgets-threat-detected = Észlelés
widgets-emergency-label = Vészhelyzet
widgets-alert-label = Riasztás
widgets-critical-level = Kritikus
widgets-error-level = Hiba
widgets-warning-label = Figyelmeztetés
widgets-notice-label = Megjegyzés
widgets-info-label = Információ
widgets-debug-label = Hibakeresés
widgets-log-reading = Ellenőrzés…
widgets-system-journal = Rendszernapló
widgets-neither-log-file-nor = Sem naplófájl, sem naplósorok nem találhatók.
widgets-no-log-file-found = Nem található naplófájl. { $reason }
widgets-program-use = Használatban: { $program }
widgets-ignored-program = Figyelmen kívül hagyva: { $program }
widgets-refused-program = Elutasítva: { $program }
widgets-scanner-programs-trusted = A vizsgálóprogramok megbízhatók
widgets-freshclam-trusted = A freshclam megbízható
widgets-malware-running-clamdscan = Egy az Ön nevében futó kártevő hamis clamdscan programot helyezhet a keresési útvonalán előrébb, amely minden fájlt tisztának jelent. A { -brand-name } csak olyan ClamAV-programokat futtat, amelyeket a fiókja nem cserélhet le, például a teljes rendszerre telepítetteket.
widgets-malware-running-freshclam = Egy az Ön nevében futó kártevő hamis freshclam programot helyezhet a keresési útvonalán előrébb, amely csak úgy tesz, mintha új aláírásokat töltene le. A { -brand-name } csak olyan ClamAV-programokat futtat, amelyeket a fiókja nem cserélhet le, például a teljes rendszerre telepítetteket.
widgets-check-interrupted = Ismeretlen, mert a ClamAV-programok ellenőrzése befejezés előtt leállt
widgets-clamav-program-replaced-account = Egy ClamAV-programot a fiókja lecserélhet, ezért a { -brand-name } megtagadja a futtatását
widgets-clamav-program-account-shadows = A fiókjában lévő egyik ClamAV-program elfedi a rendszerpéldányt. A { -brand-name } figyelmen kívül hagyja, de ellenőrizze, honnan származik
widgets-count-settings-reduced-since =
    { $count ->
        [one] { $count } beállítás a konfiguráltnál kevesebbet ellenőriz ekkor óta: { $time }
       *[other] { $count } beállítás a konfiguráltnál kevesebbet ellenőriz ekkor óta: { $time }
    }
widgets-count-settings-reduced-unknown-time =
    { $count ->
        [one] { $count } beállítás a konfiguráltnál kevesebbet ellenőriz, egy nem olvasható időpont óta
       *[other] { $count } beállítás a konfiguráltnál kevesebbet ellenőriz, egy nem olvasható időpont óta
    }
widgets-changed-label = Módosítva
widgets-change-time-unknown-override = Ismeretlen
widgets-reduction-unknown-unread = Nem tudható, csökkentették-e a beállításokat, mert a futásidejű módosítások nem olvashatók: { $error }
widgets-reduction-unknown-file = Nem tudható, csökkentették-e a beállításokat, mert a futásidejű módosítások fájlja nem a várt formátumú. A ClamAV a javításáig a rendszerkonfigurációval fut.
widgets-reduction-unknown-reading = Nem tudható, csökkentették-e a beállításokat, mert a futásidejű módosítások olvasása befejezés előtt leállt
widgets-settings-changed-runtime = Futásidőben módosított beállítások
widgets-administrator-can-change = A rendszergazdai jelszó birtokában bárki módosíthatja futásidőben a ClamAV beállításait. A { -brand-name } nem tudta olvasni ezeket a módosításokat, így nem tudja megállapítani, hogy kevesebbet ellenőriznek-e, mint a konfigurációja.
widgets-protection-reduced-purpose = Szándékosan csökkentett védelem
widgets-administrator-changed-runtime = Valaki a rendszergazdai jelszóval futásidőben úgy módosította a ClamAV beállításait, hogy kevesebbet ellenőriznek, mint a konfigurációja. A beállítást tartalmazó oldal rendszermódosításai mutatják, ki és mikor.
widgets-base-configuration-changed-now = A konfigurációjában { $base }, most { $changed }
widgets-administrator-changed-clamd = Valaki a rendszergazdai jelszóval futásidőben módosította a ClamAV beállításait. A clamd.conf nem olvasható, így a { -brand-name } nem tudja megállapítani, hogy ezek a módosítások kevesebbet ellenőriznek-e, mint a konfigurációja. A beállítást tartalmazó oldal rendszermódosításai mutatják, ki és mikor.
widgets-settings-changed-runtime-clamd = A beállítások futásidőben módosultak, de a clamd.conf nem olvasható az összehasonlításhoz
widgets-changed-now = Most { $changed }
widgets-daemon-awaited = A démon szükséges hozzá, lásd: Vizsgálómotor
widgets-unknown-run = Ismeretlen. { $reason }
widgets-copy-timeline-tooltip = Másolás
widgets-moment-details = { $moment } · { $details }
widgets-why-matters-tooltip = Miért fontos ez
widgets-what-is-sent-label = Ami elküldésre kerül:
widgets-clamav-audience = A Cisco Talos, a ClamAV mögött álló csapat vizsgálja felül. A mintákat automatikusan dolgozzák fel, csak a Talos férhet hozzájuk, és sosem teszik nyilvánossá vagy osztják meg más cégekkel. A felülvizsgálat legalább 48 órát vesz igénybe, és az eredménye egy későbbi aláírás-adatbázis-verzióban jelenik meg.
widgets-report-false-positive-clamav = Téves riasztást jelent a ClamAV számára?
widgets-file-content = A fájl tartalma
widgets-detection-name = Az észlelés neve: { $detection }
widgets-report-missed-malware-clamav = Kimaradt kártevőt jelent a ClamAV számára?
widgets-name-email-address-talos = Az Ön neve és e-mail-címe, hogy a Talos kapcsolatba léphessen Önnel
widgets-lazaret-sends-file-clamav = A { -brand-name } a clamsubmit segítségével elküldi ezt a fájlt a ClamAV számára. Csak olyan fájlokat küldjön, amelyek semmi személyeset nem tartalmaznak.
widgets-send-report-button = Jelentés küldése
widgets-file-content-named-sha = A fájl tartalma, a SHA-256 kivonatával megnevezve, nem a nevével
widgets-upload-malwarebazaar-heading = Feltölti a MalwareBazaar szolgáltatásba?
widgets-lazaret-uploads-file-malwarebazaar = A { -brand-name } feltölti ezt a fájlt az abuse.ch által üzemeltetett MalwareBazaar szolgáltatásba.
widgets-tag-clamav-detection = A ClamAV észleléséből származó címke: { $tag }
widgets-abuse-ch-key-sample = Az Ön abuse.ch-kulcsa; a minta a fióknév nélkül kerül listázásra
widgets-malwarebazaar-public-anyone-download = A MalwareBazaar nyilvános: feltöltés után bárki letöltheti a mintát, és a feltöltés nem vonható vissza. Csak kártevőt töltsön fel, sosem olyan dokumentumot, amely személyes adatokat tartalmazhat.
widgets-upload-publicly-button = Nyilvános feltöltés
widgets-upload-virustotal-heading = Feltölti a VirusTotal szolgáltatásba?
widgets-lazaret-uploads-file-virustotal = A { -brand-name } elemzésre feltölti ezt a fájlt a VirusTotal szolgáltatásba.
widgets-virustotal-key-links-upload = Az Ön VirusTotal-kulcsa, amely a feltöltést a fiókjához kapcsolja
widgets-virustotal-shares-uploaded-files = A VirusTotal megosztja a feltöltött fájlokat az előfizető ügyfeleivel, akik letölthetik őket, és bárki megnézheti az eredményt a fájl kivonata alapján. Sosem töltsön fel olyan dokumentumot, amely személyes adatokat tartalmazhat.
widgets-upload-button = Feltöltés
widgets-cancel-submission = Mégse
widgets-name-title = Az Ön neve
widgets-email-address-title = Az Ön e-mail-címe
widgets-file-unreadable = A fájl nem olvasható: { $error }
widgets-file-not-regular = Csak szabályos fájlok küldhetők
widgets-file-too-large = A fájl nagyobb, mint { $size }, ami a { -brand-name } által elküldhető legnagyobb méret.
widgets-reading-interrupted = A fájl olvasása váratlanul leállt
widgets-history-failure = A beküldési előzmények nem olvashatók: { $error }
widgets-earlier-submission = Ezt a fájlt már elküldték ekkor: { $local-time }. Semmi sem lett újra elküldve.
widgets-report-recorded = Jelentés elküldve. A ClamAV felülvizsgálata legalább 48 órát vesz igénybe; az Elküldött minták oldal mutatja, ha egy aláírásfrissítés megváltoztatja az ítéletet.
widgets-report-not-sent-error = A jelentés nem lett elküldve: { $error }
widgets-report-sent-without-record = Jelentés elküldve, de nem sikerült felvenni az Elküldött minták közé: { $error }
widgets-nothing-uploaded-error = Nem történt feltöltés. { $reason }
widgets-bazaar-key-not-usable = A MalwareBazaarhoz nincs használható kulcs.
widgets-virustotal-key-not-usable = A VirusTotalhoz nincs használható kulcs.
widgets-malwarebazaar-refused-sample-reason = A MalwareBazaar elutasította a mintát: { $reason }
widgets-malwarebazaar-sample-sent-samples = A minta a MalwareBazaar szolgáltatásban van. Az Elküldött minták oldal hivatkozik a nyilvános bejegyzésére.
widgets-not-detected-clamav = A MalwareBazaar csak olyan fájlokat fogad, amelyeket egy ClamAV-aláírás észlelt
widgets-unwanted-program-only = A MalwareBazaar nem fogad esetlegesen nemkívánatos programokat, csak kártevőket
widgets-sent-without-record = Feltöltve, de nem sikerült felvenni az Elküldött minták közé: { $error }
widgets-virustotal-refused-file-reason = A VirusTotal elutasította a fájlt: { $reason }
widgets-virustotal-analysing-file-refresh = A VirusTotal elemzi a fájlt. Az Elküldött minták oldalon a Frissítés megmutatja az ítéletét.
widgets-report-already-available =
    { $total ->
        [one] A VirusTotal már ismeri ezt a fájlt: { $total } motorból { NUMBER($count) } észleli. Nem történt feltöltés.
       *[other] A VirusTotal már ismeri ezt a fájlt: { $total } motorból { NUMBER($count) } észleli. Nem történt feltöltés.
    }
widgets-known-user = { $local-time } · { $outcome } · kérte: { $uid } azonosítójú felhasználó
widgets-unknown-user = { $time } · { $outcome } · kérte: ismeretlen felhasználó
widgets-change-lowered-protection-tooltip = Ez a módosítás csökkentette a védelmet
widgets-system-changes-failed = A rendszernapló nem olvasható: { $error }
widgets-system-changes-title = Rendszermódosítások
widgets-refresh-tooltip = Frissítés
widgets-system-changes-cannot-title = A rendszermódosítások nem olvashatók
widgets-loading-system-changes = Rendszermódosítások betöltése…
widgets-trust-file-heading = Megbízhatónak jelöli ezt a fájlt?
widgets-clamav-never-report-body =
    A ClamAV soha többé nem jelent pontosan ilyen tartalmú fájlt egyetlen felhasználónak sem, bárhol is legyen ezen a számítógépen, és bármelyik vizsgálat is találja meg, a hozzáféréskori vizsgálat pedig nem blokkolja többé. Ehhez a rendszergazdai jelszó szükséges, és a jelszókérő ablak ugyanazt a SHA-256 kivonatot mutatja, mint alább.

    { $path }
widgets-stop-showing-file-body =
    A { -brand-name } a jövőben nem mutat Önnek pontosan ilyen tartalmú fájlt az Ön által futtatott vizsgálatokban, bárhol is legyen. Maga a ClamAV továbbra is észleli, így a hozzáféréskori vizsgálat továbbra is blokkolhatja és riaszthat, más felhasználók pedig továbbra is látják. A Megbízható minden felhasználónak lehetőség mindenki számára leállítja az észlelését a ClamAV-ban, és ehhez a rendszergazdai jelszó szükséges.

    { $path }
widgets-trust-for-me-button = Megbízható számomra
widgets-user-trust-unreadable = Az Ön által megbízhatónak jelölt fájlok és észlelések nem olvashatók, ezért ez a vizsgálat minden észlelést megjelenít: { $error }
widgets-trust-for-all-users-button = Megbízható minden felhasználónak…
widgets-sha-256-hash-size =
    SHA-256 { $hash }
    { $size }
widgets-ignore-detection-heading = Figyelmen kívül hagyja ezt az észlelést?
widgets-stop-showing-detection-body = A { -brand-name } a jövőben egyetlen ilyen észlelésű fájlt sem mutat Önnek, nem csak ezt az egyet. Maga a ClamAV és a többi felhasználó továbbra is látja őket. Ha csak egyetlen fájl téves riasztás, biztonságosabb a fájlt a kivonata alapján megbízhatónak jelölni.
widgets-ignore-button = Figyelmen kívül hagyás
widgets-stop-trusting-file-heading = Visszavonja a fájl megbízhatóságát?
widgets-clamav-report-file-body = A ClamAV ismét jelenteni fogja ezt a fájlt minden felhasználónak, ha egy aláírásra illeszkedik.
widgets-show-file-again-body = A { -brand-name } ismét megmutatja Önnek ezt a fájlt, ha a ClamAV észleli.
widgets-stop-trusting-button = Megbízhatóság visszavonása
widgets-report-detection-again-heading = Ismét jelenti ezt az észlelést?
widgets-clamav-report-files-body = A ClamAV ismét jelenteni fogja az ilyen észlelésű fájlokat minden felhasználónak.
widgets-show-detection-again-body = A { -brand-name } ismét megmutatja Önnek az ilyen észlelésű fájlokat.
widgets-report-again-button = Ismételt jelentés
widgets-cancel-trust = Mégse

host-action-applied = Alkalmazva
host-action-rolled-back = Visszavonva
host-authorization-refused-described = Jelszó nélkül elutasítva
host-action-failed = Sikertelen
host-outcome-not-recorded = Az eredmény nincs rögzítve
host-unrecognized-outcome = Ismeretlen eredmény: { $outcome }
host-blocking-turned-off = Az észlelt fájlok megnyitáskor többé nem lesznek blokkolva. Csak jelentésre kerülnek.
host-check-turned-off = A ClamAV ezen a számítógépen nem ellenőrzi többé ezt a fajta fájlt vagy fenyegetést.
host-watched-folder-removed = Az eltávolított mappában lévő fájlok megnyitáskor többé nem lesznek ellenőrizve.
host-skipped-location-added = A ClamAV teljesen kihagyja az új bejegyzést, így az ott lévő kártevők észrevétlenek maradnak.
host-scan-limit-lowered = Az új korlát feletti fájlok vagy archívumok ellenőrzés nélkül kimaradnak.
host-current-values-unknown = A jelenlegi érték nem olvasható, így ez a módosítás csökkentheti a védelmet.
host-detection-trusted = A ClamAV többé nem jelenti azt, amit itt megbízhatónak jelöl.
host-key-now-current-new =
    { $key }
    Most: { $current }
    Új: { $new }
host-system-records-change-reduced = A rendszer rögzíti, hogy ez a módosítás csökkentette a védelmet, és az oldalsáv a visszavonásig megjelölve tartja a módosított beállítás oldalát.
host-trust-applied = A ClamAV betöltötte a módosítást
host-trust-recorded-for-you = A { -brand-name } ezt többé nem mutatja Önnek. Maga a ClamAV, a hozzáféréskori blokkolás és a többi felhasználó továbbra is látja.
host-applied-reload-pending = Mentve. A clamd nem válaszolt, ezért az aláírásai következő újratöltésekor veszi át a módosítást
host-authorization-refused-outcome = A jelszava nélkül semmi sem változott
host-admin-service-missing = Ebben a telepítésben nincs ClamAV adminisztrációs szolgáltatás, ezért a ClamAV megbízhatósági listája nem módosítható a { -brand-name } alkalmazásból
host-trust-failed = Semmi sem változott: { $error }
host-nixos-configuration = A(z) { $path } fájlt a NixOS-konfigurációja állítja elő, ezért a { -brand-name } nem tudja módosítani. Egy beállítás Konfigurációs sor másolása funkciójával másolja a sorát a konfigurációjához, majd építse újra a rendszert.
host-system-package = A(z) { $path } a rendszerhez tartozik, ezért a { -brand-name } nem tudja módosítani. Egy beállítás Konfigurációs sor másolása funkciójával másolja ki a fájlba illesztendő sort.
host-contact-incomplete = A jelentés elküldéséhez adja meg a nevét és egy érvényes e-mail-címet
host-submitter-missing = A jelentéshez a clamsubmit szükséges, amely a ClamAV része, de itt nincs telepítve
host-submitter-replaceable = A { -brand-name } megtagadta a clamsubmit futtatását, mert a fiókja lecserélheti
host-report-not-sent-error = A jelentés nem lett elküldve: { $error }
host-upload-timed-out = A jelentés nem lett elküldve: a feltöltés túl sokáig tartott
host-unrecognized-source = Egyéb forrás
host-file-progress =
    { $total ->
        [one] { NUMBER($done) } / { $total } fájl
       *[other] { NUMBER($done) } / { $total } fájl
    }
host-count-second-left =
    { $count ->
        [one] Körülbelül { $count } másodperc van hátra
       *[other] Körülbelül { $count } másodperc van hátra
    }
host-count-minute-left =
    { $count ->
        [one] Körülbelül { $count } perc van hátra
       *[other] Körülbelül { $count } perc van hátra
    }
host-count-hour-left =
    { $count ->
        [one] Körülbelül { $count } óra van hátra
       *[other] Körülbelül { $count } óra van hátra
    }
host-seconds-s = { $seconds } mp
host-minutes-min-seconds-s = { $minutes } perc { $seconds } mp
host-hours-h-minutes-min = { $hours } óra { $minutes } perc
host-blocked-access = A hozzáféréskori vizsgálat blokkolta
host-exact-byte-size =
    { $count ->
        [one] { $size } ({ $count } bájt)
       *[other] { $size } ({ $count } bájt)
    }
host-hash-unknown = Nem szerepel az ismert fájlok adatbázisában
host-listed-malicious-source-name = Kártékonyként listázza: { $source } ({ $name } néven)
host-listed-malicious-unnamed-source-name = Egy meg nem nevezett forrás kártékonyként listázza ({ $name } néven)
host-listed-malicious-source = Kártékonyként listázza: { $source }
host-listed-malicious-unnamed-source = Egy meg nem nevezett forrás kártékonyként listázza
host-listed-known-file-source = Ismert fájlként listázza: { $source }
host-listed-known-file-unnamed-source = Egy meg nem nevezett forrás ismert fájlként listázza
host-known-file-name-listed = Ismert fájl: { $name }, listázza: { $source }
host-known-file-name-unnamed-source = Ismert fájl: { $name }, egy meg nem nevezett forrás listázza
host-unnamed-family = névtelen család
host-no-known-samples = Nem ismert kártevőminta
host-known-malware-sample = Ismert kártevőminta
host-samples-found = Ismert kártevő: { $family }, először látva: { $date }
host-samples-found-date-unknown = Ismert kártevő: { $family }, először ismeretlen időpontban látva
host-unnamed-family-samples-found = Névtelen családba tartozó ismert kártevő, először látva: { $date }
host-unnamed-family-date-unknown = Névtelen családba tartozó ismert kártevő, először ismeretlen időpontban látva
host-file-unknown-virustotal = A VirusTotal számára ismeretlen
host-count-total-engine-verdict =
    { $total ->
        [one] { $total } motorból { NUMBER($count) } észleli
       *[other] { $total } motorból { NUMBER($count) } észleli
    }
host-api-key-rejected = A VirusTotal elutasította az API-kulcsot.
host-request-limit-reached = Elérte a VirusTotal kéréskorlátját. Próbálja újra később.
host-virustotal-answered-status = A VirusTotal válaszának állapota: { $status }
host-virustotal-not-reached-error = A VirusTotal nem érhető el: { $error }
host-virustotal-sent-answer-lazaret = A VirusTotal olyan választ küldött, amelyet a { -brand-name } nem tud olvasni
host-request-failed-circl = A CIRCL hashlookup nem érhető el: { $error }
host-answer-unreadable-circl = A CIRCL hashlookup olvashatatlan választ küldött
host-unexpected-status-circl = A CIRCL hashlookup válaszának állapota: { $status }
host-request-failed-bazaar = A MalwareBazaar nem érhető el: { $error }
host-answer-unreadable-bazaar = A MalwareBazaar olvashatatlan választ küldött
host-request-limit-reached-bazaar = Elérte a MalwareBazaar kéréskorlátját. Próbálja újra később.
host-unexpected-status-bazaar = A MalwareBazaar válaszának állapota: { $status }
host-key-refused = A MalwareBazaar elutasította az API-kulcsot
host-query-refused = A MalwareBazaar válasza: { $status }
headless-count-threat-found =
    { $count ->
        [one] { $count } fenyegetés található
       *[other] { $count } fenyegetés található
    }
headless-open-lazaret-review = Az áttekintésükhöz nyissa meg a { -brand-name } alkalmazást.
headless-count-threats-quarantined =
    { $count ->
        [one] { $count } karanténba helyezve.
       *[other] { $count } karanténba helyezve.
    }
headless-scan-infected = Fenyegetés észlelve
headless-scheduled-scan-failed = Az ütemezett vizsgálat sikertelen
headless-open-lazaret-read-log = A napló megtekintéséhez nyissa meg a { -brand-name } alkalmazást.
