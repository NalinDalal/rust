use std::{
    future::Future,
    pin::Pin,
    thread,
    time::{Duration, Instant},
};

use futures::{future::join_all, join, FutureExt}; // From `futures` crate
use tokio as trpl; // Use tokio as 'trpl' to match description

// Simulates a blocking operation
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

#[tokio::main]
async fn main() {
    println!("=== Static join with `join!` macro ===");
    let a = async { 1u32 };
    let b = async { "Hello!" };
    let c = async { true };
    let (a_result, b_result, c_result) = join!(a, b, c);
    println!("{a_result}, {b_result}, {c_result}\n");

    println!("=== Dynamic futures with `join_all` using Pin<Box> ===");
    let tx1_fut = async move {
        println!("tx1 started");
        trpl::time::sleep(Duration::from_millis(10)).await;
        println!("tx1 finished");
    };

    let rx_fut = async {
        println!("rx started");
        trpl::time::sleep(Duration::from_millis(20)).await;
        println!("rx finished");
    };

    let tx_fut = async move {
        println!("tx started");
        trpl::time::sleep(Duration::from_millis(15)).await;
        println!("tx finished");
    };

    let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
        vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];

    join_all(futures).await;
    println!("All dynamic futures finished!\n");

    println!("=== Using `pin!` for local async blocks ===");
    let mut tx1 = Box::pin(async {
        println!("tx1 (pin!) started");
        trpl::time::sleep(Duration::from_millis(10)).await;
        println!("tx1 (pin!) finished");
    });

    let mut rx = Box::pin(async {
        println!("rx (pin!) started");
        trpl::time::sleep(Duration::from_millis(20)).await;
        println!("rx (pin!) finished");
    });

    let mut tx = Box::pin(async {
        println!("tx (pin!) started");
        trpl::time::sleep(Duration::from_millis(15)).await;
        println!("tx (pin!) finished");
    });

    let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
        vec![tx1.as_mut(), rx.as_mut(), tx.as_mut()];
    join_all(futures).await;
    println!("All pinned dynamic futures finished!\n");

    println!("=== Racing async blocks ===");
    let slow = async {
        println!("'slow' started.");
        trpl::time::sleep(Duration::from_millis(100)).await;
        println!("'slow' finished.");
    };

    let fast = async {
        println!("'fast' started.");
        trpl::time::sleep(Duration::from_millis(50)).await;
        println!("'fast' finished.");
    };

    futures::future::select(Box::pin(slow), Box::pin(fast)).await;
    println!("Race done!\n");

    println!("=== Yielding control ===");

    let one_ms = Duration::from_millis(1);
    let a = async {
        println!("'a' started.");
        slow("a", 30);
        trpl::task::yield_now().await;
        slow("a", 10);
        trpl::task::yield_now().await;
        slow("a", 20);
        trpl::task::yield_now().await;
        println!("'a' finished.");
    };

    let b = async {
        println!("'b' started.");
        slow("b", 75);
        trpl::task::yield_now().await;
        slow("b", 10);
        trpl::task::yield_now().await;
        slow("b", 15);
        trpl::task::yield_now().await;
        slow("b", 35);
        trpl::task::yield_now().await;
        println!("'b' finished.");
    };

    futures::future::select(Box::pin(a), Box::pin(b)).await;
    println!("Cooperative yield race done!\n");

    println!("=== Comparing `sleep` vs `yield_now` ===");

    let one_ns = Duration::from_nanos(1);
    let start = Instant::now();
    async {
        for _ in 0..1000 {
            trpl::time::sleep(one_ns).await;
        }
    }
    .await;
    let elapsed = Instant::now() - start;
    println!(
        "'sleep' version finished after {} seconds.",
        elapsed.as_secs_f32()
    );

    let start = Instant::now();
    async {
        for _ in 0..1000 {
            trpl::task::yield_now().await;
        }
    }
    .await;
    let elapsed = Instant::now() - start;
    println!(
        "'yield' version finished after {} seconds.",
        elapsed.as_secs_f32()
    );
}
