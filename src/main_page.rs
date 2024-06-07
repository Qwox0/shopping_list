use crate::{
    header_bar::HeaderBar,
    item::{RefreshList, ShowNewItem},
    list::ListView,
};
use leptos::*;

#[component]
pub fn MainPage() -> impl IntoView {
    provide_context(ShowNewItem::default());
    provide_context(RefreshList::default());

    view! {
        <HeaderBar />

        <section id="list-sec">
            <ListView />
        </section>
    }
}
