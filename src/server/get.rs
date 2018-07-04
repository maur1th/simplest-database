use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


fn find(key: &str) -> Option<String> {
    let file = File::open("db.txt")
        .expect("Could not create / open db file.");
    let file = BufReader::new(file);
    let mut matches = file.lines().filter_map(|line| {
        let line = line.expect("Error reading db");
        match line.starts_with(&format!("{},", key)) {
            true => Some(line),
            false => None
        }
    });
    matches.next()
}

pub fn new(key: &str) -> String {
    println!("Get: {}", key);
    find(key).map_or(String::from("No match found."), |r| {
        let index = r.find(",").expect("DB Error");
        r[index+1..].to_owned()
    })
}
