let's start with rust- fast and safe
to open book-> `rustup doc --book`
already have installed rust locally, to check if exist->`cargo`
`cargo` is a package manager for rust

initialise rust project-> `cargo init`
to run rust file: `rustc main.rs
./main  # Runs the compiled binary
`
now to create a project using cargo-> 
`cargo new hello_cargo
cd hello_cargo
`
generates a git repo with files-> Cargo.toml,src/main.rs

initialize a library that you can deploy for other people to use-> `cargo init --lib`
to run via cargo, need to have a cargo.toml file; if cargo present-> `cargo main.rs`

hello world-> main.rs
to compile-> `cargo run`

`cargo.toml`-> think of it as a package.json for rust

From your hello_cargo directory, build your project by entering the following command:

`
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
`

command creates an executable file in target/debug/hello_cargo; to run the
executable ->
`$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
`

`cargo run` is easy to use
`cargo check` check the file i.e. it will compile whole file but not produce an
excutable

------
# Chapter 3

# Variables
just like cpp we have signed and unsigned integers
just need to define the type of variable when declaring a number
`x:i8=34` or `x:i32=1`
now to print like x:32,y:45 do like->
`print!("x:{}",x);`
`{}` indicates that it is a placeholder which points to someother things
think of `{}` as little crab pincers that hold a value in place

constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.
`const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`
Constants are valid for the entire time a program runs, within the scope in which they were declared
| Length   | Signed  | Unsigned |
|----------|---------|----------|
| 8-bit    | i8      | u8       |
| 16-bit   | i16     | u16      |
| 32-bit   | i32     | u32      |
| 64-bit   | i64     | u64      |
| 128-bit  | i128    | u128     |
| arch     | isize   | usize    |
signed-> +ve,-ve
unsigned-> +ve only
Signed numbers are stored using two’s complement representation.

--------------------

# loops
`for _i in 0..100 {print!("{}",i)}` like 0 to 100
for other things like array, maps, string
for string-> in loops.rs file
for loop, while loop

The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
loop2.rs->code

`while`-> loop3.rs
While the condition is true, the loop runs. When the condition ceases to be true, the program calls break, stopping the loop

## Assignment
- Convert temperatures between Fahrenheit and Celsius.-> for_temp.rs
- Generate the nth Fibonacci number.-> fib.rs
- Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.


# Functions
need to give a return type also when declaring it.

Note: `println!("Sum of {} and {} is {}", a, b, sum);`
in the given statement we had 3 arguments so we called `{}` 3 times and added the
argument in order we want them

# Memory Managment in Rust
Whenever you run a program (C++, Rust, JS), it allocates and deallocates memory on the RAM.
3 ways-> 

| Garbage Collector | Manual | The Rust Way |
|------------------|--------|--------------|
| 1. Written by smart people | 1. You allocate and deallocate memory yourself | 1. Rust has its own ownership model for memory management |
| 2. Usually no dangling pointers/memory issues | 2. Can lead to dangling pointers/memory issues | 2. Makes it extremely safe to memory errors |
| 3. You can't do manual memory management | 3. Learning curve is high since you have to do manual MM |3. Can do both Manual as well as automatic, but the language itself has a way of writing so that it is memory safe |
| 4. Examples - Java, JS | 4. Examples - C | 4. Example - Rust |

Hence Rust is designed to ensure safety and efficiency without the need for a garbage collector.

ways-> Mutability, Heap and memory, Ownership model, Borrowing and references, Lifetimes

## Mutability
it is Immutable-> can't be changed
need to specify if want to be mutable-> `mut` keyword
```rs
let mut x: i32 = 1;
x=x+1;//since we are adding 1 to x so it is mutable, need to defined accordingly
let y=2;
y=y+1;  //wrong
```

reason:
1. Immutable data is inherently thread-safe because if no thread can alter the data, then no synchronization is needed when data is accessed concurrently.
2. Knowing that certain data will not change allows the compiler to optimize code better.

mutate only when needed; not suggested to keep data mutable to avoid dangling
pointer error

## Stack v/s Heap
`Stack`: primitive data type stores on ram in a form of stack with it's size reserved, so if it's either 4 or 4 million no worry
`Heap`: used for data that can grow at run time, like heaps or vectors

- define a variable, when called stored into stack frame
- say 2 function, 1st calling other, so 2nd function gets pushed on heap; it's stack
  frame gets copied to stack, so when that function excutes, it pops that other
  frame then run the simple/original function
`memory.rs`

# Chapter 4 
# Ownership
Ownership is a set of rules that govern how a Rust program manages memory. Allprograms have to manage the way they use a computer’s memory while running. 
Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; 
in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a
system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. 
None of the features of ownership will slow down your program while it’s running.

Ex: say a girl always want a boyfriend/owner, say if she is single she will die,
but need to have atleast one owner/boyfriend. so if owner dies, then i must find a new owner.

## Rules:
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

`Scope`: A scope is the range within a program for which an item is valid.

## Stack Variables
if stack goes out of scope then heap dies,


## Heap variables
Heap variables are like Rihana. They always want to have a single owner(boyfriend), and if their owner goes out of scope(boyfriend dies), they get deallocated(Rihana also dies).
Any time the owner of a heap variable goes out of scope, the value is de-allocated from the heap.it gets cleared
when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.
ex: Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the host finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.


pushing to stack faster than allocating on heap cause stack is directly pushed,
but for heap it has to find space to hold data
accessing on stack is also faster cause you have to follow a pointer to get there.

## Why
beacuse of dangling pointer error, say one clears it, but another has pointer to it. so not allowed.
heap-> single owner

now say->
```rs
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid

```
whenever `s` goes out of scope, rust calls a special function `drop` which clears the memory

## Variables and Data Interacting with Move
```rs
    let x = 5;
    let y = x;  //it is all good;bind the value 5 to x; then make a copy of the value in x and bind it to y
```

string version:3 parts{a pointer to the memory that holds the contents of the string, a length, and a capacity};stored on stack
```rs
//error code
fn main () {
let s1: String = String:: from("Hi there");
let s2: String = s1;
println!("{}",s2);}
//so now if i call s1 it will give error
```
This is to avoid memory issues like
1. Double free error: when owner goes out of scope, data gets cleared cause:when s2 and s1 go out of scope, they will both try to free the same memory 
 Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.


