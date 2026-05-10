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
use image::RgbImage;
use log::info;
use tokio::sync::{broadcast, Mutex};
use ndarray::Array4;
use crate::config::Config;
use crate::model::AppState;
use crate::services::global_detector::global_detection_loop;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Cli::parse();

    if let Err(e) = logger::setup_logger(&args) {
        eprintln!("Failed to setup logger: {}", e);
        exit(1);
    }

    let config_path = args.config_path.unwrap_or_else(|| "config.toml".parse().unwrap());
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

    // Pre-create all required directories to avoid repeated checks
    tokio::fs::create_dir_all("frames").await?;
    tokio::fs::create_dir_all("frames/detected").await?;
    tokio::fs::create_dir_all("reports").await?;

    // Pre-allocate reusable buffers to avoid repeated allocations
    let input_buffer = Arc::new(Mutex::new(Array4::<f32>::zeros((1, 3, 640, 640))));
    let image_buffer = Arc::new(Mutex::new(RgbImage::new(640, 640)));

    info!("Creating app...");
    let state = AppState {
        session: Arc::new(Mutex::new(session)),
        req_client,
        config: Arc::new(config),
        tx: tx.clone(),
        input_buffer,
        image_buffer,
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
    Ok(())
}
