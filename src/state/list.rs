use crate::state::{item::Item, item_serialized::ItemSerialized};
use leptos::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListMsg {
    GetAll,
    Add(ItemSerialized),
    Remove(Uuid),

    Get(usize), // for debugging
}

#[server(GetItemList, "/api", "Cbor")] // Cbor: + smaller + allows for enums with non String values - needs wasm even in forms
pub async fn get_item_list(cx: Scope, msg: ListMsg) -> Result<Vec<ItemSerialized>, ServerFnError> {
    use crate::state::db::InDb;
    log!("got msg: {:?}", msg);
    let mut connection = ItemList::get_db_connection()
        .await
        .expect("got db connection");

    // change value
    let list = match msg {
        ListMsg::Add(item) => connection
            .add(item)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string()))?
            .get_all(cx)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string()))?,
        ListMsg::Remove(id) => connection
            .remove(id)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string()))?
            .get_all(cx)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string()))?,
        ListMsg::GetAll => connection
            .get_all(cx)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string()))?,

        ListMsg::Get(count) => connection
            .get_all(cx)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string()))?
            .into_iter()
            .take(count)
            .collect(),
    };

    log!("request list: {:?}", list);
    //std::thread::sleep(std::time::Duration::from_millis(500));
    Ok(list)
}

#[derive(Debug, Clone, Copy)]
pub struct ItemList {
    list: Resource<ListMsg, Result<Vec<ItemSerialized>, ServerFnError>>,
    message: RwSignal<ListMsg>,
}

impl ItemList {
    pub fn new(cx: Scope) -> Self {
        let message = create_rw_signal(cx, ListMsg::GetAll);
        let list = create_resource(cx, message, move |msg| get_item_list(cx, msg));
        ItemList { list, message }
    }

    pub fn read(&self, cx: Scope) -> Option<Result<Vec<ItemSerialized>, ServerFnError>> {
        self.list.read(cx)
    }

    pub fn with<U>(
        &self,
        cx: Scope,
        f: impl FnOnce(&Result<Vec<ItemSerialized>, ServerFnError>) -> U,
    ) -> Option<U> {
        self.list.with(cx, f)
    }

    pub fn with2<U>(
        &self,
        cx: Scope,
        f: impl FnOnce(&Vec<ItemSerialized>) -> U,
    ) -> Option<Result<U, ServerFnError>> {
        self.list.with(cx, move |a| match a {
            Ok(items) => Ok(f(items)),
            Err(e) => Err(e.to_owned()),
        })
    }

    pub fn read2(&self, cx: Scope) -> Option<Result<Vec<Item>, ServerFnError>> {
        let deserialize_list = |list: &Vec<ItemSerialized>| {
            list.iter()
                .map(|i| Item::from_serialized_ref(cx, i))
                .collect()
        };
        self.list.with(cx, |serialized| {
            serialized
                .as_ref()
                .map(deserialize_list)
                .map_err(ToOwned::to_owned)
        })
    }

    pub fn send_msg(&self, msg: ListMsg) {
        self.message.set(msg)
    }

    pub fn add(&self, item: ItemSerialized) {
        self.message.set(ListMsg::Add(item))
    }

    pub fn remove(&self, id: Uuid) {
        self.message.set(ListMsg::Remove(id))
    }
}

#[cfg(feature = "ssr")]
impl crate::state::db::InDb for ItemList {
    const DB_URL: &'static str = "sqlite:./data/ShoppingList.db";
    const TABLE_NAME: &'static str = "items";
    const HEADERS_TUPLE: &'static str = "(id, name, amount, state)";
    type RowType = ItemSerialized;

    fn bind_values<'a>(
        query: sqlx::query::Query<'a, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'a>>,
        row: Self::RowType,
    ) -> sqlx::query::Query<'a, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'a>> {
        query
            .bind(row.id.clone())
            .bind(row.name.clone())
            .bind(row.amount.clone())
            .bind(row.state.clone())
    }
}
