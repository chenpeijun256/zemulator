use std::fs;
use serde_json::json;

pub fn create_soc(cfg_file: String) {
    match fs::read(cfg_file) {
        Ok(bytes) => {
            match String::from_utf8(bytes) {
                Ok(json_str) => {
                    println!("{}", json_str);
                    let json = json!(json_str);
                    println!("name:{:?}", json["name"].to_string());
                    // for m in json["mem"].as_array() {
                    //     println!("mem name: {}", m[0]);
                    // }
                },
                Err(_) => println!("string from_utf8 error."),
            }
        },
        Err(_) => println!("file read error."),
    }
}