use leptos::{
    create_rw_signal, logging, spawn_local, RwSignal, ServerFnError, SignalGet, SignalGetUntracked,
    SignalSet, SignalUpdate, SignalWith,
};
use std::future::Future;

/// Works like a [`RwSignal`] but executes an async function everytime the
/// signal changes.
///
/// This is intended to be used to send data to the server to store it.
///
/// ```rust
/// let count = ServerSyncSignal::new(count, move |next| set_count(id, next));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ServerSyncSignal<T: 'static, SF> {
    client_sig: RwSignal<T>,
    sync_fn: SF,
}

impl<T, SF> ServerSyncSignal<T, SF> {
    pub fn new<F>(value: T, sync_fn: SF) -> Self
    where
        SF: Fn(T) -> F + 'static,
        F: Future<Output = Result<(), ServerFnError>>,
        RwSignal<T>: SignalGetUntracked<Value = T>,
        T: Clone,
        Self: Clone,
    {
        ServerSyncSignal { client_sig: create_rw_signal(value), sync_fn }
    }
}

impl<T, SF> SignalGet for ServerSyncSignal<T, SF>
where RwSignal<T>: SignalGet<Value = T>
{
    type Value = T;

    fn get(&self) -> Self::Value {
        self.client_sig.get()
    }

    fn try_get(&self) -> Option<Self::Value> {
        self.client_sig.try_get()
    }
}

impl<T, SF> FnOnce<()> for ServerSyncSignal<T, SF>
where T: Clone
{
    type Output = T;

    #[inline(always)]
    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl<T, SF> FnMut<()> for ServerSyncSignal<T, SF>
where T: Clone
{
    #[inline(always)]
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl<T, SF> Fn<()> for ServerSyncSignal<T, SF>
where T: Clone
{
    #[inline(always)]
    extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl<T, SF> SignalWith for ServerSyncSignal<T, SF>
where RwSignal<T>: SignalWith<Value = T>
{
    type Value = T;

    fn with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> O {
        self.client_sig.with(f)
    }

    fn try_with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        self.client_sig.try_with(f)
    }
}

impl<T, SF, F> SignalSet for ServerSyncSignal<T, SF>
where
    SF: Fn(T) -> F + 'static,
    F: Future<Output = Result<(), ServerFnError>>,
    RwSignal<T>: SignalGetUntracked<Value = T>,
    T: Clone,
    Self: Clone,
{
    type Value = T;

    fn set(&self, next: Self::Value) {
        let cur = self.client_sig.get_untracked();
        self.client_sig.set(next.clone());
        let s = self.clone();
        spawn_local(async move {
            if let Err(err) = (s.sync_fn)(next).await {
                logging::error!("Error while calling server function: {}", err);
                s.client_sig.set(cur);
            }
        });
    }

    fn try_set(&self, new_value: Self::Value) -> Option<Self::Value> {
        todo!()
    }
}

impl<T, SF, F> FnOnce<(T,)> for ServerSyncSignal<T, SF>
where
    SF: Fn(T) -> F + 'static,
    F: Future<Output = Result<(), ServerFnError>>,
    RwSignal<T>: SignalGetUntracked<Value = T>,
    T: Clone,
    Self: Clone,
{
    type Output = ();

    #[inline(always)]
    extern "rust-call" fn call_once(self, args: (T,)) -> Self::Output {
        self.set(args.0)
    }
}

impl<T, SF, F> FnMut<(T,)> for ServerSyncSignal<T, SF>
where
    SF: Fn(T) -> F + 'static,
    F: Future<Output = Result<(), ServerFnError>>,
    RwSignal<T>: SignalGetUntracked<Value = T>,
    T: Clone,
    Self: Clone,
{
    #[inline(always)]
    extern "rust-call" fn call_mut(&mut self, args: (T,)) -> Self::Output {
        self.set(args.0)
    }
}

impl<T, SF, F> Fn<(T,)> for ServerSyncSignal<T, SF>
where
    SF: Fn(T) -> F + 'static,
    F: Future<Output = Result<(), ServerFnError>>,
    RwSignal<T>: SignalGetUntracked<Value = T>,
    T: Clone,
    Self: Clone,
{
    #[inline(always)]
    extern "rust-call" fn call(&self, args: (T,)) -> Self::Output {
        self.set(args.0)
    }
}

impl<T, SF, F> SignalUpdate for ServerSyncSignal<T, SF>
where
    SF: Fn(T) -> F + 'static,
    F: Future<Output = Result<(), ServerFnError>>,
    RwSignal<T>: SignalGetUntracked<Value = T>,
    T: Clone,
    Self: Clone,
{
    type Value = T;

    fn update(&self, f: impl FnOnce(&mut Self::Value)) {
        let cur = self.client_sig.get_untracked();
        self.client_sig.update(f);
        let next = self.client_sig.get_untracked();
        let s = self.clone();
        spawn_local(async move {
            if let Err(err) = (s.sync_fn)(next).await {
                logging::error!("Error while calling server function: {}", err);
                s.client_sig.set(cur);
            }
        });
    }

    fn try_update<O>(&self, f: impl FnOnce(&mut Self::Value) -> O) -> Option<O> {
        todo!()
    }
}
