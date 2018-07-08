use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::fs::File;


fn find(key: &str) -> io::Result<String> {
    let file = File::open("db.txt")?;
    let file = BufReader::new(file);
    let mut results: Vec<String> = Vec::new();
    for line in file.lines() {
        let line = line?;
        if line.starts_with(&format!("{},", key)) {
            results.push(line);
        }
    }
    results.last()
        .map(|s| s.to_owned())
        .ok_or(Error::new(ErrorKind::NotFound, "No match found."))
}

pub fn new(key: &str) -> io::Result<String> {
    println!("Get: {}", key);
    let line = find(key)?;
    let value: String = {
        let index = line.find(",")
            .ok_or(Error::new(ErrorKind::InvalidData, "DB Error"))?;
        line[index+1..].to_owned()
    };
    Ok(value)
}
