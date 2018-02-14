use std::env;

extern crate server;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).expect("Missing argument").as_ref() {
        "server" => server::run(),
        _ => panic!("Unknown argument: {}", args[1]),
    };
}
