{ lib, pkgs, ... }:
{
  imports = [ ./. ];

  services.lazaret = {
    enable = true;
    liveTuning = {
      allowedSettings = [
        "MaxThreads"
        "OnAccessMaxFileSize"
        "DetectPUA"
      ];
      reductionLifetime = 30;
    };
  };

  services.clamav = {
    daemon.settings = {
      MaxThreads = 4;
      DetectPUA = true;
    };
    scanner = {
      enable = true;
      interval = "Mon *-*-* 12:30:00";
      scanDirectories = [ "/var/lib/sample-scan" ];
    };
  };

  systemd.services.clamav-daemon.serviceConfig.ExecStartPre = [
    "${pkgs.bash}/bin/bash -c '! ${pkgs.gnugrep}/bin/grep -qx \"MaxThreads 3\" /etc/clamav/clamd.conf'"
  ];

  specialisation.newBase.configuration.services.clamav.daemon.settings.MaxThreads = lib.mkForce 11;

  systemd.tmpfiles.rules = [
    "d /var/lib/sample-scan 0755 root root -"
    "d /home/sample-user/Documents 0755 sample-user users -"
    "f+ /var/lib/clamav/lazaret-local.ign2 0644 root root - Doc.Fixture.FalsePositive-1"
    "f+ /var/lib/clamav/local.fp 0644 root root - 0123456789abcdef0123456789abcdef:12:Legacy.Hand.Written-1"
  ];

  environment.systemPackages = [
    pkgs.glib
    pkgs.util-linux
  ];
}
