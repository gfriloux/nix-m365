mod config;
mod token;

use anyhow::Result;
use clap::Parser;
use config::Config;
use std::path::PathBuf;
use token::TokenPair;

#[derive(Parser)]
#[command(about = "Refresh M365 OAuth2 tokens using a refresh token")]
struct Args {
    #[arg(long)]
    config: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::load(&args.config)?;
    let refresh_token = config.read_refresh_token()?;
    let token = TokenPair::fetch(&config, refresh_token.trim())?;
    token.save(&config)
}
