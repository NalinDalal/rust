pub trait Summary {
    fn Summarize(&self) -> String {
        return String::from("Summarize");
    }
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    //User struct implements Summary trait
    fn summarize(&self) -> String {
        return format!("User {} is {} years old", self.name, self.age);
    }
}

fn main() {
    let user = User {
        name: String::from("nalin"),
        age: 21,
    };
    println!("{}", user.summarize());
}
