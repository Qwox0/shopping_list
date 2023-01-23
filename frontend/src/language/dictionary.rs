use anyhow::Context;
use leptos::*;
use serde::Deserialize;

use super::Language;

macro_rules! init_dict {
    ( $vis:vis $dict_name:ident: $( $name:ident: $attr_type:ty ),* ) => {
        #[derive(Deserialize, Eq, Hash, Debug, PartialEq, Clone)]
        $vis struct $dict_name {
            $(pub $name: $attr_type),*
        }

        impl Default for $dict_name {
            fn default() -> Self {
                Self { $($name: <$attr_type>::default()),* }
            }
        }
    };
}

init_dict! { pub Dictionary:
    shopping_list: String,
    list_header: ListHeader,
    item: Item
}

init_dict! { ListHeader:
    item_name: String,
    amount: String
}

init_dict! { Item:
    edit: String,
    remove: String
}

impl Dictionary {
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
