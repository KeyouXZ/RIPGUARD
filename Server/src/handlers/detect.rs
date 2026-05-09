use crate::model::AppState;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    Json
};

pub async fn detect_handler(State(state): State<AppState>, mut multipart: Multipart) -> impl IntoResponse {
    let mut session = state.session.lock().await;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            let data = field.bytes().await.unwrap();

            let results = crate::services::detect_service::detect(&mut session, data.clone())
                .await
                .map_err(|e| (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    e.to_string()
                )).unwrap();

            return Json(serde_json::json!({
                "success": true,
                "size": data.len(),
                "result": results,
            }));
        }
    }

    Json(serde_json::json!({
        "success": false,
        "error": "no file received"
    }))
}