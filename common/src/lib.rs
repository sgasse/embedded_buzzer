#![no_std]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct GameInfo {
    pub instruction: u32,
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Message {
    InitGame,
    Ping(u32),
    ButtonPress(ButtonPress),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ButtonPress {
    pub button_id: u8,
    pub millis_since_init: u32,
}
