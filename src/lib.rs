use std::{sync::Arc};

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
use serde::Serialize;
use shuttle_service::ShuttleAxum;
use sync_wrapper::SyncWrapper;
use tokio::{
    sync::{watch},
};
use tokio::sync::broadcast;
use twitch_irc::message::ServerMessage;

pub mod twitch;

struct AppState {
    // Receiver for websocket messages
    ws_rx: watch::Receiver<Message>,

    /// Sender for Twitch messages
    twitch_tx: broadcast::Sender<ServerMessage>,
}


#[derive(Serialize)]
struct Response {
    datetime: DateTime<Utc>,
    data: ServerMessage,
}

#[shuttle_service::main]
async fn main() -> ShuttleAxum {
    let (ws_tx, ws_rx) = watch::channel(Message::Text("{}".to_string()));

    let (twitch_tx, _twitch_rx) = broadcast::channel(100);

    let state = Arc::new(AppState {
        ws_rx,
        twitch_tx: twitch_tx.clone(),
    });


    twitch::initialize_twitch_chat("techygrrrl").await;
    // twitch::initialize_twitch_chat("techygrrrl", twitch_tx).await;


    // tokio::spawn(async move {
    //     twitch::initialize_twitch_chat("techygrrrl", twitch_tx).await;
    // }).await.unwrap();

    // tokio::spawn(async move {
    //     // loop {
    //     //     // let is_up = client.get(uri.clone()).await;
    //     //     let response = Response {
    //     //         datetime: Utc::now(),
    //     //     };
    //     //     let msg = serde_json::to_string(&response).unwrap();
    //     //     if ws_tx.send(Message::Text(msg)).is_err() {
    //     //         break;
    //     //     }
    //     //     sleep(duration).await;
    //     // }
    // });

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
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

/// Web socket server implementation
async fn websocket(stream: WebSocket, state: Arc<AppState>) {

    let mut twitch_rx = state.twitch_tx.subscribe();


    // TODO: twitch thangs
    // twitch::initialize_twitch_chat("techygrrrl");

    // twitch::initialize_twitch_chat("techygrrrl", (message) -> {
    //     // do things with the message
    //     print(message)
    // });

    // By splitting we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // let mut ws_rx = {
    //     // TODO: ????
    //     state.ws_rx.clone()
    // };

    // This task will receive watch messages and forward it to this connected client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(server_message) = twitch_rx.recv().await {
            let server_message = serde_json::to_string(&server_message).unwrap();

            if sender.send(Message::Text(server_message)).await.is_err() {
                break;
            }
        }
        // while let Ok(()) = ws_rx.changed().await {
        //     let msg = ws_rx.borrow().clone();
        //
        //     // TODO: Send ServerMessage from Twitch
        //     if sender.send(msg).await.is_err() {
        //         break;
        //     }
        // }
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

}

/// This is the main page.
async fn index() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}
