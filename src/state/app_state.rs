use crate::state::{language::LanguageContext, list::ItemList};
use leptos::*;

pub struct AppState {
    pub language: LanguageContext,
    pub item_list: ItemList,
}

impl AppState {
    pub fn new(cx: Scope) -> AppState {
        AppState {
            language: LanguageContext::new(cx),
            item_list: ItemList::new(cx),
        }
    }

    pub fn from_context(cx: Scope) -> &'static AppState {
        use_context::<&'static AppState>(cx).expect("`&'static AppState` was provided")
    }
}
