use crate::head::{SiteHead, SiteHeadProps};
use crate::list::item::ItemSerialized;
use crate::{
    connection_status::*,
    language::{context::LanguageContext, *},
    list::{ListView, ListViewProps},
};
use anyhow::Context;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    //_ = SetLanguage::register();
    //_ = crate::language::dictionary::LoadDictionary::register();
    GetItemList::register().expect("could register server fn")
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    // provide language text to all components (use <Text /> or text!())
    // the initial language still has to be set!
    let language = create_rw_signal(cx, Language::from_cookies(cx).unwrap_or_default());
    provide_context(cx, LanguageContext::new(cx, language));

    view! { cx,
        <SiteHead />
        <Router>
            <nav>
                <A href="en">"English"</A>
                <A href="de">"Deutsch"</A>
            </nav>
            <main>
                <Routes>
                    <Route path="/de" view=|cx| view! { cx, <HomePage lang=Language::German/> } ssr=SsrMode::Async />
                    <Route path="/en" view=|cx| view! { cx, <HomePage lang=Language::English/> } ssr=SsrMode::Async />
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

#[server(GetItemList, "/api")]
pub async fn get_item_list(cx: Scope, count: usize) -> Result<Vec<ItemSerialized>, ServerFnError> {
    let list = crate::db::ItemsDbTable::new()
        .await
        .expect("got db")
        .get_all(cx)
        .await
        .expect("got items")
        .into_iter()
        .take(count)
        .collect();
    log!("request list: {:?}", list);
    std::thread::sleep(std::time::Duration::from_millis(1000));
    Ok(list)
}

#[component]
fn Test(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal::<usize>(cx, 0);
    let r_list = create_resource(cx, count, move |count| get_item_list(cx, count));
    let r_fallback = move || {
        if let Some(l) = r_list.read(cx) {
            view! { cx, <p>{ move || format!("Some({:?})", l) }</p> }
        } else {
            view! { cx, <p>"Loading (Suspense Fallback)..."</p> }
        }
    };

    view! { cx,
        <div>
            <input type="number"
                on:change=move |e| set_count(event_target_value(&e).parse::<usize>().unwrap_or(1))
            />
            <br/>
            <Suspense fallback=r_fallback>
                <p>{ move || format!("{:?}", r_list.read(cx)) }</p>
            </Suspense>
        </div>
    }
}
