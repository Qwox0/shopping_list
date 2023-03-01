#![allow(unused)]
#![allow(incomplete_features)]
#![feature(result_flattening)]
#![feature(async_fn_in_trait)]

pub mod app;
pub mod connection_status;
#[cfg(feature = "ssr")]
pub mod db;
pub mod head;
pub mod language;
pub mod list;
pub mod state;
pub mod util;

#[cfg(feature = "hydrate")]
//#[cfg_attr(feature = "hydrate", wasm_bindgen::prelude::wasm_bindgen)]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;
    use state::render_state::RenderState;
    console_error_panic_hook::set_once();

    mount_to_body(move |cx| {
        let render_state: &'static RenderState = Box::leak(Box::new(RenderState::new(cx)));
        provide_context::<&'static RenderState>(cx, render_state);

        view! { cx, <App/> }
    });
}
