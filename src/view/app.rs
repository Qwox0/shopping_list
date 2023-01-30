use crate::{
    language::{selector::*, *},
    view::{connection_status::*, list::*},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = SetLanguage::register();
    //_ = ToggleDarkMode::register();
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    // provide language text to all components (use <Text/> or text!())
    let lang_reader = LangReader::new(cx);
    provide_context(cx, lang_reader.clone());

    view! { cx,
        <SiteHead />
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=move |cx| view! { cx, <HomePage lang=Language::English/> }/>
                    <Route path="" view=move |cx| view! { cx, "other: " <Outlet /> }>
                        <Route path="a" view=move |cx| view! { cx, "empty a" }/>
                        <Route path=":id" view=move |cx| view! { cx, ":id" }/>
                        <Route path="" view=move |cx| view! { cx, "fallback" }/>
                    </Route>
                    <Route path="a" view=move |cx| view! { cx, "a" }/>
                    <Route path="b" view=move |cx| view! { cx, "b" }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn SiteHead(cx: Scope) -> impl IntoView {
    view! { cx,
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="description" content="Fullstack Rust Shopping List"/>
        <Stylesheet id="leptos" href="/pkg/shopping_list.css"/> // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Link rel="shortcut icon" type_="image/ico" href="/img/favicon.ico"/>

        <Script type_="text/javascript" src="/js/init_sw.js"/>

        <Title text="Shopping List"/>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope, lang: Language) -> impl IntoView {
    view! { cx,
        <header>
            <LanguageSelector/>
            <ConnectionStatus/>
        </header>
        <main>
            <h1> <Text getter=|d| &d.shopping_list/> </h1>
            <ShoppingList/>
        </main>
        { move || format!("{}", lang)}
    }
}

/* not usable :c
<For each=|| LANGUAGES
    key=|lang| lang.clone()
    view=move |lang| view! { cx,
        <Route path={lang.short()} view=move |cx| view! { cx, <HomePage lang/> }/>
    }
/>
// nor this one
let a:Vec<_> = LANGUAGES.into_iter().map(move |lang| {
    view! {cx,
        //<Route path={lang.short()} view=move |cx| view! { cx, <HomePage lang/> }/>
        <Route path="c" view=move |cx| view! { cx, "c" }/>
    }
}).collect();
        // */
