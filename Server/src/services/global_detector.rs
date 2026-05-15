use crate::{
    model::{AppState, Payload},
    services::{detection::real_detection, enrich_weather::enrich_weather, grab_frame::grab_frame},
};
use log::error;

async fn process_detection_cycle(
    app_state: &crate::model::AppState,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    const MAX_CACHE: usize = 100;

    let frame_path = grab_frame().await?;

    //let mut detections = fake_detections();
    let mut detections = real_detection(app_state, &frame_path).await;

    if detections.detections.is_empty() {
        return Ok(None);
    }

    enrich_weather(&app_state.req_client, &mut detections).await;

    let payload = Payload::Detection(detections.clone());
    let msg = serde_json::to_string(&payload)?;

    // cache system
    {
        let mut cache = app_state.cache.lock().await;
        let ttl = app_state.config.general.update_interval;

        cache.push_back(detections);

        cache.retain(|item| !item.is_expired(ttl));

        while cache.len() > MAX_CACHE {
            cache.pop_front();
        }
    }

    Ok(Some(msg))
}

pub async fn global_detection_loop(app_state: AppState) {
    let tx = app_state.tx.clone();
    let config = app_state.config.clone();

    loop {
        match process_detection_cycle(&app_state).await {
            Ok(Some(msg)) => {
                let _ = tx.send(msg);
            }

            Ok(None) => {}

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
