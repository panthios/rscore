#![warn(missing_docs)]

//! # `inscope`
//! 
//! This crate provides a smart pointer that allows
//! dropping a value while it is still borrowed. In most
//! cases, this is completely useless. However, it can be
//! useful when a resource is deleted but still needs to
//! be used while other threads process the deletion.
//! 
//! Deleting a value while it is still borrowed is an
//! unsafe operation. You will need to use `unsafe` blocks
//! to use this crate effectively.
//! 
//! ## Example
//! 
//! ```
//! use inscope::Scope;
//! 
//! let scope = Scope::new(42);
//! assert_eq!(*scope, Some(42));
//! 
//! unsafe {
//!    scope.delete();
//! }
//! 
//! assert_eq!(*scope, None);
//! ```

use std::ops::{Deref, DerefMut};
use mutable_constant::Mc;


/// A smart pointer that can drop its content while borrowed.
/// 
/// This uses a `Mc` internally from the `mutable-constant`
/// crate. This allows mutation on immutable values, which
/// is necessary to drop the value while it is borrowed.
pub struct Scope<T>(Mc<Option<T>>);

impl<T> Scope<T> {
    /// Creates a new `Scope` from a value.
    /// 
    /// # Example
    /// ```
    /// use inscope::Scope;
    /// 
    /// let scope = Scope::new(42);
    /// ```
    pub fn new(t: T) -> Scope<T> {
        Scope(Mc::new(Some(t)))
    }

    /// Deletes the inner value.
    /// 
    /// # Safety
    /// 
    /// This is unsafe because the internal `Mc` is defiantly
    /// mutated to delete the value. This is not allowed by
    /// Rust's borrowing rules.
    /// 
    /// # Example
    /// 
    /// ```
    /// use inscope::Scope;
    /// 
    /// let scope = Scope::new(42);
    /// 
    /// unsafe {
    ///    scope.delete();
    /// }
    /// ```
    pub unsafe fn delete(&self) {
        self.0.as_defiant_mut().take();
    }
}

impl<T> Deref for Scope<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Scope<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}