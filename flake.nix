{
  description = "Rust web app with Rocket and SQLite for meditation_app";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustup
            sqlite
            pkg-config
            openssl
          ];

          shellHook = ''
            export PATH=$HOME/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin:$PATH
            rustup default nightly
            echo "Rust nightly, SQLite, and dependencies are ready!"
          '';
        };
      }
    );
}                                                                                                                                                                                                                                                                                                                                                                                                                                                   
