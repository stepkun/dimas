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
	/// @TODO:
	#[error("expression expected")]
	ExpressionExpected,
	/// @TODO:
	#[error("could not create Chunk for VM")]
	NoChunk,
	/// @TODO:
	#[error("could not parse HexNumber {0} at line {1}")]
	ParseHex(String, i16),
	/// @TODO:
	#[error("HexNumber storage is full")]
	ToManyHexNumbers,
	/// @TODO:
	#[error("could not parse Number {0} at line {1}")]
	ParseNumber(String, i16),
	/// @TODO:
	#[error("Number storage is full")]
	ToManyNumbers,
	/// @TODO:
	#[error("unexpected character {0} at line {1}")]
	UnexpectedChar(String, i16),
	/// @TODO:
	#[error("unexpected Token")]
	UnexpectedToken,
	/// @TODO:
	#[error("unterminated String {0} at line {1}")]
	UnterminatedString(String, i16),

	/// @TODO:
	#[error("this should be unreachable")]
	Unreachable,
}
// region:		--- Error
