#![allow(dead_code)]

mod header;
mod jello;
mod method;
mod query;
mod req;
mod res;
mod status;

// reexport to make it jellostar::
pub use status::Status;

pub type Handle<S> = fn(&mut S, &mut req::Request, &mut res::Response) -> Status;

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::jello::Jello;

    #[test]
    fn basic() {}

    #[test]
    fn jello() {
        Jello::<()>::new()
            .handle("/hello-world", |_: _, _, _| {
                println!("hello-world");
                crate::Status::Ok
            })
            .handle("/", |_: _, _, _| {
                println!("root");
                crate::Status::Ok
            });
    }

    #[test]
    fn state() {
        Jello::new()
            .state(HashMap::<usize, &'static str>::new())
            .handle("/create", |state, _, _| {
                let count = state.len();
                state.insert(count, "hola");
                crate::Status::Created
            })
            .handle("/get", |state, _, _| {
                state.values().for_each(|s| print!("{}", s));
                println!("");
                crate::Status::Ok
            });
    }
}
