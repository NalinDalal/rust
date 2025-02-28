//Procedural Macro
//Custom derive macros allow you to define how Rust derives certain traits for types. A common use case is generating code for trait implementations (like Debug, Clone, etc.).
#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
    age: u32,
}

//Attribute-like Macros
#[route("GET")]
fn home() {
    println!("Welcome to the home page!");
}

#[route("POST")]
fn create_post() {
    println!("Creating a new post!");
}
