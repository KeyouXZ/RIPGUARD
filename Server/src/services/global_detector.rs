use std::sync::Arc;
use log::{error};
use ort::session::Session;
use tokio::sync::Mutex;
use crate::{
    services::{
        detection::real_detection,
        enrich_weather::enrich_weather,
        grab_frame::grab_frame
    },
    model::AppState
};

async fn process_detection_cycle(
    req_client: &reqwest::Client,
    session: &Arc<Mutex<Session>>
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let frame_path = grab_frame().await?;

    //let mut detections = fake_detections();
    let mut detections = real_detection(session, &frame_path).await;

    enrich_weather(req_client, &mut detections).await;

    let msg = serde_json::to_string(&detections)?;

    Ok(msg)
}


pub async fn global_detection_loop(
    app_state: AppState
) {
    // TODO: make cache system instead of sending all of the image trough websocket..cuz that will be expensive on resource usage
    let req_client = app_state.req_client;
    let config = app_state.config;
    let tx = app_state.tx;
    let session = app_state.session;

    loop {
        match process_detection_cycle(&req_client, &session).await {

            Ok(msg) => {
                let _ = tx.send(msg);
            }

            Err(err) => {
                error!("Detection error: {}", err);
            }
        }

        tokio::time::sleep(
            std::time::Duration::from_millis(config.general.update_interval)
        ).await;
    }
}