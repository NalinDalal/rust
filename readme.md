let's start with rust- fast and safe
already have installed rust locally, to check if exist->`cargo`
`cargo` is a package manager for rust

initialise rust project-> `cargo init`
to run rust file: `rustc main.rs
./main  # Runs the compiled binary
`

initialize a library that you can deploy for other people to use-> `cargo init --lib`
to run via cargo, need to have a cargo.toml file; if cargo present-> `cargo main.rs`

hello world-> main.rs
to compile-> `cargo run`

# Variables
just like cpp we have signed and unsigned integers
just need to define the type of variable when declaring a number
`x:i8=34` or `x:i32=1`
now to print like x:32,y:45 do like->
`print!("x:{}",x);`

# loops
`for _i in 0..100 {print!("{}",i)}` like 0 to 100
for other things like array, maps, string
for string-> in loops.rs file

# Functions
