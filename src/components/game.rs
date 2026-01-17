use crate::components::Board;
use crate::shared::*;
use leptos::either::Either;
use leptos::prelude::set_interval;
use leptos::prelude::*;
use leptos_router::hooks::*;
use leptos_router::params::Params;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

#[derive(Params, PartialEq, Clone)]
struct GameParams {
    room_code: Option<String>,
}

#[component]
pub fn Game() -> impl IntoView {
    let params = use_params::<GameParams>();
    let query = use_query_map();

    let room_code = move || {
        params.with(|p| {
            p.as_ref()
                .ok()
                .and_then(|params| params.room_code.clone())
                .unwrap_or_else(|| "Unknown".to_string())
        })
    };

    let action = move || query.with(|q| q.get("action").unwrap_or_else(|| "join".to_string()));

    let (ws, set_ws) = signal_local::<Option<WebSocket>>(None);
    let (player_color, set_player_color) = signal::<Option<PlayerColor>>(None);
    let (fen, set_fen) =
        signal("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    let (moves, set_moves) = signal::<Vec<MoveRecord>>(Vec::new());
    let (white_time, set_white_time) = signal(600u64);
    let (black_time, set_black_time) = signal(600u64);
    let (current_turn, set_current_turn) = signal(PlayerColor::White);
    let (status, set_status) = signal("Connecting...".to_string());
    let (game_over, set_game_over) = signal(false);

    set_interval(
        move || {
            if game_over.get() {
                return;
            }

            let turn = current_turn.get();
            match turn {
                PlayerColor::White => {
                    let current = white_time.get();
                    if current > 0 {
                        set_white_time.set(current - 1);
                    } else if current == 0 {
                        set_game_over.set(true);
                        set_status.set("Black wins on time!".to_string());
                    }
                }
                PlayerColor::Black => {
                    let current = black_time.get();
                    if current > 0 {
                        set_black_time.set(current - 1);
                    } else if current == 0 {
                        set_game_over.set(true);
                        set_status.set("White wins on time!".to_string());
                    }
                }
            }
        },
        std::time::Duration::from_secs(1),
    );

    Effect::new(move |_| {
        let ws_url = format!("ws://localhost:3000/ws");
        match WebSocket::new(&ws_url) {
            Ok(socket) => {
                let socket_clone = socket.clone();
                let room_code_val = room_code();
                let action_val = action();

                let onopen = Closure::wrap(Box::new(move || {
                    let msg = if action_val == "create" {
                        ClientMessage::CreateRoom {
                            room_code: room_code_val.clone(),
                        }
                    } else {
                        ClientMessage::JoinRoom {
                            room_code: room_code_val.clone(),
                        }
                    };

                    if let Ok(json) = serde_json::to_string(&msg) {
                        let _ = socket_clone.send_with_str(&json);
                    }
                }) as Box<dyn FnMut()>);
                socket.set_onopen(Some(onopen.as_ref().unchecked_ref()));
                onopen.forget();

                let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Some(txt) = e.data().as_string() {
                        if let Ok(msg) = serde_json::from_str::<ServerMessage>(&txt) {
                            handle_server_message(
                                msg,
                                set_player_color,
                                set_fen,
                                moves,
                                set_moves,
                                set_white_time,
                                set_black_time,
                                set_current_turn,
                                set_status,
                                set_game_over,
                            );
                        }
                    }
                }) as Box<dyn FnMut(MessageEvent)>);
                socket.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                onmessage.forget();

                set_ws.set(Some(socket));
            }
            Err(_) => {
                set_status.set("Failed to connect".to_string());
            }
        }
    });

    let make_move = move |from: String, to: String, promotion: Option<String>| {
        if let Some(socket) = ws.get() {
            let msg = ClientMessage::MakeMove {
                from,
                to,
                promotion,
            };
            if let Ok(json) = serde_json::to_string(&msg) {
                let _ = socket.send_with_str(&json);
            }
        }
    };

    let resign = move |_| {
        if let Some(socket) = ws.get() {
            let msg = ClientMessage::Resign;
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

            <div class="game-controls">
                <button
                    class="btn btn-danger"
                    on:click=resign
                    disabled=move || game_over.get()
                >
                    "Resign"
                </button>
            </div>

            <div class="move-history">
                <h3>"Move History"</h3>
                <div class="moves-list">
                    <For
                        each=move || moves.get()
                        key=|m| m.timestamp
                        children=move |m: MoveRecord| {
                            view! { <div class="move-item">{m.san}</div> }
                        }
                    />
                </div>
            </div>
        </div>
    }
}

fn handle_server_message(
    msg: ServerMessage,
    set_player_color: WriteSignal<Option<PlayerColor>>,
    set_fen: WriteSignal<String>,
    moves: ReadSignal<Vec<MoveRecord>>,
    set_moves: WriteSignal<Vec<MoveRecord>>,
    set_white_time: WriteSignal<u64>,
    set_black_time: WriteSignal<u64>,
    set_current_turn: WriteSignal<PlayerColor>,
    set_status: WriteSignal<String>,
    set_game_over: WriteSignal<bool>,
) {
    match msg {
        ServerMessage::RoomCreated { player_color, .. } => {
            set_player_color.set(Some(player_color));
            set_status.set("Waiting for opponent...".to_string());
        }
        ServerMessage::RoomJoined { player_color, .. } => {
            set_player_color.set(Some(player_color));
            set_status.set("Game starting...".to_string());
        }
        ServerMessage::GameState {
            fen,
            moves: game_moves,
            white_time,
            black_time,
            current_turn,
        } => {
            set_fen.set(fen);
            set_moves.set(game_moves);
            set_white_time.set(white_time / 1000);
            set_black_time.set(black_time / 1000);
            set_current_turn.set(current_turn);
            set_status.set("Game in progress".to_string());
        }
        ServerMessage::MoveMade { fen, san, .. } => {
            set_fen.set(fen.clone());
            let parts: Vec<&str> = fen.split_whitespace().collect();
            if parts.len() >= 2 {
                set_current_turn.set(if parts[1] == "w" {
                    PlayerColor::White
                } else {
                    PlayerColor::Black
                });
            }
            let mut new_moves = moves.get_untracked();
            new_moves.push(MoveRecord {
                san,
                from: String::new(),
                to: String::new(),
                timestamp: js_sys::Date::now() as u64,
            });
            set_moves.set(new_moves);
        }
        ServerMessage::OpponentJoined => {
            set_status.set("Opponent joined! Game started.".to_string());
        }
        ServerMessage::OpponentLeft => {
            set_status.set("Opponent left the game".to_string());
        }
        ServerMessage::GameOver { result } => {
            set_game_over.set(true);
            set_status.set(format!("Game Over: {:?}", result));
        }
        ServerMessage::Error { message } => {
            set_status.set(format!("Error: {}", message));
        }
        _ => {}
    }
}

fn format_time(seconds: u64) -> String {
    let mins = seconds / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}", mins, secs)
}
