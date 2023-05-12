use std::error::Error;
use futures::future::join_all;
use async_trait::async_trait;
use crate::Moment;


#[async_trait]
impl<T> Moment for Option<T>
where
    T: Moment,
{
    type Output = Option<T::Output>;

    async fn resolve(&self) -> Self::Output {
        if let Some(value) = self {
            Some(value.resolve().await)
        } else {
            None
        }
    }
}

#[async_trait]
impl<T, E> Moment for Result<T, E>
where
    T: Moment,
    E: Send + Sync + Clone + Error
{
    type Output = Result<T::Output, E>;

    async fn resolve(&self) -> Self::Output {
        match self {
            Ok(value) => Ok(value.resolve().await),
            Err(error) => Err(error.clone())
        }
    }
}

#[async_trait]
impl<T> Moment for Vec<T>
where
  T: Moment
{
  type Output = Vec<T::Output>;

  async fn resolve(&self) -> Self::Output {
    let mut output = Vec::with_capacity(self.len());

    for value in self {
      output.push(value.resolve());
    }

    let output = join_all(output).await;

    output
  }
}

#[async_trait]
impl Moment for String {
  type Output = String;

  async fn resolve(&self) -> Self::Output {
    self.clone()
  }
}

#[async_trait]
impl Moment for &str {
  type Output = String;

  async fn resolve(&self) -> Self::Output {
    self.to_string()
  }
}

#[async_trait]
impl Moment for () {
  type Output = ();

  async fn resolve(&self) -> Self::Output {
    ()
  }
}