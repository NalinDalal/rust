extern crate trpl;
use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);
        println!("calling the stream_from_iter method");
        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
        let mut filtered = stream.filter(|value| value % 3 == 0 || value % 5 == 0);
        println!("calling the filter method");
        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });
}
