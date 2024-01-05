use std::time::Duration;

use common::{Message, MsgBuffer};
use postcard::to_allocvec;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::UiBackendRouter;

pub async fn process_incoming(mut socket: TcpStream, uib_router: UiBackendRouter) {
    let ui_tx = uib_router.frontend_tx.clone();
    let mut buf = MsgBuffer::<2000>::default();

    loop {
        match socket.read(buf.as_buf()).await {
            Ok(num_read) => {
                buf.cursor += num_read;
                buf.process_msgs_ok(|msg| {
                    ui_tx.send(msg).ok();
                });
            }
            Err(e) => {
                println!("Error on reading data: {e}");
                return;
            }
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

pub async fn process_outgoing(mut socket: TcpStream, uib_router: UiBackendRouter) {
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
    }
}
