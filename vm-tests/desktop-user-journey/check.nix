{ machine, ... }:
{
  name = "lazaret-desktop-user-journey";

  nodes.machine = machine;

  testScript = builtins.readFile ../shared/run_journey.py + ''
    run_journey("desktop-user-journey")
  '';
}
