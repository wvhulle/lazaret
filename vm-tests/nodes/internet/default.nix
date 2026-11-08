{
  lib,
  pkgs,
  fixtureCorpus,
  xtask,
  ...
}:
{
  system.stateVersion = "26.05";

  virtualisation.memorySize = 512;

  networking = {
    nftables.enable = true;
    firewall = {
      logRefusedConnections = true;
      allowedTCPPorts = [
        443
        8080
      ];
    };
  };

  systemd.services.fake-internet = {
    wantedBy = [ "multi-user.target" ];
    path = [ pkgs.nftables ];
    serviceConfig.ExecStart = lib.escapeShellArgs [
      (lib.getExe' xtask "xtask")
      "serve-fake-internet"
      "--certificate"
      "${fixtureCorpus}/tls/fake-internet.crt"
      "--private-key"
      "${fixtureCorpus}/tls/fake-internet.key"
      "--signature-directory"
      "${fixtureCorpus}/signatures"
      "--listen-address"
      "0.0.0.0"
      "--control-port"
      "8080"
    ];
  };
}
