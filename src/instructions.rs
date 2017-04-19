use std::string;

//The instruction decoding is more legible this way
#[allow(non_camel_case_types)]
pub enum CPUInstruction {
	CLS,
	RET,
	SYS,
	JP,
	CALL,
	SE,
	SNE,
	SE_Vy,
	LD,
	ADD,
	LD_Vy,
	OR,
	AND,
	XOR,
	ADD_Vy,
	SUB,
	SHR,
	SUBN,
	SHL,
	SNE_Vy,
	LD_I,
	JP_V0,
	RND,
	DRW,
	SKP,
	SKNP,
	LD_Vx_DT,
	LD_K,
	LD_DT_Vx,
	LD_ST_Vx,
	ADD_I,
	LD_F,
	LD_B,
	LD_ADDR_I_Vx,
	LD_Vx_ADDR_I,
	empty,
}

fn middle_nibs(input: u16) -> u16 {
	input & 0x0FF0
}

fn sec_nib(input: u16) -> u16 {
	input & 0x0F00
}

pub fn convert_to_opcode(ins: CPUInstruction, param: u16) -> u16 {
	let param: u16 = param & 0x0FFF;
	match ins { //Match the first one
		CPUInstruction::CLS => 0x00E0,
		CPUInstruction::RET => 0x00EE,
		CPUInstruction::SYS => 0x0 | param,
		CPUInstruction::JP => 0x1000 | param,
		CPUInstruction::CALL => 0x2000 | param,
		CPUInstruction::SE => 0x3000 | param,
		CPUInstruction::SNE => 0x4000 | param,
		CPUInstruction::SE_Vy => 0x5000 | middle_nibs(param),
		CPUInstruction::LD => 0x6000 | param,
		CPUInstruction::ADD => 0x7000 | param,
		CPUInstruction::LD_Vy => 0x8000 | middle_nibs(param),
		CPUInstruction::OR => 0x8001 | middle_nibs(param),
		CPUInstruction::AND => 0x8002 | middle_nibs(param),
		CPUInstruction::XOR => 0x8003 | middle_nibs(param),
		CPUInstruction::ADD_Vy => 0x8004 | middle_nibs(param),
		CPUInstruction::SUB => 0x8005 | middle_nibs(param),
		CPUInstruction::SHR => 0x8006 | middle_nibs(param),
		CPUInstruction::SUBN => 0x8007 | middle_nibs(param),
		CPUInstruction::SHL => 0x800E | middle_nibs(param),
		CPUInstruction::SNE_Vy => 0x9000 | middle_nibs(param),
		CPUInstruction::LD_I => 0xA000 | param,
		CPUInstruction::JP_V0 => 0xB000 | param,
		CPUInstruction::RND => 0xC000 | param,
		CPUInstruction::DRW => 0xD000 | param,
		CPUInstruction::SKP => 0xE09E | sec_nib(param),
		CPUInstruction::SKNP => 0xE0A1 | sec_nib(param),
		CPUInstruction::LD_Vx_DT => 0xF007 | sec_nib(param),
		CPUInstruction::LD_K => 0xF00A | sec_nib(param),
		CPUInstruction::LD_DT_Vx => 0xF015 | sec_nib(param),
		CPUInstruction::LD_ST_Vx => 0xF018 | sec_nib(param),
		CPUInstruction::ADD_I => 0xF01E | sec_nib(param),
		CPUInstruction::LD_F => 0xF029 | sec_nib(param),
		CPUInstruction::LD_B => 0xF033 | sec_nib(param),
		CPUInstruction::LD_ADDR_I_Vx => 0xF055 | sec_nib(param),
		CPUInstruction::LD_Vx_ADDR_I => 0xF065 | sec_nib(param),
		CPUInstruction::empty => panic!("Shouldn't ever happen"),
	}
}
