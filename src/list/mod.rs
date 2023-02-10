mod header;
mod item;

use header::*;
use item::*;
use leptos::*;

#[derive(Clone)]
struct ItemList {
    pub items: Vec<Item>,
}

impl ItemList {
    pub fn new() -> Self {
        ItemList { items: Vec::new() }
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item)

    }
}

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
