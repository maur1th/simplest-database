use std::env;

mod server;
mod client;

fn main() {
    let mut args = env::args().skip(1);
    let mode = args.next().expect("Missing argument");
    let port = 5432;
    match mode.as_ref() {
        "server" => server::run(port),
        "client" => client::run(port),
        _ => panic!("Unknown argument: {}", mode),
    };
}
