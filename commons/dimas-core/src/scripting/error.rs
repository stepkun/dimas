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
	#[error("could not create Chunk for VM")]
	NoChunk,
	/// @TODO:
	#[error("could not parse HexNumber {0} at line {1}")]
	ParseHex(String, usize),
	/// @TODO:
	#[error("could not parse Number {0} at line {1}")]
	ParseNumber(String, usize),
	/// @TODO:
	#[error("unexpected character {0} at line {1}")]
	UnexpectedChar(String, usize),
	/// @TODO:
	#[error("unknown Operation Code")]
	UnknownOpCode,
	/// @TODO:
	#[error("unterminated String {0} at line {1}")]
	UnterminatedString(String, usize),
}
// region:		--- Error
