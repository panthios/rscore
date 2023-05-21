use poll_persist::PollPersist;
use tokio::time::{sleep, Duration};


#[tokio::test]
async fn test_poll_persist() {
  let mut poll_persist = PollPersist::new(async {
    sleep(Duration::from_millis(50)).await;
    42
  });

  assert_eq!(*poll_persist.resolve().await, 42);

  if let PollPersist::Pending(_) = poll_persist {
    panic!("PollPersist should be resolved");
  }

  assert_eq!(*poll_persist.resolve().await, 42);
}