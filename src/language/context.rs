use leptos::*;

use super::{
    dictionary::{load_dictionary_action, Dictionary},
    Language,
};

/*
#[derive(Clone)]
pub struct EmptyLanguageContext;
    */

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

    pub fn get_word<F>(&self, getter: F) -> String
    where
        F: Fn(&crate::language::dictionary::Dictionary) -> String,
    {
        self.0.with(|props| {
            props
                .as_ref()
                .expect("initial language was set for `LanguageContext`")
                .get_word(getter)
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
        let initial_dict = Dictionary::try_from_language(initial_language).unwrap_or_default();
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

    pub fn get_word<F, S>(&self, getter: F) -> String
    where
        F: Fn(&crate::language::dictionary::Dictionary) -> S,
        S: Into<String>,
    {
        self.dictionary
            .with(|s| getter(s))
            .unwrap_or(getter(&self.initial_dict))
            .into()
    }
}
