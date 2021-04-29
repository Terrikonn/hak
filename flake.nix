{
  description = "Project dev toolchain";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
          {
            devShell = pkgs.mkShell {
              buildInputs = with pkgs; [
                openssl
                pkg-config
                exa
                ripgrep
                tokei
                bat
                fd
                cargo-edit
                cargo-watch
                (
                  rust-bin.nightly.latest.minimal.override {
                    targets = [ "riscv64gc-unknown-none-elf" ];
                  }
                )
              ];

              shellHook = ''
                export RUSTC_WRAPPER="sccache"
                alias ls=exa
                cargo install cargo-binutils
              '';
            };
          }
    );
}
