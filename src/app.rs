use crate::head::{SiteHead, SiteHeadProps};
use crate::{
    connection_status::*,
    language::{context::LanguageContext, *},
    list::{ListView, ListViewProps},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    //_ = SetLanguage::register();
    //_ = crate::language::dictionary::LoadDictionary::register();
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    // provide language text to all components (use <Text /> or text!())
    // the initial language still has to be set!
    let language = create_rw_signal(cx, Language::from_cookies(cx).unwrap_or_default());
    provide_context(cx, LanguageContext::new(cx, language));

    macro_rules! lang_route {
        ( $path:expr => $lang:expr ) => {
            view! { cx,
                <Route path=$path view=|cx| view! { cx, <HomePage lang=$lang/> } />
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
                    <Route path="" view=move |cx| view! { cx, <Redirect path=language.get().short() /> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope, lang: Language) -> impl IntoView {
    use_context::<LanguageContext>(cx)
        .expect("`LanguageContext` was provided in the `App` component")
        .set_language(cx, lang);

    //log!("path: {:?}", use_location(cx).pathname.get());

    view! { cx,
        <header>
            //<LanguageSelector/>
            <ConnectionStatus/>
        </header>
        <main>
            <h1> <Text getter=|d| d.shopping_list.clone()/> </h1>
            <ListView/>
        </main>
        <Test/>
    }
}

#[component]
fn Test(cx:Scope) -> impl IntoView {
    view! { cx,
        <div class="test">
        </div>
    }

}
