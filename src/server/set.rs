use std::io::prelude::*;
use std::io::{SeekFrom, Result};
use std::fs::{OpenOptions};


fn write(item: &super::Item, log_file: &str) -> u64 {
    let mut log = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file)
        .expect("Could not open log file.");
    let bytes = item.to_buf();
    println!("Set: {}: {}", &item.key, &item.value);
    log.write_all(bytes.as_slice()).expect("Could not write to log.");
    let current_offset = log.seek(SeekFrom::Current(0))
        .expect("Error reading log.");
    current_offset - bytes.len() as u64
}

pub fn new(index: super::Index, params: &[&str]) -> Result<String> {
    let item = super::Item::new(params)?;
    let offset = write(&item, "db.txt");
    index.update(&item.key, offset);
    Ok(format!("{} {}", &item.key, &item.value))
}
