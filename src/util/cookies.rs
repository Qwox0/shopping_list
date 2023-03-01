use leptos::*;
use std::fmt::Display;
use super::get_html_document;

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

    #[cfg(not(feature = "ssr"))]
    {
        get_html_document()
            .expect("Document is available on the Client")
            .set_cookie(&cookie)
            .expect("no error while setting cookie")
    }
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
}
