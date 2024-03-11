use crate::img_popup::ImgPopup;
use leptos::*;

#[component]
pub fn Image(full_url: String, thumb_url: String) -> impl IntoView {
    let popup_data = ImgPopup::from_context();
    let open_popup = move |_| popup_data.open(&full_url);
    view! {
        <img
            src=thumb_url
            class="image cursor-pointer"
            on:click=open_popup
        />
    }
}
