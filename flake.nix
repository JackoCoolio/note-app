{
  description = "Rust project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: let
        rustToolchainFile = (pkgs.lib.importTOML ./rust-toolchain.toml).toolchain;
        rustToolchain = (
          inputs'.fenix.packages.fromToolchainName {
            name = rustToolchainFile.channel;
            sha256 = "sha256-AJ6LX/Q/Er9kS15bn9iflkUwcgYqRQxiOIL2ToVAXaU=";
          }
        );
        rust = rustToolchain.toolchain;
        rustPlatform = pkgs.makeRustPlatform {inherit (rustToolchain) rustc cargo;};
      in {
        packages.default = let
          packageDef = (pkgs.lib.importTOML ./Cargo.toml).package;
        in
          rustPlatform.buildRustPackage {
            pname = packageDef.name;
            version = packageDef.version;

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };

        devShells.default = pkgs.mkShell {
          # uncomment after running `cargo init`
          # inputsFrom = [self'.packages.default];
          nativeBuildInputs = [
            rust
            pkgs.bacon
          ];
        };

        formatter = pkgs.alejandra;
      };
      flake = {};
    };
}
