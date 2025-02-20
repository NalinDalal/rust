fn main(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    let ans=even_filter(&vec);  //pass via reference
    println!("{:?}", vec); // [1, 2,3]
    println!("{:?}",ans);
    println!("{:?}",even_filter(vec));//[2]
}

fn even_filter(vec:Vec<i32>)->Vec<i32>{
    let mut new vec = Vec::new();
    for val in vec{
        if val % 2 == 0 { 
            new_vec.push(val);
        }
    }
    return new_vec;
}

//via macros
fn macro(){
    let numbers=vec![1,2,3];
    for number in numbers{
        println!("{}",number);
    }
}

//via generic types
fn generic() {
let numbers: Vec<i32> = vec! [1, 2, 3];
for number in numbers{ println! ("{}", number);}}
