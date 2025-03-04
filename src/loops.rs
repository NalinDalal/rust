fn forLoops() {
    let mut x: i8 = 10;
    //for loop
    for _ in 0..100 {
        x = x + 100;
    }
    println!("x is {}", x);
}
//loops();

fn main() {
    println!("counting from 0 to 100->");
    forLoops();
    let sentence: String = String::from("harkirat");
    let first_word: String = get_first_word(sentence);
    let n: i32 = 1000;
    for i in 0..n {
        println!("Hello, world! {}", i);
    }
    print!("First word is: {}", first_word);
}
fn get_first_word(sentence: String) -> String {
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    //return ans;
    ans // No need for explicit `return ans;`
}

fn while_loop() {
    let mut x: i8 = 100;
    println!("while loop run->");
    while x > 0 {
        //while loop called with condition
        x = x - 1;
        println!("{}", x);
    }
}
