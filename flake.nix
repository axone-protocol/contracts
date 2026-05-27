{
  description = "AXONE CosmWasm contracts development shell";

  inputs = {
    nixpkgs.url = "git+https://github.com/NixOS/nixpkgs?ref=nixpkgs-unstable";
    rust-overlay.url = "git+https://github.com/oxalica/rust-overlay";
  };

  outputs =
    { nixpkgs, rust-overlay, ... }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };

          rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };

          cargoTool =
            {
              pname,
              version,
              hash,
            }:
            rustPlatform.buildRustPackage rec {
              inherit pname version;

              src = pkgs.fetchCrate {
                inherit pname version hash;
              };

              cargoLock.lockFile = "${src}/Cargo.lock";
              doCheck = false;
            };

          cargo-cranky = cargoTool {
            pname = "cargo-cranky";
            version = "0.3.0";
            hash = "sha256-DBDpqaoHe01N2N7ldjtW8cbPCcQsbxTuDgjVi4EbKw8=";
          };

          cargo-toml-lint = cargoTool {
            pname = "cargo-toml-lint";
            version = "0.1.1";
            hash = "sha256-U3y9gnFvkqJmyFqRAUQorJQY0iRzAE9UUXzFmgZIyaM=";
          };

          cargo-llvm-cov = cargoTool {
            pname = "cargo-llvm-cov";
            version = "0.6.9";
            hash = "sha256-Lk1GwW2IGrwgnFahUIHi14q+T1N+bxE+mn2N+21SkmA=";
          };

          cosmwasm-check = cargoTool {
            pname = "cosmwasm-check";
            version = "3.0.2";
            hash = "sha256-jFtGSPBgL9scYbC7GbgAxqOinUTcDk9Uu7qCcYe63Nw=";
          };

          cargo-workspaces = cargoTool {
            pname = "cargo-workspaces";
            version = "0.3.6";
            hash = "sha256-JqLKFVM/EnVAPF7erINpHdaaDG+g2nbB0iE/hB1gml8=";
          };

          cargo-hack = cargoTool {
            pname = "cargo-hack";
            version = "0.6.14";
            hash = "sha256-RWYCESNNrB4eZGHGbbXAZJ+NhrRY5rImoAG7OFRPHZ0=";
          };

          cargo-machete = cargoTool {
            pname = "cargo-machete";
            version = "0.7.0";
            hash = "sha256-pnB9//n0EqAMonJCCjiftou/aIWNBj7LAtuhnNEdmmQ=";
          };

          cargo-sort-derives = cargoTool {
            pname = "cargo-sort-derives";
            version = "0.10.0";
            hash = "sha256-gb/+iaK4+P4wnDoiWQgeHBloks4L2Gx3tLoOS3Jk50M=";
          };

          darwinBuildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin (
            with pkgs.darwin.apple_sdk.frameworks;
            [
              CoreFoundation
              Security
              SystemConfiguration
            ]
          );
        in
        {
          default = pkgs.mkShell {
            packages = [
              rustToolchain
              pkgs.cargo-binstall
              pkgs.cargo-make
              pkgs.jq
              pkgs.nodejs
              pkgs.openssl
              pkgs.pkg-config
              pkgs.taplo
              cargo-cranky
              cargo-hack
              cargo-llvm-cov
              cargo-machete
              cargo-sort-derives
              cargo-toml-lint
              cargo-workspaces
              cosmwasm-check
            ] ++ darwinBuildInputs;

            env = {
              OPENSSL_DIR = "${pkgs.openssl.dev}";
              RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
            };
          };
        }
      );
    };
}
