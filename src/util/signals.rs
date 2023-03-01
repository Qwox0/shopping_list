use leptos::*;

pub trait ReadSignalUtils<T: PartialEq> {
    fn contains(&self, rhs: &T) -> bool;
}

impl<T: PartialEq> ReadSignalUtils<T> for ReadSignal<T> {
    fn contains(&self, rhs: &T) -> bool {
        self.with(|x| x == rhs)
    }
}

impl<T: PartialEq> ReadSignalUtils<T> for RwSignal<T> {
    fn contains(&self, rhs: &T) -> bool {
        self.with(|x| x == rhs)
    }
}
