use super::{Language, LANGUAGES};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::str::FromStr;

#[server(SetLanguage, "/api")]
pub async fn set_language(cx: Scope, new_language: Language) -> Result<Language, ServerFnError> {
    use actix_web::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
    use leptos_actix::{ResponseOptions, ResponseParts};

    let response =
        use_context::<ResponseOptions>(cx).expect("to have leptos_actix::ResponseOptions provided");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!("language={new_language}; Path=/"))
            .expect("to create header value"),
    );
    response_parts.headers = headers;

    std::thread::sleep(std::time::Duration::from_millis(1500));
    log!("new language: {:?}", new_language);

    response.overwrite(response_parts).await;
    Ok(new_language)
}

fn language_from_cookie(cx: Scope) -> Language {
    const SITE_DEFAULT_LANGUAGE: Language = Language::English;
    crate::util::get_cookie(cx, "language")
        .map(|s| Language::from_str(&s).ok())
        .flatten()
        .unwrap_or(SITE_DEFAULT_LANGUAGE)
}

#[component]
pub fn LanguageSelector(cx: Scope) -> impl IntoView {
    let set_lang = use_context::<crate::language::LangReader>(cx)
        .expect("`LangReader` context is available")
        .language
        .write_only();

    let initial = language_from_cookie(cx);

    let set_language_action = create_server_action::<SetLanguage>(cx);
    let input = set_language_action.input();
    let value = set_language_action.value();

    let current_language = create_memo(cx, move |_| { // memo prevent unnecessary updates
        match (input(), value()) {
            (Some(submission), _) => submission.new_language, // if there's some current input, use that optimistically
            (_, Some(Ok(lang))) => lang, // otherwise, if there was a previous value confirmed by server, use that
            _ => initial,                // otherwise, use the initial value
        }
    });

    let set_language =
        move |new_language| set_language_action.dispatch(SetLanguage { new_language });

    create_effect(cx, move |_| {
        //let changed_lang = current_language();
        //log!("changed lang: {:?}", changed_lang);
        //set_lang(changed_lang);

        log!("action: {:?}", (input().map(|x| x.new_language), value()));
    });

    let options: Vec<_> = LANGUAGES
        .iter()
        .map(|lang| {
            view! {cx,
                <option
                    value=move || lang.to_string()
                >{lang.to_string()}</option>
            }
        })
        .collect();

    view! { cx,
        /*
        */
        <select
            name="new_language"
            on:change=move |e| set_language(Language::from_str(&event_target_value(&e)).expect("valid language option"))
        >
            {options}
        </select>
        /*
        <ActionForm action=set_language_action>
            <select
                name="new_language"
                //on:change=
            >
                {options}
            </select>
            <input type="submit"
                value="submit"
            />
        </ActionForm>
        */
        <span>
            {move || current_language().to_string()}
        </span>
    }
}
