// Concurrent programming, in which different parts of a program execute independently,
// and parallel programming, in which different parts of a program execute at the same time,
// are becoming increasingly important as more computers take advantage of their multiple processors.

// Using Threads to Run Code Simultaneously

// Splitting the computation in your program into multiple threads to run multiple tasks at the same
// time can improve performance, but it also adds complexity. Because threads can run simultaneously,
// there’s no inherent guarantee about the order in which parts of your code on different threads will
// run. This can lead to problems, such as:

// Race conditions, in which threads are accessing data or resources in an inconsistent order
// Deadlocks, in which two threads are waiting for each other, preventing both threads from continuing
// Bugs that only happen in certain situations and are hard to reproduce and fix reliably

//  creating a new thread with spawn

use std::{ sync::{ Arc, Mutex, mpsc }, thread, time::Duration };

pub fn run() {
    // The calls to thread::sleep force a thread to stop its execution for a short duration,
    // allowing a different thread to run. The threads will probably take turns, but that isn’t
    // guaranteed: it depends on how your operating system schedules the threads.

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // We can fix the problem of the spawned thread not running or of it ending prematurely by saving
    // the return value of thread::spawn in a variable. The return type of thread::spawn is
    // JoinHandle<T>. A JoinHandle<T> is an owned value that, when we call the join method on it, will
    // wait for its thread to finish.

    handle.join().unwrap(); // if handle is added before the for loop of main thread, it will wait for the spawned thread to complete its execution

    // The two threads continue alternating, but the main thread waits because of the call to handle.
    // join() and does not end until the spawned thread is finished.
}

// using move closures with threads

// We’ll often use the move keyword with closures passed to thread::spawn because the closure will
// then take ownership of the values it uses from the environment, thus transferring ownership of
// those values from one thread to another

// pub fn run1() {
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(|| {
//         println!("Here's a vector: {v:?}");
//     });

//     handle.join().unwrap();
// }

pub fn run1() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}

// Using Message Passing to Transfer Data Between Threads

// One increasingly popular approach to ensuring safe concurrency is message passing,
// where threads or actors communicate by sending each other messages containing data.

// To accomplish message-sending concurrency, Rust’s standard library provides an implementation of channels. A channel is a general programming concept by which data is sent from one thread to another.

// You can imagine a channel in programming as being like a directional channel of water, such as a stream or a river. If you put something like a rubber duck into a river, it will travel downstream to the end of the waterway.

// A channel has two halves: a transmitter and a receiver. The transmitter half is the upstream location where you put the rubber duck into the river, and the receiver half is where the rubber duck ends up downstream. One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages. A channel is said to be closed if either the transmitter or receiver half is dropped.

pub fn run2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {val}");
        // borrow of moved value: `val`
        // value borrowed here after move
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
    // Got: hi
}

// Sending Multiple Values and Seeing the Receiver Waiting
pub fn run3() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

// Shared-State Concurrency
// Do not communicate by sharing memory.

// In a way, channels in any programming language are similar to single ownership because once you transfer a value down a channel, you should no longer use that value. Shared-memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time.

// Using Mutexes to Allow Access to Data from One Thread at a Time

// Mutex is an abbreviation for mutual exclusion, as in a mutex allows only one thread to access
// some data at any given time. To access the data in a mutex, a thread must first signal that it
// wants access by asking to acquire the mutex’s lock. The lock is a data structure that is part of
// the mutex that keeps track of who currently has exclusive access to the data. Therefore, the
// mutex is described as guarding the data it holds via the locking system.

// Mutexes have a reputation for being difficult to use because you have to remember two rules:

// - You must attempt to acquire the lock before using the data.
// - When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

// For a real-world metaphor for a mutex, imagine a panel discussion at a conference with only one
// microphone. Before a panelist can speak, they have to ask or signal that they want to use the
// microphone. When they get the microphone, they can talk for as long as they want to and then hand
// the microphone to the next panelist who requests to speak. If a panelist forgets to hand the
// microphone off when they’re finished with it, no one else is able to speak. If management of the
// shared microphone goes wrong, the panel won’t work as planned!

// The API of Mutex<T>

pub fn run4() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}

// A Mutex<T> lets multiple threads safely share and modify data.
// We must call `.lock()` to access the inner value, which blocks until
// this thread gets the lock. `lock()` returns a MutexGuard, which acts
// like a mutable reference to the data and automatically releases the
// lock when it goes out of scope. This ensures only one thread updates
// the data at a time and prevents us from forgetting to unlock it.

// Sharing a Mutex<T> Between Multiple Threads  (Atomic Reference Counting with Arc<T>)

pub fn run5() {
    // Shared counter wrapped in Arc + Mutex so multiple threads can access it
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..100 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // Lock the mutex to safely update the shared counter
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

// Multiple threads update a shared counter. We wrap the counter
// in Arc<Mutex<T>> so it can be safely shared across threads.
// Each thread locks the mutex, increments the value, and the
// lock is automatically released when the guard goes out of scope.

// Mutex<T>: allows safe mutation
// Arc<T>: allows multiple threads to own the same data
// Together → safe shared mutation across threads.
