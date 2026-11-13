-brand-name = Lazaret
startup-settings-schema-missing = O esquema de configurações do { -brand-name } não está instalado. Execute make install ou defina GSETTINGS_SCHEMA_DIR.
startup-unknown-page-name = O { -brand-name } não tem nenhuma página chamada { $page }, então abre a primeira área que precisa de atenção. Páginas: { $pages }

window-signature-sources = Fontes de assinaturas
window-database-updates = Atualizações do banco de dados
window-scan-engine = Mecanismo de verificação
window-access-scanning = Verificação no acesso
window-scheduled-scans-page = Verificações agendadas
window-manual-scan = Verificação manual
window-removable-drives = Unidades removíveis
window-detections-label = Detecções
window-quarantine-label = Quarentena
window-trusted-detections = Detecções confiáveis
window-sent-samples = Amostras enviadas
window-quarantine-database-not-locked = Não foi possível bloquear o banco de dados da quarentena
window-quarantine-database-not-opened = Não foi possível abrir o banco de dados da quarentena
window-file-operation-failed-error = Uma operação de arquivo falhou: { $error }
window-lock-task-interrupted = Uma etapa da quarentena em segundo plano parou inesperadamente: { $error }
window-translator-credits = translator-credits
window-export-history-menu-item = Exportar histórico…
window-clear-history-menu-item = Limpar histórico…
window-preferences-label = Preferências
window-about-menu-item = Sobre o { -brand-name }
window-quit-menu-item = Sair
window-export-history-heading = Exportar histórico?
window-export-history-body = Salva todas as verificações e atualizações de assinaturas registradas, com seus detalhes.
window-cancel-history-button = Cancelar
window-json-label = JSON
window-csv-label = CSV
window-clear-history-heading = Limpar histórico?
window-clear-history-body = Isso exclui permanentemente todas as verificações e atualizações de assinaturas registradas. Isso não pode ser desfeito.
window-clear-label = Limpar tudo
window-scanning-drive-title = Verificando { $drive }
window-open-removable-drives-button = Abrir unidades removíveis
window-not-export-history-toast = Não foi possível exportar o histórico: { $error }
window-history-export-finished = Histórico exportado
window-history-cleared = Histórico limpo
window-not-clear-history-toast = Não foi possível limpar o histórico: { $error }
window-other-sources-unknown = Outras fontes desconhecidas
window-source-out-date = Uma fonte está desatualizada
window-official-signatures = Somente assinaturas oficiais
window-official-count-other-source =
    { $count ->
        [one] Oficiais e { $count } outra fonte
       *[other] Oficiais e { $count } outras fontes
    }
window-database-missing = Nenhuma definição de vírus
window-age-definitions-unknown = Idade das definições desconhecida
window-last-update-failed = A última atualização falhou
window-definitions-dated-future = Definições com data no futuro
window-updated-label = Atualizado { $relative-time }
window-definitions-label = Definições de { $relative-time }
window-freshclam-refused = O freshclam foi recusado
window-unknown-whether-freshclam-trusted = Não se sabe se o freshclam é confiável
window-freshclam-copy-account-ignored = Uma cópia do freshclam na sua conta é ignorada
window-clamd-uses-older-signatures = O clamd usa assinaturas mais antigas
window-clamd-answering = O clamd está respondendo
window-unit-active-not-answering = O clamd não responde
window-unit-state-unknown-daemon = Estado do clamd desconhecido
window-unit-stopped-daemon = O clamd não está em execução
window-never-set-up-daemon = O clamd não está configurado
window-engine-missing = ClamAV não instalado
window-scanner-refused = O clamscan foi recusado
window-version-unverified = Versão do ClamAV desconhecida
window-scanner-program-refused = Um programa de verificação foi recusado
window-unknown-whether-scanners-trusted = Não se sabe se os verificadores são confiáveis
window-scanner-copy-account-ignored = Uma cópia do verificador na sua conta é ignorada
window-runtime-changes-unknown = Alterações em tempo de execução desconhecidas
window-settings-reduced-runtime = Configurações reduzidas em tempo de execução
window-waiting-daemon = Aguardando o clamd
window-unit-state-unknown-access = Estado desconhecido
window-never-set-up-access = Não configurado
window-unit-stopped-access = O clamonacc não está em execução
window-config-unreadable = clamd.conf ilegível
window-nothing-watched = Nada monitorado
window-checks-files-not-block = Verifica arquivos, não bloqueia
window-checks-files-kernel-cannot = Verifica arquivos, o kernel não pode bloquear
window-checks-files-blocking-unknown = Verifica arquivos, bloqueio desconhecido
window-blocking-paths = Bloqueando { $paths }
window-folder-watch-limit-unknown = Limite de monitoramento de pastas desconhecido
window-mount-watch-conflicts = Conflitos no monitoramento de montagens
window-clamd-user-not-excluded = O clamd não está excluído
window-whole-root-included = Monitora / diretamente
window-watch-limit-low = Limite de monitoramento de pastas baixo
window-timer-not-active = Temporizador inativo
window-last-scan-failed = A última verificação falhou
window-next-scan = Próxima verificação { $relative-time }
window-no-next-scan-planned = Nenhuma próxima verificação planejada
window-service-missing = Serviço de verificação ausente
window-service-unreadable = Serviço de verificação desconhecido
window-lazaret-schedule = Agendado pelo { -brand-name }
window-no-schedule = Nenhuma verificação agendada
window-schedule-changed-outside-lazaret = Agendamento alterado fora do { -brand-name }
window-ready-scan = Pronto para verificar
window-uses-slower-clamscan = Usa o clamscan, mais lento
window-exclusion-skip-malware = Uma exclusão pode deixar passar malware
window-device-scan-running = Verificando { $drive }
window-device-scan-ended = Ameaças encontradas em { $drive }
window-journal-unreadable = Journal ilegível
window-following-failed = O acompanhamento do journal parou
window-notifications-off = Notificações desativadas
window-alerts-stop-lazaret-quits = Os alertas param quando o { -brand-name } é encerrado
window-other-programs-attach-lazaret = Outros programas podem se conectar ao { -brand-name }
window-unknown-whether-programs-attach = Não se sabe se programas podem se conectar
window-nothing-decide = Nada a decidir
window-count-needs-decision =
    { $count ->
        [one] { $count } precisa de uma decisão
       *[other] { $count } precisam de uma decisão
    }
window-loading-label = Carregando…
window-quarantine-empty = Vazia
window-count-item =
    { $count ->
        [one] { $count } item
       *[other] { $count } itens
    }
window-quarantine-cannot-read = Não é possível ler a quarentena
window-nothing-trusted = Nada confiável
window-count-item-trusted =
    { $count ->
        [one] { $count } item confiável
       *[other] { $count } itens confiáveis
    }
window-signature-folder-unreadable = Pasta de assinaturas ilegível
window-another-allowlist-silences-clamav = Outra lista de permissões silencia o ClamAV
window-lazaret-allowlist-unreadable = A lista de permissões do { -brand-name } está ilegível
window-sent-samples-cannot-read = Não é possível ler as amostras enviadas
window-nothing-sent = Nada enviado
window-count-samples-refused =
    { $count ->
        [one] { $count } recusada pelo seu serviço
       *[other] { $count } recusadas pelos seus serviços
    }
window-count-samples-awaiting-review =
    { $count ->
        [one] { $count } aguardando análise
       *[other] { $count } aguardando análise
    }
window-count-samples-sent =
    { $count ->
        [one] { $count } enviada
       *[other] { $count } enviadas
    }
window-signatures-label = Assinaturas
window-scans-label = Verificações
window-findings-label = Descobertas
window-detections-come = De onde vêm as detecções
window-files-get-checked = Como os arquivos são verificados
window-found-label = O que foi encontrado
window-needs-review = Precisa de análise
window-not-working = Não está funcionando
window-waiting-another-part = Aguardando outra parte
window-summary-pending = Verificando…
window-menu-tooltip = Menu
window-show-lazaret = Mostrar o { -brand-name }
window-show-detections = Mostrar detecções
window-quick-scan = Verificação rápida
window-update-definitions = Atualizar definições
window-quit-tray-item = Sair
window-quit-completely = Sair completamente
window-hide-tray = Minimizar para bandeja
window-ask-on-next-close = Perguntar no próximo fechamento
window-close-lazaret = Fechar o { -brand-name }?
window-detection-alerts-stop-lazaret = Os alertas de detecção param enquanto o { -brand-name } não está em execução. Mantê-lo em execução em segundo plano ou sair completamente? Você pode alterar isso depois nas Preferências.
window-cancel-close-button = Cancelar
window-quit-close-button = Sair
window-keep-running = Manter em execução

scans-node-js-dependencies = Dependências do Node.js
scans-git-repository-data = Dados do repositório Git
scans-python-virtual-environment = Ambiente virtual do Python
scans-build-output-directory = Diretório de saída da compilação
scans-distribution-output-directory = Diretório de saída da distribuição
scans-python-bytecode-cache = Cache de bytecode do Python
scans-remove-exclusion-tooltip = Remover
scans-preset-exclusions-title = Exclusões predefinidas
scans-common-directories-exclude-description = Diretórios comuns a excluir da verificação. As alterações são salvas imediatamente.
scans-custom-exclusions-title = Exclusões personalizadas
scans-enter-absolute-path-description = Insira um caminho absoluto ou um padrão como *.tmp. As alterações são salvas imediatamente.
scans-path-exclude-tmp-title = /path/to/exclude ou *.tmp
scans-prefer-clamd-when-available = Automático
scans-always-clamd = clamd (daemon)
scans-always-clamscan = clamscan (independente)
scans-options-title = Opções
scans-backend-title = Mecanismo de verificação
scans-automatic-uses-daemon-subtitle = O modo automático usa o daemon quando ele responde e, caso contrário, o clamscan.
scans-show-live-scan-title = Mostrar progresso da verificação em tempo real
scans-display-files-scanned-subtitle = Exiba os arquivos que estão sendo verificados em tempo real.
scans-not-added-history-error = Não foi possível adicionar a verificação ao histórico: { $error }
scans-no-profile-manual = Nenhum perfil (manual)
scans-removable-drive-scanned-title = Uma unidade removível está sendo verificada. Inicie esta verificação quando ela terminar.
scans-open-removable-drives-button = Abrir unidades removíveis
scans-detecting-backend = Verificando…
scans-backend-undetermined = Mecanismo: desconhecido, não foi possível localizar os programas de verificação: { $error }
scans-backend-clamd-daemon = Mecanismo: clamd (daemon)
scans-backend-clamscan-standalone = Mecanismo: clamscan (independente)
scans-scan-title = Verificação
scans-preparing-label = Preparando…
scans-cancelling-label = Cancelando…
scans-scanning-name = Verificando { $name }
scans-scanning-count-location =
    { $count ->
        [one] Verificando { $count } local
       *[other] Verificando { $count } locais
    }
scans-cancelled-user = Verificação cancelada pelo usuário
scans-threats-found-scan = Ameaças encontradas
scans-not-finish-cleanly = A verificação não terminou corretamente
scans-no-threats-found = Nenhuma ameaça encontrada
scans-not-create-eicar-test = Não foi possível criar o arquivo de teste EICAR: { $error }
scans-add-files-tooltip = Adicionar arquivos
scans-add-folder-manual-tooltip = Adicionar pasta
scans-more-tooltip = Mais
scans-run-eicar-test-label = Executar teste EICAR
scans-copy-results-label = Copiar resultados
scans-export-results-label = Exportar resultados…
scans-start-scan-label = Iniciar verificação
scans-cancel-manual-label = Cancelar
scans-lazaret-refuses-clamav-title = O { -brand-name } recusa um programa do ClamAV que sua conta poderia substituir
scans-details-button = Detalhes
scans-profile-title = Perfil de verificação
scans-profile-short-title = Perfil
scans-manage-profiles-tooltip = Gerenciar perfis
scans-targets-manual-title = Alvos de varredura
scans-drop-files-folders-label = Solte arquivos ou pastas para verificar
scans-add-files-label = Adicionar arquivos…
scans-add-folder-label = Adicionar pasta…
scans-found-far-scan-title = Encontrado até agora
scans-new-scan-button = Nova verificação
scans-recent-scans-title = Verificações recentes
scans-started-lazaret-newest-description = Verificações iniciadas pelo { -brand-name }, das mais recentes para as mais antigas. Selecione uma para ler sua transcrição.
scans-choose-files-scan = Escolher arquivos para verificar
scans-choose-folder-scan = Escolher uma pasta para verificar
scans-quick-scan = Verificação rápida
scans-full-scan = Verificação completa
scans-home-folder = Pasta inicial
scans-fast-scan-downloads-folder = Verificação rápida da pasta Downloads para detectar ameaças rapidamente
scans-comprehensive-system-wide-scan = Verificação abrangente de todo o sistema, incluindo todos os diretórios acessíveis
scans-user-home-directory-personal = Verificação do diretório pessoal do usuário e de seus arquivos pessoais
scans-profile-file-unavailable = Não foi possível acessar o arquivo do perfil: { $error }
scans-profile-data-invalid = Os dados do perfil não são válidos: { $error }
scans-profile-file-not-replaced = Não foi possível substituir o arquivo do perfil: { $error }
scans-profile-task-interrupted = A tarefa do perfil foi interrompida: { $error }
scans-profile-name-missing = O nome do perfil é obrigatório
scans-profile-name-too-long = O nome deve ter 50 caracteres ou menos
scans-profile-name-taken = Outro perfil já usa este nome
scans-profile-targets-missing = Adicione pelo menos um alvo
scans-edit-profile-title = Editar perfil
scans-save-label = Salvar
scans-name-title = Nome
scans-description-title = Descrição
scans-targets-profile-title = Alvos
scans-add-folder-profile-tooltip = Adicionar pasta
scans-add-path-title = Adicionar caminho
scans-exclusions-title = Exclusões
scans-added-exclusions-preferences-description = Adicionadas às exclusões das Preferências enquanto este perfil é executado.
scans-add-pattern-title = Adicionar padrão
scans-cancel-profile = Cancelar
scans-defaults-restored = Perfis padrão restaurados
scans-profile-deleted = Perfil excluído
scans-profile-imported = Perfil importado
scans-manage-profiles-title = Gerenciar perfis
scans-restore-defaults-tooltip = Restaurar padrões
scans-new-profile-tooltip = Novo perfil
scans-import-tooltip = Importar
scans-profiles-title = Perfis de verificação
scans-select-profile-edit-description = Selecione um perfil para editar ou usar na verificação
scans-restore-default-profiles = Restaurar perfis padrão
scans-restore-built-default-profiles = Restaurar os perfis padrão integrados? Todas as personalizações dos perfis padrão serão redefinidas; os perfis personalizados serão preservados.
scans-delete-profile = Excluir perfil
scans-delete-profile-name = Excluir o perfil '{ $name }'?
scans-delete-label = Excluir
scans-profile-exported = Perfil exportado
scans-remove-profile-tooltip = Remover
scans-count-target =
    { $count ->
        [one] { $count } destino
       *[other] { $count } destinos
    }
