use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    RwLock,
};

static ID: AtomicU32 = AtomicU32::new(0);
static COUNTER: AtomicU32 = AtomicU32::new(0);
static USERS: Lazy<RwLock<HashMap<u32, UnboundedSender<u32>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub async fn click_count() -> impl IntoResponse {
    let count = COUNTER.load(Ordering::Relaxed);
    format!("{count}")
}

pub async fn click_sock(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

async fn handle_socket(stream: WebSocket) {
    let user_id = ID.fetch_add(1, Ordering::Relaxed);
    let (mut sender, mut receiver) = stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<u32>();
    USERS.write().await.insert(user_id, tx);

    tokio::spawn(async move {
        while let Some(count) = rx.recv().await {
            if let Err(e) = sender.send(Message::Text(format!("{count}"))).await {
                eprintln!("Error sending over click socket: {e}");
            }
        }

        if let Err(e) = sender.close().await {
            eprintln!("Error closing click socket sender: {e}");
        }
    });

    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(_) => {
                println!("Received click!");
                let new_count = COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
                for tx in USERS.read().await.values() {
                    if let Err(e) = tx.send(new_count) {
                        eprintln!("Error sending user{user_id} click: {e}");
                    }
                }
            }
            _ => (),
        }
    }

    USERS.write().await.remove(&user_id);
}
