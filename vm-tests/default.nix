{ lazaretModule, xtask }:
let
  machineDescriptions = {
    desktop-user-journey = "a desktop user's detections moving through quarantine, lookups, submissions, trust, a signature update and restore, against a fake internet";
    administrator-journey = "the root admin service: trust for all users, a system scan timer and live tuning, through real polkit prompts";
  };
in
{
  inherit machineDescriptions;

  checksFor =
    pkgs:
    let
      fixtureCorpus = import ./shared/fixture-samples.nix { inherit pkgs xtask; };

      machineModule = machineName: {
        imports = [
          lazaretModule
          ./shared/common.nix
          ./${machineName}/machine.nix
        ];
        _module.args = { inherit fixtureCorpus xtask; };
        networking.hostName = "lazaret-${machineName}";
      };
    in
    pkgs.lib.mapAttrs' (
      machineName: _machineDescription:
      pkgs.lib.nameValuePair "vm-${machineName}" (
        pkgs.testers.runNixOSTest (
          import ./${machineName}/check.nix {
            machine = machineModule machineName;
          }
        )
      )
    ) machineDescriptions;
}
