use crate::shared::PlayerColor;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Board(fen: ReadSignal<String>) -> impl IntoView {
    view! {
        <div class="chess-board">
            <For
                each=move || {
                    let mut squares = Vec::new();
                    for rank in (0..8).rev() {
                        for file in 0..8 {
                            squares.push((file, rank));
                        }
                    }
                    squares
                }
                key=|(f, r)| format!("{}{}", f, r)
                children=move |(file, rank)| {
                    let square_name = format!("{}{}",
                        (b'a' + file as u8) as char,
                        rank + 1
                    );
                    let piece = create_memo(move |_| {
                        let board_state = parse_fen(&fen.get());
                        board_state.get(&square_name).cloned()
                    });
                    let is_light = (file + rank) % 2 == 0;
                    view! {
                        <div class="square" class:light=is_light class:dark=!is_light>{move || piece.get().map(|p| {
                                view! { <div class="piece">{get_piece_symbol(&p)}</div> }
                            })} </div>
                    }
                }
            />
                </div>
    }
}

fn parse_fen(fen: &str) -> HashMap<String, String> {
    let mut board = HashMap::new();
    let parts: Vec<&str> = fen.split_whitespace().collect();

    if parts.is_empty() {
        return board;
    }

    let ranks: Vec<&str> = parts[0].split('/').collect();

    for (rank_idx, rank_str) in ranks.iter().enumerate() {
        let rank = 8 - rank_idx;
        let mut file = 0;

        for ch in rank_str.chars() {
            if ch.is_ascii_digit() {
                file += ch.to_digit(10).unwrap() as usize;
            } else {
                let square = format!("{}{}", (b'a' + file as u8) as char, rank);
                board.insert(square, ch.to_string());
                file += 1;
            }
        }
    }

    board
}

fn get_piece_symbol(piece: &str) -> &'static str {
    match piece {
        "K" => "♔",
        "Q" => "♕",
        "R" => "♖",
        "B" => "♗",
        "N" => "♘",
        "P" => "♙",
        "k" => "♚",
        "q" => "♛",
        "r" => "♜",
        "b" => "♝",
        "n" => "♞",
        "p" => "♟",
        _ => "",
    }
}
