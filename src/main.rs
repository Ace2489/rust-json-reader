use std::fs;
use std::env;
use json::JsonValue;

fn main(){

    let Some(json_path) = env::args().nth(1) else {panic!("No file to query")};
    
    let Some(query)= env::args().nth(2) else {panic!("No query to execute")};

    let Ok(json_str) = fs::read_to_string(&json_path) else {panic!("No file named {json_path:?} exists")};

    let parsed = json::parse(&json_str.to_string());

    match parsed {
        Ok(JsonValue::Object(obj))=>{
            if let Some(value) = obj.get(&query){
                println!("Found the value {value:?}");
            }
        }
        Ok(_)=> println!("Root is not an object!"),
        Err(err)=>println!("The operation failed with the error {err:?}"),   
    
    }
}
