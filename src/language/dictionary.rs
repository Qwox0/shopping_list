use super::Language;
use crate::util::set_cookie;
use anyhow::Context;
use leptos::*;
use serde::{Deserialize, Serialize};

#[server(LoadDictionary, "/api")]
pub async fn load_dictionary_action(
    cx: Scope,
    lang: Language,
) -> Result<Dictionary, ServerFnError> {
    set_cookie(cx, "language", lang);
    Dictionary::try_from_language(lang)
        .map_err(|err| ServerFnError::ServerError(format!("failed to load dict: {}", err)))
}

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
    /// trys to read the language file (see "target/site/language/{some_lang_in_short}.toml") for
    /// the given language and interprets it
    pub fn try_from_language(language: Language) -> anyhow::Result<Dictionary> {
        #[cfg(not(feature = "ssr"))]
        anyhow::bail!("language files are only available on the server!");

        let path = format!("target/site/language/{}.toml", language.short());
        let content =
            std::fs::read_to_string(&path).context(format!("failed to read file: {:?}", path))?;
        Ok(toml::from_str::<Dictionary>(&content).unwrap())
        //.context(format!("failed to parse Dictionary from: {:?}", path))
    }
}

/* ------------------------------ old pending values (now the default values are used (String -> ""))
trait HasPendingValue {
    fn get_pending_value() -> Self;
}

macro_rules! init_pending_value {
    ($type:ty, $pending_value:expr) => {
        impl HasPendingValue for $type {
            fn get_pending_value() -> Self {
                $pending_value
            }
        }
    };
}

//init_pending_value!(String, "pending ...".to_string());
init_pending_value!(String, "".to_string());
init_pending_value!(i32, 0);


macro_rules! init_dict {
    ( $dict_name:ident, $( $name:ident: $attr_type:ty ),* ) => {
        #[derive(Deserialize, Eq, Hash, Debug, PartialEq, Clone)]
        pub struct $dict_name {
            $(pub $name: $attr_type),*
        }

        impl HasPendingValue for $dict_name {
            fn get_pending_value() -> Self {
                Self {
                    $($name: <$attr_type>::get_pending_value()),*
                }
            }
        }
    };
}
*/
