use common::item::Item;
use leptos::*;

mod header;
mod item;

use header::*;
use item::*;

use crate::state::ItemList;

#[derive(Debug, Clone)]
struct EntriesList(Vec<Item>);

impl EntriesList {
    pub fn new() -> Self {
        EntriesList(Vec::new())
    }

    pub fn add(&mut self, item: Item) {
        self.0.push(item)
    }
}

#[component]
pub fn ShoppingList(cx: Scope) -> impl IntoView {
    //let list = create_resource(cx, move || 0, |_| get_items());
    //provide_context(cx, set_list);
    let (list, set_list) = create_signal(cx, ItemList::new());

    let items = move || {
        view! { cx,
            <For each=move || list.with(|l| l.items)
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
            <ul>
                {items}
            </ul>

            <input type="button"
                value="Debug"
                on:click=move |_| {
                    log!("{:?}", list().0.iter().map(|i| format!("{}", i)).collect::<Vec<_>>());
                }
            />
        </section>
    }
}

/*
*/
