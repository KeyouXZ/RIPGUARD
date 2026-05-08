use crate::model::AppState;
use axum::extract::State;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::{StreamExt, SinkExt};
use log::info;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}
async fn handle_socket(
    socket: WebSocket,
    state: AppState,
) {
    info!("New client connected!");
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to global broadcast
    let mut rx = state.tx.subscribe();

    // Sender task
    let send_task = tokio::spawn(async move {

        while let Ok(msg) = rx.recv().await {

            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Receiver task
    let recv_task = tokio::spawn(async move {

        while let Some(Ok(msg)) = receiver.next().await {

            if let Message::Text(text) = msg {
                info!("Client: {}", text);
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    info!("Client disconnected");
}