use futures_core::Stream;
use std::{pin::pin, time::Duration};
use tokio::sync::mpsc; // replaces trpl::channel
use tokio::time::sleep; // replaces trpl::sleep
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

//use trpl::{ReceiverStream, Stream, StreamExt};
//new type: ReceiverStream, which converts the rx receiver from the trpl::channel into a Stream with a next method.

#[tokio::main] // replaces trpl::run
async fn main() {
    let mut messages = pin!(get_messages().timeout(Duration::from_millis(200))); //adding a timeout to the stream

    while let Some(result) = messages.next().await {
        match result {
            Ok(message) => println!("{message}"),
            Err(reason) => eprintln!("Problem: {reason:?}"),
        }
    }
}

fn get_messages() -> impl Stream<Item = String> {
    //return type is impl Stream<Item = String>
    let (tx, rx) = mpsc::channel(32); //replaces trpl::channel

    tokio::spawn(async move {
        // replaces trpl::spawn_task
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            sleep(Duration::from_millis(time_to_sleep)).await; // replaces trpl::sleep

            if let Err(send_error) = tx.send(format!("Message: '{message}'")).await {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = mpsc::channel(32); // replaces trpl::channel

    tokio::spawn(async move {
        // replaces trpl::spawn_task
        let mut count = 0;
        loop {
            sleep(Duration::from_millis(1)).await; // replaces trpl::sleep
            count += 1;

            if let Err(send_error) = tx.send(count).await {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}

