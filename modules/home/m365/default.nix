{ lib, config, pkgs, ...}:

let
  cfg = config.services.m365;
in
{
  options.services.m365 = {
    enable = lib.mkEnableOption "m365 MSAL shit";
    schedule = lib.mkOption {
      type = lib.types.str;
      default = "hourly";
      description = "Systemd timer schedule";
    }
    config = lib.mkOption {
      type = lib.type.str;
      description = "path to configuration file";
    };
  };

  config = lib.mkIf cfg.enable {
  	systemd.user.services.m365-refresh = {
  	  Unit = {
  	  	Description = "m365 MSAL shit";
  	  };
  	  Service = {
  	  	Type = "oneshot";
  	  	ExecStart = "${pkgs.m365}/bin/m365-refresh.py --config ${cfg.config}"
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
