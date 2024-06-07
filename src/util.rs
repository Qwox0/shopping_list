use leptos::{
    create_effect, html::ElementDescriptor, svg::g, HtmlElement, NodeRef, SignalUpdate, SignalWith,
};
use std::{cell::RefCell, cmp::Ordering};

pub trait DecLen {
    fn dec_len(&self) -> u8;
}

impl DecLen for u64 {
    fn dec_len(&self) -> u8 {
        // TODO: binary search
        match *self {
            ..=9 => 1,
            ..=99 => 2,
            ..=999 => 3,
            ..=9999 => 4,
            ..=99999 => 5,
            ..=999999 => 6,
            ..=9999999 => 7,
            ..=99999999 => 8,
            ..=999999999 => 9,
            ..=9999999999 => 10,
            ..=99999999999 => 11,
            ..=999999999999 => 12,
            ..=9999999999999 => 13,
            ..=99999999999999 => 14,
            ..=999999999999999 => 15,
            ..=9999999999999999 => 16,
            ..=99999999999999999 => 17,
            ..=999999999999999999 => 18,
            ..=9999999999999999999 => 19,
            ..=u64::MAX => 20,
        }
    }
}

/// # Panic
///
/// Panics if the context doesn't exists
pub fn force_use_context<Ctx: Clone + 'static>() -> Ctx {
    leptos::use_context().unwrap_or_else(|| {
        panic!("expected context of type: {:?}", std::any::type_name::<Ctx>());
    })
}

pub trait IntoJsFuture {
    fn into_future(self) -> wasm_bindgen_futures::JsFuture;
}

impl IntoJsFuture for web_sys::js_sys::Promise {
    fn into_future(self) -> wasm_bindgen_futures::JsFuture {
        wasm_bindgen_futures::JsFuture::from(self)
    }
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum JsSetError {
    #[error("Can only set properties on Objects")]
    NotAnObject,

    #[error("Setting the propertiy was not successful")]
    NotSuccessful,
}

pub trait JsSet {
    fn set(&mut self, prop: &str, val: &wasm_bindgen::JsValue) -> Result<(), JsSetError>;
}

impl JsSet for web_sys::js_sys::Object {
    fn set(&mut self, prop: &str, val: &wasm_bindgen::JsValue) -> Result<(), JsSetError> {
        let prop = &prop.into();
        match web_sys::js_sys::Reflect::set(&self, prop, val) {
            Ok(true) => Ok(()),
            Ok(false) => Err(JsSetError::NotSuccessful),
            Err(_) => Err(JsSetError::NotAnObject), /* see <https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/set#exceptions> */
        }
    }
}

pub trait OptionDo<T> {
    fn do_(self, f: impl FnOnce(T));
}

impl<T> OptionDo<T> for Option<T> {
    fn do_(self, f: impl FnOnce(T)) {
        self.map(f);
    }
}

pub trait SignalWithMap<T>: SignalWith<Value = Option<T>> {
    fn with_map<O>(&self, f: impl FnOnce(&T) -> O) -> Option<O> {
        self.with(|o| o.as_ref().map(f))
    }
}

impl<Sig: SignalWith<Value = Option<T>>, T> SignalWithMap<T> for Sig {}

pub trait SignalUpdateSome<T>: SignalUpdate<Value = Option<T>> {
    fn update_some(&self, f: impl FnOnce(&mut T)) {
        self.update(|o| o.as_mut().do_(f))
    }
}

impl<Sig: SignalUpdate<Value = Option<T>>, T> SignalUpdateSome<T> for Sig {}

/// runs `f` when the `element` is rendered
pub fn on_render_elem<T>(element: NodeRef<T>, f: impl Fn(&HtmlElement<T>) + 'static)
where T: ElementDescriptor + Clone {
    element.on_load(move |el| {
        create_effect(move |_| f(&el));
    });
}

/// runs `f` when the component is rendered
pub fn on_render(f: impl FnOnce() + 'static) {
    let mut f = RefCell::new(Some(f));
    create_effect(move |_| f.take().do_(|f| f()));
}

pub trait VecExt<T> {
    fn sorted(self) -> Self
    where T: Ord;

    fn sorted_by(self, f: impl FnMut(&T, &T) -> Ordering) -> Self;

    fn sorted_by_key<K>(self, f: impl FnMut(&T) -> K) -> Self
    where K: Ord;
}

impl<T> VecExt<T> for Vec<T> {
    fn sorted(mut self) -> Self
    where T: Ord {
        self.sort();
        self
    }

    fn sorted_by(mut self, compare: impl FnMut(&T, &T) -> Ordering) -> Self {
        self.sort_by(compare);
        self
    }

    fn sorted_by_key<K>(mut self, f: impl FnMut(&T) -> K) -> Self
    where K: Ord {
        self.sort_by_key(f);
        self
    }
}
