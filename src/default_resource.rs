use leptos::{
    create_local_resource, create_local_resource_with_initial_value, create_resource,
    create_resource_with_initial_value, Resource, Serializable, SignalGet, SignalSet, SignalUpdate,
    SignalWith,
};
use std::future::Future;

#[derive(Debug)]
pub struct DefaultResource<S: 'static, T: 'static> {
    res: Resource<S, T>,
    default: fn() -> T,
}

impl<S: 'static, T: 'static> Clone for DefaultResource<S, T> {
    fn clone(&self) -> Self {
        Self { res: self.res.clone(), default: self.default.clone() }
    }
}

impl<S: 'static, T: 'static> Copy for DefaultResource<S, T> {}

impl<S, T> DefaultResource<S, T>
where
    S: PartialEq + Clone + 'static,
    T: 'static,
{
    pub fn new_local<Fu: Future<Output = T> + 'static>(
        source: impl Fn() -> S + 'static,
        fetcher: impl Fn(S) -> Fu + 'static,
        default: fn() -> T,
    ) -> Self {
        let res = create_local_resource_with_initial_value(source, fetcher, Some(default()));
        DefaultResource { res, default }
    }

    pub fn new_server<Fu: Future<Output = T> + 'static>(
        source: impl Fn() -> S + 'static,
        fetcher: impl Fn(S) -> Fu + 'static,
        default: fn() -> T,
    ) -> Self
    where
        T: Serializable,
    {
        let res = create_resource_with_initial_value(source, fetcher, Some(default()));
        DefaultResource { res, default }
    }
}

impl<S, T> SignalGet for DefaultResource<S, T>
where
    S: PartialEq + Clone + 'static,
    T: Clone + 'static,
{
    type Value = T;

    fn get(&self) -> Self::Value {
        self.res.get().unwrap_or_else(&self.default)
    }

    fn try_get(&self) -> Option<Self::Value> {
        todo!()
    }
}

impl<S, T> FnOnce<()> for DefaultResource<S, T>
where
    S: PartialEq + Clone + 'static,
    T: Clone + 'static,
{
    type Output = T;

    #[inline(always)]
    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl<S, T> FnMut<()> for DefaultResource<S, T>
where
    S: PartialEq + Clone + 'static,
    T: Clone + 'static,
{
    #[inline(always)]
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl<S, T> Fn<()> for DefaultResource<S, T>
where
    S: PartialEq + Clone + 'static,
    T: Clone + 'static,
{
    #[inline(always)]
    extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl<S, T> SignalWith for DefaultResource<S, T>
where
    S: PartialEq + Clone + 'static,
    T: 'static,
{
    type Value = T;

    fn with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> O {
        self.res.with(|o| match o.as_ref() {
            Some(t) => f(t),
            None => f(&(self.default)()),
        })
    }

    fn try_with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        todo!()
    }
}

impl<S, T> SignalSet for DefaultResource<S, T>
where
    S: PartialEq + Clone + 'static,
    T: 'static,
{
    type Value = T;

    fn set(&self, new_value: Self::Value) {
        self.res.set(new_value)
    }

    fn try_set(&self, new_value: Self::Value) -> Option<Self::Value> {
        self.res.try_set(new_value)
    }
}

impl<S, T> FnOnce<(T,)> for DefaultResource<S, T>
where
    S: PartialEq + Clone + 'static,
    T: 'static,
{
    type Output = ();

    #[inline(always)]
    extern "rust-call" fn call_once(self, args: (T,)) -> Self::Output {
        self.set(args.0)
    }
}

impl<S, T> FnMut<(T,)> for DefaultResource<S, T>
where
    S: PartialEq + Clone + 'static,
    T: 'static,
{
    #[inline(always)]
    extern "rust-call" fn call_mut(&mut self, args: (T,)) -> Self::Output {
        self.set(args.0)
    }
}

impl<S, T> Fn<(T,)> for DefaultResource<S, T>
where
    S: PartialEq + Clone + 'static,
    T: 'static,
{
    #[inline(always)]
    extern "rust-call" fn call(&self, args: (T,)) -> Self::Output {
        self.set(args.0)
    }
}

impl<S, T> SignalUpdate for DefaultResource<S, T>
where
    S: PartialEq + Clone + 'static,
    T: 'static,
{
    type Value = T;

    fn update(&self, f: impl FnOnce(&mut Self::Value)) {
        self.res.update(|s| f(s.get_or_insert_with(&self.default)))
    }

    fn try_update<O>(&self, f: impl FnOnce(&mut Self::Value) -> O) -> Option<O> {
        self.res.try_update(|s| f(s.get_or_insert_with(&self.default)))
    }
}
