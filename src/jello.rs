use crate::{err, runtime::Runtime};
use std::net::SocketAddr;

use crate::Handle;

/// user facing configuration
pub struct Conf {
    pub threads: u8,
    pub memory_per_request: usize,
    pub memory_total: usize,
    pub max_connections: usize,
}

/// Server, holds configuration, handlers, state and addr it listens on
#[must_use]
pub struct Jello<'server, S> {
    conf: Conf,
    handlers: Vec<(&'server str, Handle<S>)>,
    state: Option<S>,
    addr: Option<SocketAddr>,
}

// this impl serves the purpose of omiting Jello::<()>::new when constructing.
impl<'server> Jello<'server, ()> {
    pub fn new() -> Self {
        Self {
            conf: Conf {
                threads: 1,
                memory_per_request: 4 * 1024,
                memory_total: 4 * 1024 * 1024 * 1024,
                max_connections: 10000,
            },
            handlers: Vec::with_capacity(8),
            state: None,
            addr: None,
        }
    }

    pub fn with_configuration(conf: Conf) -> Self {
        Self {
            conf,
            handlers: Vec::with_capacity(8),
            state: None,
            addr: None,
        }
    }
}

impl<'server, S> Jello<'server, S> {
    pub fn with_state(state: S) -> Self {
        Self {
            conf: Conf {
                threads: 1,
                memory_per_request: 4 * 1024,
                memory_total: 4 * 1024 * 1024 * 1024,
                max_connections: 10000,
            },
            handlers: Vec::with_capacity(8),
            state: Some(state),
            addr: None,
        }
    }

    pub fn add_config(mut self, conf: Conf) -> Self {
        self.conf = conf;
        self
    }

    /// register [handle] to be executed if a request to [path] is made
    pub fn handle(mut self, path: &'server str, handle: Handle<S>) -> Self {
        self.handlers.push((path, handle));
        self
    }

    // TODO: think about a good error enum here, but that depends on Runtime::run's impl
    pub fn listen(mut self, addr: SocketAddr) -> Result<(), err::Error> {
        let runtime = Runtime {
            state: self.state.as_mut(),
            handlers: &self.handlers,
            conf: &self.conf,
        };
        runtime.run(addr)
    }
}
