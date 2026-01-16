#[cfg(feature = "ssr")]
use crate::shared::{GameResult, MoveRecord, PlayerColor};
#[cfg(feature = "ssr")]
use chess::{Board, ChessMove, Color, Piece, Square};
#[cfg(feature = "ssr")]
use std::str::FromStr;
#[cfg(feature = "ssr")]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "ssr")]
pub struct GameState {
    pub board: Board,
    pub moves: Vec<MoveRecord>,
    pub white_time_ms: u64,
    pub black_time_ms: u64,
    pub last_move_time: u64,
    pub game_over: bool,
    pub result: Option<GameResult>,
}

#[cfg(feature = "ssr")]
impl GameState {
    fn current_time_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}
