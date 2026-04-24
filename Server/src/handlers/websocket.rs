use crate::model::{ApiResponse, Detection};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use futures::future::join_all;
use log::info;
use rand::RngExt;

pub async fn ws_handler(ws: WebSocketUpgrade, req_client: reqwest::Client, config: crate::config::Config) -> impl IntoResponse {
    ws.on_upgrade(move |sock| async move { handle_socket(sock, req_client, config).await })
}

async fn handle_socket(socket: WebSocket, req_client: reqwest::Client, config: crate::config::Config) {
    let (mut sender, mut receiver) = socket.split();

    tokio::spawn(async move {
        loop {
            let mut detections = {
                // TODO: real usage of onnx here

                let mut rng = rand::rng();

                vec![
                    Detection {
                        bbox: [
                            rng.random_range(0.0..0.5), // x
                            rng.random_range(0.0..0.5), // y
                            rng.random_range(0.5..1.0), // width
                            rng.random_range(0.5..1.0), // height
                        ],
                        confidence: rng.random_range(0.5..1.0),
                        latitude: rng.random_range(-8.1..-7.9),
                        longitude: rng.random_range(110.2..110.5),
                        wind_speed: None,
                    }
                ]
            };

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

            tokio::time::sleep(std::time::Duration::from_millis(config.general.update_interval)).await;
        }
    });

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            info!("Client says: {}", text);
        }
    }

    info!("Client disconnected");
}
