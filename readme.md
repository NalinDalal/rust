let's start with rust- fast and safe
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

# Ownership
Ownership is a set of rules that govern how a Rust program manages memory. Allprograms have to manage the way they use a computer’s memory while running. 
Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; 
in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a
system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. 
None of the features of ownership will slow down your program while it’s running.

Ex: say a girl always want a boyfriend/owner, say if she is single she will die,
but need to have atleast one owner/boyfriend. so if owner dies, then i must find a new owner.

## Stack Variables
if stack goes out of scope then heap dies, 

## Heap variables
Heap variables are like Rihana. They always want to have a single owner(boyfriend), and if their owner goes out of scope(boyfriend dies), they get deallocated(Rihana also dies).
Any time the owner of a heap variable goes out of scope, the value is de-allocated from the heap.it gets cleared

## Why
beacuse of dangling pointer error, say one clears it, but another has pointer to it. so not allowed.
heap-> single owner
```rs
//error code
fn main () {
let s1: String = String:: from("Hi there");
let s2: String = s1;
println!("{}",s2);}
//so now if i call s1 it will give error
```
This is to avoid memory issues like
1. Double free error.
2. Dangling pointers.
when owner goes out of scope, data gets cleared

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

References mean giving the address of a string rather than the ownership of the string over to a function
```rs
fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1;

    println!("{}", s2);
    println!("{}", s1);    // This is valid, The first pointer wasn't invalidated
}
```

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

that means rihana can only have only one editor at a time, either the onwer or
only one borrowerer

## Rules
- There can me many immutable references at the same time
- There can be only one mutable reference  at a time
- If there is a mutable reference , you can’t have another immutable reference either. 
- If someone makes an immutable reference , they don’t expect the value to change suddenly
- If more than one mutable references happen, there is a possibility of a data race and synchronization issues

# Struct
to collect over similar data together to bind them, like classes in cpp, objects in js
allows to make custom data binded together
```rs
struct Rect {
    width: u32,
    height: u32,
}
```

# Enums

enums and pattern matching, enums with values
Result and Option enum

# Pattern Matching
Let you pattern match across various variants of an enum and run some logic

# Error Handling
we know what type of error handling is in cpp, js like try catch block
Rust provides an Enum for same
```rs
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
it is an enum (with generic types)
This enum is what a function can return/returns when it has a possibility of throwing an error

example: error.rs
it returns a enum with the `Ok` variant having a string value and `Err` variant
having an Error value

option enum-> to introduce concept of nullability in a safe and expressive way.
if u ever have a function that should return null, return an Option instead.

# Package Management
You can add an external crate to your project by running -> 
```bash
cargo add crate_name
````

crate-> term for packages in rust, just like express, zod etc.
ex: chrono for date and time

# Memory Management
amount of space a number take doesn't changes as time goes b, but for a string
it may change

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

# Collections
just like stl in cpp.
Rust's standard library includes a number of very useful data structures called collections.
Most other data types represent one specific value, but collections can contain multiple values. the data these collections point to is stored on the heap

# Vectors
Similar to vector in cpp
same as stack and heap,
basically the array is on heap, but stack has the pointer to that heap
heap can be increased decreased
need to make it mutable
```rs
let mut vec=Vec::new();
vec.push(1);
vec.push(2);
```
more code in vector.rs file

# HashMaps
they store key value pairs in rust
use a library-> `use std::collections::HashMaps`
`HashMap.rs`

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
like for filer->
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


# Generics
used to remove code repetetion
like generics in cpp
```rs
fn main() {
let bigger = largest(1, 2);
let bigger_char = largest( 'a', 'b');
println!("{}", bigger);
println!("{}", bigger_char);}

fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T
{
if
a > b {a}else{b}}
```
args are T, return type is also T, just trait bound it

# Traits
like interface in TS,JS
defines functionality of a type which can be shared to other types
it's like a blueprint for Structs to follow

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


# Lifetimes
say to print longest string b/w 2
so u define a function longest() utilise the size function 
now for main function u call a variable initially
but after input you store that function into that vairable initialises initially
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

so basically lifetime is span where variable and function are valid

so like s1 has lifespan `a, for s2 it is `a
so ans lifespan is intersection of both
so basically space where s1, s2 is valid both, their return type is valid at
their intersection

say you have s2 inside s1 so return type will always will be in s2, as it is
intersection s1 ans s2

## Structs with lifeTimes
same but struct is also treated a function
```rs
struct User<'a,'b>{
    first_name:&'a str,
    last_name:&'b str,`
}
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
