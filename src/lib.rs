#![allow(dead_code)]

/// all possible errors jello's usage can produce and a API for interacting with them
mod err;
/// http headers with minimalist API
mod header;
/// server builder and runtime entry point
mod jello;
/// macros for construcing ipv4&ipv6, for easier error handling
mod macros;
/// http methods
mod method;
/// uri query abstraction ?<key>=<value>&<key>=<value>
mod query;
/// request abstraction for extracting values
mod req;
/// response abstraction for writing to response bodies
mod res;
/// the main I/O event loop, sheduling connections
mod runtime;
/// http status codes: u16 and &str representations
mod status;

// reexports to establish a public and stable API
pub use jello::*;
pub use method::Method;
pub use status::Status;

/// Non-capturing closures or function pointers
pub type Handle<S> = fn(&mut S, &req::Request, &mut res::Response) -> Status;

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{addr, jello::Jello, or500};

    #[test]
    fn basic() {
        Jello::new()
            .handle("/", |_, _, res| {
                or500!(res.write_str("Hello World"));
                crate::Status::Ok
            })
            .listen(addr!((0,0,0,0):1234))
            .expect("Failed to start server");
    }

    #[test]
    fn jello() {
        Jello::new()
            .handle("/hello-world", |_, _, res| {
                or500!(res.write_str("Hello World"));
                crate::Status::Ok
            })
            .handle("/", |_, _, res| {
                or500!(res.write_str("root"));
                crate::Status::Ok
            })
            .listen(addr!((0,0,0,0):1234))
            .expect("Failed to start server");
    }

    #[test]
    fn state() {
        Jello::with_state(HashMap::<usize, &'static str>::new())
            .handle("/create", |state, _, _| {
                let count = state.len();
                state.insert(count, "hola");
                crate::Status::Created
            })
            .handle("/get", |state, _, _| {
                state.values().for_each(|s| print!("{}", s));
                println!("");
                crate::Status::Ok
            })
            .listen(addr!((0,0,0,0):1234))
            .expect("Failed to start server");
    }
}
