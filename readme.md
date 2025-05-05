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

### Capturing the Environment with Closures
example->
 Every so often, our t-shirt company gives away an exclusive, limited-edition shirt to someone on our mailing list as a promotion. People on the mailing list can optionally add their favorite color to their profile. If the person chosen for a free shirt has their favorite color set, they get that color shirt. If the person hasn’t specified a favorite color, they get whatever color the company currently has the most of.

`Closure.rs` file-> built 3 tshirt using a `store` method, 2 blue, 1 red; called
a `giveaway` method for both guys, one of them has a preference for a red one.

### Closure Type Inference and Annotation
Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the compiler can infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables (there are rare cases where the compiler needs closure type annotations too).

add type annotations if we want to increase explicitness and clarity at the cost of being more verbose than is strictly necessary.
```rs
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

similar to:
```rs
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```
1st line shows a function definition,
2nd line shows a fully annotated closure definition.
3rd line, we remove the type annotations from the closure definition.
4th line, we remove the brackets, which are optional because the closure body has only one expression.

For closure definitions, the compiler will infer one concrete type for each of
their parameters and for their return value.

### Capturing References or Moving Ownership
3 ways to capture values:
- borrowing immutably, 
- borrowing mutably, and 
- taking ownership
```rs
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");   //way to call
    //closure

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
```
We don’t use the closure again after the closure is called, so the mutable borrow ends
Between the closure definition and the closure call, an immutable borrow to print isn’t allowed because no other borrows are allowed when there’s a mutable borrow.ex:
```rs
fn main() {
    let mut list = vec![1, 2, 3];

    // Define a closure that mutably borrows `list`
    let mut borrows_mutably = || list.push(4);

    // ❗ Trying to immutably borrow here before the closure is called
    println!("Before calling closure: {list:?}");

    // Now we try to call the closure which needs a mutable borrow
    borrows_mutably();
}
```

`error[E0502]: cannot borrow 'list' as immutable because it is also borrowed as mutable`
`closure` don't have like lifetimes hardcoded like it's not a thing for them,
but if you want to enforce it then use `move` to spawn a new thread.
```rs
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
```
specify that list should be moved into the closure by putting the move keyword at the beginning of the closure definition.

### Moving Captured Values Out of Closures and the Fn Traits
Closure captures references and lifetimes, and the body itself decides what to do with it:
- move a captured value out of the closure, 
- mutate the captured value, 
- neither move nor mutate the value, or 
- capture nothing from the environment to begin with.

the capturing itself decides their working.automatically implement one, two, or all three of these Fn traits:
1. `FnOnce` applies to closures that can be called once. All closures implement this trait, cause all closures can be called. 
A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.

2. `FnMut` applies to closures that don’t move captured values out of their body, but that might mutate the captured values. 
These closures can be called more than once.

3. `Fn` applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.
example:
```rs
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

## Processing a Series of Items with Iterators
iterators help to iterating over each item and determining when the sequence has finished.
iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.
```rs
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
```

separate the declaration and use:
```rs
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
```


## The `Iterator` Trait and the `next` Method
```rs
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```
`Item` is type of return value
implementing this method->
```rs
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

```
code consumes, or uses up, the iterator
`iter`->the values we get from the calls to next are immutable references to the values in the vector
`into_iter`-> iterator that takes ownership of v1 and returns owned values
`iter_mut`-> iterate over mutable references

## Methods that Consume the Iterator
various implementation in stl
Methods that call `next` are called **consuming adapters** b/c upon calling them
they are consumed.
```rs
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
```
We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on.

## Methods that Produce Other Iterators
Iterator adapters are methods defined on the Iterator trait that don’t consume the iterator.call the iterator adapter method map, which takes a closure to call on each item as the items are iterated through .
map method returns a new iterator that produces the modified items.
```rs
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
```
produces warning, b/c the iterator is not consumed, do like: `let _ = v1.iter().map(|x| x + 1);`

## Using Closures that Capture Their Environment
iteraotrs use closure as argument, if closure is true then include item, else
skip
closure gets an item from the iterator and returns a bool
```rs
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {       //takes ownership of a vector of shoes and a shoe size as parameters.
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    //call into_iter to create an iterator that takes ownership of the vector.
    //filter to adapt that iterator into a new iterator that only contains elements for which the closure returns true.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```

## Loops v/s Iterators
say we look for a word in a string with help of both `iter` and `for`, both have
same performance, not once one is fast than other
implementations of closures and iterators are such that runtime performance is not affected

# Chap 14
# Cargo and Crates
Cargo can help to manage packages on rust, crates is sort of standard place to
publish libraries for rust, install binaries from there.

## Customising build with release profile
release profiles are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code.
2 profiles: `dev` and `release`
`dev` : when we run `cargo build`
`release`: when we run `cargo build --release`
can control the build with help of Cargo.toml file
```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3       #controls the number of optimizations
```
keep it as it is, cause in release mod upon more compilation it takes more time
hence affecting prod.

## Publishing a Crate to Crates.io

do comments in code you are going to publish as packages
given description,
then heading

    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let answer = my_crate::add_one(arg);
    ///
    /// assert_eq!(6, answer);
    /// ```
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }


then code snippet
run `cargo doc --open`; it will build the HTML for your current crate’s documentation.

## Documentation Comments as Tests
`cargo test` will run the code examples in your documentation as tests! 
after this if u change it, it crash and the code `panics`

## Commenting Contained Items
`//!` adds the comments as documentation itself

## Exporting a Convenient Public API with pub use
like use a `pub` function
define a `public` module, have a `public` module
then u can import it.
```rs
//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
```
use it like:
```rs
use art::kinds::PrimaryColor;
use art::utils::mix;
```
now the problem here is that need to remember what is where
hence to prevent further confusion for user do like:
```rs
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}
```

to publish crates, need to have a crate account, login on `crates.io/me` with
github, get api key, then on terminal `cargo login` paste the key
add metadata in cargo.toml file then `cargo publish` to publish the crate
```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

`cargo yank`- prevent new projects from depending on a specific version of a crate that's already been published to crates.io.
avoid the dependency we provide in args

```sh
$ cargo yank --vers 1.0.1   #yank version 1.0.1
$ cargo yank --vers 1.0.1 --undo    #undo the yank
```

## 14.3 Cargo Workspace
you want to split your package further into multiple library crates. -> use `Cargo Workspace`
they can help manage multiple related packages that are developed in tandem.

A workspace is a set of packages that share the same Cargo.lock and output directory
to get started, make a normal rust project or intitalise a empty directory with
cargo
```toml
[workspace]
resolver = "2"
```

create the adder binary crate by running cargo new within the add directory:
```sh
cargo new adder
```
it add a new rust project inside current rust project

note: `even after initialising the nested rust directory, there exist only one
target folder`

say u add another directory, and want to use one into other, update the toml
file as:
```toml
[dependencies]
add_one = { path = "../add_one" }
```

To run the binary crate from the add directory, we can specify which package in the workspace we want to run by using the -p argument and the package name with cargo run:
```sh
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

### External Packages
go to `Cargo.toml` file of required project; add following:
```toml
[dependencies]
rand = "0.8.5"
```

## Adding a Test to a Workspace
let’s add a test of the `add_one::add_one` function within the `add_one` crate:
```rs
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

to run it down:
`$ cargo test -p add_one`

All binaries installed with cargo install are stored in the installation root’s bin folder
```sh
cargo install ripgrep
```

# Chap 15
# Smart Pointer
pointer we know that they point to something
in Rust they are known by name `References`
now rust has some complex pointer also
Smart pointer enables you to allow data to have multiple owners by keeping track of the number of owners and, when no owners remain, cleaning up the data.


while references only borrow data, in many cases, smart pointers own the data they point to.

implemented using structs.they implement the `Deref` and `Drop` traits.
- `Deref trait` allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers
- `Drop trait` allows you to customize the code that’s run when an instance of the smart pointer goes out of scope

We'll cover :`Box<T>`,`Rc<T>`,`Ref<T>`
- `Box<T>` for allocating values on the heap
- `Rc<T>`, a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time

## `Box<T>`
most straightforward smart pointer
store data on the heap

uised in these situations:
- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

### Using Box<T> to store data on Heap
```rs
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
```
variable b to have the value of a Box that points to the value 5, which is allocated on the heap.
when a box goes out of scope, as b does at the end of main, it will be deallocated.

a case where boxes allow us to define types that we wouldn’t be allowed to if we didn’t have boxes.

### Enabling Recursive Types with Boxes
A value of recursive type can have another value of the same type as part of itself.
issue: `Rust needs to known how much space is going to be used`
Solution: Use a `Con` list

#### Con List
introduced in lisp
made up of nested pairs, and is the Lisp version of a linked list.
ex: `(1, (2, (3, Nil)))`

2 parts: `value of the current item and the next item`
last item: only a value called Nil without a next item
ex:
```rs
enum List {
    Cons(i32, List),
    Nil,
}
```


ex: Storing 1,2,3
```rs
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

shows error on compilation, says infinite size
`Cons` needs an amount of `space` equal to the size of an `i32` plus the size of a `List`.
like say a Con has i32, and then other cons
`Con={i32+Con}`

#### Using `Box<T>` to Get a Recursive Type with a Known Size
instead of storing a value directly, we should change the data structure to store the value indirectly by storing a pointer to the value instead.

```rs
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

`Box<T>` type is a smart pointer because it implements the `Deref trait`, which allows `Box<T>` values to be treated like `references`. 
When a `Box<T>` value goes out of scope, the `heap data` that the box is pointing to is `cleaned up` as well because of the Drop trait implementation.

## Treating Smart Pointers Like Regular References with the `Deref` Trait
`Deref` trait allows you to customize the behavior of the dereference operator `*`
`Deref` trait makes it possible for smart pointers to work in ways similar to references.

### Following the Pointer to the Value
A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else
so `&x` is reference; `*y` is dereference
```rs
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);  //use *y to follow the reference to the value it’s pointing to
}
```
compilation gives error b/c Comparing a number and a reference to a number isn’t allowed because they’re different types. We must use the dereference operator to follow the reference to the value it’s pointing to.

### Using `Box<T>` Like a Reference
```rs
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```
box copies over value of x and y is it's instamce

### Defining Our Own Smart Pointer
how smart pointers behave differently from references by default. 
`Box<T>` type is ultimately defined as a tuple struct with one element;
Similarly:
```rs
struct MyBox<T>(T); //struct is defined, a generic parameter T is declared

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {  //MyBox::new function takes one parameter of type T and returns a MyBox instance that holds the value passed in.
        MyBox(x)
    }
}
```
then run it in:
```rs
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```
gives error cause rust doesn't knows how to dereference.
Our MyBox<T> type can’t be dereferenced because we haven’t implemented that ability on our type.

### Treating a Type Like a Reference by Implementing the Deref Trait
The Deref trait, provided by the standard library, requires us to implement one method named deref that borrows self and returns a reference to the inner data.
```rs
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0     //filled with it so deref returns a reference to the value we want to access with the * operator
    }
}
```
Without the Deref trait, the compiler can only dereference & references.

### Implicit Deref Coercions with Functions and Methods
Deref coercion converts a reference to a type that implements the Deref trait
into a reference to another type.
ex: deref coercion can convert `&String` to `&str` because String implements the Deref trait such that it returns &st
- Deref coercion is a convenience Rust performs on arguments to functions and methods, and works only on types that implement the Deref trait. 
- It happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition. 
ex:
```rs
fn hello(name: &str) {
    println!("Hello, {name}!");
}
fn main() {
    let m = MyBox::new(String::from("Rust"));   //Rust can turn &MyBox<String> into &String by calling deref
    hello(&m);      //function with the argument &m, which is a reference to a MyBox<String> value
}
```
`(*m)` dereferences the `MyBox<String>` into a `String`

### How Deref Coercion Interacts with Mutability
Rust does deref coercion when it finds types and trait implementations in three cases:

- From `&T` to `&U` when `T: Deref<Target=U>`
- From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
- From `&mut T` to `&U` when `T: Deref<Target=U>`

## Running Code on Cleanup with the Drop Trait
The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when a value is about to go out of scope
drop basically is used to clear the memory
when a Box<T> is dropped it will deallocate the space on the heap that the box points to.
ex:
```rs
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```
```sh
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```
Rust automatically called drop for us when our instances went out of scope, calling the code we specified.

### Dropping a Value Early with `std::mem::drop`
want to clean up a value early

Rust doesn’t let you call the `Drop` trait’s `drop` method manually; instead you have to call the `std::mem::drop` function provided by the standard library if you want to force a value to be dropped before the end of its scope.
```rs
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
}
```

gives error telling that we’re not allowed to explicitly call `drop`.
b/c rust also automatically calls `drop` a end of `main` function leading to
double memory free

custom:
```rs
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```
no error

## 15.4 Rc<T>, the Reference Counted Smart Pointer
consider a example where a single value has multiple owner, ex: `graph`.
multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it. 
A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it and so has no owners.

hence use `Rc<T>`;keeps track of the number of references to a value to
determine whether or not the value is still in use.

### Using `Rc<T>` to Share Data
We’ll create list a that contains 5 and then 10. Then we’ll make two more lists: b that starts with 3 and c that starts with 4. Both b and c lists will then continue on to the first a list containing 5 and 10. 
```md
        b(3)
         |
        a(5)
       /   \
    a(10)  a(Nil)
         |
        c(4)
