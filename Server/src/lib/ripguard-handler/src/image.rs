use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use ripguard_model::AppState;

pub async fn image_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let detection = {
        let cache = state.cache.lock().await;

        cache.iter().find(|d| d.id == id).cloned()
    };

    let Some(detection) = detection else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let Some(path) = &detection.image_path else {
        return StatusCode::NOT_FOUND.into_response();
    };

    match tokio::fs::read(path).await {
        Ok(bytes) => ([("Content-Type", "image/png")], bytes).into_response(),

        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
