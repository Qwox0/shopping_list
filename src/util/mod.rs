mod cookies;
mod from_with_scope;
mod local_storage;
mod signals;
pub use cookies::*;
pub use from_with_scope::*;
pub use local_storage::*;
pub use signals::*;

use leptos::*;

// Extend leptos

pub fn err_fallback(cx: Scope, errors: RwSignal<Errors>) -> impl IntoView {
    view! { cx,
        <div class="error">
            <h1>"Something went wrong."</h1>
            <ul>
            {move || errors.get()
                .into_iter()
                .map(|(_, error)| view! { cx, <li>{error.to_string()} </li> })
                .collect::<Vec<_>>()
            }
            </ul>
        </div>
    }
}

/// Returns the [`web_sys::Window`] Object if on the Client.
/// During SSR this returns [`None`]
pub fn get_window() -> Option<web_sys::Window> {
    #[cfg(feature = "ssr")]
    return None;
    #[cfg(not(feature = "ssr"))]
    return Some(window());
}

#[allow(unused)]
pub fn get_html_document() -> Option<web_sys::HtmlDocument> {
    #[cfg(feature = "ssr")]
    return None;
    #[cfg(not(feature = "ssr"))]
    {
        use wasm_bindgen::JsCast;
        return Some(document().unchecked_into::<web_sys::HtmlDocument>());
    }
}

pub fn is_server_available() -> bool {
    get_window()
        .map(|w| w.navigator().on_line())
        .unwrap_or(true)
}
