// Copyright © 2025 Stephan Kunz

//! Op-Code implementation for `DiMAS` scripting bytecode

/// The available instructions for the virtual machine-
#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
	/// No operation
	None = 0,
	/// Defining a constant
	Constant,
	/// Nil value
	Nil,
	/// True
	True,
	/// False:
	False,
	/// Get next instruction
	Pop,
	/// Define a global/external variable
	DefineExternal,
	/// Get a global/external variable
	GetExternal,
	/// Set a global/external variable
	SetExternal,
	/// Equality
	Equal,
	/// Comparison greater
	Greater,
	/// Comparison less
	Less,
	/// Jump
	Jmp,
	/// Jump if condition is fulfilled
	JmpIfTrue,
	/// Jump if condition is not fulfilled
	JmpIfFalse,
	/// Add two numbers
	Add,
	/// Subtract number
	Subtract,
	/// Multiply two numbers
	Multiply,
	/// Divide number
	Divide,
	/// Bitwise not
	BitwiseNot,
	/// Bitwise and
	BitwiseAnd,
	/// Bitwise or
	BitwiseOr,
	/// Bitwise exclusive or
	BitwiseXor,
	/// Not Equal
	Not,
	/// Negation
	Negate,
	/// Return value to caller
	Return,
	/// Print value to "stdout"
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
