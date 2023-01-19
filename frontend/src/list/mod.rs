use leptos::*;

mod header;
mod item;

use header::*;
use item::*;

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
    let (list, set_list) = create_signal(cx, EntriesList::new());

    provide_context(cx, set_list);

    let items = move || {
        view! { cx,
            <For each=move || list.get().0.clone()
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
