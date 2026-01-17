use crate::shared::PlayerColor;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Board<F>(
    fen: ReadSignal<String>,
    player_color: ReadSignal<Option<PlayerColor>>,
    current_turn: ReadSignal<PlayerColor>,
    on_move: F,
    game_over: ReadSignal<bool>,
) -> impl IntoView
where
    F: Fn(String, String, Option<String>) + 'static + Copy + Send,
{
    let (selected_square, set_selected_square) = signal::<Option<String>>(None);
    let (promotion_state, set_promotion_state) = signal::<Option<(String, String)>>(None);

    let board_state = Memo::new(move |_| parse_fen(&fen.get()));

    let handle_square_click = move |square: String| {
        if game_over.get() {
            return;
        }

        let my_color = player_color.get();
        let turn = current_turn.get();

        if my_color != Some(turn) {
            return;
        }

        if let Some(from) = selected_square.get() {
            if from != square {
                if is_promotion_move(&from, &square, &board_state.get(), turn) {
                    set_promotion_state.set(Some((from.clone(), square.clone())));
                } else {
                    on_move(from.clone(), square.clone(), None);
                }
            }
            set_selected_square.set(None);
        } else {
            let current_board = board_state.get();
            if let Some(piece) = current_board.get(&square) {
                let is_white_piece = piece.chars().next().unwrap().is_uppercase();
                let is_my_piece = match turn {
                    PlayerColor::White => is_white_piece,
                    PlayerColor::Black => !is_white_piece,
                };

                if is_my_piece {
                    set_selected_square.set(Some(square));
                }
            }
        }
    };

    let handle_promotion = move |piece: &str| {
        if let Some((from, to)) = promotion_state.get() {
            on_move(from, to, Some(piece.to_string()));
            set_promotion_state.set(None);
        }
    };

    let is_flipped = move || player_color.get() == Some(PlayerColor::Black);

    view! {
        <div class="board-container">
            <div class="chess-board" class:flipped=is_flipped>
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
                        let square_name = format!("{}{}", (b'a' + file as u8) as char, rank + 1);
                        let sq_click = square_name.clone();
                        let sq_selected = square_name.clone();
                        let sq_memo = square_name.clone();

                        let piece = Memo::new(move |_| {
                            board_state.get().get(&sq_memo).cloned()
                        });

                        let is_selected = move || {
                            selected_square.get() == Some(sq_selected.clone())
                        };

                        let is_light = (file + rank) % 2 == 0;

                        view! {
                            <div
                                class="square"
                                class:light=is_light
                                class:dark=!is_light
                                class:selected=is_selected
                                on:click=move |_| handle_square_click(sq_click.clone())
                            >
                                {move || piece.get().map(|p| {
                                    view! { <div class="piece">{get_piece_symbol(&p)}</div> }
                                })}
                            </div>
                        }
                    }
                />
            </div>

            {move || promotion_state.get().map(|_| {
                let color = current_turn.get();
                view! {
                    <div class="promotion-overlay">
                        <div class="promotion-dialog">
                            <h3>"Choose promotion piece"</h3>
                            <div class="promotion-pieces">
                                <button
                                    class="promotion-button"
                                    on:click=move |_| handle_promotion("q")
                                >
                                    {if color == PlayerColor::White { "♕" } else { "♛" }}
                                </button>
                                <button
                                    class="promotion-button"
                                    on:click=move |_| handle_promotion("r")
                                >
                                    {if color == PlayerColor::White { "♖" } else { "♜" }}
                                </button>
                                <button
                                    class="promotion-button"
                                    on:click=move |_| handle_promotion("b")
                                >
                                    {if color == PlayerColor::White { "♗" } else { "♝" }}
                                </button>
                                <button
                                    class="promotion-button"
                                    on:click=move |_| handle_promotion("n")
                                >
                                    {if color == PlayerColor::White { "♘" } else { "♞" }}
                                </button>
                            </div>
                        </div>
                    </div>
                }
            })}
        </div>
    }
}

fn is_promotion_move(
    from: &str,
    to: &str,
    board: &HashMap<String, String>,
    turn: PlayerColor,
) -> bool {
    if let Some(piece) = board.get(from) {
        let is_pawn = piece == "P" || piece == "p";
        if !is_pawn {
            return false;
        }

        let to_rank = to.chars().nth(1).and_then(|c| c.to_digit(10)).unwrap_or(0);

        match turn {
            PlayerColor::White => to_rank == 8,
            PlayerColor::Black => to_rank == 1,
        }
    } else {
        false
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
