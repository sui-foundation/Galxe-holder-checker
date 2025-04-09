{ secrets }:
{ config, lib, pkgs, ... }:
{
  nix.settings.trusted-users = [ "@wheel" "root" ];
  users.users.root.openssh.authorizedKeys.keys = [
    secrets.ssh_keys.sf-ghc.pub
  ];

  environment.systemPackages = with pkgs; [
    neovim
    busybox
  ];
  networking.firewall.allowedTCPPorts = [ 80 443 ];
}
