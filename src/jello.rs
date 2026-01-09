use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use crate::Handle;

/// user facing configuration
pub struct Conf {
    pub threads: u8,
    pub memory_per_request: usize,
    pub memory_total: usize,
}

/// Jello is the server
#[must_use]
pub struct Jello<'server, S> {
    conf: Conf,
    handlers: Vec<(&'server str, Handle<S>)>,
    state: Option<S>,
    addr: Option<SocketAddr>,
}

struct Runtime<'r, S> {
    handlers: &'r [(&'r str, Handle<S>)],
    state: Option<&'r mut S>,
}

#[macro_export]
macro_rules! addr {
    (($a:literal, $b:literal, $c:literal, $d:literal):$port:literal) => {{
        use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new($a, $b, $c, $d), $port))
    }};
    (($a:literal, $b:literal, $c:literal, $d:literal, $e:literal, $f:literal, $g:literal, $h:literal) : $port:literal) => {{
        use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
        SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::new($a, $b, $c, $d, $e, $f, $g, $h),
            $port,
            0,
            0,
        ))
    }};
}

#[macro_export]
/// Call [$call], if it returns false, return InternalServerError
macro_rules! or500 {
    ($call:expr) => {
        if !$call {
            return crate::Status::InternalServerError;
        }
    };
}

impl<'server> Jello<'server, ()> {
    pub fn new() -> Self {
        Self {
            conf: Conf {
                threads: 1,
                memory_per_request: 4 * 1024,
                memory_total: 4 * 1024 * 1024 * 1024,
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
    pub fn listen(mut self, addr: SocketAddr) -> Result<(), ()> {
        let runtime = Runtime {
            state: self.state.as_mut(),
            handlers: &self.handlers,
        };
        runtime.run(addr)
    }
}

impl<'r, S> Runtime<'r, S> {
    pub fn run(self, remote: SocketAddr) -> Result<(), ()> {
        // todo!("this is new stuff, how does this work")
        Ok(())
    }
}
