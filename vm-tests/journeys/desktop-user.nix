{
  name = "lazaret-desktop-user-journey";

  nodes = {
    machine = ../nodes/desktop/desktop-user.nix;
    internet = ../nodes/internet;
    fileserver = ../nodes/fileserver;
  };

  testScript = builtins.readFile ../run_journey.py + ''
    internet.start()
    fileserver.start()
    fileserver.wait_for_unit("nfs-server.service")
    fileserver.wait_for_unit("team-share-placement.service")
    machine.start()
    internet.wait_for_unit("fake-internet.service")
    wait_for_desktop_session()
    machine.wait_for_unit("srv-team\\x2dshare.mount")
    for listening_port in (443, 8080):
        machine.wait_for_open_port(listening_port, addr="internet", timeout=timedelta(seconds=60))
    for listening_port in (2049, 8081):
        machine.wait_for_open_port(listening_port, addr="fileserver", timeout=timedelta(seconds=60))
    run_journey("desktop-user-journey")
    with subtest("nothing knocked on a closed port of the fake internet"):
        refused_connections = [
            kernel_line for kernel_line in internet.succeed("dmesg").splitlines() if "refused connection" in kernel_line
        ]
        assert not refused_connections, "\n".join(refused_connections)
  '';
}
