{ config, pkgs, lib, ... }:
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
      default = [ "usdc=0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7" ];
      description = "The package address for coin";
    };
    hostName = mkOption {
      type = types.str;
      description = "hostname for the server";
    };
    acmeContactMail = mkOption {
      type = types.str;
      description = "email for acme";
    };
  };

  config = mkIf cfg.enable {
    networking.hostName = "ghc";
    systemd.services.ghc = {
      enable = true;
      description = "Galxe holder checker service";
      restartIfChanged = true;
      unitConfig.X-StopOnRemoval = false;
      path = [ cfg.package ];
      script = ''
        ${cfg.package}/bin/Galxe-holder-checker ${concatMapStrings (o: " -c " + o) cfg.coinAddrs}
      '';
      serviceConfig = {
        WorkingDirectory = "~";
        Restart = "always";
        RestartSec = 10;
      };
      wantedBy = [ "multi-user.target" ];
    };
    services.nginx = {
      enable = true;
      virtualHosts."${cfg.hostName}" = {
        addSSL = true;
        enableACME = true;
        locations."/.well-known".extraConfig = ''
          root /var/www/${cfg.hostName};
        '';
        locations."/.well-known/acme-challenge" = {
          root = "/var/lib/acme/acme-challenge/.well-known/acme-challenge/";
        };
        locations = {
          "/".extraConfig = ''
            proxy_pass http://127.0.0.1:3000;
          '';
        };
      };
    };
    security.acme = {
      acceptTerms = true;
      defaults.email = cfg.acmeContactMail;
      certs.${cfg.hostName} = { };
    };
    environment.systemPackages = with pkgs; [
      nginx
    ];
    networking.firewall.allowedTCPPorts = [ 80 443 ];
    users.users.nginx.extraGroups = [ "acme" ];
  };
}
