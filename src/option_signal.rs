use leptos::{create_signal, ReadSignal, SignalSet, WriteSignal};
use std::ops::Deref;

pub fn create_option_signal<T>() -> (ReadSignal<Option<T>>, OptionWriteSignal<T>) {
    let (get, set) = create_signal(None);
    //(OptionReadSignal(get), OptionWriteSignal(set))
    (get, OptionWriteSignal(set))
}

pub struct OptionReadSignal<T: 'static>(ReadSignal<Option<T>>);

impl<T> Deref for OptionReadSignal<T> {
    type Target = ReadSignal<Option<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for OptionReadSignal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Copy for OptionReadSignal<T> {}

#[derive(Debug)]
pub struct OptionWriteSignal<T: 'static>(WriteSignal<Option<T>>);

impl<T> Clone for OptionWriteSignal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Copy for OptionWriteSignal<T> {}

impl<T> SignalSet for OptionWriteSignal<T> {
    type Value = T;

    fn set(&self, new_value: Self::Value) {
        self.0.set(Some(new_value))
    }

    fn try_set(&self, new_value: Self::Value) -> Option<Self::Value> {
        self.0.try_set(Some(new_value)).flatten()
    }
}

impl<T> OptionWriteSignal<T> {
    pub fn reset(&self) {
        self.0.set(None)
    }
}

impl<T> FnOnce<(T,)> for OptionWriteSignal<T> {
    type Output = ();

    extern "rust-call" fn call_once(self, args: (T,)) -> Self::Output {
        self.0.set(Some(args.0))
    }
}

impl<T> FnMut<(T,)> for OptionWriteSignal<T> {
    extern "rust-call" fn call_mut(&mut self, args: (T,)) -> Self::Output {
        self.0.set(Some(args.0))
    }
}

impl<T> Fn<(T,)> for OptionWriteSignal<T> {
    extern "rust-call" fn call(&self, args: (T,)) -> Self::Output {
        self.0.set(Some(args.0))
    }
}
