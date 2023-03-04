mod header;

use crate::{
    language::text_macro::text,
    list::{
        header::{ListHeader, ListHeaderProps},
        item::{ItemView, ItemViewProps},
    },
    util::{FromWithScope, InLocalStorage},
};
use anyhow::Context;
use item::*;
use leptos::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const LIST_STORAGE_KEY: &str = "shopping-list-items";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListMsg {
    Get(usize), // serde_urlencoded only supports Strings
    GetAll,
}

#[server(GetItemList, "/api", "Cbor")] // Cbor: + smaller + allows for enums with non String values - needs wasm even in forms
pub async fn get_item_list(cx: Scope, msg: ListMsg) -> Result<Vec<ItemSerialized>, ServerFnError> {
    log!("msg: {:?}", msg);
    let list = crate::db::ItemsDbTable::new()
        .await
        .expect("got db")
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
    pub fn new(cx: Scope, init_msg: ListMsg) -> Self {
        let message = create_rw_signal(cx, init_msg);
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

    pub fn read2(&self, cx: Scope) -> Option<Result<Vec<Item>, ServerFnError>> {
        let deserialize_list = |list: &Vec<ItemSerialized>| {
            list.iter().map(|i| Item::from_serialized_ref(cx, i)).collect()
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
}

#[component]
pub fn List(cx: Scope) -> impl IntoView {
    let list = ItemList::new(cx, ListMsg::Get(0));
    /*
    let items_view = move |items: Vec<Item>| {
        view! { cx,
            <ul class="items">
                <For each=move || items.into_iter().rev() //move || list.with(|l| l.items.clone())
                    key=|item| item.id
                    view=move |cx, item| view! { cx,
                        <li> <ItemView item/> </li>
                    }
                />
            </ul>
        }
    };
    let res_view = move |res: Result<Vec<Item>, ServerFnError>| {
        view! { cx,
            <ErrorBoundary fallback=crate::util::err_fallback>
                { move || res.map(items_view) }
            </ErrorBoundary>
        }
    };

    let list_view = move || list.read2(cx).map(res_view);
    */
    let fallback = move || match list.read2(cx) {
        Some(l) => view! { cx, <><p>{ move || format!("Some({:?})", l) }</p></> }, //view! { cx, <>{ move || res_view(l) }</> },
        None => view! { cx, <><p>"Loading (Suspense Fallback)..."</p></> },
    };

    let l = move |cx: Scope, l: &Vec<ItemSerialized>| {
        l.iter()
            .rev()
            .map(move |a| Item::from_serialized(cx, a.clone()))
            .collect::<Vec<Item>>()
    };

    view! { cx,
        <section class="shopping-list">
            <ListHeader/>
            <Suspense fallback>
                //{ list_view }
                <ErrorBoundary fallback=crate::util::err_fallback>
                    { move || list.with(cx, move |res| {
                        res.clone().map(|items| {
                            view! { cx,
                                <ul class="items">
                                    /*
                                    { move || items.iter().rev().map(|item| {
                                        view! { cx,
                                            <li> <ItemView item=Item::from_serialized(cx, item)/> </li>
                                        }
                                    })}
                                    */
                                    //<For each=move || items.iter().rev() //move || list.with(|l| l.items.clone())
                                    <For each=move || l(cx, &items)
                                        key=|item| item.id
                                        view=move |cx, item| view! { cx,
                                            <li> <ItemView item/> </li>
                                        }
                                    />
                                </ul>
                            }
                        })
                    }) }
                </ErrorBoundary>
            </Suspense>
        </section>
        <div>
            <input type="number"
                on:change=move |e| list.send_msg(ListMsg::Get(event_target_value(&e).parse::<usize>().unwrap_or(1)))
            />
            <Suspense fallback>
                <p>{ move || format!("{:?}", list.read(cx)) }</p>
            </Suspense>
        </div>
    }
    /*

    view! {cx,
    }
        */
}

//let initial = ItemList::from(vec![]); //initial_prefers_dark(cx);

/*
let set_list_action = SetShoppingListAction::create(cx);
let (input, value) = set_list_action.get_signals();
let list_serialized = move || {
    match (input(), value()) {
        (Some(submission), _) => submission.new_list, // if there's some current input, use that optimistically
        (_, Some(Ok(value))) => value, // otherwise, if there was a previous value confirmed by server, use that
        _ => ItemListSerialized(vec![]), // otherwise, use the initial value
    }
};
let list = move || ItemList::from_serialized(cx, list_serialized());

provide_context(cx, set_list_action);

#[derive(Debug, Clone)]
//pub struct List(Vec<Item>);
pub struct List<'a>(LocalStorage<'a, Vec<Item>, ListSerialized>);

impl List<'_> {
    pub fn new(cx: Scope) -> Self {
        List(LocalStorage::new(cx, LIST_STORAGE_KEY, vec![]))
    }

    pub fn split(&self) -> (ReadSignal<Vec<Item>>, WriteSignal<Vec<Item>>) {
        self.0.get().split()
    }

    pub fn add(&mut self, item: Item) {
        self.0.update(|list| list.push(item));
    }

    pub fn add_new_item(&mut self, cx: Scope, new_item: Item) {
        self.add(Item::from_new_item(cx, new_item))
    }

    pub fn remove(&mut self, id: Uuid) {
        self.0.update(|list| list.retain(|item| item.id != id))
    }
}

impl FromWithScope<ListSerialized> for Vec<Item> {
    fn from(cx: Scope, serialized: ListSerialized) -> Self {
        serialized
            .0
            .into_iter()
            .map(|item| <item::Item as FromWithScope<ItemSerialized>>::from(cx, item))
            .collect()
    }
}
*/

#[derive(Debug, Clone, Copy)]
pub struct List(pub RwSignal<Vec<Item>>);

impl List {
    pub fn new(cx: Scope) -> Self {
        List(List::load_item(cx, vec![]))
    }

    pub fn add(&mut self, item: Item) {
        self.0.update(|list| list.push(item));
    }

    pub fn add_new_item(&mut self, cx: Scope, new_item: Item) {
        self.add(Item::from_new_item(cx, &new_item))
    }

    pub fn remove(&mut self, id: Uuid) {
        self.0.update(|list| list.retain(|item| item.id != id))
    }
}

impl InLocalStorage for List {
    const STORAGE_KEY: &'static str = LIST_STORAGE_KEY;
    type Content = Vec<Item>;
    type Serialized = Vec<ItemSerialized>;

    fn serialize(content: &Self::Content) -> Self::Serialized {
        content.iter().map(ItemSerialized::from).collect()
    }

    fn deserialize(cx: Scope, serialized: Self::Serialized) -> Self::Content {
        serialized
            .into_iter()
            .map(|item| Item::from_serialized(cx, item))
            .collect()
    }
}

// ------ Serialized -------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListSerialized(Vec<ItemSerialized>);

impl ListSerialized {
    pub fn try_from_local_storage() -> anyhow::Result<Self> {
        let a = crate::util::get_window()
            .context("Failed to get window")?
            .local_storage()
            .ok()
            .flatten()
            .context("Failed to get local storage")?
            .get_item(LIST_STORAGE_KEY)
            .ok()
            .flatten()
            .context("Failed to get Storage")?;
        log!("Storage {}: {:?}", LIST_STORAGE_KEY, a);
        serde_json::from_str::<ListSerialized>(&a);

        Ok(ListSerialized(vec![]))
    }

    pub fn save_to_local_storage(&self) -> anyhow::Result<()> {
        let json = serde_json::to_string(&self).context("Failed to serialize")?;
        crate::util::get_window()
            .context("Failed to get window")?
            .local_storage()
            .ok()
            .flatten()
            .context("Failed to get local storage")?
            .set_item(LIST_STORAGE_KEY, &json)
            .ok()
            .context("Failed to set local storage")
    }
}

impl From<&Vec<Item>> for ListSerialized {
    fn from(value: &Vec<Item>) -> Self {
        ListSerialized(value.iter().map(ItemSerialized::from).collect())
    }
}

// ------ Actions -------

/*
#[derive(Clone)]
struct SetShoppingListAction(pub Action<SetShoppingList, Result<ListSerialized, ServerFnError>>);

impl SetShoppingListAction {
    pub fn create(cx: Scope) -> Self {
        Self(create_server_action::<SetShoppingList>(cx))
    }

    pub fn get_signals(
        &self,
    ) -> (
        RwSignal<Option<SetShoppingList>>,
        RwSignal<Option<Result<ListSerialized, ServerFnError>>>,
    ) {
        let input = self.0.input(); // input is `Some(value)` when pending, and `None` if not pending
        let value = self.0.value(); // value contains most recently-returned value
        (input, value)
    }

    pub fn get_list(&self) -> List {
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

    pub fn set_list(&self, list: &List) {
        self.0.dispatch(SetShoppingList {
            new_list: ListSerialized::from(list),
        })
    }
}

#[server(SetShoppingList, "/api")]
pub async fn set_shopping_list(
    cx: Scope,
    new_list: ListSerialized,
) -> Result<ListSerialized, ServerFnError> {
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
*/
