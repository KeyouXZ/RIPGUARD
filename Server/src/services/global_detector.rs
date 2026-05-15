use crate::{
    model::AppState,
    services::{detection::real_detection, enrich_weather::enrich_weather, grab_frame::grab_frame},
};
use log::error;

async fn process_detection_cycle(
    app_state: &crate::model::AppState,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let frame_path = grab_frame().await?;

    //let mut detections = fake_detections();
    let mut detections = real_detection(app_state, &frame_path).await;

    enrich_weather(&app_state.req_client, &mut detections).await;

    let msg = serde_json::to_string(&detections)?;

    Ok(msg)
}

pub async fn global_detection_loop(app_state: AppState) {
    // TODO: make cache system instead of sending all of the image trough websocket..cuz that will be expensive on resource usage
    let tx = app_state.tx.clone();
    let config = app_state.config.clone();

    loop {
        match process_detection_cycle(&app_state).await {
            Ok(msg) => {
                let _ = tx.send(msg);
            }

            Err(err) => {
                error!("Detection error: {}", err);
            }
        }

        tokio::time::sleep(std::time::Duration::from_millis(
            config.general.update_interval,
        ))
        .await;
    }
}
