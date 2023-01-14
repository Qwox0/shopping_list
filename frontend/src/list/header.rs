use leptos::*;

use crate::list::{item::Item, EntriesList};

#[component]
pub fn ListHeader(cx: Scope) -> impl IntoView {
    let set_list = use_context::<WriteSignal<EntriesList>>(cx).unwrap();

    view! {
        cx,
        <header>
            <input class="new-item-name"
                on:focusout=move |e| set_list.update(|list| list.add(Item::new(cx, event_target_value(&e))))
            />
        </header>
    }
}

fn handle_keydown<T>(event: web_sys::KeyboardEvent, ws: WriteSignal<T>) {
    let target = event_target::<web_sys::HtmlInputElement>(&event);
    event.stop_propagation();
    let key_code = event.key_code();
    let key = event.key();
    let title = event_target_value(&event);
    log!("key_code: {:?}, ", key_code);
    log!("key: {:?}, ", key);
    log!("target_value: {:?}, ", title);
}
