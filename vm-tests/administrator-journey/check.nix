{ machine, ... }:
{
  name = "lazaret-administrator-journey";

  nodes.machine = machine;

  testScript = builtins.readFile ../shared/desktop_session.py + builtins.readFile ./check.py;
}
