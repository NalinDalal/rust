#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        //known as getter method,
        //we give a method the same name as a field we want it to only return the value in the field
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!(
        "The area of the rectangle1 is {} square pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle1 has a nonzero width; it is {}", rect1.width);
    }
    //width method return true if the value in the instance’s width field is greater than 0 and false if the value is 0
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
