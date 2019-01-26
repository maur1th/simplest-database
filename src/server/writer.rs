use std::thread;
use std::sync::mpsc::channel;

fn req_write(contents) {
    let (tx, rx) = channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move|| {
            tx.send(i).unwrap();
        });
    }
}
