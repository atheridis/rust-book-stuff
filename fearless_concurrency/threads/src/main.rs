use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // one();
    // two();
    // three();
    four();
}

fn one() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
    println!("here");
}

fn two() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn three() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        thread::sleep(Duration::from_millis(2000));
        tx.send(val).unwrap();
        // println!("val is = {}", val);
    });

    println!("here");
    let received = rx.recv().unwrap();
    println!("there");
    println!("Got: {}", received);
}

fn four() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(700));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for received in rx {
        // println!("Got: {}", 1);
        println!("Got: {}", received);
        // println!("Got: {}", 2);
    }
}
