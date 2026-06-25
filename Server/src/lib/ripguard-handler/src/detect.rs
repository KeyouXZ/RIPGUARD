use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use ripguard_model::AppState;

pub async fn detect_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("");

        if name == "file" {
            let data = field.bytes().await.unwrap();
            let data_size = data.len();

            let results = ripguard_service::detect(&state, data)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
                .unwrap();

            return Json(serde_json::json!({
                "success": true,
                "size": data_size,
                "result": results,
            }));
        }
    }

    Json(serde_json::json!({
        "success": false,
        "error": "no file received"
    }))
}
