use super::{ItemListSerialized, SetShoppingListAction};
use crate::{
    language::text_macro::text,
    list::{
        header::{ListHeader, ListHeaderProps},
        item::item_view::{ItemView, ItemViewProps},
        ItemList, SetShoppingList,
    },
};
use leptos::*;

#[component]
pub fn ListView(cx: Scope) -> impl IntoView {
    //let list = create_resource(cx, move || 0, |_| get_items());
    //provide_context(cx, set_list);
    //create_resource_with_initial_value(cx, || (), move ||, initial_value)
    let (list, set_list) = create_signal(cx, ItemList::empty());
    provide_context(cx, set_list);

    //let initial = ItemList::from(vec![]); //initial_prefers_dark(cx);

    /*
    let set_list_action = SetShoppingListAction::create(cx);
    let (input, value) = set_list_action.get_signals();
    let list_serialized = move || {
        match (input(), value()) {
            (Some(submission), _) => submission.new_list, // if there's some current input, use that optimistically
            (_, Some(Ok(value))) => value, // otherwise, if there was a previous value confirmed by server, use that
            _ => ItemListSerialized(vec![]), // otherwise, use the initial value
        }
    };
    let list = move || ItemList::from_serialized(cx, list_serialized());

    provide_context(cx, set_list_action);
    */

    let items = move || {
        view! { cx,
            <For each=move || list().into_iter().rev() //move || list.with(|l| l.items.clone())
                key=|item| item.id
                view=move |item| view! { cx,
                    <li> <ItemView item/> </li>
                }
            />
        }
    };

    view! {cx,
        <section class="shopping-list">
            <ListHeader/>
            <ul class="items">
                {items}
            </ul>

            /*
            <input type="button"
                value="Debug"
                on:click=move |_| {
                    log!("{:?}", list().iter().map(|i| format!("{:?}", i)).collect::<Vec<_>>());
                }
            />
            */
        </section>
    }
}
