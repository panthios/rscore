use std::ops::{Deref, DerefMut};
use mutable_constant::Mc;


pub struct Scope<T>(Mc<Option<T>>);

impl<T> Scope<T> {
    pub fn new(t: T) -> Scope<T> {
        Scope(Mc::new(Some(t)))
    }

    pub unsafe fn delete(&self) {
        self.0.as_defiant_mut().take();
    }
}

impl<T> Deref for Scope<T> {
    type Target = Mc<Option<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Scope<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}