2. Dangling pointers.

## Fix
Clone the string, basically clones over the content
```rs
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1); // Compiles now
}
```

but say want to pass the same string, dont want to clone as it may take more space
bydefault refer to same place, so 1st owner becomes inValid
```rs
fn main() {
    let s1 = String::from("hello");
    let s2 = takes_ownership(s1);
    println!("{}", s2);
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string); 
    return some_string; // return the string ownership back to the original main fn
}
```


better way without passing over the ownership-> `references`

# References
rihana now comes back to same old guy, use references
Rihana now says I’d like to be borrowed from time to time. I will still have a single owner, but I can still be borrowed by other variables temporarily
A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable

References mean giving the address of a string rather than the ownership of the string over to a function
```rs
fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1;   //necessary to put '&'

    println!("{}", s2);
    println!("{}", s1);    // This is valid, The first pointer wasn't invalidated
}
```
`&s1` syntax lets us create a reference that refers to the value of s1 but does not own it.

# Borrowing
transferring ownership of variables to fns
passing a reference to the string to the function `take_ownership`,the ownership of the string remains with the original variable, in the mainFunction.
allows to use `my_string` again after the function call.
borrowing.rs
```rs
fn main() {
    let my_string = String::from("Hello, Rust!");
    takes_ownership(&my_string);  // Pass a reference to my_string
    println!("{}", my_string);    // This is valid because ownership was not transferred
}

fn takes_ownership(some_string: &String) {
    println!("{}", some_string);  // some_string is borrowed and not moved
}
```
so what is rihana ?
rihana is a whore{it's ownership can be borrowed anytime as per user requirement}

# mutable references
if want to update value of variable
only one at a time
if rihana does something with one borrowerer then she can't do with any other
irrespective of their Mutability

big restriction:  if you have a mutable reference to a value, you can have no other references to that value

that means rihana can only have only one editor at a time, either the onwer or
only one borrowerer

## Rules
- There can me many immutable references at the same time
- There can be only one mutable reference  at a time
- If there is a mutable reference , you can’t have another immutable reference either. 
- If someone makes an immutable reference , they don’t expect the value to change suddenly
- If more than one mutable references happen, there is a possibility of a data race and synchronization issues

# Chap 5
# Struct
to collect over similar data together to bind them, like classes in cpp, objects in js
allows to make custom data binded together
a custom data type that lets you package together and name multiple related values that make up a meaningful group
```rs
struct Rect {
    width: u32,
    height: u32,
}
```
more flexible than tuple
`width`,`u32`: these are pieces of data known as `fields`
consist of `key-value pairs`; key-name, value-data

they can be mutable, but agaion need to declare the when declaring them

syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
```rs
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 //get remaining fields from user1 struct via copy trait
    };
}
```

## Tuple Struct
Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields
used for:
- want to give the whole tuple a name and make the tuple a different type from other tuples
- when naming each field as in a regular struct would be verbose or redundant.
```rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
used to add more meaning 

can't directly print a struct like->
```rs
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```
creates problem:->`error[E0277]: `Rectangle` doesn't implement `std::fmt::Display``
compiler gt's confused b/w what to print and what not to like '{}' and ','. so
it just gives up compilation, use -> `:?` -> tells println! we want to use an output format called Debug.
so compiler doesn't even compiler the code here but gives a hint to use debug
trait more effectively->
```sh
   = help: the trait `Debug` is not implemented for `Rectangle`
   = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```
final file->
```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
}
```
use `{:#?}` instead of `{:?}` to style the output

# Method Syntax
just like function but defined inside context of a struct/enum/trait
method.rs file
In main, where we called the area function and passed rect1 as an argument, we can instead use method syntax to call the area method on our Rectangle instance. The method syntax goes after an instance: we add a dot followed by the method name, parentheses, and any arguments.

`getter`-> when the function name is the thing returning itself; ex:
```rs
fn width(width:u32)->u32{
    return width;
}
```
diff method can be declared inside diff impl under same name->
```rs
impl Rectangle{fn area(&self) -> u32 {
        self.width * self.height
    }}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

```

# Chap 6
# Enums
Enums allow you to define a type by enumerating its possible variants
enumerate- To count off or name one by one;
```rs
enum IpAddrKind {
    V4,
    V6,
}
```

enums with values
Result and Option enum

# Pattern Matching
Let you pattern match across various variants of an enum and run some logic
The match statement is used to pattern match against player_direction to handle each possible variant.
Think of a match expression as being like a coin-sorting machine
```rs
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {    //list `match` with expression `coin` 
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

```
feels like if else, but that is like boolean but here it's literally
everythng;it can be any type
`Coin::Penny=>1` think of it as a arm; each arm is separated by comma
 If a pattern matches the value, the code associated with that pattern is executed

to bind with values we use `useState`->
```rs
enum Coin{Quarter(UsState),}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

can do same with generics also
to get the inner `T` value out of the Some case when using `Option<T>`; we can also handle `Option<T>` using match

we always need to exhaust every last possibility in order for the code to be valid. 

can do like spcl action on spcl value
```rs
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        _=>reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
fn reroll() {}
```
`_` corresponds to a special pattern that matches any value and does not bind to
that value.

that's like default case


# Chap 7
# Package Management
- `Packages`: A Cargo feature that lets you build, test, and share crates
- `Crates`: A tree of modules that produces a library or executable
- `Modules and use`: Let you control the organization, scope, and privacy of paths
- `Paths`: A way of naming an item, such as a struct, function, or module


You can add an external crate to your project by running -> 
```bash
cargo add crate_name
````

crate-> term for packages in rust, just like express, zod etc.
ex: chrono for date and time

## Crate
Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server.

## Packages
A package is a bundle of one or more crates that provides a set of
functionality. `Cargo` is a package containing binary crate for cli
Package has atleast one crate in it.

## Cheatsheet
1. Look for root file(src/main.rs or src/lib.rs etc)
2. declare new modules,submodules here-> `mod filename`
3. you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow
4. `Private v/s Public`: Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod. To make items within a public module public as well, use pub before their declarations.
5. `use` keyword : use it directly to call directly via shortcut rather than
   path way->`use crate::garden::vegetables::Asparagus;`

module can be nested->
```rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

## Path
used to find files in crate and module tree, 2 type: 
- `absolute`:full path from crate root
- `relative`:starts from the current module and uses self, super, or an identifier in the current module.

ex:
```rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

parent can't use their child's private item, but child can use parent's private
items; because child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined. 

paths can be exposed by adding `pub` keywords

Adding the pub keyword in front of mod hosting makes the module public. With this change, if we can access front_of_house, we can access hosting. But the contents of hosting are still private; making the module public doesn’t make its contents public. 

The `pub` keyword on a module only lets code in its ancestor modules refer to it, not access its inner code.
final code:
```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

## Starting Relative Paths with super
instead of `../..` just use `super`
```rs
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```


## bring paths into scope
Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem
ideal way->
```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
Bringing the function’s parent module into scope with use means we have to specify the parent module when calling the function

## Providing New Names with the as Keyword
`use std::fmt::Result;
use std::io::Result as IoResult;`
now both won't conflict even in same scope

## Re-exporting Names with pub use
when we bring name into scope it is private hence to use it we must add pub
```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
## Using External Packages
say we want to use a package named `rand`; add that into Cargo.toml file->
`rand = "0.8.5"`
to bring `rand` definitions into the scope of our package, we added a `use` line starting with the name of the crate, `rand`
```rs
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

for standard library-> `use std::collections::HashMap;`
since it is shipped with language itself hence don't need to update anything and
hence use it directly into the code
`std` is name of standard crate library


## Using Nested Path to clean up
call directly nested packages like 
`use std::{cmp::Ordering, io};` 
instead of
`use std::cmp::Ordering;
use std::io;`


like see this
`use std::io;
use std::io::Write;`

just nest it as->
`use std::io::{self, Write};`

we want to bring all public items defined in a path into scope, use `glob`/`*`
operator-> `use std::collections::*;`

modules can be separated into diff files also
Once the compiler knows the file is part of the project (and knows where in the module tree the code resides because of where you’ve put the mod statement), other files in your project should refer to the loaded file’s code using a path to where it was declared,

Final Summary:
`Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module. You can do this by specifying absolute or relative paths. These paths can be brought into scope with a use statement so you can use a shorter path for multiple uses of the item in that scope. Module code is private by default, but you can make definitions public by adding the pub keyword.`


# Memory Management
amount of space a number take doesn't changes as time goes by, but for a string it may change

it's okay if size of variable is defines, but say we use a function
so the function get's pushed onto stack, so like 2 variable then a function
so it will create a stack frame, push funtion on it
now for funtion, another frame gets pushed on memory
this frame contains inside of function, first it will get pushed
then one for function itself
then for the main function

so basically, main gets pushed to stack, then the function, then function data;
now while compiling first stack data gets popped, then stack itself, then main
function

for data structure like number, we do not neet to worry
they are non dynamic variable

but not true for string

when to store on heap

# Memory Management

| **Heap** | **Stack** |
|----------|----------|
| **Dynamic, allocated at runtime** | **Static, allocated at compile time** |
| **Much larger in size** | **Smaller in size** |
| **Slower due to dynamic allocation and deallocation** | **Faster** |
| **Use: Dynamic and large data structures (e.g., Vec, HashMap, Box)** | **Use: Small, fixed-size variables and function call information** |

# Memory Storage

| **Stored on the Stack** | **Stored on the Heap** |
|------------------------|----------------------|
| Numbers - `i32`, `u64`, `f32` | Strings |
| Booleans - `true`, `false` | Vectors |
| Fixed sized arrays - `[i32; 4]` | HashMap |
| Structs - `{ x: i32, y: i32 }` | Large Arrays/Structs that can’t fit in the Stack |
| References (later) | |


say for string
store it on stack with a pointer to Heap storing the data
Why are strings stored on the heap?
1. They are large
2. Their size can change at runtime, and the size of a stack frame needs to be fixed


# Moving
say a1 had some data, but we assigned it another variable like:
```rs
let a1 = String::from("harkirat") ;|
let a2=a1;
Println! ("number is {}", a1);
```
this is wrong , to overcome it use `clone`
```rs
let a2=a1.clone();
```
rust just doesn't allow it b/c of dangling pointer 

just move the owner
```rs
fn create_string() {
let s1: String = String:: from ("Hello");
let s2: String = s1;
// Print the string
println! ("{}", s1):;}
fn main(){create_string();}
```
borrow of moved value: 's1'
value borrowed here after move
solution->
either pass back the ownership or clone it
```rs
s2=s1
    //do opr with s2
//then    
    s1=s2
```
or clone it
```rs
let s2 = s1.clone();
```

# Borrow
instead of moving, just borrow it
```rs
fn main(){
    let s1=String::from("harkirat");
    do_something(s2:&s1);   //sort of a pointer
    println!("number is {}",s1);
}
fn do_something(s2:&String){
    println!("{}",s2);
}
```

## Rules
• At any given time, you can have either one mutable reference or any number of immutable references.
• References must always be valid.

# Chapter 8
# Collections

just like stl in cpp.
Rust's standard library includes a number of very useful data structures called collections.
Most other data types represent one specific value, but collections can contain multiple values. the data these collections point to is stored on the heap
mainly 3:
- A _vector_ allows you to store a variable number of values next to each other.
- A _string_ is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
- A _hash map_ allows you to associate a value with a specific key. It’s a particular implementation of the more general data structure called a map.

vector and hashmap covers most use cases for generics


# Vectors
Similar to vector in cpp
same as stack and heap,
basically the array is on heap, but stack has the pointer to that heap
heap can be increased decreased
need to make it mutable
```rs
let mut vec=Vec::new(); //create a new empty vector
vec.push(1);
vec.push(2);
```
- Vectors are implemented using generics-> `let v: Vec<i32> = Vec::new();`->this says holds i32 data type
- `Vec<T>` says that any type can be provided by stl can hold any type

more code in `vector.rs` file

via macros-> `let v = vec![1, 2, 3];`
Because we’ve given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the type annotation isn’t necessary

## Modify a Vector
use push method-> `v.push(5);` again need to declare v as mutable

## Reading/Accessing element
2 ways are there-> the `indexing syntax` and the `get` method.
```rs
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];    //to access nth we write n-1 cause zero-indexed property
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
```

now if you directly access any element outside the range, program panics cause
it references a nonexistent element.

When the `get` method is passed an index that is outside the vector, it returns `None` without panicking.

you can’t have mutable and immutable references in the same scope.
so once you have an immutable reference you just can't call another mutable one,
so problemmatic.
error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

## Iterating over Values in a Vector
- use a for loop
```rs
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
```

- Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker’s rules.

## Using an Enum to Store Multiple Types
- Vectors can only store values that are of the same type
- Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element.


# String
string we know what is
```rs    
let mut s = String::new();     //declration of empty string
    let s = String::from("initial contents");   //create a String from a string literal
```
strings are UTF-8 encoded

## Updating a String
string can grow in size and its content can change

- using `push_str`
```rs
    let mut s = String::from("foo");
    s.push_str("bar");
```

- using push method{takes single char as argument}
```rs
    let mut s = String::from("lo");
    s.push('l');
```

- Concatenation with the + Operator or the format! Macro
```rs
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
     let s = format!("{s1}-{s2}-{s3}");
```


## Indexing
if you try to access parts of a String using indexing syntax in Rust, you’ll get an error
` let h = s1[0];`-> error

## Internal Working
It is a wrapper over a `Vec<u8>`
```rs
let hello = "Здравствуйте";     //size is 24 instead of 12 because each Unicode scalar value in that string takes 2 bytes of storage.
let answer = &hello[0];
```

**A final reason Rust doesn’t allow us to index into a String to get a character
is that indexing operations are expected to always take constant time (O(1)).
But it isn’t possible to guarantee that performance with a String, because Rust
would have to walk through the contents from the beginning to the index to
determine how many valid characters there were.**

## String Slicing
Rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:
```rs
let hello = "Здравствуйте";
let s = &hello[0..4];   //this code panics
```
use caution when creating string slices with ranges, because doing so can crash your program.

## Iterating over string
for char->
```rs
for c in "Зд".chars() {
    println!("{c}");
}
```

o/p:
`З
д
`
for bytes->
```rs
for b in "Зд".bytes() {
    println!("{b}");
}
```

o/p:
`208
151
208
180
`


# HashMaps
they store key value pairs in rust
ype HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory

use a library-> `use std::collections::HashMaps`
`HashMap.rs`
```rs
    use std::collections::HashMap;

    let mut scores = HashMap::new();    //hashmap created

    scores.insert(String::from("Blue"), 10);    //blue team with 10 points
    scores.insert(String::from("Yellow"), 50);  //yellow team with 50 points
```
to get value out of hash via a key-> `get` method
```rs
 let score = scores.get(&team_name).copied().unwrap_or(0);
```

## HashMap and Ownership
type of values implementing `Copy` trait ex:i32 values are copied into hashmap
for owned values ex:String values are moved and hashmap is new owner
```rs
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
```

## Updating a Hash Map
we can increase the content but each unique key can only have one value
associated with it at a time

## OverWriting a Value
If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced.
it just upates/overwrites the value

## Adding a Key and Value Only If a Key Isn’t Present
check if a value exist or not, if then some action, if not then some action
`entry` api is used here,takes the key you want to check as parameter;returns
enum called `Entry` that represent i=value may exist or not

    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);

