use std::{future::Future, pin::Pin};


pub enum PollPersist<T: Future> {
    Pending(Pin<Box<T>>),
    Ready(T::Output)
}

impl<T: Future> PollPersist<T> {
    pub fn new(future: T) -> Self {
        PollPersist::Pending(Box::pin(future))
    }

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