extern crate clap;

use clap::{Arg, App};

use std::fs::File;
use std::io::{BufReader, BufRead};
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

	let file = match File::open(args.value_of("file").unwrap()) {
		Err(_) => {
			println!("Failed to open file");
			process::exit(1);
		},
		Ok(file) => file,
	};

	let file = BufReader::new(file);

	for line in file.lines() {
		let mut line = line.unwrap();
		match interpret_line(&mut line) {
			Ok(out) => println!("{:x}", out),
			Err(e) => {
				println!("Syntax error: {}", e);
				process::exit(2);
			},
		}
	}
}
