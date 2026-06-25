use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub general: GeneralConfig,
}

impl Config {
    pub fn read_from_file(path: &str) -> Result<Self> {
        let toml_str =
            fs::read_to_string(path).context(format!("Failed to read config file at: {}", path))?;

        let config: Config = toml::from_str(&toml_str).context("Failed to parse TOML content")?;

        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let toml_string = toml::to_string(&self).context("Failed to serialize config to TOML")?;

        fs::write(path, toml_string).context("Failed to write config file to disk")?;

        Ok(())
    }

    pub fn create_config(path: &str) -> Result<Config> {
        let default_config = Config::default();

        default_config
            .save_to_file(path)
            .context("Failed to create default configuration file")?;

        Ok(default_config)
    }

    pub fn load_config(path: &str) -> Result<Config> {
        let config = Config::read_from_file(path)?;

        Ok(config)
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct GeneralConfig {
    pub update_interval: u64, // in milisecond
    pub debug: bool,
    pub port: u16,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            update_interval: 5000,
            debug: false,
            port: 3000,
        }
    }
}