scans-default-profile = Perfil padrão
scans-run-tooltip = Executar
scans-edit-tooltip = Editar
scans-delete-tooltip = Excluir
scans-export-tooltip = Exportar
scans-remove-target-tooltip = Remover
scans-check-again-access-tooltip = Verificar novamente
scans-checking-access-tooltip = Verificando
scans-status-access-title = Status
scans-access-settings-changed-description = Configurações de verificação no acesso alteradas pelo { -brand-name }, conforme registrado pelo sistema. Softwares executados como você não podem remover esses registros.
scans-no-access-settings-title = Nenhuma configuração de verificação no acesso foi alterada pelo { -brand-name }
scans-clamonacc-log-title = Registro do clamonacc
scans-clamonacc-not-written-title = O clamonacc não escreveu no journal
scans-checking-access-scanning = Verificando a verificação no acesso…
scans-command-copied-clipboard-access = Comando copiado para a área de transferência
scans-no-readable-config = Nenhum clamd.conf legível foi encontrado
scans-reading-task-aborted-clamd = A leitura do clamd.conf parou antes de terminar
scans-watched-mount = { $path } (montagem inteira)
scans-nothing-watched = Nenhum configurado
scans-default-limit = O padrão do ClamAV
scans-configuration-label = Configuração
scans-watched-folders = Pastas monitoradas
scans-largest-file-scanned = Maior arquivo verificado
scans-no-kernel-config-found = Não existe /proc/config.gz nem o arquivo de configuração do kernel em /boot.
scans-kernel-config-unreadable = Não foi possível ler a configuração do kernel.
scans-compressed-config-corrupt = Não foi possível descompactar /proc/config.gz.
scans-kernel-release-unreadable = Não foi possível ler a versão do kernel em execução.
scans-reading-task-aborted-config = A leitura da configuração do kernel parou antes de terminar.
scans-blocks-detected-files = Os arquivos detectados são bloqueados
scans-prevention-off = Os arquivos são verificados, mas os arquivos detectados não são bloqueados
scans-prevention-conflicts-mounts = Os arquivos são verificados, mas os arquivos detectados não são bloqueados, porque OnAccessPrevention não funciona com OnAccessMountPath
scans-kernel-cannot-block = Os arquivos são verificados, mas este kernel não pode bloquear arquivos detectados, porque foi compilado sem CONFIG_FANOTIFY_ACCESS_PERMISSIONS
scans-kernel-support-unknown = Os arquivos são verificados, mas não se sabe se o kernel pode bloquear arquivos detectados. { $reason }
scans-config-unreadable-monitor = Em execução, mas não foi possível ler o clamd.conf, então não se sabe o que é verificado nem se os arquivos detectados são bloqueados
scans-running-no-folders-status = Em execução, mas nenhuma pasta ou montagem é monitorada
scans-running-detected-status = Em execução, mas nenhuma pasta ou montagem é monitorada, e os arquivos detectados não seriam bloqueados
scans-unit-state-unknown-access = Não se sabe se a verificação no acesso está em execução, porque o systemd não pôde informar
scans-never-set-up-access = A verificação no acesso não está configurada
scans-unit-stopped-access = { $unit } não está em execução
scans-access-blocking-title = Bloqueio no acesso
scans-clamonacc-checks-files-watched = O clamonacc verifica os arquivos nas pastas monitoradas à medida que são abertos. Com a prevenção ativada, o kernel recusa o acesso a um arquivo detectado, e é por isso que esse arquivo não pode ser aberto, embora ainda esteja lá.
scans-mount-watching = Monitoramento de montagens
scans-onaccessmountpath-watches-whole-mount = OnAccessMountPath monitora uma montagem inteira por meio do fanotify. O ClamAV o documenta como incompatível com OnAccessPrevention, OnAccessIncludePath, OnAccessExcludePath e OnAccessExtraScanning, portanto os arquivos detectados em uma montagem monitorada nunca são bloqueados. Monitore pastas com OnAccessIncludePath para bloquear.
scans-onaccessmountpath-cannot-combined-options = OnAccessMountPath não pode ser combinado com { $options }
scans-loop-protection = Proteção contra loop de verificação
scans-clamd-opens-every-file = O clamd abre todos os arquivos que deve verificar. A menos que OnAccessExcludeUname ou OnAccessExcludeUID indique o usuário com o qual o clamd é executado, o clamonacc vê essas leituras e pede ao clamd que verifique os mesmos arquivos novamente.
scans-clamd-not-excluded-clamonacc = O clamd não está excluído, então o clamonacc pode verificar suas próprias leituras indefinidamente
scans-watching-whole-file-system = Monitorando todo o sistema de arquivos
scans-clamonacc-refuses-onaccessincludepath-include = O clamonacc recusa OnAccessIncludePath /. Inclua as pastas que recebem arquivos, como os diretórios pessoais, ou monitore uma montagem com OnAccessMountPath.
scans-onaccessincludepath-rejected-clamonacc = OnAccessIncludePath / é rejeitado pelo clamonacc
scans-folder-watch-limit = Limite de monitoramento de pastas
scans-path-watched-fs-inotify = { $path } é monitorado, mas fs.inotify.max_user_watches é apenas { $limit }, então as pastas além do limite não são verificadas
scans-unknown-whether-path-fits = Não se sabe se { $path } cabe no limite de monitoramento de pastas, porque não foi possível ler fs.inotify.max_user_watches
scans-clamonacc-watches-every-folder = O clamonacc monitora cada pasta abaixo de um caminho incluído com o inotify, um monitoramento por pasta. Um diretório pessoal inteiro ou uma árvore maior pode precisar de mais monitoramentos do que o antigo limite padrão de 8192 permite. Aumente fs.inotify.max_user_watches com o sysctl.
scans-device-auto-scan-title = Verificação automática de dispositivos
scans-automatically-scan-removable-description = Verifique automaticamente unidades removíveis e externas quando forem conectadas.
scans-enable-device-auto-title = Ativar verificação automática de dispositivos
scans-max-device-size-title = Tamanho máximo do dispositivo (GB)
scans-no-size-limit-subtitle = 0 significa sem limite de tamanho.
scans-skip-battery-device-title = Pular na bateria
scans-drive-scan-not-added = Não foi possível adicionar a verificação da unidade ao histórico: { $error }
scans-device-auto-scan = Verificação automática de dispositivos
scans-device-connected-during-another = Um dispositivo foi conectado durante outra verificação, então não foi verificado.
scans-removable-drives-title = Unidades removíveis
scans-cancel-removable-label = Cancelar
scans-found-far-removable-title = Encontrado até agora
scans-drives-session-title = Unidades nesta sessão
scans-drives-connected-since-description = Unidades conectadas desde que o { -brand-name } foi iniciado, das mais recentes para as mais antigas, e o que aconteceu com cada uma. Esta lista é mantida apenas até o { -brand-name } ser encerrado.
scans-no-drive-connected-title = Nenhuma unidade foi conectada ainda
scans-close-results-button = Fechar resultados
scans-drive-scans-title = Verificações de unidades
scans-drives-lazaret-scanned-description = Unidades que o { -brand-name } verificou ao serem conectadas, das mais recentes para as mais antigas. Selecione uma para ler sua transcrição.
scans-auto-scan-off = Desativado
scans-every-drive = Verifica todas as unidades
scans-drives-up-size = Verifica unidades de até { $size }
scans-skipped-larger-than-limit = Não verificada: { $size } é maior que o limite de { $limit }
scans-skipped-battery = Não verificada: o computador estava usando a bateria
scans-skipped-auto-scan-off = Não verificada: a verificação automática de dispositivos está desativada
scans-skipped-while-scanning = Não verificada: outra verificação estava em execução
scans-size-unreadable = Não verificada: não foi possível ler o tamanho da unidade
scans-drive-settling = Verificando a unidade…
scans-drive-scanning = Verificando…
scans-threats-detected = Verificada, ameaças encontradas
scans-no-threats-detected = Verificada, nenhuma ameaça encontrada
scans-finished-problems = Verificada, mas a verificação não terminou corretamente
scans-scan-cancelled = Verificação cancelada
scans-device-scan-running = Verificando { $drive }
scans-drive-scan = Verificação de unidade
scans-check-again-scan-tooltip = Verificar novamente
scans-checking-scan-tooltip = Verificando
scans-status-scan-title = Status
scans-engine-detection-settings-description = Configurações do mecanismo e de detecção alteradas pelo { -brand-name }, conforme registrado pelo sistema. Softwares executados como você não podem remover esses registros.
scans-no-engine-detection-title = Nenhuma configuração do mecanismo ou de detecção foi alterada pelo { -brand-name }
scans-clamd-log-title = Registro do clamd
scans-no-clamd-log-title = Nenhum registro do clamd foi encontrado
scans-checking-scan-engine = Verificando o mecanismo de verificação…
scans-command-copied-clipboard-scan = Comando copiado para a área de transferência
scans-program-start-failed = Não foi possível iniciar o clamscan: { $error }
scans-probe-timed-out = O clamscan não respondeu a tempo
scans-scanner-refused = { $path } recusado, porque sua conta pode substituí-lo
scans-engine-missing = O ClamAV não foi encontrado neste computador
scans-clamav-installed = ClamAV instalado
scans-clamav-engine-recognizes-malware = O ClamAV é o mecanismo que reconhece malware. O { -brand-name } usa a cópia instalada neste computador e nunca inclui a sua própria.
scans-clamd-still-uses-signatures = O clamd ainda usa as assinaturas { $loaded }, mas { $disk } está no disco
scans-clamd-answering = O clamd está respondendo
scans-unit-active-not-answering = { $unit } está em execução, mas o clamd não responde. Ele ainda pode estar carregando as assinaturas.
scans-unit-state-unknown-daemon = O clamd não responde, e o systemd não pôde informar o estado dele
scans-unit-stopped-daemon = { $unit } não está em execução
scans-never-set-up-daemon = O clamd não está configurado, então as verificações recorrem ao clamscan, mais lento
scans-same-version-disk = A mesma versão que está no disco
scans-loaded-disk = { $loaded }, enquanto { $disk } está no disco
scans-unknown-loaded-stored-versions = Desconhecido, não foi possível comparar as versões carregada e armazenada
scans-loaded-signatures = Assinaturas carregadas
scans-daemon-running = Daemon em execução
scans-clamd-keeps-signatures-loaded = O clamd mantém as assinaturas carregadas na memória. O bloqueio no acesso e as verificações agendadas enviam seus arquivos a ele.
scans-socket-protected = Somente o sistema pode substituir { $path }
scans-socket-replaceable = Sua conta pode substituir { $replaceable }, então um programa executado como você poderia colocar um clamd falso em { $path } que informa todos os arquivos como limpos
scans-socket-missing = Nenhum socket do clamd em { $path }, então o clamd não está em execução ou escuta em outro lugar
scans-socket-not-checkable = A sandbox do Flatpak não consegue ver quem é o dono do socket do clamd
scans-clamd-socket-protected = Socket do clamd protegido
scans-socket-mechanism = As verificações chegam ao clamd pelo socket dele. Deixe o clamd criá-lo em um diretório em que apenas o root ou o usuário clamav possam escrever, como /run/clamav, para que nenhum programa executado como você possa colocar um clamd falso no lugar dele.
scans-socket-inspecting = Verificando quem pode substituir o socket do clamd…
scans-timer-inactive = Não agendado
scans-next-run-time = Próxima execução: { $time }
scans-scheduled-time-next-run = Agendado. O horário da próxima execução ainda não é conhecido.
scans-systemd-unavailable = As verificações agendadas precisam de uma sessão de usuário do systemd, que não está disponível.
scans-system-already-runs-scheduled = Seu sistema já executa verificações agendadas, então estas configurações estão bloqueadas para evitar um segundo agendamento. Próxima verificação: { $time }. Altere-o na configuração do sistema.
scans-system-scan-timer-settings = Seu sistema tem um temporizador de verificação, então estas configurações estão bloqueadas para evitar um segundo agendamento. Altere-o na configuração do sistema.
scans-reading-user-timer = Verificando…
scans-inspecting-system-timer = Verificando se o seu sistema já executa verificações agendadas. Estas configurações permanecem bloqueadas até lá.
scans-system-timer-unknown = O { -brand-name } não conseguiu verificar se o seu sistema já executa verificações agendadas, então estas configurações estão bloqueadas para evitar um segundo agendamento: { $error }
scans-scan-time-unreadable = Insira o horário da verificação como HH:MM, por exemplo 02:00.
scans-unit-files-unwritable = Erro ao criar os arquivos do systemd: { $error }
scans-timer-not-applied-error = Não foi possível aplicar o temporizador: { $error }
scans-hourly-label = A cada hora
scans-daily-label = Diariamente
scans-weekly-label = Semanalmente
scans-monthly-label = Mensalmente
scans-monday-label = Segunda-feira
scans-tuesday-label = Terça-feira
scans-wednesday-label = Quarta-feira
scans-thursday-label = Quinta-feira
scans-friday-label = Sexta-feira
scans-saturday-label = Sábado
scans-sunday-label = Domingo
scans-schedule-title = Agendamento
scans-apply-label = Aplicar
scans-enable-scheduled-scans-title = Ativar verificações agendadas
scans-frequency-title = Frequência da verificação
scans-time-24-hour-title = Horário da verificação (24 horas)
scans-day-week-title = Dia da semana
scans-used-weekly-scans-subtitle = Usado para verificações semanais.
scans-day-month-title = Dia do mês
scans-used-monthly-scans-subtitle = Usado para verificações mensais.
scans-targets-separated-commas-title = Alvos da verificação (separados por vírgulas; vazio verifica sua pasta pessoal)
scans-skip-battery-schedule-title = Pular na bateria
scans-not-start-scheduled-subtitle = Não iniciar verificações agendadas quando estiver usando a bateria.
scans-auto-quarantine-title = Quarentena automática
scans-automatically-quarantine-threats-subtitle = Coloque automaticamente em quarentena as ameaças encontradas pelas verificações agendadas. Enquanto o { -brand-name } estiver aberto, as ameaças são apenas informadas.
scans-review-button = Analisar
scans-check-again-scheduled-tooltip = Verificar novamente
scans-checking-scheduled-tooltip = Verificando
scans-system-scan-title = Verificação do sistema
scans-folders-scanned-title = Pastas verificadas
scans-run-now-label = Executar agora
scans-starting-scan-tooltip = Iniciando a verificação…
scans-scanning-title = Verificando
scans-started-at = Iniciada às { $time }
scans-stopping-scan = Parando a verificação…
scans-stop-scan-label = Parar
scans-stop-scan-tooltip = Parar a verificação em andamento
scans-scan-stopped = A verificação foi parada
scans-stop-authorization-refused = Parar a verificação requer autorização de administrador
scans-not-stopped-error = Não foi possível parar a verificação: { $error }
scans-status-scheduled-title = Status
scans-scheduled-runs-title = Execuções agendadas
scans-lazaret-schedule-ran-description = Verificações executadas pelo agendamento do { -brand-name }, das mais recentes para as mais antigas. Selecione uma para ler sua transcrição.
scans-checking-schedule = Verificando o agendamento…
scans-command-copied-clipboard-scheduled = Comando copiado para a área de transferência
scans-not-scheduled = Não agendado
scans-next-scan = Próxima verificação
scans-threats-found-scheduled = ameaças encontradas
scans-nothing-found = nada encontrado
scans-no-completed-scan = Ainda não executada
scans-last-scan = Última verificação
scans-unknown-label = Desconhecido
scans-system-timer = O temporizador do sistema não está ativo
scans-last-scan-ended-error = A última verificação terminou com um erro
scans-next-scan-time = Próxima verificação: { $time }
scans-timer-starts-scan-service = O temporizador inicia um serviço de verificação que não está instalado
scans-service-not-read-systemd = Não foi possível ler o serviço de verificação do systemd
scans-lazaret-schedule = Agendado pelo { -brand-name }
scans-no-schedule = Nenhuma verificação está agendada
scans-not-scheduled-timer-active = Não agendado, porque o temporizador não está ativo
scans-folders-scanned = Pastas verificadas
scans-scheduled-scan = Verificação agendada
scans-scheduled-scan-catches-files = Uma verificação agendada detecta arquivos que não foram abertos ou que só se tornaram detectáveis após uma atualização de assinaturas.
scans-unit-names-no-folders = A unidade de verificação não indica nenhuma pasta
scans-system-scan-service-unit = O serviço de verificação do seu sistema { $unit } verifica estas pastas com as configurações do próprio sistema. Executá-lo requer autorização de administrador.
scans-needs-daemon-see-scan = Precisa do daemon, veja Mecanismo de verificação
scans-bus-call-failed = O systemd não respondeu: { $error }
scans-unrecognized-load-state = O systemd informou um estado de unidade que o { -brand-name } não reconhece
scans-found-malware = A verificação encontrou malware
scans-unit-finished = A verificação terminou
scans-not-finish-cleanly-result = A verificação não terminou corretamente: { $result }
scans-authorization-refused = Iniciar a verificação requer autorização de administrador
scans-not-started-error = Não foi possível iniciar a verificação: { $error }

