use std::path::Path;
use std::fs;
use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::process::exit;

fn handle_client(stream: UnixStream) {
    println!("Got a connection: {:?}", stream);
}

pub fn run() {
    println!("Starting server");
    let socket = Path::new("/tmp/simplest-db.sock");
    if socket.exists() {
        println!("Removing previous socket file");
        fs::remove_file(socket).unwrap();
    }
    let listener = match UnixListener::bind(socket) {
        Err(e) => {
            eprintln!("Could not bind listener due to {}", e);
            exit(1);
        }
        Ok(listener) => listener,
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
