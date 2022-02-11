{
  description = "Rust dev shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustpkg = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            targets = [ "thumbv7em-none-eabihf" ];
          });

      in with pkgs; {
        devShell = mkShell { buildInputs = [ flip-link probe-run rustpkg ]; };
      });
}
