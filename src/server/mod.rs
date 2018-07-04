use std::io::prelude::*;
use std::str;
use std::thread;
use std::net::{TcpListener,TcpStream};
use std::process::exit;

mod set;
mod get;


fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let mut msg = str::from_utf8(&buffer)
        .unwrap()
        .trim_matches(char::from(0))
        .split(",");
    let action = msg.next().expect("Missing argument");
    let params: Vec<&str> = msg.collect();
    let res: String = match action.as_ref() {
        "set" => set::new(params).to_owned(),
        "get" => get::new(params[0]).to_owned(),
        _ => format!("Unknown argument: {}", action),
    };
    stream.write_all(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn run(port: i32) {
    println!("Starting server on port {}...", port);
    let listener = match TcpListener::bind(format!("localhost:{}", port)) {
        Err(e) => {
            eprintln!("Could not bind listener due to {}", e);
            exit(1);
        }
        Ok(listener) => {
            println!("Server started");
            listener
        }
    };
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                /* connection succeeded */
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                /* connection failed */
                println!("Connection failed: {:?}", err);
                break;
            }
        }
    }
}
