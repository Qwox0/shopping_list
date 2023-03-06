use crate::{
    state::{app_state::AppState, language::Language},
    view::{
        connection_status::*,
        head::{SiteHead, SiteHeadProps},
        list::{List, ListProps},
        text::{Text, TextProps},
    },
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub fn app(cx: Scope) -> impl IntoView {
    // global state
    let render_state: &'static AppState = Box::leak(Box::new(AppState::new(cx)));
    provide_context::<&'static AppState>(cx, render_state);

    view! { cx, <App />}
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    // provide language text to all components (use <Text /> or text!())
    // the initial language still has to be set!
    //let language = create_rw_signal(cx, Language::from_cookies(cx).unwrap_or_default());
    //provide_context(cx, LanguageContext::new(cx, language));

    let language = move || AppState::from_context(cx).language.get_lang();

    create_effect(cx, move |_| {
        log!("test:{:?}", AppState::from_context(cx).item_list.read(cx))
    });

    view! { cx,
        <SiteHead />
        <Router>
            <nav>
                <A href="en">"English"</A>
                <A href="de">"Deutsch"</A>
            </nav>
            <main>
                <Routes>
                    <Route path="/de" view=|cx| view! { cx, <HomePage lang=Language::German/> } ssr=SsrMode::Async /> // or InOrder; OutOfOrder shows Loading
                    <Route path="/en" view=|cx| view! { cx, <HomePage lang=Language::English/> } ssr=SsrMode::Async />

                    <Route path="" view=move |cx| view! { cx, <Redirect path=language().short() /> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope, lang: Language) -> impl IntoView {
    AppState::from_context(cx).language.set_lang(cx, lang);

    //log!("path: {:?}", use_location(cx).pathname.get());

    view! { cx,
        <header>
            //<LanguageSelector/>
            <ConnectionStatus/>
        </header>
        <main>
            <h1> <Text getter=|d| d.shopping_list.clone()/> </h1>
            <List/>
        </main>
        <Test/>
    }
}

/*
#[server(GetItemList2, "/api")]
pub async fn get_item_list2(cx: Scope, count: usize) -> Result<Vec<ItemSerialized>, ServerFnError> {
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

#[server(GetItemList, "/api", "Cbor")] // Cbor: + smaller + allows for enums with non String values - needs wasm even in forms
pub async fn get_item_list(cx: Scope, msg: ListMsg) -> Result<Vec<ItemSerialized>, ServerFnError> {
    log!("msg: {:?}", msg);
    let list = crate::db::ItemsDbTable::new()
        .await
        .expect("got db")
        .get_all(cx)
        .await
        .expect("got items");
    let list = match msg {
        ListMsg::Get(count) => list.into_iter().take(count).collect(),
        ListMsg::GetAll => list,
    };
    log!("request list: {:?}", list);
    std::thread::sleep(std::time::Duration::from_millis(1000));
    Ok(list)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListMsg {
    Get(usize), // serde_urlencoded only supports Strings
    GetAll,
}

#[derive(Debug, Clone, Copy)]
pub struct ItemList {
    list: Resource<ListMsg, Result<Vec<ItemSerialized>, ServerFnError>>,
    message: RwSignal<ListMsg>,
}

impl ItemList {
    pub fn new(cx: Scope, init_msg: ListMsg) -> Self {
        let message = create_rw_signal(cx, init_msg);
        let list = create_resource(cx, message, move |msg| get_item_list(cx, msg));
        ItemList { list, message }
    }

    pub fn read(&self, cx: Scope) -> Option<Result<Vec<ItemSerialized>, ServerFnError>> {
        self.list.read(cx)
    }

    pub fn set_msg(&self, msg: ListMsg) {
        self.message.set(msg)
    }
}

*/
#[component]
fn Test(cx: Scope) -> impl IntoView {
    /*
        let (count, set_count) = create_signal::<usize>(cx, 0);
        let list1 = create_resource(cx, count, move |count| get_item_list2(cx, count));

        let (msg, set_msg) = create_signal(cx, ListMsg::Get(0));
        let list2 = create_resource(cx, msg, move |msg| get_item_list(cx, msg));

        let list3 = ItemList::new(cx, ListMsg::Get(0));

        macro_rules! get_fallback {
            ( $list: expr ) => {
                move || match $list.read(cx) {
                    Some(l) => view! { cx, <p>{ move || format!("Some({:?})", l) }</p> },
                    None => view! { cx, <p>"Loading (Suspense Fallback)..."</p> },
                }
            };
        }

        view! { cx,
            <div>
                <input type="number"
                    //on:change=move |e| set_count(event_target_value(&e).parse::<usize>().unwrap_or(1))
                    //on:change=move |e| set_msg(ListMsg::Get(event_target_value(&e).parse::<usize>().unwrap_or(1)))
                    //on:change=move |e| list.set_msg(ListMsg::Get(event_target_value(&e).parse::<usize>().unwrap_or(1)))
                    on:change=move |e| {
                        let new_value = event_target_value(&e).parse::<usize>().unwrap_or(1);
                        set_count(new_value);
                        set_msg(ListMsg::Get(new_value));
                        list3.set_msg(ListMsg::Get(new_value));
                    }
                />
                /*
                <Suspense fallback=move || view! { cx, <p>"Loading (Suspense Fallback)..."</p> }>
                    <p>{ move || format!("{:?}", list.read(cx)) }</p>
                </Suspense>
                <Suspense fallback=get_fallback!(list1)>
                    <p>{ move || format!("{:?}", list1.read(cx)) }</p>
                </Suspense>
                <Suspense fallback=get_fallback!(list2)>
                    <p>{ move || format!("{:?}", list2.read(cx)) }</p>
                </Suspense>
                */
                <Suspense fallback=get_fallback!(list3)>
                    <p>{ move || format!("{:?}", list3.read(cx)) }</p>
                </Suspense>
            </div>
        }
    */
    view! { cx,
        <p>"Test"</p>
    }
}