findings-open-preferences = Abrir preferências
findings-journal-unreadable-alerts = Não é possível ler o journal do sistema. Adicione seu usuário ao grupo systemd-journal.
findings-notifications-turned-off-preferences = As notificações estão desativadas nas preferências
findings-alerts-stop-quit-lazaret = Os alertas param quando você sai do { -brand-name }, porque ele não está configurado para ser executado em segundo plano
findings-running-background-following-journal = Em execução em segundo plano e acompanhando o journal
findings-alerts-label = Alertas
findings-lazaret-follows-system-journal = O { -brand-name } acompanha o journal do sistema e notifica você quando algo é detectado. Ele só pode fazer isso enquanto está em execução, então deve ser executado em segundo plano.
findings-notifications-on = Ativado
findings-notifications-off = Desativado
findings-running-label = Em execução
findings-not-running = Não está em execução
findings-unit-not-found = Não configurado
findings-systemd-unavailable = Desconhecido, não é possível acessar o systemd
findings-following-journal-access = Acompanhado
findings-journal-unreadable-access = Não pode ser lido
findings-following-failed = Acompanhamento parado
findings-notifications-label = Notificações
findings-background-service = Serviço em segundo plano
findings-system-journal = Journal do sistema
findings-attach-unknown-setting = Não se sabe se outros programas podem se conectar, porque não é possível ler a configuração do kernel: { $reason }
findings-attach-unknown-reports = Não se sabe se outros programas podem se conectar, porque o kernel informa um valor que o { -brand-name } não reconhece
findings-attach-unknown-check = Não se sabe se outros programas podem se conectar, porque a verificação parou antes de terminar
findings-attach-restricted = Programas não podem se conectar a outros programas que você executa, como o { -brand-name }
findings-any-program-may-attach = Qualquer programa que você executa pode se conectar ao { -brand-name } e alterar o que ele mostra, incluindo as detecções e as decisões que você toma aqui
findings-yama-not-loaded = Este kernel não restringe conexões, então qualquer programa que você executa pode se conectar ao { -brand-name } e alterar o que ele mostra
findings-other-programs-cannot-attach = Outros programas não podem se conectar ao { -brand-name }
findings-program-attaches-lazaret-hide = Um programa que se conecta ao { -brand-name } pode ocultar alertas ou alterar suas decisões. Definir kernel.yama.ptrace_scope como 1 permite que apenas o processo pai de um programa se conecte a ele.
findings-file-owned-other-user = Este arquivo pertence a outro usuário, então não pode ser movido nem excluído daqui
findings-file-not-checkable = Não foi possível verificar este arquivo, então ele não pode ser movido nem excluído daqui
findings-removal-failed = Não foi possível excluir o arquivo: { $error }
findings-file-check-interrupted = A verificação do arquivo parou inesperadamente
findings-history-not-updated = Não foi possível salvar sua decisão: { $error }
findings-journal-unreadable-list = O { -brand-name } não consegue ler o journal do sistema, então novas detecções em segundo plano não aparecerão aqui. Isso não significa que nada foi encontrado. Adicione seu usuário ao grupo systemd-journal.
findings-loading-detections = Carregando detecções…
findings-nothing-detected-background = Nada foi detectado em segundo plano.
findings-access-scanning-scheduled-scans = O que a verificação no acesso e as verificações agendadas encontraram.
findings-detections-title = Detecções
findings-back-detections-tooltip = Voltar às detecções
findings-back-quarantine-tooltip = Voltar para a quarentena
findings-alerts-title = Alertas
findings-detections-reach-keeps-description = Como as detecções chegam até você e o que impede outros programas de ocultá-las.
findings-check-again-background-tooltip = Verificar novamente
findings-checking-background-tooltip = Verificando
findings-detection-title = Detecção
findings-copied-clipboard-toast = Copiado para a área de transferência
findings-command-copied-clipboard-toast = Comando copiado para a área de transferência
findings-detections-lazaret-stopped = As detecções enquanto o { -brand-name } não estava em execução não podem ser contadas, porque não foi possível ler onde ele parou de ler o journal do sistema: { $error }
findings-detections-not-read-error = Não foi possível ler as detecções: { $error }
findings-malware-blocked-signature = Malware bloqueado: { $signature }
findings-malware-found-signature = Malware encontrado: { $signature }
findings-count-detection-not-running =
    { $count ->
        [one] { $count } detecção enquanto o { -brand-name } não estava em execução
       *[other] { $count } detecções enquanto o { -brand-name } não estava em execução
    }
