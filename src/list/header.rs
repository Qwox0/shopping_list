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
    /*
    let set_list = |list: &ItemList| {
        use_context::<SetShoppingListAction>(cx)
            .expect("`SetShoppingListAction` is available")
            .set_list(list)
    };
    */
    let set_list =
        use_context::<WriteSignal<ItemList>>(cx).expect("`WriteSignal<ItemList>` is available");

    let (new_item, set_new_item) = create_signal(cx, Item::new(cx, "", 1));
    let s = move || new_item().name;// create_rw_signal(cx, String::new());
    let is_preview_empty = move || new_item().name.get();
    //move || new_item.with(|i| i.name.contains(&"".to_string()) && i.amount.contains(&1));
    //create_effect(cx, move |_| log!("{}", is_preview_empty()));
    let is_preview_empty2 = move || s().get();
    create_effect(cx, move |_| log!("{}", is_preview_empty2()));
    create_effect(cx, move |_| log!("{}", new_item()));

    view! {
        cx,
        <header>
            <div class="input-fields">
                <input
                    type="text"
                    placeholder=text!(cx, |d| d.list_header.item_name.clone())
                    class="new-item-name"
                    //on:focusout=move |e| set_new_item_name(event_target_value(&e))
                    on:input=move |e| s().set(event_target_value(&e))
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
