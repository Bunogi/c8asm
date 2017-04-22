extern crate clap;

use clap::{Arg, App};

use std::fs::File;
use std::io::{Read};
use std::process;

mod instructions;
use instructions::*;

mod interpret;
use interpret::*;

fn main() {
	let args = App::new("b8asm")
		.version("0.0")
		.author("Bunogi")
		.arg(Arg::with_name("file").help("File to use").required(true))
		.about("Assembles Chip-8 programs")
		.get_matches();

	let mut file = match File::open(args.value_of("file").unwrap()) {
		Err(_) => {
			println!("Failed to open file");
			process::exit(1);
		},
		Ok(file) => file,
	};

	let mut offset = 0u16;

	let mut lines = String::new();
	file.read_to_string(&mut lines).unwrap();

	let lines: Vec<&str> = lines.split("\n").collect();
	let mut ready_lines: Vec<String> = Vec::new();

	for line in lines {
		let mut line = line.to_string();
		sanitize_line(&mut line);
		if line != "" {
			ready_lines.push(line);
		}
	}

	let mut labels: Vec<Label> = Vec::new();
	let mut instructions: Vec<&str> = Vec::new();

	//First pass: Find labels and variables
	let mut index = 0;
	for line in &ready_lines {
		let data: Vec<&str> = line.split(' ').collect();
		if data.len() == 1 { //Possible label
			match data[0].find(':') {
				Some(i) => labels.push( Label {name: &data[0][0..i], offset: offset} ), //Definetly a label
				None    => {
					println!("Malformed label on line {}: {}", index + 1, line.as_str());
					process::exit(2);
				},
			}
		} else {
			offset += 1;
			instructions.push(line);
		}
		index += 1; 
	}

	index = 0;
	for line in &instructions {
		let data: Vec<&str> = line.split(' ').collect();
		match interpret_line(&data, &labels) {
			Ok(out) => println!("{:x}", out),
			Err(e)  => {
				println!("Syntax error on line {}: {}", index + 1, e + line);
				process::exit(2);
			},
		}
		index += 1;
	}
}
