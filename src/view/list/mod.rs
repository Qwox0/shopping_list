mod header;
mod item;

use crate::state::list::ItemList;
use header::*;
use item::*;
use leptos::*;



#[component]
pub fn ShoppingList(cx: Scope) -> impl IntoView {
    //let list = create_resource(cx, move || 0, |_| get_items());
    //provide_context(cx, set_list);
    //create_resource_with_initial_value(cx, || (), move ||, initial_value)
    let (list, set_list) = create_signal(cx, ItemList::new());
    provide_context(cx, set_list);

    let items = move || {
        view! { cx,
            <For each=move || list.with(|l| l.items.clone())
                key=|item| item.id
                view=move |item| view! { cx,
                    <Item item/>
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

            <input type="button"
                value="Debug"
                on:click=move |_| {
                    log!("{:?}", list().items.iter().map(|i| format!("{}", i)).collect::<Vec<_>>());
                }
            />
        </section>
    }
}
