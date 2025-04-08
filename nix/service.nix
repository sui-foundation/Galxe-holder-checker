{ config, lib, ... }:
with lib;
let
  cfg = config.ghc;
in
{
  options.ghc = {
    enable = mkEnableOption "Galxe holder checker service for campaign";
    package = mkOption {
      type = types.package;
      description = "The Galxe holder checker service binary";
    };
  };

  config = mkIf cfg.enable {
    systemd.services.ghc = {
    enable = true;
    description = "Galxe holder checker service";
    restartIfChanged = true;
    unitConfig.X-StopOnRemoval = false;
    path = [ cfg.package ];
    script = ''
      ${cfg.package}/bin/ghc
    '';
    serviceConfig = {
      WorkingDirectory = "~";
      Restart = "always";
      RestartSec = 10;
    };
    wantedBy = [ "multi-user.target" ];
    };
  };
}
