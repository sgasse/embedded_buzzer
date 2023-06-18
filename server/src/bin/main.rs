use std::{net::SocketAddr, num::Wrapping, time::Duration};

use axum::{routing::get, Router};
use common::Message;
use postcard::{from_bytes, to_allocvec};
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
