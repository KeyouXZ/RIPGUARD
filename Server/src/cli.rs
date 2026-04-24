use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short = 'p', long, default_value = "./logs")]
    pub log_path: PathBuf,
    #[arg(short = 'c', long)]
    pub config_path: Option<PathBuf>,
    #[arg(short = 'v', long)]
    pub verbose: bool,
}
