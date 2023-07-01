//! Websocket connection module.
//!
//! Test in the browser with:
//! ```no_run
//! var conn = new WebSocket('ws://127.0.0.1:3001/ws');
//! conn.addEventListener("message", (event) => console.log(event));
//! conn.send("Hello from frontend");
//! ```
use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{self, WebSocket},
        ConnectInfo, WebSocketUpgrade,
    },
    response::IntoResponse,
    Extension,
};
use futures_util::{SinkExt, StreamExt};

use crate::UiBackendRouter;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(uib_router): Extension<UiBackendRouter>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| process_websocket(socket, addr, uib_router))
}

async fn process_websocket(stream: WebSocket, addr: SocketAddr, uib_router: UiBackendRouter) {
    println!("New websocket client: {}", addr);

    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    let mut ui_rx = uib_router.frontend_rx.resubscribe();

    // Receive updates from board.
    tokio::spawn(async move {
        loop {
            match ui_rx.recv().await {
                Ok(msg) => {
                    let msg = ws::Message::Text(serde_json::to_string(&msg).unwrap());
                    if let Err(e) = sender.send(msg).await {
                        println!(
                            "Could not send to websocket client {addr}, dropping connection ({e})."
                        );
                        return;
                    }
                }
                Err(e) => println!("Error receiving: {e}"),
            }
        }
    });

    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        if let ws::Message::Text(msg) = message {
            println!("Frontend (via {}): {}", addr, msg);

            match serde_json::from_str(&msg) {
                Ok(msg) => {
                    uib_router.board_tx.send(msg).ok();
                }
                Err(_) => {}
            }
        }
    }

    return;
}
