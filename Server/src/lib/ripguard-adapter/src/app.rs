// Copyright (C) 2026 KeyouXZ
// SPDX-License-Identifier: AGPL-3.0-or-later

use ripguard_model::AppState;
use ripguard_route::{http, ws};

pub fn create_app(app_state: AppState) -> axum::Router {
    axum::Router::new()
        .merge(http::routes(app_state.clone()))
        .merge(ws::routes(app_state))
}
