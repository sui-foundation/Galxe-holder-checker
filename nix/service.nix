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
    coinAddrs= mkOption {
      type = lib.types.listOf types.str;
      default = [ "usdc=0x5d4b302506645c37ff133b98c4b50a5ae14841659738d6d733d59d0d217a93bf" ];
      description = "The package address for coin";
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
      ${cfg.package}/bin/ghc ${lib.strings.concatImapStrings (o: " -c " + o) cfg.coinAddrs}
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
