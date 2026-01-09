use std::io::Write;
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{io, net::SocketAddr};

use crate::{Handle, err};

pub struct Runtime<'r, S> {
    pub handlers: &'r [(&'r str, Handle<S>)],
    pub state: Option<&'r mut S>,
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

impl<'r, S> Runtime<'r, S> {
    pub fn run(self, addr: SocketAddr) -> Result<(), err::Error> {
        setup_signal_handler();

        let Ok(listener) = TcpListener::bind(addr) else {
            return Err(err::Error::TcpListenerError("failed to bind to addr"));
        };

        if listener.set_nonblocking(true).is_err() {
            return Err(err::Error::TcpListenerError(
                "failed to move into nonblocking mode",
            ));
        }

        dbg!("started");

        while RUNNING.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let _ = stream.set_nonblocking(false);

                    // TODO: replace this with a free buffer from the preallocated buffer list, if
                    // there is none, we close the connction
                    let mut buf = [0; 1024];

                    match io::Read::read(&mut stream, &mut buf) {
                        Ok(_) => {
                            // TODO: parse headers from bytes
                            // TODO: extract body from bytes via offset
                            // TODO: construct Request and Response
                            // TODO: call Handle
                            // TODO: build http response bytes

                            // TODO: remove this once we are further
                            let response =
                                b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\nConnection: close\r\n\r\nHello World";
                            if let Err(_) = io::Write::write_all(&mut stream, response) {
                                // INFO: we go to the next connection if we cant write; we just
                                // skip it
                                dbg!("failed to write");
                                continue;
                            }

                            let _ = stream.flush();
                            let _ = stream.shutdown(std::net::Shutdown::Both);
                        }
                        Err(_) => {
                            // INFO: we go to the next connection if we cant read; we just
                            // skip it, should we close?
                            continue;
                        }
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // we have got no request, ig we loop
                    continue;
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
