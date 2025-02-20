use std::thread;
use std::time::Duration;

fn main() {
    let sum = 0;
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("hi from spawned thread {}", i);
        }
    });

    handle.join().unwrap(); //waits for the thread to finish its execution
                            //then only control reaches to next line
    for i in 0..5 {
        println!("hi from main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
