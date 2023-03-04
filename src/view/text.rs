use crate::state::{app_state::AppState, language::Dictionary};
use leptos::*;

/// prevent multiple definitions of text
mod text_macro {
    /// get Text in the currently selected language
    /// For displaying text inside the [`leptos::view`] macro, use the [`crate::view::text::Text`] component instead!
    ///
    /// text!(cx, getter) -> (|| -> String)
    ///
    /// # Types
    ///
    /// cx: [`leptos::Scope`]
    /// getter: FnOnce(&Dictionary) -> &T
    /// Dictionary: [`crate::state::language::Dictionary`]
    macro_rules! text {
        ( $cx:ident, $getter:expr ) => {{
            let cx: ::leptos::Scope = $cx;
            move || {
                AppState::from_context(cx)
                    .language
                    .get_text(cx, $getter)
                    .to_string()
            }
        }};
    }
    pub(crate) use text;
}

pub(crate) use text_macro::text;

/// write Text in the currently selected language
#[component]
pub fn Text<F>(cx: Scope, getter: F) -> impl IntoView
where
    F: Fn(&Dictionary) -> String + 'static,
{
    view! { cx,
        <span> { text!(cx, &getter) } </span>
    }
}
