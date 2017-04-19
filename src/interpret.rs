use std::string;
use std::result;
use std::num;

use instructions::*;

//Returns a string with comments removed, with everything in lowercase
pub fn sanitize_line(input: &String) -> String {
	input.clone() //Please implement me
}

fn convert_number(input: &str) -> Result<u16, String> {
	let convert = input.to_string();
	let result: result::Result<u16, num::ParseIntError>;
	if convert[..2].to_string() == "0x" {
		result = convert[2..].parse::<u16>();
	} else {
		result = u16::from_str_radix(&convert[..], 16);
	}
	match result {
		Ok(num) => return Ok(num),
		Err(_) => return Err("Not a number: ".to_string() + input),
	}
}

fn interpret_ld_instruction(data: &Vec<&str>) -> Result<(CPUInstruction, u16), String> {
	//LD Vx, y
	if data[1].chars().nth(0).unwrap() == 'v' {
		if data[1].chars().count() > 2 {
			return Err("Unknown register: ".to_string() + data[1]);
		}
		if data[2].chars().nth(0).unwrap() == 'v' {
			let convert = convert_number(&data[2][1..]);
			match convert {
				Ok(n) => {
					let x = convert_number(&data[1][1..]);
					match x {
						Ok(m) => return Ok((CPUInstruction::LD_Vy, (m >> 4) | (n >> 8))),
						Err(msg) => return Err(msg),
					}
				}
				Err(msg) => return Err(msg),
			}
		}
		match data[2] {
			"dt"  => return Ok((CPUInstruction::LD_Vx_DT, 0)),
			"k"   => return Ok((CPUInstruction::LD_K, 0)),
			"[i]" => return Ok((CPUInstruction::LD_Vx_ADDR_I, 0)),
			_     => {
				let convert = convert_number(&data[1][1..]);
				match convert {
					Ok(n) => return Ok((CPUInstruction::LD, n)),
					Err(msg) => return Err(msg),
				}
			}
		}
	}
	//
	Err("Not an ld variant".to_string())
}

//Returns two bytes to be written to the binary file
pub fn interpret_line(input: &String) -> Result<u16, String> {
	let input = sanitize_line(input);
	let data: Vec<&str> = input.split(' ').collect();

	//let (instruction, operand) = match data[0] {
	//	"ld" => get_ld_variant(&data),
	//	_ => (CPUInstruction::empty, 0),
	//};

	Ok(convert_to_opcode(CPUInstruction::DRW, 0xFFFF))
}
