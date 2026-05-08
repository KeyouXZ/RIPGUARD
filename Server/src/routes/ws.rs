use axum::{Router, routing::get};
use axum::extract::State;
use crate::model::AppState;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/ws", get(move |ws| async move {
            crate::handlers::websocket::ws_handler(ws, State(app_state)).await
        }))
}