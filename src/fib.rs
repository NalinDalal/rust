/*fn main() {
    let mut a = 0;
    let mut b = 1;
    //Generate the nth Fibonacci number.
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
        return b;
    }
    println!("The {}th Fibonacci number is {}", n, a);
}

Problem:
Incorrect Variable Updates: The fib function is not updating a and b correctly because function arguments in Rust are immutable unless passed by reference.
Premature return Statement: The return b; inside the loop causes the function to return after the first iteration, which is incorrect.
Incorrectly Printing b: The b in println! inside main does not contain the computed Fibonacci value. The result of fib is not stored.
Fix Needed for Recursion or Iteration: The Fibonacci sequence should be generated properly.
*/

fn main() {
    let n = 5;
    let result = fib(n);
    println!("The {}th Fibonacci number is {}", n, result);
}

fn fib(n: u32) -> u32 {
    let (mut a, mut b) = (0, 1);

    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }

    a // Return the nth Fibonacci number
}
