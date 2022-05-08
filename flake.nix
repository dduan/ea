{
  description = "Tree command, improved.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      with nixpkgs.legacyPackages.${system};
      let
        info = (fromTOML (builtins.readFile ./Cargo.toml)).package;
      in
      rec {
        packages = flake-utils.lib.flattenTree {
          ea = rustPlatform.buildRustPackage rec {
            pname = info.name;
            version = info.version;
            src = ./.;
            cargoSha256 = "sha256-YLjvhDVbeJLpWsSQmqxC/l8jSrOM2WIUxW/RqPJHgMI=";
            lockFile = ./Cargo.lock;
            nativeBuildInputs = [ installShellFiles pandoc ];
            preFixup = ''
              make manual
              installManPage docs/ea.1
            '';
          };
        };
        defaultPackage = packages.ea;
        devShell = pkgs.mkShell {
          buildInputs = [
            pandoc
            cargo
            clippy
            rust-analyzer
            rustc
          ] ++ pkgs.lib.lists.optionals stdenv.isDarwin [
            libiconv
          ];
        };
      });
}
