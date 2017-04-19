extern crate clap;

use clap::{Arg, App, SubCommand};

use std::fs::File;

mod instructions;
use instructions::*;

mod interpret;
use interpret::*;

fn main() {
	//let args = App::new("b8asm")
	//	.version("0.0")
	//	.author("Bunogi")
	//	.args_from_usage("<file> 'assembly file to use'")
	//	.about("Assembles Chip-8 programs")
	//	.get_matches(); */

	//let file = args.value_of("file").unwrap();
	match interpret_line(&"ld 0, 0x20".to_string()) {
		Ok(out) => println!("{:x}", out),
		Err(e) => println!("Syntax error: {}", e),
	}
	println!("opcode_convert: {:x}",
	         convert_to_opcode(CPUInstruction::DRW, 0x0FFF));
}