findings-detections-not-running-detection = As detecções enquanto o { -brand-name } não estava em execução não podem ser contadas
findings-lazaret-not-read-stopped = O { -brand-name } não conseguiu ler onde parou de acompanhar o journal do sistema, então só monitora novas detecções. Verifique a página Detecções e o journal do sistema para saber o que foi encontrado enquanto ele estava fechado.
findings-quarantine-incident = Quarentena
findings-ask-incident-deletion = Excluir…
findings-delete-incident-file = Excluir permanentemente
findings-cancel-incident-deletion = Cancelar
findings-dismiss-incident = Manter
findings-look-up-incident-signature = Consultar
findings-submit-incident-sample = Enviar…
findings-restored-quarantine = Restaurado da quarentena
findings-moved-quarantine = Movido para a quarentena
findings-deleted-label = Excluído
findings-kept-label = Mantido por você
findings-belongs-another-user = Pertence a outro usuário
findings-blocked-needs-decision = Bloqueado, precisa de uma decisão
findings-not-blocked-needs-decision = Não bloqueado, precisa de uma decisão
findings-needs-decision = Precisa de uma decisão
findings-no-longer-there-detection = Não está mais lá
findings-state-unknown-detection = Não foi possível verificar o arquivo
findings-signature-test-file-state = { $signature } (arquivo de teste) · { $state }
findings-signature-state = { $signature } · { $state }
findings-access-schedule = Verificação no acesso e verificação agendada
findings-access-only = Verificação no acesso
findings-schedule-only = A verificação agendada
findings-file-label = Arquivo
findings-signature-source = Fonte da assinatura
findings-found-label = Encontrado por
findings-last-seen = Visto pela última vez
findings-times-seen = Vezes visto
findings-file-path = Caminho
findings-file-owner = Proprietário
findings-file-group = Grupo
findings-file-permissions = Permissões
findings-file-size = Tamanho
findings-file-modified = Modificado
findings-file-status = Estado
findings-file-facts-live = Como o arquivo está agora, o que pode diferir de quando foi detectado
findings-file-facts-recorded = Como registrado quando o arquivo foi movido para a quarentena
findings-open-details = Abrir detalhes
findings-detected-label = Detectado
findings-signature-matches-harmless-eicar = Esta assinatura corresponde ao padrão de teste inofensivo EICAR, que existe para verificar se a detecção funciona.
findings-clamav-matched-file-against = O ClamAV encontrou neste arquivo uma correspondência com uma assinatura. Uma correspondência é uma forte evidência, mas falsos positivos acontecem, especialmente com ferramentas compactadas ou administrativas.
findings-signature-first-seen-time = { $signature }, visto pela primeira vez { $local-time }
findings-opening-file-refused-why = A abertura deste arquivo é recusada, e é por isso que ele não abre
findings-not-opened-again-after = Não foi aberto novamente após sua decisão
findings-no-longer-there-blocked = O arquivo não está mais lá
findings-not-blocked-scheduled-scan = Não bloqueado: a verificação agendada o encontrou, e ele ainda pode ser aberto
findings-not-blocked-right-now = Não bloqueado no momento: ele ainda pode ser aberto
findings-state-unknown-blocked = Não foi possível verificar se o arquivo está bloqueado
findings-blocked-label = Bloqueado
findings-access-prevention-kernel-refuses = Com a prevenção no acesso, o kernel se recusa a abrir um arquivo detectado em uma pasta monitorada. O arquivo permanece onde está até você decidir o que fazer com ele.
findings-looking-up = Consultando…
findings-no-known-samples-match = Nenhuma amostra conhecida corresponde a esta assinatura.
findings-unknown-label = desconhecido
findings-count-known-samples-families = { $count } amostras conhecidas. Famílias: { $families }. Vista pela primeira vez: { $date }.
findings-ask-malwarebazaar-signature-description = Pergunte ao MalwareBazaar sobre esta assinatura. Apenas o nome dela é enviado.
findings-set-up-lookup-button = Configurar…
findings-bazaar-no-key = Sem chave
findings-bazaar-key-unreadable = Chave ilegível
findings-stop-asking-file-description = Parar de perguntar sobre este arquivo
findings-delete-file-permanently-description = Excluir este arquivo permanentemente? Isso não pode ser desfeito.
findings-go-back-without-description = Voltar sem excluir
findings-move-somewhere-safe-description = Mova-o para um lugar seguro. Você pode restaurá-lo depois.
findings-remove-good-description = Removê-lo definitivamente
findings-know-file-fine-description = Você sabe que este arquivo é seguro. O ClamAV ainda o informa na próxima verificação; confie nele na Quarentena ou em um resultado de verificação para impedir isso.
findings-decision-title = Sua decisão
findings-unfamiliar-quarantine-keeping-file = Se não o reconhecer, coloque-o em quarentena. Manter um arquivo só faz o { -brand-name } parar de perguntar: a verificação no acesso continua bloqueando-o até que o caminho seja excluído na configuração do ClamAV, como mostrado aqui.
findings-quarantined-files-restored-quarantine = Arquivos em quarentena podem ser restaurados na página Quarentena. Um arquivo que você manteve continua bloqueado pela verificação no acesso até que seu caminho seja excluído na configuração do ClamAV.
findings-file-other-owner-decision = Este arquivo pertence a outro usuário, então o { -brand-name } não pode movê-lo nem excluí-lo
findings-state-unknown-review = O { -brand-name } não conseguiu verificar este arquivo, então não pode movê-lo nem excluí-lo
findings-choose-happens-file = Escolha o que acontece com este arquivo
findings-malwarebazaar-label = MalwareBazaar
findings-resolved-title = Resolvido
findings-resolved-detection-opens-again = Uma detecção resolvida é reaberta se o ClamAV encontrar o mesmo arquivo novamente mais tarde, por exemplo, depois que você baixá-lo uma segunda vez.
findings-waiting-decision = Aguardando sua decisão
findings-resolved-state-step = { $state }, { $time }
findings-send-quarantined-copy-description = Envie a cópia em quarentena para o ClamAV, o MalwareBazaar ou o VirusTotal. Um falso positivo informado ao ClamAV é corrigido para todos por uma atualização de assinaturas posterior.
findings-cancel-quarantine = Cancelar
findings-delete-entries = Excluir
findings-scanner-program-not-installed = O programa de verificação não está instalado.
findings-scanner-program-replaced-account = O programa de verificação em { $program } pode ser substituído pela sua conta por meio de { $path }.
findings-content-pipe-failed = Não foi possível passar o arquivo para o verificador: { $error }
findings-scanner-spawn-failed = Não foi possível iniciar o verificador: { $error }
findings-verdict-timed-out = A verificação demorou demais.
findings-detection-settings-unreadable = Não foi possível ler as configurações de detecção do clamd: { $error }
findings-program-lookup-failed = Não foi possível localizar os programas de verificação: { $error }
findings-payload-unreadable = Não foi possível ler a cópia em quarentena: { $error }
findings-restore-file-heading = Restaurar este arquivo?
findings-clamav-no-longer-verdict = O ClamAV não detecta mais este arquivo com as assinaturas atuais.
findings-restore-button = Restaurar
findings-restore-file-clamav-heading = Restaurar um arquivo que o ClamAV ainda detecta?
findings-clamav-still-detects-verdict = O ClamAV ainda detecta este arquivo como { $detection } com as assinaturas de hoje. Só o restaure se tiver certeza de que é um falso positivo, e considere marcá-lo como confiável antes.
findings-clamav-still-detects-unnamed-verdict = O ClamAV ainda detecta este arquivo com as assinaturas de hoje, sem dar nome à detecção. Só o restaure se tiver certeza de que é um falso positivo, e considere marcá-lo como confiável antes.
findings-restore-anyway-button = Restaurar mesmo assim
findings-restore-unchecked-file-heading = Restaurar um arquivo não verificado?
findings-file-not-checked-verdict = Não foi possível verificar o arquivo novamente antes de restaurá-lo. { $reason }
findings-location-outside-home-folder = Este local está fora da sua pasta pessoal.
findings-file-goes-back = O arquivo volta para:
findings-entry-count =
    { $count ->
        [one] { $count } item
       *[other] { $count } itens
    }
findings-clear-old-items = Limpar itens antigos?
findings-count-quarantined-file-older =
    { $count ->
        [one] { $count } arquivo em quarentena com mais de 30 dias será excluído permanentemente.
       *[other] { $count } arquivos em quarentena com mais de 30 dias serão excluídos permanentemente.
    }
findings-delete-quarantined-file = Excluir arquivo em quarentena?
findings-file-permanently-deleted = O arquivo será excluído permanentemente.
findings-quarantine-title = Quarentena
findings-more-tooltip = Mais
findings-check-all-again-label = Verificar tudo de novo
findings-count-still-detected-after-recheck =
    { $count ->
        [one] { $count } arquivo ainda é detectado.
       *[other] { $count } arquivos ainda são detectados.
    }
findings-count-not-rechecked =
    { $count ->
        [one] Não foi possível verificar { $count } arquivo.
       *[other] Não foi possível verificar { $count } arquivos.
    }
findings-clear-old-items-label = Limpar itens antigos…
findings-search-tooltip = Pesquisar
findings-search-threat-name-placeholder = Pesquisar por nome ou caminho da ameaça
findings-quarantine-storage-title = Armazenamento da quarentena
findings-quarantine-storage-description = Ameaças detectadas isoladas do seu sistema.
findings-look-up-hash-button = Consultar hash
findings-reputation-section-title = Reputação
findings-reputation-section-description = Consultar hash envia apenas o hash SHA-256 do arquivo, nunca o arquivo.
findings-submit-sample-row-title = Enviar o próprio arquivo
findings-report-false-positive-row-title = Avisar a Cisco Talos
findings-report-false-positive-row-subtitle = Uma atualização de assinaturas posterior corrige a detecção para todos.
findings-trust-file-row-title = Confiar neste computador
findings-trust-file-row-subtitle = O ClamAV deixa de relatar este arquivo exato aqui e o lista em Detecções confiáveis. Restaure-o depois.
findings-total-size-title = Tamanho total
findings-dismiss-label = Dispensar
findings-quarantine-cannot-read-title = Não é possível ler a quarentena
findings-loading-quarantine = Carregando a quarentena…
findings-quarantine-empty = A quarentena está vazia
findings-hash-copied-clipboard = Hash copiado para a área de transferência
findings-count-quarantined-file-no =
    { $count ->
        [one] { $count } arquivo em quarentena não é mais detectado. Verifique-o antes de restaurá-lo.
       *[other] { $count } arquivos em quarentena não são mais detectados. Verifique-os antes de restaurá-los.
    }
findings-count-report-clamav-settled =
    { $count ->
        [one] { $count } relatório ao ClamAV foi resolvido pelas novas assinaturas
       *[other] { $count } relatórios ao ClamAV foram resolvidos pelas novas assinaturas
    }
findings-checking-label = Verificando…
findings-no-longer-detected = Não é mais detectado
findings-still-detected = Ainda detectado
findings-integrity-unknown-hash = Desconhecida. Este registro de quarentena não tem um hash utilizável, então o arquivo não pode ser verificado, consultado on-line, marcado como confiável nem restaurado. Você ainda pode excluí-lo.
findings-not-checked = Ainda não verificado
findings-integrity-unknown-label = Integridade desconhecida
findings-restore-label = Restaurar
findings-delete-entry-label = Excluir
findings-set-up-source-button = Configurar…
findings-reputation-no-key = Sem chave
findings-reputation-key-unreadable = Chave ilegível
findings-view-report-label = Ver relatório
findings-false-positive-title = Falso positivo
findings-report-label = Informar…
findings-submit-quarantine-label = Enviar…
findings-send-clamav-malwarebazaar-tooltip = Enviar para o ClamAV, o MalwareBazaar ou o VirusTotal
findings-trust-label = Confiar…
findings-copy-hash-tooltip = Copiar hash
findings-submit-quarantined-label = Enviar…
findings-service-label = Serviço
findings-status-label = Status
findings-quarantine-record-no-usable = Este registro de quarentena não tem um hash utilizável, então o { -brand-name } não o envia a lugar nenhum
findings-file-no-longer-quarantine = Este arquivo não está mais em quarentena
findings-service-not-take-detection = Este serviço não aceita esta detecção
findings-file-no-usable-hash = Este arquivo não está mais em quarentena com um hash utilizável
findings-check-malwarebazaar-virustotal-tooltip = Consultar o MalwareBazaar e o VirusTotal por novidades
findings-checking-sent-tooltip = Verificando
findings-send-file-title = Enviar um arquivo
findings-nothing-leaves-computer-description = Nada sai deste computador até você confirmar o que é enviado.
findings-report-missed-malware-label = Informar malware não detectado…
findings-send-clamav-malicious-tooltip = Enviar ao ClamAV um arquivo malicioso que ele não detecta
findings-sent-title = Enviados
findings-lazaret-sent-became-description = O que o { -brand-name } enviou e o que aconteceu com isso.
findings-filter-sent-samples-tooltip = Filtrar amostras enviadas
findings-online-services-title = Serviços on-line
findings-each-service-receives-subtitle = O que cada serviço recebe e as chaves do MalwareBazaar e do VirusTotal
findings-choose-file-clamav-misses = Escolher o arquivo que o ClamAV não detecta
findings-false-positive = Falso positivo
findings-missed-malware = Malware não detectado
findings-malware-sample = Amostra de malware
findings-clamav-false-positive = Informar falso positivo ao ClamAV
findings-clamav-missed-malware = Informar malware não detectado ao ClamAV
findings-malwarebazaar-sample-choice = Enviar ao MalwareBazaar
findings-virustotal-sample-choice = Enviar ao VirusTotal
findings-awaiting-clamav = Aguardando análise do ClamAV. As análises levam pelo menos 48 horas; uma atualização de assinaturas posterior mostra o resultado.
findings-fixed-database-version = Corrigido na versão { $version } do banco de dados
findings-fixed-current-signature-database = Corrigido no banco de dados de assinaturas atual
findings-detected-since-database-version = Detectado desde a versão { $version } do banco de dados
findings-detected-current-signature-database = Detectado pelo banco de dados de assinaturas atual
findings-uploaded-waiting-malwarebazaar-list = Enviado; aguardando o MalwareBazaar listá-lo
findings-listed-malwarebazaar = Listado no MalwareBazaar
findings-malwarebazaar-refused-reason = O MalwareBazaar o recusou: { $reason }
findings-queued-analysis-virustotal = Na fila para análise no VirusTotal
findings-count-total-engine-flags =
    { $total ->
        [one] { NUMBER($count) } de { $total } mecanismo o marca como malicioso
       *[other]
            { $count ->
                [one] { NUMBER($count) } de { $total } mecanismos o marca como malicioso
               *[other] { NUMBER($count) } de { $total } mecanismos o marcam como malicioso
            }
    }
findings-virustotal-refused-reason = O VirusTotal o recusou: { $reason }
findings-service-kind-hash-date = { $service } · { $kind } · { $hash } · { $date }
findings-integrity-unknown-candidate = { $detection } · Nenhum hash utilizável, então o { -brand-name } não pode provar o que enviaria
findings-all-services = Todos os serviços
findings-any-status-option = Qualquer status
findings-waiting-label = Aguardando
findings-settled-label = Resolvido
findings-refused-label = Recusado
findings-all-submissions-option = Todos os envios
findings-listing-loading = Carregando envios…
findings-nothing-submitted = Nada enviado
findings-no-submission-matches-filters = Nenhum envio corresponde aos filtros
findings-candidates-loading = Carregando a quarentena…
findings-quarantine-empty-sent = A quarentena está vazia
findings-no-upload-changed = Nenhum envio mudou
findings-count-upload-new-status =
    { $count ->
        [one] { $count } envio tem um novo status
       *[other] { $count } envios têm um novo status
    }
findings-choose-service-heading = Para onde este arquivo deve ir?
findings-set-up-keys-response = Configurar chaves…
findings-each-service-gets-file = Cada serviço recebe o próprio arquivo. A próxima etapa mostra exatamente o que é enviado e quem pode vê-lo.
findings-cancel-submission = Cancelar
findings-open-remote-entry-tooltip = Abrir a entrada remota
findings-same-upload-running = Este arquivo já está sendo enviado para este serviço. Aguarde o término antes de enviá-lo novamente.
findings-check-again-trusted-tooltip = Verificar novamente
findings-checking-trusted-tooltip = Verificando
findings-status-title = Status
findings-files-trusted-detections-description = Arquivos confiáveis e detecções ignoradas ou restauradas pelo { -brand-name }, conforme registrado pelo sistema. Softwares executados como você não podem remover esses registros.
findings-no-trusted-detections-title = Nenhuma detecção confiável foi alterada pelo { -brand-name }
findings-checking-trusted-detections = Verificando as detecções confiáveis…
findings-command-copied-clipboard = Comando copiado para a área de transferência
findings-other-allowlist = Outra lista de permissões
findings-file-reason = { $file }: { $reason }
findings-trusted-files = Arquivos confiáveis
findings-ignored-detections = Detecções ignoradas
findings-allowlist-not-write-status = Uma lista de permissões que o { -brand-name } não escreveu faz o ClamAV ficar em silêncio
findings-count-file-detection-status =
    { $count ->
        [one] { $count } arquivo ou detecção nunca é informado
       *[other] { $count } arquivos ou detecções nunca são informados
    }
