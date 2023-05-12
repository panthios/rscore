# `inscope`

This crate implements an `Option`-like type that can be used to represent
values that are "in scope" or "out of scope".

This crate is currently unstable and is not recommended for use in production.

## Why?

This crate was created to solve a problem that I encountered while working on
various other Panthios utilities. I needed a way to represent values that were
deletable in other threads, and the deletion should be reflected in the
original thread. This could be solved with an `Arc<Mutex<T>>`, but that is
overkill for this specific use case. Instead, the [`mutable-constant`](../mutable-constant/README.md)
crate can be used to create a mutable `Option` that can be set to `None` from
another thread.

## Example

```rust
use inscope::Scope;

let scope = Scope::new(42);
assert_eq!(*scope, Some(42));

// This requires unsafe because defiant
// `Mc` mutations are unsafe
unsafe {
  scope.delete();
}

assert_eq!(*scope, None);
```