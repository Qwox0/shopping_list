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
