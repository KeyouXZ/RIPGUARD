use axum::{Router, routing::post};

pub fn routes() -> Router {
    Router::new()
        .route("/detect", post(|| async { crate::handlers::detect::detect_handler().await.unwrap() }))
}