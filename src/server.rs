use std::io::prelude::*;
use std::str;
use std::thread;
use std::net::{TcpListener,TcpStream};
use std::process::exit;

struct Item {
    key: String,
    value: String,
}

fn set(params: Vec<&str>) {
    let item = Item {key: params[0].to_string(), value: params[1].to_string()};
    println!("set: {} {}", item.key, item.value);
}

fn get(key: &str) {
    println!("get {}", key);
}

fn handle_client(mut stream: TcpStream) {
    println!("Got a connection: {:?}", stream);
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let mut msg = str::from_utf8(&buffer).unwrap().split(",");
    let action = msg.next().expect("Missing argument");
    let params: Vec<&str> = msg.collect();
    match action.as_ref() {
        "set" => set(params),
        "get" => get(params[0]),
        _ => panic!("Unknown argument: {}", action),
    };
    stream.write_all(b"done").unwrap();
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
