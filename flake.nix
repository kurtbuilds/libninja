{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      utils,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        defaultPackage =
          with pkgs;
          rustPlatform.buildRustPackage {
            name = "libninja";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
        devShell =
          with pkgs;
          mkShell {
            nativeBuildInputs = [
              pkg-config
              autoPatchelfHook
            ];
            buildInputs = [
              cargo
              rustc
              rustfmt
              rustPackages.clippy
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      }
    );
}
