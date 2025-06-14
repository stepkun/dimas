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
	/// Did not get the expected `Token`.
	#[error("Expecting {0} found {1} at line {2}")]
	ExpectedToken(ConstString, ConstString, usize),
	/// Whatever it is: It is not an expression.
	#[error("expression expected at line {0}")]
	ExpressionExpected(usize),
	/// Compile failed to create a `Chunk`.
	#[error("could not create Chunk for VM")]
	NoChunk,
	/// Not a hex number.
	#[error("could not parse HexNumber {0} at line {1}")]
	ParseHex(ConstString, usize),
	/// Not an int number.
	#[error("could not parse IntNumber {0} at line {1}")]
	ParseInt(ConstString, usize),
	/// Not a float number.
	#[error("could not parse Number {0} at line {1}")]
	ParseNumber(ConstString, usize),
	/// Exceeded the size of storage for values.
	#[error("Value storage is full")]
	ToManyValues,
	/// This Char should not be here.
	#[error("unexpected character {0} at line {1}")]
	UnexpectedChar(ConstString, usize),
	/// Got an unexpected `Token`.
	#[error("unexpected Token at line {0}")]
	UnexpectedToken(usize),
	/// Missing string termination.
	#[error("unterminated String {0} at line {1}")]
	UnterminatedString(ConstString, usize),

	/// Pass through core error.
	#[error("{0}")]
	Core(#[from] dimas_core::error::Error),
	/// No arithemetic with boolean for now.
	#[error("Boolean values do not allow arithmetic operations")]
	BoolNoArithmetic,
	/// Tried to redefine an enum value.
	#[error("Enum variant [{0}] already exists with value [{1}] new value: [{2}]")]
	DuplicateEnumVariant(ConstString, i8, i8),
	/// Enum value is not defined.
	#[error("could not find Enum {0} at line {1}")]
	EnumValNotFound(ConstString, usize),
	/// Storage for global values exceeded.
	#[error("Variable [{0}] exceeds type limits")]
	GlobalExceedsLimits(ConstString),
	/// This is a type that is not knpwn in Scripting.
	#[error("Variable [{0}] has an unknown type")]
	GlobalHasUnknownType(ConstString),
	/// Tried to read a non existing variable.
	#[error("Variable [{0}] has not been defined")]
	GlobalNotDefined(ConstString),
	/// Type of variable is different than expected.
	#[error("Variable [{0}] has a wrong type")]
	GlobalWrongType(ConstString),
	/// Nil does not allow anything.
	#[error("Value is 'Nil' which does not allow any operation")]
	NilValue,
	/// Expected Boolean, got something else.
	#[error("Value is not a Boolean type")]
	NoBoolean,
	/// Comparisons (greater, less) only with numbers
	#[error("comparing Values needs two numeric types")]
	NoComparison,
	/// Expected Double, got something else.
	#[error("Value is not a Double type")]
	NoDouble,
	/// Expected Integer, got something else.
	#[error("Value is not an Integer type")]
	NoInteger,
	/// Expected String, got something else.
	#[error("Value is not a String type")]
	NoString,
	/// Expected Number, got something else.
	#[error("Value is not a number type")]
	NoNumber,
	/// Strings only allow additions.
	#[error("to Strings you can only 'ADD' something")]
	OnlyAdd,
	/// Stack of values exceeded.
	#[error("Value stack overflow")]
	StackOverflow,
	/// An unknown `OpCode`.
	#[error("unknown Operation Code")]
	UnknownOpCode,

	/// A really unexpected error happened.
	#[error("unexpected [{0}] in file [{1}] at line [{2}]")]
	Unexpected(ConstString, ConstString, u32),
	/// This code line never should have been reached.
	#[error("should be unreachable in {0} at line {1}")]
	Unreachable(ConstString, u32),
}
// region:		--- Error
