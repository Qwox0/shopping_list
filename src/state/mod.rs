//pub mod cookies;
//pub mod db;
//pub mod local_storage;
pub mod app_state;
#[cfg(feature = "ssr")]
pub mod db;
pub mod item;
pub mod item_serialized;
pub mod language;
pub mod list;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    use leptos::ServerFn;
    self::list::GetItemList::register().expect("could register server fn");
}
