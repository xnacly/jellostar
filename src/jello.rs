use crate::Handle;

/// user facing configuration
pub struct JelloConf {
    pub threads: u8,
    pub memory_per_request: usize,
    pub memory_total: usize,
}

/// Jello is the server
pub struct Jello<'server, S> {
    conf: JelloConf,
    handlers: Vec<(&'server str, Handle<S>)>,
    state: Option<S>,
}

impl<'server, S> Jello<'server, S> {
    pub fn new() -> Self {
        Self {
            conf: JelloConf {
                // by default we single thread
                threads: 1,
                // by default we give each request a 4KB chunk of memory
                memory_per_request: 4 * 1024,
                // in general the server is never allowed to exceed 4GB mem usage
                memory_total: 4 * 1024 * 1024 * 1024,
            },
            handlers: Vec::with_capacity(8),
            state: None,
        }
    }

    pub fn with_configuration(conf: JelloConf) -> Self {
        Self {
            conf,
            handlers: Vec::with_capacity(8),
            state: None,
        }
    }

    pub fn state(&'server mut self, state: S) -> &'server mut Self {
        self.state = Some(state);
        self
    }

    /// register [handle] to be executed if a request to [path] is made
    pub fn handle(&'server mut self, path: &'server str, handle: Handle<S>) -> &'server mut Self {
        // TODO:
        self
    }
}
