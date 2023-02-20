mod header;
mod item;
pub mod list_view;

use header::*;
use item::*;
use leptos::*;
use list_view::{ListView, ListViewProps};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ItemList(Vec<Item>);

impl ItemList {
    pub fn empty() -> Self {
        ItemList(vec![])
    }

    /*
    pub fn from_serialized(cx: Scope, serialized: ItemListSerialized) -> Self {
        ItemList(
            serialized
                .0
                .into_iter()
                .map(|i| NewItem::from_serialized(cx, i))
                .collect(),
        )
    }
    */

    pub fn add(&mut self, cx: Scope, new_item: Item) {
        self.0.push(Item::from_new_item(cx, new_item))
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Item> {
        self.0.iter()
    }
}

impl IntoIterator for ItemList {
    type Item = Item;
    type IntoIter = std::vec::IntoIter<Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// ------ Serialized -------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemListSerialized(Vec<ItemSerialized>);

impl From<&ItemList> for ItemListSerialized {
    fn from(value: &ItemList) -> Self {
        ItemListSerialized(value.iter().map(|item| item.into()).collect())
    }
}

#[derive(Clone)]
struct SetShoppingListAction(
    pub Action<SetShoppingList, Result<ItemListSerialized, ServerFnError>>,
);

impl SetShoppingListAction {
    pub fn create(cx: Scope) -> Self {
        Self(create_server_action::<SetShoppingList>(cx))
    }

    pub fn get_signals(
        &self,
    ) -> (
        RwSignal<Option<SetShoppingList>>,
        RwSignal<Option<Result<ItemListSerialized, ServerFnError>>>,
    ) {
        let input = self.0.input(); // input is `Some(value)` when pending, and `None` if not pending
        let value = self.0.value(); // value contains most recently-returned value
        (input, value)
    }

    pub fn get_list(&self) -> ItemList {
        todo!()
        /*
        let (input, value) = self.get_signals();
        let list_serialized = move || {
            match (input(), value()) {
                (Some(submission), _) => submission.new_list, // if there's some current input, use that optimistically
                (_, Some(Ok(value))) => value, // otherwise, if there was a previous value confirmed by server, use that
                _ => ItemListSerialized(vec![]), // otherwise, use the initial value
            }
        };
        ItemList::from_serialized(cx, list_serialized())
        */
    }

    pub fn set_list(&self, list: &ItemList) {
        self.0.dispatch(SetShoppingList {
            new_list: ItemListSerialized::from(list),
        })
    }
}

#[server(SetShoppingList, "/api")]
pub async fn set_shopping_list(
    cx: Scope,
    new_list: ItemListSerialized,
) -> Result<ItemListSerialized, ServerFnError> {
    todo!();
    /*
    use actix_web::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
    use leptos_actix::{ResponseOptions, ResponseParts};

    let response =
        use_context::<ResponseOptions>(cx).expect("to have leptos_actix::ResponseOptions provided");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!("darkmode={prefers_dark}; Path=/"))
            .expect("to create header value"),
    );
    response_parts.headers = headers;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    response.overwrite(response_parts).await;
    Ok(prefers_dark)
    */
}
