use crate::{
    barcode_scanner::Barcode,
    item::{
        data::{Item, NewItem, PendingItem},
        server_functions::{get_list, InsertFromClient, InsertFromClientAction, ItemIds},
        ItemView, NewItemView, ShowNewItem,
    },
    util::force_use_context,
};
use leptos::*;
use serde::{Deserialize, Serialize};
use std::vec;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct List(pub(crate) Vec<Item>);

impl IntoView for List {
    fn into_view(self) -> View {
        view! {
            <For
                each=move || self.0.clone()
                key=|item| item.id
                children=|item| view! { <ItemView item/> }
            />
        }
    }
}

impl List {
    pub fn local_remove_id(&mut self, id: i64) {
        let Some(idx) = self.0.iter().position(|i| i.id == id) else { return };
        self.0.remove(idx);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListResource(pub Resource<usize, List>);

impl ListResource {
    fn new(source: impl Fn() -> usize + 'static) -> ListResource {
        ListResource(create_resource(source, |_| async {
            get_list()
                .await
                .inspect_err(|err| logging::error!("ERROR while getting list: {}", err))
                .unwrap_or_default()
        }))
    }
}

#[component]
pub fn ListView() -> impl IntoView {
    let show_new_item = force_use_context::<ShowNewItem>().0;

    let insert_from_client = create_server_multi_action::<InsertFromClient>();
    provide_context(InsertFromClientAction(insert_from_client));

    let items = ListResource::new(move || insert_from_client.version().get());
    provide_context(items);

    let submissions = insert_from_client.submissions();
    let pending_items = move || {
        submissions.with(|vec| {
            vec.iter()
                .flat_map(|a| a.input.get())
                .map(|i| PendingItem::from(i.new_item).into_view())
                .collect_view()
        })
    };

    view! {
        <ul id="shopping_list">
            <NewItemView show=show_new_item />
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                { move || items.0().map(List::into_view) }
            </Transition>
            { pending_items }
        </ul>
    }
}
