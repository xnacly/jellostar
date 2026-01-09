# Jellostar

> A zero runtime allocation and high performance http server core


```rust
fn main() {
    Jello::new()
        .handle("/", |_, _, res| {
            or500!(res.write_str("Hello World"));
            crate::Status::Ok
        })
        .listen(addr!((0,0,0,0):1234))
        .expect("Failed to start server");
}
```


## Features

- all heap allocation happens before the server starts accepting connections
- no locking or state between threads
- multi threading enabled via `Jellostar::Conf::threads` > 1 and `REUSEPORT`
- one state machine and its state per thread
- fine grained memory usage control via `Jello::Conf::memory_per_request` and
  `Jello::Conf::memory_totoal`.
- injecting state via `Jello::with_state`, extraction via a handlers first
  param

## Non features

- zero dynamic routing data extraction like `/user/:id`
- no per request heap allocation in the server core
- no async/Future overhead

## Comparison to similar projects

## Why its fast

- Preallocated and reused buffers per request
- No heap allocation in the core
- Synchronous, poll-based loop per thread, via an I/O state machine
- Minimal abstraction overhead (no async/await, no trait objects)

## Examples

For usage do look at the [reference](./reference/) crate.
