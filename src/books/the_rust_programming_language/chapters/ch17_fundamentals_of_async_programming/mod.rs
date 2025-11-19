// Parallelism and Concurrency

// When an individual works on several different tasks before any of them is complete, this is concurrency.

// When the team splits up a group of tasks by having each member take one task and work on it alone, this is parallelism. Each person on the team can make progress at the exact same time

// On a machine with a single CPU core, the CPU can perform only one operation at a time, but it can
// still work concurrently. Using tools such as threads, processes, and async, the computer can pause
// one activity and switch to others before eventually cycling back to that first activity again. On a
// machine with multiple CPU cores, it can also do work in parallel. One core can be performing one
// task while another core performs a completely unrelated one, and those operations actually happen
// at the same time.

// When working with async in Rust, we’re always dealing with concurrency.
// Depending on the hardware, the operating system, and the async runtime we are using
// (more on async runtimes shortly), that concurrency may also use parallelism under the hood.

/* //////////////////////////////////////////////////////////////
                        FUTURES AND ASYNC SYNTAX
////////////////////////////////////////////////////////////// */

// The key elements of asynchronous programming in Rust are futures and Rust’s async and await keywords.

// A future is a value that may not be ready now but will become ready at some point in the future.

// Rust provides a Future trait as a building block so that different async operations can be
// implemented with different data structures but with a common interface. In Rust, futures are types
// that implement the Future trait. Each future holds its own information about the progress that has
// been made and what “ready” means.

// You can apply the async keyword to blocks and functions to specify that they can be interrupted and
// resumed. Within an async block or async function, you can use the await keyword to await a future
// (that is, wait for it to become ready). Any point where you await a future within an async block or
// function is a potential spot for that async block or function to pause and resume. The process of
// checking with a future to see if its value is available yet is called polling.

// When writing async Rust, we use the async and await keywords most of the time. Rust compiles them into equivalent code using the Future trait, much as it compiles for loops into equivalent code using the Iterator trait.

use std::{ pin::{ Pin, pin }, result, thread, time::Duration };

use trpl::{ Either, Html, ReceiverStream, Stream, StreamExt };

use crate::books::the_rust_programming_language::chapters::ch10_generic_traits_lifetimes::run;

pub async fn page_title(url: &str) -> Option<String> {
    // let response = trpl::get(url).await;
    // let response_txt = response.text().await;
    // Html::parse(&response_txt)
    //     .select_first("title")
    //     .map(|title| title.inner_html())

    println!("Fetching URL: {}", url);

    let response = trpl::get(url).await;
    println!("Response received!");

    let response_txt = response.text().await;
    println!("Response text length: {}", response_txt.len());

    let document = Html::parse(&response_txt);
    println!("HTML parsed successfully!");

    let title_element = document.select_first("h2");
    match &title_element {
        Some(_) => println!("Found <h2> element."),
        None => println!("No <h2> element found."),
    }

    let title = title_element.map(|t| t.inner_html());
    if let Some(ref t) = title {
        println!("H2 text: {}", t);
    }

    title
}

// writing async fn is equivalent to writing a function that returns a future of the return type. To the compiler, a function definition such as the async fn page_title is equivalent to the code below

// use std::future::Future;
// use trpl::Html;

// fn page_title(url: &str) -> impl Future<Output = Option<String>> {
//     async move {
//         let text = trpl::get(url).await.text().await;
//         Html::parse(&text)
//             .select_first("title")
//             .map(|title| title.inner_html())
//     }
// }

// Racing Our Two URLs Against Each Other
pub fn run1() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = run2(&args[1]);
        let title_fut_2 = run2(&args[2]);

        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");

        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}

async fn run2(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

/* //////////////////////////////////////////////////////////////
                    APPLYING CONCURRENCY WITH ASYNC
////////////////////////////////////////////////////////////// */

// creating a new task with spawn_task

// The trpl crate supplies a spawn_task function that looks very similar to the thread::spawn
// API, and a sleep function that is an async version of the thread::sleep API

pub fn run3() {
    trpl::run(async {
        // let handle = trpl::spawn_task(async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // });

        // for i in 1..5 {
        //     println!("hi number {i} from the second task!");
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }

        // handle.await.unwrap();

        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });
}

// In Rust, std::thread::spawn creates a new thread and returns a JoinHandle, which you can .join() to wait for the thread to finish.

// Similarly, for asynchronous code, trpl::join is used to wait for multiple futures to complete.

// Counting Up on Two Tasks Using Message Passing

// Sharing data between futures works similarly to sharing data between threads, but we use asynchronous versions of message-passing types and functions.
// In thread-based concurrency, we used:
// std::sync::mpsc::channel()
// and spawned separate threads to send and receive messages.

// In futures-based concurrency, we use:
// trpl::channel()
// or other async equivalents — allowing tasks to communicate without blocking.
// The communication is asynchronous — sending and receiving happen via await, which does not block the thread, only the current future.

