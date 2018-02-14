use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::path::Path;

pub fn run(socket: &Path) {
    let mut stream = UnixStream::connect(socket).unwrap();
    stream.write_all(b"hello world").unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    println!("{}", response);
}