```
```rs
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```
error: The Cons variants own the data they hold, so when we create the b list, a is moved into b and b owns a. Then, when we try to use a again when creating c, we’re not allowed to because a has been moved.

Instead, we’ll change our definition of List to use Rc<T> in place of Box<T>
```rs
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); //conv is to call Rc::clone(&a) instead of a.clone()
    let c = Cons(4, Rc::clone(&a)); //doesn't makes any deep copy, only increments the reference count
}
```

### Cloning an `Rc<T>` Increases the Reference Count
```rs
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

## 15.5 `RefCell<T>` and the Interior Mutability Pattern
`Interior mutability` is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data

We can use types that use the interior mutability pattern only when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can’t guarantee that.

### Enforcing Borrowing Rules at Runtime with `RefCell<T>`
`RefCell<T>` type represents single ownership over the data it holds, here
borrow rules are enforced at run time

in case of references, if you break these rules, you’ll get a compiler error. With RefCell<T>, if you break these rules, your program will panic and exit.

checking the borrowing rules at compile time is the best choice in the majority of cases, which is why this is Rust’s default.

The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed, where they would’ve been disallowed by the compile-time checks. Static analysis, like the Rust compiler, is inherently conservative

consider Halting problem, what if error code is compiled with ownership issues, or error-free code
doesn't compiles, it will create trust issues.

sol: `RefCell<T>` type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.

use this only for `single-threaded scenarios`; gives compilation error for
`multi-threaded scenarios`

- `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime.
- Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.

Mutating the value inside an immutable value is the `interior mutability pattern`.

### Interior Mutability: A Mutable Borrow to an Immutable Value
A consequence of the borrowing rules is that when you have an immutable value, you can’t borrow it mutably.
```rs
fn main() {
    let x = 5;
    let y = &mut x;
}
```
gives compilation error cause x is not mutably defined, but we are using it as
an mutable in y, sort of conflicting right.
also u just can't borrow mutable like that. creates dangling pointer error,
cuase what if it's cleared b/f it right.

use `RefCell<T>` to mutate an immutable value and see why that is useful.

### A Use Case for Interior Mutability: Mock Objects
test object: use a type in place of another type, in order to observe particular behavior and assert it’s implemented correctly.
stand in for other types when we’re running tests.

`Mock objects`: specific types of test doubles that record what happens during a test so you can assert that the correct actions took place.


ex: create a library that tracks a value against a maximum value and sends messages based on how close to the maximum value the current value is. This library could be used to keep track of a user’s quota for the number of API calls they’re allowed to make
```rs
pub trait Messenger {   //trait is the interface here which we want to use a
    //mock obj to behave similarly to real one
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```
mockmessenger can't be changed cause it has reference of an immutable value;
obviously don't wanna change it just for testing

interior mutability can help! store the sent_messages within a RefCell<T>, and then the send method will be able to modify sent_messages to store the messages we’ve seen
```rs
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,    //type changes to RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),    //hence instance is created arounf a empty vector here
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) { //still immutable ref
            self.sent_messages.borrow_mut().push(String::from(message));
            //called to make it mutable
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

### Keeping Track of Borrows at Runtime with `RefCell<T>`
`&`, `&mut` are gnrlly used for references, but say we use `borrow` and `borrow_mut` methods, which are part of the safe API that belongs to `RefCell<T>`.

- `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active. 
- Every time we call borrow, the `RefCell<T>` increases its count of how many
immutable borrows are active.
- When a `Ref<T>` value goes out of scope, the count of immutable borrows goes `down by one`.
- if rules are violated then instead of compilation error, we get run time panic
ex:
```rs
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }
```
here we borrowed a value, then w/o clearing it borrowed it again
code panicked with the message already `borrowed: BorrowMutError`. This is how `RefCell<T>` handles violations of the borrowing rules at runtime.

### Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`
use `RefCell<T>` is in combination with `Rc<T>`
`Rc<T>`- multiple owners of some data, but only immutable access .
`Rc<T>` that holds a `RefCell<T>`- a value that can have `multiple owners` and that you can `mutate`!
ex:
```rs
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));   //instance of Rc<RefCell<i32>>  stored in a variable named value

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
```
```sh
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```

By using `RefCell<T>`, we have an outwardly immutable List value.

`RefCell<T>` does not work for `multithreaded code❌`! 
`Mutex<T>` : thread-safe version of `RefCell<T>`

## 15.6 Reference Cycles Can Leak Memory
it is difficult but not impossible to accidently create memory that is never cleaned up (known as a memory leak). 

it’s possible to create references where items refer to each other in a cycle. 
This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.

### Creating a Reference Cycle
```rs
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());}
```
creates a list in a and a list in b that points to the list in a. Then it modifies the list in a to point to b, creating a reference cycle
o/p:
```bin
$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
     Running `target/debug/cons-list`
a initial rc count = 1
a next item = Some(RefCell { value: Nil })
a rc count after b creation = 2
b initial rc count = 1
b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
b rc count after changing a = 2
a rc count after changing a = 2
```
ref count for a,b=2
at end bis dropped, ref count=1
then a is also dropped, ref count=1

 instance’s memory can’t be dropped, because the other Rc<List> instance still refers to it
memory allocated to the list will remain uncollected forever.

rust can't catch memory cycles, avaoid making them
reorganizing your data structures so that some references express ownership and some references don’t.

### Preventing Reference Cycles: Turning an `Rc<T>` into a `Weak<T>`
create a weak reference to the value within an Rc<T> instance by calling Rc::downgrade and passing a reference to the Rc<T>

They won’t cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involved is 0.

When you call Rc::downgrade, you get a smart pointer of type Weak<T>, hence by
deafult count of strong one is still 1, and count of weak is now increased to 1

ex: creating a tree-> `tree.rs`

# Chap 16
# Fearless Concurrency
Concurrent programming: where different parts of a program execute independently.
parallel programming: where different parts of a program execute at the same time

ownership and type systems are a powerful set of tools to help manage memory safety and concurrency problems!
you can fix your code while you’re working on it rather than potentially after it has been shipped to production.
topics to be covered:
- How to create threads to run multiple pieces of code at the same time
- Message-passing concurrency, where channels send messages between threads
- Shared-state concurrency, where multiple threads have access to some piece of data
- The Sync and Send traits, which extend Rust’s concurrency guarantees to user-defined types as well as types provided by the standard library


# 16.1 MultiThreading
run mutliple independents parts in single process
this parts are called threads
an executed program’s code is run in a process, and the operating system will manage multiple processes at once. Within a program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.

ex: `thread.rs`

We’ll often use the `move` keyword with closures passed to `thread::spawn` because the closure will then take ownership of the values it uses from the environment, thus transferring ownership of those values from one thread to another. In the “Capturing References or Moving Ownership” section of Chapter 13, we discussed move in the context of closures. Now, we’ll concentrate more on the interaction between move and thread::spawn.

since multithreading means to run 2/more code simultaneously, it may lead to:
- Race conditions, where threads are accessing data or resources in an inconsistent order
- Deadlocks, where two threads are waiting for each other, preventing both threads from continuing
- Bugs that happen only in certain situations and are hard to reproduce and fix reliably

**Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread.**

To create a new thread, we call the thread::spawn function and pass it a closure
```rs
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));        //thread::sleep is
            //responsible for force stopping the thread for a small amount of time
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
```

## Waiting for All Threads to Finish Using join Handles
spawned thread are stopped prematurely b/c:
- due to the main thread ending
- there is no guarantee on the order in which threads run
- can’t guarantee that the spawned thread will get to run at all!

fix: save the return value of `thread::spawn` in a variable
return type `JoinHandle`. `JoinHandle` is an owned value that, when we call the `join` method on it, will wait for its thread to finish.
```rs
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```
first main then spawned one, then main then spawned one, when the main is
completed spawned runs freely, notice not asynchronous, but synchronization run
time.

move handle.join() before the for loop in main, like this:
```rs
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
```
first the spawned one runs completely then the main thread runs

**Note: Small details, such as where `join` is called, can affect whether or not your threads run at the same time.**

### Using move Closures with Threads
use the `move` keyword with closures passed to `thread::spawn` because the closure will then take ownership of the values it uses from the environment, thus transferring ownership of those values from one thread to another. 

note: thread::spawn doesn't takes any arguments, but it needs to capture the value; ex:
```rs
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
```
closure uses v, so it will capture v and make it part of the closure’s environment
but thread::spawn runs it in new env, hence need to be accessible their, so rust
borrows it, but can't tell how much long the thread will run so the ref can't ve
validated for specific time.

consider:
```rs
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
```
here the v is running in a thread and main function, now thread runs for how
long can't say, and when the main drops it.when the spawned thread starts to execute, v is no longer valid, so a reference to it is also invalid.

solution:
```rs
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
```

```sh
Here's a vector: [1, 2, 3]
```

## 16.2 | Using Message Passing to Transfer Data Between Threads | Message Passing/Channel
A channel is a general programming concept by which data is sent from one thread to another.


passing over a variable, lie delegating a process in parts to 10 diff cpu, or core
A channel has two halves: a transmitter and a receiver. 
The transmitter half is the upstream location where you put rubber ducks into the river, and 
the receiver half is where the rubber duck ends up downstream. 
One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages. 
A channel is said to be closed if either the transmitter or receiver half is dropped.
```rs
use std::sync::mpsc;
use std::thread;
 
fn main(){
    let (tx,rx)=mpsc::channel();    //returns a tuple of two channel: sender end, receiver end
    //tx: transmitter end; rx: receiver end
    thread::spawn(move || {
        tx.send((String)::from("hello world"));
        //send method to send some data
        //returns a Result<T, E> type; handles error bydefault
    });
let received = rx.recv().unwrap();
    println!("Got: {received}");
    match value{
        Ok(value)=>println!("{}",value);
Err(err)=>println!("Error while reading the data"),
    }
}
```

`channel.rs` file
`msgpassing.rs` file -> message passing b/w files

### Channels and Ownership Transference
consider following code:
```rs
thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {val}");
    });
```
why do you this is wrong?
it is wrong b/c the `val` data is sent, hence anything may happen with it, say
that antoher thread deletes it, so when calling the print , it may lead to
dangling pointer error, make sense right.
`send` function takes ownership of its parameter, and 
when the value is moved, the `receiver` takes ownership of it


### Sending multiple values and receiving them
```rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        //create a thread

        for val in vals {
            tx.send(val).unwrap();  //contents of thread are sent
            thread::sleep(Duration::from_secs(1));  //have a delay/pause of 1 second
        }
    });

    for received in rx {    //rx is treated as iterator
        println!("Got: {received}");
    }
}
```

