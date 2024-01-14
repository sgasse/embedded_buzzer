use common::{Message, MsgBuffer};
use postcard::to_allocvec;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::UiBackendRouter;

pub async fn board_connection(mut socket: TcpStream, uib_router: UiBackendRouter) {
    let (mut reader, mut writer) = socket.split();

    // Get server channels.
    let ui_tx = uib_router.frontend_tx.clone();
    let mut board_rx = uib_router.board_rx.resubscribe();

    // Initialize buffer to read messages to.
    let mut read_buf = MsgBuffer::<2000>::default();

    // Send init instruction.
    let serialized = to_allocvec(&Message::InitBoard).unwrap();
    if let Err(e) = writer.write_all(&serialized).await {
        println!("Error in sending init data: {e}");
    }

    loop {
        tokio::select! {
            read = reader.read(read_buf.as_buf()) => {
                match read {
                    Ok(num_read) => {
                        read_buf.cursor += num_read;
                        read_buf.process_msgs_ok(|msg| {
                            ui_tx.send(msg).ok();
                        });
                    }
                    Err(e) => {
                        println!("Error in reading data: {e}");
                        return;
                    }
                }
            }
            recv = board_rx.recv() => {
                match recv {
                    Ok(msg) => match writer.write_all(&to_allocvec(&msg).unwrap()).await {
                        Ok(()) => {}
                        Err(e) => {
                            println!("Error in writing: {e}");
                            return;
                        }
                    },
                    Err(e) => {
                        println!("Error in receiving from channel: {e}");
                        return;
                    }
                }
            },
        }
    }
}
