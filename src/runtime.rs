use std::io::Write;
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{io, net::SocketAddr};

use crate::{Conf, Handle, err};

pub struct Runtime<'r, S> {
    pub handlers: &'r [(&'r str, Handle<S>)],
    pub state: Option<&'r mut S>,
    pub conf: &'r Conf,
}

static RUNNING: AtomicBool = AtomicBool::new(true);

extern "C" fn handle_sigint(_: i32) {
    RUNNING.store(false, Ordering::SeqCst);
}

fn setup_signal_handler() {
    unsafe {
        libc::signal(libc::SIGINT, handle_sigint as usize);
    }
}

enum ConnState {
    ReadingHeaders,
    ReadingBody {
        content_len: usize,
        body_read: usize,
    },
    WritingResponse {
        writte: usize,
        total: usize,
    },
    Done,
}

struct Connection<'c> {
    stream: std::net::TcpStream,
    state: ConnState,
    read_buf: &'c mut [u8],
    write_buf: &'c mut [u8],
    len: usize,
}

impl<'r, S> Runtime<'r, S> {
    pub fn run(self, addr: SocketAddr) -> Result<(), err::Error> {
        setup_signal_handler();

        // TODO: replace this with a preallocated memory region
        let mut connections = vec![const { None }; self.conf.max_connections];

        let Ok(listener) = TcpListener::bind(addr) else {
            return Err(err::Error::TcpListenerError("failed to bind to addr"));
        };

        if listener.set_nonblocking(true).is_err() {
            return Err(err::Error::TcpListenerError(
                "failed to move into nonblocking mode",
            ));
        }

        // TODO: parse headers from bytes
        // TODO: extract body from bytes via offset
        // TODO: construct Request and Response
        // TODO: call Handle
        // TODO: build http response bytes

        while RUNNING.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((stream, _addr)) => {
                    let _ = stream.set_nonblocking(true);
                    // TODO: replace this with a free buffer from the preallocated buffer list, if
                    // there is none, we close the connction
                    connections.push(Some(()));
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // we got no request, do nothin
                }
                Err(_) => {
                    return Err(err::Error::TcpListenerError(
                        "failed to accept a connection",
                    ));
                }
            }
        }

        Ok(())
    }
}
