# Jellostar

> A zero runtime allocation and high performance http server core

## Features

- no locking or state between threads
- multi threading enabled via `--threads` > 1 and `REUSEPORT`
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
    jellostar::Server::new()
        .handle("/", |_, req, res| {
            res.write(b"Hello, world!");
            jellostar::Status::Ok
        })
        .listen();
}
```

### Todo

```rust
// Note: The allocations here are part of user state.
// jellostar core still performs no allocations in the request/response path.

pub struct Todo {
    text: String,
}

fn new_todo(state: &mut jellostar::State, req: &mut jellostar::Request, res: &mut jellostar::Response) -> jellostar::Err {
    match req.kind {
        jellostar::Post => {
            let Some([text]) = req.query<String, bool>("text") else {
                jellostar::err!(jellostar::Status::BadRequest, "Missing 'text' query param");
            }

            let todos = state.get("todo");
            todos.push(todos.len(), Todo{text});
            jellostar::Status::Created
        },
        _ => jellostar::Status::UnsupportedMethod,
    }
}

fn get_todo(state: &mut jellostar::State, req: &mut jellostar::Request, res: &mut jellostar::Response) -> jellostar::Err {
    match req.kind {
        jellostar::Get => {
            let Some([id]) = req.query<usize>("id") else {
                return jellostar::Status::BadRequest;
            }

            let todos = state.get("todo");
            if let Some(todo) = todos.get(id) else {
                jellostar::err!(jellostar::Status::NotFound, "No todo found with the given id");
            }

            res.write(todo.text.as_bytes())
            jellostar::Status::Ok
        },
        _ => jellostar::Status::UnsupportedMethod,
    }
}

fn main() {
    jellostar::Server::new()
        .state("todos", HashMap<usize, Todo>::new())
        .handle("/new", new_todo)
        .handle("/get", get_todo)
        .listen()
}
```

