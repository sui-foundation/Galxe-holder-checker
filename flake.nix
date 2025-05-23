{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
        rust-overlay.follows = "rust-overlay";
      };
    };
    sui = {
      url = "github:yanganto/sui/flake-testnet-v1.46.1";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
        rust-overlay.follows = "rust-overlay";
        crane.follows = "crane";
      };
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    secrets = {
      url = "github:yanganto/Sekreto";
    };
    nixos-generators = {
      url = "github:nix-community/nixos-generators";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig.extra-substituters = [
    "https://nix-community.cachix.org"
    "https://nix-cache.ant-lab.tw"
    "https://cache.garnix.io"
  ];
  nixConfig.extra-trusted-public-keys = [
    "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="
    "nix-cache.ant-lab.tw:zIdryBfFvXk6AyoaN8P5WWFELzDWOK7bQvIzl8nL5Y8="
  ];
  nixConfig.connect-timeout = 1;

  outputs = inputs @ { flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        ./nix/shell.nix
        ./nix/package.nix
        ./nix/machine.nix
        ./nix/module.nix
      ];
      systems = [
        "x86_64-linux"
      ];
      perSystem = { pkgs, system, ... }: {
        _module.args.pkgs = import inputs.nixpkgs {
          system = "x86_64-linux";
          overlays = [
            (_self: super: rec {
              craneLib = inputs.crane.mkLib pkgs;
            })
          ];
        };
      };
    };
}
