use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Extension},
    response::Response,
};
use std::sync::Arc;
use tokio::sync::broadcast;
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::json;

use crate::database::Database;
use crate::models::WebSocketMessage;

// Global broadcaster for WebSocket messages
lazy_static::lazy_static! {
    static ref BROADCASTER: broadcast::Sender<WebSocketMessage> = {
        let (tx, _) = broadcast::channel(100);
        tx
    };
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Extension(db): Extension<Arc<Database>>,
) -> Response {
    ws.on_upgrade(|socket| websocket_connection(socket, db))
}

async fn websocket_connection(socket: WebSocket, _db: Arc<Database>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = BROADCASTER.subscribe();
    
    // Send initial data
    let initial_message = WebSocketMessage {
        message_type: "connected".to_string(),
        data: json!({"status": "connected"}),
    };
    
    if let Ok(msg) = serde_json::to_string(&initial_message) {
        let _ = sender.send(Message::Text(msg)).await;
    }
    
    // Handle incoming messages
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json_msg) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json_msg)).await.is_err() {
                    break;
                }
            }
        }
    });
    
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(_) => {
                    // Handle incoming text messages if needed
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    });
    
    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }
}

pub fn broadcast_todo_update(message_type: &str, data: serde_json::Value) {
    let message = WebSocketMessage {
        message_type: message_type.to_string(),
        data,
    };
    
    let _ = BROADCASTER.send(message);
}