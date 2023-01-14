use leptos::*;
use uuid::Uuid;

use crate::list::EntriesList;

#[derive(Debug, Clone)]
pub struct Item {
    pub id: Uuid,
    pub name: RwSignal<String>,
}

impl Item {
    pub fn new(cx: Scope, name: impl Into<String>) -> Self {
        Item {
            id: Uuid::new_v4(),
            name: create_rw_signal(cx, name.into()),
        }
    }
}

#[component]
pub fn Item(cx: Scope, item: Item) -> impl IntoView {
    let set_list = use_context::<WriteSignal<EntriesList>>(cx).unwrap();

    view! {cx,
        <li>
            {move || item.name.get()}
        </li>
    }
}
