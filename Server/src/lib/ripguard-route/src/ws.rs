// Copyright (C) 2026 Keyou
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::extract::State;
use axum::{Router, routing::get};
use ripguard_handler::websocket;
use ripguard_model::AppState;

pub fn routes(app_state: AppState) -> Router {
    Router::new().route(
        "/ws",
        get(move |ws| async move { websocket::ws_handler(ws, State(app_state)).await }),
    )
}
