use crate::{
    language::*,
    view::{connection_status::*, list::*},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    // provide language text to all components (use init_dict!() and dict!())
    let lang_reader = LangReader::new(cx);
    provide_context(cx, lang_reader.clone());

    view! { cx,
        <SiteHead />

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
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
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        <Script type_="text/javascript" src="init_sw.js"/>

        <Title text="Shopping List"/>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let set_lang = use_context::<crate::language::LangReader>(cx)
        .expect("`LangReader` context is available")
        .language
        .write_only();
    view! { cx,
        <header>
            <LanguageSelector set_lang/>
            <ConnectionStatus/>
        </header>
        <main>
            <h1> <Text getter=|d| &d.shopping_list/> </h1>
            <ShoppingList/>
        </main>
    }
}
