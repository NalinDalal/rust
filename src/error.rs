use std::fs;
use std::io::Error;

fn main() {
    // Try to read the file "hello.txt"
    let greeting_file_result = fs::read_to_string("hello.txt");

    match greeting_file_result {
        Ok(file_content) => {
            println!("File read successfully: {:?}", file_content);
        }
        Err(error) => {
            println!("Failed to read file: {:?}", error);
        }
    }

    // Read from file using the corrected function
    match read_from_file_unsafe("example.txt".to_string()) {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error reading file: {}", error),
    }
}

// Function to read file content and handle errors properly
fn read_from_file_unsafe(filename: String) -> Result<String, String> {
    match fs::read_to_string(&filename) {
        Ok(content) => Ok(content),
        Err(_) => Err("Error reading file".to_string()), // Handle error gracefully
    }
}
