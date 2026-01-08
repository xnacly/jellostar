pub struct Query<'r> {
    pairs: &'r [(&'r [u8], &'r [u8])],
}

impl<'q> Query<'q> {
    pub fn set(&mut self, name: &'q [u8], val: &'q [u8]) {
        todo!("Query::set")
    }

    pub fn get(&mut self, name: &'q [u8]) -> Option<&'q [u8]> {
        todo!("Query::get")
    }
}
