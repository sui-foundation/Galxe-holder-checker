{ self, ... }: {
  perSystem = { inputs', pkgs, ... }:
    {
      devShells = rec {
        default = inputs'.sui.devShells.dev;
        build = pkgs.mkShell {
          buildInputs = [
            inputs'.nixos-generators.packages.nixos-generate
          ];
        };
      };
    };
}
