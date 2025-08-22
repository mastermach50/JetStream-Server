# shell.nix for AOD
{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  name = "AOD";
  buildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
    rust-analyzer

    pkg-config
    libxkbcommon
    xdotool
  ];

  LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
    stdenv.cc.cc.lib
  ];

  RUST_BACKTRACE = 1;
}
