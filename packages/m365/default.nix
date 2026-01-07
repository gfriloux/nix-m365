{ lib, pkgs, python3Packages, ... }:

python3Packages.buildPythonApplication {
  pname = "m365-refresh";
  src = ./src;

  propagatedBuildInputs = with python3Packages; [
    msal
  ];

  format = "other";

  installPhase = ''
    mkdir -p $out/bin
    install -m755 m365-refresh.py $out/bin/
  '';

  meta = with lib; {
    description = "Refresh oauth2 access token using MSAL";
    platforms = platforms.linux;
  };
}
