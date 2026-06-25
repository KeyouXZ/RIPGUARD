use axum::{Router, routing::post};
use ripguard_handler::{detect, report};
use ripguard_model::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/detect", post(detect::detect_handler))
        .with_state(state)
        .route("/report", post(report::report_handler))
}
