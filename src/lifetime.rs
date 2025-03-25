fn longest(a: String, b: String) -> String {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}

//with structs
struct User<'a, 'b> {
    first_name: &'a str,
    last_name: &'b str,
}
fn main() {
    let user: User;
    let first_name = String::from("Nalin");
    //lifetime is just inside that
    {
        let last_name = String::from("Singh");
        user = User {
            first_name: &first_name,
            last_name: &last_name, //dangling pointer error, cause user doesn't exist anymore, so
                                   //it just have that scope
                                   //outside the lastname lifetime block
                                   //says last_name dropped while still borrowed
                                   //cause the thing is we are calling again user later after the lifetime
                                   //so to help with it, just move that println function to scope
        };
        println!("The name of user is {}", user.first_name);
    }
    let longest_str;
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        longest_str = longest(str1, str2);
    }
    println!("{}", longest_str);
}

//generic type parameters, trait bounds, and lifetimes all in one function!
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
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
