use crate::barcode_scanner::BarcodeError;
use std::any;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    ServerFn(#[from] leptos::ServerFnErrorErr),

    #[cfg(feature = "ssr")]
    #[error(transparent)]
    DB(#[from] sqlx::Error),

    #[error("didn't find product")]
    DidntFindProduct,

    #[error("missing \"{}\" field on product", .0)]
    MissingProductField(&'static str),

    #[error("missing context: {}", .0)]
    MissingContext(&'static str),

    #[error(transparent)]
    BarcodeError(#[from] BarcodeError),
}

pub type Result<T> = core::result::Result<T, Error>;

impl Error {
    pub fn missing_ctx<Ctx>() -> Self {
        Self::MissingContext(any::type_name::<Ctx>())
    }
}

#[cfg(feature = "ssr")]
impl From<leptos::ServerFnError> for Error {
    fn from(err: leptos::ServerFnError) -> Self {
        Error::ServerFn(err.into())
    }
}

/*
#[component]
pub fn ErrorView(error: Error) -> impl IntoView {
    view! {
        <span class="error">
            { format!("{}", error) }
        </span>
    }
}

#[component]
pub fn ResultView<ResF, Ok, OkF, IV>(res: ResF, ok: OkF) -> impl IntoView
where
    ResF: Fn() -> Result<Ok> + 'static,
    OkF: FnOnce(Ok) -> IV,
    IV: IntoView + 'static,
{
    /*
    res()
        .map(ok)
        .map(IV::into_view)
        .unwrap_or_else(|error| view! { <ErrorView error/> })
        */
    res().map(ok).into_view()
}

#[component]
pub fn OptionView<Opt, T, F, IV>(opt: Opt, some: F) -> impl IntoView
where
    Opt: Fn() -> Option<T> + 'static,
    F: Fn(T) -> IV,
    IV: IntoView + 'static,
{
    opt().map(some).into_view()
}
*/
