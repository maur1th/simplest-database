use std::io;
use std::io::prelude::*;
use std::fs::OpenOptions;

struct Item {
    key: String,
    value: String,
}

fn parse(params: Vec<&str>) -> Result<Item, &str> {
    let params_string: Vec<String> = params.into_iter()
        .map(|s| s.to_owned())
        .collect();
    match params_string.as_slice() {
        [key, value] => Ok(Item {key: key.to_owned(), value: value.to_owned()}),
        _ => Err("wrong number of arguments")
    }
}


fn write(item: &Item) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open("db.txt")?;
    file.write_all(format!("{},{}\n", item.key, item.value).as_bytes())?;
    Ok(())
}

pub fn new(params: Vec<&str>) -> String {
    let item = match parse(params) {
        Ok(item) => item,
        Err(err) => return err.to_owned()
    };
    println!("set: {} {}", &item.key, &item.value);
    match write(&item) {
        Ok(()) => format!("{} {}", &item.key, &item.value),
        Err(e) => e.to_string()
    }
}
