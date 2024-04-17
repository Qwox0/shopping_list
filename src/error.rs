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

pub trait ErrInto<T> {
    fn err_into(self) -> T;
}

impl<T, E: Into<E2>, E2> ErrInto<core::result::Result<T, E2>> for core::result::Result<T, E> {
    fn err_into(self) -> core::result::Result<T, E2> {
        self.map_err(Into::into)
    }
}
