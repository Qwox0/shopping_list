use super::Language;
#[allow(unused_imports)]
use anyhow::Context;
use gloo_net::http::Request;
use leptos::*;
use serde::{Deserialize, Serialize};

/*
#[server(LoadDictionary, "/api")]
pub async fn load_dictionary_action(
    cx: Scope,
    lang: Language,
) -> Result<Dictionary, ServerFnError> {
    crate::util::set_cookie(cx, "language", lang);
    Dictionary::try_from_language(lang)
        .map_err(|err| ServerFnError::ServerError(format!("failed to load dict: {}", err)))
}
*/

macro_rules! init_dict {
    ( $dict_name:ident -> $( $name:ident: $attr_type:ty, )* ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $dict_name {
            $(pub $name: $attr_type),*
        }

        impl Default for $dict_name {
            fn default() -> Self {
                Self { $($name: <$attr_type>::default()),* }
            }
        }
    };
    ( $dict_name:ident -> $( $name:ident: $attr_type:ty ),* ) => {
        init_dict! { $dict_name -> $( $name: $attr_type, )* }
    };
}

init_dict! { Dictionary ->
    shopping_list: String,
    default_: String,
    list_header: ListHeaderDict,
    item: ItemDict
}

init_dict! { ListHeaderDict ->
    item_name: String,
    amount: String,
}

init_dict! { ItemDict ->
    edit: String,
    remove: String,
    remove_question_1: String, // first part
    remove_question_2: String, // second part
}

impl Dictionary {
    pub fn try_from_toml(toml_string: impl Into<String>) -> anyhow::Result<Self> {
        toml::from_str::<Dictionary>(&toml_string.into()).context("failed to parse Toml String")
    }

    /// trys to read the language file (see "target/site/language/{some_lang_in_short}.toml") for
    /// the given language and interprets it.
    ///
    /// This will only work during ssr. To fetch a Dictionary from the client use
    /// [`Dictionary::fetch`]
    pub fn try_from_language(language: Language) -> anyhow::Result<Self> {
        if leptos_dom::is_browser() {
            anyhow::bail!("language files are only available on the server!");
        }
        let path = format!("target/site/language/{}.toml", language.short());
        let toml_string =
            std::fs::read_to_string(&path).context(format!("failed to read file: {:?}", path))?;
        Dictionary::try_from_toml(toml_string)
    }

    /// Fetch a Dictionary. On the client this will create a request. On the server this is
    /// equivalent to [Dictionary::try_from_language].
    ///
    /// To get a Dictionary nonsynchronously use [Dictionary::try_from_language].
    pub async fn fetch(language: Language) -> anyhow::Result<Self> {
        if leptos_dom::is_server() {
            Dictionary::try_from_language(language)
        } else {
            let toml_string = Request::get(&format!("/language/{}.toml", language.short()))
                .send()
                .await
                .context("Language Request failed")?
                .text()
                .await
                .context("Failed reading response as String")?;
            Dictionary::try_from_toml(toml_string)
        }
    }
}
