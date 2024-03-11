use crate::util::force_use_context;
use leptos::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct ImgPopup(RwSignal<Option<String>>);

impl ImgPopup {
    pub fn from_context() -> ImgPopup {
        force_use_context()
    }

    pub fn open(&self, url: impl ToString) {
        self.0.update(|u| {
            let _ = u.insert(url.to_string());
        });
    }

    pub fn close(&self) {
        self.0.update(|u| {
            u.take();
        })
    }

    pub fn is_open(&self) -> bool {
        self.0.with(Option::is_some)
    }
}

#[component]
pub fn ImgPopupView(data: ImgPopup) -> impl IntoView {
    view! {
        <div
            class="popup-img-container cursor-pointer"
            hidden=move || !data.is_open()
            on:click=move |_| data.close()
        >
            <img
                src=move || data.0.get()
                class="popup-img"
            />
        </div>
    }
}
