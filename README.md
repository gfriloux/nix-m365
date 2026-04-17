# nix-m365

Based on the work of [UvA-FNWI/M365-IMAP](https://github.com/UvA-FNWI/M365-IMAP).
Initialisation part needs to be done using [UvA-FNWI/M365-IMAP](https://github.com/UvA-FNWI/M365-IMAP),
once you get :

- Client ID
- Client Secret
- Refresh Token
- Access Token

You can use this nix module.

Two implementations are available : a Python one (`m365`) and a Rust one (`m365-rs`).
The Rust version is statically compiled and has no runtime dependencies.

---

## m365 (Python)

### Configuration file

Example `config.py`:
```py
ClientId = open("/run/user/60021/imap_smtp_oauth2_client_id",'r').read()
ClientSecret = open("/run/user/60021/imap_smtp_oauth2_client_secret",'r').read()
Scopes = ['https://outlook.office.com/IMAP.AccessAsUser.All','https://outlook.office.com/SMTP.Send']
RefreshTokenFileName = "/home/user/.config/m365/imap_smtp_refresh_token"
AccessTokenFileName = "/home/user/.config/m365/imap_smtp_access_token"
Authority = None
```

### Home Manager module

```nix
services.m365-refresh = {
  enable = true;
  schedule = "hourly";
  config = "/home/user/.config/m365/config.py";
};
```

---

## m365-rs (Rust)

### Configuration file

Example `config.toml`:
```toml
client_id     = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
client_secret = "your-client-secret"
authority     = "https://login.microsoftonline.com/your-tenant-id"
refresh_token_file = "/home/user/.config/m365/imap_smtp_refresh_token"
access_token_file  = "/home/user/.config/m365/imap_smtp_access_token"
```

### Home Manager module

```nix
services.m365-rs-refresh = {
  enable = true;
  schedule = "hourly";
  config = /home/user/.config/m365/config.toml;
};
```
