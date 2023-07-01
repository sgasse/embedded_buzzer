use std::net::SocketAddr;

use axum::{routing::get, Router};
use server::{
    net_sockets::{process_incoming, process_outgoing},
    websocket::ws_handler,
};
use tokio::net::TcpListener;
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
