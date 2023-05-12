use crate::Moment;
use async_trait::async_trait;


#[derive(Clone)]
pub enum Node<T: Moment> {
  Before(T),
  After(T::Output)
}

impl<T> Node<T>
where
  T: Moment
{
  pub fn new(value: T) -> Self {
    Node::Before(value)
  }

  pub async fn into_after(self) -> Self {
    match self {
      Node::Before(value) => Node::After(value.resolve().await),
      Node::After(value) => Node::After(value)
    }
  }
}

#[async_trait]
impl<T> Moment for Node<T>
where
  T: Moment
{
  type Output = Node<T::Output>;

  async fn resolve(&self) -> Self::Output {
    match self {
      Node::Before(value) => Node::Before(value.resolve().await),
      Node::After(value) => Node::After(value.resolve().await)
    }
  }
}