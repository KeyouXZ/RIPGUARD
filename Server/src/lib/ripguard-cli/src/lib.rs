// Copyright (C) 2026 Keyou
// SPDX-License-Identifier: AGPL-3.0-or-later

use clap::{CommandFactory, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "RIPGUARD", version, about = "A fast lightweight onnx runtime")]
pub struct Cli {
    /// The directory where log files will be saved
    #[arg(short = 'p', long, default_value = "./logs")]
    pub log_path: PathBuf,

    /// Enable verbose logging for detailed debugging output
    #[arg(short = 'v', long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "init", about = "Initialize a default configuration file")]
    Init {
        /// The path to a custom configuration file (e.g., config.toml)
        #[arg(short = 'c', long)]
        config_path: Option<PathBuf>,
    },

    #[command(name = "start", about = "Start the RIPGUARD server")]
    Start {
        /// The path to a custom configuration file (e.g., config.toml)
        #[arg(short = 'c', long)]
        config_path: Option<PathBuf>,

        /// The port to listen on
        #[arg(short = 'p', long)]
        port: Option<u16>,
    },
}

pub fn parse_args() -> Cli {
    Cli::parse()
}

pub fn show_help() -> anyhow::Result<()> {
    Cli::command().print_help()?;
    Ok(())
}
