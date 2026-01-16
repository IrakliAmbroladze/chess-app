#[cfg(feature = "ssr")]
use axum::extract::{
    ws::{Message, WebSocket},
    State, WebSocketUpgrade,
};
#[cfg(feature = "ssr")]
use axum::{response::IntoResponse, routing::get, Router};
#[cfg(feature = "ssr")]
use futures_util::{SinkExt, StreamExt};
#[cfg(feature = "ssr")]
use std::collections::HashMap;
#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use tokio::sync::RwLock;
#[cfg(feature = "ssr")]
use tower_http::services::ServeDir;

#[cfg(feature = "ssr")]
mod game;
#[cfg(feature = "ssr")]
pub mod shared;

#[cfg(feature = "ssr")]
use crate::game::GameState;
#[cfg(feature = "ssr")]
use crate::shared::*;

#[cfg(feature = "ssr")]
type GameRooms = Arc<RwLock<HashMap<String, GameRoom>>>;
#[cfg(feature = "ssr")]
type GameStates = Arc<RwLock<HashMap<String, GameState>>>;
#[cfg(feature = "ssr")]
type PlayerSessions =
    Arc<RwLock<HashMap<String, tokio::sync::mpsc::UnboundedSender<ServerMessage>>>>;

#[cfg(feature = "ssr")]
#[derive(Clone)]
struct AppState {
    rooms: GameRooms,
    games: GameStates,
    sessions: PlayerSessions,
}
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState {
        rooms: Arc::new(RwLock::new(HashMap::new())),
        games: Arc::new(RwLock::new(HashMap::new())),
        sessions: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .fallback_service(ServeDir::new("dist"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

#[cfg(feature = "ssr")]
async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

#[cfg(feature = "ssr")]
async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let player_id = uuid::Uuid::new_v4().to_string();
    state.sessions.write().await.insert(player_id.clone(), tx);

    // Send task
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap();
            if sender.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    // Receive task
    let sessions = state.sessions.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    tracing::info!("Received: {:?}", client_msg);
                    // Handle message
                }
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    state.sessions.write().await.remove(&player_id);
}

#[cfg(feature = "ssr")]
async fn handle_client_message(msg: ClientMessage, player_id: &str, state: &AppState) {
    match msg {
        ClientMessage::CreateRoom { room_code } => {
            tracing::info!("Creating room {} for player {}", room_code, player_id);

            let rooms = state.rooms.read().await;
            if rooms.contains_key(&room_code) {
                drop(rooms);
                send_to_player(
                    player_id,
                    ServerMessage::Error {
                        message: "Room code already in use".to_string(),
                    },
                    state,
                )
                .await;
                return;
            }
            drop(rooms);

            let room = GameRoom {
                room_code: room_code.clone(),
                white_player: Some(player_id.to_string()),
                black_player: None,
            };

            state.rooms.write().await.insert(room_code.clone(), room);
            state
                .games
                .write()
                .await
                .insert(room_code.clone(), GameState::new(600_000));

            send_to_player(
                player_id,
                ServerMessage::RoomCreated {
                    room_code,
                    player_color: PlayerColor::White,
                },
                state,
            )
            .await;
        }

        ClientMessage::JoinRoom { room_code } => {
            tracing::info!("Player {} attempting to join room {}", player_id, room_code);
            let mut rooms = state.rooms.write().await;

            if let Some(room) = rooms.get_mut(&room_code) {
                if room.black_player.is_none() {
                    room.black_player = Some(player_id.to_string());

                    send_to_player(
                        player_id,
                        ServerMessage::RoomJoined {
                            room_code: room_code.clone(),
                            player_color: PlayerColor::Black,
                        },
                        state,
                    )
                    .await;

                    if let Some(white_id) = &room.white_player {
                        send_to_player(white_id, ServerMessage::OpponentJoined, state).await;
                    }

                    drop(rooms);
                    send_game_state(&room_code, state).await;
                } else {
                    send_to_player(
                        player_id,
                        ServerMessage::Error {
                            message: "Room is full".to_string(),
                        },
                        state,
                    )
                    .await;
                }
            } else {
                send_to_player(
                    player_id,
                    ServerMessage::Error {
                        message: "Room not found".to_string(),
                    },
                    state,
                )
                .await;
            }
        }

        ClientMessage::MakeMove {
            from,
            to,
            promotion,
        } => {
            if let Some((room_code, _)) = find_player_room(player_id, state).await {
                let mut games = state.games.write().await;

                if let Some(game) = games.get_mut(&room_code) {
                    match game.make_move(&from, &to, promotion.as_deref()) {
                        Ok(san) => {
                            let fen = game.get_fen();
                            drop(games);

                            broadcast_to_room(
                                &room_code,
                                ServerMessage::MoveMade { from, to, san, fen },
                                state,
                            )
                            .await;

                            send_game_state(&room_code, state).await;
                        }
                        Err(reason) => {
                            send_to_player(player_id, ServerMessage::InvalidMove { reason }, state)
                                .await;
                        }
                    }
                }
            }
        }

        ClientMessage::Resign => {
            if let Some((room_code, color)) = find_player_room(player_id, state).await {
                let winner = match color {
                    PlayerColor::White => PlayerColor::Black,
                    PlayerColor::Black => PlayerColor::White,
                };

                broadcast_to_room(
                    &room_code,
                    ServerMessage::GameOver {
                        result: GameResult::Resignation { winner },
                    },
                    state,
                )
                .await;
            }
        }
    }
}

#[cfg(feature = "ssr")]
async fn send_game_state(room_code: &str, state: &AppState) {
    let games = state.games.read().await;
    if let Some(game) = games.get(room_code) {
        let msg = ServerMessage::GameState {
            fen: game.get_fen(),
            moves: game.moves.clone(),
            white_time: game.white_time_ms,
            black_time: game.black_time_ms,
            current_turn: game.current_turn(),
        };
        drop(games);
        broadcast_to_room(room_code, msg, state).await;
    }
}

#[cfg(feature = "ssr")]
async fn send_to_player(player_id: &str, msg: ServerMessage, state: &AppState) {
    let sessions = state.sessions.read().await;
    if let Some(tx) = sessions.get(player_id) {
        let _ = tx.send(msg);
    }
}

#[cfg(feature = "ssr")]
async fn broadcast_to_room(room_code: &str, msg: ServerMessage, state: &AppState) {
    let rooms = state.rooms.read().await;
    if let Some(room) = rooms.get(room_code) {
        if let Some(white) = &room.white_player {
            send_to_player(white, msg.clone(), state).await;
        }
        if let Some(black) = &room.black_player {
            send_to_player(black, msg, state).await;
        }
    }
}

#[cfg(feature = "ssr")]
async fn find_player_room(player_id: &str, state: &AppState) -> Option<(String, PlayerColor)> {
    let rooms = state.rooms.read().await;
    for (code, room) in rooms.iter() {
        if room.white_player.as_deref() == Some(player_id) {
            return Some((code.clone(), PlayerColor::White));
        }
        if room.black_player.as_deref() == Some(player_id) {
            return Some((code.clone(), PlayerColor::Black));
        }
    }
    None
}

#[cfg(feature = "ssr")]
async fn cleanup_player(player_id: &str, state: &AppState) {
    if let Some((room_code, _)) = find_player_room(player_id, state).await {
        broadcast_to_room(&room_code, ServerMessage::OpponentLeft, state).await;
    }
}
