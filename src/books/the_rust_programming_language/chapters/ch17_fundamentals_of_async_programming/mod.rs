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

use std::{ pin::Pin, time::Duration };

use trpl::{ Either, Html };

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
