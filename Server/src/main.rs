mod routes;
mod handlers;
mod services;
mod model;
mod app;


use std::process::exit;

#[tokio::main]
async fn main() {
    println!("Model loaded!");

    let req_client = match reqwest::Client::builder().build() {
        Ok(client) => client,
        Err(err) => {
            log::error!("Failed to build reqwest client: {}", err);
            exit(1);
        }
    };

    let app = app::create_app(req_client);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await;
    let listener = listener.unwrap_or_else(|e| {
        log::error!("Error: {}", e);
        exit(1);
    });

    println!("Server running...");

    if let Err(e) = axum::serve(listener, app).await {
        log::error!("Server error: {}", e);
    }
}
