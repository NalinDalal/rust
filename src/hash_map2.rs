//Write a function that takes a vector of tuples (each tuple containing a key and a value) and returns a Hashmap where the keys are the unique keys from the input tuples and the values are vectors of all corresponding values associated with each key.
use std::collections::HashMap;
fn group_values_by_key(pairs: Vec<(String, i32)>) -> HashMap<String, Vec<i32>> {
    //sol->
    let mut hm = HashMap::new();
    for (key, value) in pairs {
        hm.entry(key).or_insert(Vec::new()).push(value);
    }
    hm
}
fn main() {
    let pairs: Vec<(String, i32)> = vec![
        (String::from("John"), 21),
        (String::from("raman"), 31),
        (String::from("Naman"), 25),
        (String::from("Nitesh"), 35),
    ];
    let grouped_pairs = group_values_by_key(pairs);
    for (key, value) in grouped_pairs {
        println!("{}: {:?}", key, value);
    }
}
