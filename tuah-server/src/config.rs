use std::env;

use anyhow::{
    Ok,
    Result,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(long, default_value = ":")]
    pub addr: String,

    #[arg(long, default_value = "")]
    pub cert: String,

    #[arg(long, default_value = "")]
    pub key: String,
}

pub fn fetch_config() -> Result<Config> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let mut args = Config::parse();

    if args.addr == ":" {
        args.addr = format!(":{}", port);
    }

    Ok(args)
}
