use axum::{
    Json,
    response::IntoResponse
};
use chrono::Local;
use log::info;
use tokio::{fs, fs::OpenOptions, io::AsyncWriteExt};
use crate::model::ErrorReport;

pub async fn report_handler(Json(payload): Json<ErrorReport>) -> impl IntoResponse {
    info!("Report Received!");

    fs::create_dir_all("reports").await.unwrap();
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let output_path = format!("reports/{}-{}.log", timestamp, payload.source);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_path)
        .await
        .unwrap();

    let log = format!(
        "\n--- ERROR REPORT ---\nplatform: {}\nsource: {}\nmessage:\n{}\n",
        payload.platform, payload.source, payload.message
    );

    let _ = file.write_all(log.as_bytes()).await;

    Json(serde_json::json!({
        "status": "ok"
    }))
}