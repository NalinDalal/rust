macro_rules! generate_functions {
    ($($func_name:ident),*) => {
        $(
            fn $func_name() {
                println!("Hello from {}", stringify!($func_name));
            }
        )*
    };
}

generate_functions!(foo, bar, baz);

fn main() {
    foo(); // Prints: Hello from foo
    bar(); // Prints: Hello from bar
    baz(); // Prints: Hello from baz
}
