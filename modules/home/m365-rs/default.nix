{
  lib,
  config,
  pkgs,
  ...
}: let
  cfg = config.services.m365-rs-refresh;
in {
  options.services.m365-rs-refresh = {
    enable = lib.mkEnableOption "m365 OAuth2 token refresh (Rust)";
    schedule = lib.mkOption {
      type = lib.types.str;
      default = "hourly";
      description = "Systemd timer schedule";
    };
    config = lib.mkOption {
      type = lib.types.path;
      description = "Path to TOML configuration file";
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.user.services.m365-rs-refresh = {
      Unit = {
        Description = "m365 OAuth2 token refresh (Rust)";
      };
      Service = {
        Type = "oneshot";
        ExecStart = "${pkgs.m365-rs}/bin/m365-refresh --config ${cfg.config}";
      };
    };

    systemd.user.timers.m365-rs-refresh = {
      Unit = {
        Description = "Timer for m365-rs-refresh";
      };
      Timer = {
        OnCalendar = cfg.schedule;
        Persistent = true;
      };
      Install = {
        WantedBy = ["timers.target"];
      };
    };
  };
}
