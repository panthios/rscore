# `mutable-constant`

This crate provides a smart pointer that allows mutation even when the pointer is immutable.

This crate is very new, and is not at a stable version yet. It is not recommended for production use.

## Why would you want this?

Consider the following example:

```rust
let mut x = 0;
let y = &x;

// Error: `x` already borrowed as immutable
x += 1;
```

In this example, `x` is the clear owner of a value, and `y` wants to reference the value for later. In most cases, this error is justified, since `y` could change unexpectedly somewhere else. However, in this case, we want to know when `x` changes through `y`. The common solution is to use something like `Cell` or `RefCell`:

```rust
let x = Cell::new(0);
let y = &x;

x.set(x.get() + 1);
```

This works, but it's not very ergonomic. `Cell` and `RefCell` are also not very efficient, since they use runtime checks to ensure that the value is not borrowed mutably. There are a few situations where this is not acceptable:

- You want the zero-cost abstraction of a basic reference
- The value is generally immutable, but can change in specific situations. Other than those cases, the value should be immutable, and that fact should be enforced by the compiler.
- You want a cleaner solution that clearly communicates your intent, including possible mutations and the lifetime of the value.

This crate provides a solution to these problems. It is not a `RefCell` alternative. Instead, it is a cheaper alternative to shared mutability in general, where mutable references are either compiler-enforced or explicitly unsafe. Here's how it could be used in the above case:

```rust
use mutable_constant::Mc;

let x = Mc::new(0);
let y = &x;

// This is unsafe, since `x` is already referenced
unsafe {
  *x.as_defiant_mut() += 1;
}
```