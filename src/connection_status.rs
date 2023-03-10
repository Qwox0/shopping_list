use leptos::*;

#[component]
pub fn ConnectionStatus(cx: Scope) -> impl IntoView {
    let (is_online, set_online) = create_signal(cx, crate::util::is_server_available());

    window_event_listener("online", move |_| set_online(true));
    window_event_listener("offline", move |_| set_online(false));

    view! {cx,
        <div class="connection">
            {move || if is_online() { "online ✅" } else { "offline ❌" }}
        </div>
    }
}
