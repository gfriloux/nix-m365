use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub authority: String,
    pub refresh_token_file: PathBuf,
    pub access_token_file: PathBuf,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let content =
            fs::read_to_string(path).with_context(|| format!("failed to read config {path:?}"))?;
        Self::parse(&content)
    }

    fn parse(content: &str) -> Result<Self> {
        toml::from_str(content).context("failed to parse config")
    }

    pub fn token_url(&self) -> String {
        format!("{}/oauth2/v2.0/token", self.authority.trim_end_matches('/'))
    }

    pub fn read_refresh_token(&self) -> Result<String> {
        fs::read_to_string(&self.refresh_token_file)
            .with_context(|| format!("failed to read refresh token {:?}", self.refresh_token_file))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        r#"
            client_id = "test-client-id"
            client_secret = "test-secret"
            authority = "https://login.microsoftonline.com/tenant-id"
            refresh_token_file = "/tmp/refresh"
            access_token_file = "/tmp/access"
        "#
    }

    #[test]
    fn parse_valid_config() {
        let config = Config::parse(sample()).unwrap();
        assert_eq!(config.client_id, "test-client-id");
        assert_eq!(config.client_secret, "test-secret");
    }

    #[test]
    fn parse_missing_field() {
        assert!(Config::parse(r#"client_id = "only-this""#).is_err());
    }

    #[test]
    fn parse_invalid_toml() {
        assert!(Config::parse("not valid toml [[[").is_err());
    }

    #[test]
    fn token_url_no_trailing_slash() {
        let config = Config::parse(sample()).unwrap();
        assert_eq!(
            config.token_url(),
            "https://login.microsoftonline.com/tenant-id/oauth2/v2.0/token"
        );
    }

    #[test]
    fn token_url_with_trailing_slash() {
        let mut config = Config::parse(sample()).unwrap();
        config.authority = "https://login.microsoftonline.com/tenant-id/".to_string();
        assert_eq!(
            config.token_url(),
            "https://login.microsoftonline.com/tenant-id/oauth2/v2.0/token"
        );
    }
}
