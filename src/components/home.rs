use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn Home() -> impl IntoView {
    let navigate = use_navigate();
    let navigate_clone1 = navigate.clone();
    let create_game = move |_| {
        let code = 12;
        navigate_clone1(&format!("/game/{}?action=create", code), Default::default());
    };
    view! {
        <div>
            <h1>"Chess Game"</h1>
            <button on:click=create_game>"Create New Game"</button>
        </div>
    }
}
