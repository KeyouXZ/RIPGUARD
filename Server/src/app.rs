pub fn create_app(req_client: reqwest::Client) -> axum::Router {
    axum::Router::new()
        .merge(crate::routes::http::routes())
        .merge(crate::routes::ws::routes(req_client))
}