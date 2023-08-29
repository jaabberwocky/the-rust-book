use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct ConcurrencyRunner;

impl ConcurrencyRunner {
    pub fn run_threads() {
        let mut v: Vec<i32> = Vec::new();

        let handle = thread::spawn(move || {
            for i in 1..10 {
                v.push(i);
                println!("spawned thread - v: {:?}", v);
                thread::sleep(Duration::from_millis(1000));
            }
        });

        for i in 1..5 {
            println!("main thread - i: {}", i);
            thread::sleep(Duration::from_millis(500));
        }

        handle.join().unwrap();
    }

    pub fn msg_passing() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000));
            let val = String::from("hi");
            tx.send(val).unwrap();
            // println!("val is {}", val); // error: value borrowed here after move
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    pub fn send_multiple_messages() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    pub fn multiple_producer_multiple_messages() {
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        let tx2 = tx.clone();
        let tx3 = tx.clone();

        // if you don't do this, the main thread won't stop!
        // because rx will only stop when all senders are dropped
        drop(tx);

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send("thread1:".to_string() + &val).unwrap();
                thread::sleep(Duration::from_millis(100));
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
                tx2.send("thread2:".to_string() + &val).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("here"),
                String::from("are"),
                String::from("even"),
                String::from("more"),
                String::from("messages"),
            ];

            for val in vals {
                tx3.send("thread3:".to_string() + &val).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    pub fn simple_mutex() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }

    pub fn complex_mutex() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for i in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                println!("thread {} spawned", i);
                let mut num = counter.lock().unwrap();

                *num += i;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}