findings-own-allowlist-unreadable-status = O { -brand-name } não consegue ler a própria lista de permissões, então não sabe sobre o que o ClamAV fica em silêncio
findings-folder-unreadable = O { -brand-name } não consegue ler a pasta de assinaturas, então não sabe sobre o que o ClamAV fica em silêncio
findings-signature-folder = Pasta de assinaturas
findings-trusted-detections = Detecções confiáveis
findings-trust-list-mechanism = Arquivos e nomes de detecção nas listas de permissões do ClamAV nunca são informados, por nenhuma verificação. Um malware que obtenha direitos de administrador poderia se esconder dessa forma, então verifique se cada entrada foi adicionada de propósito.
findings-clamav-reports-everything-detects = O ClamAV informa tudo o que detecta
findings-remove-tooltip = Remover
findings-access-denied = Permissão negada
findings-other-read-failure = Falha na leitura: { $reason }
findings-reading-task-aborted = A leitura parou antes de terminar
findings-journal-unreadable-added = Não se sabe quem adicionou isto. { $reason }
findings-known-user = Adicionado { $local-time } pelo usuário { $uid }
findings-unknown-user = Adicionado { $time } por um usuário desconhecido
findings-no-record-added-system = Nenhum registro de quem adicionou isto no journal do sistema
findings-count-entry-not-added =
    { $count ->
        [one] { $count } entrada, não adicionada pelo { -brand-name }
       *[other] { $count } entradas, não adicionadas pelo { -brand-name }
    }
findings-entries-cannot-counted-reason = As entradas não podem ser contadas ({ $reason }), não adicionadas pelo { -brand-name }
findings-list-unreadable = O { -brand-name } não consegue ler esta lista
findings-no-trusted-files = Nenhum arquivo confiável
findings-no-ignored-detections = Nenhuma detecção ignorada
findings-lazaret-cannot-read-signature = O { -brand-name } não consegue ler a pasta de assinaturas
findings-trusted-detections-title = Detecções confiáveis
findings-trusted-by-you-title = Confiáveis para você
findings-trusted-by-you-description = Suas verificações não mostram estes arquivos, e nem suas verificações nem os alertas em segundo plano mostram estas detecções. O próprio ClamAV ainda as detecta, então a verificação no acesso ainda pode bloquear os arquivos e outros usuários ainda os veem. Qualquer alteração em um arquivo confiável faz com que ele apareça novamente.
findings-nothing-trusted-by-you = Você não marcou nada como confiável
findings-trust-by-you-unreadable = O { -brand-name } não consegue ler o que você marcou como confiável: { $error }
findings-file-trusted-by-you = Um arquivo em que você confiou
findings-trust-for-all-users-tooltip = Confiar para todos os usuários…
findings-trusted-for-all-users-title = Confiáveis para todos os usuários
findings-clamav-never-reports-description = O ClamAV nunca informa o que está listado aqui, para nenhum usuário e em nenhuma verificação neste computador. Cada alteração precisa de uma senha de administrador e é registrada no journal do sistema, onde softwares executados como você não podem remover o registro.
findings-ignored-detections-title = Detecções ignoradas
findings-detection-names-clamav-description = Nomes de detecção que o ClamAV ignora em todos os arquivos, para todos os usuários. O { -brand-name } não os adiciona mais, porque eles ocultam todos os arquivos com essa detecção; remova-os e confie nos arquivos pelo hash.
findings-other-allowlists-title = Outras listas de permissões
findings-allowlist-files-signature-description = Arquivos de lista de permissões na pasta de assinaturas que o { -brand-name } não escreveu. Verifique se você ou seu administrador os adicionaram de propósito.

signatures-history-load-failed = Falha ao carregar os registros históricos: { $error }
signatures-database-update-finished = Atualização do banco de dados concluída
signatures-check-again-database-tooltip = Verificar novamente
signatures-checking-database-tooltip = Verificando
signatures-update-history-title = Histórico de atualizações
signatures-updates-started-lazaret-description = Atualizações iniciadas pelo { -brand-name }, das mais recentes para as mais antigas. Selecione uma para ler sua transcrição.
signatures-historical-logs-cannot-title = Não é possível ler os registros históricos
signatures-status-title = Status
signatures-loading-history = Carregando o histórico…
signatures-no-updates-started-lazaret = Nenhuma atualização foi iniciada pelo { -brand-name } ainda
signatures-checking-signatures = Verificando as assinaturas…
signatures-command-copied-clipboard-database = Comando copiado para a área de transferência
signatures-update-not-added-history = Não foi possível adicionar a atualização ao histórico: { $error }
signatures-update-signatures = Atualizar agora
signatures-cancel-signature-update = Cancelar
signatures-definitions-built-date = Definições criadas { $relative-time } ({ $date })
signatures-definitions-exist-not-determined = As definições existem, mas não foi possível determinar a idade delas
signatures-no-virus-definitions-found = Nenhuma definição de vírus foi encontrada
signatures-age-definitions-unknown-checking = A idade das definições é desconhecida, porque a verificação parou antes de terminar
signatures-last-update-failed = A última atualização falhou
signatures-no-completed-update = Nenhuma atualização concluída registrada ainda
signatures-updater-updated = Atualizado pelo { $updater }
signatures-provider-fetched = Obtido de { $provider }
signatures-count-signature-size-file =
    { $count ->
        [one] { $count } assinatura, { $size } ({ $file })
       *[other] { $count } assinaturas, { $size } ({ $file })
    }
signatures-file-unreadable = Número de assinaturas desconhecido, não foi possível ler o arquivo: { $error }, { $size } ({ $file })
signatures-other-signature-sources = Outras fontes de assinaturas
signatures-unknown-reason-database = Desconhecido. { $reason }
signatures-fetch-newest-signatures = Baixar agora as assinaturas mais recentes de todos os atualizadores
signatures-manual-update = Atualização manual
signatures-update-service = Serviço de atualização
signatures-database-folder = Pasta do banco de dados
signatures-updater-output = Saída do atualizador
signatures-current-label = Assinaturas atualizadas
signatures-signature-mechanism = As assinaturas descrevem malwares conhecidos, e novas são publicadas constantemente. Assinaturas antigas deixam passar ameaças recentes, mesmo quando todo o resto está em execução.
signatures-updating-signatures = Atualizando as assinaturas…
signatures-updater-running-now = Em execução
signatures-update-cancelling = Cancelando a atualização…
signatures-starting-update = Iniciando a atualização…
signatures-daemon-running = Ativo
signatures-timer-scheduled = Agendado por um temporizador do systemd
signatures-service-stopped = Instalado, mas parado
signatures-service-not-found = Nenhum serviço freshclam detectado; as atualizações são executadas manualmente
signatures-systemd-unreachable-service = Não foi possível consultar o systemd sobre um serviço freshclam; as atualizações são executadas manualmente
signatures-count-day-old =
    { $count ->
        [one] { $count } dia de idade
       *[other] { $count } dias de idade
    }
signatures-definitions-missing = Nenhum banco de dados de vírus legível foi encontrado
signatures-directory-unreadable = Desconhecido. { $reason }
signatures-scanner-error = O freshclam falhou com o código de saída { $code }.
signatures-killed-signal = O freshclam foi interrompido pelo sinal { $signal }.
signatures-unrecognized-wait-status = O freshclam terminou de uma forma que o { -brand-name } não reconhece.
signatures-wait-failed = O { -brand-name } perdeu o acompanhamento do freshclam antes que ele terminasse.
signatures-authorization-refused = A autorização falhou. Tente novamente e insira sua senha.
signatures-systemd-not-reached = Não foi possível acessar o systemd para executar a atualização.
signatures-service-job-ended = O serviço de atualização não terminou; o systemd informou: { $result }
signatures-update-rate-limited-mirror = Atualização limitada pelo espelho. Aguarde alguns minutos e tente novamente.
signatures-update-blocked-cdn = Atualização bloqueada pela CDN. Isso pode ser devido a limitação de taxa. Aguarde e tente novamente mais tarde.
signatures-mirror-unavailable = O espelho do ClamAV está indisponível no momento. Tente novamente mais tarde.
signatures-certificate-rejected = Erro de certificado SSL/TLS. O espelho pode ter problemas de configuração.
signatures-connection-timed-out = A conexão expirou. Verifique sua conexão de rede.
signatures-database-locked = O banco de dados está bloqueado. Outra instância do freshclam pode estar em execução.
signatures-permission-denied = Permissão negada. Talvez você precise de privilégios elevados para atualizar o banco de dados.
signatures-connection-failed = Erro de conexão. Verifique sua conexão de rede.
signatures-name-resolution-failed = Falha na resolução de DNS. Verifique suas configurações de rede.
signatures-update-failed-unknown-error = A atualização falhou com um erro desconhecido. Verifique os registros para obter detalhes.
signatures-updated-databases = Atualizado: { $databases }
signatures-already-current-databases = Já atual: { $databases }
signatures-cooldown-unknown = Limitado: { $database }
signatures-cooldown-until = Limitado: { $database } até { $cooldown }
signatures-definitions-updated = Atualização do banco de dados concluída
signatures-already-current = Banco de dados já está atualizado
signatures-service-ran-without-verdict = O serviço de atualização foi executado, mas o registro dele não diz se as definições mudaram.
signatures-update-requested-daemon = O serviço aceitou o sinal; ele executa a atualização de forma assíncrona. Verifique os logs para confirmar a conclusão.
signatures-update-cancelled = Atualização do banco de dados cancelada
signatures-scanner-program-missing = Instale o freshclam antes de solicitar atualizações do banco de dados.
signatures-provider-needs-credential-fangfrisch =
    { $credential ->
        [purchase-receipt] { $provider } precisa de um número de recibo de compra, e o fangfrisch só o lê das suas configurações. No NixOS, services.clamav.fangfrisch.settings é copiado para o Nix store, que todos os usuários deste computador podem ler, então o número de recibo de compra ficaria exposto. Deixe { $provider } desativado ou execute o fangfrisch com um arquivo de configurações mantido fora do Nix store, como um segredo do agenix ou do sops-nix.
       *[customer-identifier] { $provider } precisa de um ID de cliente, e o fangfrisch só o lê das suas configurações. No NixOS, services.clamav.fangfrisch.settings é copiado para o Nix store, que todos os usuários deste computador podem ler, então o ID de cliente ficaria exposto. Deixe { $provider } desativado ou execute o fangfrisch com um arquivo de configurações mantido fora do Nix store, como um segredo do agenix ou do sops-nix.
    }
signatures-provider-needs-credential-website =
    { $credential ->
        [purchase-receipt] { $provider } precisa de um número de recibo de compra obtido no site dele. O fangfrisch o lê do seu arquivo de configurações, geralmente { $path }. Adicione estas linhas lá, substitua o marcador pelo seu número de recibo de compra e deixe o arquivo legível somente pelo root, para que outros usuários não possam vê-lo.
       *[customer-identifier] { $provider } precisa de um ID de cliente obtido no site dele. O fangfrisch o lê do seu arquivo de configurações, geralmente { $path }. Adicione estas linhas lá, substitua o marcador pelo seu ID de cliente e deixe o arquivo legível somente pelo root, para que outros usuários não possam vê-lo.
    }
signatures-not-use = Não está em uso
signatures-add-lines-fangfrisch-settings = Adicione estas linhas ao arquivo de configurações do fangfrisch, geralmente { $path }.
signatures-provider-needs-free-abuse = { $provider } precisa de uma Auth-Key gratuita do abuse.ch. Adicione estas linhas ao arquivo de configurações do fangfrisch, geralmente { $path }, substitua o marcador pela sua chave e torne o arquivo legível apenas pelo root, para que outros usuários não possam vê-la.
signatures-use-label = Em uso
signatures-unknown-reason-source = Desconhecido. { $reason }
signatures-check-again-signature-tooltip = Verificar novamente
signatures-checking-signature-tooltip = Verificando
signatures-clamav-malware-signatures-description = De onde vêm as assinaturas de malware do ClamAV e quais outras fontes você poderia adicionar.
signatures-sources-title = Fontes de assinaturas
signatures-abuse-ch-key-title = Chave do abuse.ch
signatures-abuse-ch-auth-title = Auth-Key do abuse.ch
signatures-abuse-ch-key-uses = Usada para as assinaturas do URLhaus e para consultas e envios ao MalwareBazaar.
signatures-abuse-ch-no-key = Sem chave
signatures-abuse-ch-key-unreadable = Chave ilegível
signatures-command-copied-clipboard-signature = Comando copiado para a área de transferência
signatures-not-open-documentation-title = Não foi possível abrir a documentação: { $error }
signatures-checking-api-key = Verificando a chave de API…
signatures-check-passed = Aprovado
signatures-needs-review = Aviso
signatures-check-failed = Falhou
signatures-status-unknown = Desconhecido
signatures-check-skipped = Ignorado
signatures-check-progress = Verificando
signatures-copy-tooltip = Copiar
signatures-learn-more-title = Saiba mais
signatures-open-browser-tooltip = Abrir no navegador
signatures-checking-title = Verificando…

