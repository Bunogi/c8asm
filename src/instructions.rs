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
	use CPUInstruction::*;
	match ins { //Match the first one
		CLS          => 0x00E0,
		RET          => 0x00EE,
		SYS          => 0x0     | param,
		JP           => 0x1000  | param,
		CALL         => 0x2000  | param,
		SE           => 0x3000  | param,
		SNE          => 0x4000  | param,
		SE_Vy        => 0x5000  | middle_nibs(param),
		LD           => 0x6000  | param,
		ADD          => 0x7000  | param,
		LD_Vy        => 0x8000  | middle_nibs(param),
		OR           => 0x8001  | middle_nibs(param),
		AND          => 0x8002  | middle_nibs(param),
		XOR          => 0x8003  | middle_nibs(param),
		ADD_Vy       => 0x8004  | middle_nibs(param),
		SUB          => 0x8005  | middle_nibs(param),
		SHR          => 0x8006  | middle_nibs(param),
		SUBN         => 0x8007  | middle_nibs(param),
		SHL          => 0x800E  | middle_nibs(param),
		SNE_Vy       => 0x9000  | middle_nibs(param),
		LD_I         => 0xA000  | param,
		JP_V0        => 0xB000  | param,
		RND          => 0xC000  | param,
		DRW          => 0xD000  | param,
		SKP          => 0xE09E  | sec_nib(param),
		SKNP         => 0xE0A1  | sec_nib(param),
		LD_Vx_DT     => 0xF007  | sec_nib(param),
		LD_K         => 0xF00A  | sec_nib(param),
		LD_DT_Vx     => 0xF015  | sec_nib(param),
		LD_ST_Vx     => 0xF018  | sec_nib(param),
		ADD_I        => 0xF01E  | sec_nib(param),
		LD_F         => 0xF029  | sec_nib(param),
		LD_B         => 0xF033  | sec_nib(param),
		LD_ADDR_I_Vx => 0xF055  | sec_nib(param),
		LD_Vx_ADDR_I => 0xF065  | sec_nib(param),
		empty        => panic!("Shouldn't ever happen"),
	}
}
