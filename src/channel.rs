use std::{
    sync::mpsc,
    thread::{self,spwan};
}

    fn main(){
    let (tx,rx)=mpsc::channel();
    spawn(move || {
        ts.send(String)::from("hello world"));
    });

    match value{
        Ok(value)=>println!("{}",value);
Err(err)=>println!("Error while reading the data"),
    }
}
