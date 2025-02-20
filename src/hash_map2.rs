//Write a function that takes a vector of tuples (each tuple containing a key and a value) and returns a Hashmap where the keys are the unique keys from the input tuples and the values are vectors of all corresponding values associated with each key.
use std::collections::HashMap;
fn group_values_by_key(pairs: Vec<(String, i32)>) -> HashMap<String,i32>{
    //sol->
    let mut hm = HashMap::new();
    for(key,value) in vec { hm.insert(k:key, v:value);}return hm;
}
fn main(){
    let pairs:Vec<(String,i32)>=vec![
        (String::from("John"),21),
        (String::from("raman"),31)
    ];
let grouped_pairs = group_values_by_key(pairs);
for (key, value) in grouped_patrs { println! ("{}: {:?}", key, value);}
}