```sh
Got: hi
Got: from
Got: the
Got: thread

```

### Creating Multiple Producers by Cloning the Transmitter
`mpsc` - multiple producer, single consumer
`mpsc.rs`

## 16.3 Shared State Concurrency
handling concurrency:
- Message passing
- multiple threads to access the same shared data

### Using Mutexes to Allow Access to Data from One Thread at a Time
a `Mutex(mutual exclusion)` allows only one thread to access some data at any given time. guarding the data it holds via the locking system.
2 rules:
- You must attempt to acquire the lock before using the data.
- When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

ex: a panel discussion at a conference with only one microphone. Before a panelist can speak, they have to ask or signal that they want to use the microphone. When they get the microphone, they can talk for as long as they want to and then hand the microphone to the next panelist who requests to speak. If a panelist forgets to hand the microphone off when they’re finished with it, no one else is able to speak. If management of the shared microphone goes wrong, the panel won’t work as planned!

It is a smart pointer

#### API of `Mutex<T>`
ex:
```rs
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();    //lock() used to acquire the locks
//all to lock would fail if another thread holding the lock panicked; fails and
//panic if other prcoess is holding lock
        *num = 6;
    }

    println!("m = {m:?}");
}
```

the call to `lock` returns a smart pointer called **MutexGuard**, wrapped in a **LockResult** that we handled with the call to unwrap

MutexGuard smart pointer implements Deref to point at our inner data

a Drop implementation that releases the lock automatically when a MutexGuard goes out of scope,

### Sharing a Mutex<T> Between Multiple Threads
spin upto 10 threads as the times goes on from 0 to 10 using `mutex<T>`.
```rs
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```
closure: one that moves the counter into the thread, acquires a lock on the Mutex<T> by calling the lock method, and then adds 1 to the value in the mutex. 
When a thread finishes running its closure, num will go out of scope and release the lock so another thread can acquire it.
doesn't compiles because: we can’t move the ownership of counter into multiple threads

### Multiple Ownership with Multiple Threads
wrap the `Mutex<T>` in `Rc<T>` and clone the `Rc<T>` before moving ownership to the thread.
```rs
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```
compiler syas thst:
`Rc<Mutex<i32>>` cannot be sent between threads safely -> the trait `Send` is not implemented for `Rc<Mutex<i32>>`

### Atomic Reference Counting with `Arc<T>`
`Arc<T>` is a type like `Rc<T>`
- safe to use in concurrent situations. 
- `a` stands for atomic, meaning it’s an atomically reference counted type

atomics work like primitive types but are safe to share across threads.
then why not use `Arc<T>` by-default: thread safety comes with a performance penalty.
fix: `mut-thread.rs`
```rs
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

## 16.4 Extensible Concurrency with the `Sync` and `Send` Traits
`Send` is use to transwfer the ownership of values of the type implementing Send can be transferred between threads.
exception:
Rc<T>: this cannot be Send because if you cloned an Rc<T> value and tried to transfer ownership of the clone to another thread, both threads might update the reference count at the same time.

### Allowing Access from Multiple Threads with `Sync`

- The `Sync` marker trait means a type can be safely referenced from multiple threads.
- A type `T` is `Sync` if `&T` (an immutable reference to `T`) is `Send`.
- If `&T` can be safely sent to another thread, then `T` is `Sync`.
- Primitive types are `Sync`.
- Types composed entirely of `Sync` types are also `Sync`.

- `Rc<T>` is **not** `Sync` (same reason it's not `Send`).
- `RefCell<T>` and related `Cell<T>` types are **not** `Sync`:
  - Their runtime borrow checking is not thread-safe.
- `Mutex<T>` **is** `Sync`:
  - Allows shared access across threads.


### Implementing `Send` and `Sync` Manually Is Unsafe

- Types composed of `Send` and `Sync` types are automatically also `Send` and `Sync`.
- These traits are marker traits (no methods to implement).
- Enforce concurrency-related safety.

- Manually implementing `Send` or `Sync` is unsafe:
  - Requires `unsafe` Rust.
  - Needs careful consideration to maintain safety guarantees.

# Chap 17
# Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams
asynchronous rust is as same as asynchronous js
no two threads will run simultaneously let alone process.
only one process runs at a time
consider:
If you had only one CPU core and your operating system didn’t pause that export until it completed—that is, if it executed the export synchronously—you couldn’t do anything else on your computer while that task was running. 
That would be a pretty frustrating experience right.

Fortunately, your computer’s operating system can, and does, invisibly interrupt the export often enough to let you get other work done simultaneously.

import a video from a friend, takes time
now if you open it it will take 1-2 seconds, which not much for human, but for
computer that's a eternity.

here, the operating system’s invisible interrupts provide a form of concurrency.
That concurrency happens only at the level of the entire program

We could avoid blocking our main thread by spawning a dedicated thread to download each file. However, the overhead of those threads would eventually become a problem. It would be preferable if the call didn’t block in the first place. It would also be better if we could write in the same direct style we use in blocking code, similar to this:

```rs
let data = fetch_data_from(url).await;
println!("{data}");
```

## Parallelism and Concurrency
When an individual works on several different tasks before any of them is complete, this is `concurrency`.
You’re just one person, so you can’t make progress on both tasks at the exact
same time, but you can multi-task, making progress on one at a time by switching
between them.


When the team splits up a group of tasks by having each member take one task and work on it alone, this is `parallelism`.
Each person on the team can make progress at the exact same time.

concurreny: multiple task one person
parallelism: multiple person one task

you might realize that one of your own tasks depends on another of your tasks. Now your concurrent work has also become serial.

Parallelism and concurrency can intersect with each other.

## 17.1 Futures and the Async Syntax
key elements of asynchronous programming in Rust are futures and Rust’s async and await keywords.
A future is a value that may not be ready now but will become ready at some point in the future.
ex: Promise in js

apply via `Future` trait.
futures are types that implement the Future trait. Each future holds its own information about the progress that has been made and what “ready” means.

apply the `async` keyword to blocks and functions to specify that they can be interrupted and resumed

`await` keyword to await a future (that is, wait for it to become ready).

process of checking with a future to see if its value is available yet is called `polling`.

just like js, in rust we use `async` and `await`.
Rust compiles them into equivalent code using the `Future` trait, much as it compiles `for` loops into equivalent code using the `Iterator` trait.

let's understand by writing a project: a web scrapper; pass 2 url, fetch
concurrently, return whichever 1st ends.

use `future` crate for async programming.
`Tokio` is the most widely used async runtime in Rust today.

We use the `tokio` crate under the hood for `trpl` because it’s `well tested and widely used`.

create a new project:
```sh
$ cargo new hello-async
$ cd hello-async
$ cargo add trpl
```

writing a function that takes one page URL as a parameter, makes a request to it, and returns the text of the title element

When Rust sees a block marked with the async keyword, it compiles it into a unique, anonymous data type that implements the Future trait.
When Rust sees a function marked with async, it compiles it into a non-async function whose body is an async block. An async function’s return type is the type of the anonymous data type the compiler creates for that async block.

Every Rust program that executes async code has at least one place where it sets up a runtime and executes the futures.


```rs
async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await; //call the url ,hvae await method on
    //it
    let response_text = response.text().await; //get the text from the response
    Html::parse(&response_text) //parse into HTML
        .select_first("title") //find
        //the first instance of a given CSS selector. here 'title'
        .map(|title_element| title_element.inner_html())
}

