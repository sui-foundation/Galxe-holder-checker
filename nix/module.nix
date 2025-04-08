{ self, inputs, lib, ... }:
{
  flake = _: {
    nixosModules = {
      ghcModule = { pkgs, lib, ... }: {
        imports = [
          ./service.nix
        ];
        ghc = {
          enable = true;
          package = self.packages.${pkgs.hostPlatform.system}.ghc;
        };
      };
    };
  };
}
