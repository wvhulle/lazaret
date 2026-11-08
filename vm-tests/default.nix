{ lazaretModule, xtask }:
{
  checksFor =
    pkgs:
    let
      fixtureCorpus = import ./fixture-corpus.nix { inherit pkgs xtask; };

      journeyTests = {
        desktop-user-journey = ./journeys/desktop-user.nix;
        administrator-journey = ./journeys/administrator.nix;
      };
    in
    pkgs.lib.mapAttrs' (
      journeyName: journeyTest:
      pkgs.lib.nameValuePair "vm-${journeyName}" (
        pkgs.testers.runNixOSTest {
          imports = [ journeyTest ];
          node.specialArgs = {
            inherit fixtureCorpus lazaretModule xtask;
          };
        }
      )
    ) journeyTests;
}