```
is similar to:
```rs
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> + '_ {
    //returned trait is future, Output type is Option<String>
    async move {    //async move is a function type
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```

we can't use `async` keyword for main function; reason: async code needs a runtime: 
a Rust crate that manages the details of executing asynchronous code. A program’s main function can initialize a runtime, but it’s not a runtime itself.

we use the run function from the trpl crate, which takes a future as an argument and runs it to completion. Behind the scenes, calling run sets up a runtime that’s used to run the future passed in. Once the future completes, run returns whatever value the future produced.

run like: `cargo run -- https://www.rust-lang.org`

Each await point—that is, every place where the code uses the await keyword—represents a place where control is handed back to the runtime. To make that work, Rust needs to keep track of the state involved in the async block so that the runtime can kick off some other work and then come back when it’s ready to try advancing the first one again.
```rs
enum PageTitleFuture<'a> {
    Initial { url: &'a str },
    GetAwaitPoint { url: &'a str },
    TextAwaitPoint { response: trpl::Response },
}
```
Writing the code to transition between each state by hand would be tedious and error-prone,
compiler handles this
the Rust compiler creates and manages the state machine data structures for async code automatically.

Ultimately, something has to execute this state machine, and that something is a runtime.

that's why we can't keep the main function `async`.
If `main` were an async function, something else would need to manage the state machine for whatever future `main` returned, but `main` is the `starting point` for the program! Instead, we called the `trpl::run` function in main to set up a `runtime` and run the future returned by the `async` block until it returns `Ready`.

If `main` were an async function, something else would need to manage the state machine for whatever future `main` returned, but `main` is the `starting point` for the program! Instead, we called the `trpl::run` function in main to set up a `runtime` and run the future returned by the `async` block until it returns `Ready`.


begin by calling page_title for each of the user-supplied URLs. We save the resulting futures as title_fut_1 and title_fut_2
pass the futures to trpl::race, which returns a value to indicate which of the futures passed to it finishes first.
Either future can legitimately “win,” so it doesn’t make sense to return a Result. Instead, race returns a type we haven’t seen before, trpl::Either. The Either type is somewhat similar to a Result in that it has two cases. Unlike Result, though, there is no notion of success or failure baked into Either. Instead, it uses Left and Right to indicate “one or the other”:
```rs
enum Either<A, B> {
    Left(A),
    Right(B),
}

```

## 17.2 | Applying Concurrency with Async
In this section we’ll focus on what’s different between threads and futures.
In many cases, the APIs for working with concurrency using async are very similar to those for using threads. 
In other cases, they end up being quite different.

### Creating a New Task with `spawn_task`
well trpl crate provides a `spawn_task` function that looks very similar to the `thread::spawn` API.
a `sleep` function that is an `async version` of the `thread::sleep` API.
ex:
```rs
use std::time::Duration;

fn main() {
    trpl::run(async {   //set up `main` function with `trpl::run` so that our top-level function can be `async`.
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;  //waits for half a second (500 milliseconds) before sending the next message.
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
```

version stops as soon as the for loop in the body of the main async block finishes, because the task spawned by spawn_task is shut down when the main function ends.

to run till the actual process ends: use a `join handle` to wait for the first task to complete.\
With threads, we used the join method to “block” until the thread was done running.
we can use await to do the same thing, because the task handle itself is a future. 
Its Output type is a Result, so we also unwrap it after awaiting it.
```rs
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
```
runs until both loops finish.

looks like both `async` & `threads` are same.
using `await` instead of calling `join` on the join handle, and awaiting the `sleep` calls.

`bigger difference is that we didn’t need to spawn another operating system
thread to do this`
async blocks compile to anonymous futures, we can put each loop in an async block and have the runtime run them both to completion using the `trpl::join` function.


 use the join method on the JoinHandle type
returned when you call std::thread::spawn. The trpl::join function is similar,
but for futures.
 
so when we give it two futures, it produces another new futures whose o/p is
tuple of each future you passed in{o/p is a tuple ; the tuple has both futures
contained in it a/f they complete}.

we use `trpl::join` to wait for both `fut1` and `fut2` to finish. We `don't
await fut1 and fut2` but instead the `new future` produced by `trpl::join`.
ex:
```rs
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
```

trpl::join function is fair, meaning it checks each future equally often, alternating between them, and never lets one race ahead if the other is ready.
os decides which thread to check and how long to let it run.runtime decides which task to check.

### Counting Up on Two Tasks Using Message Passing
Sharing data between futures will be via use of message passing again, but this
time with async versions of the types and functions.

begin with just a single async block—not spawning a separate task as we spawned a separate thread.
```rs
        let (tx, mut rx) = trpl::channel();     //trpl::channel, an async version of the multiple-producer, single-consumer channel API;
        //use a mutable receiver rx

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();    //recv method produces a future which needs to be await
        println!("Got: {received}");
```
synchronous `Receiver::recv` method in `std::mpsc::channel` blocks until it receives a message.
`trpl::Receiver::recv` method does not, because it is async.

```rs
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {   //rx.recv produces a future
            //which is awaited
            println!("received '{value}'");
        }
```
- message arrive right away
- no concurrency
- wait until we determine that no more message arrive

problems:
- msg don't arrive in a queue, they arrive all at once at 2 sec time.
- program never exit, it just waits and waits, need to quit manually

To get the sleep delay happens between each message, put the tx and rx operations in their own async blocks

```rs
        let tx_fut = async {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
```
the message prints after interval of 500 milliseconds rather than all at once
a/f 2 sec.

The program still never exits, though, because of the way while let loop interacts with trpl::join:

- The future returned from trpl::join completes only once both futures passed to it have completed.
- The tx future completes once it finishes sleeping after sending the last message in vals.
- The rx future won’t complete until the while let loop ends.
- The while let loop won’t end until awaiting rx.recv produces None.
- Awaiting rx.recv will return None only once the other end of the channel is closed.
- The channel will close only if we call rx.close or when the sender side, tx, is dropped.
- We don’t call rx.close anywhere, and tx won’t be dropped until the outermost async block passed to trpl::run ends.
- The block can’t end because it is blocked on trpl::join completing, which takes us back to the top of this list.

`move` keyword works with async blocks just as it does with closures.
ex:
```rs
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
```

This async channel is also a multiple-producer channel, so we can call clone on tx if we want to send messages from multiple futures
```rs
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
```
move `async` file to `bin` folder and run that directly by:
`cargo run --bin async`

## 17.3 | Working with Any Number of Futures
` trpl::join!(tx1_fut, tx_fut, rx_fut);`

 Using join! to wait for multiple futures
this macro form only works when we know the number of futures ahead of time

 In real-world Rust, though, pushing futures into a collection and then waiting on some or all the futures of them to complete is a common pattern.

to check for futures iterate and join all of them.
`trpl::join_all` function accepts any type that implements the `Iterator trait`,
```rs
        let futures = vec![tx1_fut, rx_fut, tx_fut];

        trpl::join_all(futures).await;
```
After all, none of the async blocks returns anything, so each one produces a `Future<Output = ()>`.
even if you declare it `async` creates prpblem cause the `rust-lang` assignes
each `asnyc` in new block
Remember that `Future` is a trait, though, and that the compiler creates a `unique enum for each async block`.

solution: use `trait objects`. lets us treat each of the anonymous futures produced by these types as the same type, because all of them implement the Future trait.

 start by wrapping each future in the `vec!` in a `Box::new`
```rs
        let futures =
            vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];

        trpl::join_all(futures).await;
```

```rs
        let futures: Vec<Box<dyn Future<Output = ()>>> =        //output of the future is the unit type ();
        //trait annotated dynamic by `dyn`; wrap whole trait in `box`
        //state explicitly that futures is a Vec containing these items.

            vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];
```

gives vaious error upon compilation:
- first async block (src/main.rs:8:23: 20:10) does not implement the Unpin trait and suggests using pin! or Box::pin to resolve it.
- use Box::pin to pin the futures themselves.

```rs
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
```
compiles man!!!

- `Pin<Box<T>>` adds a small amount of overhead from putting these futures on the heap with Box—and we’re only doing that to get the types to line up.
- don't need heap allocation cause Futures are local to only this scope
- Pin is itself a wrapper type, so we can get the benefit of having a single type in the Vec—the original reason we reached for Box—without doing a heap allocation. 
- We can use Pin directly with each future, using the std::pin::pin macro.
- we must still be explicit about the type of the pinned reference; otherwise, Rust will still not know to interpret these as dynamic trait objects, which is what we need them to be in the Vec.
- therefore `pin!` each `future` when we define it, and define futures as a Vec containing pinned mutable references to the dynamic future type
```rs
        let tx1_fut = pin!(async move {
            // --snip--
        });

        let rx_fut = pin!(async {
            // --snip--
        });

        let tx_fut = pin!(async move {
            // --snip--
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx1_fut, rx_fut, tx_fut];

                let a = async { 1u32 }; //anonymous future for a implements Future<Output = u32>
        let b = async { "Hello!" }; //for b implements Future<Output = &str>
        let c = async { true };     //for c implements Future<Output = bool>.

        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");

```
use `trpl::join!` to await them, because it allows us to pass in multiple future types and produces a tuple of those types
cannot use trpl::join_all, because it requires all of the futures passed in to
have the same type.

fundamental tradeoff: 
- we can either deal with a dynamic number of futures with join_all, as long as they all have the same type, or 
- we can deal with a set number of futures with the join functions or the join! macro, even if they have different types.

### Racing Futures
When we “join” futures with the join family of functions and macros, we require all of them to finish before we move on. use 2 futures: fast, slow
```rs
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        trpl::race(slow, fast).await;

```

print msg at each function
pass `slow` and `fast` to `trpl::sleep` and check who exceutes/finishes fast.
Rust only pauses async blocks and hands control back to a runtime at an await point. Everything between await points is synchronous.

### Yielding Control to the Runtime
example of long-running function:
```rs
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));   //uses thread::sleep so blocks
    //the current thread for some time period
    println!("'{name}' ran for {ms}ms");
}
```
uses std::thread::sleep instead of trpl::sleep so that calling slow will block the current thread for some number of milliseconds.
```rs
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
```
o/p:
```bin
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.
```
race still finishes as soon as a is done
- `a` future does all of its work until the `trpl::sleep` call is awaited
- `b` future does all of its work until its own `trpl::sleep` call is awaited
-  `a` future complete

if we removed the trpl::sleep at the end of the a future, it would complete without the b future running at all.
```rs
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 35);
            trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };
```
o/p:
```bin
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.
```
`a` runs for a bit b/f `b` cause if calles `slow` b/f `trpl::sleep`. in this
shit a `await` point get hit which just makes it wait for indefinitely long.

we really don't want to sleep; just hand back and forth the control using
`yield_now` function.
replace all those `sleep` calls with `yield_now`.
```rs
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 35);
            trpl::yield_now().await;
            println!("'b' finished.");
        };
```

will always sleep for at least a millisecond, even if we pass it a Duration of one nanosecond. 
ex:
```rs
fn main(){
        let one_ns = Duration::from_nanos(1);
        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::sleep(one_ns).await;  //pass a one-nanosecond Duration to trpl::sleep
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'sleep' version finished after {} seconds.",
            time.as_secs_f32()
        );

        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;    //yield_now is way faster
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'yield' version finished after {} seconds.",
            time.as_secs_f32()
        );

}
```
This means that async can be useful even for compute-bound tasks, depending on what else your program is doing, because it provides a useful tool for structuring the relationships between different parts of the program. 
This is a form of cooperative multitasking, where each future has the power to determine when it hands over control via await points. Each future therefore also has the responsibility to avoid blocking for too long. 
In some Rust-based embedded operating systems, this is the only kind of multitasking!

### Building Our Own Async Abstractions
We can also compose futures together to create new patterns.ex:
timeout to work with a slow future.
```rs
        let slow = async {
            trpl::sleep(Duration::from_millis(100)).await;
            "I finished!"
        };

        match timeout(slow, Duration::from_millis(10)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
```

We can also compose futures together to create new patterns.

build a api for timeout:
- It needs to be an async function itself so we can await it.
- Its first parameter should be a future to run. We can make it generic to allow it to work with any future.
- Its second parameter will be the maximum time to wait. If we use a Duration, that will make it easy to pass along to trpl::sleep.
- It should return a Result. If the future completes successfully, the Result will be Ok with the value produced by the future. If the timeout elapses first, the Result will be Err with the duration that the timeout waited for.
```rs
async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    // Here is where our implementation will go!
}
```
race the future passed in against the duration.
- use trpl::sleep to make a timer future from the duration, and 
- use trpl::race to run that timer with the future the caller passes in.
-  pass future_to_try to race first so it gets a chance to complete even if max_time is a very short duration
- If future_to_try finishes first, race will return Left with the output from future_to_try. 
- If timer finishes first, race will return Right with the timer’s output of ().
ex:
```rs
use trpl::Either;

// --snip--

fn main() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
```

o/p:
```bin
Failed after 2 seconds
```

## 17.4 | Streams: Futures in Sequences
 The async recv method produces a sequence of items over time. This is an instance of a much more general pattern known as a stream.

there are two differences between iterators and the async channel receiver:
1. time: iterators are synchronous, while the channel receiver is asynchronous. 
2. API: iterator has synchronous `next` method, but the `trpl::Stream` have
   Asynchronous method `recv`; otherwise they are similar
similarity suggest that we can create a stream from any iterator.

```rs
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);    //convert to iterator; double all values
        let mut stream = trpl::stream_from_iter(iter);  //convert the iterator into a stream

        while let Some(value) = stream.next().await {   //loop over the items in the stream as they arrive
            println!("The value was: {value}");
        }

```
reason for the compiler error is that we need the right trait in scope to be able to use the next method. 
use trait `StreamExt`. {Ext is a common pattern in the Rust community for extending one trait with another.}
- Stream trait defines a low-level interface that effectively combines the Iterator and Future traits.
- StreamExt supplies a higher-level set of APIs on top of Stream, including the next method

fix to the compiler error is to add a use statement for trpl::StreamExt.
```rs
use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}
```

### Composing Streams
some example of stream are:
- items becoming available in a queue, 
- chunks of data being pulled incrementally from the filesystem when the full data set is too large for the computer’s , or 
- data arriving over the network over time.

they can be used with any type of fututres and combined with them.

building a little stream of messages as a stand-in for a stream of data we might see from a WebSocket or another real-time communication protocol
`socket.rs`
create a function called get_messages that returns impl Stream<Item = String>, create an async channel, loop over the first 10 letters of the English alphabet, and send them across the channel.
used a new type: ReceiverStream, which converts the rx receiver from the trpl::channel into a Stream with a next method.

let’s add a feature that requires streams: adding a timeout that applies to every item in the stream, and a delay on the items we emit,
add a timeout to stream method via timeout method, which comes from the StreamExt trait.
update while let cause now it returns some value of type `Result<Ok,Err>`
`Ok`-> msg arrived in time.
`Err`-> timeout elapsed before any message arrived.

pin the messages after applying the timeout to them, because the timeout helper produces a stream that needs to be pinned to be polled.
wrap the function content under `async move`.

- In get_messages, we use the enumerate iterator method with the messages array so that we can get the index of each item we’re sending along with the item itself. 
- apply a 100-millisecond delay to even-index items and a 300-millisecond delay to odd-index items to simulate the different delays we might see from a stream of messages in the real world. Because our timeout is for 200 milliseconds, this should affect half of the messages.

use async in order to delay b/w messgae w/o blocking. but can't make them into
aysnc function cause return will be changed to `Future<Output = Stream<Item = String>>` instead of a `Stream<Item = String>>`.

Awaiting get_messages would require it to send all the messages, including the sleep delay between each message, before returning the receiver stream.

Instead, we leave get_messages as a regular function that returns a stream, and we spawn a task to handle the async sleep calls.

timeout doesn’t prevent the messages from arriving in the end b/c channel is
`unbounded`

### Merging Streams
create another stream which o/p every milliseconds
use the sleep function to send a message on a delay and combine it with the same approach we used in get_messages of creating a stream from a channel. 

this time, we’re going to send back the count of intervals that have elapsed, so the return type will be `impl Stream<Item = u32>`
```rs
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;  //define a count
        loop {
            trpl::sleep(Duration::from_millis(1)).await;    //asynchronously sleeps for one millisecond
            count += 1; //inifintely increase the count
            tx.send(count).unwrap();    //send over channel
        }
    });

    ReceiverStream::new(rx)
}
```
now we can merge the function:
```rs
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals();    //call the method
        let merged = messages.merge(intervals); //merge both messages and
        //intervals
```
neither messages nor intervals needs to be pinned or mutable, because both will be combined into the single merged stream.
but doesn't compiles cause both stream have diff types
`messages` -> `Timeout<impl Stream<Item = String>>`; `Timeout` -> Stream for a timeout call.; `intervals` -> `impl Stream<Item = u32>`.
```rs
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))  //to transform the intervals into a string.match the Timeout from messages.
            .timeout(Duration::from_secs(10));  //create a 10-second timeout 
        let merged = messages.merge(intervals);
        let mut stream = pin!(merged);  //so that the while let loop’s next calls can iterate

```

two problems-> 
1. it will never stop! You’ll need to stop it with ctrl-c.
2. the messages from the English alphabet will be buried in the midst of all the interval counter.

solution->
1. use the throttle method on the intervals stream so that it doesn’t overwhelm the messages stream
    Throttling is a way of limiting the rate at which a function will be called—or, in this case, how often the stream will be polled.
    do in like 100 ms.

    throttle call produces a new stream that wraps the original stream so that the original stream gets polled only at the throttle rate, not its own “native” rate.

to handle errors:
- send calls could fail when the other side of the channel closes.{ignored this possibility by calling unwrap}
- explicitly handle the error, at minimum by ending the loop so we don’t try to
send any more messages.

```rs
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
```

To limit the number of items we will accept from a stream, we apply the take method to the merged stream, because we want to limit the final output, not just one stream or the other.

## 17.5 | A Closer Look at the Traits for Async
we read about `Futures`, `pin` and various traits, but don't know how to use,
where to use
let's dive into these scenarios

### The `Future` Trait
```rs
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;    //says what the future resolves to

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

the poll method returns:
```rs
enum Poll<T> {  //type is similar to an Option
    Ready(T),   //future has finished its work and the T value is available.
    Pending,    //indicates that the future still has work to do, so the caller will need to check again later.
}
```
`await` functions are also called under the `Poll enum` under the hood, that's like `await` points to `Poll` enum.

```rs
match page_title(url).poll() {
    Ready(page_title) => match page_title {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    }
    Pending => {
        // But what goes here?
        //need some way to try again, and again, and again, until the future is finally ready.
        //need a loop here
    }
}
```
Rust makes sure that the loop can hand off control to something that can pause work on this future to work on other futures and then check this one again later.

recv waits for future.
The recv call returns a future, and awaiting the future polls it.
it gives you a future — a special object that represents a value that will be available later.
consider it similar to a promise in js
recv are often wrapped in futures to facilitate non-blocking I/O operations. When you .await a future in asynchronous Rust, it polls the future to check if the operation is complete or needs to be scheduled for later, which is relevant to how recv is used in such environments.

runtime puses the future, until we get something in return{`Some(message)`} or maybe `None` when channel closes.

when return type is `Poll::Pending`-> future is pending
return type of `Poll` is `Poll::Ready(Some(message))` or `Poll::Ready(None)`->
future is ready.

`a runtime polls each future it is responsible for, putting the future back to sleep when it is not yet ready.`

- `recv()` → returns a future (a "not yet" value).

- `await recv()` → tells the runtime to keep checking (polling) until that value is ready.


### The `Pin` and `Unpin` Traits
pinning is required for `boxes` if they wanna implement and `futures`.
Directly awaiting a future with await pins the future implicitly. That’s why we don’t need to use pin! everywhere we want to await futures.

join_all requires that the types of the items in the collection all implement the Future trait, and 
Box<T> implements Future only if the T it wraps is a future that implements the Unpin trait.

```rs
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```
The cx parameter and its Context type are the key to how a runtime actually knows when to check any given future while still being lazy.
A type annotation for `self` is works like type annotations for other function parameters, but with two key differences:
- It tells Rust what type self must be for the method to be called.
- It can’t be just any type. It’s restricted to the type on which the method is implemented, a reference or smart pointer to that type, or a Pin wrapping a reference to that type.

`Pin` is a wrapper for pointer-like types such as `&`, `&mut`, `Box`, and `Rc`.
- Pin is not a pointer itself and doesn’t have any behavior of its own like Rc and Arc do with reference counting; 
- it’s purely a tool the compiler can use to enforce constraints on pointer usage.

Rust looks at what data is needed between one await point and either the next await point or the end of the async block
rust works for blocks like scope, so yeah remember the heap collects over
blocks, like that.

When we move a future—whether by pushing it into a data structure to use as an iterator with join_all or by returning it from a function—that actually means moving the state machine Rust creates for us.

any object that has a reference to itself is unsafe to move, because references always point to the actual memory address of whatever they refer to.

in safe code, compiler prevents you from moving any item with an active reference to it. use `pin` for this guarantee.
When we pin a value by wrapping a pointer to that value in Pin, it can no longer move. 
Thus, if you have `Pin<Box<SomeType>>`, you actually pin the `SomeType` value, not the Box pointer.

the Box pointer can still move around freely. If a pointer moves around, but the data it points to is in the same place, there’s no problem.

However, most types are perfectly safe to move around, even if they happen to be behind a Pin pointer. 
We only need to `think` about pinning `when items have internal references`.

but consider this: `Pin<Vec<String>>`, you’d have to do everything via the safe but restrictive APIs provided by Pin, even though a `Vec<String>` is always safe to move if there are no other references to it. 
We need a way to tell the compiler that it’s fine to move items around in cases like this—and there’s where `Unpin` comes into play.

`Unpin` is a `marker trait`, similar to the Send and Sync traits, and has no functionality of its own.
- informs the compiler that a given type does not need to uphold any guarantees about whether the value in question can be safely moved.
- tell the compiler it’s safe to use the type implementing a given trait in a particular context.

hence `Unpin` is implemented automatically just call it like: `impl !Unpin for SomeType` {SomeType is the type to be implemented on}.
1. `Unpin` is the “normal” case, and `!Unpin` is the special case
2. whether a type implements `Unpin` or `!Unpin` only matters when you’re using a `pinned pointer to that type like Pin<&mut SomeType>`.

consider a String, we can wrap up a String in pin, but unpin needs not be created, automatically called.
- we can do things that would be illegal if String implemented !Unpin instead,  such as replacing one string with another at the exact same location in memory.
- doesn’t violate the Pin contract, because String has no internal references that make it unsafe to move around! That is precisely why it implements Unpin rather than !Unpin.


### The `Stream` Trait
streams are similar to asynchronous iterators. Stream has no definition in the standard library as of this writing, but 
there is a very common definition from the futures crate used throughout the ecosystem.

From Iterator, we have the idea of a sequence: its next method provides an `Option<Self::Item>`. 
From Future, we have the idea of readiness over time: its poll method provides a `Poll<Self::Output>`.
```rs
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}
```
Stream trait defines an associated type called Item for the type of the items produced by the stream
`poll_next` method to call the values, produces a sequence of items in the same way Iterator::next does. 
Its return type combines `Poll` with `Option`. 
- The outer type is Poll, because it has to be checked for readiness, just as a future does. 
- The inner type is Option, because it needs to signal whether there are more messages, just as an iterator does.

can also use `next` and `StreamExt`; StreamExt trait supplies the next method so we can do just that:
```rs
trait StreamExt: Stream {
    async fn next(&mut self) -> Option<Self::Item>
    where
        Self: Unpin;

