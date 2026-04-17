{ lib, pkgs, ... }:

pkgs.pkgsStatic.rustPlatform.buildRustPackage {
  pname = "m365-rs";
  version = "1.0.0";
  src = lib.cleanSource ./.;

  cargoHash = "sha256-RHctvPjA5LIq98JrD7UUocOeKrHEH6GYyftdyHHj7ws=";

  meta = with lib; {
    description = "Refresh M365 OAuth2 access token (Rust, statically compiled)";
    platforms = platforms.linux;
  };
}
