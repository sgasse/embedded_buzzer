use std::{num::Wrapping, time::Duration};

use common::Message;
use postcard::{from_bytes, to_allocvec};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::UiBackendRouter;

pub async fn process_incoming(mut socket: TcpStream, uib_router: UiBackendRouter) {
    let ui_tx = uib_router.frontend_tx.clone();
    let mut buf = vec![0; 1000];

    loop {
        match socket.read(&mut buf).await {
            Ok(num_read) => match from_bytes::<Message>(&buf[0..num_read]) {
                Ok(message) => {
                    println!("Board: {message:?}");
                    ui_tx.send(message).ok();
                }
                Err(e) => {
                    println!("Error in deserializing received message: {e}");
                }
            },
            Err(e) => {
                println!("Error on reading data: {e}");
                return;
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

pub async fn process_outgoing(mut socket: TcpStream, uib_router: UiBackendRouter) {
    // let mut counter = Wrapping(0);

    let init_game = Message::InitBoard;
    let serialized = to_allocvec(&init_game).unwrap();
    if let Err(e) = socket.write_all(&serialized).await {
        println!("Error in sending init data: {e}");
    }

    let mut board_rx = uib_router.board_rx.resubscribe();
    loop {
        match board_rx.recv().await {
            Ok(msg) => match socket.write_all(&to_allocvec(&msg).unwrap()).await {
                Ok(()) => {}
                Err(e) => {
                    println!("Error in writing: {e}");
                    return;
                }
            },
            Err(e) => {
                println!("Error in receiving from board_rx: {e}");
            }
        }
        // let ping = Message::Ping(counter.0);
        // match socket.write_all(&to_allocvec(&ping).unwrap()).await {
        //     Ok(()) => {}
        //     Err(e) => {
        //         println!("Error in writing: {e}");
        //         return;
        //     }
        // }

        // counter += 1;

        // tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
