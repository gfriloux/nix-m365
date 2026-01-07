{ lib, config, pkgs, ...}:

let
  cfg = config.services.m365;
in
{
  options.services.m365-refresh = {
    enable = lib.mkEnableOption "m365 MSAL shit";
    schedule = lib.mkOption {
      type = lib.types.str;
      default = "hourly";
      description = "Systemd timer schedule";
    }
  };

  config = lib.mkIf cfg.enable {
  	systemd.user.services.m365-refresh = {
  	  Unit = {
  	  	Description = "m365 MSAL shit";
  	  };
  	  Service = {
  	  	Type = "oneshot";
  	  	ExecStart = "${pkgs.m365}/bin/m365-refresh.py"
  	  };
  	};

  	systemd.user.timers.m365-refresh = {
  	  Unit = {
  	  	Description = "Timer for m365-refresh";
  	  };
  	  Timer = {
  	  	OnCalendar = cfg.schedule;
  	  	Persistent = true;
  	  };
  	  Install = {
  	  	WantedBy = [ "timers.target" ];
  	  };
  	};
  };
}
