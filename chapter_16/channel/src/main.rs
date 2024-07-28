use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel(); // multiple producer, single consumer
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap(); // send takes ownership, then receiver
            thread::sleep(Duration::from_secs(5));
        }
    });

    thread::spawn(move || { // why concurrency is challenging
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap(); // send takes ownership, then receiver
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx { // we can treat rx like an iterator
        println!("got: {received}");
    } // because we don't have any code that pauses or delays, we know that the delay in our spawned thread works
}
