use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i_thread in 0..2 {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);
        handles.push(thread::spawn(move || {
            if i_thread % 2 == 0 {
                let mut num_a = a.lock().unwrap();
                *num_a += 1;
                
                println!("thread a locked mutex a");

                thread::sleep(Duration::from_secs(1));

                let mut num_b = b.lock().unwrap();
                *num_b += 1;

                println!("thread a locked mutex b");
            } else {
                // causes the deadlock
                let mut num_b = b.lock().unwrap();
                *num_b += 1;

                println!("thread b locked mutex b");

                let mut num_a = a.lock().unwrap();
                *num_a += 1;

                println!("thread b locked mutex a");
            }
        }));
    }

    // will never unblock the thread
    for handle in handles {
        handle.join().unwrap();
    }
}