preferences-launcher-not-found = O { -brand-name } não conseguiu encontrar o próprio lançador: { $error }
preferences-action-unwritable = Não foi possível adicionar a ação do gerenciador de arquivos: { $error }
preferences-action-unremovable = Não foi possível remover a ação do gerenciador de arquivos: { $error }
preferences-dialog-title = Preferências
preferences-general-title = Geral
preferences-window-behavior-title = Comportamento da janela
preferences-detection-alerts-work-description = Os alertas de detecção só funcionam enquanto o { -brand-name } está em execução.
preferences-closing-window-title = Ao fechar a janela
preferences-start-background-title = Iniciar em segundo plano
preferences-launch-background-without-subtitle = Iniciar em segundo plano sem abrir uma janela.
preferences-desktop-notifications-title = Notificações da área de trabalho
preferences-notify-detections-finished-subtitle = Notificar sobre detecções e sobre verificações e atualizações concluídas enquanto a janela não estiver em foco.
preferences-file-manager-integration-title = Integração com gerenciador de arquivos
preferences-add-context-menu-description = Adicione ações ao menu de contexto dos gerenciadores de arquivos compatíveis.
preferences-online-services-title = Serviços on-line
preferences-online-services-summary = Consultar envia apenas um hash ou um nome de assinatura. Um arquivo só sai deste computador quando você o envia.
preferences-circl-receives-summary = Recebe o hash SHA-256 de um arquivo e informa se é um arquivo conhecido e inofensivo. Não precisa de chave.
preferences-talos-receives-summary = Recebe os arquivos que você denuncia, com seu nome e endereço de e-mail. A Talos nunca os torna públicos. Não precisa de chave.
preferences-virustotal-receives-summary = Recebe o hash SHA-256 de um arquivo. Os arquivos enviados são compartilhados com os clientes pagantes do VirusTotal.
preferences-malwarebazaar-receives-summary = Recebe um nome de assinatura ou o hash SHA-256 de um arquivo. Os arquivos enviados se tornam públicos.
preferences-key-file-not-permitted-hint = Torne-o legível para o seu usuário, por exemplo definindo o dono do segredo do agenix ou sops-nix.
preferences-key-file-missing-hint = Verifique o caminho do arquivo da chave na configuração do sistema.
preferences-key-file-unreadable-hint = Verifique se o caminho aponta para um arquivo comum.
preferences-key-not-valid-hint = Copie a chave inteira do site do serviço.
preferences-keyring-unavailable-hint = Desbloqueie seu chaveiro e abra as Preferências novamente.
preferences-stored-key-unreadable-hint = Exclua a chave e cole-a novamente.
preferences-virustotal-service-hint = Plano gratuito: 4 solicitações por minuto, 500 por dia.
preferences-abuse-ch-service-hint = Gratuita no Portal de Autenticação do abuse.ch.
preferences-not-configured = Não configurada
preferences-checking-label = Verificando…
preferences-set-system-configuration-title = Definida pela configuração do sistema
preferences-key-keyring-title = Chave no seu chaveiro
preferences-key-file-title = Arquivo da chave
preferences-key-unreadable-status = Ilegível
preferences-key-file-not-permitted = Não é possível ler o arquivo da chave: sem permissão
preferences-key-file-missing = O arquivo da chave não existe
preferences-key-file-unreadable = Não é possível ler o arquivo da chave
preferences-key-not-valid = Não é uma chave válida para este serviço
preferences-keyring-unavailable = O chaveiro está indisponível
preferences-stored-key-unreadable = Não é possível ler a chave do seu chaveiro
preferences-delete-key-label = Excluir chave
preferences-paste-api-key-title = Colar chave de API
preferences-get-free-api-title = Obter uma chave de API gratuita

widgets-clean-label = Limpo
widgets-scan-infected = Infectado
widgets-error-status = Erro
widgets-cancelled-label = Cancelado
widgets-skipped-label = Ignorado
widgets-success-label = Sucesso
widgets-up-date = Atualizado
widgets-requested-label = Solicitado
widgets-ran-label = Executado
widgets-counted-threats =
    { $count ->
        [one] { $count } ameaça encontrada
       *[other] { $count } ameaças encontradas
    }
widgets-uncounted-threats = O verificador informou uma ameaça sem indicar o arquivo.
widgets-no-threats-found-activity = Nenhuma ameaça encontrada
widgets-scan-failed = A verificação não terminou corretamente
widgets-scan-cancelled-activity = Verificação cancelada pelo usuário
widgets-scheduled-scan-skipped-battery = Verificação agendada ignorada (com bateria)
widgets-virus-definitions-updated = Definições de vírus atualizadas
widgets-virus-definitions-up-date = As definições de vírus estão atualizadas
widgets-update-requested-service = Atualização solicitada ao serviço
widgets-update-service-no-result = O serviço de atualização foi executado sem informar um resultado
widgets-update-failed = Falha na atualização do banco de dados
widgets-update-cancelled = Atualização do banco de dados cancelada
widgets-scan-log = REGISTRO DE VERIFICAÇÃO
widgets-update-log = REGISTRO DE ATUALIZAÇÃO
widgets-timestamp-label = Carimbo de data e hora
widgets-status-label = Status
widgets-duration-label = Duração
widgets-summary-label = Resumo
widgets-path-label = Caminho
widgets-full-output = Saída completa
widgets-scan-history-cannot-title = Não é possível ler o histórico de verificações
widgets-retry-label = Tentar novamente
widgets-copy-activity-tooltip = Copiar
widgets-log-details-copied-toast = Detalhes do registro copiados para a área de transferência
widgets-not-save-file-error = Não foi possível salvar o arquivo: { $error }
widgets-not-choose-file-error = Não foi possível escolher o arquivo: { $error }
widgets-chosen-file-not-local = { $location } não é um arquivo deste computador
widgets-export-title = Exportar
widgets-database-newer-release = Este banco de dados foi criado por uma versão mais recente do { -brand-name }: ele tem a versão de estrutura { $found }, e esta versão conhece até a { $known }. Atualize o { -brand-name } para abri-lo.
widgets-migration-step-failed = A atualização da estrutura do banco de dados falhou na etapa { $step }: { $error }
widgets-version-check-failed = Não foi possível verificar a versão da estrutura do banco de dados: { $error }
widgets-copy-configuration-line-title = Copiar linha de configuração
widgets-nothing-applied-choose-description = Nada é aplicado. Escolha um valor e copie a linha, depois adicione-a você mesmo à sua configuração.
widgets-unit-title = Unidade
widgets-line-copy-title = Linha a copiar
widgets-copy-line-label = Copiar linha
widgets-plain-bytes = bytes
widgets-kibibytes-unit = KiB
widgets-mebibytes-unit = MiB
widgets-unknown-option = O { -brand-name } não conhece esta opção, então apenas a mostra.
widgets-not-yes-no = Precisa de análise: este valor deve ser yes ou no.
widgets-not-whole-number = Precisa de análise: este valor deve ser um número inteiro.
widgets-count-out-range = Precisa de análise: este valor deve estar entre { $minimum } e { $maximum }.
widgets-not-size = Precisa de análise: este valor deve ser um tamanho em bytes ou um número seguido de K ou M.
widgets-path-not-absolute = Precisa de análise: este valor deve ser um caminho completo que começa com /.
widgets-empty-text = Precisa de análise: este valor está vazio.
widgets-apply-tooltip = Aplicar
widgets-unit-tooltip = Unidade
widgets-applying-restarting-clamav-tooltip = Aplicando e reiniciando o ClamAV
widgets-detect-potentially-unwanted-programs = Detectar programas potencialmente indesejados
widgets-heuristic-detections = Detecções heurísticas
widgets-flag-office-files-macros = Sinalizar arquivos do Office com macros
widgets-flag-encrypted-archives-documents = Sinalizar arquivos compactados e documentos criptografados
widgets-look-inside-archives = Examinar arquivos compactados
widgets-flag-files-too-large = Sinalizar arquivos grandes demais para verificação completa
widgets-adware-hacking-tools-remote = Adware, ferramentas de invasão e ferramentas de acesso remoto. Espere mais falsos positivos.
widgets-suspicious-files-no-signature = Arquivos suspeitos que nenhuma assinatura identifica, como executáveis corrompidos e truques de phishing.
widgets-macros-common-way-deliver = Macros são uma forma comum de distribuir malware, mas muitos documentos legítimos as usam.
widgets-clamav-cannot-look-inside = O ClamAV não consegue examinar o conteúdo deles, então eles podem esconder malware.
widgets-checks-files-inside-zip = Verifica os arquivos dentro de zip, rar e outros arquivos compactados.
widgets-otherwise-files-above-size = Caso contrário, arquivos acima dos limites de tamanho e recursão passam sem verificação e sem aviso.
widgets-clamd-settings = Configurações do clamd
widgets-access-scanning-settings = Configurações da verificação no acesso
widgets-update-settings = Configurações de atualização
widgets-change-snippet-copied = Copiado. Adicione-o à sua configuração para aplicá-lo.
widgets-stale-confirmation-dropped = A alteração não foi feita: as configurações foram bloqueadas novamente ou outra alteração começou antes.
widgets-settings-applied = Aplicado, mas ainda não está na configuração do sistema.
widgets-previous-restored = O ClamAV não voltou a funcionar bem com esse valor, então as configurações anteriores foram restauradas.
widgets-change-not-made-error = A alteração não foi feita: { $error }
widgets-authorization-refused = Nada foi alterado sem sua senha
widgets-admin-service-missing = Esta instalação não tem o serviço de administração do ClamAV, então as configurações do ClamAV não podem ser alteradas pelo { -brand-name }
widgets-change-setting = Alterar esta configuração?
widgets-cancel-engine = Cancelar
widgets-reduce-protection = Reduzir proteção
widgets-apply-label = Aplicar
widgets-loading-config = Lendo a configuração…
widgets-config-missing = Arquivo de configuração não encontrado
widgets-detection-title = Detecção
widgets-clamav-reports-every-description = O que o ClamAV informa em todas as verificações neste computador. O { -brand-name } passa as mesmas opções ao clamscan, para que os dois verificadores concordem.
widgets-no-settings-section-title = Nenhuma configuração nesta seção
widgets-journal-read-failed = Não foi possível ler o journal do sistema: { $error }
widgets-journal-reading-incomplete = A leitura do journal do sistema parou antes de terminar
widgets-folder-unreadable = Não foi possível ler a pasta de assinaturas: { $error }
widgets-reading-signature-folder-stopped = A leitura da pasta de assinaturas parou antes de terminar
widgets-entry-timestamp-invalid = Uma entrada do journal tem um carimbo de data/hora fora do intervalo suportado.
widgets-reader-thread-failed = Não foi possível iniciar o leitor do journal: { $error }
widgets-journal-reading-stopped = A leitura do journal do sistema parou. { $reason }
widgets-file-hash-mismatch = Hash do arquivo não corresponde — o arquivo pode estar corrompido
widgets-restore-refused-integrity-unknown = Este registro de quarentena não tem um hash utilizável, então o arquivo não pode ser verificado e não é restaurado. Você ainda pode excluí-lo.
widgets-upload-refused-integrity-unknown = Este registro de quarentena não tem um hash utilizável, então o { -brand-name } não pode provar que o arquivo é o mesmo que colocou em quarentena e não o envia a lugar nenhum.
widgets-payload-too-large-send = O arquivo é maior que { $size }, o máximo que o { -brand-name } envia.
widgets-blocked-file-other-filesystem = A verificação no acesso bloqueia este arquivo e ele está em outro sistema de arquivos, então não pode ser movido para a quarentena. Em vez disso, você pode excluí-lo.
widgets-entry-not-found = Entrada de quarentena não encontrada: { $id }
widgets-recording-failed-file-restored = Falha ao registrar a entrada de quarentena no banco de dados. O arquivo foi restaurado para o local original.
widgets-recording-failed-rollback = Falha ao registrar a entrada de quarentena no banco de dados. A reversão também falhou: { $error }. O arquivo pode ficar órfão em: { $path }
widgets-entry-record-not-removed = O arquivo foi movido, mas não foi possível remover o registro de quarentena dele. Ele ainda pode aparecer na lista.
widgets-system-call-failed = Uma chamada de sistema falhou: { $error }
widgets-file-operation-failed = Uma operação de arquivo falhou: { $error }
widgets-database-operation-failed = A operação no banco de dados da quarentena falhou: { $error }
widgets-blocking-step-interrupted = Uma etapa da quarentena em segundo plano parou inesperadamente: { $error }
widgets-path-contains-nul = O caminho contém um byte NUL
widgets-path-has-no-parent = O caminho não tem pasta pai
widgets-path-has-no-file = O caminho não tem nome de arquivo
widgets-database-path-not-utf8 = O caminho do banco de dados da quarentena não é UTF-8 válido
widgets-database-not-regular-file = O banco de dados da quarentena não é um arquivo regular
widgets-source-blocked-access = A verificação no acesso bloqueia a leitura do arquivo
widgets-moved-payload-still-blocked = O arquivo bloqueado continua ilegível após ser movido para a quarentena
widgets-source-not-regular-file = O arquivo não é um arquivo regular
widgets-payload-not-regular-file = O arquivo em quarentena não é um arquivo regular
widgets-not-enough-free-space = Não há espaço livre suficiente para colocar o arquivo em quarentena
widgets-file-replaced-during-operation = O arquivo foi substituído enquanto era movido, então não foi removido
widgets-path-outside-quarantine = O caminho não está dentro da pasta da quarentena
widgets-invalid-restore-path = O caminho de restauração não é válido
widgets-protected-restore-destination = Não é permitido restaurar em uma pasta protegida do sistema
widgets-stored-path-unreadable = Não foi possível ler um caminho armazenado no banco de dados da quarentena
widgets-target-missing = O alvo da verificação não existe ou contém um byte NUL inválido
widgets-target-not-scannable = O alvo da verificação não é um arquivo regular nem um diretório
widgets-target-directory-unreadable = Não foi possível listar o alvo da verificação { $path }: { $error }
widgets-file-list-unwritable = Não foi possível gravar a lista privada de arquivos do clamd: { $error }
widgets-enumeration-interrupted = A listagem do alvo da verificação parou inesperadamente.
widgets-no-files-scanned = Nenhum arquivo pôde ser verificado
widgets-unscanned-files =
    { $count ->
        [one] { $count } arquivo não foi verificado
       *[other] { $count } arquivos não foram verificados
    }