    // other methods...
}
```


## 17.6 | Putting It All Together: Futures, Tasks, and Threads

Threads and async are two different but complementary models of concurrency. Threads have been around longer and are supported by most operating systems, but they come with overhead and aren’t always suitable, especially for systems without an OS. Async tasks, on the other hand, are managed by the runtime rather than the OS and allow for more lightweight concurrency.

You can replace async code with threads in many cases, as shown in Listing 17-41, without changing the behavior of the program:

```rust
Filename: src/main.rs
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    // This is *not* `trpl::spawn` but `std::thread::spawn`!
    thread::spawn(move || {
        let mut count = 0;
        loop {
            // Likewise, this is *not* `trpl::sleep` but `std::thread::sleep`!
            thread::sleep(Duration::from_millis(1));
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
```

Async tasks are generally more memory-efficient and easier to compose than threads. Futures, which power async, can be paused and resumed, unlike threads which are "fire and forget." However, threads are sometimes simpler for parallel, compute-heavy workloads.

You don’t have to choose one over the other—using both together can be powerful. Async runtimes often leverage multiple threads under the hood. For example, you can spawn a thread to do blocking work and use async to receive results:

```rust
Filename: src/main.rs
use std::{thread, time::Duration};

fn main() {
    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::run(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}
```

**Guidelines for choosing:**
- Use **threads** for parallel, compute-bound tasks.
- Use **async** for IO-bound, highly concurrent tasks.
- Combine both when necessary, leveraging each where it excels.

# Chap 18
# Object-Oriented Programming Features of Rust
Rust is a language that supports both OOPs and POPs.

## 18.1 | Characteristics of Object-Oriented Languages
## Objects Contain Data and Behavior
Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data. The procedures are typically called methods or operations.

Rust is object-oriented: 
- structs and enums have data, and 
- impl blocks provide methods on structs and enums. 

Even though structs and enums with methods aren’t called objects, they provide the same functionality, according to the Gang of Four’s definition of objects.

## Encapsulation that Hides Implementation Details
Encapsulation, which means that the implementation details of an object aren’t accessible to code using that object.
Hence interaction is possible only via `publicAPI`.
code using the object shouldn’t be able to reach into the object’s internals and
change data or behavior directly.
consider this:
```rs
pub struct AveragedCollection { //`pub` keyword suggest that the struct can be
    //used by anyone
    list: Vec<i32>, //but the data inside the struct is private
    average: f64,
//private so there is no way for external code to add or remove items to or from the list field directly
}
```

implement add, remove, and average methods on the struct
```rs
impl AveragedCollection {
    pub fn add(&mut self, value: i32) { //item is added to list
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {   //item is removed
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```
As long as the signatures of the add, remove, and average public methods stay the same, code using AveragedCollection wouldn’t need to change in order to compile
`HashSet<i32>` and `Vec<i32>` have different methods for adding and removing items, so the external code would likely have to change if it were modifying list directly.

### Inheritance as a Type System and as Code Sharing
Inheritance is a mechanism whereby an object can inherit elements from another object’s definition, thus gaining the parent object’s data and behavior without you having to define them again.

There is no way to define a struct that inherits the parent struct’s fields and method implementations without using a macro.

but some tools are there:
- implement particular behavior for one type, and inheritance enables to reuse that implementation for a different type.
- Any type implementing the Summary trait would have the summarize method available on it without any further code.

We can also override the default implementation of the summarize method when we implement the Summary trait, which is similar to a child class overriding the implementation of a method inherited from a parent class.

inheritance relates to the type system: to enable a child type to be used in the same places as the parent type. This is also called `polymorphism`, which means that you can substitute multiple objects for each other at runtime if they share certain characteristics.

Let’s look at how trait objects enable polymorphism in Rust.

## 18.2 | Using Trait Objects That Allow for Values of Different Types
we can store different types of data in each cell and still have a vector that represented a row of cells. This is a perfectly good solution when our interchangeable items are a fixed set of types that we know when our code is compiled.
sometimes we want our library user to be able to extend the set of types that
are valid in a particular situation.
consider an example where we create an gui to draw stuff.
now many things would have some common behavious like square and rectangle
having 4 sides, stc.

### Defining a Trait for Common Behavior
define a trait named `Draw` that will have one method named `draw`. define a vector that takes a trait object. 

`trait object` points to both an instance of a type implementing our specified trait and a table used to look up trait methods on that type at runtime.
create a trait object by specifying some sort of pointer, such as a & reference or a `Box<T>` smart pointer, then the `dyn` keyword, and then specifying the relevant trait.

**Wherever we use a trait object, Rust’s type system will ensure at compile time
that any value used in that context will implement the trait object’s trait.**

In a struct or enum, the data in the struct fields and the behavior in impl
blocks are separated.

trait objects are more like objects in other languages in the sense that they combine data and behavior. But trait objects differ from traditional objects in that we can’t add data to a trait object. 

Trait objects aren’t as generally useful as objects in other languages: their
specific purpose is to allow abstraction across common behavior.ex:
```rs
pub trait Draw {
    fn draw(&self);
}
```

struct named Screen that holds a vector named components.
This vector is of type Box<dyn Draw>, which is a trait object; it’s a stand-in for any type inside a Box that implements the Draw trait.
```rs
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```
On the Screen struct, we’ll define a method named run that will call the draw method on each of its components:
```rs
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```
works differently from defining a struct that uses a generic type parameter with trait bounds.

`generic type`-> substituted with one concrete type at a time
`trait obj`-> multiple concrete types to fill in for the trait object at runtime.
ex:
```rs
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

**`homogeneous collections`**-> generics and trait bounds is preferable because the definitions will be monomorphized at compile time to use the concrete types.
**`method using trait objects`**-> instance can hold a Vec<T> that contains a Box<Button> as well as a Box<TextField>

### Implementation
add some types that implement the Draw trait. 
note: GUI is out of scope for book
ex:
```rs
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

someone using our library decides to implement a SelectBox struct that has width, height, and options fields, they implement the Draw trait on the SelectBox type as well,
```rs
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
```
user can now write their main function to create a Screen instance.
- can add a SelectBox and a Button by putting each in a Box<T> to become a trait object.
- can call the run method on the Screen instance, which will call draw on each of the components

When we wrote the library, we didn’t know that someone might add the SelectBox type, but our Screen implementation was able to operate on the new type and draw it because SelectBox implements the Draw trait, which means it implements the draw method.

1. `Trait Objects Enable Polymorphism`: Rust allows treating different types uniformly using trait objects (e.g., Box<dyn Draw>), similar to duck typing—code only cares that values implement required methods like draw, not their specific types.

2. `Compile-Time Safety Over Runtime Errors`: Unlike duck typing in dynamic languages, Rust enforces trait implementation at compile time, preventing runtime errors due to missing methods and ensuring type safety.

### Trait Objects Perform Dynamic Dispatch
1. **Static vs Dynamic Dispatch:** Rust uses *static dispatch* with generics through monomorphization, generating concrete code at compile time, which improves performance and enables inlining.

2. **Trait Objects Use Dynamic Dispatch:** When using trait objects, Rust employs *dynamic dispatch*, resolving method calls at runtime using vtables, which introduces a slight performance cost and limits some optimizations.

3. **Flexibility vs Performance Trade-off:** While dynamic dispatch via trait objects offers greater flexibility (e.g., heterogeneous collections), it sacrifices some compile-time performance benefits and is constrained by Rust’s `dyn` compatibility rules.


## 18.3 | Implementing an Object-Oriented Design Pattern
The state pattern is an object-oriented design pattern. The crux of the pattern is that we define a set of states a value can have internally. The states are represented by a set of state objects, and the value’s behavior changes based on its state.

ex: Blog -> "draft”, “review”, or “published”.

advantage of using the state pattern:
- business requirements of the program change, we won’t need to change the code of the value holding the state or the code that uses the value
- update the code inside one of the state objects to change its rules or perhaps add more state objects.

final functionality will look like this:
- A blog post starts as an empty draft.
- When the draft is done, a review of the post is requested.
- When the post is approved, it gets published.
- Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

 other changes attempted on a post should have no effect.
allow the user to create a new draft blog post with Post::new
- allow text to be added to the blog post
- accessing post draft b/f approval should give nothing
- enable a request for a review of the post, and we want content to return an empty string while waiting for the review
- a/f approval gets published, hence returns the post

crate used is `Post` type. Changing from one state to another will be managed internally within the Post type

1. Defining the post in `blog.rs` file-> to hold some content

whole done in `blog.rs` file with documentation 


# Chap 19
# Patterns and Matching
Patterns are a special syntax in Rust for matching against the structure of types, both complex and simple.
use with `match` construction to have more control over them in the program
Pattern is combination of:
- Literals
- Destructured arrays, enums, structs, or tuples
- Variables
- Wildcards
- Placeholders

ex:x, (a, 3), and Some(Color::Red)

Working-> 
- components describe the shape of data.
- program then matches values against the patterns to determine whether it has the correct shape of data to continue running a particular piece of code.

## 19.1 | All the Places Patterns Can Be Used
Pattern are used in various places in Rust, we have using them w/o knowing a damn thing about them.

### `match` Arms
use patterns in the arms of match expressions.
match - a value to match on, and one or more match arms that consist of a pattern and an expression to run if the value matches that arm’s pattern
```rs
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

ex:
```rs
match x {
    None => None,   //None and Some are pattern here
    Some(i) => Some(i + 1),
}
```
Note: **need to be exhaustive in the sense that all possibilities for the value
in the match expression must be accounted for**

one way to ensure: **catch all** arm

The particular pattern `_` will match anything, but it never binds to a variable, so it’s often used in the last match arm.
The `_` pattern can be useful when you want to ignore any value not specified

### Conditional `if let` Expressions
a shorter way to write the equivalent of a match that only matches one case.
optionally can have `else` code if any of condition just doesn't matches, that's
like exception handling in case anything doesn't matches.

gives us more flexibility than a match expression; ex:
```rs
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        //can also introduce new variables which shadow existing variables in the same way that match arms can
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

downside of using if let expressions is that the compiler doesn’t check for
exhaustiveness, whereas with match expressions it does

say last else block was missed then the compiler wouldn't alert us to the possible logic bug.

### `while let` Conditional Loops
- allows a while loop to run for as long as a pattern continues to match
previously we used it to keep looping as long as a stream produced new values:
```rs
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }
```

### `for` loops
when we write for loop, after the keyword `for` anything we write is a
`pattern`.
ex: `for x in y` -> `x` is pattern.

ex:
```rs
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {    //adapt an iterator using the enumerate method so it produces a value and the index for that value, placed into a tuple.
        println!("{value} is at index {index}");
    }
```

o/p:
```sh
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s
     Running `target/debug/patterns`
a is at index 0
b is at index 1
c is at index 2
```

### `let` Statements
consider this:
`let x = 5;` this is also a `pattern`, more formally the definiton looks like:
`let PATTERN = EXPRESSION;`

x is a pattern that means “bind what matches here to the variable x.”
since x is whole pattern-> “bind everything to the variable x, whatever the value is.” 

consider for tuples:
```rs
    let (x, y, z) = (1, 2, 3);  //compares the value (1, 2, 3) to the pattern (x, y, z); binds 1 to x, 2 to y, and 3 to z
        let (x, y) = (1, 2, 3); //error
```

### Function Parameters
Function parameters can also be patterns.
consider:
```rs
fn foo(x: i32) {    //x part is a pattern!
    // match a tuple in a function’s arguments to the pattern.
}
```

ex:
```rs
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

fn main() {
    let point = (3, 5); //values &(3, 5) match the pattern &(x, y)
    //x=3,y=5
    print_coordinates(&point);
}
```

o/p:
```sh
Current location: (3, 5)
```

## 19.2 | Refutability: Whether a Pattern Might Fail to Match
Pattern has 2 forms: `Refutable` & `irRefutable`
1. **Irrefutable patterns** always match any value (e.g., `let x = 5;`), so they can’t fail.
2. **Refutable patterns** may fail to match certain values (e.g., `Some(x)` may not match `None`).
3. **`let`, function parameters, and `for` loops** require irrefutable patterns because they can't handle non-matching cases.
4. **`if let`, `while let`, and `let-else`** accept both refutable and irrefutable patterns, but the compiler warns if you use an irrefutable one since it defeats the purpose of conditional logic.
5. Understanding **refutability** helps you fix compiler errors when patterns don't match the construct they're used in.

ex: use a refutable pattern where Rust requires an irrefutable pattern and vice vers
`    let Some(x) = some_option_value;`
- If some_option_value was a None value, it would fail to match the pattern Some(x), meaning the pattern is refutable
- However, the let statement can only accept an irrefutable pattern because there is nothing valid the code can do with a None value.
- complains at compilation Because we didn’t cover (and couldn’t cover!) every
valid value with the pattern Some(x)

use `if let` in case we have a refutable pattern where an irrefutable pattern is needed
```rs
    if let Some(x) = some_option_value {    //if the pattern doesn’t match, the code will just skip the code in the curly brackets
        println!("{x}");    //runs perfectly
    }
```

However, if we give if let an irrefutable pattern (a pattern that will always match), such as x, as shown below, the compiler will give a warning.
```rs
    if let x = 5 {
        println!("{x}");
    };
```

1. **`match` arms** use refutable patterns to handle multiple possible values, with the **last arm** typically using an irrefutable pattern to catch all remaining cases.
2. A **single-arm `match` with an irrefutable pattern** is allowed but unnecessary—it can be replaced with a simpler `let` statement.

## 19.3 | Pattern Syntax
gather all the syntax valid in patterns and discuss why and when you might want to use each one.

### Matching Literals
you can match patterns against literals directly. ex:
```rs
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
```

### Matching Named Variables
- Named variables are irrefutable patterns that match any value
- complication in `match`, `if let`, `while let`-> starts a new scope, variables declared as part of a pattern inside the expression will shadow those with the same name outside, as is the case with all variables.
```rs
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"), //pattern in the first match arm doesn’t match the defined value of x, 
        //the code continues.
        Some(y) => println!("Matched, y = {y}"),    //new scope declared, so
        //doesn't refers to previous y
        //new y binds to the inner value of the Some in x
        _ => println!("Default case, x = {x:?}"),
    }
//no x introduced in match scope so shadows over outer x
//now when match get's over, so it's y scope is also terminated
    println!("at the end: x = {x:?}, y = {y}");
```

o/p:
```bin
Matched, y = 5
at the end: x = Some(5), y = 10
```

### Multiple Patterns
match multiple patterns using the `|` syntax.
```rs
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),    //if the value of x matches either of the values in that arm, that arm’s code will run
        3 => println!("three"),
        _ => println!("anything"),
    }

```

### Matching Ranges of Values with `..=`
`..=` syntax allows us to match to an inclusive range of values.
```rs
    let x = 5;

    match x {
        1..=5 => println!("one through five"),  //If x is 1, 2, 3, 4, or 5, the first arm will match
        _ => println!("something else"),
    }
```
more practical than using the `|` operator
- compiler checks that the range isn’t empty at compile time,

```rs
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

```
`Rust can tell that 'c' is within the first pattern’s range and prints early ASCII letter.`

## Destructuring to Break Apart Values
use patterns to destructure structs, enums, and tuples to use different parts of these values. Let’s walk through each value.

### Destructuring structs

This excerpt demonstrates how to **destructure a struct** in Rust using pattern matching with `let` and `match` statements. It uses a `Point` struct with `x` and `y` fields to show:

1. **Basic destructuring** with custom variable names:

   ```rust
   let Point { x: a, y: b } = p;
   ```

   This assigns `p.x` to `a` and `p.y` to `b`.

2. **Shorthand destructuring** where variable names match field names:

   ```rust
   let Point { x, y } = p;
   ```

   This assigns `p.x` to `x` and `p.y` to `y` more concisely.

3. **Destructuring in `match` expressions**, including pattern matching with literal values:

   ```rust
   match p {
       Point { x, y: 0 } => println!("On the x axis at {x}"),
       Point { x: 0, y } => println!("On the y axis at {y}"),
       Point { x, y } => println!("On neither axis: ({x}, {y})"),
   }
   ```

   This allows matching specific struct field values while binding others, and shows how `match` stops at the first match.


### Destructuring Enums
 how to **destructure enum variants** in Rust using `match` patterns that reflect how each variant is defined:

1. **Unit-like variant** (`Message::Quit`) has no data, so it’s matched directly without any destructuring.

2. **Struct-like variant** (`Message::Move { x, y }`) uses curly braces and named fields, similar to struct destructuring.

3. **Tuple-like variants** (`Message::Write(String)` and `Message::ChangeColor(i32, i32, i32)`) use parentheses with variables matching the number of elements.

The example shows a `match` on a `Message` enum that destructures each variant appropriately to access and print its internal data. It highlights that the **pattern must match the shape and structure** of the enum variant being matched.
```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
    }
}
```

### Destructuring Nested Structs and Enums
```rs
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
}
```
- 1st arm matches a `Message::ChangeColor` enum variant that contains a `Color::Rgb` variant; then the pattern binds to the three inner `i32` values.
- 2nd arm matches a `Message::ChangeColor` enum variant, but the inner enum matches `Color::Hsv` instead.

### Destructuring Structs and Tuples
destructure where we nest structs and tuples inside a tuple and destructure all the primitive values out:
```rs
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```
break complex types into their component parts so we can use the values we’re interested in separately.
Destructuring with patterns is a convenient way to use pieces of values, such as the value from each field in a struct, separately from each other.

### Ignoring Values in a Pattern
it’s sometimes useful to ignore values in a pattern, such as in the last arm of a match, to get a catchall that doesn’t actually do anything but does account for all remaining possible values. 

using the _ pattern (which you’ve seen), using the _ pattern within another pattern, using a name that starts with an underscore, or using .. to ignore remaining parts of a value.

#### Ignoring an Entire Value with `_`
a wildcard pattern that will match any value but not bind to the value.
```rs
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

