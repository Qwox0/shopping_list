use crate::state::{language::LanguageContext, list::ItemList};
use leptos::*;

pub struct AppState {
    pub language: LanguageContext,
    pub item_list: ItemList,
    pub is_online: ReadSignal<bool>,
    set_online: WriteSignal<bool>,
}

impl AppState {
    pub fn new(cx: Scope) -> AppState {
        let (is_online, set_online) = Self::init_is_online_signal(cx);
        AppState {
            language: LanguageContext::new(cx),
            item_list: ItemList::new(cx),
            is_online,
            set_online,
        }
    }

    pub fn from_context(cx: Scope) -> &'static AppState {
        use_context::<&'static AppState>(cx).expect("`&'static AppState` was provided")
    }

    fn init_is_online_signal(cx: Scope) -> (ReadSignal<bool>, WriteSignal<bool>) {
        let (is_online, set_online) = create_signal(cx, crate::util::is_server_available());
        window_event_listener("online", move |_| set_online(true));
        window_event_listener("offline", move |_| set_online(false));
        return (is_online, set_online);
    }
}
