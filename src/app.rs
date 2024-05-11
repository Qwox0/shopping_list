use crate::{
    barcode_scanner::Barcode,
    camera::CameraService,
    db_tool::DBTool,
    item::{data::NewItem, openfoodsfacts, server_functions::add_item_from_barcode},
    language::Language,
    login::LoginView,
    main_page::MainPage,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use web_sys::{Document, Navigator, Window};

pub fn use_window() -> Option<Window> {
    #[cfg(feature = "ssr")]
    return None;
    #[cfg(not(feature = "ssr"))]
    return Some(window());
}

pub fn use_document() -> Option<Document> {
    #[cfg(feature = "ssr")]
    return None;
    #[cfg(not(feature = "ssr"))]
    return Some(document());
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    /*
    let a = use_window()
        .as_ref()
        .map(Window::navigator)
        .as_ref()
        .and_then(Navigator::language)
        .and_then(Language::new)
        .unwrap_or_default();
    logging::log!("preferred lang: {:?}", a);

    let ht = use_document().as_ref().and_then(Document::document_element);

    logging::log!("1: {:?}", ht.is_some());

    create_effect(move |_| {
        logging::log!("2: {:?}", ht.is_some());
    });
    */

    provide_context(CameraService::new());

    //#[cfg(feature = "ssr")]
    //provide_context(crate::db::DB::new().expect("could connect to DB"));

    view! {
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="description" content="Fullstack Rust Shopping List"/>
        <Stylesheet id="leptos" href="/pkg/shopping_list.css"/> // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Link rel="shortcut icon" type_="image/svg+xml" href="/img/favicon.svg"/>
        <Link rel="manifest" href="/pwa.webmanifest"/>

        <Script type_="text/javascript" src="/js/init_sw.js"/>

        <Title text="Shopping List"/>

        // content for this welcome page
        /*
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
        */
        <Router fallback=|| view! { <h1>"Not Found"</h1> }.into_view()>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=MainPage
                        ssr=SsrMode::Async
                    />
                    <Route path="/login" view=LoginView/>
                    <Route path="/db" view=DBTool/>
                </Routes>
            </main>
        </Router>
    }
}
