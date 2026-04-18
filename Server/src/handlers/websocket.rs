use crate::model::{ApiResponse, Detection};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use futures::future::join_all;

pub async fn ws_handler(ws: WebSocketUpgrade, req_client: reqwest::Client) -> impl IntoResponse {
    ws.on_upgrade(|sock| async { handle_socket(sock, req_client).await })
}

async fn handle_socket(socket: WebSocket, req_client: reqwest::Client) {
    let (mut sender, mut receiver) = socket.split();

    tokio::spawn(async move {
        loop {
            let mut detections = vec![
                Detection {
                    bbox: [0.0, 0.0, 1.0, 1.0],
                    confidence: 0.9,
                    latitude: -8.02,
                    longitude: 110.32,
                    wind_speed: None,
                }
            ];

            let futures = detections.iter().map(|det| {
                let client = req_client.clone();
                async move {
                    let url = format!(
                        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
                        det.latitude, det.longitude
                    );

                    let res = client.get(&url).send().await.ok()?;
                    let data = res.json::<ApiResponse>().await.ok()?;

                    Some(data.current_weather.windspeed as f32)
                }
            });

            let results = join_all(futures).await;

            for (det, wind) in detections.iter_mut().zip(results) {
                det.wind_speed = wind;
            }

            let msg = serde_json::to_string(&detections).unwrap();

            if sender.send(Message::Text(msg.into())).await.is_err() {
                break; // client disconnected
            }

            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            println!("Client says: {}", text);
        }
    }

    println!("Client disconnected");
}
