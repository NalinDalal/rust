use std::fs;
enum Result<A, B> {
    Ok(A),
    Err(B),
}
fn main() {
    let greeting_file_result = fs::read_to_string("hello.txt");
    match greeting_file_result {
        Ok(file_content) => {
            println!("File read successfully: {:?}", file_content);
        }
        Err(error) => {
            println!("Failed to read file: {:?}", error);
        }
    }
    let res=read_from_file_unsafe("example.txt".to_string());

}
option
fn read_from_file_unsafe(String)->String{
    let res=fs::read_to_string("example.txt");
    return res.unwrap();
    if let Ok(file_content:String)->Result<String,String>{
        let res=fs::read_to_string("example.txt");

    let let Ok(content:String)=res{return Ok(content);}else{return Err("error reading fle".to_string());}
}
}
