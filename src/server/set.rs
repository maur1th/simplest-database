use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::fs::OpenOptions;

struct Item {
    key: String,
    value: String,
}

fn parse(params: Vec<&str>) -> io::Result<Item> {
    let params_string: Vec<String> = params.into_iter()
        .map(|s| s.to_owned())
        .collect();
    match params_string.as_slice() {
        [key, value] => Ok(Item {key: key.to_owned(), value: value.to_owned()}),
        _ => Err(Error::new(ErrorKind::InvalidInput, "wrong number of arguments")),
    }
}

fn write(item: &Item) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open("db.txt")?;
    file.write_all(format!("{},{}\n", item.key, item.value).as_bytes())?;
    Ok(())
}

pub fn new(params: Vec<&str>) -> io::Result<String> {
    let item = parse(params)?;
    println!("Set: {}: {}", &item.key, &item.value);
    write(&item)?;
    Ok(format!("{} {}", &item.key, &item.value))
}
