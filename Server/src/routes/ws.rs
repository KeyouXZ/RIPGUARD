use axum::{Router, routing::get};

pub fn routes(req_client: reqwest::Client, config: crate::config::Config) -> Router {
    Router::new()
        .route("/ws", get(move |ws| async move {
            crate::handlers::websocket::ws_handler(ws, req_client.clone(), config.clone()).await
        }))
}