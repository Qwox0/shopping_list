use crate::{
    barcode_scanner::Barcode,
    camera::CameraService,
    item::{openfoodsfacts, ItemData},
    language::Language,
    list::add_item_from_barcode,
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

    provide_context(CameraService::new());

    #[cfg(feature = "ssr")]
    provide_context(crate::db::DB::new().expect("could connect to DB"));

    view! {
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="description" content="Fullstack Rust Shopping List"/>
        <Stylesheet id="leptos" href="/pkg/shopping_list.css"/> // id=leptos means cargo-leptos will hot-reload this stylesheet
        // <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Link rel="shortcut icon" type_="image/svg+xml" href="/img/favicon.svg"/>
        // <Link rel="manifest" href="/pwa.webmanifest"/>

        // <Script type_="text/javascript" src="/js/init_sw.js"/>

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
                    <Route path="" view=MainPage/>
                    <Route path="/db" view=DBTool/>
                </Routes>
            </main>
        </Router>
    }
}

#[server]
pub async fn db_action(barcode: String, action: String) -> Result<String, ServerFnError> {
    let barcode = Barcode::try_from(barcode)?;

    match action.as_str() {
        "request json" => Ok(format!("{:#}", openfoodsfacts::request_with_barcode(barcode).await?)),
        "request ItemData" => {
            let a = ItemData::from_barcode(barcode).await?;
            Ok(format!("{:#?}", a))
        },
        "Add Item" => add_item_from_barcode(barcode).await.map(|_| format!("Added Item")),
        _ => Err(ServerFnError::new(format!("invalid action: {:?}", action))),
    }
}

#[component]
pub fn DBTool() -> impl IntoView {
    let action = create_server_action::<DbAction>();
    let output = action.value();
    let text = move || match output().transpose() {
        Ok(s) => s.unwrap_or_default(),
        Err(err) => format!("ERROR: {err}"),
    };

    view! {
        <h1>"DB Tool"</h1>
        <ActionForm action=action>
            <label for="barcode-input">"barcode: "</label>
            <input type="text" id="barcode-input" name="barcode"/>
            <br/>
            <input type="submit" name="action" value="request json"/>
            <input type="submit" name="action" value="request ItemData"/>
            <input type="submit" name="action" value="Add Item"/>
        </ActionForm>
        <br/>
        <textarea
            prop:value=text
            onmouseover="this.style.height = this.scrollHeight + 10 + \"px\""
            style:width="100%"
        />
    }
}
