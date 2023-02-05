use crate::{
    language::{dictionary::LoadDictionary, selector::*, text_macro::text, *},
    view::{
        connection_status::*,
        list::{ShoppingList, ShoppingListProps},
    },
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = SetLanguage::register();
    _ = LoadDictionary::register();
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    // provide language text to all components (use <Text /> or text!())
    // the initial language still has to be set!
    provide_context(cx, LanguageContext::new_empty(cx));

    macro_rules! lang_route {
        ( $path:expr => $lang:expr ) => {
            view! { cx,
                <Route path=$path view=|cx| view! { cx, <HomePage lang=$lang/> }>
                    <Route path="" view=move |cx| view! { cx,
                        <h1> <Text getter=|d| &d.shopping_list/> </h1>
                        <ShoppingList/>
                    }/>
                </Route>
            }
        };
    }

    view! { cx,
        <SiteHead />
        <Router>
            <nav>
                <A href="en">"English"</A>
                <A href="de">"Deutsch"</A>
            </nav>
            <main>
                <Routes>
                    { lang_route!("/de" => Language::German) }
                    { lang_route!("/en" => Language::English) }
                    <Route path="" view=move |cx| view! { cx, <Redirect path="/en" /> }/>
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

        //<Script type_="text/javascript" src="/js/init_sw.js"/> // TODO: include code

        <Title text="Shopping List"/>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope, lang: Language) -> impl IntoView {
    let lc = use_context::<LanguageContext>(cx)
        .expect("empty `LanguageContext` was provided inside the `App` component");
    log!("lang1: {:?}", lc.get_lang());
    lc.set_language(cx, lang);
    log!("lang2: {:?}", lc.get_lang());

    log!("path: {:?}", use_location(cx).pathname.get());

    view! { cx,
        <header>
            //<LanguageSelector/>
            <ConnectionStatus/>
        </header>
        <main>
            <Outlet/>
        </main>
        //{ text!(cx, |d| &d.shopping_list) }
    }
}
