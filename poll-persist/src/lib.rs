#![warn(missing_docs)]

//! # `poll-persist`
//! 
//! This crate provides a wrapper around a [`Future`] that
//! can be polled multiple times. This is useful when you
//! want to await a future from multiple threads without
//! cloning it.
//! 
//! [`Future`]: https://doc.rust-lang.org/std/future/trait.Future.html

use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};
use mutable_constant::Mc;


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

/// A clone-able future that waits for resolution.
/// 
/// This allows multiple threads to await the same future
/// without cloning it. Cloning this future will not clone
/// the underlying future, it will simply clone the handle
/// to it.
#[pin_project::pin_project]
pub struct PollHook<T: Future>
where
    T::Output: Clone
{
    #[pin]
    future: Arc<Mc<PollPersist<T>>>
}

impl<T: Future> PollHook<T>
where
    T::Output: Clone
{
    /// Creates a new `PollHook` from a future.
    /// 
    /// This will wrap the future in an `Arc` so that it
    /// can be cloned.
    pub fn new(future: T) -> Self {
        PollHook {
            future: Arc::new(Mc::new(PollPersist::new(future)))
        }
    }

    /// Resolves the future.
    /// 
    /// # Safety
    /// 
    /// This function is unsafe due to its use of `as_defiant_mut`. It
    /// is safe to call this function if you are sure that the future
    /// is not being resolved from another thread.
    pub async unsafe fn resolve(&mut self) -> &T::Output {
        self.future.as_ref().as_defiant_mut().resolve().await
    }
}

impl<T: Future> Future for PollHook<T>
where
    T::Output: Clone
{
    type Output = T::Output;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let future = self.project().future.clone();

        match future.as_ref().as_ref() {
            PollPersist::Pending(_) => Poll::Pending,
            PollPersist::Ready(output) => Poll::Ready(output.clone())
        }
    }
}

impl<T: Future> Clone for PollHook<T>
where
    T::Output: Clone
{
    fn clone(&self) -> Self {
        PollHook {
            future: self.future.clone()
        }
    }
}