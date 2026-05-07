use axum::{Router, routing::post};

pub fn routes() -> Router {
    Router::new()
        .route("/detect", post(crate::handlers::detect::detect_handler))
}