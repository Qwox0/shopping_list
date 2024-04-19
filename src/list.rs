use crate::{
    barcode_scanner::Barcode,
    item::{
        data::{Item, NewItem},
        ItemView, NewItemView, ShowNewItem,
    },
    util::force_use_context,
};
use leptos::*;
use serde::{Deserialize, Serialize};
use std::vec;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct List(Vec<Item>);

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

#[server]
pub async fn get_list() -> Result<List, ServerFnError> {
    Ok(Item::select_all(&crate::db::MY_DB)
        .await
        .inspect_err(|err| eprintln!("ERROR (get_list): {}", err))
        .map(List)?)
}

/// Returns id of the created item.
#[server]
pub async fn add_item_from_barcode(barcode: Barcode) -> Result<(), ServerFnError> {
    let i = NewItem::from_barcode(barcode).await?;
    i.insert(&crate::db::MY_DB).await?;
    Ok(())
}

#[component]
pub fn ListView() -> impl IntoView {
    let show_new_item = force_use_context::<ShowNewItem>();

    let add_item_from_barcode = create_server_multi_action::<AddItemFromBarcode>();

    let list = create_resource(move || (add_item_from_barcode.version().get()), |_| get_list());

    let items_view = move || Some(list()?.map(List::into_view));

    view! {
        <ul id="shopping_list">
            <NewItemView hidden=move || !show_new_item.0.get() />
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                { move || items_view() }
            </Transition>
        </ul>
    }
}
