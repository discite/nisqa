let
  # Rolling updates, not deterministic.
  pkgs = import (fetchTarball ("channel:nixpkgs-unstable")) { config.allowUnfree = true; };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    pkgconfig
    openssl
    gcc
    cargo
    cargo-watch
    rustc
    rust-analyzer
    (vscode-with-extensions.override {
      vscodeExtensions = with vscode-extensions; [
        rust-lang.rust-analyzer
        tamasfe.even-better-toml
        vadimcn.vscode-lldb
      ];
    })
  ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
