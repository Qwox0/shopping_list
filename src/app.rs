use crate::{
    barcode_scanner::BarcodeScanner, camera::CameraService, main_page::MainPage,
    option_signal::create_option_signal,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    provide_context(CameraService::new());

    #[cfg(feature = "ssr")]
    provide_context(crate::db::DB::new().expect("could connect to DB"));

    view! {
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="description" content="Fullstack Rust Shopping List"/>
        <Stylesheet id="leptos" href="/pkg/shopping_list.css"/> // id=leptos means cargo-leptos will hot-reload this stylesheet
        // <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
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
                    <Route path="/camtest" view=CameraTest/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn CameraTest() -> impl IntoView {
    let (barcode, set_barcode) = create_option_signal();

    create_effect(move |_| {
        logging::error!("{:?}", barcode());
        window().alert_with_message(&format!("{:?}", barcode()));
    });

    let inner_view = move || match barcode() {
        None => view! { <BarcodeScanner set_barcode/> }.into_view(),
        Some(Ok(barcode)) => {
            let msg = format!("Found barcode: {:?}", barcode);
            logging::log!("{msg}");
            view! { <p>{ msg }</p> }.into_view()
        },
        Some(Err(e)) => {
            view! { <p>{ format!("Error while scanning for barcodes: {}", e) }</p> }.into_view()
        },
    };

    view! {
        <h1>CameraTest</h1>
        { inner_view }
    }
}
