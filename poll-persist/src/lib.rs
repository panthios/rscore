#![warn(missing_docs)]

//! # `poll-persist`
//! 
//! This crate provides a wrapper around a [`Future`] that
//! can be polled multiple times. This is useful when you
//! want to await a future from multiple threads without
//! cloning it.
//! 
//! [`Future`]: https://doc.rust-lang.org/std/future/trait.Future.html

use std::{future::Future, pin::Pin};


/// A persistent future that can be polled multiple times.
/// 
/// When resolved, this future will update itself to store the
/// resolved value. This allows it to be polled multiple times
/// without cloning. Any resolution after the first will be
/// completed immediately.
pub enum PollPersist<T: Future> {
    /// The future has not been resolved yet.
    Pending(Pin<Box<T>>),
    /// The future has been resolved.
    Ready(T::Output)
}

impl<T: Future> PollPersist<T> {
    /// Creates a new `PollPersist` from a future.
    pub fn new(future: T) -> Self {
        PollPersist::Pending(Box::pin(future))
    }

    /// Resolves the future.
    pub async fn resolve(&mut self) -> &T::Output {
        match self {
            PollPersist::Pending(future) => {
                let output = future.as_mut().await;
                *self = PollPersist::Ready(output);
                match self {
                    PollPersist::Ready(output) => output,
                    _ => unreachable!()
                }
            },
            PollPersist::Ready(output) => output
        }
    }
}