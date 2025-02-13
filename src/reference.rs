fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1;

    println!("{}", s2);
    println!("{}", s1); // This is valid, The first pointer wasn't invalidated
}
