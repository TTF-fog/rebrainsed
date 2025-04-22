use std::fs;
use serde_json::Value;

pub fn load_jetbrains_ides() -> Vec<String> {
    match fs::read_to_string("names.json") {
        Ok(contents) => {
            match serde_json::from_str::<Value>(&contents) {
                Ok(json) => {
                    json["jetbrains_ides"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                }
                Err(e) => {
                    eprintln!("Error parsing JSON: {}", e);
                    Vec::new()
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            Vec::new()
        }
    }
}

pub fn is_jetbrains_ide(name: &str, ides: &[String]) -> (bool,String) {
    for ide in ides {
        if ide.to_lowercase() == name.to_lowercase() {
            return (true,ide.to_string())
        }
    }
    return (false,String::new())
} 