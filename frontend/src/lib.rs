mod connection_status;
mod language;
mod list;

use connection_status::*;
use language::*;
use leptos::*;
use list::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (lang, set_lang) = create_signal(cx, Language::English);
    let lang = create_local_resource(cx, lang, |lang| Dictionary::fetch(lang));
    // provide language text to all components (use init_dict!() and dict!())
    provide_context(cx, LangReader(lang));

    view! {
        cx,
        <header>
            <LanguageSelector set_lang/>
            <ConnectionStatus/>
        </header>
        <main>
            //<h1> "Shopping List" </h1>
            <h1> <span lang="en">"Shopping List"</span>  </h1>
            //<h1> <span lang="de">"Einkaufsliste"</span>  </h1>
            <ShoppingList/>
        </main>
    }
}

/*
/// A simple counter component.
///
/// You can use doc comments like this to document your component.
#[component]
pub fn SimpleCounter(
    cx: Scope,
    /// The starting value for the counter
    initial_value: i32,
    /// The change that should be applied each time the button is clicked.
    step: i32
) -> impl IntoView {
    let (value, set_value) = create_signal(cx, initial_value);

    view! { cx,
        <div>
            <button on:click=move |_| set_value(0)>"Clear"</button>
            <button on:click=move |_| set_value.update(|value| *value -= step)>"-1"</button>
            <span>"Value: " {move || value().to_string()} "!"</span>
            <button on:click=move |_| set_value.update(|value| *value += step)>"+1"</button>
        </div>
    }
}
*/
