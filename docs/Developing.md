# Devoloping ea

Although not required, we recommend using Nix flake as part of `ea`'s development process: to replicate the exact development environment as the maintainers, run `nix shell`, or adding `use flake` in an [direnv][] `.envrc` file.

If you update `Cargo.toml` in any way, you will have to update `flake.nix`, this is enforced by checks in continuous integration.

If you don't use Nix, see  `buildInputs` in `flake.nix` for the tools necassary to the development.

[direnv]: https://direnv.net
