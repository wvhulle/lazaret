let
  teamShareMountPoint = "/srv/team-share";
in
{
  virtualisation.fileSystems.${teamShareMountPoint} = {
    device = "fileserver:/export/team";
    fsType = "nfs";
    options = [
      "vers=4.2"
      "_netdev"
    ];
  };

  services.clamav.daemon.settings.OnAccessIncludePath = [ teamShareMountPoint ];
}
