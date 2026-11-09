lazaretFlake:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  lazaretSettings = config.services.lazaret;
  apiKeyFileSettings = lib.filterAttrs (settingName: keyFilePath: keyFilePath != null) {
    virustotal-key-file = lazaretSettings.apiKeyFiles.virustotal;
    abuse-ch-key-file = lazaretSettings.apiKeyFiles.abuseCh;
  };
  isOutsideStore =
    keyFilePath: lib.hasPrefix "/" keyFilePath && !lib.hasPrefix builtins.storeDir keyFilePath;
  settingsPath = "page/codeberg/wvhulle/Lazaret";
  unsignedSettingNames = [ "device-scan-max-gigabytes" ];
  typedSettingValue =
    settingName: settingValue:
    if builtins.isInt settingValue then
      if builtins.elem settingName unsignedSettingNames then
        lib.gvariant.mkUint32 settingValue
      else
        lib.gvariant.mkInt32 settingValue
    else
      settingValue;
  declaredSettings = lib.mapAttrs typedSettingValue (lazaretSettings.settings // apiKeyFileSettings);
  declaredSettingsDatabase =
    pkgs.runCommand "lazaret-declared-settings"
      {
        nativeBuildInputs = [ (lib.getBin pkgs.dconf) ];
        keyfileText = lib.generators.toDconfINI { ${settingsPath} = declaredSettings; };
        lockText = lib.concatMapStrings (settingName: "/${settingsPath}/${settingName}\n") (
          builtins.attrNames declaredSettings
        );
        passAsFile = [
          "keyfileText"
          "lockText"
        ];
      }
      ''
        mkdir -p keyfiles/locks
        cp "$keyfileTextPath" keyfiles/lazaret
        cp "$lockTextPath" keyfiles/locks/lazaret
        dconf compile "$out" keyfiles
      '';
  profileScript = pkgs.writeShellScript "lazaret-dconf-profile" ''
    replacedProfile="''${DCONF_PROFILE:-/etc/dconf/profile/user}"
    case "$replacedProfile" in
      /*) ;;
      *) replacedProfile="/etc/dconf/profile/$replacedProfile" ;;
    esac
    profileDirectory="''${XDG_RUNTIME_DIR:-/tmp}/lazaret"
    mkdir -p "$profileDirectory"
    profileDraft=$(mktemp "$profileDirectory/dconf-profile.XXXXXX")
    {
      echo "user-db:user"
      echo "file-db:${declaredSettingsDatabase}"
      if [ -r "$replacedProfile" ]; then
        grep -v -e '^user-db:' -e '^file-db:${declaredSettingsDatabase}$' "$replacedProfile" || true
      fi
    } > "$profileDraft"
    mv "$profileDraft" "$profileDirectory/dconf-profile"
    echo "$profileDirectory/dconf-profile"
  '';
  declaresSettings = declaredSettings != { };
  launchedPackage =
    if declaresSettings then
      pkgs.symlinkJoin {
        name = "lazaret-with-declared-settings";
        paths = [ lazaretSettings.package ];
        nativeBuildInputs = [ pkgs.makeWrapper ];
        postBuild = ''
          wrapProgram $out/bin/lazaret --run 'export DCONF_PROFILE="$(${profileScript})"'
        '';
        meta.mainProgram = "lazaret";
      }
    else
      lazaretSettings.package;
in
{
  options.services.lazaret = {
    declaredSettingsDatabase = lib.mkOption {
      type = lib.types.package;
      readOnly = true;
      internal = true;
      default = declaredSettingsDatabase;
      description = "The compiled, locked dconf database of the declared settings.";
    };
    enable = lib.mkEnableOption "Lazaret in the background, so detections raise alerts";
    package = lib.mkOption {
      type = lib.types.package;
      default = lazaretFlake.packages.${pkgs.stdenv.hostPlatform.system}.default;
      defaultText = lib.literalExpression "lazaret.packages.\${pkgs.stdenv.hostPlatform.system}.default";
      description = "The Lazaret package to run.";
    };
    settings = lib.mkOption {
      type = lib.types.attrsOf (
        lib.types.oneOf [
          lib.types.bool
          lib.types.int
          lib.types.str
        ]
      );
      default = { };
      example = {
        close-action = "minimize";
        notifications-enabled = true;
      };
      description = ''
        Lazaret preferences, compiled into a read-only dconf database in the Nix store and
        locked. Lazaret starts through a wrapper that adds this database to its dconf profile,
        so these values show as set in your configuration and can only be changed here. Set API
        keys with `apiKeyFiles`.
      '';
    };
    apiKeyFiles = {
      virustotal = lib.mkOption {
        type = lib.types.nullOr lib.types.str;
        default = null;
        example = lib.literalExpression "config.age.secrets.virustotal-key.path";
        description = ''
          Runtime path of a file holding the VirusTotal API key, for example an agenix or
          sops-nix secret readable by your user. Only the path is stored, never the key. When set,
          Lazaret uses this file and doesn't offer to paste a key.
        '';
      };
      abuseCh = lib.mkOption {
        type = lib.types.nullOr lib.types.str;
        default = null;
        example = lib.literalExpression "config.age.secrets.abuse-ch-key.path";
        description = ''
          Runtime path of a file holding the abuse.ch Auth-Key used for MalwareBazaar lookups.
          Only the path is stored, never the key.
        '';
      };
    };
  };

  config = lib.mkIf lazaretSettings.enable {
    home.packages = [ launchedPackage ];
    assertions = lib.mapAttrsToList (settingName: keyFilePath: {
      assertion = isOutsideStore keyFilePath;
      message = "services.lazaret.apiKeyFiles: ${keyFilePath} must be an absolute path outside the Nix store, so the key never ends up in it";
    }) apiKeyFileSettings;
    systemd.user.services.lazaret = {
      Unit = {
        Description = "Lazaret detection alerts";
        PartOf = [ "graphical-session.target" ];
        After = [ "graphical-session.target" ];
        Requisite = [ "graphical-session.target" ];
      };
      Service = {
        ExecStart = "${lib.getExe launchedPackage} --background";
        Environment = [
          "PATH=%h/.nix-profile/bin:/etc/profiles/per-user/%u/bin:/run/current-system/sw/bin:/usr/local/bin:/usr/bin"
        ];
        Restart = "on-failure";
      };
      Install.WantedBy = [ "graphical-session.target" ];
    };
  };
}
