#![no_std]

use defmt::{warn, Format};
use postcard::take_from_bytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Format)]
pub enum Message {
    InitBoard,
    InitReactionGame(u32),
    Ping(u32),
    ButtonPress(ButtonPress),
    LedUpdate(LedUpdate),
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Format)]
pub struct ButtonPress {
    pub button_id: u8,
    pub millis_since_init: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Format)]
pub struct LedUpdate {
    pub button_id: u8,
    pub on: bool,
}

pub struct MsgBuffer<const BUF_SIZE: usize> {
    pub cursor: usize,
    pub buf: [u8; BUF_SIZE],
}

impl<const T: usize> MsgBuffer<T> {
    pub fn process_msgs_ok<F>(&mut self, callback: F) -> bool
    where
        F: Fn(Message),
    {
        let mut to_deserialize = &self.buf[0..self.cursor];

        loop {
            match take_from_bytes::<Message>(to_deserialize) {
                Ok((message, unused)) => {
                    to_deserialize = unused;
                    callback(message);

                    if to_deserialize.is_empty() {
                        self.cursor = 0;
                        return true;
                    }
                }
                Err(_) => {
                    warn!("Could not deserialize buffer, skipping...");

                    // Example: cursor_pos at 12, read 8 bytes, 4 left over
                    // ---- ---- | ---- |
                    //                  cursor_pos: 12
                    // -> copy from: 8..12 to 0..4
                    let left_over_len = to_deserialize.len();
                    let copy_start_idx = self.cursor - left_over_len;

                    for idx in 0..left_over_len {
                        self.buf[idx] = self.buf[copy_start_idx + idx];
                    }
                    self.cursor = left_over_len;

                    return false;
                }
            }
        }
    }

    pub fn as_buf(&mut self) -> &mut [u8] {
        &mut self.buf[self.cursor..]
    }
}

impl<const T: usize> Default for MsgBuffer<T> {
    fn default() -> Self {
        Self {
            cursor: 0,
            buf: [0u8; T],
        }
    }
}
