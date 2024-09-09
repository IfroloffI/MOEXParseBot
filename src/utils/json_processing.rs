use std::fs;
use serde_json;

pub mod json{
    fn create_valid_json_value(value: String) -> String {
        value[1..value.len() - 1].to_string()
    }

    pub fn get_private_data() -> serde_json::Value {
        let got_data: Result<String, std::io::Error> = std::fs::read_to_string("protected/data.json");
        let s = match got_data {
            Ok(s) => s,
            Err(_) => panic!("Can't read file")
        };
        serde_json::from_str(&s).expect("Can't parse json")
    }

    pub fn get_token(secure_data: &serde_json::Value) -> String {
        create_valid_json_value(secure_data["privateData"]["token"].to_string())
    }
}
