{
  description = "Tree command, improved.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      with nixpkgs.legacyPackages.${system};
      rec {
        packages = flake-utils.lib.flattenTree {
          ea = rustPlatform.buildRustPackage rec {
            pname = "ea";
            version = "0.1.0";
            src = ./.;
            cargoSha256 = "sha256-O3uIpQ+HynHVOw1mMUBOe0vgcQnxW9zKzLCzFbTU2Xg=";
            lockFile = ./Cargo.lock;
          };
        };
        defaultPackage = packages.ea;
        devShell = pkgs.mkShell {
          buildInputs = [
            cargo
            rust-analyzer
            rustc
          ];
        };
      });
}
