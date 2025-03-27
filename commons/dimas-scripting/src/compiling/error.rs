// Copyright © 2025 Stephan Kunz

//! `scripting` errors

use alloc::string::String;
// region		--- modules
use thiserror::Error;
// endregion:	--- modules

// region:		--- Error
/// `scripting` error type
#[derive(Error, Debug)]
pub enum Error {
	/// Pass through VM error
	#[error("{0}")]
	VM(#[from] crate::execution::error::Error),
	/// @TODO:
	#[error("Expecting {0} found {1} at line {2}")]
	ExpectedToken(String, String, usize),
	/// @TODO:
	#[error("expression expected at line {0}")]
	ExpressionExpected(usize),
	/// @TODO:
	#[error("could not create Chunk for VM")]
	NoChunk,
	/// @TODO:
	#[error("could not parse HexNumber {0} at line {1}")]
	ParseHex(String, usize),
	/// @TODO:
	#[error("could not parse Number {0} at line {1}")]
	ParseNumber(String, usize),
	/// @TODO:
	#[error("Value storage is full")]
	ToManyValues,
	/// @TODO:
	#[error("unexpected character {0} at line {1}")]
	UnexpectedChar(String, usize),
	/// @TODO:
	#[error("unexpected Token at line {0}")]
	UnexpectedToken(usize),
	/// @TODO:
	#[error("unterminated String {0} at line {1}")]
	UnterminatedString(String, usize),

	/// @TODO:
	#[error("this should be unreachable in {0}")]
	Unreachable(String),
}
// region:		--- Error
