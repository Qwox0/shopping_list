use leptos::*;
use std::fmt::Display;

// Extend leptos

pub trait ReadSignalUtils<T: PartialEq> {
    fn contains(&self, rhs: &T) -> bool;
}

impl<T: PartialEq> ReadSignalUtils<T> for ReadSignal<T> {
    fn contains(&self, rhs: &T) -> bool {
        self.with(|x| x == rhs)
    }
}

impl<T: PartialEq> ReadSignalUtils<T> for RwSignal<T> {
    fn contains(&self, rhs: &T) -> bool {
        self.with(|x| x == rhs)
    }
}

/// Returns the [Window] Object if on the Client.
/// During SSR this returns `None`
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
    return Some(document().unchecked_into::<web_sys::HtmlDocument>());
}

/// Returns the value of the (first) cookie with the given name.
/// During SSR this checks the Request Cookies
#[allow(unused_variables)]
pub fn get_cookie(cx: Scope, cookie_name: impl Into<String>) -> Option<String> {
    let cookie_name = cookie_name.into();

    #[cfg(not(feature = "ssr"))]
    return Some(
        get_html_document()
            .expect("Document is available on the Client")
            .cookie() // String with all Cookies ("name1=val1; name2=val2") in a Result
            .ok()?
            .split("; ")
            .find_map(|cookie| cookie.strip_prefix(&format!("{}=", cookie_name)))?
            .to_string(),
    );
    #[cfg(feature = "ssr")]
    return Some(
        use_context::<actix_web::HttpRequest>(cx)?
            .cookie(&cookie_name)? // single cookie Object (first one?)
            .value()
            .to_string(),
    );
}

pub fn set_cookie(cx: Scope, key: impl Display, value: impl Display) {
    let cookie = format!("{key}={value}; Path=/");

    #[cfg(feature = "ssr")]
    {
        use actix_web::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
        use leptos_actix::{ResponseOptions, ResponseParts};

        let response = use_context::<ResponseOptions>(cx)
            .expect("to have leptos_actix::ResponseOptions provided");
        let mut response_parts = ResponseParts::default();
        let mut headers = HeaderMap::new();
        headers.insert(
            SET_COOKIE,
            HeaderValue::from_str(&cookie).expect("to create header value"),
        );
        response_parts.headers = headers;
        response.overwrite(response_parts);
    }
    #[cfg(not(feature = "ssr"))]
    {
        get_html_document()
            .expect("Document is available on the Client")
            .set_cookie(&cookie)
            .expect("no error while setting cookie")
    }
}
