use tokio::join;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32); // buffer size 32

    let tx1 = tx.clone();

    let tx1_fut = async move {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx1.send(val).await.unwrap();
            sleep(Duration::from_millis(500)).await;
        }
    };

    let tx_fut = async move {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).await.unwrap();
            sleep(Duration::from_millis(1500)).await;
        }
    };

    let rx_fut = async move {
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    };

    join!(tx1_fut, tx_fut, rx_fut);
}
