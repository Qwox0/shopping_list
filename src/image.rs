use crate::popup::{Popup, PopupSignal};
use leptos::*;

#[component]
pub fn Image(
    #[prop(optional_no_strip)] full_url: Option<String>,
    #[prop(optional_no_strip)] thumb_url: Option<String>,
) -> impl IntoView {
    let popup = PopupSignal::new();
    if let Some(full_url) = full_url.or_else(|| thumb_url.clone()) {
        view! {
            <img
                src=thumb_url
                alt="Item image"
                title="Full image"
                class="image cursor-pointer"
                on:click=move |ev| {
                    ev.stop_propagation();
                    popup.open();
                }
            />
            <Popup popup>
                <img src=&full_url />
            </Popup>
        }
        .into_view()
    } else {
        view! {
            <img src=thumb_url alt="Item image" class="image" />
        }
        .into_view()
    }
}
