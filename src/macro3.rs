use serde::{Deserialize, Serialize};

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
