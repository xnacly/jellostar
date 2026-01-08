use crate::{header::Header, method::Method, query::Query};

pub struct Request<'r> {
    method: Method,
    path: &'r [u8],
    headers: &'r [Header<'r>],
    body: &'r [u8],
    query: Option<Query<'r>>,
}

impl<'r> Request<'r> {
    pub fn method(&self) -> Method {
        self.method
    }

    pub fn path(&self) -> &'r [u8] {
        self.path
    }

    pub fn path_str(&self) -> &'r str {
        std::str::from_utf8(self.path).unwrap_or_else(|_err| "")
    }

    pub fn body(&self) -> &'r [u8] {
        self.body
    }

    pub fn headers(&self) -> &'r [Header<'r>] {
        self.headers
    }

    pub fn header(&self, name: &[u8]) -> Option<&'r [u8]> {
        self.headers
            .iter()
            .find(|h| h.name.eq_ignore_ascii_case(name))
            .map(|h| h.value)
    }

    pub fn query(&self) -> Option<&Query<'r>> {
        self.query.as_ref()
    }
}
