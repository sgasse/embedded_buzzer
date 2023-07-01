use std::{num::Wrapping, time::Duration};

use common::Message;
use postcard::{from_bytes, to_allocvec};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn process_incoming(mut socket: TcpStream) {
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

pub async fn process_outgoing(mut socket: TcpStream) {
    let mut counter = Wrapping(0);

    let init_game = Message::InitGame;
    let serialized = to_allocvec(&init_game).unwrap();
    if let Err(e) = socket.write_all(&serialized).await {
        println!("Error in sending init data: {e}");
    }

    loop {
        let ping = Message::Ping(counter.0);
        match socket.write_all(&to_allocvec(&ping).unwrap()).await {
            Ok(()) => {}
            Err(e) => {
                println!("Error in writing: {e}");
                return;
            }
        }

        counter += 1;

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
