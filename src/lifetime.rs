fn longest(a: String, b: String) -> String {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}

fn main() {
    let longest_str;
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        longest_str = longest(str1, str2);
    }
    println!("{}", longest_str);
}

//with structs
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

