use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRoom {
    pub room_code: String,
    pub white_player: Option<String>,
    pub black_player: Option<String>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    CreateRoom { room_code: String },
    JoinRoom { room_code: String },
    ChatMessage { text: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    RoomCreated {
        room_code: String,
        player_color: PlayerColor,
    },
    RoomJoined {
        room_code: String,
        player_color: PlayerColor,
    },
    ChatMessage {
        text: String,
    },
    Error {
        message: String,
    },
}
