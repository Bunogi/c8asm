use std::result;
use std::num;

use instructions::*;

//Returns a string with comments removed, with everything in lowercase
pub fn sanitize_line(input: &mut String){
	println!("Input: {}", *input);
	if let Some(i) = input.find(',') { input.remove(i); }

	//TODO: remove comments from input
	input.trim_left();
	input.trim_right();
	*input = input.to_lowercase();
}

fn convert_number(input: &str) -> Result<u16, String> {
	let convert = input.to_string();
	let result: result::Result<u16, num::ParseIntError>;
	if convert.chars().count() >= 2 {
		if convert[..2].to_string() == "0x" {
			result = u16::from_str_radix(&convert[2..], 16);
		} else {
			result = convert.parse::<u16>();
		}
	}
	else {
		result = convert.parse::<u16>();
	}

	match result {
		Ok(num) => return Ok(num),
		Err(_)  => return Err("Not a number: ".to_string() + input),
	};
}

fn interpret_ld_instruction(data: &Vec<&str>) -> Result<(CPUInstruction, u16), String> {
	//LD Vx, Vy
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
						Ok(m)    => return Ok((CPUInstruction::LD_Vy, (m << 4) | (n << 8))),
						Err(msg) => return Err(msg),
					}
				}
				Err(msg) => return Err(msg),
			}
		}
		//LD Vx, *
		match data[2] {
			"dt"  => return Ok((CPUInstruction::LD_Vx_DT, 0)),
			"k"   => return Ok((CPUInstruction::LD_K, 0)),
			"[i]" => return Ok((CPUInstruction::LD_Vx_ADDR_I, 0)),
			_     => { //LD Vx, byte
				let x: u16;
				let byte: u16;

				let convert = convert_number(&data[1][1..]);
				match convert {
					Ok(n)    => {
						x = n;
					},
					Err(msg) => return Err(msg),
				}

				match convert_number(&data[2]) {
					Ok(n)    => byte = n & 0x00FF,
					Err(msg) => return Err(msg),
				}
				return Ok((CPUInstruction::LD, (x << 8) | byte));

			}
		}
	}

	//LD I, addr
	if data[1] == "i" {
		let addr = match convert_number(&data[2]) {
			Ok(n)    => n,
			Err(msg) => return Err(msg),
		};
		return Ok((CPUInstruction::LD_I, addr));
	}

	let v = match convert_number(&data[2]) {
		Ok(n)    => n << 8,
		Err(msg) => return Err(msg),
	};
	match data[1] {
		"dt"  => return Ok((CPUInstruction::LD_DT_Vx, v)),
		"st"  => return Ok((CPUInstruction::LD_ST_Vx, v)),
		"f"   => return Ok((CPUInstruction::LD_F, v)),
		"b"   => return Ok((CPUInstruction::LD_B, v)),
		"[i]" => return Ok((CPUInstruction::LD_ADDR_I_Vx, v)),
		_     => return Err("Not an ld variant".to_string()),
	}
}

//Returns two bytes to be written to the binary file
pub fn interpret_line(input: &mut String) -> Result<u16, String> {
	sanitize_line(input);
	let data: Vec<&str> = input.split(' ').collect();

	match data[0] {
		"ld" => match interpret_ld_instruction(&data) {
			Ok((ins, op)) => return Ok(convert_to_opcode(ins, op)),
			Err(s)        => return Err(s),
		},
		_    => return Err("Failed to parse line: {}".to_string()  + input),
	};

}
