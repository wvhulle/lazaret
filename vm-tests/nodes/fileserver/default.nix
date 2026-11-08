{
  lib,
  fixtureCorpus,
  xtask,
  ...
}:
let
  teamShareDirectory = "/export/team";
in
{
  system.stateVersion = "26.05";

  virtualisation.memorySize = 512;

  services.nfs.server = {
    enable = true;
    exports = ''
      ${teamShareDirectory} *(rw,no_root_squash,no_subtree_check,sync)
    '';
  };

  networking.firewall.allowedTCPPorts = [
    2049
    8081
  ];

  systemd.tmpfiles.rules = [ "d ${teamShareDirectory} 0775 1000 100 -" ];

  systemd.services.team-share-placement = {
    wantedBy = [ "multi-user.target" ];
    serviceConfig.ExecStart = lib.escapeShellArgs [
      (lib.getExe' xtask "xtask")
      "serve-team-share-placement"
      "--sample-directory"
      "${fixtureCorpus}/samples"
      "--share-directory"
      teamShareDirectory
      "--listen-address"
      "0.0.0.0"
      "--port"
      "8081"
    ];
  };
}
