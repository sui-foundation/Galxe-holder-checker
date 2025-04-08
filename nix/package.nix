{ self, ... }: {
  perSystem = { inputs', pkgs, ... }:
    {
      packages = {
        ghc = pkgs.rustPlatform.buildRustPackage {
          name = "ghc";
          src = self;
          cargoHash = "sha256-u3WF3JF3+giocvdFG2Tmn/GzqloXVkmOx84G5LvQFL8=";
          buildInputs = with pkgs; [ openssl ];
          nativeBuildInputs = with pkgs; [ pkg-config ];
        };
      };
    };
}
