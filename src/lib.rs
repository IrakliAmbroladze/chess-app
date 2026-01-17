#[cfg(feature = "hydrate")]
use leptos::prelude::*;
#[cfg(feature = "hydrate")]
use leptos_router::components::*;
#[cfg(feature = "hydrate")]
use leptos_router::{ParamSegment, StaticSegment};
#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
mod components;
#[cfg(feature = "hydrate")]
pub mod shared;

#[cfg(feature = "hydrate")]
use components::{Game, Home};

#[cfg(feature = "hydrate")]
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| "Not found">
                    <Route path=StaticSegment("") view=Home />
                    <Route path=(StaticSegment("game"), ParamSegment("room_code")) view=Game />
                </Routes>
            </main>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
