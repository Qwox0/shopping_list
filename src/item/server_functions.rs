use super::{data::Item, variant_data::NewVariant};
use crate::{
    barcode_scanner::Barcode,
    item::data::{ItemImpl, NewItem},
    list::List,
};
use leptos::{logging, server, Action, MultiAction, ServerFnError};
use serde::{Deserialize, Serialize};

#[server]
pub async fn get_list() -> Result<List, ServerFnError> {
    Ok(Item::select_all(&crate::db::MY_DB)
        .await
        .inspect_err(|err| eprintln!("ERROR (get_list): {}", err))
        .map(List)?)
}

/// Returns id of the created item.
#[server]
pub async fn add_item_from_barcode(barcode: Barcode) -> Result<i64, ServerFnError> {
    let i = NewItem::from_barcode(barcode).await?;
    let i = i.insert(&crate::db::MY_DB).await?;
    Ok(i.id)
}

#[server]
pub async fn remove_item(id: i64) -> Result<bool, ServerFnError> {
    Item::remove(id, &crate::db::MY_DB).await.map_err(Into::into)
}

#[server]
pub async fn set_completed(item_id: i64, completed: bool) -> Result<(), ServerFnError> {
    logging::log!("set completed for {item_id}: {completed}");
    let mut conn = crate::db::MY_DB.connection().await?;
    sqlx::query!("UPDATE item SET completed = ? WHERE id = ?", completed, item_id)
        .execute(conn.as_mut())
        .await?;
    Ok(())
}

#[server]
pub async fn set_amount(item_id: i64, amount: u64) -> Result<(), ServerFnError> {
    logging::log!("set amount for {item_id}: {amount}");
    let amount = amount as i64;
    let mut conn = crate::db::MY_DB.connection().await?;
    sqlx::query!("UPDATE item SET amount = ? WHERE id = ?", amount, item_id)
        .execute(conn.as_mut())
        .await?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemIds {
    pub item_id: i64,
    pub variant_ids: Box<[i64]>,
}

#[server]
pub async fn insert_from_client(new_item: NewItem) -> Result<ItemIds, ServerFnError> {
    let item = new_item.insert(&crate::db::MY_DB).await?;
    // std::thread::sleep(std::time::Duration::from_millis(10000));
    Ok(ItemIds { item_id: item.id, variant_ids: item.variants.into_iter().map(|a| a.id).collect() })
}

#[derive(Clone, Copy)]
pub struct InsertFromClientAction(
    pub MultiAction<InsertFromClient, Result<ItemIds, ServerFnError>>,
);

/// returns the variant id.
#[server]
pub async fn insert_variant_from_client(
    item_id: i64,
    new_variant: NewVariant,
) -> Result<i64, ServerFnError> {
    let mut conn = crate::db::MY_DB.connection().await?;
    let variant = new_variant.insert(item_id, conn.as_mut()).await?;
    Ok(variant.id)
}
