use leptos::prelude::*;
use leptos_router::hooks::*;

#[component]
pub fn Home() -> impl IntoView {
    let (room_code, set_room_code) = signal(String::new());
    let navigate = use_navigate();
    let navigate_clone1 = navigate.clone();
    let create_game = move |_| {
        let code = generate_room_code();
        navigate_clone1(&format!("/game/{}?action=create", code), Default::default());
    };
    view! {
        <div>
            <h1>"Chess Game"</h1>
            <button on:click=create_game>"Create New Game"</button>
            <input
                type="text"
                placeholder="Room Code"
                prop:value=room_code
                on:input=move |ev| set_room_code.set(event_target_value(&ev))
            />
        </div>
    }
}

fn generate_room_code() -> String {
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    (0..6)
        .map(|_| {
            let idx = (js_sys::Math::random() * chars.len() as f64) as usize;
            chars[idx]
        })
        .collect()
}
