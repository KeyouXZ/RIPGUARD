// Copyright (C) 2026 Keyou
// SPDX-License-Identifier: AGPL-3.0-or-later

use log::error;
use ripguard_model::{AppState, Payload};

use crate::{enrich_weather, grab_frame, real_detection};

const MAX_CACHE: usize = 256;

async fn process_detection_cycle(
    app_state: &AppState,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let frame_path = grab_frame().await?;

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

        cache.push_back(detections.clone());

        cache.retain(|item| !item.is_expired());

        let del_payload = Payload::Detection(detections.clone());
        let del_msg = serde_json::to_string(&del_payload)?;
        app_state.tx.send(del_msg)?;

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
