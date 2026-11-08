{ pkgs, xtask }:
pkgs.runCommand "lazaret-vm-fixture-corpus"
  {
    nativeBuildInputs = [
      xtask
      pkgs.openssl
      pkgs.zip
    ];
  }
  ''
    xtask build-vm-fixtures $out
  ''
