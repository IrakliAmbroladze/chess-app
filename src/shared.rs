use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerColor {
    White,
    Black,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveRecord {
    pub san: String,
    pub from: String,
    pub to: String,
    pub timestamp: u64,
}