fn main() {
    foo(3, 4);
}
```

#### Ignoring Parts of a Value with a Nested `_`
use `_` inside another pattern to ignore just part of a value.
want to test for only part of a value but have no use for the other parts in the corresponding code we want to run.
```rs
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");
```

```bin
Can't overwrite an existing customized value
setting is Some(5)
```
In the first match arm, we don’t need to match on or use the values inside either `Some` variant, but we do need to test for the case when `setting_value` and `new_setting_value` are the Some variant.

use underscores in multiple places within one pattern to ignore particular
values.
```rs
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

```

```bin
Some numbers: 2, 8, 32
```

#### Ignoring an Unused Variable by Starting Its Name with `_`
you can tell Rust not to warn you about the unused variable by starting the name of the variable with an underscore. 

syntax `_x` still binds the value to the variable, whereas `_` doesn’t bind at all.
```rs
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {   //doesn't compiles
        println!("found a string");
    }
    //because the s value will still be moved into _s, which prevents us from using s again
    println!("{s:?}");

```

correct code:
```rs
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }
//because we never bind s to anything; it isn’t moved.
    println!("{s:?}");

```

#### Ignoring Remaining Parts of a Value with `..`
we can use the `..` syntax to use specific parts and ignore the rest,
The `..` pattern ignores any parts of a value that we haven’t explicitly matched in the rest of the pattern

consider, we want to operate only on the x coordinate and ignore the values in the y and z fields.
```rs
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {x}"),    //quicker than having to list y: _ and z: _
    }
    //syntax .. will expand to as many values as it needs to be.
