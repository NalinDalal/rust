fn main() {
    let my_string = String::from("Hello, Rust!");
    takes_ownership(&my_string); // Pass a reference to my_string
    println!("{}", my_string); // This is valid because ownership was not transferred
}

fn takes_ownership(some_string: &String) {
    println!("{}", some_string); // some_string is borrowed and not moved
}
