use thiserror::Error;

#[derive(Debug, Error)]
pub enum LangError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
