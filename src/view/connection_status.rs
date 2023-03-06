use crate::state::app_state::AppState;
use leptos::*;

#[component]
pub fn ConnectionStatus(cx: Scope) -> impl IntoView {
    let is_online = AppState::from_context(cx).is_online;
    view! {cx,
        <div class="connection">
            {move || if is_online() { "online ✅" } else { "offline ❌" }}
        </div>
    }
}
