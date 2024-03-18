use crate::{
    header_bar::HeaderBar,
    item::{NewItemView, ShowNewItem},
    list::ListView,
};
use leptos::*;

#[component]
pub fn MainPage() -> impl IntoView {
    provide_context(ShowNewItem::default());

    view! {
        <HeaderBar />

        <section id="list-sec">
            <ListView />
        </section>

        <img src="img/barcode-scan-svgrepo-com.svg" />
        <img src="img/barcode-scan-svgrepo-com.svg" />
        <img src="img/barcode-scan-svgrepo-com.svg" />
        <img src="img/barcode-scan-svgrepo-com.svg" />
        <img src="img/barcode-scan-svgrepo-com.svg" />
    }
}