or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, it inserts the parameter as the new value for this key and returns a mutable reference to the new value

## Updating based on Old Value
```rs
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

```


# Iterators
The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don't have to reimplement that logic yourself.

In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up. For example, the code in Listing 13-10 creates an iterator over the items in the vector `v1` by calling the `iter` method defined on `Vec<T>`. This code by itself doesn't do anything useful.

they literally don't have any effect on code till they are consumed or called.

1. using for loops:
```rs
fn main() {
let nums = vec! [1, 2, 3];
for value in nums{ println!("{}", value);}}
```
2. Iterating after creating an 'iterator'
```rs
fn main() {
let nums= vec! [1, 2, 3];
let iter = nums.iter();
for value in iter {
println! ("{}", value);
}}
```

The iter method in Rust provides a way to iterate over the elements of a collection by `borrowing them`.
You can't mutate the variables since we have an immutable reference to the internal elements
it borrows the values, doesn't consumes them and becomes their owner.

3. using 'iter_mut'
```rs
fn main(){
    let mut v1=vec![1,2,3];
    let v1_iter=v1.iter_mut();
    for val in v1_iter{
        *val=*val+1
    }
    println!("{:?}",v1);
}
```

4. using '.next'
it check and then consumes if any next value exist, else null
so here the while check it iter.next has some value then enter scope, else stop
```rs
fn main(){
let nums=vec![1,2,3];
let mut iter=nums.iter();
while let Some(val)=iter.next(){
    print!("{}",val);
}}
```
it is similar to:
```rs 
for val in v1_iter {
*val = *val + 1}
```

