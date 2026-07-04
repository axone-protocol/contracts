{
  description = "AXONE Smart Contracts development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
  };

  outputs =
    { nixpkgs, ... }:
    let
      supportedSystems = [
        "aarch64-darwin"
        "x86_64-darwin"
        "x86_64-linux"
        "aarch64-linux"
      ];

      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            packages = [
              pkgs.cargo-binstall
              pkgs.cargo-make
              pkgs.gawk
              pkgs.git
              pkgs.gnumake
              pkgs.gnused
              pkgs.jq
              pkgs.nodejs_22
              pkgs.openssl
              pkgs.perl
              pkgs.pkg-config
              pkgs.rustup
              pkgs.taplo
            ];

            shellHook = ''
              echo "AXONE contracts development environment loaded"
            '';
          };
        }
      );
    };
}
