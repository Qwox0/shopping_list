use leptos::logging;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Language {
    #[default]
    English,
}

impl Language {
    pub fn new(lang: impl AsRef<str>) -> Option<Language> {
        Self::try_from(lang.as_ref())
            .inspect_err(|e| logging::warn!("unknown language: {:?}", e))
            .ok()
    }
}

impl<'a> TryFrom<&'a str> for Language {
    type Error = &'a str;

    fn try_from(lang: &'a str) -> Result<Self, Self::Error> {
        match lang {
            "en" => Ok(Language::English),
            l if l.starts_with("en-") => Ok(Language::English),
            // "en-US" => Ok(Language::English), // TODO
            lang => Err(lang),
        }
    }
}
