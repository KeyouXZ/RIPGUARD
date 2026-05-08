use axum::{Router, routing::post};
use crate::model::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/detect", post(crate::handlers::detect::detect_handler))
        .with_state(state)
}