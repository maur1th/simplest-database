use std::io::prelude::*;
use std::io::{BufReader, SeekFrom, Result};
use std::fs::File;


fn find(log_file: &str, key: &str) -> Option<u64> {
    let log = File::open(log_file).expect("Could not open log file.");
    let mut reader = BufReader::new(log);
    let mut line = String::new();
    let mut pos = 0;
    let mut res = None;
    while let Ok(num) = reader.read_line(&mut line) {
        if num == 0 {
            break;
        }
        if line.starts_with(&format!("{},", key)) {
            res = Some(pos);
        }
        pos += num as u64;
        line.clear();
    }
    res
}

fn get_offset(index: &super::Index, key: &str) -> Option<u64> {
    let index = index.inner.lock().expect("Poisoned lock.");
    let res = (*index).get(key).map(|v| v.to_owned());
    res.map(|x| {
        println!("match");
        x
    })
}

fn read(log: &mut File, offset: u64) -> String {
    log.seek(SeekFrom::Start(offset)).expect("Error reading log.");
    let mut reader = BufReader::new(log);
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("Could not read log.");
    buf
}

fn get_result(log_file: &str, offset: u64) -> Result<String> {
    let mut log = File::open(log_file)?;
    let line = read(&mut log, offset);
    let index = line.find(",").expect("Log is corrupted.");
    Ok(line[index+1..].to_owned())
}

pub fn new(db: super::Database, key: &str) -> Result<String> {
    let log_file = "db.txt";
    println!("Get: {}", key);
    let offset = get_offset(&db.index, key)
        .or_else(|| find(log_file, key).map(|offset| {
            db.index.update(key, offset);
            offset
        }));
    if let Some(offset) = offset {
        get_result(log_file, offset)
    } else {
        Ok(format!("No match for {}.", key))
    }
}
