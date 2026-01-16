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
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|p| p.room_code.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    };

    view! {
        <div class="game">
            <h2>"Room: " {room_code}</h2>
            <p>"Game will go here"</p>
        </div>
    }
}
