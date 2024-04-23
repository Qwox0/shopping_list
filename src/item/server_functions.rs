use super::NewItem;
use leptos::{logging, server, ServerFnError};
use serde::{Deserialize, Serialize};

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
    Ok(ItemIds { item_id: item.id, variant_ids: item.variants.into_iter().map(|a| a.id).collect() })
}
