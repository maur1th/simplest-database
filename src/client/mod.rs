use std::io::prelude::*;
use std::io;
use std::net::TcpStream;

fn query(msg: &[String], port: i32) -> io::Result<String> {
    let mut stream = TcpStream::connect(format!("localhost:{}", port))?;
    stream.write_all(msg.join(",").as_bytes())?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
}

pub fn run(msg: &[String], port: i32) {
    let res = query(msg, port).expect("Connection error.");
    println!("{}", res);
}
