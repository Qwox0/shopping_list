use leptos::{
    create_local_resource, create_local_resource_with_initial_value, create_resource,
    create_resource_with_initial_value, Resource, Serializable, SignalGet, SignalSet, SignalUpdate,
    SignalWith,
};
use std::future::Future;

#[derive(Debug)]
pub struct DefaultResource<S: 'static, T: 'static, D> {
    res: Resource<S, T>,
    default: D,
}

impl<S: 'static, T: 'static, D: Clone> Clone for DefaultResource<S, T, D> {
    fn clone(&self) -> Self {
        Self { res: self.res.clone(), default: self.default.clone() }
    }
}

impl<S: 'static, T: 'static, D: Copy> Copy for DefaultResource<S, T, D> {}

impl<S, T, D> DefaultResource<S, T, D>
where
    S: PartialEq + Clone + 'static,
    T: 'static,
    D: Fn() -> T,
{
    pub fn new_local<Fu: Future<Output = T> + 'static>(
        source: impl Fn() -> S + 'static,
        fetcher: impl Fn(S) -> Fu + 'static,
        default: D,
    ) -> Self {
        let res = create_local_resource_with_initial_value(source, fetcher, Some(default()));
        DefaultResource { res, default }
    }

    pub fn new_server<Fu: Future<Output = T> + 'static>(
        source: impl Fn() -> S + 'static,
        fetcher: impl Fn(S) -> Fu + 'static,
        default: D,
    ) -> Self
    where
        T: Serializable,
    {
        let res = create_resource_with_initial_value(source, fetcher, Some(default()));
        DefaultResource { res, default }
    }
}

impl<S, T, D> SignalGet for DefaultResource<S, T, D>
where
    S: PartialEq + Clone + 'static,
    T: Clone + 'static,
    D: Fn() -> T,
{
    type Value = T;

    fn get(&self) -> Self::Value {
        self.res.get().unwrap_or_else(&self.default)
    }

    fn try_get(&self) -> Option<Self::Value> {
        todo!()
    }
}

impl<S, T, D> FnOnce<()> for DefaultResource<S, T, D>
where
    S: PartialEq + Clone + 'static,
    T: Clone + 'static,
    D: Fn() -> T,
{
    type Output = T;

    #[inline(always)]
    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl<S, T, D> FnMut<()> for DefaultResource<S, T, D>
where
    S: PartialEq + Clone + 'static,
    T: Clone + 'static,
    D: Fn() -> T,
{
    #[inline(always)]
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl<S, T, D> Fn<()> for DefaultResource<S, T, D>
where
    S: PartialEq + Clone + 'static,
    T: Clone + 'static,
    D: Fn() -> T,
{
    #[inline(always)]
    extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl<S, T, D> SignalWith for DefaultResource<S, T, D>
where
    S: PartialEq + Clone + 'static,
    T: 'static,
    D: Fn() -> T,
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

impl<S, T, D> SignalSet for DefaultResource<S, T, D>
where
    S: PartialEq + Clone + 'static,
    T: 'static,
    D: Fn() -> T,
{
    type Value = T;

    fn set(&self, new_value: Self::Value) {
        self.res.set(new_value)
    }

    fn try_set(&self, new_value: Self::Value) -> Option<Self::Value> {
        self.res.try_set(new_value)
    }
}

impl<S, T, D> FnOnce<(T,)> for DefaultResource<S, T, D>
where
    S: PartialEq + Clone + 'static,
    T: 'static,
    D: Fn() -> T,
{
    type Output = ();

    #[inline(always)]
    extern "rust-call" fn call_once(self, args: (T,)) -> Self::Output {
        self.set(args.0)
    }
}

impl<S, T, D> FnMut<(T,)> for DefaultResource<S, T, D>
where
    S: PartialEq + Clone + 'static,
    T: 'static,
    D: Fn() -> T,
{
    #[inline(always)]
    extern "rust-call" fn call_mut(&mut self, args: (T,)) -> Self::Output {
        self.set(args.0)
    }
}

impl<S, T, D> Fn<(T,)> for DefaultResource<S, T, D>
where
    S: PartialEq + Clone + 'static,
    T: 'static,
    D: Fn() -> T,
{
    #[inline(always)]
    extern "rust-call" fn call(&self, args: (T,)) -> Self::Output {
        self.set(args.0)
    }
}

impl<S, T, D> SignalUpdate for DefaultResource<S, T, D>
where
    S: PartialEq + Clone + 'static,
    T: 'static,
    D: Fn() -> T,
{
    type Value = T;

    fn update(&self, f: impl FnOnce(&mut Self::Value)) {
        self.res.update(|s| f(s.get_or_insert_with(&self.default)))
    }

    fn try_update<O>(&self, f: impl FnOnce(&mut Self::Value) -> O) -> Option<O> {
        self.res.try_update(|s| f(s.get_or_insert_with(&self.default)))
    }
}
