#[derive(Debug)]
pub enum Error {
    TcpListenerError(&'static str),
    TcpStreamError(&'static str),
}
