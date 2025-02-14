struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);
    }
}
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    print!("User 1 username: {:?}", user1.username);
    let rect1 = Rect {
        width: 10,
        height: 20,
    };
    print!("The are of rectangle is {}", rect1.area()); //note we called method defined on instance
                                                        //of structure Rect which is rect1
    print!("The perimeter of rectangle is {}", rect1.perimeter());
}
