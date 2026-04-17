use crate::config::Config;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

impl TokenPair {
    pub fn fetch(config: &Config, refresh_token: &str) -> Result<Self> {
        let params = [
            ("grant_type", "refresh_token"),
            ("client_id", config.client_id.as_str()),
            ("client_secret", config.client_secret.as_str()),
            ("refresh_token", refresh_token),
        ];

        let response = reqwest::blocking::Client::new()
            .post(config.token_url())
            .form(&params)
            .send()
            .context("HTTP request failed")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            anyhow::bail!("token request failed ({}): {}", status, body);
        }

        response.json().context("failed to parse token response")
    }

    pub fn save(&self, config: &Config) -> Result<()> {
        fs::write(&config.refresh_token_file, &self.refresh_token).with_context(|| {
            format!(
                "failed to write refresh token {:?}",
                config.refresh_token_file
            )
        })?;
        fs::write(&config.access_token_file, &self.access_token).with_context(|| {
            format!(
                "failed to write access token {:?}",
                config.access_token_file
            )
        })
    }
}
