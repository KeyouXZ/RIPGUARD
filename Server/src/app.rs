use crate::model::AppState;

pub fn create_app(app_state: AppState) -> axum::Router {
    axum::Router::new()
        .merge(crate::routes::http::routes(app_state.clone()))
        .merge(crate::routes::ws::routes(app_state))
}