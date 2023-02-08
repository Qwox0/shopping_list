pub mod dictionary;

use self::dictionary::{get_dict, load_dictionary_action, Dictionary};
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

#[derive(Clone)]
pub struct EmptyLanguageContext;

//TODO: non pub content
#[derive(Clone, Debug)]
pub struct LanguageContext(pub RwSignal<Option<LanguageContextProps>>);
//pub struct LanguageContext<T>(RwSignal<T>);

impl LanguageContext {
    pub fn new_empty(cx: Scope) -> Self {
        LanguageContext(create_rw_signal(cx, None))
    }

    pub fn set_language(&self, cx: Scope, lang: Language) {
        if let Some(a) = self.0.get() {
            a.language.set(lang);
        } else {
            self.set_initial_language(cx, lang)
        }
    }

    pub fn set_initial_language(&self, cx: Scope, initial_language: Language) {
        self.0.update(|option| {
            //let _ = option.insert(LanguageContextProps::new(cx, initial_language));
            *option = Some(LanguageContextProps::new(cx, initial_language));
        });
    }

    pub fn get_word<T, F>(&self, getter: F) -> String
    where
        F: FnOnce(&crate::language::dictionary::Dictionary) -> T,
        T: Into<String>,
    {
        self.0.with(|option| {
            let props = option
                .as_ref()
                .expect("initial language was set for `LanguageContext`");
            props
                .dictionary
                .with(|dict| dict.get(getter))
                .unwrap_or(props.initial_dict.get(getter))
        })
    }
}

//TODO: non pub
#[derive(Clone, Debug)]
pub struct LanguageContextProps {
    pub language: RwSignal<Language>,
    pub dictionary: Resource<Language, Dictionary>,
    pub initial_dict: Dictionary,
}

impl LanguageContextProps {
    pub fn new(cx: Scope, initial_language: Language) -> Self {
        let initial_dict = get_dict(initial_language).unwrap_or_default();
        let language = create_rw_signal(cx, initial_language);
        // create_resource increases first page load
        // create_local_resource loads imediately but overwrites the correct ssr'ed value
        let dictionary = create_resource_with_initial_value(
            //let dict = create_local_resource_with_initial_value(
            cx,
            //language,
            language,
            move |lang| async move {
                load_dictionary_action(cx, lang)
                    .await
                    .expect("got valid Dictionary")
            },
            None,
            //Some(initial_value.clone()), //-> thread 'actix-rt|system:0|arbiter:1' panicked at 'failed while trying to write to Resource serializer: TrySendError { kind: Disconnected }'
        );
        LanguageContextProps {
            language,
            dictionary,
            initial_dict,
        }
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
            let lang_context = use_context::<crate::language::LanguageContext>(cx)
                .expect("`LanguageContext` is available");
            text!(lang_context -> $getter)
        }};
        ( $lang_context:ident -> $getter:expr ) => {{
            let lang_context: crate::language::LanguageContext  = $lang_context;
            move || { format!("{}", lang_context.get_word($getter)) }
        }};
    }
    pub(crate) use text;
}

/// write Text in the currently selected language
#[component]
pub fn Text<F, T>(cx: Scope, getter: F) -> impl IntoView
where
    F: FnOnce(&Dictionary) -> T,
    T: Into<String>,
{
    view! { cx,
        <span> { text!(cx, getter) } </span>
    }
}

/*
#[component]
pub fn LanguageSelector(cx: Scope, set_lang: WriteSignal<Language>) -> impl IntoView {
    let options: Vec<_> = LANGUAGES
        .iter()
        .map(|a| {
            view! {cx,
                <option>{a.to_string()}</option>
            }
        })
        .collect();

    view! {cx,
        <select
            on:change=move |e| set_lang(Language::try_from(event_target_value(&e)).expect("valid language String"))
        >
            {options}
        </select>
    }
}
*/