pub fn run4() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        // -------------------------------
        // Simple one-message example
        // -------------------------------

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("received '{received}'"); // received 'hi'

        // -------------------------------
        // (Commented out example)
        // Sending multiple messages in a loop
        // -------------------------------

        // let vals = vec![
        //     String::from("hi"),
        //     String::from("from"),
        //     String::from("the"),
        //     String::from("future")
        // ];

        // for val in vals {
        //     tx.send(val).unwrap();
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }

        // while let Some(value) = rx.recv().await {
        //     println!("received '{value}'");
        // }
        // ⚠️ However, there are two problems here:
        // 1️⃣ All messages arrive at once (after ~2 seconds) — not spaced out.
        //     Why? Because sending and receiving are both happening sequentially
        //     in the same async block — the receiver only starts *after* sending finishes.
        //
        // 2️⃣ The program never exits. The receiver loop (`while let Some`) waits forever
        //     for new messages that will never come, so you have to stop it with Ctrl + C.

        // -------------------------------
        // Concurrent senders and receiver
        // -------------------------------

        // Create a clone of the sender.
        // Each clone can send independently into the same channel.
        let tx1 = tx.clone();

        // First sender task — sends 4 short messages every 0.5 seconds
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future")
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // Receiver task — runs continuously and prints messages as they arrive.
        // The `.await` inside the loop lets other tasks run while waiting.
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        // Second sender task — sends 4 more messages every 1.5 seconds.
        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you")
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        // Run all three futures concurrently.
        // `trpl::join3` waits for *all* three futures to finish.
        // Unlike awaiting each one separately, this runs them concurrently,
        // so sending and receiving happen in parallel.
        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
        // trpl::join(tx_fut, rx_fut).await;
    })
}

/* //////////////////////////////////////////////////////////////
                   WORKING WITH ANY NUMBERS OF FUTURE
////////////////////////////////////////////////////////////// */

pub fn run5() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        // Create a clone of the sender.
        // Each clone can send independently into the same channel.
        let tx1 = tx.clone();

        // First sender task — sends 4 short messages every 0.5 seconds
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future")
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // Receiver task — runs continuously and prints messages as they arrive.
        // The `.await` inside the loop lets other tasks run while waiting.
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        // Second sender task — sends 4 more messages every 1.5 seconds.
        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you")
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![
            Box::pin(tx1_fut),
            Box::pin(rx_fut),
            Box::pin(tx_fut)
        ];

        trpl::join_all(futures).await;

        let a = async { 1u32 };
        let b = async { "Hello!" };
        let c = async { true };

        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");

        // racing futures

        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        trpl::race(slow, fast).await;
    });

    // let tx1_fut = pin!(async move {
    //         // --snip--
    //     });

    //     let rx_fut = pin!(async {
    //         // --snip--
    //     });

    //     let tx_fut = pin!(async move {
    //         // --snip--
    //     });

    //     let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
    //         vec![tx1_fut, rx_fut, tx_fut];
}

// yielding control to the runtime

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms} ms");
}

pub fn run6() {
    trpl::run(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;

        // let one_ms = Duration::from_millis(1);

        // let a = async {
        //     println!("'a' started.");
        //     slow("a", 30);
        //     trpl::sleep(one_ms).await;
        //     slow("a", 10);
        //     trpl::sleep(one_ms).await;
        //     slow("a", 20);
        //     trpl::sleep(one_ms).await;
        //     println!("'a' finished.");
        // };

        // let b = async {
        //     println!("'b' started.");
        //     slow("b", 75);
        //     trpl::sleep(one_ms).await;
        //     slow("b", 10);
        //     trpl::sleep(one_ms).await;
        //     slow("b", 15);
        //     trpl::sleep(one_ms).await;
        //     slow("b", 350);
        //     trpl::sleep(one_ms).await;
        //     println!("'b' finished.");
        // };

        // The a future still runs for a bit before handing off control to b, because it calls slow before ever calling trpl::sleep, but after that the futures swap back and forth each time one of them hits an await point. In this case, we have done that after every call to slow, but we could break up the work in whatever way makes the most sense to us.
    })
}

/* //////////////////////////////////////////////////////////////
                      STREAMS: FUTURE IN SEQUENCE
    ////////////////////////////////////////////////////////////// */

pub fn run7() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        // while let Some(value) = stream.next().await {
        //     println!("The value was: {value}");
        // }

        let mut filtered = stream.filter(|value| (value % 3 == 0 || value % 5 == 0));

        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    })
}

// The value was: 2
// The value was: 4
// The value was: 6
// The value was: 8
// The value was: 10
// The value was: 12
// The value was: 14
// The value was: 16
// The value was: 18
// The value was: 20

// After : let mut filtered = stream.filter(|value| (value % 3 == 0 || value % 5 == 0));

// The value was: 6
// The value was: 10
// The value was: 12
// The value was: 18
// The value was: 20

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
    // let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    // for message in messages {
    //     tx.send(format!("Message: '{message}'")).unwrap();
    // }

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }

            tx.send(format!("Message: '{message}'")).unwrap();
        }
    });

    ReceiverStream::new(rx)
}

pub fn run9() {
    trpl::run(async {
        let mut messages = get_messages();

        while let Some(message) = messages.next().await {
            println!("{message}");
        }
    });
}

