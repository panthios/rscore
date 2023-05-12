use crate::Moment;
use async_trait::async_trait;


/// A node in a tree of asynchronous computations.
/// 
/// This enum is the core of the `moment-node` crate.
/// It allows any type that implements the `Moment`
/// trait to track its progress over time.
/// 
/// When resolving a `Node`, the enum state is kept the
/// same with resolved contents by default. This means the
/// type changes from `Node<T>` to `Node<T::Output>`. If you
/// do not want this, use the `Node::into_after` method. This
/// will keep the same type while resolving, allowing for easier
/// tree construction in some cases.
#[derive(Clone)]
pub enum Node<T: Moment> {
  /// The node has not been resolved yet.
  /// 
  /// This state does not mean the value is not
  /// finalized. It's just a hint to certain tree
  /// implementations that the value is not
  /// guaranteed to be stable.
  Before(T),
  /// The node has been resolved.
  /// 
  /// This state is guaranteed to be stable in a
  /// best-practices tree implementation. It is
  /// created with the `Node::into_after` method.
  After(T::Output)
}

impl<T> Node<T>
where
  T: Moment
{
  /// Create a new node.
  /// 
  /// This method is equivalent to `Node::Before(value)`.
  pub fn new(value: T) -> Self {
    Node::Before(value)
  }

  /// Convert the node to the `After` state.
  /// 
  /// This should only be done when the value is
  /// guaranteed to be stable. If the value may change
  /// on another `resolve`, use the `resolve` method
  /// instead. This rule is not enforced at compile time,
  /// so users should be careful when using this method.
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