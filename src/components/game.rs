use leptos::prelude::*;
use leptos_router::hooks::*;

#[component]
pub fn Game() -> impl IntoView {
    let params = use_params_map();
    let room_code = move || params.read().get("room_code").unwrap_or_default();

    view! {
        <div class="game">
            <h2>"Room: " {room_code}</h2>
            <p>"Game will go here"</p>
        </div>
    }
}
