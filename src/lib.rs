#![allow(dead_code)]

mod header;
mod jello;
mod method;
mod query;
mod req;
mod res;
mod status;

// reexport to make it jellostar::Status
pub use jello::*;
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
