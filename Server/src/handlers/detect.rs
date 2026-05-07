use axum::Json;
use crate::model::{DetectRequest, Detection};

pub async fn detect_handler(
    Json(payload): Json<DetectRequest>,
) -> Result<Json<Vec<Detection>>, String> {

    let results = crate::services::detect_service::detect(payload.image)
        .await
        .map_err(|e| e.to_string())?;

    Ok(results)
}