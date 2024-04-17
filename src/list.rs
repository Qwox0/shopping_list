use crate::{
    barcode_scanner::Barcode,
    item::{Item, ItemData, ItemView, NewItemView, ShowNewItem},
    util::force_use_context,
};
use leptos::*;
use serde::{Deserialize, Serialize};
use std::vec;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct List {
    list: Vec<Item>,
}

impl IntoView for List {
    fn into_view(self) -> View {
        view! {
            <For
                each=move || self.list.clone()
                key=|item| item.id
                children=|item| view! { <ItemView item/> }
            />
        }
    }
}

#[server]
pub async fn get_list() -> Result<List, ServerFnError> {
    pub async fn get_list() -> Result<List, ServerFnError> {
        // let mut conn = crate::db::DB::connection_from_context().await?;
        let mut conn = crate::db::db().await?;

        //let list = sqlx::query_as!(Item, "SELECT * FROM items").fetch_all(&mut
        // conn).await?;
        let list = sqlx::query_as("SELECT * FROM items").fetch_all(&mut conn).await?;

        Ok(List { list })
    }
    get_list().await.inspect_err(|err| eprintln!("ERROR (get_list): {}", err))
}

/// Returns id of the created item.
#[server]
pub async fn add_item_from_barcode(barcode: Barcode) -> Result<(), ServerFnError> {
    pub async fn add_item_from_barcode(barcode: Barcode) -> Result<(), ServerFnError> {
        let ItemData { name, amount, barcode, img_url, thumb_url, .. } =
            ItemData::from_barcode(barcode).await?;

        //let mut conn = crate::db::DB::connection_from_context().await?;
        let mut conn = crate::db::db().await?;

        let _new_id = sqlx::query!(
            r#"
INSERT INTO items(name, amount, barcode, img_url, thumb_url)
VALUES ( ?, ?, ?, ?, ? )"#,
            name,
            amount,
            barcode,
            img_url,
            thumb_url
        )
        .execute(&mut conn)
        .await?
        .last_insert_rowid();

        Ok(())
    }
    add_item_from_barcode(barcode)
        .await
        .inspect_err(|err| eprintln!("ERROR (add_item_from_barcode): {}", err))
}

#[component]
pub fn ListView() -> impl IntoView {
    let show_new_item = force_use_context::<ShowNewItem>();

    let add_item_from_barcode = create_server_multi_action::<AddItemFromBarcode>();

    async fn barcode_to_item(barcode: Barcode) -> ItemData {
        ItemData::from_barcode(barcode).await.unwrap()
    }

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
