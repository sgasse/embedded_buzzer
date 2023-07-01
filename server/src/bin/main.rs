use std::{net::SocketAddr, num::Wrapping, time::Duration};

use axum::{
    extract::{
        ws::{self, WebSocket},
        ConnectInfo, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router, ServiceExt,
};
use common::Message;
use futures_util::{SinkExt, StreamExt};
use postcard::{from_bytes, to_allocvec};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Open sockets
    tokio::spawn(async {
        let listener = TcpListener::bind("192.168.100.1:8000").await.unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            tokio::spawn(process_incoming(socket));
        }
    });

    tokio::spawn(async {
        let send_to_board = TcpListener::bind("192.168.100.1:8001").await.unwrap();

        loop {
            let (socket, _) = send_to_board.accept().await.unwrap();
            tokio::spawn(process_outgoing(socket));
        }
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/ws", get(ws_handler))
        .nest_service("/assets", ServeDir::new("assets"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Let's get this started!"
}

async fn process_incoming(mut socket: TcpStream) {
    let mut buf = vec![0; 1000];
    loop {
        match socket.read(&mut buf).await {
            Ok(num_read) => {
                // println!("Read {} bytes", num_read,);

                match from_bytes::<Message>(&buf[0..num_read]) {
                    Ok(message) => {
                        println!("Got message {message:?}");
                    }
                    Err(e) => {
                        println!("Error in deserializing received message: {e}");
                    }
                }
            }
            Err(e) => {
                println!("Error on reading data: {e}");
                return;
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn process_outgoing(mut socket: TcpStream) {
    let mut counter = Wrapping(0);

    let init_game = Message::InitGame;
    let serialized = to_allocvec(&init_game).unwrap();
    if let Err(e) = socket.write_all(&serialized).await {
        println!("Error in sending init data: {e}");
    }

    loop {
        let ping = Message::Ping(counter.0);
        match socket.write_all(&to_allocvec(&ping).unwrap()).await {
            Ok(()) => {
                // println!("Wrote data with counter {}", counter.0);
            }
            Err(e) => {
                println!("Error in writing: {e}");
                return;
            }
        }

        counter += 1;

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| process_websocket(socket, addr))
}

async fn process_websocket(stream: WebSocket, addr: SocketAddr) {
    println!("New websocket client: {}", addr);

    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    tokio::spawn(async move {
        let mut counter = 0;
        loop {
            if let Err(e) = sender
                .send(ws::Message::Text(format!("Msg #{counter}")))
                .await
            {
                println!("Error sending to websocket client {addr}: {e}, closing socket");
                return;
            }
            counter += 1;
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    });

    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        if let ws::Message::Text(msg) = message {
            println!("Got text message: {}", msg);
        }
    }

    return;
}

// var a = new WebSocket('ws://127.0.0.1:3001/ws');
// a.addEventListener("message", (event) => console.log(event));
// a.send("Hello");
