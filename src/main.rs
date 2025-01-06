use std::fs;
use json::JsonValue;

fn main(){
    let json_str = fs::read_to_string("data.json").expect("No file found");

    let parsed = json::parse(&json_str.to_string());

    match parsed {
        Ok(JsonValue::Object(obj))=>{
            if let Some(value) = obj.get("naive"){
                println!("Found the value {value:?}");
            }
        }
        Ok(_)=> println!("Root is not an object!"),
        Err(err)=>println!("The operation failed with the error {err:?}"),   
    
    }
}
