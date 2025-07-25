mod borrowing;
mod channel;
mod closure;
mod closure_cap;
mod enum1;
mod enum2;
mod error;
mod fib;
mod for_temp;
mod function;
mod future;
mod hash_map;
mod hash_map2;
mod hash_map3;
mod iterator;
mod lifetime;
mod loop2;
mod loop3;
mod loops;
mod macro1;
mod macro2;
mod macro3;
mod macro4;
mod macro5;
mod memory;
mod method;
mod mpsc;
mod number;
mod option_enum;
mod packages;
mod pattern;
mod reference;
mod socket;
mod stream;
mod structs;
mod thread;
mod trait1;
mod vector;
fn main() {
    println!("Hello, world!");
    let x: i32 = 5;
    println!("x is {}", x);
    let y: u32 = 1000;
    println!("y is unsigned integer{}", y);
    let z = std::f64::consts::PI; //64 bit float no
    println!("z is a float no{}", z);
    //by default turns to i32

    //constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
