use std::env;
use std::path::Path;

mod server;
mod client;

fn main() {
    let mut args = env::args().skip(1);
    let mode = args.next().expect("Missing argument");
    let socket = Path::new("/tmp/simplest-db.sock");
    match mode.as_ref() {
        "server" => server::run(socket),
        "client" => client::run(socket),
        _ => panic!("Unknown argument: {}", mode),
    };
}
