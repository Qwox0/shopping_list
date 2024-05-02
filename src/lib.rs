#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(type_changing_struct_update)]
#![allow(unused)]

pub mod app;
#[cfg(feature = "ssr")]
mod auth;
mod barcode_scanner;
mod camera;
#[cfg(feature = "ssr")]
mod db;
mod db_tool;
mod default_resource;
mod error;
#[cfg(feature = "ssr")]
pub mod fileserv;
mod header_bar;
mod image;
mod item;
mod language;
mod list;
mod login;
mod main_page;
mod option_signal;
mod popup;
mod server_sync_signal;
mod subsignal;
mod util;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
