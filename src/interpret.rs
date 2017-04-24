use std::result;
use std::num;

use instructions::*;

//Returns a string with comments removed, with everything in lowercase
pub fn sanitize_line(input: &mut String) {
	println!("Input: {}", *input);
	while let Some(i) = input.find(',') { input.remove(i); }

	match input.find(';') {
		Some(i) => *input = input[0..i].to_string(),
		None    => {},
	}

	//TODO: remove comments from input
	input.trim_left();
	input.trim_right();
	*input = input.to_lowercase();
}

fn convert_number(input: &str) -> Option<u16> {
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
		Ok(num) => Some(num),
		Err(_)  => None,
	}
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
				Some(n) => {
					let x = convert_number(&data[1][1..]);
					match x {
						Some(m) => return Ok((CPUInstruction::LD_Vy, (m << 4) | (n << 8))),
						None    => return Err(format!("Not a number: {}", data[2])),
					}
				}
				None => return Err(format!("Unknown register: {}",  data[2])),
			}
		}
		//LD Vx, *
		match data[2] {
			"dt"  => return Ok((CPUInstruction::LD_Vx_DT, convert_register(data[1]).unwrap() << 8)),
			"k"   => return Ok((CPUInstruction::LD_K, convert_register(data[1]).unwrap() << 8)),
			"[i]" => return Ok((CPUInstruction::LD_Vx_ADDR_I, convert_register(data[1]).unwrap() << 8)),
			_     => { //LD Vx, byte
				let x: u16;
				let byte: u16;

				let convert = convert_number(&data[1][1..]);
				match convert {
					Some(n) => {
						x = n;
					},
					None => return Err(format!("Unknown register: {}", data[1])),
				}

				match convert_number(&data[2]) {
					Some(n) => byte = n & 0x00FF,
					None    => return Err(format!("Not a number: {}", data[2])),
				}
				return Ok((CPUInstruction::LD, (x << 8) | byte));

			}
		}
	}

	let v = match convert_register(data[2]) {
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

fn convert_register(input: &str) -> Result<u16, String> {
	if input.chars().nth(0).unwrap() != 'v' {
		return Err("Invalid register: ".to_string() + input);
	}
	let num = u16::from_str_radix(&input[1..], 16).unwrap();
	match num > 0xF {
		true  => Err("Register out of range: ".to_string() + input),
		false => Ok(num),
	}
}

pub struct Label {
	pub name: String,
	pub offset: u16,
}

//Returns the instruction and label, as well as whether a label is used
pub fn interpret_line(data: &Vec<&str>) -> Result<(CPUInstruction, u16, i8), String> {
	macro_rules! x {
		() => (
			(convert_register(&data[1]).unwrap() << 8)
		)
	}
	macro_rules! y {
		() => (
			(convert_register(&data[2]).unwrap() << 4)
		)
	}

	use instructions::CPUInstruction::*;

	let label_pos: i8;

	//Sets up the return value
	macro_rules! check_label {
		($instr:expr, $index:expr) => {
			match convert_number(data[$index as usize]) {
				Some(n) => {
					label_pos = -1;
					($instr, n)
				},
				None    => {
					label_pos = $index;
					($instr, 0,)
				},
			}
		}
	}

	let (ins, op) = match data[0] {
		""    => (blank, 0), //Skip statements without an instruction
		"cls" => (CLS, 0),
		"ret" => (RET, 0),
		"sys" => (SYS, convert_number(data[1]).unwrap()),
		"jp"  => {
			match data.len() {
				2 =>  {
					check_label!(JP, 1)
				}
				3 => {
					if data[1] != "v0" {
						return Err("Invalid register: ".to_string() + data[1] + ". Did you mean V0?");
					} else {
						check_label!(JP_V0, 2)
					}
				},
				_ => return Err("Too many parameters!".to_string()),
			}
		},
		"call" => check_label!(CALL, 1),
		"se" => {
			if data[2].chars().nth(0).unwrap() != 'v' {
				(SE, x!() | convert_number(data[2]).unwrap())
			} else {
				(SE_Vy, x!() | y!())
			}
		},
		"sne" => {
			if data[2].chars().nth(0).unwrap() != 'v' {
				(SNE, x!() | convert_number(data[2]).unwrap())
			} else {
				(SNE_Vy, x!() | y!())
			}
		},
		"add" => {
			if data[2].chars().nth(0).unwrap() != 'v' {
				(ADD, x!() | convert_number(data[2]).unwrap())
			} else if data[1] == "i" {
				(ADD_I, y!() << 4)
			} else {
				(ADD_Vy, x!() | y!())
			}
		},
		"or"   => (OR,   x!() | y!()),
		"and"  => (AND,  x!() | y!()),
		"xor"  => (XOR,  x!() | y!()),
		"sub"  => (SUB,  x!() | y!()),
		"shr"  => (SHR,  x!()),
		"subn" => (SUBN, x!() | y!()),
		"shl"  => (SHL,  x!()),
		"rnd"  => (RND,  x!() | (convert_number(data[2]).unwrap() & 0xFF)),
		"drw"  => (DRW,  x!() | y!() | (convert_number(data[3]).unwrap() & 0xFF)),
		"skp"  => (SKP,  x!()),
		"sknp" => (SKNP, x!()),
		"ld"   =>  {
			//LD I, addr
			if data[1] == "i" {
				check_label!(LD_I, 2)
			} else {
				match interpret_ld_instruction(&data) {
					Ok((ins, op)) => (ins, op),
					Err(s)        => return Err(s),
				}
			}
		},

		_ => return Err(format!("Unknown instruction: {}", data[0]).to_string()),
	};
	Ok((ins, op, label_pos))
}
