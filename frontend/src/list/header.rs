use leptos::*;

use crate::{
    language::text_macro::text,
    list::{item::Item, EntriesList},
};

#[component]
pub fn ListHeader(cx: Scope) -> impl IntoView {
    let (new_item_name, set_new_item_name) = create_signal(cx, "".to_string());
    let (new_item_amount, set_new_item_amount) = create_signal(cx, 0);

    let set_list = use_context::<WriteSignal<EntriesList>>(cx).unwrap();
    view! {
        cx,
        <header>
            <input
                type="text"
                placeholder={text!(cx, |d| &d.list_header.item_name)}
                class="new-item-name"
                on:focusout=move |e| set_new_item_name(event_target_value(&e))
            />
            <input
                type="number"
                placeholder={text!(cx, |d| &d.list_header.amount)}
                class="new-item-amount"
                on:focusout=move |e| set_new_item_amount(event_target_value(&e).parse().unwrap_or(1))
            />
            <input value="+"
                type="button"
                class="new-item-button"
                on:click=move |_| set_list.update(|list| list.add(Item::new(cx, new_item_name(), new_item_amount())))
            />
        </header>
    }
}
