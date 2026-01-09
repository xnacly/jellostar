pub struct Response<'res> {
    buf: &'res mut [u8],
    pos: usize,
}

impl<'res> Response<'res> {
    pub fn write(&mut self, b: &[u8]) -> bool {
        let remaining = self.buf.len() - self.pos;
        if b.len() > remaining {
            return false;
        }

        self.buf[self.pos..self.pos + b.len()].copy_from_slice(b);
        self.pos += b.len();
        true
    }

    pub fn write_str(&mut self, s: &'res str) -> bool {
        self.write(s.as_bytes())
    }
}
