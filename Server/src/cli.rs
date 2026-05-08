use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short = 'l', long, default_value = "./logs")]
    pub log_path: PathBuf,
    #[arg(short = 'c', long)]
    pub config_path: Option<PathBuf>,
    #[arg(short = 'v', long)]
    pub verbose: bool,
    #[arg(short = 'p', long)]
    pub port: Option<u16>,
}
