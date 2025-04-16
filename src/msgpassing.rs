use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

/* use std::sync::mpsc;
use std::thread;
    fn main(){
    let (tx,rx)=mpsc::channel();    //returns a tuple of two channel: sender end, receiver end
    //tx: transmitter end; rx: receiver end
    thread::spawn(move || {
        tx.send((String)::from("hello world"));
        //send method to send some data
        //returns a Result<T, E> type; handles error bydefault
    });
let received = rx.recv().unwrap();
    println!("Got: {received}");
    match value{
        Ok(value)=>println!("{}",value);
Err(err)=>println!("Error while reading the data"),
    }
}*/
