{
  pkgs,
  fixtureCorpus,
  xtask,
  ...
}:
{
  system.stateVersion = "26.05";

  users.users.sample-user = {
    isNormalUser = true;
    uid = 1000;
    password = "sample-password";
    extraGroups = [
      "systemd-journal"
      "wheel"
      "ydotool"
    ];
  };

  services.clamav.daemon.enable = true;

  security.sudo.extraRules = [
    {
      users = [ "sample-user" ];
      commands = [
        {
          command = "ALL";
          options = [ "NOPASSWD" ];
        }
      ];
    }
  ];

  systemd.tmpfiles.rules = [
    "d /var/lib/clamav 0755 clamav clamav -"
    "C /var/lib/clamav/lazaret-fixture.hdb - - - - ${fixtureCorpus}/signatures/v1/lazaret-fixture.hdb"
    "C /var/lib/clamav/lazaret-fixture.ndb - - - - ${fixtureCorpus}/signatures/lazaret-fixture.ndb"
  ];

  environment = {
    etc."lazaret-test/samples".source = "${fixtureCorpus}/samples";
    systemPackages = [
      pkgs.curl
      pkgs.sqlite
      xtask
    ];
  };

  virtualisation = {
    memorySize = 3072;
    cores = 2;
    restrictNetwork = true;
  };
}
