use leptos::{
    create_rw_signal, create_signal, ReadSignal, RwSignal, SignalGet, SignalSet, SignalWith,
    WriteSignal,
};
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub struct OptionSignal<Sig>(Sig);

impl<T> OptionSignal<RwSignal<Option<T>>> {
    pub fn new() -> OptionSignal<RwSignal<Option<T>>> {
        OptionSignal(create_rw_signal(None))
    }
}

impl<Sig, T> OptionSignal<Sig>
where Sig: SignalSet<Value = Option<T>>
{
    pub fn wrap(sig: Sig) -> OptionSignal<Sig> {
        Self(sig)
    }
}

impl<Sig> Deref for OptionSignal<Sig> {
    type Target = Sig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Sig, T> FnOnce<()> for OptionSignal<Sig>
where Sig: SignalGet<Value = Option<T>>
{
    type Output = Option<T>;

    extern "rust-call" fn call_once(self, args: ()) -> Self::Output {
        self.0.get()
    }
}

impl<Sig, T> FnMut<()> for OptionSignal<Sig>
where Sig: SignalGet<Value = Option<T>>
{
    extern "rust-call" fn call_mut(&mut self, args: ()) -> Self::Output {
        self.0.get()
    }
}

impl<Sig, T> Fn<()> for OptionSignal<Sig>
where Sig: SignalGet<Value = Option<T>>
{
    extern "rust-call" fn call(&self, args: ()) -> Self::Output {
        self.0.get()
    }
}

impl<Sig, T> SignalSet for OptionSignal<Sig>
where Sig: SignalSet<Value = Option<T>>
{
    type Value = T;

    fn set(&self, new_value: Self::Value) {
        self.0.set(Some(new_value))
    }

    fn try_set(&self, new_value: Self::Value) -> Option<Self::Value> {
        self.0.try_set(Some(new_value)).flatten()
    }
}

impl<Sig, T> OptionSignal<Sig>
where Sig: SignalSet<Value = Option<T>>
{
    pub fn reset(&self) {
        self.0.set(None)
    }
}

impl<Sig, T> FnOnce<(T,)> for OptionSignal<Sig>
where Sig: SignalSet<Value = Option<T>>
{
    type Output = ();

    extern "rust-call" fn call_once(self, args: (T,)) -> Self::Output {
        self.0.set(Some(args.0))
    }
}

impl<Sig, T> FnMut<(T,)> for OptionSignal<Sig>
where Sig: SignalSet<Value = Option<T>>
{
    extern "rust-call" fn call_mut(&mut self, args: (T,)) -> Self::Output {
        self.0.set(Some(args.0))
    }
}

impl<Sig, T> Fn<(T,)> for OptionSignal<Sig>
where Sig: SignalSet<Value = Option<T>>
{
    extern "rust-call" fn call(&self, args: (T,)) -> Self::Output {
        self.0.set(Some(args.0))
    }
}
