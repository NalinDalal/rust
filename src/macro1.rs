macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Hello from {}", stringify!($func_name));
        }
    };
}

create_function!(hello); // This will create a function called "hello"

fn main() {
    hello(); // Prints "Hello from hello"
    say_hello!(); // Expands to: println!("Hello, world!");
}
