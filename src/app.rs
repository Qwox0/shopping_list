use crate::{
    header_bar::HeaderBar,
    img_popup::{ImgPopup, ImgPopupView},
    list::ListView,
    main_page::MainPage,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let popup = ImgPopup::default();
    provide_context(popup);

    view! {
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="description" content="Fullstack Rust Shopping List"/>
        <Stylesheet id="leptos" href="/pkg/shopping_list.css"/> // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Link rel="manifest" href="/pwa.webmanifest"/>

        <Script type_="text/javascript" src="/js/init_sw.js"/>

        <Title text="Shopping List"/>

        <ImgPopupView data=popup/>

        //<HeaderBar/>

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
        <Router>
            <main>
                <Routes>
                    <Route path="" view=MainPage/>
                </Routes>
            </main>
        </Router>
    }
}
