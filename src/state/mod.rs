#[cfg(feature = "ssr")]
use crate::db::ItemsDbTable;
use crate::list::item::{Item, ItemSerialized};
use leptos::*;

pub struct RenderState {
    pub item_list: RwSignal<Vec<Item>>,
}

impl RenderState {
    pub fn new(cx: Scope) -> RenderState {
        RenderState {
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
