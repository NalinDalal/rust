use std::time::{Duration, Instant};
use tokio::task::yield_now;
use tokio::time::sleep;

#[tokio::main]

async fn main() {
    let one_ns = Duration::from_nanos(1);
    let start = Instant::now();
    async {
        for _ in 1..1000 {
            sleep(one_ns).await; //pass a one-nanosecond Duration to sleep
        }
    }
    .await;
    let time = Instant::now() - start;
    println!(
        "'sleep' version finished after {} seconds.",
        time.as_secs_f32()
    );

    let start = Instant::now();
    async {
        for _ in 1..1000 {
            yield_now().await; //yield_now is way faster
        }
    }
    .await;
    let time = Instant::now() - start;
    println!(
        "'yield' version finished after {} seconds.",
        time.as_secs_f32()
    );
}
