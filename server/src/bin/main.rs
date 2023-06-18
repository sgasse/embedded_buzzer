use std::{net::SocketAddr, num::Wrapping, time::Duration};

use axum::{routing::get, Router};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

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

    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
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
                println!(
                    "Read {} bytes: {}",
                    num_read,
                    String::from_utf8_lossy(&buf[0..num_read])
                );
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
    loop {
        match socket
            .write_all(format!("Response {}", counter.0).as_bytes())
            .await
        {
            Ok(()) => {
                println!("Wrote response with counter {}", counter.0);
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
