mod borrowing;
mod function;
mod loops;
mod memory;
mod number;
mod reference;
mod struct;
fn main() {
    println!("Hello, world!");
    let x: i32 = 5;
    println!("x is {}", x);
    let y: u32 = 1000;
    println!("y is unsigned integer{}", y);
    let z: f64 = 3.14; //64 bit float no
    println!("z is a float no{}", z);
    //by default turns to i32
}
