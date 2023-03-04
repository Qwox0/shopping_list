use anyhow::Context;
use leptos::*;
use serde::{de::DeserializeOwned, Serialize};

/*
/// Wrapper for [`web_sys::Storage`].
#[derive(Debug, Clone, Copy)]
pub struct LocalStorage<'a, T, Serialized = T>
where
    T: 'static + FromWithScope<Serialized>,
    Serialized: for<'b> From<&'b T> + Serialize + DeserializeOwned,
{
    value: RwSignal<T>,
    key: &'a str,
    serialized_type: PhantomData<Serialized>,
}

impl<'a, T, Serialized> LocalStorage<'a, T, Serialized>
where
    T: 'a + 'static + FromWithScope<Serialized>,
    Serialized: for<'b> From<&'b T> + Serialize + DeserializeOwned,
{
    pub fn new(cx: Scope, key: &'static str, default: T) -> Self {
        let s = load_with_key(key).ok();
        let value = s
            .as_ref()
            .map(|s| serde_json::from_str::<Serialized>(&s).ok())
            .flatten()
            .map(|item| T::from(cx, item))
            .unwrap_or(default);
        let value = create_rw_signal(cx, value);

        create_effect(cx, move |_| {
            if let Err(e) = serde_json::to_string(&value.with(|t| Serialized::from(t)))
                .context("Failed to serialize")
                .map(|json| save(&key, &json))
                .flatten()
            {
                warn!("{}", e);
            }
        });

        Self {
            value,
            key,
            serialized_type: PhantomData,
        }
    }

    pub fn update(&mut self, f: impl FnOnce(&mut T)) {
        self.value.update(f)
    }

    pub fn get(&self) -> RwSignal<T> {
        self.value
    }
}
*/

fn load_with_key(key: &str) -> Result<String, anyhow::Error> {
    log!("load: {}", key);
    let s = crate::util::get_window()
        .context("Failed to get window")?
        .local_storage()
        .ok()
        .flatten()
        .context("Failed to get local storage")?
        .get_item(key)
        .ok()
        .flatten()
        .context("Failed to get Storage");
    if let Ok(str) = s.as_ref() {
        log!("load: {} -> {}", key, str);
    }
    s
}

fn save(key: &str, json: &str) -> anyhow::Result<()> {
    log!("save: {key} -> {json}");
    crate::util::get_window()
        .context("Failed to get window")?
        .local_storage()
        .ok()
        .flatten()
        .context("Failed to get local storage")?
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
