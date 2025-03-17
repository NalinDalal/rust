// Define an enum called Direction with explicit values to enforce four strict variants.
#[derive(Debug)]
enum Direction {
    North = 0, // Represents moving up
    East = 1,  // Represents moving right
    South = 2, // Represents moving down
    West = 3,  // Represents moving left
}

// Function to move a character based on direction
fn move_around(direction: Direction) {
    // Uses pattern matching to determine movement
    match direction {
        Direction::North => println!("Moving North"),
        Direction::East => println!("Moving East"),
        Direction::South => println!("Moving South"),
        Direction::West => println!("Moving West"),
    }
}

// Define an enum called Shape
enum Shape {
    Circle(f64),         // Variant with associated data (radius)
    Square(f64),         // Variant with associated data (side length)
    Rectangle(f64, f64), // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(shape: &Shape) -> f64 {
    // Uses pattern matching to extract values and calculate area
    match shape {
        Shape::Circle(radius) => {
            // Area of a circle = π * r^2
            std::f64::consts::PI * radius * radius
        }
        Shape::Square(side) => {
            // Area of a square = side * side
            side * side
        }
        Shape::Rectangle(width, height) => {
            // Area of a rectangle = width * height
            width * height
        }
    }
}

fn main() {
    // Create instances of different shapes with given dimensions
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    // Create an instance of Direction
    let my_direction = Direction::North;

    // Move character in the specified direction
    move_around(my_direction);

    // Compute and print the area of each shape
    println!("Circle area: {:.2}", calculate_area(&circle));
    println!("Square area: {:.2}", calculate_area(&square));
    println!("Rectangle area: {:.2}", calculate_area(&rectangle));
}
