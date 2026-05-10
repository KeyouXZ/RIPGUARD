mod routes;
mod handlers;
mod services;
mod model;
mod app;
mod logger;
mod cli;
mod config;
mod yolo;

use std::process::exit;
use std::sync::Arc;
use clap::Parser;
use log::info;
use tokio::sync::{broadcast, Mutex};
use crate::config::Config;
use crate::model::AppState;
use crate::services::global_detector::global_detection_loop;

#[tokio::main]
async fn main() {
    let args = cli::Cli::parse();

    if let Err(e) = logger::setup_logger(&args) {
        eprintln!("Failed to setup logger: {}", e);
        exit(1);
    }

    let config_path = args.config_path.unwrap_or_else(|| "config.toml".to_string().parse().unwrap());
    let config = Config::load_or_create(config_path.to_str().unwrap());

    let port = match args.port {
        Some(x) => x,
        None => config.general.port,
    };
    
    info!("Loading model...");
    let session = yolo::create_yolov8_session();
    info!("Model loaded!");

    info!("Loading reqwest client...");
    let req_client = match reqwest::Client::builder().build() {
        Ok(client) => client,
        Err(err) => {
            log::error!("Failed to build reqwest client: {}", err);
            exit(1);
        }
    };
    info!("Reqwest client loaded!");

    let (tx, _) = broadcast::channel::<String>(100);


    info!("Creating app...");
    let state = AppState {
        session: Arc::new(Mutex::new(session)),
        req_client,
        config,
        tx: tx.clone()
    };

    let detection_state = state.clone();
    tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(global_detection_loop(detection_state));
    });

    let app = app::create_app(state);

    let listener = tokio::net::TcpListener::bind(format!("[::]:{}", port)).await;
    let listener = listener.unwrap_or_else(|e| {
        log::error!("Error: {}", e);
        exit(1);
    });

    info!("Server running...");

    if let Err(e) = axum::serve(listener, app).await {
        log::error!("Server error: {}", e);
    }
}
