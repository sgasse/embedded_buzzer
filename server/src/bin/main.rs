use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};

use axum::{routing::get, Extension, Router};
use common::SERVER_ADDR;
use server::{
    net_sockets::{process_incoming, process_outgoing},
    websocket::ws_handler,
    UiBackendRouterInner,
};
use tokio::{net::TcpListener, sync::broadcast};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let (frontend_tx, frontend_rx) = broadcast::channel(10);
    let (board_tx, board_rx) = broadcast::channel(10);
    let uib_router = Arc::new(UiBackendRouterInner {
        frontend_tx,
        frontend_rx,
        board_tx,
        board_rx,
    });

    let server_ip = Ipv4Addr::from(SERVER_ADDR);

    // Open sockets
    let uib_router_ = uib_router.clone();
    tokio::spawn(async move {
        let uib_router_ = uib_router_.clone();
        let listener = TcpListener::bind(SocketAddrV4::new(server_ip, 8000))
            .await
            .unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            tokio::spawn(process_incoming(socket, uib_router_.clone()));
        }
    });

    let uib_router_ = uib_router.clone();
    tokio::spawn(async move {
        let uib_router_ = uib_router_.clone();
        let send_to_board = TcpListener::bind(SocketAddrV4::new(server_ip, 8001))
            .await
            .unwrap();

        loop {
            let (socket, _) = send_to_board.accept().await.unwrap();
            tokio::spawn(process_outgoing(socket, uib_router_.clone()));
        }
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/ws", get(ws_handler))
        .nest_service("/reaction", ServeFile::new("assets/reaction.html"))
        .nest_service("/quiz", ServeFile::new("assets/quiz.html"))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(Extension(uib_router));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Let's get this started!"
}
