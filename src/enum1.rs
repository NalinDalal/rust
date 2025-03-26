enum Direction {
    North,
    East,
    South,
    West,
}

fn move_around(direction: Direction) {
    //implement logic to move a character around
}

//well it allows char to move only in 4 direction
//enforcing four variants
//make code more strict
//enum with values
// Define an enum called Shape
enum Shape {
    Circle(f64),         // Variant with associated data (radius)
    Square(f64),         // Variant with associated data (side length)
    Rectangle(f64, f64), // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    // calculates the area of the shape
    return 0.0;
}

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    let my_direction = Direction::North;
    move_around(my_direction);
}
