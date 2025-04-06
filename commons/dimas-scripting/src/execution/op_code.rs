// Copyright © 2025 Stephan Kunz

//! Op-Code implementation for `DiMAS` scripting bytecode

/// @TODO:
#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
	/// @TODO:
	None = 0,
	/// @TODO:
	Constant,
	/// @TODO:
	Nil,
	/// @TODO:
	True,
	/// @TODO:
	False,
	/// @TODO:
	Pop,
	/// @TODO:
	DefineExternal,
	/// @TODO:
	GetExternal,
	/// @TODO:
	SetExternal,
	/// @TODO:
	Equal,
	/// @TODO:
	Greater,
	/// @TODO:
	Less,
	/// @TODO:
	Jmp,
	/// @TODO:
	JmpIfTrue,
	/// @TODO:
	JmpIfFalse,
	/// @TODO:
	Add,
	/// @TODO:
	Subtract,
	/// @TODO:
	Multiply,
	/// @TODO:
	Divide,
	/// @TODO:
	BitwiseNot,
	/// @TODO:
	BitwiseAnd,
	/// @TODO:
	BitwiseOr,
	/// @TODO:
	BitwiseXor,
	/// @TODO:
	Not,
	/// @TODO:
	Negate,
	/// @TODO:
	Return,
	/// @TODO:
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
