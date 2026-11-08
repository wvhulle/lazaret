{
  pkgs,
  nodes,
  fixtureCorpus,
  ...
}:
let
  fakeInternetAddress = nodes.internet.networking.primaryIPAddress;
in
{
  networking.hostFiles = [
    (pkgs.runCommand "fake-internet-hosts" { } ''
      sed 's/^/${fakeInternetAddress} /' ${fixtureCorpus}/fake-internet-host-names > $out
    '')
  ];

  security.pki.certificateFiles = [ "${fixtureCorpus}/tls/fixture-ca.crt" ];
}