it has a next function under the hood in it's own toolchain

5. using `into_iter`
it is simialr to iter but it's like giving the ownership, so it takes over the ownership of the collection
Useful when
    1. You no longer need the original collection
    2. When you need to squeeze performance benefits by transferring ownership (avoiding references)

Many iterator adapters take closures as arguments, and commonly the closures we’ll specify as arguments to iterator adapters will be closures that capture their environment.

iter v/s into_iter
iter stores the references, but into_iter does directly on vector

| Iterator Type  | Description |
|---------------|-------------|
| `iter()`      | If you want immutable references to the inner variables and don't want to transfer ownership |
| `iter_mut()`  | If you want mutable references to the inner variables and don't want to transfer ownership |
| `into_iter()` | If you want to move the variable into the iterator and don't want to use it afterwards |

`for` iterator directly applied on a vector is same as `into_iter`


## Consuming Adapters
on top of an iterator create a variable.
now various function can be applied on it
```rs
let v1 = vec![1, 2, 31;
let v1_iter = v1.iter();
let total: i32 = v1_iter.sum();
```
it can be used only once, like can't be called again after once used, as it
ends up consumes up that iterator to that it gets moved to that variable it is
assigned to, takes over the ownership

## Iterator Adapters
Iterator adaptors are methods defined on the Iterator trait that don't consume the iterator. 
Instead, they produce different iterators by changing some aspect of the original iterator.

1. map
```rs
fn main() {
let
v1: Vec<i32> = vec! [1, 2, 3];
let iter = v1. iter();
let iter2 = :
map (|x| x + 1);
for x in iter2 {
println! ("{}", x);}}
```
like it convert 1 to 2, 2 to 3, 3 to 4
so at index 0 it updates 1 to 2

