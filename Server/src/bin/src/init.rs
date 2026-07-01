// Copyright (C) 2026 Keyou
// SPDX-License-Identifier: AGPL-3.0-or-later

use anyhow::{Context, Result};
use log::{info, warn};
use std::path::PathBuf;

pub async fn init(config_path: &Option<PathBuf>) -> Result<()> {
    info!("Starting initialization process...");

    let default_path = PathBuf::from("config.toml");
    let path = config_path.as_ref().unwrap_or(&default_path);

    if path.exists() {
        warn!(
            "Configuration file already exists at {:?}. Skipping creation.",
            path
        );
        return Ok(());
    }

    ripguard_config::Config::create_config(path.to_str().context("Invalid path encoding")?)?;

    info!("Configuration saved successfully at: {:?}", path);
    Ok(())
}
