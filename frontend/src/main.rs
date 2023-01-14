use frontend::*;
use leptos::*;

use crate::connection_status::*;
use crate::list::*;

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx,
        <ConnectionStatus/>
        <h1> "Shopping List" </h1>
        <ShoppingList/>
    })
}
