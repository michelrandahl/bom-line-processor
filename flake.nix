{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        from-rust-toolchain-file = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            pkg-config
            from-rust-toolchain-file
            rust-analyzer
            cargo-generate # for creating new crates based on a template
            cargo-binutils # tools for examining rust binaries (`cargo-size`, `cargo-strip`, `cargo-objdump`)
            gdb
          ];

          shellHook = ''
            export PS1="[\e[1;32mNix-rust-line-processor\e[0m] $PS1"
          '';
        };
      }
    );
}
