use std::io::prelude::*;
use std::net::TcpStream;

pub fn run(port: i32) {
    let mut stream = TcpStream::connect(format!("localhost:{}", port)).unwrap();
    stream.write_all(b"hello world").unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    println!("{}", response);
}
