{ lib, pkgs, ... }:
let
  testKeyDirectory = "/var/lib/lazaret-test-keys";
  downloadsDirectory = "/home/sample-user/Downloads";
in
{
  imports = [
    ./.
    ../internet/client.nix
    ../fileserver/client.nix
  ];

  services.lazaret = {
    enable = true;
    apiKeyFiles.virustotal = "${testKeyDirectory}/virustotal";
  };

  services.clamav = {
    clamonacc.enable = true;
    daemon.settings = {
      OnAccessIncludePath = [ downloadsDirectory ];
      OnAccessPrevention = true;
      OnAccessExtraScanning = true;
      OnAccessMaxFileSize = "100M";
      SelfCheck = 10;
    };
    updater = {
      enable = true;
      settings = {
        ExcludeDatabase = [
          "main"
          "daily"
          "bytecode"
        ];
        DatabaseCustomURL = "https://signatures.fixture.test/lazaret-fixture.hdb";
      };
    };
    fangfrisch = {
      enable = true;
      sanesecurity.enable = false;
      urlhaus.authKeyFile = "${testKeyDirectory}/abuse-ch";
    };
  };

  systemd.tmpfiles.rules = [
    "d ${downloadsDirectory} 0755 sample-user users -"
    "d /home/sample-user/Documents 0755 sample-user users -"
    "d ${testKeyDirectory} 0755 root root -"
    "f+ ${testKeyDirectory}/abuse-ch 0440 root users - FIXTUREKEYMUSTNEVERAPPEAR"
    "f+ ${testKeyDirectory}/virustotal 0440 root users - 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
  ];

  systemd.user.services.lazaret.environment.GDK_DEBUG = "no-portals";

  systemd.services.clamav-fangfrisch.environment.REQUESTS_CA_BUNDLE =
    "/etc/ssl/certs/ca-certificates.crt";

  systemd.services.clamav-fangfrisch.serviceConfig.ExecStart =
    lib.mkForce "${lib.getExe pkgs.fangfrisch} --conf \${RUNTIME_DIRECTORY}/fangfrisch.conf refresh";

  environment.systemPackages = [ pkgs.glib ];
}
