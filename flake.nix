{
  description = "jj-lsp";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };
        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-SDu4snEWjuZU475PERvu+iO50Mi39KVjqCeJeNvpguU=";
        };
        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };
      in {
        packages.default = rustPlatform.buildRustPackage {
          pname = "jj-lsp";
          version = "0.1.1-dev1";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          doCheck = true;
        };

        devShells.default = pkgs.mkShell {
          packages = [
            toolchain
          ];
        };
      });
}
