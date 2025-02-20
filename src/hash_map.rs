use std::collections::HashMap;
fn main(){
    let mut users=HashMap::new();
users.insert(k:String::from("John"),v:22);
    users.insert(k:String::from("nalin"),v:32);

    let first_user_age = users.get ("John");    //Option<22>
    match first_user _age {
Some (age) => println!("age is {}", age),
None => println! ("User not found in the db"),}
}
