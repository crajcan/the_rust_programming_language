#![allow(unused_variables)]

use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn practice_blocking_with_handle_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("counted to {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..4 {
        println!("counted to {} from the MAIN thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn example_of_move_invalidating_borrow_in_closure() {
    let v = vec![1, 2, 3];

    let f = || {
        println!("Here's a vector: {:?}", v);
    };

    // here we move a previously borrowed value, thus invalidating the borrow
    // drop(v);

    // if we try to use the borrow again:
    // cannot move out of `v` because it is borrowed
    f();
}

fn example_of_closure_outliving_captured_borrow() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        // Since we're passing the closure to a new thread, we can't tell if the
        // closure will be called after `v` is moved by the main thread, so we
        // can't guarantee the closure's reference to v will be valid as long as it is
        // println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn using_move_to_capture_owned_value_in_thread() {
    let v = vec![1, 2, 3];

    // here we use move to force the closure to take ownership of `v` before
    // it is passed to the new thread
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn use_channels() {
    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");

        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn move_over_channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");

        tx.send(val).unwrap();
        // used after move
        // println!("val: {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn move_reference_over_channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");

        // We can't send a reference &val because val will be dropped at the
        // end of this thread, so the borrow we send will outlive the value
        // tx.send(&val).unwrap();
        // println!("val: {}", val);

        // the solution is to just transfer ownership
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn sending_multiple_messages() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("producer"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    // rx impl Iterator
    for received in rx {
        println!("Got: {}", received);
    }
}

fn multiple_producers() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    println!("\ntrying multiple producers\n");

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("producer"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
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
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    // rx impl Iterator
    for received in rx {
        println!("Got: {}", received);
    }
}

fn use_mutexes() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6
    }

    println!("m = {:?}", m);
}

fn share_data_between_two_threads_with_mutexes() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    let first_counter_clone = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = first_counter_clone.lock().unwrap();

        *num += 1;
    });

    handles.push(handle);

    let second_counter_clone = Arc::clone(&counter);
    let handle2 = thread::spawn(move || {
        let mut num2 = second_counter_clone.lock().unwrap();

        *num2 += 1;
    });

    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn share_data_between_many_threads_with_mutexes() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn fix_a_deadlock() {
    let strikes = Arc::new(Mutex::new(0));
    let outs = Arc::new(Mutex::new(0));

    let strikes_clone = Arc::clone(&strikes);
    let outs_clone = Arc::clone(&outs);

    let score_book_keeper = thread::spawn(move || {
        let mut out_count = outs_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(200));

        // waits for strikes lock before giving up outs lock
        let mut strike_count = strikes_clone.lock().unwrap();

        *strike_count += 1;
        *out_count += 1;
    });

    let mut strike_count = strikes.lock().unwrap();
    thread::sleep(Duration::from_millis(200));

    *strike_count += 1;

    // waits for outs lock before giving up strikes lock
    println!(
        "strikes: {}, outs: {}",
        *strike_count,
        *outs.lock().unwrap()
    );
}

fn main() {
    practice_blocking_with_handle_join();

    example_of_move_invalidating_borrow_in_closure();

    example_of_closure_outliving_captured_borrow();

    using_move_to_capture_owned_value_in_thread();

    use_channels();

    move_over_channels();

    move_reference_over_channels();

    sending_multiple_messages();

    multiple_producers();

    use_mutexes();

    share_data_between_two_threads_with_mutexes();

    share_data_between_many_threads_with_mutexes();

    fix_a_deadlock();
}
