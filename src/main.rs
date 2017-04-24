//Command line args
extern crate clap;
use clap::{Arg, App};

//Set endianness
extern crate byteorder;
use byteorder::{WriteBytesExt, BigEndian};

use std::fs::File;
use std::io::{Read, Write};
use std::process;

mod instructions;
use instructions::*;

mod interpret;
use interpret::*;

fn main() {
	let args = App::new("c8asm")
		.version("0.0.1")
		.author("Bunogi")
		.arg(Arg::with_name("file").help("File to assemble").required(true))
		.arg(Arg::with_name("output").help("Output file").default_value("./a.out").short("o"))
		.arg(Arg::with_name("stdout").help("Write output to stdout instead of your file").short("s").conflicts_with("o"))
		.about("Assembles Chip-8 programs")
		.get_matches();

	let mut file = match File::open(args.value_of("file").unwrap()) {
		Err(_) => {
			println!("Failed to open file");
			process::exit(1);
		},
		Ok(file) => file,
	};

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

	let mut labels:       Vec <Label>     = Vec ::new();
	let mut needs_labels: Vec <(u32, i8)> = Vec ::new();
	let mut opcodes:      Vec <u16>       = Vec ::new();

	let mut index  = 0;
	let mut offset = 0;

	for line in &ready_lines {
		let data: Vec<&str> = line.split(' ').collect();
		if data.len() == 1 {
			if let Some(i) = data[0].find(':') {
				labels.push(Label {name: data[0][0..i].to_string().clone(), offset: offset});
			}
			match interpret_line(&data) {
				Ok((ins, op, label_pos)) => {
					if label_pos < 0 {
						needs_labels.push((index, label_pos));
						opcodes.push(0);
					}
					else {
						opcodes.push(convert_to_opcode(ins, op));
					}
					offset += 1;
				},
				Err(e)  => {
					println!("Syntax error on line {}: {}", index + 1, e + line.as_str());
					process::exit(2);
				},
			}
		}
		index += 1;
	}

	if args.is_present("o") {
		for i in opcodes {
			println!("{:x}", i);
		}
	} else {
		//Explicitly set up big endian for the chip-8 executable
		let mut to_output: Vec<u8> = Vec::new();
		let slice: &[u16] = &opcodes;
		for &n in slice {
			let _ = to_output.write_u16::<BigEndian>(n);
		}

		let mut file = File::create(args.value_of("output").expect("No file specified!")).unwrap();
		match file.write_all(to_output.as_slice()) {
			Ok(_) => {},
			Err(s) => println!("Failed to write to file {}: {}", args.value_of("output").unwrap(), s),
		}

	}
}
