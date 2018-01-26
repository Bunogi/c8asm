use std::vec::Vec;
use std::string::String;

use interpret::sanitize_line;

pub fn preprocess(input: &Vec<&str>) -> Vec<String> {
    //Sanitize
    let mut converted: Vec<String> = Vec::new();
    for i in input {
        let mut line = i.to_string();
        sanitize_line(&mut line);
        if !line.is_empty() {
            converted.push(line);
        }
    }

    converted
}
