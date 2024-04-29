use crate::{header_bar::HeaderBar, item::ShowNewItem, list::ListView};
use leptos::*;

#[component]
pub fn MainPage() -> impl IntoView {
    provide_context(ShowNewItem::default());

    view! {
        <HeaderBar />

        <section id="list-sec">
            <ListView />
        </section>
    }
}
