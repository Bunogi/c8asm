use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;

//Returns a string with comments removed, with everything in lowercase
pub fn sanitize_line(input: &mut String) {
    while let Some(i) = input.find(',') {
        input.remove(i);
    }

    match input.find(';') {
        Some(i) => *input = input[0..i].to_string(),
        None => {}
    }

    *input = input.trim().to_string().to_lowercase();
}

//Preprocessor should support #define VAL xyz
pub fn preprocess(input: &Vec<&str>) -> Result<Vec<String>, (String, u32)> {
    let mut output: Vec<String> = input.iter().map(|x| x.to_string()).collect();

    let mut definitions: HashMap<String, String> = HashMap::new();
    let mut line_num = 0u32;

    //Find definitions
    for i in 0..output.len() {
        let current_line = output[i].trim_left();

        if current_line.starts_with("#") {
            //Preprocessor definition
            let mut words = current_line.split_whitespace();
            match &words.next().unwrap()[1..] {
                "define" => {
                    let name = words.next();
                    let mut definition = String::new();
                    words.for_each(|x| {
                        definition += x;
                        definition += " "
                    });
                    definitions.insert(name.unwrap().to_string(), definition);
                }
                _ => {
                    return Err((
                        format!("Invalid preprocessor directive: {}", output[i]),
                        line_num,
                    ))
                }
            }
            if current_line[1..8] == *"define " {
                let end_name = current_line[9..].find(' ').unwrap() + 9;
                let name = String::from(&current_line[8..end_name]);
                let definition = String::from(&current_line[end_name..]);
                definitions.insert(name, definition);
            } else {
                return Err((
                    format!("Invalid preprocessor directive: {}", output[i]),
                    line_num,
                ));
            }
        }
        line_num += 1;
    }

    line_num = 0;

    //Replace keywords
    for i in 0..output.len() {
        let mut current_line = &mut output[i];

        let mut split: Vec<String> = current_line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        //Replace variable
        let mut new: Vec<&String> = Vec::with_capacity(split.len());
        for word in &split {
            if word.starts_with("$") {
                let to_check;
                if word.ends_with(",") {
                    to_check = &word[1..word.len() - 1];
                } else {
                    to_check = &word[1..];
                }
                match definitions.get(to_check) {
                    Some(n) => new.push(n),
                    None => return Err((format!("Unknown variable: {}", word), line_num)),
                }
            } else {
                new.push(word);
            }
        }

        //Remove preprocessor lines
        current_line.clear();
        if new.len() > 0 {
            if !new[0].starts_with("#") {
                for word in new {
                    current_line.push_str(word);
                    current_line.push_str(" ");
                }
            }
        }

        line_num += 1;
    }

    //Sanitize
    let mut converted: Vec<String> = Vec::new();
    for i in output {
        let mut line = i.to_string();
        sanitize_line(&mut line);
        if !line.is_empty() {
            converted.push(line);
        }
    }

    Ok(converted)
}
