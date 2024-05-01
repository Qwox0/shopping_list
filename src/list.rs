use crate::{
    barcode_scanner::Barcode,
    item::{
        data::{Item, NewItem},
        server_functions::{get_list, InsertFromClient, ItemIds},
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
    fn new_resource(source: impl Fn() -> () + 'static) -> ListResource {
        ListResource(create_resource(source, |_| async {
            get_list()
                .await
                .inspect_err(|err| logging::error!("ERROR while getting list: {}", err))
                .unwrap_or_default()
        }))
    }

    pub fn local_remove_id(&mut self, id: i64) {
        let Some(idx) = self.0.iter().position(|i| i.id == id) else { return };
        self.0.remove(idx);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListResource(pub Resource<(), List>);

#[derive(Clone, Copy)]
pub struct InsertFromClientAction(pub Action<InsertFromClient, Result<ItemIds, ServerFnError>>);

#[component]
pub fn ListView() -> impl IntoView {
    let show_new_item = force_use_context::<ShowNewItem>();

    let insert_from_client = InsertFromClientAction(create_server_action::<InsertFromClient>());
    provide_context(insert_from_client);

    let list = List::new_resource(move || insert_from_client.0.version().track());
    provide_context(list);

    let items_view = move || list.0().map(List::into_view);

    view! {
        <ul id="shopping_list">
            <NewItemView hidden=move || !show_new_item.0.get() />
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                { move || items_view() }
            </Transition>
        </ul>
    }
}
