use std::io::{self, BufReader, Error, ErrorKind};
use std::io::prelude::*;
use std::fs::File;

fn match_line<'a>(key: &str, line: &'a str) -> Option<&'a str> {
    if line.starts_with(&format!("{},", key)) {
        println!("Match");
        return Some(line)
    }
    None
}

fn get_line(key: &str) -> io::Result<String> {
    let file = File::open("db.txt")?;
    let file = BufReader::new(file);
    for line in file.lines() {
        if let Some(result) = match_line(key, &line.unwrap()) {
            return Ok(result.to_owned())
        }
    }
    Err(Error::new(ErrorKind::NotFound, "No match found."))
}

pub fn new(key: &str) -> String {
    println!("get {}", key);
    let result = match get_line(key) {
        Ok(result) => result,
        Err(error) => return error.to_string()
    };
    let index = result.find(",").expect("DB Error") + 1;
    result[index..].to_owned()
}
