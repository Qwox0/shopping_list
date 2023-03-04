#![allow(unused)]
#![allow(incomplete_features)]
#![feature(result_flattening)]
#![feature(async_fn_in_trait)]

pub mod state;
pub mod util;
pub mod view;

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
    console_error_panic_hook::set_once();
    leptos::mount_to_body(crate::view::app::app);
}
