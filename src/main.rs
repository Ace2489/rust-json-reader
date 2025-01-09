use json::JsonValue;
use std::env;
use std::fs;

fn main() {
    let Some(json_path) = env::args().nth(1) else {
        panic!("No file to query")
    };

    let Some(query) = env::args().nth(2) else {
        panic!("No query to execute")
    };

    let queries: Vec<&str> = query.split('.').collect();

    let Ok(json_str) = fs::read_to_string(&json_path) else {
        panic!("No file named {json_path:?} exists")
    };

    let parsed = json::parse(&json_str.to_string());

    match parsed {
        Ok(JsonValue::Object(obj)) => {
            let obj = &JsonValue::Object(obj);
            let mut value: Option<&JsonValue> = Some(obj);

            for query_part in queries {
                value = match value {
                    Some(JsonValue::Object(obj)) => obj.get(query_part),
                    _ => {
                        break;
                    }
                };
            }

            if let Some(val) = value {
                println!("The value of the query is {val:#}")
            };
        }
        Ok(_) => println!("Root is not an object!"),
        Err(err) => println!("The operation failed with the error {err:?}"),
    }
}
