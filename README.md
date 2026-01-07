# nix-m365

Based on the work of [UvA-FNWI/M365-IMAP](https://github.com/UvA-FNWI/M365-IMAP).
Initialisation part needs to be done using [UvA-FNWI/M365-IMAP](https://github.com/UvA-FNWI/M365-IMAP),
once you get :

- Client ID
- Client Secret
- Refresh Token
- Access Token

You can use this nix module.

Exemple `config.py` file:
```py
ClientId = open("/run/user/60021/imap_smtp_oauth2_client_id",'r').read()
ClientSecret = open("/run/user/60021/imap_smtp_oauth2_client_secret",'r').read()
Scopes = ['https://outlook.office.com/IMAP.AccessAsUser.All','https://outlook.office.com/SMTP.Send']
RefreshTokenFileName = "/home/user/.config/m365/imap_smtp_refresh_token"
AccessTokenFileName = "/home/user/.config/m365/imap_smtp_access_token"
Authority = None
```
