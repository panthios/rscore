use poll_persist::{PollPersist, PollHook};
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

#[tokio::test]
async fn test_poll_hook() {
  let mut poll_hook = PollHook::new(async {
    sleep(Duration::from_millis(250)).await;
    42
  });

  let poll_hook_clone = poll_hook.clone();
  tokio::spawn(async move {
    assert_eq!(poll_hook_clone.await, 42);
  });

  unsafe {
    assert_eq!(*poll_hook.resolve().await, 42);
  }
}