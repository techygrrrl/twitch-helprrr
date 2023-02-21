use std::{sync::Arc, time::Duration};

use axum::{
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::{Html, IntoResponse},
    Router, routing::get,
};
use axum::extract::State;
use chrono::{DateTime, Utc};
use futures::{SinkExt, StreamExt};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use serde::Serialize;
use shuttle_service::ShuttleAxum;
use sync_wrapper::SyncWrapper;
use tokio::{
    sync::{Mutex, watch},
    time::sleep,
};

pub mod twitch;

struct AppState {
    clients_count: usize,
    rx: watch::Receiver<Message>,
}

const PAUSE_SECS: u64 = 15;
const STATUS_URI: &str = "https://api.shuttle.rs/status";

#[derive(Serialize)]
struct Response {
    clients_count: usize,
    datetime: DateTime<Utc>,
    is_up: bool,
}

#[shuttle_service::main]
async fn main() -> ShuttleAxum {
    let (tx, rx) = watch::channel(Message::Text("{}".to_string()));

    let state = Arc::new(Mutex::new(AppState {
        clients_count: 0,
        rx,
    }));

    // Client implementation of websockets

    // Spawn a thread to continually check the status of the api
    let state_send = state.clone();
    tokio::spawn(async move {
        let duration = Duration::from_secs(PAUSE_SECS);

        let https = HttpsConnector::new();

        let client = Client::builder().build::<_, hyper::Body>(https);
        let uri: Uri = STATUS_URI.parse().unwrap();

        loop {
            let is_up = client.get(uri.clone()).await;
            let is_up = is_up.is_ok();

            let response = Response {
                clients_count: state_send.lock().await.clients_count,
                datetime: Utc::now(),
                is_up,
            };
            let msg = serde_json::to_string(&response).unwrap();

            if tx.send(Message::Text(msg)).is_err() {
                break;
            }

            sleep(duration).await;
        }
    });

    let router = Router::new()
        .route("/", get(index))
        .route("/websocket", get(websocket_handler))
        .with_state(state);

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}

/// Handler that supports the server web socket implementation
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<Mutex<AppState>>) {
    // By splitting we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    let mut rx = {
        let mut state = state.lock().await;
        state.clients_count += 1;
        state.rx.clone()
    };

    // This task will receive watch messages and forward it to this connected client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(()) = rx.changed().await {
            let msg = rx.borrow().clone();

            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // This task will receive messages from this client.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            println!("this example does not read any messages, but got: {text}");
        }
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // This client disconnected
    state.lock().await.clients_count -= 1;
}

/// This is the main page.
async fn index() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}
