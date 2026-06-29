use anyhow::{Context, Result};
use image::RgbImage;
use libloading::Library;
use log::{error, info};
use ndarray::Array4;
use ripguard_model::{AppState, Detection};
use std::{collections::VecDeque, path::PathBuf, process::exit, sync::Arc, time::Duration};
use tokio::{
    fs,
    sync::{Mutex, broadcast},
};

pub async fn start(port: &Option<u16>, config_path: &Option<PathBuf>) -> Result<()> {
    // Check for libonnxruntime
    unsafe {
        match Library::new("libonnxruntime.so") {
            Ok(_) => info!("Found libonnxruntime.so"),
            Err(e) => {
                error!("Failed: {e}");
                exit(1);
            }
        }
    }

    info!("Loading config...");
    let default_path = PathBuf::from("config.toml");
    let path = config_path.as_ref().unwrap_or(&default_path);

    let config =
        ripguard_config::Config::load_config(path.to_str().context("Invalid path encoding")?)?;

    let port = match port {
        Some(x) => *x,
        None => config.general.port,
    };

    info!("Loading model...");
    let session = ripguard_adapter::yolo::create_yolov8_session();
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

    // Read data json if available
    let json = fs::read_to_string("data.json").await?;
    let data_json: VecDeque<Detection> = serde_json::from_str(&json)?;

    // Pre-allocate reusable buffers to avoid repeated allocations
    let input_buffer = Arc::new(Mutex::new(Array4::<f32>::zeros((1, 3, 640, 640))));
    let image_buffer = Arc::new(Mutex::new(RgbImage::new(640, 640)));
    let cache = Arc::new(Mutex::new(data_json));

    let cache_len = cache.lock().await.len();
    let last_id = if cache_len > 0 {
        cache.lock().await.back().unwrap().id
    } else {
        0
    };

    info!("Creating app...");
    let state = AppState {
        session: Arc::new(Mutex::new(session)),
        req_client,
        config: Arc::new(config),
        tx: tx.clone(),
        input_buffer,
        image_buffer,
        cache,
        last_id: Arc::new(Mutex::new(last_id)),
    };

    let detection_state = state.clone();
    tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(ripguard_service::global_detection_loop(detection_state));
    });

    let config = state.config.clone();

    info!("Starting file deletor...");
    tokio::spawn(async move {
        ripguard_service::start_file_deletor(
            Duration::from_secs(config.general.update_interval / 1000), // milis to second
            "frames",
            1_073_741_824, // 1GB
        )
        .await;
    });

    let app = ripguard_adapter::app::create_app(state);

    let listener = tokio::net::TcpListener::bind(format!("[::]:{}", port)).await;
    let listener = listener.unwrap_or_else(|e| {
        log::error!("Error: {}", e);
        exit(1);
    });

    info!("Server running at port {}", port);

    if let Err(e) = axum::serve(listener, app).await {
        log::error!("Server error: {}", e);
    }
    Ok(())
}
