use crate::util::ReadSignalUtils;
use crate::{
    language::text_macro::text,
    list::{
        item::Item,
        item::{ItemView, ItemViewProps},
        List,
       // SetShoppingListAction,
    },
};
use leptos::*;

#[component]
pub fn ListHeader(cx: Scope) -> impl IntoView {
    let mut list: List = use_context(cx).expect("`List` is available");

    let (new_item, set_new_item) = create_signal(cx, Item::empty(cx));
    let (new_item_name, set_new_item_name) = new_item().name.split();
    let (new_item_amount, set_new_item_amount) = new_item().amount.split();

    let is_preview_empty = move || new_item_name() == "" && new_item_amount() == 1;

    view! {
        cx,
        <header>
            <div class="input-fields">
                <input type="text"
                    class="new-item-name"
                    placeholder=text!(cx, |d| d.list_header.item_name.clone())
                    on:input=move |e| set_new_item_name(event_target_value(&e))
                />
                <input type="number"
                    class="new-item-amount"
                    placeholder=text!(cx, |d| format!("{} ({}: 1)", d.list_header.amount, d.default_))
                    on:input=move |e| set_new_item_amount.set(event_target_value(&e).parse::<usize>().unwrap_or(1))
                />
                <input type="button"
                    class="new-item-button"
                    value="+"
                    on:click=move |_| list.add_new_item(cx, new_item())
                />
            </div>
            <div class="new-item-preview" hidden=move || is_preview_empty()>
                <ItemView item=new_item() />
            </div>
        </header>
    }
}
