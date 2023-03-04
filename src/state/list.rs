use crate::state::{item::Item, item_serialized::ItemSerialized};
use leptos::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListMsg {
    Get(usize), // serde_urlencoded only supports Strings
    GetAll,
}

#[server(GetItemList, "/api", "Cbor")] // Cbor: + smaller + allows for enums with non String values - needs wasm even in forms
pub async fn get_item_list(cx: Scope, msg: ListMsg) -> Result<Vec<ItemSerialized>, ServerFnError> {
    use crate::state::db::InDb;
    log!("msg: {:?}", msg);
    /*
    let list = crate::state::db::ItemsDbTable::new()
        .await
        .expect("got db")
        */
    //let list = crate::state::db::DbConnection::<ItemSerialized>::new();
    let list = ItemList::get_db_connection()
        .await
        .expect("got db connection")
        .get_all(cx)
        .await
        .expect("got items");
    let list = match msg {
        ListMsg::Get(count) => list.into_iter().take(count).collect(),
        ListMsg::GetAll => list,
    };
    log!("request list: {:?}", list);
    std::thread::sleep(std::time::Duration::from_millis(1000));
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

    pub fn add(&self, item: Item) {
        todo!();
    }

    pub fn add_new_item(&self, cx: Scope, new_item: Item) {
        todo!();
    }

    pub fn remove(&self, id: Uuid) {
        todo!();
    }
}

#[cfg(feature = "ssr")]
impl crate::state::db::InDb for ItemList {
    const DB_URL: &'static str = "sqlite:./data/ShoppingList.db";
    const TABLE_NAME: &'static str = "items";
    const COLUMNS_TUPLE: &'static str = "()";
    type RowType = ItemSerialized;

    fn bind_values<'a>(
        query_builder: &'a mut sqlx::QueryBuilder<'a, sqlx::Sqlite>,
        row: Self::RowType,
    ) -> &mut sqlx::QueryBuilder<'a, sqlx::Sqlite> {
        query_builder
            .push_bind(row.id.clone())
            .push_bind(row.name.clone())
            .push_bind(row.amount.clone())
            .push_bind(row.state.clone())
    }
}
