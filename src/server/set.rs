use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::fs::OpenOptions;

struct Item {
    key: String,
    value: String,
}

fn parse(params: &[&str]) -> io::Result<Item> {
    match params {
        [key, value] => Ok(Item {key: key.to_string(), value: value.to_string()}),
        _ => Err(Error::new(ErrorKind::InvalidInput, "wrong number of arguments")),
    }
}

fn write(item: &Item) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open("db.txt")?;
    file.write_all(format!("{},{}\n", item.key, item.value).as_bytes())?;
    Ok(())
}

pub fn new(params: &[&str]) -> io::Result<String> {
    let item = parse(params)?;
    println!("Set: {}: {}", &item.key, &item.value);
    write(&item)?;
    Ok(format!("{} {}", &item.key, &item.value))
}
