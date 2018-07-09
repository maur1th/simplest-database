use std::{str, thread};
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::net::{TcpListener,TcpStream};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

mod set;
mod get;

#[derive(Clone)]
pub struct Index {
    inner: Arc<Mutex<HashMap<String, u64>>>,
}

impl Index {
    pub fn new() -> Index {
        Index { inner: Arc::new(Mutex::new(HashMap::new())) }
    }

    fn update(&self, key: &str, offset: u64) {
        let mut index = self.inner.lock().expect("Poisoned lock.");
        (*index).insert(key.to_owned(), offset);
    }
}


#[derive(Debug)]
struct Item {
    key: String,
    value: String,
}

impl Item {
    pub fn new(params: &[&str]) -> Result<Item> {
        match params {
            [key, value] => Ok(Item {key: key.to_string(), value: value.to_string()}),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Wrong number of arguments.")),
        }
    }

    fn to_buf(&self) -> Vec<u8> {
        format!("{},{}\n", self.key, self.value)
            .as_bytes()
            .to_owned()
    }
}


fn handle_routes(index: Index, action: &str, params: &[&str]) -> Result<String> {
    match action {
        "set" => set::new(index, &params),
        "get" => get::new(index, params[0]),
        _ => Err(Error::new(ErrorKind::InvalidInput, format!("Unknown argument: {}", action))),
    }
}

fn handle_client(index: Index, mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let mut msg = str::from_utf8(&buffer)
        .unwrap()
        .trim_matches(char::from(0))
        .split(",");
    let action = msg.next().expect("Missing argument");
    let params: Vec<&str> = msg.collect();
    let res: String = match handle_routes(index, action, &params) {
        Ok(res) => res,
        Err(err) => {
            println!("Got an error: {}", err);
            format!("{:?}", err)
        }
    };
    stream.write_all(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn run(port: i32) {
    let index = Index::new();
    println!("Starting server on port {}...", port);
    let listener = TcpListener::bind(format!("localhost:{}", port))
        .expect("Could not bind listener.");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let i2 = index.clone();
                thread::spawn(move || handle_client(i2, stream));
            }
            Err(err) => {
                println!("Connection failed: {:?}", err);
                break;
            }
        }
    }
}
