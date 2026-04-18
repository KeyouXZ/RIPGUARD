use axum::{Router, routing::get};

pub fn routes(req_client: reqwest::Client) -> Router {
    Router::new()
        .route("/ws", get(|ws| async { crate::handlers::websocket::ws_handler(ws, req_client).await }))
}