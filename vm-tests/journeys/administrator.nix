{
  name = "lazaret-administrator-journey";

  nodes.machine = ../nodes/desktop/administrator.nix;

  testScript = builtins.readFile ../run_journey.py + ''
    start_all()
    wait_for_desktop_session()
    run_journey("administrator-journey")
  '';
}
