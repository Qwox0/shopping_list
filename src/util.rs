use leptos::*;
use web_sys::Window;

/// Returns the [Window] Object if on the Client.
/// During SSR this returns `None`
pub fn get_window() -> Option<Window> {
    #[cfg(feature = "ssr")]
    return None;
    #[cfg(not(feature = "ssr"))]
    return Some(window());
}

/// Returns the value of the (first) cookie with the given name.
/// During SSR this checks the Request Cookies
pub fn get_cookie<'a>(cx: Scope, cookie_name: impl Into<String>) -> Option<String> {
    __get_cookie(cx, cookie_name.into())
}

#[cfg(not(feature = "ssr"))]
fn __get_cookie<'a>(_cx: Scope, cookie_name: String) -> Option<String> {
    Some(
        document()
            .unchecked_into::<web_sys::HtmlDocument>()
            .cookie() // String with all Cookies ("name1=val1; name2=val2") in a Result
            .ok()?
            .split("; ")
            .find_map(|cookie| cookie.strip_prefix(&format!("{}=", cookie_name)))?
            .to_string(),
    )
}

#[cfg(feature = "ssr")]
fn __get_cookie<'a>(cx: Scope, cookie_name: String) -> Option<String> {
    Some(
        use_context::<actix_web::HttpRequest>(cx)?
            .cookie(&cookie_name)? // single cookie Object (first one?)
            .value()
            .to_string(),
    )
}
