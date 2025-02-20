fn main() {
    let nums = vec![1, 2, 3];

    // Using for loop directly on the vector
    for value in &nums {
        println!("{}", value);
    }

    // Using iterator
    let iter = nums.iter();
    for value in iter {
        println!("{}", value);
    }
}
