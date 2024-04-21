use leptos::*;

#[component]
pub fn Popup(popup: PopupSignal, children: ChildrenFn) -> impl IntoView {
    // disable scrolling while the popup is open
    create_effect(move |_| {
        if let Some(b) = document().scrolling_element() {
            let v = if popup.is_open() { "hidden" } else { "visible" };
            let _ = b.set_attribute("style", ("overflow: ".to_string() + v + ";").as_str());
        }
    });

    view! {
        <div
            class="popup-container cursor-pointer"
            on:click=move |_| popup.close()
            hidden=move || !popup.is_open()
        >
            <Show when=move || popup.is_open()>
                { children() }
            </Show>
        </div>
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