```

ex:
```rs
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
```
but 
`.., second, ..` is wrong cause 
`           (.., second, ..) => {
  |          --          ^^ can only be used once per tuple pattern
  |          |
`
reason: impossible for Rust to determine how many values in the tuple to ignore before matching a value with second and then how many further values to ignore thereafter.

#### Extra Conditionals with Match Guards
A match guard is an additional `if` condition, specified after the pattern in a `match` arm, that must also match for that arm to be chosen.
- useful for expressing more complex ideas than a pattern alone allows.
only available in match expressions
```rs
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),    //x%2==0 is match guard; true if num is even
        //Some(4) matches Some(x);Then the match guard checks whether the remainder of dividing x by 2 is equal to 0
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
```

downside of this additional expressiveness is that the compiler doesn’t try to check for exhaustiveness when match guard expressions are involved.

have a deault case declared as:
```rs
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),  //only matches if the value of x is equal to 4, 5, or 6 and if y is true.
        _ => println!("no"),
    }

```

## `@` Bindings
 at operator `@` lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match.
```rs
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
            //we’re capturing whatever value matched the range while also testing that the value matched the range pattern.
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }

```
Using @ lets us test a value and save it in a variable within one pattern.

# Chap 20
# Advanced Features
- Unsafe Rust: how to opt out of some of Rust’s guarantees and take responsibility for manually upholding those guarantees
- Advanced traits: associated types, default type parameters, fully qualified syntax, supertraits, and the newtype pattern in relation to traits
- Advanced types: more about the newtype pattern, type aliases, the never type, and dynamically sized types
- Advanced functions and closures: function pointers and returning closures
- Macros: ways to define code that defines more code at compile time

# 20.1 | Unsafe Rust
Rust has bydefault memory safety induced, but inside it there's a second
language which is not memory safe
it’s called unsafe Rust and works just like regular Rust, but gives us extra superpowers.

it's better to reject valid programs than to accept invalid programs

## Unsafe Superpowers
use the `unsafe` keyword and then start a new block that holds the unsafe code,
5 abilities/superpower in unsafe rust:
- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of a union

unsafe doesn’t turn off the borrow checker or disable any other of Rust’s safety checks: 
    - if you use a reference in unsafe code, it will still be checked.

unsafe keyword only gives you access to these five features that are then not checked by the compiler for memory safety.

unsafe does not mean the code inside the block is necessarily dangerous or that it will definitely have memory safety problems: 
    - the intent is that as the programmer, you’ll ensure the code inside an unsafe block will access memory in a valid way.

keep unsafe blocks small to easily debug memeory issues

it’s best to enclose unsafe code within a safe abstraction and provide a safe API
Wrapping unsafe code in a safe abstraction prevents uses of unsafe from leaking out into all the places that you or your users might want to use the functionality implemented with unsafe code, because using a safe abstraction is safe.

## Dereferencing a Raw Pointer
compiler ensures that references are always valid. Unsafe Rust has two new types called raw pointers that are similar to references.
raw pointers can be immutable(`*const T`) or mutable (`*mut T`)
immutable means that the pointer can’t be directly assigned to after being dereferenced.

raw pointers:
- Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
- Aren’t guaranteed to point to valid memory
- Are allowed to be null
- Don’t implement any automatic cleanup

```rs
    let mut num = 5;    //mutabe variable

    let r1 = &raw const num;    //immutable pointer; *const i32
    let r2 = &raw mut num;  //mutable pointer; *mut i32
```
- We can create raw pointers in safe code; 
- we just can’t dereference raw pointers outside an unsafe block.

a raw pointer whose validity we can’t be so certain of, using as to cast a value instead of using the raw reference operators.
```rs
    let address = 0x012345usize;
    let r = address as *const i32;  //create a raw pointer to an arbitrary location in memory
```

- we can create raw pointers in safe code, but we can’t dereference raw pointers and read the data being pointed to.
```rs
    let mut num = 5;

    let r1 = &raw const num;    //it's const and mut refence, but were it being
    //a immut and mut, problem would have been there
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
//runs safely
```

### Calling an Unsafe Function or Method
2nd type of opr is calling `unsafe funtion`.they have an extra unsafe before the rest of the definition
```rs
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
```
must call the dangerous function within a separate unsafe block, else get error.
With the unsafe block, we’re asserting to Rust that we’ve read the function’s documentation, we understand how to use it properly, and we’ve verified that we’re fulfilling the contract of the function.

To perform unsafe operations in the body of an unsafe function, you still need
to use an unsafe block just as within a regular function.

### Creating a Safe Abstraction over Unsafe Code
just b/c function has unsafe code, doesn't means whole function is unsafe. In
fact, wrapping unsafe code in a safe function is a common abstraction. 
ex:
```rs
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);    //if we pass an index that is greater than the length to split the slice at, the function will panic before it attempts to use that index.

    (&mut values[..mid], &mut values[mid..])    //return two mutable slices in a tuple
}

```
For simplicity, we’ll implement split_at_mut as a function rather than a method and only for slices of i32 values rather than for a generic type T.
compilation error:
Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice; it only knows that we’re borrowing from the same slice twice.

useage of unsafe code, raw pointer to make it work:
```rs
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();  //as_mut_ptr method to access the raw pointer of a slice
    //returns a raw pointer with the type *mut i32

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),    //unsafe because it takes a raw pointer and must trust that this pointer is valid
            slice::from_raw_parts_mut(ptr.add(mid), len - mid), //add method on raw pointers is also unsafe, because it must trust that the offset location is also a valid pointer.
        )
    }
}
```

### Using `extern` Functions to Call External Code
Sometimes, your Rust code might need to interact with code written in another language. 
For this, Rust has the keyword extern that facilitates the creation and use of a Foreign Function Interface (FFI).
FFI is a way to define functions and enable a different (foreign) programming language to call those functions.
ex:
```rs
unsafe extern "C" {
   safe fn abs(input: i32) -> i32;  //list the names and signatures of external functions from another language we want to call
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```
Marking a function as safe does not inherently make it safe! Instead, it is like a promise you are making to Rust that it is safe. It is still your responsibility to make sure that promise is kept!

### Accessing or Modifying a Mutable Static Variable
If two threads are accessing the same mutable global variable, it can cause a data race.
global variables are called static variables.
```rs
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {HELLO_WORLD}");
}
```
Static variables are similar to constants.
names of static variables are in `SCREAMING_SNAKE_CASE` by convention; store references with the `'static` lifetime.

A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory
Another difference is that static variables can be mutable.
Accessing and modifying mutable static variables is <i>unsafe</i>.

```rs
static mut COUNTER: u32 = 0;    //code that reads or writes from COUNTER must be within an unsafe block

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {  //marked unsafe cause Having multiple threads access COUNTER would likely result in data races, so it is undefined behavior
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}
```
compiler won't allow you to create references to a mutable static variable.can only access it via a raw pointer.

With mutable data that is globally accessible, it’s difficult to ensure there are no data races, which is why Rust considers mutable static variables to be unsafe.

### Implementing an Unsafe Trait
use `unsafe` to implement an unsafe trait.

A trait is unsafe when at least one of its methods has some invariant that the
compiler can’t verify.
ex:
```rs
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {       //By using unsafe impl, we’re promising that we’ll uphold the invariants that the compiler can’t verify.

    // method implementations go here
}

fn main() {}
```

Note: **Accessing union fields is unsafe because Rust can’t guarantee the type
of the data currently being stored in the union instance**.

### Using Miri to check unsafe code
use `Miri` tool to check if the code written is safe and correct or not.
The borrow checker is a static tool which works at compile time,whereas Miri is a dynamic tool which works at runtime.

checks your code by running your program, or its test suite, and detecting when you violate the rules it understands about how Rust should work.
Using Miri requires a nightly build of Rust, install it via
```sh
rustup +nightly component add miri                      # install locally
cargo +nightly miri run or cargo +nightly miri test     # use in project
```

- It helpfully and correctly notices that we have shared references to mutable data, and warns about it
- it can actually tell us that some code is sure to be wrong and make recommendations about how to fix it.

If Miri does catch a problem, you know there’s a bug, but just because Miri doesn’t catch a bug doesn’t mean there isn’t a problem.

## 20.2 | Advanced Traits
getting into the nitty-gritty of traits.

### Specifying Placeholder Types in Trait Definitions with Associated Types
associated types are inserted into signatures of traits.
ex: Iterator trait that the standard library provides
```rs
pub trait Iterator {
    type Item;  //Item is associated type which is a placeholder

    fn next(&mut self) -> Option<Self::Item>;
}
```

consider following example to differ b/w Associated Types and Generics:
```rs
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
```

