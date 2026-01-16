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
    pub fn new(time_control_ms: u64) -> Self {
        Self {
            board: Board::default(),
            moves: Vec::new(),
            white_time_ms: time_control_ms,
            black_time_ms: time_control_ms,
            last_move_time: Self::current_time_ms(),
            game_over: false,
            result: None,
        }
    }

    pub fn update_time(&mut self) {
        let current_time = Self::current_time_ms();
        let elapsed = current_time.saturating_sub(self.last_move_time);

        match self.board.side_to_move() {
            Color::White => {
                if self.white_time_ms > elapsed {
                    self.white_time_ms -= elapsed;
                } else {
                    self.white_time_ms = 0;
                    self.game_over = true;
                    self.result = Some(GameResult::Timeout {
                        winner: PlayerColor::Black,
                    });
                }
            }
            Color::Black => {
                if self.black_time_ms > elapsed {
                    self.black_time_ms -= elapsed;
                } else {
                    self.black_time_ms = 0;
                    self.game_over = true;
                    self.result = Some(GameResult::Timeout {
                        winner: PlayerColor::White,
                    });
                }
            }
        }
    }

    pub fn get_fen(&self) -> String {
        format!("{}", self.board)
    }

    pub fn current_turn(&self) -> PlayerColor {
        match self.board.side_to_move() {
            Color::White => PlayerColor::White,
            Color::Black => PlayerColor::Black,
        }
    }

    fn current_time_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}
