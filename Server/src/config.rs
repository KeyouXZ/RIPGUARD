use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Debug, Default, Copy, Clone)]
pub struct Config {
    pub general: GeneralConfig,
}

impl Config {
    pub fn read_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let toml_str = fs::read_to_string(path)?;

        let config: Config = toml::from_str(&toml_str)?;

        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let toml_string = toml::to_string(&self)?;

        fs::write(path, toml_string)?;

        Ok(())
    }

    pub fn load_or_create(path: &str) -> Config {
        match Config::read_from_file(path) {
            Ok(config) => {
                info!("Configuration loaded successfully from {}", path);
                config
            }
            Err(e) => {
                warn!("⚠️ Could not load config from {}. Error: {}", path, e);
                info!("Creating and saving a default configuration...");

                let default_config = Config::default();

                if let Err(save_err) = default_config.save_to_file(path) {
                    panic!(
                        "❌ FATAL: Failed to save default configuration to {}: {}",
                        path, save_err
                    );
                } else {
                    info!("Default configuration saved to {}", path);
                }

                // Return the new default instance
                default_config
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct GeneralConfig {
    pub update_interval: u64, // in milisecond
    pub debug: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            update_interval: 5000,
            debug: false,
        }
    }
}
