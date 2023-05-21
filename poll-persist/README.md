# `poll-persist`

This crate provides a `Future` wrapper that can be resolved multiple times without consuming it.

`poll-persist` is currently in a very early stage of development. It is not recommended for use in production code, but feel free to use it for experimentation.

## Why would you want this?

The [Barley system](https://github.com/panthios/barley) requires a `Future` wrapper that can be resolved multiple times from separate threads. This is not possible with the standard `Future` trait, since it consumes the future when it is resolved. This crate provides a solution to this problem. If this sounds useful to you, feel free to use it.