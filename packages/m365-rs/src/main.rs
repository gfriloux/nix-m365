use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
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
    refresh_token_file: PathBuf,
    access_token_file: PathBuf,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let config_content = fs::read_to_string(&args.config)
        .with_context(|| format!("failed to read config {:?}", args.config))?;
    let config: Config = toml::from_str(&config_content)
        .context("failed to parse config")?;

    let refresh_token = fs::read_to_string(&config.refresh_token_file)
        .with_context(|| format!("failed to read refresh token {:?}", config.refresh_token_file))?;
    let refresh_token = refresh_token.trim();

    let token_url = format!(
        "{}/oauth2/v2.0/token",
        config.authority.trim_end_matches('/')
    );
    let scopes = config.scopes.join(" ");

    let params = [
        ("grant_type", "refresh_token"),
        ("client_id", config.client_id.as_str()),
        ("client_secret", config.client_secret.as_str()),
        ("refresh_token", refresh_token),
        ("scope", scopes.as_str()),
    ];

    let response = reqwest::blocking::Client::new()
        .post(&token_url)
        .form(&params)
        .send()
        .context("HTTP request failed")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        anyhow::bail!("token request failed ({}): {}", status, body);
    }

    let token: TokenResponse = response
        .json()
        .context("failed to parse token response")?;

    fs::write(&config.refresh_token_file, &token.refresh_token)
        .with_context(|| format!("failed to write refresh token {:?}", config.refresh_token_file))?;
    fs::write(&config.access_token_file, &token.access_token)
        .with_context(|| format!("failed to write access token {:?}", config.access_token_file))?;

    Ok(())
}
