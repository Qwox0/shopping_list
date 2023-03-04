use super::{dictionary::Dictionary, Language};
use leptos::*;

#[derive(Clone, Debug)]
pub struct LanguageContext {
    language: RwSignal<Language>,
    dictionary: Resource<Language, Dictionary>,
}

impl LanguageContext {
    pub fn new(cx: Scope, language: RwSignal<Language>) -> Self {
        LanguageContext {
            language,
            dictionary: create_resource_with_initial_value(
                // create_local_resource_with_initial_value(
                cx,
                language,
                move |lang| async move {
                    /* load_dictionary_action(cx, lang) .await .expect("got valid Dictionary") */
                    Dictionary::fetch(lang)
                        .await
                        .expect("able to fetch Dictionary")
                },
                Dictionary::try_from_language(language.get()).ok(),
            ),
        }
    }

    pub fn set_language(&self, cx: Scope, lang: Language) {
        crate::util::set_cookie(cx, "language", lang);
        self.language.set(lang)
    }

    pub fn get_word<F>(&self, cx: Scope, getter: F) -> String
    where
        F: Fn(&crate::language::dictionary::Dictionary) -> String,
    {
        self.dictionary
            .with(cx, getter)
            .unwrap_or("pending".to_string())
    }
}

/*
#[derive(Clone, Debug)]
struct LanguageContextProps {
    language: RwSignal<Language>,
    dictionary: Resource<Language, Dictionary>,
    initial_dict: Dictionary,
}

impl LanguageContextProps {
    pub fn new(cx: Scope, initial_language: Language) -> Self {
        let initial_dict = Dictionary::try_from_language(cx, initial_language).unwrap_or_default();
        let language = create_rw_signal(cx, initial_language);
        // create_resource increases first page load
        // create_local_resource loads imediately but overwrites the correct ssr'ed value
        let dictionary = create_resource_with_initial_value(
            //let dict = create_local_resource_with_initial_value(
            cx,
            language,
            move |lang| async move {
                /* load_dictionary_action(cx, lang) .await .expect("got valid Dictionary") */
                Dictionary::fetch(cx, lang)
                    .await
                    .expect("able to fetch Dictionary")
            },
            //None,
            Some(initial_dict.clone()), //-> thread 'actix-rt|system:0|arbiter:1' panicked at 'failed while trying to write to Resource serializer: TrySendError { kind: Disconnected }'
        );
        LanguageContextProps {
            language,
            dictionary,
            initial_dict,
        }
    }

    pub fn get_word<F, S>(&self, cx: Scope, getter: F) -> String
    where
        F: Fn(&crate::language::dictionary::Dictionary) -> S,
        S: Into<String>,
    {
        self.dictionary
            .with(cx, |s| getter(s))
            .unwrap_or(getter(&self.initial_dict))
            .into()
    }
}
*/