widgets-clamd-connection-lost = A conexão com o clamd foi perdida durante a verificação.
widgets-scanner-stopped-reporting = O verificador não informou um resultado para estes arquivos.
widgets-count-not-accessed-title =
    { $count ->
        [one] Não foi possível acessar { $count } arquivo
       *[other] Não foi possível acessar { $count } arquivos
    }
widgets-files-unreadable =
    { $count ->
        [one] Não foi possível ler { $count } arquivo
       *[other] Não foi possível ler { $count } arquivos
    }
widgets-nonfatal-warnings =
    { $count ->
        [one] { $count } aviso não fatal durante a verificação; alguns arquivos podem ter sido verificados apenas parcialmente
       *[other] { $count } avisos não fatais durante a verificação; alguns arquivos podem ter sido verificados apenas parcialmente
    }
widgets-count-threat-found-outcome =
    { $count ->
        [one] { $count } ameaça encontrada
       *[other] { $count } ameaças encontradas
    }
widgets-count-file-scanned-outcome =
    { $count ->
        [one] { $count } arquivo verificado
       *[other] { $count } arquivos verificados
    }
widgets-clamav-not-open-files = O ClamAV não conseguiu abrir estes arquivos.
widgets-unattributed-threats-only = O verificador informou uma ameaça sem indicar o arquivo.
widgets-preparing-scan = Preparando…
widgets-finding-files =
    { $count ->
        [one] Procurando arquivos… { $count } encontrado
       *[other] Procurando arquivos… { $count } encontrados
    }
widgets-trust-file = Confiar neste arquivo…
widgets-ignore-detection-for-me = Ignorar esta detecção para mim…
widgets-report-false-positive = Informar falso positivo…
widgets-skip-path-lazaret-scans = Ignorar este caminho nas verificações do { -brand-name }
widgets-copy-path = Copiar caminho
widgets-critical-severity = Crítico
widgets-high-severity = Alto
widgets-medium-severity = Médio
widgets-low-severity = Baixo
widgets-moved-quarantine-path-check = Movido para a quarentena: { $path }. Verifique a reputação dele na página Quarentena.
widgets-quarantine-detected-label = Quarentena
widgets-more-actions-tooltip = Mais ações
widgets-count-more-file-title =
    { $count ->
        [one] Mais { $count } arquivo, listado no relatório exportado
       *[other] Mais { $count } arquivos, listados no relatório exportado
    }
widgets-scan-details-title = Detalhes da verificação
widgets-quarantine-all =
    { $count ->
        [one] Colocar { $count } ameaça em quarentena
       *[other] Colocar { $count } ameaças em quarentena
    }
widgets-move-quarantine = Mover para a quarentena?
widgets-removes-each-original-file = Isso remove cada arquivo original do local atual.
widgets-cancel-scan = Cancelar
widgets-quarantine-label = Quarentena
widgets-detected-threats-title = Ameaças detectadas
widgets-problems-title = Problemas
widgets-move-path-quarantine = Mover '{ $path }' para a quarentena?
widgets-move-count-detected-file =
    { $count ->
        [one] Mover { $count } arquivo detectado para a quarentena?
       *[other] Mover { $count } arquivos detectados para a quarentena?
    }
widgets-path-copied-clipboard = Caminho copiado para a área de transferência
widgets-file-skipped-future-scans = Este arquivo será ignorado em verificações futuras
widgets-exclusion-already-exists = Esta exclusão já existe
widgets-results-copied-clipboard = Resultados copiados para a área de transferência
widgets-results-exported = Resultados exportados
widgets-done-total-file-scanned =
    { $total ->
        [one] { NUMBER($done) } de { $total } arquivo verificado
       *[other] { NUMBER($done) } de { $total } arquivos verificados
    }
widgets-count-file-scanned-results =
    { $count ->
        [one] { $count } arquivo verificado
       *[other] { $count } arquivos verificados
    }
widgets-count-entry-skipped-exclusions =
    { $count ->
        [one] { $count } entrada ignorada pelas exclusões
       *[other] { $count } entradas ignoradas pelas exclusões
    }
widgets-threats-detected-status = Ameaças detectadas
widgets-no-threats-detected-status = Nenhuma ameaça encontrada
widgets-finished-problems-status = A verificação não terminou corretamente
widgets-scan-cancelled-status = Verificação cancelada pelo usuário
widgets-count-threat-found-results =
    { $count ->
        [one] { $count } ameaça encontrada
       *[other] { $count } ameaças encontradas
    }
widgets-files-scanned = Arquivos verificados
widgets-files-planned = Arquivos planejados
widgets-directories-label = Diretórios
widgets-threats-found = Ameaças encontradas
widgets-file-no-longer-quarantine = O arquivo não está mais lá. Se estiver na quarentena, confie nele pela página Quarentena.
widgets-permission-denied = Não é possível ler o arquivo, possivelmente porque a verificação no acesso o bloqueia. Mova-o para a quarentena e depois confie nele pela página Quarentena.
widgets-file-not-read-scan = Não foi possível ler o arquivo: { $error }
widgets-threats-detected-finished = Ameaças detectadas
widgets-no-threats-detected-finished = Nenhuma ameaça encontrada
widgets-finished-problems-notify = A verificação não terminou corretamente
widgets-scanner-error = O verificador parou com o código de erro { $code }.
widgets-killed-signal = O verificador foi interrompido pelo sinal { $signal }.
widgets-unrecognized-wait-status = O verificador terminou de uma forma que o { -brand-name } não reconhece (status de espera { $status }).
widgets-wait-failed = O { -brand-name } perdeu o acompanhamento do processo do verificador: { $error }.
widgets-scanner-program-missing = O verificador do ClamAV não está instalado ou não está no caminho de pesquisa.
widgets-scanner-program-replaceable = O { -brand-name } se recusou a executar { $program }, porque sua conta pode substituir { $replaceable }. Instale o ClamAV para todo o sistema, para que um programa plantado por malware não possa falsificar resultados limpos.
widgets-scanner-spawn-failed = Não foi possível iniciar o verificador: { $error }
widgets-scanner-work-directory-unavailable = Não foi possível criar o diretório de trabalho privado do verificador: { $error }
widgets-scanner-output-uncaptured = Não foi possível capturar a saída do verificador.
widgets-detection-settings-unreadable = A verificação não começou, porque não foi possível ler as configurações de detecção do clamd: { $error }
widgets-program-lookup-failed = A verificação não começou, porque não foi possível localizar os programas de verificação: { $error }
widgets-scan-input-unavailable = Não foi possível abrir com segurança a entrada da verificação { $path }: { $error }
widgets-detections-warnings-errors-tooltip = Somente detecções, avisos e erros
widgets-copy-service-tooltip = Copiar
widgets-export-text-tooltip = Exportar texto
widgets-search-log-placeholder = Pesquisar no registro
widgets-reading-log = Lendo o registro…
widgets-no-log-lines-match = Nenhuma linha do registro corresponde
widgets-log-copied-clipboard = Registro copiado para a área de transferência
widgets-text-export-finished = Exportado.
widgets-threat-detected = Detecção
widgets-emergency-label = Emergência
widgets-alert-label = Alerta
widgets-critical-level = Crítico
widgets-error-level = Erro
widgets-warning-label = Aviso
widgets-notice-label = Observação
widgets-info-label = Informação
widgets-debug-label = Depuração
widgets-log-reading = Verificando…
widgets-system-journal = Journal do sistema
widgets-neither-log-file-nor = Não foram encontrados nem um arquivo de registro nem linhas do journal.
widgets-no-log-file-found = Nenhum arquivo de registro foi encontrado. { $reason }
widgets-program-use = { $program } em uso
widgets-ignored-program = { $program } ignorado
widgets-refused-program = { $program } recusado
widgets-scanner-programs-trusted = Os programas de verificação são confiáveis
widgets-freshclam-trusted = O freshclam é confiável
widgets-malware-running-clamdscan = Um malware executado como você poderia colocar um clamdscan falso antes no seu caminho de pesquisa que informa todos os arquivos como limpos. O { -brand-name } só executa programas do ClamAV que sua conta não pode substituir, como os instalados para todo o sistema.
widgets-malware-running-freshclam = Um malware executado como você poderia colocar um freshclam falso antes no seu caminho de pesquisa que finge baixar novas assinaturas. O { -brand-name } só executa programas do ClamAV que sua conta não pode substituir, como os instalados para todo o sistema.
widgets-check-interrupted = Desconhecido, porque a verificação dos programas do ClamAV parou antes de terminar
widgets-clamav-program-replaced-account = Um programa do ClamAV pode ser substituído pela sua conta, então o { -brand-name } se recusa a executá-lo
widgets-clamav-program-account-shadows = Um programa do ClamAV na sua conta encobre a cópia do sistema. O { -brand-name } o ignora, mas verifique de onde ele veio
widgets-count-settings-reduced-since =
    { $count ->
        [one] { $count } configuração verifica menos que o configurado desde { $time }
       *[other] { $count } configurações verificam menos que o configurado desde { $time }
    }
widgets-count-settings-reduced-unknown-time =
    { $count ->
        [one] { $count } configuração verifica menos que o configurado, desde um horário que não pôde ser lido
       *[other] { $count } configurações verificam menos que o configurado, desde um horário que não pôde ser lido
    }
widgets-changed-label = Alterado
widgets-change-time-unknown-override = Desconhecido
widgets-reduction-unknown-unread = Não se sabe se as configurações foram reduzidas, porque não foi possível ler as alterações em tempo de execução: { $error }
widgets-reduction-unknown-file = Não se sabe se as configurações foram reduzidas, porque o arquivo de alterações em tempo de execução não está no formato esperado. O ClamAV é executado com a configuração do sistema até que ele seja corrigido.
widgets-reduction-unknown-reading = Não se sabe se as configurações foram reduzidas, porque a leitura das alterações em tempo de execução parou antes de terminar
widgets-settings-changed-runtime = Configurações alteradas em tempo de execução
widgets-administrator-can-change = Alguém com a senha de administrador pode alterar as configurações do ClamAV em tempo de execução. O { -brand-name } não conseguiu ler essas alterações, então não sabe se elas verificam menos do que a sua configuração.
widgets-protection-reduced-purpose = Proteção reduzida de propósito
widgets-administrator-changed-runtime = Alguém com a senha de administrador alterou as configurações do ClamAV em tempo de execução de uma forma que verifica menos do que a sua configuração. As alterações do sistema na página responsável pela configuração mostram quem e quando.
widgets-base-configuration-changed-now = { $base } na sua configuração, { $changed } agora
widgets-administrator-changed-clamd = Alguém com a senha de administrador alterou as configurações do ClamAV em tempo de execução. Não foi possível ler o seu clamd.conf, então o { -brand-name } não sabe se essas alterações verificam menos do que a sua configuração. As alterações do sistema na página responsável pela configuração mostram quem e quando.
widgets-settings-changed-runtime-clamd = As configurações foram alteradas em tempo de execução, mas não foi possível ler o clamd.conf para compará-las
widgets-changed-now = { $changed } agora
widgets-daemon-awaited = Precisa do daemon, veja Mecanismo de verificação
widgets-unknown-run = Desconhecido. { $reason }
widgets-copy-timeline-tooltip = Copiar
widgets-moment-details = { $moment } · { $details }
widgets-why-matters-tooltip = Por que isso importa
widgets-what-is-sent-label = O que é enviado:
widgets-clamav-audience = A Cisco Talos, a equipe por trás do ClamAV, faz a análise. As amostras são processadas de forma automatizada, apenas a Talos pode acessá-las, e elas nunca são tornadas públicas nem compartilhadas com outras empresas. Uma análise leva pelo menos 48 horas, e o resultado aparece em uma versão posterior do banco de dados de assinaturas.
widgets-report-false-positive-clamav = Informar um falso positivo ao ClamAV?
widgets-file-content = O conteúdo do arquivo
widgets-detection-name = O nome da detecção { $detection }
widgets-report-missed-malware-clamav = Informar malware não detectado ao ClamAV?
widgets-name-email-address-talos = Seu nome e endereço de e-mail, para que a Talos possa entrar em contato com você
widgets-lazaret-sends-file-clamav = O { -brand-name } envia este arquivo ao ClamAV com o clamsubmit. Envie apenas arquivos que não contenham nada privado.
widgets-send-report-button = Enviar relatório
widgets-file-content-named-sha = O conteúdo do arquivo, identificado pelo hash SHA-256, não pelo nome
widgets-upload-malwarebazaar-heading = Enviar ao MalwareBazaar?
widgets-lazaret-uploads-file-malwarebazaar = O { -brand-name } envia este arquivo ao MalwareBazaar, mantido pelo abuse.ch.
widgets-tag-clamav-detection = A etiqueta { $tag }, da detecção do ClamAV
widgets-abuse-ch-key-sample = Sua chave do abuse.ch; a amostra é listada sem o nome da sua conta
widgets-malwarebazaar-public-anyone-download = O MalwareBazaar é público: qualquer pessoa pode baixar a amostra depois que ela for enviada, e um envio não pode ser desfeito. Envie apenas malware, nunca um documento que possa conter dados pessoais.
widgets-upload-publicly-button = Enviar publicamente
widgets-upload-virustotal-heading = Enviar ao VirusTotal?
widgets-lazaret-uploads-file-virustotal = O { -brand-name } envia este arquivo ao VirusTotal para análise.
widgets-virustotal-key-links-upload = Sua chave do VirusTotal, que vincula o envio à sua conta
widgets-virustotal-shares-uploaded-files = O VirusTotal compartilha os arquivos enviados com seus clientes pagantes, que podem baixá-los, e qualquer pessoa pode consultar o resultado pelo hash do arquivo. Nunca envie um documento que possa conter dados pessoais.
widgets-upload-button = Enviar
widgets-cancel-submission = Cancelar
widgets-name-title = Seu nome
widgets-email-address-title = Seu endereço de e-mail
widgets-file-unreadable = Não foi possível ler o arquivo: { $error }
widgets-file-not-regular = Apenas arquivos regulares podem ser enviados
widgets-file-too-large = O arquivo é maior que { $size }, o máximo que o { -brand-name } envia.
widgets-reading-interrupted = A leitura do arquivo parou inesperadamente
widgets-history-failure = Não foi possível ler o histórico de envios: { $error }
widgets-earlier-submission = Este arquivo já foi enviado em { $local-time }. Nada foi enviado novamente.
widgets-report-recorded = Relatório enviado. A análise do ClamAV leva pelo menos 48 horas; Amostras enviadas mostra quando uma atualização de assinaturas muda o veredito.
widgets-report-not-sent-error = O relatório não foi enviado: { $error }
widgets-report-sent-without-record = Relatório enviado, mas não foi possível adicioná-lo a Amostras enviadas: { $error }
widgets-nothing-uploaded-error = Nada foi enviado. { $reason }
widgets-bazaar-key-not-usable = O MalwareBazaar não tem uma chave utilizável.
widgets-virustotal-key-not-usable = O VirusTotal não tem uma chave utilizável.
widgets-malwarebazaar-refused-sample-reason = O MalwareBazaar recusou a amostra: { $reason }
widgets-malwarebazaar-sample-sent-samples = O MalwareBazaar tem a amostra. Amostras enviadas tem um link para a entrada pública dela.
widgets-not-detected-clamav = O MalwareBazaar só aceita arquivos detectados por uma assinatura do ClamAV
widgets-unwanted-program-only = O MalwareBazaar não aceita programas potencialmente indesejados, apenas malware
widgets-sent-without-record = Enviado, mas não foi possível adicioná-lo a Amostras enviadas: { $error }
widgets-virustotal-refused-file-reason = O VirusTotal recusou o arquivo: { $reason }
widgets-virustotal-analysing-file-refresh = O VirusTotal está analisando o arquivo. Atualizar na página Amostras enviadas mostra o veredito.
widgets-report-already-available =
    { $total ->
        [one] O VirusTotal já conhece este arquivo: { NUMBER($count) } de { $total } mecanismo o detecta. Nada foi enviado.
       *[other]
            { $count ->
                [one] O VirusTotal já conhece este arquivo: { NUMBER($count) } de { $total } mecanismos o detecta. Nada foi enviado.
               *[other] O VirusTotal já conhece este arquivo: { NUMBER($count) } de { $total } mecanismos o detectam. Nada foi enviado.
            }
    }
