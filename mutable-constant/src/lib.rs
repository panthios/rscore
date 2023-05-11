//! # `mutable-constant`
//! 
//! This crate provides a smart pointer that allows
//! mutation on immutable values. In most cases, this
//! will behave like a normal smart pointer like `Box`,
//! but it allows mutation internally when Rust would
//! otherwise forbid it.
//! 
//! Doing this is an unsafe operation. You will need
//! to use `unsafe` blocks to use this crate effectively.
//! 
//! ## Example
//! 
//! ```
//! use mutable_constant::Mc;
//! 
//! let mc = Mc::new(42);
//! let mc_ref = mc.as_ref();
//! 
//! assert_eq!(*mc_ref, 42);
//! 
//! unsafe {
//!     *mc.as_defiant_mut() = 43; // This would normally be a compile error
//! }
//! 
//! assert_eq!(*mc_ref, 43);
//! ```

#![warn(missing_docs)]

/// A smart pointer that allows mutation on immutable values.
/// 
/// This uses a `*mut T` internally, so most internal operations
/// are unsafe. However, thanks to Rust's borrowing guarantees,
/// many of these operations are safe when used publicly. The
/// only operation that is not safe is `as_defiant_mut`, which
/// creates a mutable reference in defiance of Rust's rules.
pub struct Mc<T>(*mut T);

impl<T> Mc<T> {
    /// Creates a new `Mc` from a value.
    /// 
    /// # Example
    /// ```
    /// use mutable_constant::Mc;
    /// 
    /// let mc = Mc::new(42);
    /// ```
    pub fn new(t: T) -> Mc<T> {
        Mc(Box::into_raw(Box::new(t)))
    }

    /// Gets a mutable reference to the inner value.
    /// 
    /// # Safety
    /// 
    /// This is unsafe because the mutable reference does
    /// not obey the borrow checker. For example, a mutable
    /// reference to a `Mc` can be created while there is
    /// an immutable reference to the same `Mc`. If you do
    /// not need this specific behavior, use `as_mut` instead.
    /// 
    /// # Example
    /// ```
    /// use mutable_constant::Mc;
    /// 
    /// let mut mc = Mc::new(42);
    /// 
    /// unsafe {
    ///     *mc.as_defiant_mut() = 43;
    /// }
    /// ```
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

impl<T> std::fmt::Debug for Mc<T>
where
    T: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe { &*self.0 }.fmt(f)
    }
}

impl<T> std::fmt::Display for Mc<T>
where
    T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe { &*self.0 }.fmt(f)
    }
}

impl<T> Clone for Mc<T>
where
    T: Clone
{
    fn clone(&self) -> Self {
        Mc::new(unsafe { &*self.0 }.clone())
    }
}

impl<T> std::ops::Deref for Mc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T> std::ops::DerefMut for Mc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}