use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRoom {
    pub room_code: String,
    pub white_player: Option<String>,
    pub black_player: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlayerColor {
    White,
    Black,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveRecord {
    pub san: String, // Standard Algebraic Notation
    pub from: String,
    pub to: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    CreateRoom {
        room_code: String,
    },
    JoinRoom {
        room_code: String,
    },
    MakeMove {
        from: String,
        to: String,
        promotion: Option<String>,
    },
    Resign,
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
    GameState {
        fen: String,
        moves: Vec<MoveRecord>,
        white_time: u64,
        black_time: u64,
        current_turn: PlayerColor,
    },
    MoveMade {
        from: String,
        to: String,
        san: String,
        fen: String,
    },
    InvalidMove {
        reason: String,
    },
    OpponentJoined,
    OpponentLeft,
    GameOver {
        result: GameResult,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameResult {
    WhiteWins,
    BlackWins,
    Draw,
    Resignation { winner: PlayerColor },
    Timeout { winner: PlayerColor },
}
