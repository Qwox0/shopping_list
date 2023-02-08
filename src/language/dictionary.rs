use super::Language;
use crate::util::set_cookie;
use leptos::*;
use serde::{Deserialize, Serialize};

#[server(LoadDictionary, "/api")]
pub async fn load_dictionary_action(
    cx: Scope,
    lang: Language,
) -> Result<Dictionary, ServerFnError> {
    log!("hey");
    set_cookie(cx, "language", lang);
    //std::thread::sleep(std::time::Duration::from_millis(1000));
    get_dict(lang).ok_or(ServerFnError::ServerError(format!("failed to load dict")))
}

/*
fn test_load_dictionary(lang: Language) -> anyhow::Result<Dictionary> {
    log!("load dictionary: {:?}", lang);
    let path = format!("target/site/language/{}.toml", lang.short());
    let content =
        std::fs::read_to_string(&path).with_context(|| format!("failed to read file: {}", path))?;
    toml::from_str::<Dictionary>(&content)
        .with_context(|| format!("failed to parse language: {}", lang))
}
*/

pub fn get_dict(lang: Language) -> Option<Dictionary> {
    #[cfg(not(feature = "ssr"))]
    return None;

    let path = format!("target/site/language/{}.toml", lang.short());
    let content = std::fs::read_to_string(&path).ok()?;
    toml::from_str::<Dictionary>(&content).ok()
}

macro_rules! init_dict {
    ( $dict_name:ident: $( $name:ident: $attr_type:ty ),* ) => {
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
}

init_dict! { Dictionary:
    shopping_list: String,
    default: String,
    list_header: ListHeaderDict,
    item: ItemDict
}

init_dict! { ListHeaderDict:
    item_name: String,
    amount: String
}

init_dict! { ItemDict:
    edit: String,
    remove: String
}

impl Dictionary {
    pub fn get<'a, T, F>(&self, getter: F) -> String
    where
        F: FnOnce(&'a crate::language::dictionary::Dictionary) -> T,
        T: Into<&'a String>,
    {
        getter(self).into().clone()
    }
    /*
    pub async fn fetch(lang: Language) -> Self {
        //log!("fetch Language: {:?}", lang);
        async {
            let path = format!("/language/{}.toml", lang.short());
            let content = reqwasm::http::Request::get(&path)
                .send()
                .await
                .with_context(|| format!("Failed to Request: {:?}", path))?
                .text()
                .await
                .context("Failed to get context")?;
            toml::from_str::<Dictionary>(&content).context("Failed to parse text")
        }
        .await
        .expect("no lang fetch error")
    }
    */
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