```rs
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

while using generics we must annotate the types in each implementation

when a trait has a generic parameter, it can be implemented for a type multiple times, changing the concrete types of the generic type parameters each time.

With associated types, we don’t need to annotate types because we can’t implement a trait on a type multiple times.

Associated types also become part of the trait’s contract: implementors of the
trait must provide a type to stand in for the associated type placeholder.

## Default Generic Type Parameters and Operator Overloading

Rust allows setting **default types** for generic parameters using the syntax `<T=DefaultType>`. This is useful when the default works in most cases, so implementors don’t have to explicitly specify it.

### Operator Overloading with Traits

Rust doesn’t let you define custom operators, but it does let you overload existing ones like `+` by implementing traits from `std::ops`. For example, the `Add` trait is used to overload the `+` operator.

Here's how you can add two `Point` instances using `Add`:

```rust
// Filename: src/main.rs
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```

This works because `Add` is defined as:

```rust
trait Add<Rhs=Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

Here, `Rhs` (right-hand side) defaults to `Self`, so `Add for Point` assumes `Point + Point`.

### Customizing the Right-Hand Side

You can override the default `Rhs` when needed. For instance, adding `Millimeters` and `Meters`:

```rust
// Filename: src/lib.rs
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

This uses the **newtype pattern** (wrapping existing types in structs) and shows how to handle unit conversions during addition.

### When to Use Default Type Parameters

1. **To extend traits without breaking existing code**
2. **To simplify common use cases while allowing advanced customization**

The `Add` trait is a great example—it covers typical same-type addition but lets you opt into more complex behavior when needed.

### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

Rust allows multiple traits (and types) to define methods with the same name. When a type implements several traits with identically named methods—or has its own method with that name—you must **disambiguate** which one to call.

### Example with Methods

Two traits, `Pilot` and `Wizard`, both define a `fly` method. The `Human` struct implements both traits and also defines its own `fly` method:

```rust
// Filename: src/main.rs
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

When you call `person.fly()`, Rust chooses the method directly implemented on the type:

```rust
fn main() {
    let person = Human;
    person.fly();
}
```

Output:

```
*waving arms furiously*
```

To call the trait methods explicitly, use the trait name:

```rust
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

Output:

```
This is your captain speaking.
Up!
*waving arms furiously*
```

### Example with Associated Functions (No `self`)

With associated functions (functions not using `self`), Rust can't infer which trait you mean when names clash. For example:

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}
```

This prints:

```
A baby dog is called a Spot
```

If you try to call `Animal::baby_name()`, Rust doesn’t know which type’s implementation to use:

```rust
// This fails to compile
fn main() {
    println!("A baby dog is called a {}", Animal::baby_name());
}
```

To fix this, use **fully qualified syntax**:

```rust
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

Output:

```
A baby dog is called a puppy
```

### Fully Qualified Syntax Format

Use this when disambiguation is needed:

```
<Type as Trait>::function(arguments...);
```

Use it when:

* Multiple traits define the same method name.
* Traits define associated functions without `self`.
* Rust can't infer the type from context.

In most cases, you don’t need this syntax unless there's a name conflict.

---

### Using Supertraits to Require One Trait’s Functionality Within Another Trait

Sometimes, you might write a trait definition that depends on another trait: for a type to implement the first trait, you want to require that type to also implement the second trait. You would do this so that your trait definition can make use of the associated items of the second trait. The trait your trait definition is relying on is called a *supertrait* of your trait.

For example, let’s say we want to make an `OutlinePrint` trait with an `outline_print` method that will print a given value formatted so that it’s framed in asterisks. That is, given a `Point` struct that implements the standard library trait `Display` to result in `(x, y)`, when we call `outline_print` on a `Point` instance that has `1` for `x` and `3` for `y`, it should print the following:

```
**********
*        *
* (1, 3) *
*        *
**********
```

In the implementation of the `outline_print` method, we want to use the `Display` trait’s functionality. Therefore, we need to specify that the `OutlinePrint` trait will work only for types that also implement `Display` and provide the functionality that `OutlinePrint` needs. We can do that in the trait definition by specifying `OutlinePrint: Display`. This technique is similar to adding a trait bound to the trait. Listing 20-23 shows an implementation of the `OutlinePrint` trait.

#### Filename: src/main.rs

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

Listing 20-23: Implementing the `OutlinePrint` trait that requires the functionality from `Display`.

Because we’ve specified that `OutlinePrint` requires the `Display` trait, we can use the `to_string` function that is automatically implemented for any type that implements `Display`. If we tried to use `to_string` without adding a colon and specifying the `Display` trait after the trait name, we’d get an error saying that no method named `to_string` was found for the type `&Self` in the current scope.

Let’s see what happens when we try to implement `OutlinePrint` on a type that doesn’t implement `Display`, such as the `Point` struct:

#### Filename: src/main.rs

This code does not compile!

```rust
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

We get an error saying that `Display` is required but not implemented:

```
error[E0277]: `Point` doesn't implement `std::fmt::Display`
   --> src/main.rs:20:23
    |
20  | impl OutlinePrint for Point {}
    |                       ^^^^^ `Point` cannot be formatted with the default formatter
```

To fix this, we implement `Display` on `Point` and satisfy the constraint that `OutlinePrint` requires, like so:

#### Filename: src/main.rs

```rust
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

Then implementing the `OutlinePrint` trait on `Point` will compile successfully, and we can call `outline_print` on a `Point` instance to display it within an outline of asterisks.

---

### Using the Newtype Pattern to Implement External Traits on External Types

In Chapter 10 in the “Implementing a Trait on a Type” section, we mentioned the *orphan rule* that states we’re only allowed to implement a trait on a type if either the trait or the type are local to our crate. It’s possible to get around this restriction using the *newtype pattern*, which involves creating a new type in a tuple struct.

The tuple struct will have one field and be a thin wrapper around the type we want to implement a trait for. Then the wrapper type is local to our crate, and we can implement the trait on the wrapper. *Newtype* is a term that originates from the Haskell programming language. There is no runtime performance penalty for using this pattern, and the wrapper type is elided at compile time.

As an example, let’s say we want to implement `Display` on `Vec<T>`, which the orphan rule prevents us from doing directly because the `Display` trait and the `Vec<T>` type are defined outside our crate. We can make a `Wrapper` struct that holds an instance of `Vec<T>`; then we can implement `Display` on `Wrapper` and use the `Vec<T>` value, as shown in Listing 20-24.

#### Filename: src/main.rs

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
```

Listing 20-24: Creating a `Wrapper` type around `Vec<String>` to implement `Display`.

The implementation of `Display` uses `self.0` to access the inner `Vec<T>`, because `Wrapper` is a tuple struct and `Vec<T>` is the item at index 0 in the tuple. Then we can use the functionality of the `Display` trait on `Wrapper`.

The downside of using this technique is that `Wrapper` is a new type, so it doesn’t have the methods of the value it’s holding. We would have to implement all the methods of `Vec<T>` directly on `Wrapper` such that the methods delegate to `self.0`, which would allow us to treat `Wrapper` exactly like a `Vec<T>`. If we wanted the new type to have every method the inner type has, implementing the `Deref` trait (discussed in Chapter 15 in the “Treating Smart Pointers Like Regular References with the Deref Trait” section) on the `Wrapper` to return the inner type would be a solution. If we don’t want the `Wrapper` type to have all the methods of the inner type—for example, to restrict the `Wrapper` type’s behavior—we would have to implement just the methods we do want manually.

This *newtype pattern* is also useful even when traits are not involved. Let’s switch focus and look at some advanced ways to interact with Rust’s type system.

---

## 20.3 | Here’s your cleaned-up version with headings preserved using `###`, and all code blocks and structure intact:

---

### Using Supertraits to Require One Trait’s Functionality Within Another Trait

Sometimes, you might write a trait definition that depends on another trait: for a type to implement the first trait, you want to require that type to also implement the second trait. You would do this so that your trait definition can make use of the associated items of the second trait. The trait your trait definition is relying on is called a *supertrait* of your trait.

For example, let’s say we want to make an `OutlinePrint` trait with an `outline_print` method that will print a given value formatted so that it’s framed in asterisks. That is, given a `Point` struct that implements the standard library trait `Display` to result in `(x, y)`, when we call `outline_print` on a `Point` instance that has `1` for `x` and `3` for `y`, it should print the following:

```
**********
*        *
* (1, 3) *
*        *
**********
```

In the implementation of the `outline_print` method, we want to use the `Display` trait’s functionality. Therefore, we need to specify that the `OutlinePrint` trait will work only for types that also implement `Display` and provide the functionality that `OutlinePrint` needs. We can do that in the trait definition by specifying `OutlinePrint: Display`. This technique is similar to adding a trait bound to the trait. Listing 20-23 shows an implementation of the `OutlinePrint` trait.

#### Filename: src/main.rs

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

Listing 20-23: Implementing the `OutlinePrint` trait that requires the functionality from `Display`.

Because we’ve specified that `OutlinePrint` requires the `Display` trait, we can use the `to_string` function that is automatically implemented for any type that implements `Display`. If we tried to use `to_string` without adding a colon and specifying the `Display` trait after the trait name, we’d get an error saying that no method named `to_string` was found for the type `&Self` in the current scope.

Let’s see what happens when we try to implement `OutlinePrint` on a type that doesn’t implement `Display`, such as the `Point` struct:

#### Filename: src/main.rs

This code does not compile!

```rust
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

We get an error saying that `Display` is required but not implemented:

```
error[E0277]: `Point` doesn't implement `std::fmt::Display`
   --> src/main.rs:20:23
    |
20  | impl OutlinePrint for Point {}
    |                       ^^^^^ `Point` cannot be formatted with the default formatter
```

To fix this, we implement `Display` on `Point` and satisfy the constraint that `OutlinePrint` requires, like so:

#### Filename: src/main.rs

```rust
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

Then implementing the `OutlinePrint` trait on `Point` will compile successfully, and we can call `outline_print` on a `Point` instance to display it within an outline of asterisks.

---

### Using the Newtype Pattern to Implement External Traits on External Types

In Chapter 10 in the “Implementing a Trait on a Type” section, we mentioned the *orphan rule* that states we’re only allowed to implement a trait on a type if either the trait or the type are local to our crate. It’s possible to get around this restriction using the *newtype pattern*, which involves creating a new type in a tuple struct.

The tuple struct will have one field and be a thin wrapper around the type we want to implement a trait for. Then the wrapper type is local to our crate, and we can implement the trait on the wrapper. *Newtype* is a term that originates from the Haskell programming language. There is no runtime performance penalty for using this pattern, and the wrapper type is elided at compile time.

As an example, let’s say we want to implement `Display` on `Vec<T>`, which the orphan rule prevents us from doing directly because the `Display` trait and the `Vec<T>` type are defined outside our crate. We can make a `Wrapper` struct that holds an instance of `Vec<T>`; then we can implement `Display` on `Wrapper` and use the `Vec<T>` value, as shown in Listing 20-24.

#### Filename: src/main.rs

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
```

Listing 20-24: Creating a `Wrapper` type around `Vec<String>` to implement `Display`.

The implementation of `Display` uses `self.0` to access the inner `Vec<T>`, because `Wrapper` is a tuple struct and `Vec<T>` is the item at index 0 in the tuple. Then we can use the functionality of the `Display` trait on `Wrapper`.

The downside of using this technique is that `Wrapper` is a new type, so it doesn’t have the methods of the value it’s holding. We would have to implement all the methods of `Vec<T>` directly on `Wrapper` such that the methods delegate to `self.0`, which would allow us to treat `Wrapper` exactly like a `Vec<T>`. If we wanted the new type to have every method the inner type has, implementing the `Deref` trait (discussed in Chapter 15 in the “Treating Smart Pointers Like Regular References with the Deref Trait” section) on the `Wrapper` to return the inner type would be a solution. If we don’t want the `Wrapper` type to have all the methods of the inner type—for example, to restrict the `Wrapper` type’s behavior—we would have to implement just the methods we do want manually.

This *newtype pattern* is also useful even when traits are not involved. Let’s switch focus and look at some advanced ways to interact with Rust’s type system.

---
## 20.3 | Advanced Types


# 20.5 | Macro
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

---------------------------------------
# Project Idea
1. Backend for a full stack app
2. CLIs
---------------------------------------
