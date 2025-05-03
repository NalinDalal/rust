// Define an enum called Shape
enum Shape {
    Circle(f64),         // Variant with associated data (radius)
    Square(f64),         // Variant with associated data (side length)
    Rectangle(f64, f64), // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side_length) => side_length * side_length,
        Shape::Rectangle(width, height) => width * height,
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1), //Matching Named Variables
    }
}
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}
//function to match mutiple pattern
fn multiPatt(x: i32) {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

//Matching Ranges of Values with ..=
fn range(x: i32) {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

//destructuring matching
struct Point {
    x: i32,
    y: i32,
}

fn destruc() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    // Calculate and print the areas
    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));

    let five = Some(5);
    //check in plus_one function, does it match the first variant{None} - no
    //moves to next variant, does it matches second variant{Some(i)} - yes, excute it
    let _six = plus_one(five); //well ignores this one , even if not
                               //called
    let _none = plus_one(None);

    //normal if-else ugly approach
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        //refutability
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    //tuple matching
    let (x, y, z) = (1, 2, 3);
    print!("x= {}, y= {},z= {}", x, y, z);

    //function matching
    let point = (3, 5);
    print_coordinates(&point);

    //literal matching
    let k = 1;

    match k {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
