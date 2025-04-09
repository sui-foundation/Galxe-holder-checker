{ self, lib, inputs, ... }:
{
  flake = {inputs', pkgs, ...}: {
    nixosConfigurations = {
      ghc = lib.nixosSystem {
        system = "x86_64-linux";
        modules = [
          ({...}: {
            config.ghc = {
              hostName = "ghc.ant-lab.tw";
              acmeContactMail = "antonio.yang@sui.io";
            };
          })
          (import ./server.nix {
            secrets = inputs.secrets;
          })
          self.nixosModules.ghcModule
          ({...}: { amazonImage.sizeMB = 30 * 1000; })
        ];
      };
    };
  };
}
