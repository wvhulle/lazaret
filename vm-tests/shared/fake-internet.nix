{ pkgs, fixtureCorpus, ... }:
{
  networking.hostFiles = [ "${fixtureCorpus}/fake-internet-hosts" ];

  security.pki.certificateFiles = [ "${fixtureCorpus}/tls/fixture-ca.crt" ];

  systemd.services.fake-internet = {
    wantedBy = [ "multi-user.target" ];
    serviceConfig = {
      ExecStart = pkgs.lib.escapeShellArgs [
        pkgs.python3.interpreter
        ./fake_internet_server.py
        "--certificate"
        "${fixtureCorpus}/tls/fake-internet.crt"
        "--private-key"
        "${fixtureCorpus}/tls/fake-internet.key"
        "--state-directory"
        "/var/lib/fake-internet"
        "--signature-directory"
        "${fixtureCorpus}/signatures"
      ];
      StateDirectory = "fake-internet";
      Environment = "PYTHONUNBUFFERED=1";
    };
  };
}
