use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub network: String,
    pub wallets: Vec<WalletEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletEntry {
    pub name: String,
    pub public_key: String,
    pub secret_key: Option<String>,
    pub network: String,
    pub created_at: String,
    pub funded: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: "testnet".to_string(),
            wallets: vec![],
        }
    }
}

pub fn config_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".starforge")
}

pub fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn load() -> Result<Config> {
    let path = config_path();
    if !path.exists() {
        return Ok(Config::default());
    }
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config at {:?}", path))?;
    let config: Config = toml::from_str(&contents)
        .with_context(|| "Failed to parse config file")?;
    Ok(config)
}

pub fn save(config: &Config) -> Result<()> {
    let dir = config_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create config dir {:?}", dir))?;
    }
    let contents = toml::to_string_pretty(config)
        .with_context(|| "Failed to serialize config")?;
    fs::write(config_path(), contents)
        .with_context(|| "Failed to write config file")?;
    Ok(())
}
