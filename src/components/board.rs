use leptos::prelude::*;

#[component]
pub fn Board() -> impl IntoView {
    view! {
        <div>"I am a board"</div>
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
