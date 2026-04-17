use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Refresh M365 OAuth2 tokens using a refresh token")]
struct Args {
    #[arg(long)]
    config: PathBuf,
}

#[derive(Deserialize)]
struct Config {
    client_id: String,
    client_secret: String,
    authority: String,
    scopes: Vec<String>,
    refresh_token_file: String,
    access_token_file: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config_content = fs::read_to_string(&args.config)
        .map_err(|e| format!("failed to read config {:?}: {}", args.config, e))?;
    let config: Config = toml::from_str(&config_content)
        .map_err(|e| format!("failed to parse config: {}", e))?;

    let refresh_token = fs::read_to_string(&config.refresh_token_file)
        .map_err(|e| format!("failed to read refresh token file: {}", e))?;
    let refresh_token = refresh_token.trim().to_string();

    let token_url = format!(
        "{}/oauth2/v2.0/token",
        config.authority.trim_end_matches('/')
    );

    let scopes = config.scopes.join(" ");
    let mut params = HashMap::new();
    params.insert("grant_type", "refresh_token");
    params.insert("client_id", &config.client_id);
    params.insert("client_secret", &config.client_secret);
    params.insert("refresh_token", &refresh_token);
    params.insert("scope", &scopes);

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&token_url)
        .form(&params)
        .send()
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("token request failed ({}): {}", status, body).into());
    }

    let token: TokenResponse = response
        .json()
        .map_err(|e| format!("failed to parse token response: {}", e))?;

    fs::write(&config.refresh_token_file, &token.refresh_token)
        .map_err(|e| format!("failed to write refresh token: {}", e))?;
    fs::write(&config.access_token_file, &token.access_token)
        .map_err(|e| format!("failed to write access token: {}", e))?;

    Ok(())
}
