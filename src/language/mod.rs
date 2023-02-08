pub mod context;
pub mod dictionary;

use self::context::LanguageContextProps;
use self::dictionary::Dictionary;
use self::text_macro::text;
use anyhow::anyhow;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::hash::Hash;

//pub const LANGUAGES: [Language; 2] = [Language::English, Language::German];
pub const SITE_DEFAULT_LANGUAGE: Language = Language::English;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum Language {
    English,
    German,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl TryFrom<String> for Language {
    type Error = anyhow::Error;

    fn try_from(str: String) -> Result<Self, Self::Error> {
        match str.as_str() {
            "English" => Ok(Language::English),
            "Deutsch" => Ok(Language::German),
            "en" => Ok(Language::English),
            "de" => Ok(Language::German),
            s => Err(anyhow!("Invalid language String: {}", s)),
        }
    }
}

impl TryFrom<Option<String>> for Language {
    type Error = anyhow::Error;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        match value {
            Some(str) => Language::try_from(str),
            None => Err(anyhow!("`None` provided as language")),
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::English => write!(f, "English"),
            Language::German => write!(f, "Deutsch"),
        }
    }
}

impl Language {
    pub fn short(&self) -> String {
        match self {
            Language::English => "en",
            Language::German => "de",
        }
        .to_owned()
    }

    pub fn from_cookie(cx: Scope) -> Language {
        crate::util::get_cookie(cx, "language")
            .map(|s| Language::try_from(s).ok())
            .flatten()
            .unwrap_or(SITE_DEFAULT_LANGUAGE)
    }
}

/// prevent multiple definitions of text
pub(crate) mod text_macro {
    #[allow(unused_imports)]
    use crate::language::{dictionary::Dictionary, LanguageContextProps};
    #[allow(unused_imports)]
    use leptos::{view, Resource, Scope};
    /// get Text in the currently selected language
    /// For displaying text inside the [view] macro, use the [Text] component instead!
    ///
    /// ( $cx, $getter ) => { ... } -> (|| -> String)
    /// ( $lang_context -> $getter ) => { ... } -> (|| -> String)
    ///
    /// # Types
    ///
    /// $cx: [Scope]
    /// $lang_context: [LanguageContext]
    /// $getter: FnOnce(&Dictionary) -> &T
    /// [Dictionary]
    macro_rules! text {
        ( $cx:ident, $getter:expr ) => {{
            let cx: ::leptos::Scope = $cx;
            let lang_context = use_context::<crate::language::context::LanguageContext>(cx)
                .expect("`LanguageContext` is available");
            text!(lang_context -> $getter)
        }};
        ( $lang_context:ident -> $getter:expr ) => {{
            let lang_context: crate::language::context::LanguageContext = $lang_context;
            move || { format!("{}", lang_context.get_word($getter)) }
        }};
    }
    pub(crate) use text;
}

/// write Text in the currently selected language
#[component]
pub fn Text<F>(cx: Scope, getter: F) -> impl IntoView
where
    F: Fn(&Dictionary) -> String + 'static,
{
    view! { cx,
        <span> { text!(cx, &getter) } </span>
    }
}

