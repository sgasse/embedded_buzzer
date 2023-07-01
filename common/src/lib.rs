#![no_std]

use defmt::Format;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Format)]
pub enum Message {
    InitBoard,
    InitReactionGame(u32),
    Ping(u32),
    ButtonPress(ButtonPress),
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Format)]
pub struct ButtonPress {
    pub button_id: u8,
    pub millis_since_init: u32,
    pub millis_reaction: i32,
}
