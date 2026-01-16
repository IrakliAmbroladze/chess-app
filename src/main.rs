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
