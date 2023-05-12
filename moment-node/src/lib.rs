#![warn(missing_docs)]

//! # `moment-node`
//! 
//! This crate provides a framework for building
//! trees of asynchronous computations. The actual
//! calculations are not in this crate. Instead, other
//! crates can implement the `Moment` trait and use
//! the `Node` enum to build any kind of tree they
//! want.
//! 
//! ## Example
//! 
//! ```
//! use moment_node::Moment;
//! use async_trait::async_trait;
//! use std::ops::Add;
//! 
//! #[derive(Clone)]
//! struct AddOne<T>(pub T)
//! where
//!     T: Moment,
//!     T::Output: Add<i32>;
//! 
//! #[async_trait]
//! impl<T> Moment for AddOne<T>
//! where
//!     T: Moment,
//!     T::Output: Add<i32>,
//!     <T::Output as Add<i32>>::Output: Moment
//! {
//!     type Output = <T::Output as Add<i32>>::Output;
//! 
//!     async fn resolve(&self) -> Self::Output {
//!         self.0.resolve().await + 1
//!     }
//! }
//! 
//! #[tokio::main]
//! async fn main() {
//!     let op = AddOne(10);
//!     assert_eq!(op.resolve().await, 11);
//! }
//! ```

use async_trait::async_trait;

mod node;
pub use node::Node;

mod impls;


/// An asynchronously resolvable value.
/// 
/// This trait allows any literal or operation
/// to be used in a `Node` tree. It is not
/// automatically implemented for any type other
/// than the ones in the standard library.
#[async_trait]
pub trait Moment: Clone + Send + Sync {
    /// The type of the value that will be resolved.
    /// 
    /// If the original type is a generic of some
    /// kind (e.g. `Vec<T>`), then the output type
    /// should be the same generic type with resolved
    /// contents (e.g. `Vec<T::Output>`)
    type Output: Moment;

    /// Resolve the value.
    /// 
    /// This method should return the resolved value
    /// as soon as possible. If the value is already
    /// resolved, then it should return immediately.
    async fn resolve(&self) -> Self::Output;
}

macro_rules! literal_moments {
    ($($name:ident),*) => {
        $(
            #[async_trait]
            impl Moment for $name {
                type Output = $name;

                async fn resolve(&self) -> Self::Output {
                    *self
                }
            }
        )*
    }
}

literal_moments![
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
    bool,
    char
];