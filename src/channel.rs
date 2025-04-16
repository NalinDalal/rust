use std::{
    sync::mpsc,
    thread::{self, spawn},
};
fn main() {
    let (tx, rx) = mpsc::channel();

    // Spawn a thread to send a message

    spawn(move || {
        tx.send(String::from("hello world")).unwrap(); //unwrap to handle result
    });

    //Receive the message
    match rx.recv() {
        Ok(value) => println!("{}", value),
        Err(err) => println!("Error while reading the data: {}", err),
    }
}
