#![allow(unused)]
#![allow(incomplete_features)]
#![feature(result_flattening)]
#![feature(async_fn_in_trait)]

pub mod state;
pub mod view;
pub mod util;

/*
pub mod app;
pub mod connection_status;
#[cfg(feature = "ssr")]
pub mod db;
pub mod head;
pub mod language;
pub mod list;
pub mod state;
*/

#[cfg(feature = "hydrate")]
//#[cfg_attr(feature = "hydrate", wasm_bindgen::prelude::wasm_bindgen)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use view::app::*;
    use leptos::*;
    use state::render_state::AppState;
    console_error_panic_hook::set_once();

    mount_to_body(move |cx| {
        let render_state: &'static AppState = Box::leak(Box::new(AppState::new(cx)));
        provide_context::<&'static AppState>(cx, render_state);

        view! { cx, <App/> }
    });
}
