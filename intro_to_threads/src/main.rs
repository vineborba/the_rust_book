use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        // by default, spawned threads are killed when the main thread
        //  finishes execution, even if there is code running
        for i in 1..20 {
            println!("current number {} in the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // this prevents the default behaviour
    // if called here, will wait until code in the thread is executed
    // to continue the program
    // handle.join().unwrap();

    for i in 1..10 {
        println!("current number {} in the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    // Other way to make this in Rust version 1.63!
    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle2.join().unwrap();
}
