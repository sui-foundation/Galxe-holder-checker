{ self, ... }: {
  perSystem = { inputs', pkgs, ... }:
    {
      devShells = rec {
        default = inputs'.sui.devShells.dev;
      };
    };
}
