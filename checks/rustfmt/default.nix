{ pkgs, ... }:
pkgs.runCommand "rustfmt-check" {
  nativeBuildInputs = [ pkgs.rustfmt ];
} ''
  rustfmt --check ${../../packages/m365-rs/src/main.rs}
  touch $out
''
