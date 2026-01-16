use leptos::prelude::*;

#[component]
pub fn Board() -> impl IntoView {
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
                    let is_light = (file + rank) % 2 == 0;
                    view! {
                        <div class="square" class:light=is_light class:dark=!is_light> </div>
                    }
                }
            />
                </div>
    }
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
