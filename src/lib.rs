#[cfg(feature = "hydrate")]
use leptos::*;
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <h1>"Hello world!"</h1>
            <button>Click me</button>
        </div>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> });
}
