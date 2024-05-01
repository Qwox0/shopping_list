use leptos::{SignalUpdate, SignalWith};

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
