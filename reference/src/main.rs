fn main() {
    jellostar::Jello::new()
        .handle("/", |_, _, res| {
            res.write_str("Hello World");
            jellostar::Status::Ok
        })
        .listen(jellostar::addr!((0,0,0,0):1234))
        .expect("Failed to start server");
}
