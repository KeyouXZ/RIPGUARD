// Copyright (C) 2026 Keyou
// SPDX-License-Identifier: AGPL-3.0-or-later

use log::error;
use ripguard_cli::Commands;
use std::process::exit;

mod init;
mod start;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ripguard_cli::parse_args();

    if let Err(e) = ripguard_logger::setup_logger(&args) {
        eprintln!("Failed to setup logger: {}", e);
        exit(1);
    }

    match &args.command {
        Some(Commands::Init { config_path }) => {
            if let Err(e) = init::init(config_path).await {
                error!("{:?}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Start { port, config_path }) => {
            if let Err(e) = start::start(port, config_path).await {
                error!("{:?}", e);
                std::process::exit(1);
            }
        }
        None => {
            ripguard_cli::show_help()?;
        }
    }

    Ok(())
}
