#![allow(unused)]

#![allow(incomplete_features)]
#![feature(result_flattening)]
#![feature(async_fn_in_trait)]

pub mod app;
mod connection_status;
#[cfg(feature = "ssr")]
pub mod db;
mod head;
mod language;
pub mod list;
pub mod state;
mod util;

cfg_if::cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use app::*;
        use leptos::*;
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            console_error_panic_hook::set_once();
            mount_to_body(move |cx| {

                view! { cx, <App/> }
            });
        }
    }
}
