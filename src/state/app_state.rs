#[cfg(feature = "ssr")]
use crate::db::ItemsDbTable;
use crate::{
    language::context::LanguageContext,
    list::{
        item::{Item, ItemSerialized},
        ItemList,
    },
};
use leptos::*;

pub struct AppState {
    //pub language: LanguageContext,
    pub item_list: ItemList,
}

impl AppState {
    pub fn new(cx: Scope) -> AppState {
        AppState {
            //item_list: create_rw_signal(cx, RenderState::init_item_list(cx)),
            item_list: create_rw_signal(cx, vec![]),
        }
    }

    /*
    fn init_item_list(cx: Scope) -> Vec<Item> {
        let mut list = vec![];
        #[cfg(feature = "ssr")]
        spawn_local(async {
            list = ItemsDbTable::<ItemSerialized>::new()
                .await
                .expect("got db table")
                .get_all()
                .await
                .expect("got items")
                .into_iter()
                .map(|s| Item::from_serialized(cx, s))
                .collect();
        });
        log!("list: {:?}", list);
        list
    }
    */
}
