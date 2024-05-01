use super::data::Item;
use crate::{
    barcode_scanner::Barcode,
    item::data::{ItemImpl, NewItem},
    list::List,
};
use leptos::{logging, server, Action, ServerFnError};
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
    logging::log!("{:?}", new_item);
    let item = new_item.insert(&crate::db::MY_DB).await?;
    Ok(ItemIds { item_id: item.id, variant_ids: item.variants.into_iter().map(|a| a.id).collect() })
}

#[derive(Clone, Copy)]
pub struct InsertFromClientAction(pub Action<InsertFromClient, Result<ItemIds, ServerFnError>>);
