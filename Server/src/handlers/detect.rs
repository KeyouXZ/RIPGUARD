use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::model::{AppState, DetectRequest, DetectionResponse};

pub async fn detect_handler(
    State(state): State<AppState>,
    Json(payload): Json<DetectRequest>,
) -> Result<Json<DetectionResponse>, (StatusCode, String)> {
    let mut session = state.session.lock().await;

    let results = crate::services::detect_service::detect(&mut session, payload.image)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string()
        ))?;

    Ok(results)
}