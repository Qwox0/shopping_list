mod connection_status;
mod language;
mod list;
mod state;

use connection_status::*;
use language::*;
use leptos::*;
use list::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let lang_reader = LangReader::new(cx);
    // provide language text to all components (use init_dict!() and dict!())
    provide_context(cx, lang_reader.clone());

    let set_lang = lang_reader.language.write_only();

    view! { cx,
        <header>
            <LanguageSelector set_lang/>
            <ConnectionStatus/>
        </header>
        <main>
            <h1> <Text getter=|d| &d.shopping_list/> </h1>
            <ShoppingList/>
        </main>
    }
}
