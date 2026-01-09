# Jellostar

> A zero runtime allocation and high performance http server core


## Features

- all heap allocation happens before the server starts accepting connections.
- no locking or state between threads
- multi threading enabled via `Jellostar::Conf` > 1 and `REUSEPORT`
- one state machine and its state per thread
- fine grained memory usage control via `--request-memory` and `--memory`.
- injecting state via `Server.state(name, value)`, extraction via the `state`
  param in `jellostar::Handle`

## Non features

- zero dynamic routing data extraction like `/user/:id`
- no per request heap allocation in the server core
- no async/Future overhead

## Comparison to other similar http servers

## Why its fast

- Preallocated and reused buffers per request
- No heap allocation in the core
- Synchronous, poll-based loop per thread, via an I/O state machine
- Minimal abstraction overhead (no async/await, no trait objects)

## Examples

For usage do look at the [reference](./reference/) crate.

### Hello world

```rust
fn main() {
    Jello::new()
        .handle("/", |_, _, _| {
            println!("Pong");
            crate::Status::Ok
        })
        .listen(addr!((0,0,0,0):1234))
        .expect("Failed to start server");
}
```

