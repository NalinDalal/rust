fn main(){
    let mut vec = Vec::new();   //create a empty vector
    vec.push(1);    //use push method to add elements into a mutable vector
    vec.push(2);
    vec.push(3);
    let ans=even_filter(&vec);  //pass via reference
    println!("{:?}", vec); // [1, 2,3]
    println!("{:?}",ans);
    println!("{:?}",even_filter(vec));//[2]


    //acessing elements
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");   //with indexing syntax

    let third: Option<&i32> = v.get(2);   //with get method
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn even_filter(vec:Vec<i32>)->Vec<i32>{
    let mut new_vec = Vec::new();
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
