// Copyright © 2025 Stephan Kunz

//! `DiMAS` scripting compiletime errors

// region		--- modules
use dimas_core::ConstString;
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `scripting` error type
#[derive(Error, Debug)]
pub enum Error {
	/// @TODO:
	#[error("Expecting {0} found {1} at line {2}")]
	ExpectedToken(ConstString, ConstString, usize),
	/// @TODO:
	#[error("expression expected at line {0}")]
	ExpressionExpected(usize),
	/// @TODO:
	#[error("could not create Chunk for VM")]
	NoChunk,
	/// @TODO:
	#[error("could not parse HexNumber {0} at line {1}")]
	ParseHex(ConstString, usize),
	/// @TODO:
	#[error("could not parse IntNumber {0} at line {1}")]
	ParseInt(ConstString, usize),
	/// @TODO:
	#[error("could not parse Number {0} at line {1}")]
	ParseNumber(ConstString, usize),
	/// @TODO:
	#[error("Value storage is full")]
	ToManyValues,
	/// @TODO:
	#[error("unexpected character {0} at line {1}")]
	UnexpectedChar(ConstString, usize),
	/// @TODO:
	#[error("unexpected Token at line {0}")]
	UnexpectedToken(usize),
	/// @TODO:
	#[error("unterminated String {0} at line {1}")]
	UnterminatedString(ConstString, usize),

	/// Pass through core error.
	#[error("{0}")]
	Core(#[from] dimas_core::error::Error),
	/// @TODO:
	#[error("Boolean values do not allow arithmetic operations")]
	BoolNoArithmetic,
	/// @TODO:
	#[error("Enum variant [{0}] already exists with value [{1}] new value: [{2}]")]
	DuplicateEnumVariant(ConstString, i8, i8),
	/// @TODO:
	#[error("could not find Enum {0} at line {1}")]
	EnumValNotFound(ConstString, usize),
	/// @TODO:
	#[error("Variable [{0}] exceeds type limits")]
	GlobalExceedsLimits(ConstString),
	/// @TODO:
	#[error("Variable [{0}] has an unknown type")]
	GlobalHasUnknownType(ConstString),
	/// @TODO:
	#[error("Variable [{0}] has not been defined")]
	GlobalNotDefined(ConstString),
	/// @TODO:
	#[error("Variable [{0}] has a wrong type")]
	GlobalWrongType(ConstString),
	/// @TODO:
	#[error("Value is 'Nil' which does not allow any operation")]
	NilValue,
	/// @TODO:
	#[error("Value is not a Boolean type")]
	NoBoolean,
	/// @TODO:
	#[error("comparing Values needs two numeric types")]
	NoComparison,
	/// @TODO:
	#[error("Value is not a Double type")]
	NoDouble,
	/// @TODO:
	#[error("Value is not an Integer type")]
	NoInteger,
	/// @TODO:
	#[error("Value is not a String type")]
	NoString,
	/// @TODO:
	#[error("Value is not a number type")]
	NoNumber,
	/// @TODO:
	#[error("to Strings you can only 'ADD' something")]
	OnlyAdd,
	/// @TODO:
	#[error("Value stack overflow")]
	StackOverflow,
	/// @TODO:
	#[error("unknown Operation Code")]
	UnknownOpCode,

	/// @TODO:
	#[error("unexpected [{0}] in file [{1}] at line [{2}]")]
	Unexpected(ConstString, ConstString, u32),
	/// @TODO:
	#[error("should be unreachable in {0} at line {1}")]
	Unreachable(ConstString, u32),
}
// region:		--- Error
