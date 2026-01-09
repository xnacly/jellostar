# Jellostar

> A zero runtime allocation and high performance http server core, its a
> throughput monster


```rust
fn main() {
    jellostar::Jello::new()
        .handle("/", |_, _, res| {
            or500!(res.write_str("Hello World"));
            jellostar::Status::Ok
        })
        .listen(jellostar::addr!((0,0,0,0):1234))
        .expect("Failed to start server");
}
```


## Features

- all heap allocation happens before the server starts accepting connections
- no locking or state between threads
- multi threading enabled via `jellostar::Conf::threads` > 1 and `REUSEPORT`
- one state machine and its state per thread
- fine grained memory usage control via `jellostar::Conf::memory_per_request` and
  `jellostar::Conf::memory_totoal`.
- injecting state via `Jello::with_state`, extraction via a handlers first
  param
- non-blocking and fast handlers
- if the event loop is blocked by a slow handler, jello drops the connection,
  configurable (in `ms`) via `jellostar::Conf::max_request_time`

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
