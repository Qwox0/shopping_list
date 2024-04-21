use leptos::{
    ReadSignal, Resource, Signal, SignalGet, SignalSet, SignalSetter, SignalUpdate, SignalWith,
    StoredValue,
};

#[derive(Debug, Clone, Copy)]
pub struct SubReadSignal<Sig, F> {
    //sig: Signal<T>,
    sig: Sig,
    get_ref: F,
}

impl<Sig, F, U> SignalGet for SubReadSignal<Sig, F>
where
    Sig: SignalWith,
    F: Fn(&Sig::Value) -> &U,
    U: Clone,
{
    type Value = U;

    fn get(&self) -> Self::Value {
        self.sig.with(|t| (self.get_ref)(t).clone())
    }

    fn try_get(&self) -> Option<Self::Value> {
        Some(self.get())
    }
}

impl<Sig, F, U> SignalWith for SubReadSignal<Sig, F>
where
    Sig: SignalWith,
    F: Fn(&Sig::Value) -> &U,
{
    type Value = U;

    fn with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> O {
        self.sig.with(|t| f((self.get_ref)(t)))
    }

    fn try_with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        self.sig.try_with(|t| f((self.get_ref)(t)))
    }
}

impl<Sig, F, U> FnOnce<()> for SubReadSignal<Sig, F>
where
    Sig: SignalWith,
    F: Fn(&Sig::Value) -> &U,
    U: Clone,
{
    type Output = U;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl<Sig, F, U> FnMut<()> for SubReadSignal<Sig, F>
where
    Sig: SignalWith,
    F: Fn(&Sig::Value) -> &U,
    U: Clone,
{
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl<Sig, F, U> Fn<()> for SubReadSignal<Sig, F>
where
    Sig: SignalWith,
    F: Fn(&Sig::Value) -> &U,
    U: Clone,
{
    extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
        self.get()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SubWriteSignal<Sig, G> {
    sig: Sig,
    get_mut: G,
}

impl<Sig, G, U> SignalSet for SubWriteSignal<Sig, G>
where
    Sig: SignalUpdate,
    G: Fn(&mut Sig::Value) -> &mut U,
{
    type Value = U;

    fn set(&self, new_value: Self::Value) {
        self.sig.update(|t| *(self.get_mut)(t) = new_value)
    }

    fn try_set(&self, new_value: Self::Value) -> Option<Self::Value> {
        self.set(new_value);
        None
    }
}

impl<Sig, G, U> SignalUpdate for SubWriteSignal<Sig, G>
where
    Sig: SignalUpdate,
    G: Fn(&mut Sig::Value) -> &mut U,
{
    type Value = U;

    fn update(&self, f: impl FnOnce(&mut Self::Value)) {
        self.sig.update(|t| f((self.get_mut)(t)))
    }

    fn try_update<O>(&self, f: impl FnOnce(&mut Self::Value) -> O) -> Option<O> {
        self.sig.try_update(|t| f((self.get_mut)(t)))
    }
}

impl<Sig, G, U> FnOnce<(U,)> for SubWriteSignal<Sig, G>
where
    Sig: SignalUpdate,
    G: Fn(&mut Sig::Value) -> &mut U,
{
    type Output = ();

    extern "rust-call" fn call_once(self, args: (U,)) -> Self::Output {
        self.set(args.0);
    }
}

impl<Sig, G, U> FnMut<(U,)> for SubWriteSignal<Sig, G>
where
    Sig: SignalUpdate,
    G: Fn(&mut Sig::Value) -> &mut U,
{
    extern "rust-call" fn call_mut(&mut self, args: (U,)) -> Self::Output {
        self.set(args.0);
    }
}

impl<Sig, G, U> Fn<(U,)> for SubWriteSignal<Sig, G>
where
    Sig: SignalUpdate,
    G: Fn(&mut Sig::Value) -> &mut U,
{
    extern "rust-call" fn call(&self, args: (U,)) -> Self::Output {
        self.set(args.0);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SubRWSignal<Sig, F, G> {
    sig: Sig,
    get_ref: F,
    get_mut: G,
}

impl<Sig, F, G, U> SignalGet for SubRWSignal<Sig, F, G>
where
    Sig: SignalWith,
    F: Fn(&Sig::Value) -> &U,
    U: Clone,
{
    type Value = U;

    fn get(&self) -> Self::Value {
        self.sig.with(|t| (self.get_ref)(t).clone())
    }

    fn try_get(&self) -> Option<Self::Value> {
        Some(self.get())
    }
}

impl<Sig, F, G, U> SignalWith for SubRWSignal<Sig, F, G>
where
    Sig: SignalWith,
    F: Fn(&Sig::Value) -> &U,
{
    type Value = U;

    fn with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> O {
        self.sig.with(|t| f((self.get_ref)(t)))
    }

    fn try_with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        self.sig.try_with(|t| f((self.get_ref)(t)))
    }
}

impl<Sig, F, G, U> FnOnce<()> for SubRWSignal<Sig, F, G>
where
    Sig: SignalWith,
    F: Fn(&Sig::Value) -> &U,
    U: Clone,
{
    type Output = U;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl<Sig, F, G, U> FnMut<()> for SubRWSignal<Sig, F, G>
where
    Sig: SignalWith,
    F: Fn(&Sig::Value) -> &U,
    U: Clone,
{
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl<Sig, F, G, U> Fn<()> for SubRWSignal<Sig, F, G>
where
    Sig: SignalWith,
    F: Fn(&Sig::Value) -> &U,
    U: Clone,
{
    extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl<Sig, F, G, U> SignalSet for SubRWSignal<Sig, F, G>
where
    Sig: SignalUpdate,
    G: Fn(&mut Sig::Value) -> &mut U,
{
    type Value = U;

    fn set(&self, new_value: Self::Value) {
        self.sig.update(|t| *(self.get_mut)(t) = new_value)
    }

    fn try_set(&self, new_value: Self::Value) -> Option<Self::Value> {
        self.set(new_value);
        None
    }
}

impl<Sig, F, G, U> SignalUpdate for SubRWSignal<Sig, F, G>
where
    Sig: SignalUpdate,
    G: Fn(&mut Sig::Value) -> &mut U,
{
    type Value = U;

    fn update(&self, f: impl FnOnce(&mut Self::Value)) {
        self.sig.update(|t| f((self.get_mut)(t)))
    }

    fn try_update<O>(&self, f: impl FnOnce(&mut Self::Value) -> O) -> Option<O> {
        self.sig.try_update(|t| f((self.get_mut)(t)))
    }
}

impl<Sig, F, G, U> FnOnce<(U,)> for SubRWSignal<Sig, F, G>
where
    Sig: SignalUpdate,
    G: Fn(&mut Sig::Value) -> &mut U,
{
    type Output = ();

    extern "rust-call" fn call_once(self, args: (U,)) -> Self::Output {
        self.set(args.0);
    }
}

impl<Sig, F, G, U> FnMut<(U,)> for SubRWSignal<Sig, F, G>
where
    Sig: SignalUpdate,
    G: Fn(&mut Sig::Value) -> &mut U,
{
    extern "rust-call" fn call_mut(&mut self, args: (U,)) -> Self::Output {
        self.set(args.0);
    }
}

impl<Sig, F, G, U> Fn<(U,)> for SubRWSignal<Sig, F, G>
where
    Sig: SignalUpdate,
    G: Fn(&mut Sig::Value) -> &mut U,
{
    extern "rust-call" fn call(&self, args: (U,)) -> Self::Output {
        self.set(args.0);
    }
}

pub fn subsignal<Sig, U, F, G>(sig: Sig, get_ref: F, get_mut: G) -> SubRWSignal<Sig, F, G>
where
    Sig: SignalWith,
    F: Fn(&Sig::Value) -> &U,
    G: Fn(&mut Sig::Value) -> &mut U,
{
    SubRWSignal { sig, get_ref, get_mut }
}

struct SubReadSignal2<T: 'static, F> {
    sig: Signal<T>,
    get_ref: F,
}

pub fn subsignal2<T, U, F, G>(sig: Signal<T>, get_ref: F, get_mut: G) -> ()
where
    T: Clone,
    F: Fn(&T) -> &U,
    G: Fn(&mut T) -> &mut U,
{
    SubReadSignal2 { sig, get_ref };
}

impl<Sig, F, G, T> SubRWSignal<Sig, F, G>
where
    Sig: SignalWith,
    F: Fn(&Sig::Value) -> &T,
    G: Fn(&mut Sig::Value) -> &mut T,
{
    pub fn new(sig: Sig, get_ref: F, get_mut: G) -> Self {
        SubRWSignal { sig, get_ref, get_mut }
    }

    pub fn split(self) -> (SubReadSignal<Sig, F>, SubWriteSignal<Sig, G>)
    where Sig: Copy {
        let Self { sig, get_ref, get_mut } = self;
        (SubReadSignal { sig, get_ref }, SubWriteSignal { sig, get_mut })
    }
}

pub fn subsignals<Sig, T>(
    sig: Sig,
) -> Vec<SubRWSignal<Sig, impl Fn(&Vec<T>) -> &T, impl Fn(&mut Vec<T>) -> &mut T>>
where Sig: SignalWith<Value = Vec<T>> + Copy {
    let len = sig.with(Vec::len);
    (0..len)
        .map(|idx| subsignal(sig, move |v| &v[idx], move |v| &mut v[idx]))
        .collect()
}

#[cfg(test)]
pub mod test {
    use crate::subsignal::{subsignal, subsignals};
    use leptos::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Test {
        a: i32,
        b: Vec<f32>,
    }

    #[test]
    fn test_subsignal() {
        let rt = create_runtime();
        let test_signal = create_rw_signal(Test { a: 0, b: vec![1.0, 2.0, 3.0] });

        let (a, set_a) = subsignal(test_signal, |x| &x.a, |x| &mut x.a).split();
        let (b, set_b) = subsignal(test_signal, |x| &x.b, |x| &mut x.b).split();

        println!("test: {:?}", test_signal.get_untracked());

        println!("update a");
        set_a(10);
        println!("a: {:?}", a());
        assert_eq!(a(), 10);
        println!("test: {:?}", test_signal.get_untracked());
        assert_eq!(test_signal.get_untracked(), Test { a: 10, b: vec![1.0, 2.0, 3.0] });

        println!("update b");
        set_b(vec![5.0]);
        println!("b: {:?}", b());
        assert_eq!(&b(), &[5.0]);
        println!("test: {:?}", test_signal.get_untracked());
        assert_eq!(test_signal.get_untracked(), Test { a: 10, b: vec![5.0] });

        rt.dispose();
    }

    #[test]
    fn test_subsignals() {
        let rt = create_runtime();
        let vec_signal = create_rw_signal(vec![1, 2, 3]);

        let signals = subsignals(vec_signal);

        println!("vec: {:?}", vec_signal.get_untracked());

        println!("update 0");
        signals[0].set(10);
        println!("0: {:?}", signals[0].get());
        assert_eq!(signals[0].get(), 10);
        println!("vec: {:?}", vec_signal.get_untracked());

        println!("update 2");
        signals[2].set(0);
        println!("2: {:?}", signals[2].get());
        assert_eq!(signals[2].get(), 0);
        println!("vec: {:?}", vec_signal.get_untracked());

        rt.dispose();
    }
}
