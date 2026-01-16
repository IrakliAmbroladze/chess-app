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

    pub fn make_move(
        &mut self,
        from_str: &str,
        to_str: &str,
        promotion: Option<&str>,
    ) -> Result<String, String> {
        if self.game_over {
            return Err("Game is over".to_string());
        }

        self.update_time();

        let from = Square::from_str(from_str).map_err(|_| "Invalid from square")?;
        let to = Square::from_str(to_str).map_err(|_| "Invalid to square")?;

        let promotion_piece = if let Some(p) = promotion {
            match p {
                "q" => Some(Piece::Queen),
                "r" => Some(Piece::Rook),
                "b" => Some(Piece::Bishop),
                "n" => Some(Piece::Knight),
                _ => None,
            }
        } else {
            None
        };

        let chess_move = ChessMove::new(from, to, promotion_piece);

        if !self.board.legal(chess_move) {
            return Err("Illegal move".to_string());
        }

        let san = self.move_to_san(&chess_move);

        self.board = self.board.make_move_new(chess_move);

        let move_record = MoveRecord {
            san: san.clone(),
            from: from_str.to_string(),
            to: to_str.to_string(),
            timestamp: Self::current_time_ms(),
        };

        self.moves.push(move_record);
        self.last_move_time = Self::current_time_ms();

        if self.board.status() != chess::BoardStatus::Ongoing {
            self.game_over = true;
            self.result = Some(self.determine_result());
        }

        Ok(san)
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

    fn determine_result(&self) -> GameResult {
        match self.board.status() {
            chess::BoardStatus::Checkmate => match self.board.side_to_move() {
                Color::White => GameResult::BlackWins,
                Color::Black => GameResult::WhiteWins,
            },
            chess::BoardStatus::Stalemate => GameResult::Draw,
            _ => GameResult::Draw,
        }
    }

    // Simplified SAN notation generator
    fn move_to_san(&self, chess_move: &ChessMove) -> String {
        let piece = self.board.piece_on(chess_move.get_source());
        let capture = self.board.piece_on(chess_move.get_dest()).is_some();

        let mut san = String::new();

        if let Some(p) = piece {
            match p {
                Piece::King => san.push('K'),
                Piece::Queen => san.push('Q'),
                Piece::Rook => san.push('R'),
                Piece::Bishop => san.push('B'),
                Piece::Knight => san.push('N'),
                Piece::Pawn => {
                    if capture {
                        san.push(
                            format!("{}", chess_move.get_source())
                                .chars()
                                .next()
                                .unwrap(),
                        );
                    }
                }
            }
        }

        if capture {
            san.push('x');
        }

        san.push_str(&format!("{}", chess_move.get_dest()));

        if let Some(promo) = chess_move.get_promotion() {
            san.push('=');
            match promo {
                Piece::Queen => san.push('Q'),
                Piece::Rook => san.push('R'),
                Piece::Bishop => san.push('B'),
                Piece::Knight => san.push('N'),
                _ => {}
            }
        }

        san
    }

    fn current_time_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}
