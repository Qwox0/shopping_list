use leptos::*;
use leptos_meta::*;

#[component]
pub fn SiteHead(cx: Scope) -> impl IntoView {
    view! { cx,
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="description" content="Fullstack Rust Shopping List"/>
        <Stylesheet id="leptos" href="/pkg/shopping_list.css"/> // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Link rel="manifest" href="/pwa.webmanifest"/>

        <Script type_="text/javascript" src="/js/init_sw.js"/>

        <Title text="Shopping List"/>
    }
}
