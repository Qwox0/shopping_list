use leptos::*;

use crate::language::LangReader;
use crate::language::{dict, init_dict};

#[component]
pub fn ConnectionStatus(cx: Scope) -> impl IntoView {
    let lang = init_dict!(cx);
    let (is_online, set_online) = create_signal(cx, window().navigator().on_line());
    log!("{:?}", lang);


    window_event_listener("online", move |_| set_online(true));
    window_event_listener("offline", move |_| set_online(false));

    view! {cx,
        <div class="connection">
            {move || if is_online() { "online ✅" } else { "offline ❌" }}
        </div>
    }
}