/* -------------------------------------------------------------------------------------------------------------------------
* replaced with Text component!
/// Init multilanguage support with `let lang = init_dict!(cx);`.
/// use Dictionary in `view!()` with `{ dict!(lang, |d| d.<some_text_block>.clone()) }`.
macro_rules! init_dict {
    ($cx:ident) => {{
        use_context::<crate::language::LangReader>($cx)
            .expect("`LangReader` context is available")
            .dictionary
    }};
}
pub(crate) use init_dict;

/// use Dictionary in `view!()` with `{ dict!(lang, |d| d.<some_text_block>.clone()) }`.
///
/// First init multilanguage support with `let lang = init_dict!(cx);`.
macro_rules! dict {
    ( $lang_reader:ident, $getter:expr) => {{
        #[inline(always)]
        fn type_hint<T, F>(dict: &crate::language::dictionary::Dictionary, getter: F) -> T
        where
            F: Fn(&crate::language::dictionary::Dictionary) -> &T,
            T: Clone,
        {
            getter(dict).clone()
        }

        move || {
            format!(
                "{}",
                $lang_reader
                    .with(|dict: &crate::language::dictionary::Dictionary| type_hint(dict, $getter))
                    .unwrap_or(type_hint(&crate::language::dictionary::Dictionary::pending(), $getter))
            )
        }
        //.unwrap_or(Default::default()))
    }};
}

pub(crate) use dict;
*/

/* -------------------------------------------------------------------------------------------------------------------------
/// idea: cache already loaded Dictionaries in a HashMap
#[derive(Debug, PartialEq, Clone)]
pub struct LanguageManager {
    current_language: Language,
    list: collections::HashMap<Language, Dictionary>,
}

impl LanguageManager {
    pub fn new() -> Self {
        LanguageManager {
            current_language: Language::English,
            list: HashMap::new(),
        }
    }

    fn cache_lang(&mut self, lang: Language, text: Dictionary) {
        self.list.insert(lang, text);
    }

    /*
    pub async fn change_language(&mut self, lang: Language) -> anyhow::Result<()> {
        if !self.list.contains_key(&lang) {
            self.list.insert(
                lang.clone(),
                Dictionary::fetch(lang.clone()).await, //.with_context(|| format!("Failed to get text for: {:?}", lang))?,
            );
        }
        self.current_language = lang;
        Ok(())
    }

    pub async fn get_text(&mut self) -> Dictionary {
        todo!();

        //let lang = self.current_language.as_ref().unwrap_or(&DEFAULT_LANGUAGE);
        //self.list.get(lang).expect("contains lang")
    }
    */
}

pub async fn test(
    langs: ReadSignal<LanguageManager>,
    set_langs: WriteSignal<LanguageManager>,
) -> Dictionary {
    let lang = langs.with(|lm| lm.current_language.clone());
    if langs.with(|lm| lm.list.contains_key(&lang)) {
        langs
            .with(|lm| lm.list.get(&lang).map(|x| x.clone()))
            .unwrap()
        //langs().list.get(&lang).unwrap().clone()
    } else {
        let text = Dictionary::fetch(&lang).await;
        set_langs.update(|lm| lm.cache_lang(lang, text.clone()));

        text
    }
}
*/

/* -------------------------------------------------------------------------------------------------------------------------
/// idea: include language files in wasm binary
struct LanguageFile {

    lang: Language,
    content: &'static str,
}

impl LanguageFile {
    pub const fn new(lang: Language, content: &'static str) -> Self {
        LanguageFile { lang, content }
    }
}

const LANGUAGE_FILES: [LanguageFile; 2] = [
    LanguageFile::new(Language::English, include_str!("../../language/en.toml")),
    LanguageFile::new(Language::German, include_str!("../../language/de.toml")),
];
*/

/* -------------------------------------------------------------------------------------------------------------------------
* old LangReader
/// this extra type is required! `Resource<Language, Dictionary>` doesn't work!
#[derive(Debug, Clone)]
//pub struct Lang(Resource<Language, Dictionary>);
pub enum LangReader {
    Lang(Resource<LanguageManager, Dictionary>),
    Pending,
}
pub struct LangReader2(Option<Resource<LanguageManager, Dictionary>>);
impl LangReader2 {
    pub fn pending() -> Self {
        LangReader2(None)
    }
    pub fn get(self) -> Dictionary {
        self.0
            .map(|res| res.read())
            .flatten()
            .unwrap_or(Dictionary::pending())
    }
}

impl LangReader {
    pub fn get(self) -> Dictionary {
        match self {
            LangReader::Lang(res) => res.read(),
            LangReader::Pending => None,
        }
        .unwrap_or(Dictionary::pending())
    }
}
*/
