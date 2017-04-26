//Command line args
extern crate clap;
use clap::{Arg, App};

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
		.arg(Arg::with_name("stdout").help("Write output to stdout instead of your file")
			.short("s").conflicts_with("o"))
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

	let mut needs_labels: Vec<(CPUInstruction, &str, usize, i8)> = Vec::new();
	let mut opcodes:Vec<u8>  = Vec::new();
	let mut labels:Vec<Label> = Vec::new();

	let mut memory_offset: usize = 0;

	//Splits the opcode. Takes a u16
	for line in &ready_lines {
		let data: Vec<&str> = line.split_whitespace().collect();
		if data.len() == 1 {
			if let Some(i) = data[0].find(':') {
				labels.push(Label {name: data[0][0..i].to_string().clone(), 
					offset: 0x200 + memory_offset});
				continue;
			}
		}
		use CPUInstruction::db;
		match interpret_line(&data) {
			Ok((ins, op, label_pos)) => {
				if ins == db {
					memory_offset += 1;
					opcodes.push(op as u8);
				} else if label_pos < 0 {
					let (a, b) = convert_to_opcode(ins, op);
					opcodes.push(a);
					opcodes.push(b);
					memory_offset += 2;
				} else {
					opcodes.push(0); opcodes.push(0);
					needs_labels.push((ins, &line, memory_offset, label_pos));
					memory_offset += 2;
				}
			},
			Err(e)  => {
				println!("Syntax error in line \"{}\": {}", &line, e);
				process::exit(2);
			},
		}
	}

	for (ins, line, offset, label_pos) in needs_labels {
		let label = line.split_whitespace().nth(label_pos as usize).unwrap();
		match labels.iter().find(|&l| *l == Label{name:label.to_string(), offset: 0}) {
			Some(i) => {
				let (a, b) = convert_to_opcode(ins, i.offset as u16);
				opcodes[offset as usize] = a;
				opcodes[(offset + 1) as usize] = b;
			}
			None    => {
				println!("\"{}\": Unknown label: {}", line, label);
				process::exit(2);
			}
		}
	}

	if args.is_present("stdout") {
		for i in opcodes {
			println!("{:x}", i);
		}
	} else {
		let mut file = File::create(args.value_of("output").expect("No file specified!")).unwrap();
		match file.write_all(opcodes.as_slice()) {
			Ok(_) => {},
			Err(s) => println!("Failed to write to file {}: {}", args.value_of("output").unwrap(), s),
		}

	}
}
