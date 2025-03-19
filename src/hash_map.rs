use std::collections::HashMap;
fn main() {
    let mut users = HashMap::new();
    users.insert(String::from("John"), 22);
    users.insert(String::from("nalin"), 32);

    let first_user_age = users.get("John"); //Option<22>
    match first_user_age {
        Some(age) => println!("age is {}", age),
        None => println!("User not found in the db"),
    }
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); //score associated with blue team whose
                                             //value is 10
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    //print each pair in an arbitrary order:
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
