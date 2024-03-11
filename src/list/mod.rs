use self::item::{Item, ItemView};
use crate::barcode::Barcode;
use leptos::{leptos_dom::logging::console_log, *};
use std::vec;

mod item;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct List {
    list: Vec<Item>,
}

#[component]
pub fn ListView() -> impl IntoView {
    let (list, set_list) = create_signal(List::default());

    async fn barcode_to_item(barcode: Barcode) -> Item {
        Item::from_barcode(barcode).await.unwrap()
    }

    let r = create_resource(move || Barcode::ean13(4015184000896), barcode_to_item);

    let list = move || r.get().into_iter().collect::<Vec<_>>();

    view! {
        /*
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            {move || {
                r.get().map(|item| view! {
                    <ItemView item/>
                }
            )}}
        </Suspense>
        */
        /*
        <ul id="shopping_list">
            <For
                each=move || list()
                key=|item| item.db_id
                children=move |item: Item| {
                    view! { <ItemView item/> }
                }
            />
        </ul>
        */
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <ul id="shopping_list">
                <For
                    each=list
                    key=|item| item.db_id
                    children=move |item: Item| {
                        view! { <ItemView item/> }
                    }
                />
            </ul>
        </Suspense>
    }
}

impl IntoIterator for List {
    type IntoIter = vec::IntoIter<Self::Item>;
    type Item = Item;

    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}