// Message: 'a'
// Message: 'a'
// Message: 'b'
// Message: 'b'
// Message: 'c'
// Message: 'c'
// Message: 'd'
// Message: 'd'
// Message: 'e'
// Message: 'e'
// Message: 'f'
// Message: 'f'
// Message: 'g'
// Message: 'g'
// Message: 'h'
// Message: 'h'
// Message: 'i'
// Message: 'i'
// Message: 'j'
// Message: 'j'

pub fn run8() {
    trpl::run(async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}

// Message: 'a'
// Message: 'a'
// Problem: Elapsed(())
// Message: 'b'
// Message: 'b'
// Message: 'c'
// Message: 'c'
// Problem: Elapsed(())
// Message: 'd'
// Message: 'd'
// Message: 'e'
// Message: 'e'
// Problem: Elapsed(())
// Message: 'f'
// Message: 'f'
// Message: 'g'
// Message: 'g'
// Problem: Elapsed(())
// Message: 'h'
// Message: 'h'
// Message: 'i'
// Message: 'i'
// Problem: Elapsed(())
// Message: 'j'
// Message: 'j'

// Merging Streams
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}

pub fn run10() {
    trpl::run(async {
        // let messages = get_messages().timeout(Duration::from_millis(200));
        // let intervals = get_intervals()
        //     .map(|count| format!("Interval: {count}"))
        //     .timeout(Duration::from_secs(10));
        // let merged = messages.merge(intervals);
        // let mut stream = pin!(merged);

        // Throttling is a way of limiting the rate at which a function will be called—or, in this case, how often the stream will be polled.

        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}

// Interval: 1
// Message: 'a'
// Message: 'a'
// Interval: 2
// Interval: 3
// Problem: Elapsed(())
// Interval: 4
// Message: 'b'
// Message: 'b'
// Interval: 5
// Message: 'c'
// Message: 'c'
// Interval: 6
// Interval: 7
// Problem: Elapsed(())
// Interval: 8
// Message: 'd'
// Message: 'd'
// Interval: 9
// Message: 'e'

/* //////////////////////////////////////////////////////////////
                 A CLOSER LOOK AT THE TRAITS FOR ASYNC
////////////////////////////////////////////////////////////// */

// 1. The Future Trait

// pub trait Future {
//     type Output;
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
// }

// Key Points:
// Future::Output → the value the async computation eventually produces.
// poll(...) → checks the state of the future.
// Requires: Pin<&mut Self> and a Context.

// Poll<T>:
// enum Poll<T> { Ready(T), Pending }
// Ready(T) → work completed.
// Pending → not finished; caller must check again later.

// How await works

// Rust rewrites await into a loop that repeatedly calls poll.
// But instead of blocking, the async runtime schedules polling at the right time.
// A future should not be polled after returning Ready, unless documented safe.

// Mechanics
// Runtime polls futures and puts them to sleep when they return Pending.
// Wakes them again when they make progress.

// 2. Pinning (Pin) & Movement Safety
// Why Pin Exists

// Some futures contain self-references (generated by the compiler’s state machine).
// Moving such a type invalidates those internal references → undefined behavior.
// Pin<T> ensures: the value inside cannot move in memory.

// Pin Facts
// Pin is a wrapper around pointer-like types: &, &mut, Box, Rc, etc.
// “Pinning a pointer” means:
// → the pointer may move, but the data behind it cannot.
// Example:
// Pin<Box<F>> pins the future F, not the box

// 3. Unpin Trait
// Purpose
// A marker trait meaning: it's safe for the type to move even when pinned.
// Most normal Rust types automatically implement Unpin.
// When a type is !Unpin
// For self-referential async futures.
// These types must not move once pinned.
// Relationship
// You only care about Unpin when a value is behind a Pin<T>.
// If a type is Unpin, you can move it even through a Pin wrapper.

// 4. Why join_all Needs Pin
// In code like:
// join_all(futures).await;
// join_all needs to store the futures inside a Vec.
// If those futures are !Unpin, they must be pinned first.
// Solutions:
// - pin! macro
// - Box::pin(future)
// Because:
// Futures from async blocks may contain self-references.
// They are !Unpin by default.

// 5. The Stream Trait
// Concept
// A sequence of asynchronous values.
// Combination of:
// Iterator (sequence)
// Future (items arrive over time)
// Definition
// trait Stream {
//     type Item;
//     fn poll_next(
//         self: Pin<&mut Self>,
//         cx: &mut Context<'_>
//     ) -> Poll<Option<Self::Item>>;
// }
// Interpretation
// Poll → readiness
// Option → element-or-no-element
//     Some(item) → next item available
//     None → stream finished

// 6. StreamExt Trait
// Provides next().await and lots of utility methods.
// Implemented for all Streams.
// Actual next is async (or returns a future wrapper on older Rust):
// async fn next(&mut self) -> Option<Self::Item>
// where Self: Unpin;
// Handles calling poll_next correctly.

// Big Picture: Futures, Streams, Tasks, Threads
// Future → one async computation.
// Stream → many async computations over time.
// Tasks → lightweight units run by the async runtime.
// Runtime → drives futures by polling them, usually on top of threads.
