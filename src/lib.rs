#[cfg(feature = "hydrate")]
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div class="app">
            <h1>"Hello Leptos!"</h1>
            <button on:click=move |_| set_count.update(|n| *n += 1)>
                "Count: " {count}
            </button>
        </div>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
