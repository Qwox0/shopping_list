use std::future::Future;

use anyhow::Context;
use anyhow::Error;
use common::item::Item as ItemSync;
use leptos::wasm_bindgen::JsValue;
use leptos::*;
use thiserror::Error;

const STORAGE_KEY: &str = "item-list";

pub struct Item {
    pub id: Uuid,
    pub name: RwSignal<String>,
    pub amount: RwSignal<usize>,
    pub state: RwSignal<ItemState>,
}

impl From<ItemSync> for Item {
    fn from(item: ItemSync) -> Self {
        Item {
            id: item.id,
            name: create_rw_signal(cx, value),
            amount: (),
            state: (),
        }
    }
}

#[derive(Debug, Error)]
enum FetchError {
    #[error("a")]
    A(JsValue),
}

pub struct ItemList {
    pub items: Vec<Item>,
}

async fn test() -> i32 {
    1
}

impl ItemList {
    pub fn new() -> Self {
        ItemList {
            items: ItemList::fetch_local().unwrap_or(vec![]),
        }
    }

    fn fetch_local() -> anyhow::Result<Vec<Item>> {
        window()
            .local_storage()
            .ok()
            .flatten()
            .context("failed to open local storage")?;
        Ok(vec![])
    }

    async fn fetch_server() {}
}
