// Copyright (C) 2026 Keyou
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    Router,
    routing::{get, post},
};
use ripguard_handler::{detect, image, report};
use ripguard_model::AppState;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/image/{id}", get(image::image_handler))
        .route("/detect", post(detect::detect_handler))
        .with_state(state)
        .route("/report", post(report::report_handler))
}
