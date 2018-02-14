use std::env;
use std::path::Path;

mod server;
mod client;

fn main() {
    let args: Vec<String> = env::args().collect();
    let socket = Path::new("/tmp/simplest-db.sock");
    match args.get(1).expect("Missing argument").as_ref() {
        "server" => server::run(socket),
        "client" => client::run(socket),
        _ => panic!("Unknown argument: {}", args[1]),
    };
}
