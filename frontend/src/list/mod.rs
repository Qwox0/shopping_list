use leptos::*;

mod item;
mod header;

use item::*;
use header::*;

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

    view! {cx,
        <ListHeader/>
        <ul>
            <For
                each=move || list.get().0.clone()
                key=|item| item.id
                view=move |item| view! { cx,  <Item item/> }
            />
        </ul>
    }
}
