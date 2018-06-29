// use std::fs::OpenOptions;

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

pub fn new(params: Vec<&str>) -> &str {
    let item = match parse(params) {
        Ok(item) => item,
        Err(err) => return err
    };
    println!("set: {} {}", item.key, item.value);
    "done"
}
