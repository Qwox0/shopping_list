mod language;
mod state;
mod util;
pub mod view;

cfg_if::cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use view::app::*;
        use leptos::*;
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            console_error_panic_hook::set_once();
            mount_to_body(move |cx| view! { cx, <App/> });
        }
    }
}
