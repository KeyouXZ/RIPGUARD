use axum::Json;
use crate::model::Detection;

pub async fn detect_handler() -> Result<Json<Vec<Detection>>, Box<dyn std::error::Error>> {
    let results = crate::services::detect_service::detect().await?;
    Ok(results)
}