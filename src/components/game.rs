use leptos::prelude::*;
use leptos_router::hooks::*;
use leptos_router::params::Params;

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

    let (status, set_status) = signal("Connecting...".to_string());

    view! {
        <div class="game-container">
            <div class="game-info">
                <h2>"Room: " {room_code}</h2>
                <p class="status">{status}</p>
            </div>
        </div>
    }
}
