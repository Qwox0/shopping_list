use crate::{language::text_macro::text, state::{item_list::ItemList, item::Item}};
use leptos::*;

#[component]
pub fn ListHeader(cx: Scope) -> impl IntoView {
    let set_list = use_context::<WriteSignal<ItemList>>(cx).unwrap();

    let (new_item, set_new_item) = create_signal(cx, Item::new(cx, "", 1));

    view! {
        cx,
        <header>
            <input
                type="text"
                placeholder=text!(cx, |d| d.list_header.item_name.clone())
                class="new-item-name"
                //on:focusout=move |e| set_new_item_name(event_target_value(&e))
                on:focusout=move |e| set_new_item.update(|i| i.name.set(event_target_value(&e)))
            />
            <input
                type="number"
                placeholder=text!(cx, |d| format!("{} ({}: 1)", d.list_header.amount, d.default_))
                class="new-item-amount"
                on:focusout=move |e| set_new_item.update(|i| i.amount.set(event_target_value(&e).parse().unwrap_or(1)))
            />
            <input value="+"
                type="button"
                class="new-item-button"
                on:click=move |_| set_list.update(|list| list.add(new_item()))
            />
        </header>
    }
}
