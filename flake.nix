{
  description = "My cute Rust crate!";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, naersk, fenix, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rust-toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "Miyx2cevxtP/Ia2HB9HVN6Z5eT8ITFcoQFvgiK7jVTY=";
        };
        naersk-lib = naersk.lib.${system}.override {
          cargo = rust-toolchain;
          rustc = rust-toolchain;
        };
      in rec {
        # `nix build`
        packages = {
          hak = naersk-lib.buildPackage {
            pname = "hak";
            root = ./.;
          };
        };

        defaultPackage = packages.hak;

        # `nix develop`
        devShell = pkgs.mkShell { nativeBuildInputs = [ rust-toolchain ]; };
      });
}
