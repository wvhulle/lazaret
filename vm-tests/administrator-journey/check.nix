{ machine, ... }:
{
  name = "lazaret-administrator-journey";

  nodes.machine = machine;

  testScript = builtins.readFile ../shared/run_journey.py + ''
    run_journey("administrator-journey")
  '';
}
