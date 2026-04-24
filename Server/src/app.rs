pub fn create_app(req_client: reqwest::Client, config: crate::config::Config) -> axum::Router {
    axum::Router::new()
        .merge(crate::routes::http::routes())
        .merge(crate::routes::ws::routes(req_client, config))
}