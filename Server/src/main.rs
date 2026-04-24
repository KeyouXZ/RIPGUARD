mod routes;
mod handlers;
mod services;
mod model;
mod app;
mod logger;
mod cli;
mod config;

use std::process::exit;
use clap::Parser;
use log::info;
use crate::config::Config;

#[tokio::main]
async fn main() {
    let args = cli::Cli::parse();

    if let Err(e) = logger::setup_logger(&args) {
        eprintln!("Failed to setup logger: {}", e);
        exit(1);
    }

    let config_path = args.config_path.unwrap_or_else(|| "config.toml".to_string().parse().unwrap());
    let config = Config::load_or_create(config_path.to_str().unwrap());

    info!("Model loaded!");

    let req_client = match reqwest::Client::builder().build() {
        Ok(client) => client,
        Err(err) => {
            log::error!("Failed to build reqwest client: {}", err);
            exit(1);
        }
    };

    let app = app::create_app(req_client, config);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await;
    let listener = listener.unwrap_or_else(|e| {
        log::error!("Error: {}", e);
        exit(1);
    });

    info!("Server running...");

    if let Err(e) = axum::serve(listener, app).await {
        log::error!("Server error: {}", e);
    }
}
