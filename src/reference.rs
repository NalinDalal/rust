fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1;
    //&s1 syntax lets us create a reference that refers to the value of s1 but does not own it. Because the reference does not own it, the value it points to will not be dropped when the reference stops being used.
    //& indicates it is a reference
    println!("{}", s2);
    println!("{}", s1); // This is valid, The first pointer wasn't invalidated
}
