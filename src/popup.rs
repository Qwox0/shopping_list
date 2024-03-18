use leptos::*;

#[component]
pub fn Popup(popup: PopupSignal, children: ChildrenFn) -> impl IntoView {
    view! {
        <Show when=move || popup.is_open()>
            <div
                class="popup-container cursor-pointer"
                on:click=move |_| popup.close()
            >
                { children() }
            </div>
        </Show>
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PopupSignal {
    open: RwSignal<bool>,
}

impl PopupSignal {
    pub fn new() -> PopupSignal {
        PopupSignal { open: create_rw_signal(false) }
    }

    pub fn open(&self) {
        self.open.set(true)
    }

    pub fn close(&self) {
        self.open.set(false)
    }

    pub fn toggle(&self) {
        self.open.update(|b| *b = !*b)
    }

    pub fn is_open(&self) -> bool {
        self.open.get()
    }
}
