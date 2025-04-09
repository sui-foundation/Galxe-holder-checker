{ self, ... }: {
  perSystem = { inputs', pkgs, ... }:
    {
      packages = {
        ghc = pkgs.craneLib.buildPackage {
          name = "ghc";
          src = self;
          buildInputs = with pkgs; [ openssl ];
          nativeBuildInputs = with pkgs; [ pkg-config ];
        };
      };
    };
}
