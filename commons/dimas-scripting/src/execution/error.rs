// Copyright © 2025 Stephan Kunz

//! `scripting` errors

// region		--- modules
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `scripting` error type
#[derive(Error, Debug)]
pub enum Error {
	/// @TODO:
	#[error("The Variable has not been defined")]
	GlobalNotDefined,
	/// @TODO:
	#[error("Value is not a Boolean type")]
	NoBoolean,
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
	#[error("this should be unreachable in vm.rs line {0}")]
	Unreachable(u32),
}
// region:		--- Error