it's just like map function
like for filter->
```rs
fn main() {
let v1: Vec<i32> = vec![1, 2, 3];
let iter = v1.iter();
let iter2 = iter.filter (|x| *x % 2 == 0);
for x in iter2 {
println! ("{}",x);}}
```

Assignment -
Write the logic to first filter all odd values then double each value and create a new vector

```rs
fn filter_and_map(v: Vec<i32>) → Vec<:32> {
let new_iter = v.iter().filter(|x| *x%2 == 1).map(|x| x + 2);
let new_vec: Vec<i32> = new_iter.collect();
return new_vec;}
fn main(){
    let v1:Vec<i32>=vec![1,2,3];
    let ans=filter_and_map(v1);
    println!("{:?}",ans);
}

```

Assignment:
vector<string,number> convert to hashmap, create iterator to hashmap to vector


# Strings vs Slices

String is UTF-8 encoded
it is growable, mutable, owned
might refer to `String type` or `String Slices`
all this is utf-8 encoded

slices is generic concept, apply to vectors and strings
it is a kind of reference so can't have ownership

```rs
fn main() {
    //1. string created
let mut name = String::from ("Harkirat");
    //2. string mutated-> pushed something to the string

name.push_str(" Singh"); println!( "name is {}", name);
//3. to delete something from string
    name. replace_range(8..name.len(), "");     //delete everything from 8th character or index to end of string
println! ("name is {}", name);
}
```

it is sort of a view into the string with window given

Write a function that takes a string as an input And returns the first word from it
```rs
fn main(){
    let mut name="Nalin Dalal";
    //like return Nalin
    //so do like iterate till space
    //break, else push to ans_string
    //return it
name.replace_range(2..name.len(),"");
    println!("First char of string is {}",name);
}
```

normal approach: iterate over the original string, push it to new ans string
iterate till space is encountered
problem:
1. if `name` gets cleared, the whole `ans` string also gets cleared
2. ends up using double the memeory

new problem: return a view from the string for first word

cause we can have multiple immutable references, but only one mutable reference can exist i.e. you can't have other immutable/mutable reference

Approach 2(with slices)
```rs
fn main(){
    let name=String::from("hello world");
    let mut space_index=0;
    for i in name.chars(){
        if i==' '{
            break;
        }
        space_index=space_index+1;
    }
    let ans=&name[0..space_index];  //ans is immutable reference to name
    println!("ans is {}",ans);
}
```

but say you just declare string w/o declaring data type, like just a String Literal; type is &str
to print vector with start index and end index
```rs
fn main() {
let v = [1, 2, 3,4];
println!("{:?}"，&v[0..3]); //prints 0,1,2
}
```

# Chap 9
# Error Handling
we know what type of error handling is in cpp, js like try catch block
but rust doesn't have any try catch or exceptions.
so hence Rust provides an Enum for same
```rs
enum Result<T, E> { //for recoverable errors
    Ok(T),
    Err(E),
}
```

`panic` is also an implementation.
it is an enum (with generic types)
This enum is what a function can return/returns when it has a possibility of throwing an error

example: error.rs
it returns a enum with the `Ok` variant having a string value and `Err` variant
having an Error value

option enum-> to introduce concept of nullability in a safe and expressive way.
if u ever have a function that should return null, return an Option instead.

## Panic
They are unrecoverable errors.there’s nothing you can do about it.
two ways to cause a panic in practice: 
- by taking an action that causes our code to panic (such as accessing an array past the end).
- by explicitly calling the panic! macro.

when `Panic` occurs 2 ways->
- unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters
- immediately aborting, which ends the program without cleaning up.

to abort in panic mode add in `Cargo.toml` file->
```toml
[profile.release]
panic = 'abort'
```

call panic in normal file->
```rs
fn main() {
    panic!("crash and burn");
}
```

on running it shows our panic message and the place in our source code where the panic occurred: src/main.rs:2:5 indicates that it’s the second line, fifth character of our src/main.rs file.

say you access 100th element of array of length 99, on running it shows that we can set RUST_BACKTRACE environment variable to get a backtrace of exactly what happened to cause the error.
`BACKTRACE`: list of all the functions that have been called to get to this point.

## Recoverable Errors with Result
okay say you wanna access a file and open it via rust, now it either maybe open to
access or not at all, if open then access the file else panic->
```rs
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
```

on compiling shows that thread panicked cause no such file found.
When the result is Ok, this code will return the inner file value out of the Ok variant, and we then assign that file handle value to the variable greeting_file. After the match, we can use the file handle for reading or writing.
The other arm of the match handles the case where we get an Err value from File::open. In this example, we’ve chosen to call the panic! macro. If there’s no file named hello.txt in our current directory and we run this code, we’ll see the following output from the panic! macro:
```bash
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/error-handling`
thread 'main' panicked at src/main.rs:8:23:
Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Matching on Different Errors
```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {    //file simply doesn't exist; ErrorKind is a struct
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {    //file has someother errors
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}
```

so we can actually use `match` with `Ok and Err`
alt code->
```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
```

## UnWrap and Expect
If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro for us. Ex:
```rs
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

`expect` method lets us also choose the `panic!` error message. 
Using `expect` instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier
let us create own custom panic! error message->
```rs
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

