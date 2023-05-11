pub struct Mc<T>(*mut T);

impl<T> Mc<T> {
    pub fn new(t: T) -> Mc<T> {
        Mc(Box::into_raw(Box::new(t)))
    }

    pub unsafe fn as_defiant_mut(&self) -> &mut T {
        &mut *self.0
    }
}

impl<T> Drop for Mc<T> {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.0));
        }
    }
}

impl<T> AsRef<T> for Mc<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.0 }
    }
}

impl<T> AsMut<T> for Mc<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0 }
    }
}