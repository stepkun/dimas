// Copyright © 2025 Stephan Kunz

//! Op-Code implementation for `DiMAS` scripting bytecode

#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
	None = 0,
	Constant,
	Nil,
	True,
	False,
	Pop,
	DefineExternal,
	GetExternal,
	SetExternal,
	Equal,
	Greater,
	Less,
	Jmp,
	JmpIfTrue,
	JmpIfFalse,
	Add,
	Subtract,
	Multiply,
	Divide,
	BitwiseNot,
	BitwiseAnd,
	BitwiseOr,
	BitwiseXor,
	Not,
	Negate,
	Return,
	#[cfg(feature = "std")]
	Print = 254,
}

impl From<u8> for OpCode {
	fn from(value: u8) -> Self {
		match value {
			0 => Self::None,
			1 => Self::Constant,
			2 => Self::Nil,
			3 => Self::True,
			4 => Self::False,
			5 => Self::Pop,
			6 => Self::DefineExternal,
			7 => Self::GetExternal,
			8 => Self::SetExternal,
			9 => Self::Equal,
			10 => Self::Greater,
			11 => Self::Less,
			12 => Self::Jmp,
			13 => Self::JmpIfTrue,
			14 => Self::JmpIfFalse,
			15 => Self::Add,
			16 => Self::Subtract,
			17 => Self::Multiply,
			18 => Self::Divide,
			19 => Self::BitwiseNot,
			20 => Self::BitwiseAnd,
			21 => Self::BitwiseOr,
			22 => Self::BitwiseXor,
			23 => Self::Not,
			24 => Self::Negate,
			25 => Self::Return,
			#[cfg(feature = "std")]
			254 => Self::Print,
			_ => todo!("unknown value for OpCode"),
		}
	}
}