## Propogating Errors
instead of handling the error within the function itself you can return the error to the calling code so that it can decide what to do.
```rs
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> { //function is returning a value of type result<T,E>;T is filled with String and E with Error
    let username_file_result = File::open("hello.txt"); //open `hello.txt` file

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

## Shortcut: use `?`
```rs
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```
When the `?` operator calls the from function, the error type received is converted into the error type defined in the return type of the current function.

where to use-> in functions whose return type is compatible with the value the ? is used on. 
reason->defined to perform an early return of a value out of the function, in the same manner as the match expression

fixes:
- change the return type of your function to be compatible with the value you’re
using the ? operator on as long as you have no restrictions preventing that.
- use a match or one of the Result<T, E> methods to handle the Result<T, E> in whatever way is appropriate.

`?` can also be used with `Option<T>`-> similar to its behavior when called on a `Result<T, E>`

it can extract slice string
`?` on result returns `Result`; on `Option` returns `Option`;no typecasting b/w
`Result` and `Option`
solution-> use methods like the `ok` method on `Result` or the `ok_or` method on `Option` to do the conversion explicitly.

main can return `Result<(), E>`

## To panic! or Not to panic!
use panic when the error which is returned is unrecoverable
`unwrap`/`expect`-> u know there is ok value but compiler doesn't.

## Cases in Which You Have More Information Than the Compiler
example->
```rs
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
```

here we know that it is acceptable but compiler will return `Result` as `Err`
because the compiler isn’t smart enough to see that this string is always a
valid IP address.

## Creating Custom Types for Validation
remember that number guessing game.We never validated that the user’s guess was between those numbers before checking it against our secret number; we only validated that the guess was positive.

One way to do this would be to parse the guess as an i32 instead of only a u32 to allow potentially negative numbers, and then add a check for the number being in range

```rs
    loop {
        // --snip--

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num, //we are checking for a numerical value
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {   //matching for our value
            // --snip--
    }

```
`if`-> check for number in range, but not ideal solution cause having a check like this in every function would be tedious (and might impact performance).

solution-> make a new type and put the validations in a function to create an instance of the type rather than repeating the validations everywhere
```rs
pub struct Guess {  //stores the number
    value: i32, //declared type of variable
}

impl Guess {
    pub fn new(value: i32) -> Guess {   //instance creation and return is Guess
        if value < 1 || value > 100 {   //checks for range
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

## Summary
`panic!` macro-> your program is in a state it can’t handle and lets you tell the process to stop instead of trying to proceed with invalid or incorrect values. 
`Result` enum-> uses Rust’s type system to indicate that operations might fail in a way that your code could recover from; handle success and failure.



# Chap 10
# Generics
used to remove code repetetion;
used to define your own types, functions, and methods with generics!
Generics allow us to replace specific types with a placeholder that represents multiple types to remove code duplication.

like generics in cpp
```rs
fn main() {
    let bigger = largest(1, 2);
    let bigger_char = largest( 'a', 'b');
    println!("{}", bigger);
    println!("{}", bigger_char);
}
fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T{
    if a > b {a}
    else{b}
}
```
args are T, return type is also T, just trait bound it

see like want to find larget in a list so 2 ways->
- do in function itself
- another function to be made and then called

changes b/w two->
1. Identify duplicate code.
2. Extract the duplicate code into the body of the function, and specify the inputs and return values of that code in the function signature.
3. Update the two instances of duplicated code to call the function instead.

so like say we have 2 type of vectors, like one for i32, othr for char, both
needs to return largest, so both will have diff functions with diff data-type
and diff return-type, but notice they will have same logic->
```rs
for item in list {
        if item > largest {
            largest = item;
        }
    }
largest
```
both will just have diff wrapping but core logic will be this only.

now generalised logic->
use a `Type` Parameter named `T`{by convention in rust}; declare in signature,also so the compiler knows what that name means.

Similarly, when we use a type parameter name in a function signature, we have to declare the type parameter name before we use it.
place type name declarations inside angle brackets, <>, between the name of the function and the parameter list:
`fn largest<T>(list: &[T]) -> &T {` -> `largest` is a generic function over type `T`; returns a `reference` to a value of type `T`.

## In Structs
We can also define structs to use a generic type parameter in one or more fields using the <> syntax.
```rs
struct Point<T> {   //says Point<T> struct is generic over some type T
    x: T,   //both x,y are of same type T; instead of declare like i32 or f64 just used `T`
    y: T,
}
//note: both will need to have same data type in single ref, like {u32,u32} or {i32,i32} only, not like {u32,i32} etc.
fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

use multiple generic type parameters->
```rs
struct Point<T, U> {
    x: T,
    y: U,
}
```

## In Enums
```rs
enum Option<T> {    //enum is generic over type T
    Some(T),    //variant 1: hold value of type T
    None,   //doesn't holds any value
}
```

## In Methods
```rs
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {  //a/f declaring T, compiler identifies that arg is generic rather than concrete
    fn x(&self) -> &T { //defined a method named X on T implmented on type Point<T>
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
```

now->
```rs
impl Point<f32> {       //only avaliable for f32 data type, so other instances can't use the method
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

## Performance of code using Generics
there is no runtime cost when using generic type parameters. `Monomorphization` is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. 
the compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.


# Traits
similar to interface in TS,JS; defines functionality of a type which can be shared to other types.
it's like a blueprint for Structs to follow.
Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.
```rs
pub trait Summary {
    fn summarize(&self) -> String;
}
```

Implementing a trait on a type is similar to implementing regular methods. The difference is that after impl, we put the trait name we want to implement, then use the for keyword, and then specify the name of the type we want to implement the trait for.
can have multiple method in body.

implement trait on a type->
```rs
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```
it is similar to implementing regular methods.Instead of adding a semicolon after each signature, we use curly brackets and fill in the method body with the specific behavior that we want the methods of the trait to have for the particular type.

## Default Implementation
have a default implementation pre-defined Then, as we implement the trait on a particular type, we can keep or override each method’s default behavior.
the syntax for overriding a default implementation is the same as the syntax for implementing a trait method that doesn’t have a default implementation.

Default implementations can call other methods in the same trait, even if those
other methods don’t have a default implementation.
```rs
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```


## Traits as Parameters/Arguments
`impl trait` is syntactical sugar
it get converted to something else

syntax:
```rs
pub fn notify<T:Summary>(item:T){
    println!("Breaking news! {}",item.summarize());
}
```
bound to single trait
multiple trait bound-> `<T: Summary+fix>`

## Trait Bound Syntax
syntax sugar for longer form known as trait bound
```rs
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

## Multiple Trait bounds with + Syntax
say we want to use both `Summary` and `Display` implementation
`pub fn notify(item: &(impl Summary + Display)) {`
for generics->`pub fn notify<T: Summary + Display>(item: &T) {`

## with `where` clause
Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read.
Use `where`->

old:
```rs
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

after `where`:
```rs
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```
## Returning Types That Implement Traits
```rs
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```
By using impl Summary for the return type, we specify that the returns_summarizable function returns some type that implements the Summary trait without naming the concrete type. In this case, returns_summarizable returns a Tweet, but the code calling this function doesn’t need to know that.

## Using Trait Bounds to Conditionally Implement Methods
By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.
```rs
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

`Blanket Traits`: conditionally implement a trait for any type that implements another trait.
Implementations of a trait on any type that satisfies the trait bounds. ex:
```rs
impl<T: Display> ToString for T {
    // --snip--
}
```
`Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior.`

# Lifetimes
Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.
so basically `lifetime is span where variable and function are valid`

so like s1 and s2 has lifespan `'a`.
so ans lifespan is intersection of both
so basically space where s1, s2 is valid both, their return type is valid at
their intersection

say you have s2 inside s1 so return type will always will be in s2, as it is
intersection s1 ans s2.

Example->
```rs
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+
```
r has a lifetime of 'a but refers to memory with lifetime of 'b. hence a
dangling pointer error cause `'b<'a`. subject doesn't lives as long as
reference.

```rs
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+
```
x has lifetime `'b` >`'a`=> `r` can reference `x`


## Function with Lifetimes
say to print longest string b/w 2
so u define a function longest() utilise the size function 
now for main function u call a variable initially
but after input you store that function into that variable initialises initially
quite weird right
declare first, assign later
```rs
fn longest(a:String,b:String)->String{
    if a.len()>b.len(){
        return a;
    }else{return b;}
}

fn main(){
    let longest_str;
    let str1=String::from("small");
{
        let str2=String::from("longer");
        longest_str=longest(str1,str2);
    }
    println!("{}",longest_str);
}
```

Write a function that takes two string `references` as an input And returns the bigger amongst them

```rs
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```


consider this
```rs
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```
error because the function must return a lifetime of reference of string `a`,
but it returns nothing like that, hence it will be a compilation error.


## Structs with lifeTimes
same but struct is also treated a function.
We can define structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition.

```rs
struct User<'a,'b>{
    first_name:&'a str,
    last_name:&'b str,`
}
//instance of User can't outlive the eference it holds in its first_name,last_name field.
fn main(){
    let user:User;
    let first_name=String::from("Nalin");
//lifetime is just inside that
{
        let last_name=String::from("Singh");
        user=User{first_name:&first_name,last_name:&last_name};
    }
    println!("The name of user is {}",user.first_name);
}
```
Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.
basically compiler has three rules to check for lifetimes of references.
1. compiler assigns a lifetime parameter to each parameter that’s a reference.
`fn first_word(s: &str) -> &str {` converts to `fn first_word<'a>(s: &'a str) -> &str {`
2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
`fn first_word<'a>(s: &'a str) -> &str {` converts to `fn first_word<'a>(s: &'a str) -> &'a str {`
3. if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

```rs
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {    //fucking error, cause a dangling pointer error;we still haven’t figured out what the return type’s lifetime is.
```

## Static Lifetime
```rs
let s: &'static str = "I have a static lifetime.";
//'static, which denotes that the affected reference can live for the entire duration of the program
//basically extend the whole lifetime till the end of program
```
lifetime of all string literal is `'static`.

## Generic Type Parameters, Trait Bounds, and Lifetimes Together
```rs
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
lifetime specified is of type `T`{generic}, hence as you pass things like a
string passed so will implement it according to that string.

# Chap 11 - Automated Test
Correctness in our programs is the extent to which our code does what we intend it to do.
Rust has type safety, but we can't expect it to catch everything.Rust includes support for writing automated software tests.

Rust has three attributes for writing test:
- test attribute, 
- a few macros, and 
- should_panic attribute.

## The Anatomy of a Test Function
Let’s create a new library project called adder that will add two numbers:
$ cargo new adder --lib
     Created library `adder` project
$ cd adder

change a function to test function->
add #[test] b/f fn, run with cargo test
```rs
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```
Note the #[test] annotation: this attribute indicates this is a test function

simply run `cargo test` to check it out.
the cli will show you the test like:
```bash
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.59s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Note: Tests fail when something in the test function panics. Each test is run in
a new thread, and when the main thread sees that a test thread has died, the
test is marked as failed.

## Check via `assert!` Macro
when you want to check if a condition in a test evaluates to `true`.We give the `assert!` macro an argument that evaluates to a Boolean. If the value is `true`, nothing happens and the test passes. If the value is `false`, the `assert!` macro calls `panic!` to cause the test to fail.
`true`->`nothing`
`fail`->`panic`

```rs
#[cfg(test)]
mod tests {
    use super::*;   //tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module. 

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
```
runs smoothly.

## Testing Equality with the `assert_eq!` and `assert_ne!` Macros
well we can always check the expected result and actual with `==` but STL has a pair of `macros—assert_eq!` and `assert_ne!`—to perform this test more conveniently.
example->
```rs
pub fn add_two(a: usize) -> usize {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
```
upon running we get ->
```bash
running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

on failing testcases gives something like:
```bash

failures:

---- tests::it_adds_two stdout ----
thread 'tests::it_adds_two' panicked at src/lib.rs:12:9:
assertion `left == right` failed
  left: 5
 right: 4
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

bug is caught where it tells that the assertion is failing giving us context to
work on

`assert_ne!` macro will pass if the two values we give it are not equal and fail if they’re equal.

Under the surface, the `assert_eq!` and `assert_ne!` macros use the operators == and !=, respectively.

## Adding Custom Failure Messages
Custom messages are useful for documenting what an assertion means; when a test fails, you’ll have a better idea of what the problem is with the code.
```rs
pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

```

O/P->
```bash

failures:

---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at src/lib.rs:12:9:
Greeting did not contain name, value was `Hello!`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

## Checking for Panics with should_panic
by adding the attribute should_panic to our test function. The test passes if the code inside the function panics; the test fails if the code inside the function doesn’t panic.

```rs
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```
runs smoothly cause no error in the code, testing is like-> 0 failed, 1 passed

say, now we introduce a bug:
```rs
// --snip--
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}
```
upon test it gives that it fails, but not a very helpful message.The failure we got means that the code in the test function did not cause a panic.

## Using `Result<T, E>` in Tests
```rs
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
```
The `it_works` function now has the `Result<(), String>` return type.

You can’t use the ``#[should_panic]`` annotation on tests that use `Result<T, E>`. To assert that an operation returns an Err variant, don’t use the question mark operator on the `Result<T, E>` value. Instead, use `assert!(value.is_err())`.

## Controlling How Tests Are Run
`cargo test` create a binary and runs it as a test
`cargo test --help` displays the options you can use
`cargo test -- --help` displays option after --

## Running Tests in Parallel or Consecutively
by default they run in parallel using threads,
things to keep in midn:
- your tests don’t depend on each other 
- don't depend on any shared state,

`$ cargo test -- --test-threads=1`-> cmd to run
- set the number of test threads to 1, telling the program not to use any parallelism.
- using one thread will take longer than in parallel.

## Showing Function Output
when you run `cargo test`, it only tells how many passed and how many not
for the output use : `$ cargo test -- --show-output`
shows o/p for every test
```$ cargo test -- --show-output
   Compiling silly-function v0.1.0 (file:///projects/silly-function)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.60s
     Running unittests src/lib.rs (target/debug/deps/silly_function-160869f38cff9166)

running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

successes:

---- tests::this_test_will_pass stdout ----
I got the value 4


successes:
    tests::this_test_will_pass

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'tests::this_test_will_fail' panicked at src/lib.rs:19:9:
assertion `left == right` failed
  left: 10
 right: 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

## Run Subtest by name
just run a specific test by name rather than all of them->
`cargo test one_hundred`

## Filter to run multiple
runs all test with some common name like `add` and `add_two`
```sh
cargo test add
```

## Ignore explicitly called
just add `#[ignore]` after them
ignore the test function compilation
```rs
#[test]
#[ignore]
fn works(){
let result=add(2,4);

```

to run the ignored one-> `cargo run --ignored`

## Test organization
2 types of test are their-> `Unit`,`Integration`

### Unit Testing
test each unit of code in isolation from the rest of the code
convention: create `test` module in each file
module annotation: `cfg(test)`
runs only when you run `cargo test` doesn't run on `cargo build`
```rs
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]        //cfg-configuration
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

### Integration Testing
checks if code works correctly as a module, like as in collectively not file wise
create a `test` folder next to `src` directory
tests/integration_test.rs->
```rs
use adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
```

o/p-->
```bash
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.31s
     Running unittests src/lib.rs (target/debug/deps/adder-1082c4b063a8fbe6)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-1082c4b063a8fbe6)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
3 parts-> unit, integration, doc

The integration tests section starts with the line `Running tests/integration_test.rs`

run a particular test only-> `cargo test --test test_name`

### Submodules in Integration Tests
like `integration_test.rs` and then submodule `test/common/lib.rs`
hence it can be used anywhere as a module

# Chap 13
# Iterators and Closures
rust also supports functional program
using functions as values by passing them in arguments, returning them from other functions, assigning them to variables for later execution, and so forth.

we’ll cover:
- Closures, a function-like construct you can store in a variable
- Iterators, a way of processing a series of elements

## Closures
anonymous functions you can save in a variable or pass as arguments to other functions.
define elsewhere, call elsewhere- perfectly ok
closures can capture values from the scope in which they’re defined.



# MultiThreading
run mutliple independents parts in single process
this parts are called threads
an executed program’s code is run in a process, and the operating system will manage multiple processes at once. Within a program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.

ex: `thread.rs`

We’ll often use the `move` keyword with closures passed to `thread::spawn` because the closure will then take ownership of the values it uses from the environment, thus transferring ownership of those values from one thread to another. In the “Capturing References or Moving Ownership” section of Chapter 13, we discussed move in the context of closures. Now, we’ll concentrate more on the interaction between move and thread::spawn.

# Message Passing/Channel
passing over a variable, lie delegating a process in parts to 10 diff cpu, or core
channel, 2 part: transmitter and receiver
A channel has two halves: a transmitter and a receiver. 
The transmitter half is the upstream location where you put rubber ducks into the river, and the receiver half is where the rubber duck ends up downstream. 
One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages. 
A channel is said to be closed if either the transmitter or receiver half is dropped.

`channel.rs` file

# Macro
basically to expand single line into multiple lines
powerful feature that allows for metaprogramming by enabling the generation of code at compile-time

### Key Concepts of Rust Macros:

1. **Code Generation**: Macros allow you to define a pattern for generating code. This means you can write code that produces other code, reducing redundancy and boilerplate.
2. **Metaprogramming**: Rust macros are a form of metaprogramming because they allow you to write code that writes or manipulates other code. This can be useful for tasks like reducing boilerplate, creating domain-specific languages (DSLs), or automating repetitive patterns.

2 types: Declarative, Procedural

expand the code via -> `cargo expand`
## Declarative Macro:
replace the code written with a different code during compile time
```rs
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

fn main() {
    say_hello!();  // Expands to: println!("Hello, world!");
}
```

## Procedural Macro:
more complex macros that allow you to define custom behavior for code generation through Rust code itself
```rs

#[derive(Serialize, Deserialize)]
struct User {
	username: String,
	password: String,
	age: u32
}
```

### Types of procedural macros

1. **Custom derive macros**

Custom derive macros allow you to define how Rust derives certain traits for types. A common use case is generating code for trait implementations (like `Debug`, `Clone`, etc.).

```rust
#[derive(Serialize, Deserialize)]
struct User {
	username: String,
	password: String,
	age: u32
}
```

1. **Attribute-like Macros**:

```rust
#[route("GET")]
fn home() {
    println!("Welcome to the home page!");
}

#[route("POST")]
fn create_post() {
    println!("Creating a new post!");
}
```

1. **Function like macros**

https://github.com/100xdevs-cohort-3/proc-macro/

### Macros applied to attributes

```rust
cargo add serde serde_json
// Update serde to use the derive feature
serde = {version = "1.0.218", features = ["derive"]}
```

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    #[serde(rename = "user_name")]
    username: String,
    
    #[serde(rename = "pass_word")]
    password: String,
    
    #[serde(rename = "user_age")] 
    age: u32,
}

fn main() {
    let user = User {
        username: String::from("Alice"),
        password: String::from("password123"),
        age: 30,
    };

    // Serializing to JSON
    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json); 
    // Prints: {"user_name":"Alice","pass_word":"password123","user_age":30}
}

```

Write a macro that can take more than one function name as input and create
functions for it.-> macro3.rs
Copy and Clone are 2 traits
Copy-> Just copy the value
Clone-> pass the ownership, expensive


---------------------------------------------
remaining:
Macros
8. Futures
9. Async/await and tokio

---------------------------------------
# Project Idea
1. Backend for a full stack app
2. CLIs
---------------------------------------
