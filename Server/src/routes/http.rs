use crate::model::AppState;
use axum::{Router, routing::post};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/detect", post(crate::handlers::detect::detect_handler))
        .with_state(state)
        .route("/report", post(crate::handlers::report::report_handler))
}