widgets-known-user = { $local-time } · { $outcome } · solicitado pelo usuário { $uid }
widgets-unknown-user = { $time } · { $outcome } · solicitado por um usuário desconhecido
widgets-change-lowered-protection-tooltip = Esta alteração reduziu a proteção
widgets-system-changes-failed = Não é possível ler o journal do sistema: { $error }
widgets-system-changes-title = Alterações do sistema
widgets-refresh-tooltip = Atualizar
widgets-system-changes-cannot-title = Não é possível ler as alterações do sistema
widgets-loading-system-changes = Carregando as alterações do sistema…
widgets-trust-file-heading = Confiar neste arquivo?
widgets-clamav-never-report-body =
    O ClamAV nunca mais informará um arquivo com exatamente este conteúdo, para nenhum usuário, onde quer que esteja neste computador e qualquer que seja a verificação que o encontre, e a verificação no acesso deixa de bloqueá-lo. Sua senha de administrador é necessária, e a solicitação de senha mostra o mesmo hash SHA-256 abaixo.

    { $path }
widgets-stop-showing-file-body =
    O { -brand-name } deixará de mostrar a você um arquivo com exatamente este conteúdo nas verificações que você executa, onde quer que esteja. O próprio ClamAV ainda o detecta, então a verificação no acesso ainda pode bloqueá-lo e alertar você, e outros usuários ainda o veem. Confiar para todos os usuários impede que o ClamAV o detecte para todos e precisa da sua senha de administrador.

    { $path }
widgets-trust-for-me-button = Confiar para mim
widgets-user-trust-unreadable = Não foi possível ler os arquivos e as detecções em que você confiou, então esta verificação mostra todas as detecções: { $error }
widgets-trust-for-all-users-button = Confiar para todos os usuários…
widgets-sha-256-hash-size =
    SHA-256 { $hash }
    { $size }
widgets-ignore-detection-heading = Ignorar esta detecção?
widgets-stop-showing-detection-body = O { -brand-name } deixará de mostrar a você todos os arquivos com esta detecção, não apenas este. O próprio ClamAV e outros usuários ainda os veem. Confiar no arquivo pelo hash é mais seguro quando apenas um arquivo é um falso positivo.
widgets-ignore-button = Ignorar
widgets-stop-trusting-file-heading = Deixar de confiar neste arquivo?
widgets-clamav-report-file-body = O ClamAV voltará a informar este arquivo a todos os usuários quando ele corresponder a uma assinatura.
widgets-show-file-again-body = O { -brand-name } voltará a mostrar este arquivo a você quando o ClamAV o detectar.
widgets-stop-trusting-button = Deixar de confiar
widgets-report-detection-again-heading = Informar esta detecção novamente?
widgets-clamav-report-files-body = O ClamAV voltará a informar arquivos com esta detecção a todos os usuários.
widgets-show-detection-again-body = O { -brand-name } voltará a mostrar a você arquivos com esta detecção.
widgets-report-again-button = Informar novamente
widgets-cancel-trust = Cancelar

host-action-applied = Aplicado
host-action-rolled-back = Revertido
host-authorization-refused-described = Recusado sem senha
host-action-failed = Falhou
host-outcome-not-recorded = Resultado não registrado
host-unrecognized-outcome = Resultado não reconhecido { $outcome }
host-blocking-turned-off = Os arquivos detectados não serão mais bloqueados ao serem abertos. Eles serão apenas informados.
host-check-turned-off = O ClamAV deixará de verificar este tipo de arquivo ou ameaça neste computador.
host-watched-folder-removed = Os arquivos na pasta removida não serão mais verificados quando forem abertos.
host-skipped-location-added = O ClamAV ignorará completamente a nova entrada, então um malware ali passará despercebido.
host-scan-limit-lowered = Arquivos ou arquivos compactados acima do novo limite serão ignorados sem serem verificados.
host-current-values-unknown = Não foi possível ler o valor atual, então esta alteração pode reduzir a proteção.
host-detection-trusted = O ClamAV não informará mais o que você marcar como confiável aqui.
host-key-now-current-new =
    { $key }
    Agora: { $current }
    Novo: { $new }
host-system-records-change-reduced = O sistema registra que esta alteração reduziu a proteção, e a barra lateral continua sinalizando a página da configuração alterada até você desfazê-la.
host-trust-applied = O ClamAV carregou a alteração
host-trust-recorded-for-you = O { -brand-name } não mostra mais isto a você. O próprio ClamAV, o bloqueio no acesso e outros usuários ainda o veem.
host-applied-reload-pending = Salvo. O clamd não respondeu, então ele aplicará a alteração na próxima vez que recarregar as assinaturas
host-authorization-refused-outcome = Nada foi alterado sem sua senha
host-admin-service-missing = Esta instalação não tem o serviço de administração do ClamAV, então a lista de confiança do ClamAV não pode ser alterada pelo { -brand-name }
host-trust-failed = Nada foi alterado: { $error }
host-nixos-configuration = { $path } é gerado pela sua configuração do NixOS, então o { -brand-name } não pode alterá-lo. Use Copiar linha de configuração em uma configuração para copiar a linha dela para a sua configuração e depois recompile.
host-system-package = { $path } pertence ao sistema, então o { -brand-name } não pode alterá-lo. Use Copiar linha de configuração em uma configuração para copiar a linha a ser colocada nesse arquivo.
host-contact-incomplete = Insira seu nome e um endereço de e-mail válido para enviar um relatório
host-submitter-missing = Informar requer o clamsubmit, que vem com o ClamAV, mas não está instalado aqui
host-submitter-replaceable = O { -brand-name } se recusou a executar o clamsubmit, porque sua conta pode substituí-lo
host-report-not-sent-error = O relatório não foi enviado: { $error }
host-upload-timed-out = O relatório não foi enviado: o envio demorou demais
host-unrecognized-source = Outra fonte
host-file-progress =
    { $total ->
        [one] { NUMBER($done) } de { $total } arquivo
       *[other] { NUMBER($done) } de { $total } arquivos
    }
host-count-second-left =
    { $count ->
        [one] Cerca de { $count } segundo restante
       *[other] Cerca de { $count } segundos restantes
    }
host-count-minute-left =
    { $count ->
        [one] Cerca de { $count } minuto restante
       *[other] Cerca de { $count } minutos restantes
    }
host-count-hour-left =
    { $count ->
        [one] Cerca de { $count } hora restante
       *[other] Cerca de { $count } horas restantes
    }
host-seconds-s = { $seconds } s
host-minutes-min-seconds-s = { $minutes } min { $seconds } s
host-hours-h-minutes-min = { $hours } h { $minutes } min
host-blocked-access = Bloqueado pela verificação no acesso
host-exact-byte-size =
    { $count ->
        [one] { $size } ({ $count } byte)
       *[other] { $size } ({ $count } bytes)
    }
host-hash-unknown = Não está no banco de dados de arquivos conhecidos
host-listed-malicious-source-name = Listado como malicioso por { $source } (como { $name })
host-listed-malicious-unnamed-source-name = Listado como malicioso por uma fonte sem nome (como { $name })
host-listed-malicious-source = Listado como malicioso por { $source }
host-listed-malicious-unnamed-source = Listado como malicioso por uma fonte sem nome
host-listed-known-file-source = Listado como arquivo conhecido por { $source }
host-listed-known-file-unnamed-source = Listado como arquivo conhecido por uma fonte sem nome
host-known-file-name-listed = Arquivo conhecido { $name }, listado por { $source }
host-known-file-name-unnamed-source = Arquivo conhecido { $name }, listado por uma fonte sem nome
host-unnamed-family = família sem nome
host-no-known-samples = Não é uma amostra de malware conhecida
host-known-malware-sample = Amostra de malware conhecida
host-samples-found = Malware conhecido: { $family }, visto pela primeira vez em { $date }
host-samples-found-date-unknown = Malware conhecido: { $family }, visto pela primeira vez em data desconhecida
host-unnamed-family-samples-found = Malware conhecido de uma família sem nome, visto pela primeira vez em { $date }
host-unnamed-family-date-unknown = Malware conhecido de uma família sem nome, visto pela primeira vez em data desconhecida
host-file-unknown-virustotal = Desconhecido para o VirusTotal
host-count-total-engine-verdict =
    { $total ->
        [one] { NUMBER($count) } de { $total } mecanismo o detecta
       *[other]
            { $count ->
                [one] { NUMBER($count) } de { $total } mecanismos o detecta
               *[other] { NUMBER($count) } de { $total } mecanismos o detectam
            }
    }
host-api-key-rejected = O VirusTotal rejeitou a chave de API.
host-request-limit-reached = O limite de solicitações do VirusTotal foi atingido. Tente novamente mais tarde.
host-virustotal-answered-status = O VirusTotal respondeu com o status { $status }
host-virustotal-not-reached-error = Não foi possível acessar o VirusTotal: { $error }
host-virustotal-sent-answer-lazaret = O VirusTotal enviou uma resposta que o { -brand-name } não consegue ler
host-request-failed-circl = Não foi possível acessar o CIRCL hashlookup: { $error }
host-answer-unreadable-circl = O CIRCL hashlookup enviou uma resposta que não pôde ser lida
host-unexpected-status-circl = O CIRCL hashlookup respondeu com o status { $status }
host-request-failed-bazaar = Não foi possível acessar o MalwareBazaar: { $error }
host-answer-unreadable-bazaar = O MalwareBazaar enviou uma resposta que não pôde ser lida
host-request-limit-reached-bazaar = O limite de solicitações do MalwareBazaar foi atingido. Tente novamente mais tarde.
host-unexpected-status-bazaar = O MalwareBazaar respondeu com o status { $status }
host-key-refused = O MalwareBazaar recusou a chave de API
host-query-refused = O MalwareBazaar respondeu: { $status }
headless-count-threat-found =
    { $count ->
        [one] { $count } ameaça encontrada
       *[other] { $count } ameaças encontradas
    }
headless-open-lazaret-review = Abra o { -brand-name } para analisá-las.
headless-count-threats-quarantined =
    { $count ->
        [one] { $count } movida para a quarentena.
       *[other] { $count } movidas para a quarentena.
    }
headless-scan-infected = Ameaças detectadas
headless-scheduled-scan-failed = A verificação agendada falhou
headless-open-lazaret-read-log = Abra o { -brand-name } para ler o registro.
