use crate::components::Board;
use crate::shared::*;
use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::*;
use leptos_router::params::Params;
use web_sys::{MessageEvent, WebSocket};

#[derive(Params, PartialEq, Clone)]
struct GameParams {
    room_code: Option<String>,
}

#[component]
pub fn Game() -> impl IntoView {
    let params = use_params::<GameParams>();

    let room_code = move || {
        params.with(|p| {
            p.as_ref()
                .ok()
                .and_then(|params| params.room_code.clone())
                .unwrap_or_else(|| "Unknown".to_string())
        })
    };

    let (ws, set_ws) = signal_local::<Option<WebSocket>>(None);
    let (player_color, set_player_color) = signal::<Option<PlayerColor>>(None);
    let (fen, set_fen) =
        signal("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    let (white_time, set_white_time) = signal(600u64);
    let (black_time, set_black_time) = signal(600u64);
    let (current_turn, set_current_turn) = signal(PlayerColor::White);
    let (status, set_status) = signal("Connecting...".to_string());
    let (game_over, set_game_over) = signal(false);

    let make_move = move |from: String, to: String| {
        if let Some(socket) = ws.get() {
            let msg = ClientMessage::MakeMove {
                from,
                to,
                promotion: None,
            };
            if let Ok(json) = serde_json::to_string(&msg) {
                let _ = socket.send_with_str(&json);
            }
        }
    };

    view! {
        <div class="game-container">
            <div class="game-info">
                <h2>"Room: " {room_code}</h2>
                <p class="status">{status}</p>
                {move || player_color.get().map(|c| {
                    view! { <p class="player-color">"You are: " {format!("{:?}", c)}</p> }
                })}
            </div>

            <div class="game-board-wrapper">
                {move || {
                    let is_black = player_color.get() == Some(PlayerColor::Black);
                    if is_black {
                        Either::Left(view! {
                            <div class="timer timer-white">
                                "White: " {move || format_time(white_time.get())}
                            </div>
                            <Board
                                fen=fen
                                player_color=player_color
                                current_turn=current_turn
                                on_move=make_move
                                game_over=game_over
                            />
                            <div class="timer timer-black">
                                "Black: " {move || format_time(black_time.get())}
                            </div>
                        })
                    } else {
                        Either::Right(view! {
                            <div class="timer timer-black">
                                "Black: " {move || format_time(black_time.get())}
                            </div>
                            <Board
                                fen=fen
                                player_color=player_color
                                current_turn=current_turn
                                on_move=make_move
                                game_over=game_over
                            />
                            <div class="timer timer-white">
                                "White: " {move || format_time(white_time.get())}
                            </div>
                        })
                    }
                }}
            </div>
        </div>
    }
}

fn format_time(seconds: u64) -> String {
    let mins = seconds / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}", mins, secs)
}
