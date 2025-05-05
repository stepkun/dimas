// Copyright © 2025 Stephan Kunz

//! `DiMAS` scripting runtime errors

// region		--- modules
use dimas_core::ConstString;
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `scripting` runtime error type
#[derive(Error, Debug)]
pub enum Error {
	/// Pass through core error.
	#[error("{0}")]
	Core(#[from] dimas_core::error::Error),
	/// @TODO:
	#[error("Boolean values do not allow arithmetic operations")]
	BoolNoArithmetic,
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
	#[error("this should be unreachable in vm.rs line {0}")]
	Unreachable(u32),
}
// region:		--- Error
