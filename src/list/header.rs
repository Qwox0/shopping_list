use crate::util::ReadSignalUtils;
use crate::{
    language::text_macro::text,
    list::{
        item::Item,
        item_view::{ItemView, ItemViewProps},
        ItemList, SetShoppingListAction,
    },
};
use leptos::*;

#[component]
pub fn ListHeader(cx: Scope) -> impl IntoView {
    let set_list =
        use_context::<WriteSignal<ItemList>>(cx).expect("`WriteSignal<ItemList>` is available");

    let (new_item, set_new_item) = create_signal(cx, Item::empty(cx));
    let (new_item_name, set_new_item_name) = new_item().name.split();
    let (new_item_amount, set_new_item_amount) = new_item().amount.split();

    let is_preview_empty = move || new_item_name() == "" && new_item_amount() == 1;
    create_effect(cx, move |_| log!("{}", is_preview_empty()));

    view! {
        cx,
        <header>
            <div class="input-fields">
                <input
                    type="text"
                    placeholder=text!(cx, |d| d.list_header.item_name.clone())
                    class="new-item-name"
                    //on:focusout=move |e| set_new_item_name(event_target_value(&e))
                    on:input=move |e| set_new_item_name(event_target_value(&e))
                    //on:input=move |e| set_new_item.update(|i| i.name.set(event_target_value(&e)))
                />
                <input
                    type="number"
                    placeholder=text!(cx, |d| format!("{} ({}: 1)", d.list_header.amount, d.default_))
                    class="new-item-amount"
                    //on:input=move |e| set_new_item.update(|i| i.amount.set(event_target_value(&e).parse().unwrap_or(1)))
                />
                <input value="+"
                    type="button"
                    class="new-item-button"
                    //on:click=move |_| set_list.update(|list| list.add(cx, new_item()))
                />
            </div>
            <div class="new-item-preview" hidden=is_preview_empty()>
                <ItemView item=new_item() />
            </div>
        </header>
    }
}
