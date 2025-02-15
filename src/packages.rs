//to import packages-> cargo add <package_name>
//ex: cargo add chrono
//appends into Caargo.toml
use chrono::Local;

fn main() {
    let now = Local::now();
    println!("Current time is: {}", now);
}
