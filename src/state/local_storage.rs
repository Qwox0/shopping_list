use crate::util::FromWithScope;
use anyhow::Context;
use leptos::*;
use serde::{de::DeserializeOwned, Serialize};
use web_sys::Storage;
use std::marker::PhantomData;

fn get_local_storage() -> anyhow::Result<Storage> {
    crate::util::get_window()
        .context("Failed to get window")?
        .local_storage()
        .ok()
        .flatten()
        .context("Failed to get local storage")
}

fn load_with_key(key: &str) -> Result<String, anyhow::Error> {
    log!("load: {key}");
    get_local_storage()?
        .get_item(key)
        .ok()
        .flatten()
        .context("Failed to get item from Storage")
}

fn save(key: &str, json: &str) -> anyhow::Result<()> {
    log!("save: {key} -> {json}");
    get_local_storage()?
        .set_item(key, json)
        .ok()
        .context("Failed to set local storage")
}

pub trait InLocalStorage: Sized {
    const STORAGE_KEY: &'static str;
    type Content: std::fmt::Debug;
    type Serialized: Serialize + DeserializeOwned;

    fn serialize(content: &Self::Content) -> Self::Serialized;
    fn deserialize(cx: Scope, serialized: Self::Serialized) -> Self::Content;

    fn load_item(cx: Scope, default: Self::Content) -> RwSignal<Self::Content> {
        let value = load_with_key(Self::STORAGE_KEY)
            .ok()
            .as_ref()
            .map(|s| serde_json::from_str::<Self::Serialized>(&s).ok())
            .flatten()
            .map(|item| Self::deserialize(cx, item))
            .unwrap_or(default);
        dbg!(&value);
        let value = create_rw_signal(cx, value);

        create_effect(cx, move |_| {
            if let Err(e) = serde_json::to_string(&value.with(Self::serialize))
                .context("Failed to serialize")
                .map(|json| save(&Self::STORAGE_KEY, &json))
                .flatten()
            {
                warn!("{}", e);
            }
        });
        value
    }
}
