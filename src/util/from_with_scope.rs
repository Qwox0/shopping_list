use leptos::*;

/// Like [`From`] but with a [`leptos::Scope`] argument.
pub trait FromWithScope<T>
where
    Self: Sized,
{
    fn from(cx: Scope, value: T) -> Self;
}
