pub mod dictionary;
mod error;
pub mod text;

use self::dictionary::Dictionary;
use self::text_macro::text;
use leptos::*;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Language {
    English,
    German,
}

impl Language {
    pub fn short(&self) -> String {
        match self {
            Language::English => "en",
            Language::German => "de",
        }
        .to_owned()
    }
}

#[derive(Clone)]
pub struct LangReader {
    pub language: RwSignal<Language>,
    pub dictionary: Resource<Language, Dictionary>,
}

impl LangReader {
    pub fn new(cx: Scope) -> Self {
        let language = create_rw_signal(cx, Language::English);
        LangReader {
            language,
            dictionary: create_local_resource(cx, language, |lang| Dictionary::fetch(lang)),
        }
    }
}

/// prevent multiple definitions of text
pub(crate) mod text_macro {
    /// get Text in the currently selected language
    /// For displaying text inside the [view] macro, use the [Text] component instead!
    ///
    /// Parameters:
    /// cx: Scope
    /// getter: Fn(&[Dictionary]) -> &T,
    macro_rules! text {
        ( $cx:ident, $getter:expr) => {{
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
                    use_context::<crate::language::LangReader>($cx)
                        .expect("`LangReader` context is available")
                        .dictionary
                        .with(|dict: &crate::language::dictionary::Dictionary| type_hint(
                            dict, $getter
                        ))
                        .unwrap_or(type_hint(
                            &crate::language::dictionary::Dictionary::pending(),
                            $getter
                        ))
                )
            }
        }};
    }
    pub(crate) use text;
}

/// write Text in the currently selected language
#[component]
pub fn Text<F, T>(cx: Scope, getter: F) -> impl IntoView
where
    F: Fn(&Dictionary) -> &T + Copy + 'static,
    T: std::fmt::Display + Clone,
{
    let language = use_context::<crate::language::LangReader>(cx)
        .expect("`LangReader` context is available")
        .language;
    view! { cx,
        <span lang={move || language.with(|l| l.short())}>
            { text!(cx, getter) }
        </span>
    }
}

#[component]
pub fn LanguageSelector(cx: Scope, set_lang: WriteSignal<Language>) -> impl IntoView {
    view! {cx,
        <a href="#"
            //on:click=move |_| set_lang.update(|lm| lm.change_language(Language::English).expect("able to change language"))
            //on:click=move |_| set_langs.update(|lm| lm.current_language = Language::English)
            on:click=move |_| set_lang(Language::English)
        >"English"</a>
        <a href="#"
            on:click=move |_| set_lang(Language::German)
        >"Deutsch"</a>
    }
}

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
