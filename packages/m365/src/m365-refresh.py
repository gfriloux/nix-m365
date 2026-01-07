#!/usr/bin/env python3

from msal import ConfidentialClientApplication, SerializableTokenCache
import sys
import argparse
import importlib.util
import os
import atexit
from pathlib import Path

def load_as_config_module(config_path: str):
    path = Path(config_path).resolve()
    if not path.exists():
        raise FileNotFoundError(f"config file not found: {path}")

    spec = importlib.util.spec_from_file_location("config", str(path))
    if spec is None or spec.loader is None:
        raise ImportError(f"failed to load {path}")

    module = importlib.util.module_from_spec(spec)
    sys.modules["config"] = module
    spec.loader.exec_module(module)
    return module

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", required=True, help="path to config file")
    args = parser.parse_args()

    config = load_as_config_module(args.config)

    cache_filename = os.path.join(os.getenv("XDG_RUNTIME_DIR", ""), "nix-m365.bin")
    cache = SerializableTokenCache()
    if os.path.exists(cache_filename):
        cache.deserialize(open(cache_filename, "r").read())
    atexit.register(lambda:
        open(cache_filename, "w").write(cache.serialize())
        if cache.has_state_changed else None
    )

    app = ConfidentialClientApplication(config.ClientId, client_credential=config.ClientSecret, token_cache=cache, authority=config.Authority)
    old_refresh_token = open(config.RefreshTokenFileName,'r').read()
    token = app.acquire_token_by_refresh_token(old_refresh_token,config.Scopes)

    if 'error' in token:
        print(token)
        sys.exit("Failed to get access token")

    with open(config.RefreshTokenFileName, 'w') as f:
        f.write(token['refresh_token'])
    with open(config.AccessTokenFileName, 'w') as f:
        f.write(token['access_token'])


if __name__ == "__main__":
    main()
