#![no_std]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct GameInfo {
    pub instruction: u32,
    pub id: u32,
}
