#![allow(clippy::unused_async)]
use axum::{
    extract::{WebSocketUpgrade, ws::WebSocket},
    response::Response,
};
use log::{error, info};

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match socket.send(msg).await {
                Ok(()) => info!("Message sent successfully"),
                Err(e) => error!("Error sending websocket message: {e}"),
            }
        }
    }
}

pub async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}
