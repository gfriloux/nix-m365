{ lib, pkgs, ... }:

pkgs.pkgsStatic.rustPlatform.buildRustPackage {
  pname = "m365-rs";
  version = "1.0.0";
  src = lib.cleanSource ./.;

  cargoHash = "sha256-+Lui49OizN1TXpC5BUpXt2g1jtyi6kRlj9I1Suh3g38=";

  meta = with lib; {
    description = "Refresh M365 OAuth2 access token (Rust, statically compiled)";
    platforms = platforms.linux;
  };
}
