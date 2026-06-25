use axum::{Json, response::IntoResponse};
use chrono::Local;
use log::info;
use ripguard_model::ErrorReport;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

pub async fn report_handler(Json(payload): Json<ErrorReport>) -> impl IntoResponse {
    info!("Report Received!");

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let output_path = format!("reports/{}-{}.log", timestamp, payload.source);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_path)
        .await
        .unwrap();

    let log = format!(
        "--- ERROR REPORT ---\nplatform: {}\nsource: {}\nmessage:\n{}\n",
        payload.platform, payload.source, payload.message
    );

    let _ = file.write_all(log.as_bytes()).await;

    Json(serde_json::json!({
        "status": "ok"
    }))
}
